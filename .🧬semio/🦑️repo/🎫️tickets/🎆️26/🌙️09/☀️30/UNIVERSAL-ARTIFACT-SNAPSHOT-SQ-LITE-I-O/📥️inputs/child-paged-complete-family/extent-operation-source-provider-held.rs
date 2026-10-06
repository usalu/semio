//! 📦️ Operation octets with independently admitted and returned physical backing.

use crate::value::list::{PagedList, PagedListRefusalKind};
use crate::value::ValueRefusalKind;
use std::mem::ManuallyDrop;

const MAXIMUM_BYTES: usize = isize::MAX as usize;

/// ✍️ Writes source octets into caller-owned backing while retaining every accepted prefix.
pub trait OperationByteOutput {
    fn write_bytes(&mut self, bytes: &[u8], control: &mut crate::value::native_encoding::NativeEncodeControl<'_>) -> Result<(), crate::PackRefusal>;
}

/// 🧾 Applies the caller's cumulative allocation ceiling before any original operation work, restoring its control on every result.
pub fn with_operation_encode_policy<T,E:From<crate::PackRefusal>>(options:&crate::codec::PackEncodeOptions,control:&mut crate::value::NativeEncodeControl<'_>,operation:impl FnOnce(&mut crate::value::NativeEncodeControl<'_>)->Result<T,E>)->Result<T,E>{
    let maximum=options.limits.max_total_alloc.min(usize::MAX as u64)as usize;
    control.scoped_maximum(maximum,|control|Ok::<_,crate::PackRefusal>(operation(control))).map_err(E::from)?
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OperationByteFault {
    pub kind: ValueRefusalKind,
    pub reason: &'static str,
    pub allocated_bytes: usize,
}

impl OperationByteFault {
    fn refusal(kind: ValueRefusalKind, reason: &'static str) -> Self { Self { kind, reason, allocated_bytes: 0 } }
    fn paged(kind: PagedListRefusalKind, reason: &'static str, allocated_bytes: usize) -> Self {
        let kind = match kind {
            PagedListRefusalKind::OwnershipLimit => ValueRefusalKind::OwnershipLimit,
            PagedListRefusalKind::AllocationFailed => ValueRefusalKind::AllocationFailed,
            PagedListRefusalKind::InvariantViolated => ValueRefusalKind::InvariantViolated,
        };
        Self { kind, reason, allocated_bytes }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OperationByteAllocationStep { pub progressed: bool, pub allocated_bytes: usize }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OperationByteAppendRefusal { pub byte: u8, pub fault: OperationByteFault }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperationByteCloseStep { Pending { released_items: usize, released_bytes: usize }, Complete }

pub struct OwnedOperationBytes {
    bytes: ManuallyDrop<PagedList<u8, MAXIMUM_BYTES>>,
    maximum_payload_bytes: usize,
    maximum_allocation_bytes: usize,
    allocation_fault: Option<OperationByteFault>,
    closing: bool,
    closed: bool,
}

impl OwnedOperationBytes {
    /// 🎟️ Installs source payload and physical allocation authority without allocating.
    pub fn try_new(maximum_payload_bytes: usize, maximum_allocation_bytes: usize) -> Result<Self, OperationByteFault> {
        if maximum_payload_bytes == 0 || maximum_payload_bytes > MAXIMUM_BYTES {
            return Err(OperationByteFault::refusal(ValueRefusalKind::OwnershipLimit, "operation byte payload authority is out of range"));
        }
        if maximum_allocation_bytes == 0 || maximum_allocation_bytes > MAXIMUM_BYTES {
            return Err(OperationByteFault::refusal(ValueRefusalKind::OwnershipLimit, "operation byte backing authority is out of range"));
        }
        Ok(Self { bytes: ManuallyDrop::new(PagedList::default()), maximum_payload_bytes, maximum_allocation_bytes, allocation_fault: None, closing: false, closed: false })
    }

    pub fn len(&self) -> usize { self.bytes.len() }
    pub fn is_empty(&self) -> bool { self.bytes.is_empty() }
    pub fn allocated_bytes(&self) -> usize { self.bytes.allocated_bytes() }
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = u8> + ExactSizeIterator + '_ { self.bytes.iter().copied() }

    /// 📏️ Exposes the next complete physical allocation before source admission.
    pub fn next_allocation_bytes(&self) -> Result<usize, OperationByteFault> {
        if let Some(fault) = self.allocation_fault { return Err(fault); }
        if self.closing || self.closed { return Err(OperationByteFault::refusal(ValueRefusalKind::InvariantViolated, "operation byte owner is closing")); }
        if self.len() >= self.maximum_payload_bytes { return Err(OperationByteFault::refusal(ValueRefusalKind::OwnershipLimit, "operation byte payload authority exhausted")); }
        let requested = self.bytes.next_exact_capacity_allocation_bytes(self.maximum_payload_bytes).map_err(|error| OperationByteFault::paged(error.kind, error.reason, 0))?.ok_or(OperationByteFault::refusal(ValueRefusalKind::InvariantViolated, "operation payload extent has no unfunded page"))?;
        self.allocated_bytes().checked_add(requested).filter(|total| *total <= self.maximum_allocation_bytes).ok_or(OperationByteFault::refusal(ValueRefusalKind::OwnershipLimit, "operation byte backing authority exhausted"))?;
        Ok(requested)
    }

    /// 🧱️ Reserves one measured metadata or payload allocation under the current grant.
    pub fn reserve_one(&mut self, maximum_bytes: usize) -> Result<OperationByteAllocationStep, OperationByteFault> {
        if maximum_bytes == 0 { return Ok(OperationByteAllocationStep::default()); }
        let requested = self.next_allocation_bytes()?;
        if requested > maximum_bytes { return Ok(OperationByteAllocationStep::default()); }
        let remaining = self.maximum_allocation_bytes - self.allocated_bytes();
        let step = self.bytes.reserve_exact_capacity_one(self.maximum_payload_bytes, maximum_bytes.min(remaining)).map_err(|error| {
            let fault = OperationByteFault::paged(error.kind, error.reason, error.allocated_bytes);
            if error.allocated_bytes != 0 { self.allocation_fault = Some(fault); }
            fault
        })?;
        if self.allocated_bytes() > self.maximum_allocation_bytes {
            let fault = OperationByteFault { kind: ValueRefusalKind::OwnershipLimit, reason: "operation byte physical allocation exceeded authority; owner retained", allocated_bytes: step.allocated_bytes };
            self.allocation_fault = Some(fault);
            return Err(fault);
        }
        Ok(OperationByteAllocationStep { progressed: step.progressed, allocated_bytes: step.allocated_bytes })
    }

    pub fn has_reserved_slot(&self) -> bool { !self.closing && !self.closed && self.allocation_fault.is_none() && self.len() < self.maximum_payload_bytes && self.bytes.has_reserved_slot() }

    /// ➕️ Appends one source byte into admitted backing or returns the same rejected byte.
    pub fn push_reserved(&mut self, byte: u8) -> Result<(), OperationByteAppendRefusal> {
        let refused = |fault| OperationByteAppendRefusal { byte, fault };
        if let Some(fault) = self.allocation_fault { return Err(refused(fault)); }
        if self.closing || self.closed { return Err(refused(OperationByteFault::refusal(ValueRefusalKind::InvariantViolated, "operation byte owner is closing"))); }
        if self.len() >= self.maximum_payload_bytes { return Err(refused(OperationByteFault::refusal(ValueRefusalKind::OwnershipLimit, "operation byte payload authority exhausted"))); }
        self.bytes.push_reserved(byte).map_err(|byte| OperationByteAppendRefusal { byte, fault: OperationByteFault::refusal(ValueRefusalKind::InvariantViolated, "operation byte backing has not been admitted") })
    }

    pub fn byte_at(&self, offset: usize) -> Option<u8> { self.bytes.get(offset).copied() }
    pub fn byte_ref(&self, offset: usize) -> Option<&u8> { self.bytes.get(offset) }

    /// ♻️ Removes one logical byte or returns one complete empty physical backing allocation.
    pub fn close_one(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<OperationByteCloseStep, OperationByteFault> {
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(OperationByteCloseStep::Pending { released_items: 0, released_bytes: 0 }); }
        if self.closed { return Ok(OperationByteCloseStep::Complete); }
        self.closing = true;
        if self.bytes.pop().is_some() { return Ok(OperationByteCloseStep::Pending { released_items: 1, released_bytes: 0 }); }
        if self.bytes.terminal_is_empty() {
            unsafe { ManuallyDrop::drop(&mut self.bytes) };
            self.bytes = ManuallyDrop::new(PagedList::default());
            self.closed = true;
            return Ok(OperationByteCloseStep::Complete);
        }
        let required = self.bytes.next_release_allocation_bytes().map_err(|error| OperationByteFault::paged(error.kind, error.reason, 0))?;
        if required > maximum_bytes { return Ok(OperationByteCloseStep::Pending { released_items: 0, released_bytes: 0 }); }
        let step = self.bytes.release_empty_page(maximum_bytes).map_err(|error| OperationByteFault::paged(error.kind, error.reason, 0))?;
        Ok(OperationByteCloseStep::Pending { released_items: usize::from(step.progressed), released_bytes: step.released_allocation_bytes })
    }

    /// 🏡️ Transfers source backing to the actual parent while retaining separate physical disposal responsibility.
    pub fn return_one<const P:usize>(&mut self,parent:&mut crate::value::retirement::allocation_return::ParentAllocationReturn<P>,maximum_items:usize,maximum_bytes:usize)->Result<OperationByteReturnStep,crate::value::ValueError>{
        if maximum_items==0||maximum_bytes==0{return Ok(OperationByteReturnStep::Pending{returned_items:0,returned_bytes:0});}
        if self.closed{return Ok(OperationByteReturnStep::Complete);}
        self.closing=true;
        if self.bytes.pop().is_some(){return Ok(OperationByteReturnStep::Pending{returned_items:1,returned_bytes:0});}
        if self.bytes.terminal_is_empty(){
            unsafe{ManuallyDrop::drop(&mut self.bytes)};
            self.bytes=ManuallyDrop::new(PagedList::empty());self.closed=true;
            return Ok(OperationByteReturnStep::Complete);
        }
        let progress=self.bytes.return_empty_page(parent,1)?;
        Ok(OperationByteReturnStep::Pending{returned_items:usize::from(progress.progressed),returned_bytes:progress.returned_allocation_bytes})
    }


    pub fn next_close_byte_demand(&self) -> Result<usize, OperationByteFault> {
        if self.closed || !self.bytes.is_empty() || self.bytes.terminal_is_empty() { return Ok(1); }
        self.bytes.next_release_allocation_bytes().map_err(|error| OperationByteFault::paged(error.kind, error.reason, 0))
    }

    pub fn terminal_is_empty(&self) -> bool { self.closed }
}

impl Drop for OwnedOperationBytes {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.closed, "operation byte owner requires explicit bounded terminal retirement"); }
}

impl OperationByteOutput for OwnedOperationBytes {
    fn write_bytes(&mut self, bytes: &[u8], control: &mut crate::value::native_encoding::NativeEncodeControl<'_>) -> Result<(), crate::PackRefusal> {
        let refusal = |fault: OperationByteFault| crate::PackRefusal::from(crate::value::ValueError::new(fault.kind, fault.reason));
        control.scoped_stage(|control| {
            control.begin_stage(bytes.len())?;
            for &byte in bytes {
                while !self.has_reserved_slot() {
                    let required = self.next_allocation_bytes().map_err(refusal)?;
                    if required > 4096 { return Err(crate::PackRefusal::from(crate::value::ValueError::new(ValueRefusalKind::OwnershipLimit, "operation byte allocation exceeds its fixed page grant"))); }
                    control.checkpoint()?;
                    control.charge(required)?;
                    let progress = self.reserve_one(4096).map_err(refusal)?;
                    if !progress.progressed { return Err(crate::PackRefusal::from(crate::value::ValueError::new(ValueRefusalKind::InvariantViolated, "operation byte admitted allocation made no progress"))); }
                }
                self.push_reserved(byte).map_err(|error| refusal(error.fault))?;
                control.step()?;
            }
            Ok(())
        })
    }
}

impl serde::Serialize for OwnedOperationBytes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut sequence = serializer.serialize_seq(Some(self.len()))?;
        for byte in self.iter() { sequence.serialize_element(&byte)?; }
        sequence.end()
    }
}

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
        control.scoped_stage(|control|{
        if let Err(error)=control.begin_stage(bytes.len()){self.refused=true;return Err(error.into());}
        for byte in bytes{
            if let Err(error)=control.checkpoint(){self.refused=true;return Err(error.into());}
            if self.source.get(self.position)!=Some(byte){self.refused=true;return Err(Self::mismatch(self.position));}
            self.position+=1;
            if let Err(error)=control.step(){self.refused=true;return Err(error.into());}
        }
        Ok(())
        })
    }
}

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

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OperationByteReturnStep{Pending{returned_items:usize,returned_bytes:usize},Complete}

/// 🧭️ Reads original operation sources without materializing another payload owner.
pub trait OperationSourceCollection:Send+Sync{
    fn len(&self)->usize;
    fn source_at(&self,index:usize)->Option<crate::codec::ByteSpan<'_>>;
    fn is_empty(&self)->bool{self.len()==0}
}
impl<const N:usize> OperationSourceCollection for crate::value::list::PagedList<OwnedOperationBytes,N>{
    fn len(&self)->usize{crate::value::list::PagedList::len(self)}
    fn source_at(&self,index:usize)->Option<crate::codec::ByteSpan<'_>>{self.get(index).map(crate::codec::ByteSpan::from_source)}
}

