/// 🎚️ Applies one caller-owned complete operation ceiling before each source fragment handoff.
/// Completed writes are counted exactly; the underlying owner retains any partial refused write.
pub struct OperationByteLimitedOutput<'output>{output:&'output mut dyn OperationByteOutput,maximum_bytes:u64,completed_bytes:u64,refused:bool}

impl<'output> OperationByteLimitedOutput<'output>{
    pub fn new(output:&'output mut dyn OperationByteOutput,maximum_bytes:u64)->Self{Self{output,maximum_bytes,completed_bytes:0,refused:false}}
    pub fn completed_bytes(&self)->u64{self.completed_bytes}
    fn refusal(&self)->crate::PackRefusal{crate::PackRefusal::RetainedMalformed{kind:ValueRefusalKind::OwnershipLimit,what:"operation output byte length",offset:self.completed_bytes,detail:"complete operation output exceeds caller policy or follows a refused handoff"}}
}

impl OperationByteOutput for OperationByteLimitedOutput<'_>{
    fn write_bytes(&mut self,bytes:&[u8],control:&mut crate::value::NativeEncodeControl<'_>)->Result<(),crate::PackRefusal>{
        if self.refused{return Err(self.refusal());}
        let Some(next)=self.completed_bytes.checked_add(bytes.len()as u64).filter(|length|*length<=self.maximum_bytes)else{self.refused=true;return Err(self.refusal());};
        if let Err(error)=control.checkpoint(){self.refused=true;return Err(error.into());}
        if let Err(error)=self.output.write_bytes(bytes,control){self.refused=true;return Err(error);}
        self.completed_bytes=next;
        Ok(())
    }
}
