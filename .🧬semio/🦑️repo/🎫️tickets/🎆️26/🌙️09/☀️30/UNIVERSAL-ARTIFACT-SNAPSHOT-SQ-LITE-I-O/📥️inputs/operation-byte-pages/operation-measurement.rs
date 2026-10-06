
/// 📐️ Counts complete producer output without retaining octets or allocating payload backing.
pub struct OperationByteMeasurement{maximum_length:u64,length:u64,refused:bool}
impl OperationByteMeasurement{
    pub fn new(maximum_length:u64)->Self{Self{maximum_length,length:0,refused:false}}
    fn refusal(&self,kind:ValueRefusalKind,detail:&'static str)->crate::PackRefusal{
        crate::PackRefusal::RetainedMalformed{kind,what:"operation output measurement",offset:self.length,detail}
    }
    pub fn exact_length(&self)->Result<usize,crate::PackRefusal>{
        if self.refused{return Err(self.refusal(ValueRefusalKind::InvariantViolated,"refused measurement cannot issue a funding length"));}
        usize::try_from(self.length).ok().filter(|length|*length<=MAXIMUM_BYTES).ok_or_else(||self.refusal(ValueRefusalKind::OwnershipLimit,"operation measurement exceeds source container authority"))
    }
}
impl OperationByteOutput for OperationByteMeasurement{
    fn write_bytes(&mut self,bytes:&[u8],control:&mut crate::value::NativeEncodeControl<'_>)->Result<(),crate::PackRefusal>{
        if self.refused{return Err(self.refusal(ValueRefusalKind::InvariantViolated,"operation measurement is refused"));}
        let Some(next)=self.length.checked_add(bytes.len()as u64).filter(|length|*length<=self.maximum_length&&*length<=MAXIMUM_BYTES as u64)else{
            self.refused=true;return Err(self.refusal(ValueRefusalKind::OwnershipLimit,"complete operation exceeds measured byte authority"));
        };
        let result=control.scoped_stage(|control|{control.begin_stage(bytes.len())?;control.advance(bytes.len())?;Ok::<_,crate::PackRefusal>(())});
        if let Err(error)=result{self.refused=true;return Err(error);}
        self.length=next;Ok(())
    }
}
