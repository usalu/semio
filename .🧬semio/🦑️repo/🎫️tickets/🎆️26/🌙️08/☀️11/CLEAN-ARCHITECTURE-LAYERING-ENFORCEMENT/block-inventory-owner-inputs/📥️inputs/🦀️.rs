//! 🏭️ Controlled mutation inventories supplied by their production owners.
use std::fmt;
/// 🧭️ Exact manifest coordinate and descriptor owner scope.
#[derive(Clone, Copy)]
pub struct MutationInventoryCoordinate<'a> {
    pub artifact: &'a str, pub standard: &'a str, pub subset: &'a str,
    pub surface: Option<&'a str>, pub owner: &'a str,
}
/// 🧷️ Immutable borrowed production leaf with a fixed closed outcome roster.
#[derive(Clone, Copy)]
pub struct MutationInventoryLeaf<'a> {
    pub owner: &'a str, pub id: &'a str, pub variant: &'a str,
    pub outcomes: [MutationInventoryOutcome; 5], pub outcome_count: usize,
}
/// 🎚️ Closed outcome vocabulary of the runtime inventory wire schema.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MutationInventoryOutcome { Applied, NoOp, Empty, Disjoint, Rejected }
impl MutationInventoryOutcome {
    fn text(self) -> &'static str {
        match self { Self::Applied => "applied", Self::NoOp => "no-op", Self::Empty => "empty", Self::Disjoint => "disjoint", Self::Rejected => "rejected" }
    }
}
/// 📏️ Mandatory work and owned output capacity limits.
#[derive(Clone, Copy)]
pub struct MutationInventoryBudget { pub maximum_units: u64, pub maximum_owned_bytes: usize }
/// 📣️ Monotonic observation offered before every bounded unit of output or source work.
#[derive(Clone, Copy, Debug)]
pub struct MutationInventoryProgress { pub completed_units: u64, pub emitted_bytes: usize, pub owned_bytes: usize }
/// 🚫️ Owned refusal vocabulary, including caller cancellation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MutationInventoryError { Policy, Coordinate, Descriptor, WorkBudget, ByteBudget, Allocation, Cancelled, Output }
impl fmt::Display for MutationInventoryError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result { write!(output, "{self:?}") }
}
impl std::error::Error for MutationInventoryError {}
struct Encoder<'a, F> {
    output: Vec<u8>, budget: MutationInventoryBudget, units: u64, observe: &'a mut F,
}
impl<F: FnMut(MutationInventoryProgress) -> Result<(), MutationInventoryError>> Encoder<'_, F> {
    fn checkpoint(&mut self) -> Result<(), MutationInventoryError> {
        (self.observe)(MutationInventoryProgress { completed_units: self.units, emitted_bytes: self.output.len(), owned_bytes: self.output.capacity() })
    }
    fn charge(&mut self) -> Result<(), MutationInventoryError> {
        self.units = self.units.checked_add(1).ok_or(MutationInventoryError::WorkBudget)?;
        if self.units > self.budget.maximum_units { return Err(MutationInventoryError::WorkBudget); }
        self.checkpoint()
    }
    fn bytes(&mut self, bytes: &[u8]) -> Result<(), MutationInventoryError> {
        for byte in bytes {
            self.charge()?;
            if self.output.len() == self.budget.maximum_owned_bytes { return Err(MutationInventoryError::ByteBudget); }
            self.output.push(*byte);
        }
        Ok(())
    }
    fn string(&mut self, text: &str) -> Result<(), MutationInventoryError> {
        self.bytes(b"\"")?;
        for byte in text.bytes() {
            match byte {
                b'"' => self.bytes(b"\\\"")?, b'\\' => self.bytes(b"\\\\")?,
                b'\n' => self.bytes(b"\\n")?, b'\r' => self.bytes(b"\\r")?, b'\t' => self.bytes(b"\\t")?,
                8 => self.bytes(b"\\b")?, 12 => self.bytes(b"\\f")?,
                0..=31 => {
                    let digits = b"0123456789abcdef";
                    self.bytes(&[b'\\', b'u', b'0', b'0', digits[(byte >> 4) as usize], digits[(byte & 15) as usize]])?;
                }
                _ => self.bytes(&[byte])?,
            }
        }
        self.bytes(b"\"")
    }
    fn equal(&mut self, left: &str, right: &str) -> Result<bool, MutationInventoryError> {
        self.charge()?;
        if left.len() != right.len() { return Ok(false); }
        for (left, right) in left.bytes().zip(right.bytes()) { self.charge()?; if left != right { return Ok(false); } }
        Ok(true)
    }
    fn identifier(&mut self, text: &str, dotted: bool) -> Result<bool, MutationInventoryError> {
        let mut previous_separator = true;
        for byte in text.bytes() {
            self.charge()?;
            if byte.is_ascii_lowercase() || byte.is_ascii_digit() { previous_separator = false; }
            else if (byte == b'-' || dotted && byte == b'.') && !previous_separator { previous_separator = true; }
            else { return Ok(false); }
        }
        Ok(!previous_separator)
    }
    fn path(&mut self, text: &str) -> Result<bool, MutationInventoryError> {
        let mut start = 0;
        for (index, byte) in text.bytes().enumerate() {
            self.charge()?;
            if matches!(byte, b'\\' | b':' | 0) { return Ok(false); }
            if byte == b'/' {
                let part = &text[start..index];
                if part.is_empty() || self.equal(part, ".")? || self.equal(part, "..")? { return Ok(false); }
                start = index + 1;
            }
        }
        let part = &text[start..];
        Ok(!part.is_empty() && !self.equal(part, ".")? && !self.equal(part, "..")?)
    }
    fn eligible(&mut self, leaf: &MutationInventoryLeaf<'_>, coordinate: &MutationInventoryCoordinate<'_>, excluded: &[&str]) -> Result<bool, MutationInventoryError> {
        if !self.path(leaf.owner)? { return Err(MutationInventoryError::Descriptor); }
        for (index, byte) in coordinate.owner.bytes().enumerate() {
            self.charge()?;
            if leaf.owner.as_bytes().get(index) != Some(&byte) { return Ok(false); }
        }
        self.charge()?;
        if leaf.owner.as_bytes().get(coordinate.owner.len()) != Some(&b'/') { return Ok(false); }
        let local = &leaf.owner[coordinate.owner.len() + 1..];
        if local.is_empty() { return Ok(false); }
        let mut start = 0;
        for (index, byte) in local.bytes().enumerate() {
            self.charge()?;
            if byte == b'/' {
                for omitted in excluded { if self.equal(&local[start..index], omitted)? { return Ok(false); } }
                start = index + 1;
            }
        }
        for omitted in excluded { if self.equal(&local[start..], omitted)? { return Ok(false); } }
        Ok(true)
    }
}
/// 🧮️ Encodes exactly the immutable supplied contribution within the caller's limits.
pub fn encode_mutation_inventory<F: FnMut(MutationInventoryProgress) -> Result<(), MutationInventoryError>>(
    coordinate: &MutationInventoryCoordinate<'_>, produced_by: &str, sources: &[&[MutationInventoryLeaf<'_>]],
    excluded_owner_segments: &[&str], budget: MutationInventoryBudget, observe: &mut F,
) -> Result<Vec<u8>, MutationInventoryError> {
    if budget.maximum_units == 0 || budget.maximum_owned_bytes == 0 || budget.maximum_units > 9_007_199_254_740_991 || budget.maximum_owned_bytes as u128 > 9_007_199_254_740_991 { return Err(MutationInventoryError::Policy); }
    let mut encoder = Encoder { output: Vec::new(), budget, units: 0, observe };
    encoder.checkpoint()?;
    encoder.output.try_reserve_exact(budget.maximum_owned_bytes).map_err(|_| MutationInventoryError::Allocation)?;
    if encoder.output.capacity() > budget.maximum_owned_bytes { return Err(MutationInventoryError::ByteBudget); }
    encoder.checkpoint()?;
    if !encoder.identifier(coordinate.artifact, true)? || !encoder.identifier(coordinate.subset, false)? ||
        coordinate.standard.is_empty() || produced_by.is_empty() || coordinate.surface == Some("") || !encoder.path(coordinate.owner)? { return Err(MutationInventoryError::Coordinate); }
    for (index, segment) in excluded_owner_segments.iter().enumerate() {
        if !encoder.path(segment)? { return Err(MutationInventoryError::Coordinate); }
        for byte in segment.bytes() { encoder.charge()?; if byte == b'/' { return Err(MutationInventoryError::Coordinate); } }
        for prior in &excluded_owner_segments[..index] { if encoder.equal(segment, prior)? { return Err(MutationInventoryError::Coordinate); } }
    }
    encoder.bytes(b"{\"schema\":\"semio.repository-test.runtime-inventory/v2\",\"artifact\":")?;
    encoder.string(coordinate.artifact)?;
    encoder.bytes(b",\"standard\":")?; encoder.string(coordinate.standard)?;
    encoder.bytes(b",\"subset\":")?; encoder.string(coordinate.subset)?;
    if let Some(surface) = coordinate.surface { encoder.bytes(b",\"surface\":")?; encoder.string(surface)?; }
    encoder.bytes(b",\"bridgeVersion\":1,\"producedBy\":")?; encoder.string(produced_by)?;
    encoder.bytes(b",\"mutations\":[")?;
    let mut emitted = false;
    for (source_index, source) in sources.iter().enumerate() {
        encoder.charge()?;
        for (index, leaf) in source.iter().enumerate() {
            encoder.charge()?;
            if !encoder.eligible(leaf, coordinate, excluded_owner_segments)? { continue; }
            if !encoder.identifier(leaf.id, false)? || !(1..=5).contains(&leaf.outcome_count) { return Err(MutationInventoryError::Descriptor); }
            for outcome_index in 0..leaf.outcome_count {
                encoder.charge()?;
                for prior in &leaf.outcomes[..outcome_index] { encoder.charge()?; if *prior == leaf.outcomes[outcome_index] { return Err(MutationInventoryError::Descriptor); } }
            }
            let mut duplicate = false;
            for (prior_source_index, prior_source) in sources.iter().take(source_index + 1).enumerate() {
                encoder.charge()?;
                let prior_length = if prior_source_index == source_index { index } else { prior_source.len() };
                for prior in &prior_source[..prior_length] {
                    encoder.charge()?;
                    if encoder.equal(prior.id, leaf.id)? && encoder.eligible(prior, coordinate, excluded_owner_segments)? {
                        if !encoder.equal(prior.variant, leaf.variant)? || prior.outcome_count != leaf.outcome_count { return Err(MutationInventoryError::Descriptor); }
                        for outcome_index in 0..leaf.outcome_count { encoder.charge()?; if prior.outcomes[outcome_index] != leaf.outcomes[outcome_index] { return Err(MutationInventoryError::Descriptor); } }
                        duplicate = true; break;
                    }
                }
                if duplicate { break; }
            }
            if duplicate { continue; }
            if emitted { encoder.bytes(b",")?; }
            encoder.bytes(b"{\"id\":")?; encoder.string(leaf.id)?;
            encoder.bytes(b",\"variant\":")?; encoder.string(leaf.variant)?;
            encoder.bytes(b",\"outcomes\":[")?;
            for (index, outcome) in leaf.outcomes[..leaf.outcome_count].iter().enumerate() { encoder.charge()?; if index > 0 { encoder.bytes(b",")?; } encoder.string(outcome.text())?; }
            encoder.bytes(b"]}")?;
            emitted = true;
        }
    }
    encoder.bytes(b"]}")?;
    encoder.checkpoint()?;
    Ok(encoder.output)
}
/// 📦️ Owner-supplied coordinate, immutable aggregates and lane omissions.
pub struct MutationInventoryContribution<'a> {
    pub coordinate: MutationInventoryCoordinate<'a>,
    pub aggregates: &'a [&'a [MutationInventoryLeaf<'a>]],
    pub excluded_owner_segments: &'a [&'a str],
}
#[path = "🚪️command/🦀️.rs"]
mod command;
pub use command::inventory_for_command;
#[cfg(feature = "replication")]
#[path = "🔌️replication/🦀️.rs"]
mod replication;
#[cfg(feature = "replication")]
pub use replication::replication_inventory_leaves;
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
