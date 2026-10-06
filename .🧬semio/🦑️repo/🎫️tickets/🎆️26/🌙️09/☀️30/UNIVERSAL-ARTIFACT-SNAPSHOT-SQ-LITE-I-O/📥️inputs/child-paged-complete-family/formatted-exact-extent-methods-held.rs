struct TextExtentCounter<'a>{output:&'a mut dyn TextEncodingOutput,bytes:usize,refusal:Option<ValueError>}
impl std::fmt::Write for TextExtentCounter<'_>{
    fn write_str(&mut self,text:&str)->std::fmt::Result{
        if self.refusal.is_some(){return Err(std::fmt::Error)}
        let Some(next)=self.bytes.checked_add(text.len())else{self.refusal=Some(ValueError::new(ValueRefusalKind::OwnershipLimit,"formatted semantic extent exceeds native authority"));return Err(std::fmt::Error)};
        match self.output.measure_text_fragment(next,text){Ok(())=>{self.bytes=next;Ok(())},Err(error)=>{self.refusal=Some(error);Err(std::fmt::Error)}}
    }
}

impl<const N:usize> TextOutput<'_,'_,N>{
    fn measure_extent_fragment(&mut self,bytes:usize,_text:&str)->Result<(),ValueError>{
        if self.refused{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"semantic extent output previously refused"))}
        if bytes>self.maximum_payload_bytes||bytes>N{self.refused=true;return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"formatted semantic extent exceeds full segment authority"))}
        let result=self.control.scoped_stage(|control|{control.begin_stage(1)?;control.step()});if result.is_err(){self.refused=true;}result
    }
    fn declare_extent(&mut self,bytes:usize)->Result<(),ValueError>{
        if self.refused||self.exact_bytes.is_some()||self.bytes.len()!=0{self.refused=true;return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"semantic output cannot replace its original final extent"))}
        if bytes>self.maximum_payload_bytes||bytes>N{self.refused=true;return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"semantic final extent exceeds original payload authority"))}
        if let Err(error)=self.control.checkpoint(){self.refused=true;return Err(error)}self.exact_bytes=Some(bytes);Ok(())
    }
}
