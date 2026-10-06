/// ✍️ Validated semantic text writes retain their actual prefix under the supplied native authority.
pub trait TextEncodingOutput{fn write_text(&mut self,text:&str)->Result<(),ValueError>;}
struct TextOutput<'a,'b,const N:usize>{bytes:&'a mut PagedList<u8,N>,control:&'a mut crate::NativeEncodeControl<'b>,maximum_payload_bytes:usize,refused:bool}
impl<const N:usize> TextEncodingOutput for TextOutput<'_,'_,N>{
    fn write_text(&mut self,text:&str)->Result<(),ValueError>{
        if self.refused{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"semantic text output previously refused"))}
        let result=self.control.scoped_stage(|control|{
            self.bytes.len().checked_add(text.len()).filter(|length|*length<=self.maximum_payload_bytes&&*length<=N).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"semantic text output exceeds its logical payload authority"))?;
            control.begin_stage(text.len())?;
            for byte in text.as_bytes(){
                while !self.bytes.has_reserved_slot(){let required=self.bytes.next_allocation_bytes().map_err(error)?;control.charge(required)?;let step=self.bytes.reserve_one(required).map_err(|failure|error(failure.refusal()))?;if !step.progressed||step.allocated_bytes!=required{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"semantic text output allocation differs from exact admission"))}}
                self.bytes.push_reserved(*byte).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"semantic text output lost its funded byte"))?;control.step()?;
            }Ok::<_,ValueError>(())
        });if result.is_err(){self.refused=true;}result
    }
}
impl<const N:usize> PagedText<N>{
    /// 🧾 Only an accepted whole producer establishes a live UTF8 borrow; every refusal retains the prefix.
    pub fn encode_with(&mut self,maximum_payload_bytes:usize,control:&mut crate::NativeEncodeControl<'_>,producer:impl FnOnce(&mut dyn TextEncodingOutput)->Result<(),ValueError>)->Result<(),ValueError>{
        if self.bytes.allocated_bytes()!=0||self.complete||self.closing{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"semantic text encoding requires an empty owner"))}
        if maximum_payload_bytes>N{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"semantic text encoding payload authority exceeds its declared capacity"))}
        let mut output=TextOutput{bytes:&mut self.bytes,control,maximum_payload_bytes,refused:false};producer(&mut output)?;
        if output.refused{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"semantic text producer ignored a refused write"))}
        self.complete=true;Ok(())
    }
}
struct TextFormatOutput<'a>{output:&'a mut dyn TextEncodingOutput,refusal:Option<ValueError>}
impl std::fmt::Write for TextFormatOutput<'_>{
    fn write_str(&mut self,text:&str)->std::fmt::Result{if self.refusal.is_some(){return Err(std::fmt::Error)}match self.output.write_text(text){Ok(())=>Ok(()),Err(error)=>{self.refusal=Some(error);Err(std::fmt::Error)}}}
}
/// 🔤️ Streams authored formatting into the real text owner without creating a contiguous String.
pub fn write_encoding_format(output:&mut dyn TextEncodingOutput,arguments:std::fmt::Arguments<'_>)->Result<(),ValueError>{
    let mut writer=TextFormatOutput{output,refusal:None};let result=std::fmt::write(&mut writer,arguments);match(writer.refusal,result){(Some(error),_)=>Err(error),(None,Ok(()))=>Ok(()),(None,Err(_))=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"semantic text formatter refused its output"))}
}
