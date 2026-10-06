//! 🧹️ Borrowed contributed conformance inputs and bounded admission continuation.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConformanceInputStatus { Ready, Missing, Empty, Stub, Invalid }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConformanceFacet { Grammar, Protocol }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConformanceSpecimen { Text, Binary }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConformanceInput<'a> {
    pub id: &'a str,
    pub owner_present: bool,
    pub facet: ConformanceFacet,
    pub specimen: ConformanceSpecimen,
    pub specification_status: ConformanceInputStatus,
    pub specimen_status: ConformanceInputStatus,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConformanceOutcome { Ready, OwnerAbsent, KindMismatch, Specification(ConformanceInputStatus), Specimen(ConformanceInputStatus) }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConformanceAdmission<'a> { pub input: &'a ConformanceInput<'a>, pub outcome: ConformanceOutcome }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConformanceControlRefusal { Cancelled, Budget }
pub trait ConformanceOperationControl {
    fn checkpoint(&mut self, completed: usize, total: usize) -> Result<(), ConformanceControlRefusal>;
}
pub fn admit_conformance_input<'a>(input: &'a ConformanceInput<'a>) -> ConformanceAdmission<'a> {
    let outcome = if !input.owner_present { ConformanceOutcome::OwnerAbsent }
    else if (input.facet == ConformanceFacet::Grammar) != (input.specimen == ConformanceSpecimen::Text) { ConformanceOutcome::KindMismatch }
    else if input.specification_status != ConformanceInputStatus::Ready { ConformanceOutcome::Specification(input.specification_status) }
    else if input.specimen_status != ConformanceInputStatus::Ready { ConformanceOutcome::Specimen(input.specimen_status) }
    else { ConformanceOutcome::Ready };
    ConformanceAdmission { input, outcome }
}
pub struct ConformanceCursor<'a> { inputs: &'a [ConformanceInput<'a>], completed: usize }
impl<'a> ConformanceCursor<'a> {
    pub fn new(inputs: &'a [ConformanceInput<'a>]) -> Self { Self { inputs, completed: 0 } }
    pub fn completed(&self) -> usize { self.completed }
    pub fn complete(&self) -> bool { self.completed == self.inputs.len() }
    pub fn advance<C: ConformanceOperationControl, R: FnMut(ConformanceAdmission<'a>)>(&mut self, maximum_units: usize, control: &mut C, receive: &mut R) -> Result<bool, ConformanceControlRefusal> {
        if maximum_units == 0 { return Err(ConformanceControlRefusal::Budget); }
        for _ in 0..maximum_units {
            if self.complete() { break; }
            control.checkpoint(self.completed, self.inputs.len())?;
            receive(admit_conformance_input(&self.inputs[self.completed]));
            self.completed += 1;
        }
        Ok(self.complete())
    }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
