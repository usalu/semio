//! 🔑️ Plans unique XML identifiers from borrowed, normalized names under original indexed custody.

use super::GbxmlText;
use semio_framework_job::StepContext;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_os_kernel::{InferenceValues, InferenceWorkFailure};

const HASH_SEED: u64 = 0xcbf29ce484222325;
const CHARACTER_WORK: usize = 64;
const NO_SUFFIX: usize = 1;

/// 🧷️ Lends each proposed, normalized base name in the canonical allocation order.
pub trait GbxmlNameSource {
    fn name_source_identity(&self) -> usize;
    fn name_count(&self) -> usize;
    fn with_name<R>(&self, index: usize, read: impl FnOnce(GbxmlText<'_>) -> R) -> Option<R>;
}

#[derive(Clone, Copy, semio_framework_value::RetireOwned)]
struct Entry { ordinal: usize, suffix: usize }

#[derive(Clone, Copy, Default)]
struct Position { raw: usize, suffix: usize }

enum Character { Value(char), Skipped, End }

impl Position {
    fn next(&mut self, raw: GbxmlText<'_>, suffix: usize) -> Character {
        if let Some((value, length)) = raw.next(self.raw) {
            self.raw += length;
            return value.map_or(Character::Skipped, Character::Value);
        }
        if suffix == NO_SUFFIX { return Character::End; }
        let mut counter = suffix;
        let mut place = 1;
        while counter >= 10 { counter /= 10; place *= 10; }
        if self.suffix == 0 { self.suffix = 1; return Character::Value('-'); }
        for _ in 1..self.suffix {
            place /= 10;
            if place == 0 { return Character::End; }
        }
        self.suffix += 1;
        Character::Value(char::from(b'0' + (suffix / place % 10) as u8))
    }
}

enum Phase {
    Hash,
    Check { bucket: usize },
    Compare { bucket: usize, other: Entry, prior: Position, pending: Option<char> },
    PublishBucket { bucket: usize, entry: Option<Entry> },
    PublishAssignment { suffix: Option<usize> },
}

/// 🗂️ Owns the original collision table and one scalar suffix per source name.
pub struct OriginalNames {
    source: Option<(usize, usize)>,
    table: InferenceValues<Entry>,
    assigned: InferenceValues<usize>,
    next: usize,
    suffix: usize,
    hash: u64,
    position: Position,
    phase: Phase,
}

impl Default for OriginalNames {
    fn default() -> Self {
        Self { source: None, table: Default::default(), assigned: Default::default(), next: 0, suffix: NO_SUFFIX,
            hash: HASH_SEED, position: Default::default(), phase: Phase::Hash }
    }
}

impl OriginalNames {
    /// 🛠️ Advances a bounded name hash, comparison or retained index publication.
    pub fn advance(&mut self, source: &impl GbxmlNameSource, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "gbXML name planning was cancelled").into()); }
        let count = source.name_count();
        let identity = (source.name_source_identity(), count);
        if self.source.is_none() {
            cx.original_source_parent()?.validate_borrowed_context(cx)?;
            if cx.charge_work_unit() { self.source = Some(identity); }
            return Ok(false);
        }
        if self.source != Some(identity) { return Err(ValueError::invariant("gbXML name source changed").into()); }
        let slots = count.checked_mul(2).and_then(|count| count.checked_add(1))
            .ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "gbXML name table exceeds its representable size"))?;
        if !self.table.admit_source_step(cx)? || !self.table.initialize_step(slots, cx)? { return Ok(false); }
        if !self.assigned.admit_source_step(cx)? || !self.assigned.initialize_step(count, cx)? { return Ok(false); }
        if self.next == count { return Ok(true); }
        match &mut self.phase {
            Phase::PublishBucket { bucket, entry } => {
                if self.table.publish_step(*bucket, entry, cx)? {
                    self.phase = Phase::PublishAssignment { suffix: Some(self.suffix) };
                }
            }
            Phase::PublishAssignment { suffix } => {
                if self.assigned.publish_step(self.next, suffix, cx)? {
                    self.next += 1;
                    self.suffix = NO_SUFFIX;
                    self.reset_hash();
                }
            }
            _ if !cx.charge_work_unit() => (),
            Phase::Hash => {
                let complete = source.with_name(self.next, |name| {
                    for _ in 0..CHARACTER_WORK {
                        match self.position.next(name, self.suffix) {
                            Character::Value(character) => {
                                let mut bytes = [0; 4];
                                for byte in character.encode_utf8(&mut bytes).bytes() {
                                    self.hash = (self.hash ^ u64::from(byte)).wrapping_mul(0x100000001b3);
                                }
                            }
                            Character::Skipped => (),
                            Character::End => return true,
                        }
                    }
                    false
                });
                match complete {
                    Some(true) => self.phase = Phase::Check { bucket: (self.hash % slots as u64) as usize },
                    Some(false) => (),
                    None if self.position.raw == 0 && self.suffix == NO_SUFFIX => self.next += 1,
                    None => return Err(ValueError::invariant("gbXML base name disappeared").into()),
                }
            }
            Phase::Check { bucket } => {
                self.phase = if let Some(other) = self.table.get(*bucket).copied() {
                    self.position = Position::default();
                    Phase::Compare { bucket: *bucket, other, prior: Position::default(), pending: None }
                } else {
                    Phase::PublishBucket { bucket: *bucket, entry: Some(Entry { ordinal: self.next, suffix: self.suffix }) }
                };
            }
            Phase::Compare { bucket, other, prior, pending } => {
                let equal = source.with_name(self.next, |current| source.with_name(other.ordinal, |existing| {
                    for _ in 0..CHARACTER_WORK {
                        let candidate = if let Some(current) = *pending { Character::Value(current) } else { self.position.next(current, self.suffix) };
                        let checkpoint = *prior;
                        match (candidate, prior.next(existing, other.suffix)) {
                            (Character::Skipped, _) => *prior = checkpoint,
                            (Character::Value(value), Character::Skipped) => *pending = Some(value),
                            (Character::Value(left), Character::Value(right)) if left == right => *pending = None,
                            (Character::End, Character::Skipped) => (),
                            (Character::End, Character::End) => return Some(true),
                            _ => return Some(false),
                        }
                    }
                    None
                })).flatten().ok_or_else(|| ValueError::invariant("gbXML candidate name disappeared"))?;
                match equal {
                    Some(true) => {
                        self.suffix = self.suffix.checked_add(1).ok_or_else(|| ValueError::invariant("gbXML name suffix overflow"))?;
                        self.reset_hash();
                    }
                    Some(false) => self.phase = Phase::Check { bucket: (*bucket + 1) % slots },
                    None => (),
                }
            }
        }
        Ok(false)
    }

    fn reset_hash(&mut self) { self.phase = Phase::Hash; self.hash = HASH_SEED; self.position = Position::default(); }

    /// 🏷️ Reads a published suffix, where `1` means that the base had no suffix.
    pub fn suffix(&self, ordinal: usize) -> Option<usize> { self.assigned.get(ordinal).copied() }

    /// ♻️ Quotes retirement of the current original name table.
    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.table.terminal_is_empty() { self.table.retirement_demands(body) } else if !self.assigned.terminal_is_empty() {
            self.assigned.retirement_demands(body)
        } else {
            Ok(Default::default())
        }
    }

    /// 🧺️ Whether both original name owners are empty.
    pub fn terminal_is_empty(&self) -> bool { self.table.terminal_is_empty() && self.assigned.terminal_is_empty() }

    /// 🔚️ Retires one original name row or source loan.
    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if !self.table.terminal_is_empty() { self.table.close_step(cx)?; } else if !self.assigned.terminal_is_empty() {
            self.assigned.close_step(cx)?;
        }
        Ok(self.terminal_is_empty())
    }
}
