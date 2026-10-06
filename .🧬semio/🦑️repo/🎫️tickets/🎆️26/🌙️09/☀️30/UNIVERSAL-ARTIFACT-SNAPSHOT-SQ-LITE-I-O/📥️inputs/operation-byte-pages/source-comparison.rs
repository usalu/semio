/// 🪞️ Compares controlled producer octets with one exact immutable source span.
pub struct OperationByteComparison<'source> { source:crate::codec::ByteSpan<'source>, position:usize, refused:bool }

impl<'source> OperationByteComparison<'source> {
    pub fn new(source:crate::codec::ByteSpan<'source>)->Self{Self{source,position:0,refused:false}}
    pub fn position(&self)->usize{self.position}
    pub fn finish(self)->Result<(),crate::PackRefusal>{
        if self.refused||self.position!=self.source.len(){return Err(Self::mismatch(self.position));}
        Ok(())
    }
    fn mismatch(offset:usize)->crate::PackRefusal{crate::PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvalidValue,what:"operation canonical encoding",offset:offset as u64,detail:"producer octets differ from the exact immutable source"}}
}

impl OperationByteOutput for OperationByteComparison<'_> {
    fn write_bytes(&mut self,bytes:&[u8],control:&mut crate::value::native_encoding::NativeEncodeControl<'_>)->Result<(),crate::PackRefusal>{
        if self.refused{return Err(Self::mismatch(self.position));}
        if let Err(error)=control.begin_stage(bytes.len()){self.refused=true;return Err(error.into());}
        for byte in bytes{
            if let Err(error)=control.checkpoint(){self.refused=true;return Err(error.into());}
            if self.source.get(self.position)!=Some(byte){self.refused=true;return Err(Self::mismatch(self.position));}
            self.position+=1;
            if let Err(error)=control.step(){self.refused=true;return Err(error.into());}
        }
        Ok(())
    }
}
