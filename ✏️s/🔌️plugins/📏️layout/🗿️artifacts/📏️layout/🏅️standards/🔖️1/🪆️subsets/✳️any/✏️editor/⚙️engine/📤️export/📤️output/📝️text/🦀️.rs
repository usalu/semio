//! 📝️ Export formatting owns bounded stack bytes and publishes only complete UTF8 writes.
use std::fmt::{self,Write};
use std::mem::MaybeUninit;
pub(super) struct TextFragment{bytes:[MaybeUninit<u8>;super::OUTPUT_BODY_BYTES],length:usize,limit:usize,poisoned:bool}
impl TextFragment{
 pub fn new(limit:usize)->Result<Self,&'static str>{if limit>super::OUTPUT_BODY_BYTES{return Err("layout-output-text-capacity");}Ok(Self{bytes:[MaybeUninit::uninit();super::OUTPUT_BODY_BYTES],length:0,limit,poisoned:false})}
 pub fn written(&self)->&[u8]{unsafe{std::slice::from_raw_parts(self.bytes.as_ptr().cast::<u8>(),self.length)}}
 pub fn accepted(&self)->bool{!self.poisoned}
 pub fn finish(&self)->Result<&[u8],&'static str>{if self.poisoned{Err("layout-output-text-overflow")}else{Ok(self.written())}}
}
impl Write for TextFragment{fn write_str(&mut self,value:&str)->fmt::Result{if self.poisoned||value.len()>self.limit-self.length{self.poisoned=true;return Err(fmt::Error);}let end=self.length+value.len();unsafe{std::ptr::copy_nonoverlapping(value.as_ptr(),self.bytes.as_mut_ptr().cast::<u8>().add(self.length),value.len());}self.length=end;Ok(())}}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
