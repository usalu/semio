//! 📦️ Operation octets with independently admitted and returned physical backing.

use crate::value::list::{PagedList, PagedListRefusalKind};
use crate::value::ValueRefusalKind;
use std::mem::ManuallyDrop;

const MAXIMUM_BYTES: usize = isize::MAX as usize;

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
        let requested = self.bytes.next_allocation_bytes().map_err(|error| OperationByteFault::paged(error.kind, error.reason, 0))?;
        self.allocated_bytes().checked_add(requested).filter(|total| *total <= self.maximum_allocation_bytes).ok_or(OperationByteFault::refusal(ValueRefusalKind::OwnershipLimit, "operation byte backing authority exhausted"))?;
        Ok(requested)
    }

    /// 🧱️ Reserves one measured metadata or payload allocation under the current grant.
    pub fn reserve_one(&mut self, maximum_bytes: usize) -> Result<OperationByteAllocationStep, OperationByteFault> {
        if maximum_bytes == 0 { return Ok(OperationByteAllocationStep::default()); }
        let requested = self.next_allocation_bytes()?;
        if requested > maximum_bytes { return Ok(OperationByteAllocationStep::default()); }
        let remaining = self.maximum_allocation_bytes - self.allocated_bytes();
        let step = self.bytes.reserve_one(maximum_bytes.min(remaining)).map_err(|error| {
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

    pub fn next_close_byte_demand(&self) -> Result<usize, OperationByteFault> {
        if self.closed || !self.bytes.is_empty() || self.bytes.terminal_is_empty() { return Ok(1); }
        self.bytes.next_release_allocation_bytes().map_err(|error| OperationByteFault::paged(error.kind, error.reason, 0))
    }

    pub fn terminal_is_empty(&self) -> bool { self.closed }
}

impl Drop for OwnedOperationBytes {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.closed, "operation byte owner requires explicit bounded terminal retirement"); }
}

impl serde::Serialize for OwnedOperationBytes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut sequence = serializer.serialize_seq(Some(self.len()))?;
        for byte in self.iter() { sequence.serialize_element(&byte)?; }
        sequence.end()
    }
}
