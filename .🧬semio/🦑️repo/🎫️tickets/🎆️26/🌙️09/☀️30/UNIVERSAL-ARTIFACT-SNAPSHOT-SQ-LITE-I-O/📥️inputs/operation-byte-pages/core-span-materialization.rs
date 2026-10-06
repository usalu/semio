    fn copy_span_bytes(&self,bytes:crate::ByteSpan<'_>)->Result<Vec<u8>,PackRefusal>{
        if let Some(bytes)=bytes.contiguous(){return self.copy_bytes(bytes)}
        let mut output=self.octet_buffer(bytes.len())?;
        for byte in bytes.iter(){self.step()?;output.push(byte);}
        Ok(output)
    }
    fn check_span_utf8(&self,bytes:crate::ByteSpan<'_>,offset:u64,what:&'static str)->Result<(),PackRefusal>{
        let invalid=||PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvalidValue,what,offset,detail:"invalid utf8"};
        let mut position=0;
        while position<bytes.len(){
            self.step()?;
            let first=bytes[position];
            let length=match first{0..=127=>1,194..=223=>2,224..=239=>3,240..=244=>4,_=>return Err(invalid())};
            if position+length>bytes.len(){return Err(invalid())}
            let mut word=[0;4];
            for(index,byte)in word[..length].iter_mut().enumerate(){*byte=bytes[position+index];}
            std::str::from_utf8(&word[..length]).map_err(|_|invalid())?;
            position+=length;
        }
        Ok(())
    }
    fn copy_span_utf8(&self,bytes:crate::ByteSpan<'_>,offset:u64,what:&'static str)->Result<String,PackRefusal>{
        if let Some(bytes)=bytes.contiguous(){return self.copy_utf8(bytes,offset,what)}
        copy_validated_span_utf8(self,bytes,offset,what)
    }
