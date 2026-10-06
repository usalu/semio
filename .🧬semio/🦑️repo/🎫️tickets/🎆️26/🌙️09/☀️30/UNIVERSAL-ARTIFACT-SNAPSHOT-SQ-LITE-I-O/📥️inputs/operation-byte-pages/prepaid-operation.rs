
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OperationByteFundingStep{Pending{prepared_items:usize,allocated_bytes:usize},Ready}

/// 🪙️ Retains initialized backing across funding hops and exposes only the actual emitted prefix.
pub struct OperationBytePreparation{
    source:Option<OwnedOperationBytes>,
    exact_length:usize,
    completed:usize,
    funding_started:bool,
    refused:bool,
}
impl OperationBytePreparation{
    pub fn try_new(exact_length:usize,maximum_allocation_bytes:usize)->Result<Self,OperationByteFault>{
        Ok(Self{source:Some(OwnedOperationBytes::try_new(exact_length.max(1),maximum_allocation_bytes)?),exact_length,completed:0,funding_started:false,refused:false})
    }
    pub fn allocated_bytes(&self)->usize{self.source.as_ref().map_or(0,OwnedOperationBytes::allocated_bytes)}
    pub fn is_funded(&self)->bool{!self.refused&&self.source.as_ref().is_some_and(|source|!source.closing&&!source.closed&&source.len()==self.exact_length)}
    pub fn accepted_prefix(&self)->Option<crate::codec::ByteSpan<'_>>{
        let source=self.source.as_ref()?;
        crate::codec::ByteSpan::from_source(source).slice(0,self.completed.min(source.len())).ok()
    }
    fn invalid(what:&'static str,kind:ValueRefusalKind,offset:usize)->crate::PackRefusal{
        crate::PackRefusal::RetainedMalformed{kind,what,offset:offset as u64,detail:"operation preparation retains its exact backing and prefix"}
    }
    pub fn next_funding_byte_demand(&self)->Result<usize,OperationByteFault>{
        if self.is_funded(){return Ok(0);}
        let source=self.source.as_ref().ok_or(OperationByteFault::refusal(ValueRefusalKind::InvariantViolated,"operation preparation owner was returned"))?;
        if self.refused{return Err(OperationByteFault::refusal(ValueRefusalKind::InvariantViolated,"operation preparation is refused"));}
        if source.has_reserved_slot(){return Ok(1);}
        source.next_allocation_bytes()
    }
    pub fn fund_one(&mut self,maximum_items:usize,maximum_bytes:usize,control:&mut crate::value::NativeEncodeControl<'_>)->Result<OperationByteFundingStep,crate::PackRefusal>{
        if maximum_items==0||maximum_bytes==0{return Ok(OperationByteFundingStep::Pending{prepared_items:0,allocated_bytes:0});}
        if self.is_funded(){return Ok(OperationByteFundingStep::Ready);}
        let required=self.next_funding_byte_demand().map_err(|error|Self::invalid(error.reason,error.kind,self.completed))?;
        if required>maximum_bytes{return Ok(OperationByteFundingStep::Pending{prepared_items:0,allocated_bytes:0});}
        let result=(||{
            if !self.funding_started{control.begin_stage(self.exact_length)?;self.funding_started=true;}
            let source=self.source.as_mut().ok_or_else(||Self::invalid("operation preparation owner",ValueRefusalKind::InvariantViolated,self.completed))?;
            if !source.has_reserved_slot(){
                control.checkpoint()?;
                control.charge(required)?;
                let step=source.reserve_one(maximum_bytes).map_err(|error|Self::invalid(error.reason,error.kind,self.completed))?;
                if !step.progressed{return Err(Self::invalid("operation preparation funding",ValueRefusalKind::InvariantViolated,self.completed));}
                return Ok(OperationByteFundingStep::Pending{prepared_items:0,allocated_bytes:step.allocated_bytes});
            }
            source.push_reserved(0).map_err(|error|Self::invalid(error.fault.reason,error.fault.kind,self.completed))?;
            control.step()?;
            Ok(OperationByteFundingStep::Pending{prepared_items:1,allocated_bytes:0})
        })();
        if result.is_err(){self.refused=true;}
        result
    }
    pub fn take_ready(&mut self)->Option<OwnedOperationBytes>{
        if !self.is_funded()||self.completed!=self.exact_length{return None;}
        self.source.as_mut()?.maximum_payload_bytes=self.exact_length;
        self.source.take()
    }
    pub fn close_one(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<OperationByteCloseStep,OperationByteFault>{
        let Some(source)=self.source.as_mut()else{return Ok(OperationByteCloseStep::Complete)};
        source.close_one(maximum_items,maximum_bytes)
    }
    pub fn terminal_is_empty(&self)->bool{self.source.as_ref().is_none_or(OwnedOperationBytes::terminal_is_empty)}
}
impl OperationByteOutput for OperationBytePreparation{
    fn write_bytes(&mut self,bytes:&[u8],control:&mut crate::value::NativeEncodeControl<'_>)->Result<(),crate::PackRefusal>{
        if !self.is_funded(){return Err(Self::invalid("operation preparation funding",ValueRefusalKind::InvariantViolated,self.completed));}
        if self.completed.checked_add(bytes.len()).is_none_or(|end|end>self.exact_length){self.refused=true;return Err(Self::invalid("operation preparation measured byte length",ValueRefusalKind::OwnershipLimit,self.completed));}
        let result=control.scoped_stage(|control|{
            control.begin_stage(bytes.len())?;
            let source=self.source.as_mut().ok_or_else(||Self::invalid("operation preparation owner",ValueRefusalKind::InvariantViolated,self.completed))?;
            for byte in bytes{
                let cell=source.bytes.get_mut(self.completed).ok_or_else(||Self::invalid("operation preparation paid byte cell",ValueRefusalKind::InvariantViolated,self.completed))?;
                *cell=*byte;self.completed+=1;control.step()?;
            }
            Ok(())
        });
        if result.is_err(){self.refused=true;}
        result
    }
}
