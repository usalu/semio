//! 🐚️ Bounded dictionary assembly and stable topology alias transfer.

use super::*;
use super::deconstruct::LabelSelection;
use semio_s_spatial_kernel_semio_session::{NativeTopologyReach, ReachedTopology};
use semio_framework_3d_brep_engine::{TopologyAlias, TopologyList, TopologySelection};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Phase {
    ListSchema,
    MemberSchema,
    MemberHandle,
    MemberKind,
    MemberEntry,
    WrapOutput,
    Publish,
}
semio_framework_value::artifact_retire_leaf!(Phase);

#[derive(semio_framework_value::RetireOwned)]
enum Source {
    Ordered(TopologySelection),
    Reached { kind: ReachedTopology, count: usize },
    Selected { channel: &'static str, kind: ReachedTopology, labels: LabelSelection },
    Empty(&'static str),
}

impl Source {
    fn count(&self) -> usize {
        match self { Self::Ordered(source) => source.count(), Self::Reached { count, .. } => *count, Self::Selected { labels, .. } => labels.len(), Self::Empty(_) => 0 }
    }

    fn channel(&self) -> &'static str {
        match self {
            Self::Ordered(source) => match source.kind() { TopologyList::Shells => "shells", TopologyList::Solids => "solids" },
            Self::Reached { kind, .. } => kind.channel(),
            Self::Selected { channel, .. } | Self::Empty(channel) => channel,
        }
    }
}

#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
pub(super) struct TopologyOutput {
    source: Source,
    alias: Option<TopologyAlias>,
    list: Option<Dictionary>,
    member: Option<Dictionary>,
    output: Option<Dictionary>,
    entry: Option<BudgetedOwnedEntry>,
    key: String,
    text: String,
    live_key: String,
    claim_key: String,
    index: usize,
    phase: Phase,
    closing_entry: bool,
    claim_staged: bool,
}

impl TopologyOutput {
    pub(super) fn new(source: TopologySelection) -> Self {
        Self::with_source(Source::Ordered(source), None)
    }

    pub(super) fn reached(kind: ReachedTopology, count: usize, output: Dictionary) -> Self {
        Self::with_source(Source::Reached { kind, count }, Some(output))
    }

    pub(super) fn reset(&mut self, channel: &'static str, kind: ReachedTopology, count: usize, labels: Option<LabelSelection>, output: Dictionary) -> Option<LabelSelection> {
        let source = labels.map_or(Source::Reached { kind, count }, |labels| Source::Selected { channel, kind, labels });
        self.reset_source(source, output)
    }

    pub(super) fn reset_empty(&mut self, channel: &'static str, output: Dictionary) -> Option<LabelSelection> {
        self.reset_source(Source::Empty(channel), output)
    }

    fn reset_source(&mut self, source: Source, output: Dictionary) -> Option<LabelSelection> {
        debug_assert!(self.alias.is_none() && self.output.is_none() && self.entry.is_none() && !self.claim_staged);
        self.output = Some(output);
        self.index = 0;
        self.phase = Phase::ListSchema;
        match std::mem::replace(&mut self.source, source) { Source::Selected { labels, .. } => Some(labels), _ => None }
    }

    fn with_source(source: Source, output: Option<Dictionary>) -> Self {
        Self {
            source,
            alias: None,
            list: None,
            member: None,
            output,
            entry: None,
            key: String::new(),
            text: String::new(),
            live_key: String::new(),
            claim_key: String::new(),
            index: 0,
            phase: Phase::ListSchema,
            closing_entry: false,
            claim_staged: false,
        }
    }

    pub(super) fn phase(&self) -> &'static str {
        let Source::Ordered(source) = &self.source else {
            return if self.claim_staged { "deconstruct-staged-claim" } else { "deconstruct-components" };
        };
        match (source.kind(), self.claim_staged, self.phase) {
            (TopologyList::Shells, true, _) => "shells-staged-claim",
            (TopologyList::Shells, _, Phase::Publish) => "shells-aliases",
            (TopologyList::Shells, _, _) => "shells-list",
            (TopologyList::Solids, true, _) => "explode-staged-claim",
            (TopologyList::Solids, _, Phase::Publish) => "explode-aliases",
            (TopologyList::Solids, _, _) => "explode-list",
        }
    }

    pub(super) fn staged_claim(&self) -> Option<&str> {
        self.claim_staged.then_some(self.live_key.as_str())
    }

    pub(super) fn close_claim(&mut self, session: &Session, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        let receipt = session.close_staged_geometry_claim(&self.live_key, effect)?;
        if receipt.copied_items != 0 { self.claim_staged = false; }
        Ok(receipt)
    }

    fn field<'a>(source: &Source, phase: Phase, index: &'a str, alias: Option<&'a TopologyAlias>) -> (&'a str, Option<&'a str>) {
        match phase {
            Phase::ListSchema => ("$schema", Some("list")),
            Phase::MemberSchema => ("$schema", Some(match source {
                Source::Reached { kind, .. } | Source::Selected { kind, .. } => kind.channel(),
                _ => "geometry",
            })),
            Phase::MemberHandle => ("handle", alias.map(TopologyAlias::handle)),
            Phase::MemberKind => ("kind", Some(if source.channel() == "shells" { "shell" } else { "solid" })),
            Phase::MemberEntry => (index, None),
            Phase::WrapOutput => (source.channel(), None),
            _ => unreachable!("publication phases have no dictionary fields"),
        }
    }

    fn advance_field(&mut self) {
        self.phase = match self.phase {
            Phase::ListSchema if self.source.count() == 0 => Phase::WrapOutput,
            Phase::ListSchema => Phase::MemberSchema,
            Phase::MemberSchema => Phase::MemberHandle,
            Phase::MemberHandle if matches!(self.source, Source::Ordered(_)) => Phase::MemberKind,
            Phase::MemberHandle => Phase::MemberEntry,
            Phase::MemberKind => Phase::MemberEntry,
            Phase::MemberEntry => {
                self.alias = None;
                self.index += 1;
                if self.index == self.source.count() { Phase::WrapOutput } else { Phase::MemberSchema }
            }
            Phase::WrapOutput => { self.index = 0; Phase::Publish }
            _ => unreachable!("publication phases have no dictionary fields"),
        };
    }

    fn needs_alias(&self) -> bool {
        self.alias.is_none() && matches!(self.phase, Phase::MemberSchema | Phase::Publish)
    }

    pub(super) fn demand(&self, session: &Session, reach: Option<&NativeTopologyReach>, copy: usize) -> Result<RetirementDemand, ValueError> {
        let empty = RetirementDemand { depth: 1, ..Default::default() };
        if self.needs_alias() {
            return Ok(RetirementDemand {
                copy_bytes: size_of::<TopologyAlias>(),
                depth: match &self.source {
                    Source::Ordered(source) => session.topology_member_depth(source, self.index)?,
                    Source::Reached { kind, .. } => session.reached_topology_depth(reach.expect("reached topology scope"), *kind, self.index)?,
                    Source::Selected { kind, labels, .. } => session.reached_topology_depth(
                        reach.expect("selected topology scope"), *kind, labels.member(self.index)?,
                    )?.max(labels.depth(self.index)?),
                    Source::Empty(_) => unreachable!(),
                },
                ..empty
            });
        }
        if self.phase == Phase::Publish {
            let alias = self.alias.as_ref().expect("alias publication has a source");
            let handle = alias.handle();
            if self.live_key.len() < handle.len() { return Ok(text_demand(&self.live_key, handle)); }
            if !self.claim_staged && self.claim_key.len() < handle.len() { return Ok(text_demand(&self.claim_key, handle)); }
            return session.topology_alias_publication_demand(alias, self.claim_staged, copy);
        }
        if let Some(entry) = &self.entry {
            return if self.closing_entry { entry.next_close_demands(copy) } else { entry.next_demands(copy) };
        }
        let mut digits = [0; 20];
        let index = decimal(self.index, &mut digits);
        let (key, value) = Self::field(&self.source, self.phase, index, self.alias.as_ref());
        if self.key.len() < key.len() { return Ok(text_demand(&self.key, key)); }
        Ok(value.filter(|value| self.text.len() < value.len()).map_or(empty, |value| text_demand(&self.text, value)))
    }

    pub(super) fn step(&mut self, session: &Session, reach: Option<&NativeTopologyReach>, effect: &mut RetainedAllocationEffect<'_>) -> Result<(Option<Dictionary>, Progress), ValueError> {
        let grant = effect.grant();
        let empty = Progress::default();
        if !retained_clone_grant_admits(grant, self.demand(session, reach, grant.maximum_copy_bytes)?) {
            return Ok((None, empty));
        }
        if self.needs_alias() {
            if !effect.performed(1)? { return Ok((None, empty)); }
            self.alias = Some(match &self.source {
                Source::Ordered(source) => session.topology_member(source, self.index)?,
                Source::Reached { kind, .. } => session.reached_topology_alias(reach.expect("reached topology scope"), *kind, self.index)?,
                Source::Selected { kind, labels, .. } => session.reached_topology_alias(
                    reach.expect("selected topology scope"), *kind, labels.member(self.index)?,
                )?,
                Source::Empty(_) => unreachable!(),
            });
            return Ok((None, Progress { copied_bytes: size_of::<TopologyAlias>(), ..item() }));
        }
        if self.phase == Phase::Publish {
            let alias = self.alias.as_ref().expect("alias publication has a source");
            let handle = alias.handle();
            if self.live_key.len() < handle.len() { return write_text(&mut self.live_key, handle, effect).map(|receipt| (None, receipt)); }
            if !self.claim_staged && self.claim_key.len() < handle.len() { return write_text(&mut self.claim_key, handle, effect).map(|receipt| (None, receipt)); }
            let (ready, receipt) = session.step_topology_alias_publication(alias, &mut self.live_key, &mut self.claim_key, &mut self.claim_staged, effect)?;
            if ready {
                self.alias = None;
                self.live_key.clear();
                self.claim_key.clear();
                self.index += 1;
                if self.index == self.source.count() { return Ok((self.output.take(), receipt)); }
            }
            return Ok((None, receipt));
        }
        let receipt = self.step_dictionary(effect)?;
        let empty_list = self.phase == Phase::Publish && self.source.count() == 0;
        Ok((if empty_list { self.output.take() } else { None }, receipt))
    }

    fn step_dictionary(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        let grant = effect.grant();
        if let Some(entry) = &mut self.entry {
            if self.closing_entry {
                return match entry.close_step(grant.maximum_copy_bytes, effect) {
                    RetainedWorkPoll::Pending { .. } => Ok(entry.step_progress()),
                    RetainedWorkPoll::Complete(outcome) => {
                        let step = outcome.result.map_err(|error| error.into_value_error())?;
                        if entry.terminal_is_empty() {
                            self.entry = None;
                            self.closing_entry = false;
                            self.advance_field();
                        }
                        Ok(step.progress())
                    }
                };
            }
            let poll = entry.step(grant.maximum_copy_bytes, effect);
            let receipt = entry.step_progress();
            if let RetainedWorkPoll::Complete(outcome) = poll {
                if let Some(dictionary) = outcome.result.map_err(|error| error.into_value_error())? {
                    match self.phase {
                        Phase::ListSchema | Phase::MemberEntry => self.list = Some(dictionary),
                        Phase::WrapOutput => self.output = Some(dictionary),
                        _ => self.member = Some(dictionary),
                    }
                    self.closing_entry = true;
                }
            }
            return Ok(receipt);
        }

        let mut digits = [0; 20];
        let index = decimal(self.index, &mut digits);
        let (key, value) = Self::field(&self.source, self.phase, index, self.alias.as_ref());
        if self.key.len() < key.len() { return write_text(&mut self.key, key, effect); }
        if let Some(value) = value.filter(|value| self.text.len() < value.len()) {
            return write_text(&mut self.text, value, effect);
        }
        if !effect.performed(1)? { return Ok(Progress::default()); }
        let (base, value) = match self.phase {
            Phase::MemberEntry => (self.list.take().unwrap(), Value::Dictionary(self.member.take().unwrap())),
            Phase::WrapOutput => (self.output.take().unwrap_or_default(), Value::Dictionary(self.list.take().unwrap())),
            phase => {
                let base = if phase == Phase::ListSchema { Dictionary::new() } else { self.member.take().unwrap_or_default() };
                (base, Value::Atom(Atom::String(take(&mut self.text))))
            }
        };
        self.entry = Some(BudgetedOwnedEntry::new(base, take(&mut self.key), value));
        Ok(item())
    }
}

fn decimal(mut index: usize, digits: &mut [u8; 20]) -> &str {
    let mut pos = digits.len();
    loop {
        pos -= 1;
        digits[pos] = b'0' + (index % 10) as u8;
        index /= 10;
        if index == 0 { break; }
    }
    std::str::from_utf8(&digits[pos..]).expect("topology index is decimal")
}
