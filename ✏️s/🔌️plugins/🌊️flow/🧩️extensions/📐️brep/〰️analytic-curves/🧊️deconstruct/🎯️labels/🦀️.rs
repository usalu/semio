//! 🎯️ Funded parsing and resolution of exact persistent component labels.

use super::*;
use semio_framework_3d_brep_engine::TopologyAlias;
use semio_framework_replication_history_index::HistoryFoldIndex;

#[derive(Clone, Copy, Default)]
enum Token {
    #[default]
    Begin, First, Next, Digits, Backslash, Unicode, After, End,
}
semio_framework_value::artifact_retire_leaf!(Token);

#[derive(Default, semio_framework_value::RetireOwned)]
struct Parser {
    offset: usize,
    token: Token,
    label: u64,
    unicode: u16,
    escapes: u8,
}

impl Parser {
    fn digit(&mut self, value: u64) -> Result<(), ValueError> {
        if self.label == 0 && value == 0 { return Err(bad_label()); }
        self.label = self.label.checked_mul(10).and_then(|label| label.checked_add(value)).ok_or_else(bad_label)?;
        Ok(())
    }

    fn byte(&mut self, byte: u8) -> Result<Option<u64>, ValueError> {
        self.offset += 1;
        if !matches!(self.token, Token::Digits | Token::Backslash | Token::Unicode)
            && matches!(byte, b' ' | b'\t' | b'\r' | b'\n') { return Ok(None); }
        self.token = match (self.token, byte) {
            (Token::Begin, b'[') => Token::First,
            (Token::First, b']') | (Token::After, b']') => Token::End,
            (Token::First | Token::Next, b'"') => Token::Digits,
            (Token::Digits, b'0'..=b'9') => { self.digit(u64::from(byte - b'0'))?; Token::Digits }
            (Token::Digits, b'\\') => Token::Backslash,
            (Token::Backslash, b'u') => { self.unicode = 0; self.escapes = 0; Token::Unicode }
            (Token::Unicode, byte) => {
                let hex = char::from(byte).to_digit(16).ok_or_else(bad_label)? as u16;
                self.unicode = (self.unicode << 4) | hex;
                self.escapes += 1;
                if self.escapes == 4 {
                    if !(0x30..=0x39).contains(&self.unicode) { return Err(bad_label()); }
                    self.digit(u64::from(self.unicode - 0x30))?;
                    Token::Digits
                } else { Token::Unicode }
            }
            (Token::Digits, b'"') if self.label != 0 => {
                self.token = Token::After;
                return Ok(Some(take(&mut self.label)));
            }
            (Token::After, b',') => Token::Next,
            _ => return Err(bad_label()),
        };
        Ok(None)
    }
}

fn bad_label() -> ValueError { invalid("component labels must be exact positive uint64 decimal text in a JSON array") }

#[derive(Default, semio_framework_value::RetireOwned)]
pub(crate) struct LabelSelection {
    labels: HistoryFoldIndex<u64, Option<usize>>,
    parser: Parser,
    pending: Option<u64>,
    alias: Option<TopologyAlias>,
    rank: usize,
    scanned: usize,
    parsed: usize,
    read: bool,
    ready: bool,
}

impl LabelSelection {
    pub(crate) fn len(&self) -> usize { self.labels.len() }
    pub(crate) fn ready(&self) -> bool { self.ready }

    pub(crate) fn depth(&self, rank: usize) -> Result<usize, ValueError> { self.labels.next_rank_depth_demand(rank) }

    pub(crate) fn member(&self, rank: usize) -> Result<usize, ValueError> {
        self.labels.ranked(rank).and_then(|(_, value)| *value).ok_or_else(|| invalid("selected component no longer exists in its source topology"))
    }

    pub(crate) fn demand(&self, input: &Dictionary, session: &Session, reach: &NativeTopologyReach, kind: ReachedTopology, copy: usize) -> Result<RetirementDemand, ValueError> {
        let basic = RetirementDemand { depth: dictionary_depth(input), ..Default::default() };
        if let Some(label) = self.pending {
            if self.labels.contains_key(&label) {
                return Ok(RetirementDemand { depth: self.labels.next_contains_depth_demand(&label)?, ..basic });
            }
            return Ok(RetirementDemand {
                capacity_bytes: self.labels.next_insert_capacity_byte_demand(&label, copy)?,
                depth: self.labels.next_insert_depth_demand(&label)?,
                ..basic
            });
        }
        if !self.read || self.labels.is_empty() || self.rank == reach.count(kind) { return Ok(basic); }
        if let Some(alias) = &self.alias {
            return Ok(RetirementDemand { depth: self.labels.next_contains_depth_demand(&alias.label())?, ..basic });
        }
        Ok(RetirementDemand { depth: session.reached_topology_depth(reach, kind, self.rank)?, copy_bytes: size_of::<TopologyAlias>(), ..basic })
    }

    pub(crate) fn step(&mut self, data: &str, session: &Session, reach: &NativeTopologyReach, kind: ReachedTopology, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        let grant = effect.grant();
        if !effect.performed(1)? { return Ok(Progress::default()); }
        if let Some(label) = self.pending {
            if !self.labels.contains_key(&label) {
                if self.labels.next_insert_capacity_byte_demand(&label, grant.maximum_copy_bytes)? != 0 {
                    return self.labels.reserve_insert_step(&label, grant).map_err(|(error, progress)| error.with_retained_progress(progress));
                }
                self.labels.insert_reserved(label, None, grant).map_err(|(error, _, _)| error)?;
            }
            self.pending = None;
        } else if !self.read {
            if let Some(byte) = data.as_bytes().get(self.parser.offset) {
                self.pending = self.parser.byte(*byte)?;
                if self.pending.is_some() { self.parsed += 1; }
                if self.parsed > 600_000 { return Err(invalid("component selection exceeds its item limit")); }
            } else {
                if !matches!(self.parser.token, Token::End) { return Err(bad_label()); }
                self.read = true;
            }
        } else if self.labels.is_empty() || self.rank == reach.count(kind) {
            if self.scanned != self.labels.len() { return Err(invalid("selected component no longer exists in its source topology")); }
            self.ready = true;
        } else if let Some(alias) = self.alias.take() {
            if let Some(target) = self.labels.get_mut(&alias.label()) {
                if target.replace(self.rank).is_some() { return Err(invalid("component label is ambiguous within its source topology")); }
                self.scanned += 1;
            }
            self.rank += 1;
        } else {
            self.alias = Some(session.reached_topology_alias(reach, kind, self.rank)?);
            return Ok(Progress { copied_bytes: size_of::<TopologyAlias>(), ..item() });
        }
        Ok(item())
    }
}
