fn main (){
   let a = is_fib(4);
   println!("{}",a);
}
fn is_fib(num : i32 ) -> i32 {
  if num  == 0 {
   return 0;
  }
  if num == 1 {
   return 1;
  }
  else {
      return is_fib(num-1)+is_fib(num-2)
  }
}
