    fn copy_span_utf8(&self,bytes:crate::ByteSpan<'_>,offset:u64,what:&'static str)->Result<String,PackRefusal>{
        if let Some(bytes)=bytes.contiguous(){return self.copy_utf8(bytes,offset,what)}
        self.check(bytes.len()as u64)?;
        {let control=self.control.borrow();if bytes.len()>control.maximum_bytes().saturating_sub(control.owned_bytes()){return Err(PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::OwnershipLimit,"native text exceeds caller allowance")))}}
        copy_validated_span_utf8(self,bytes,offset,what)
    }
