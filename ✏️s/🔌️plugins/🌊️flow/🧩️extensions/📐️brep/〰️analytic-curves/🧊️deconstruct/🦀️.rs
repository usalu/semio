//! 🧊️ Retained deconstruction of one exact source geometry.

use super::*;
use super::topology_output::TopologyOutput;
use semio_s_spatial_kernel_semio_session::{NativeTopologyReach, ReachedTopology};

const TOPOLOGY: [ReachedTopology; 4] = [ReachedTopology::Vertex, ReachedTopology::Edge, ReachedTopology::Face, ReachedTopology::Shell];
const SELECTED: [(&str, &str, ReachedTopology); 2] = [
    ("edgeLabels", "selectedEdges", ReachedTopology::Edge),
    ("faceLabels", "selectedFaces", ReachedTopology::Face),
];

#[path = "🎯️labels/🦀️.rs"]
mod labels;
pub(crate) use labels::LabelSelection;

fn invalid(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, message) }

fn parameter<'a>(input: &'a Dictionary, key: &str) -> Result<&'a str, ValueError> {
    input.get(key).and_then(Value::as_dictionary).and_then(|value| value.get("value"))
        .and_then(Value::as_atom).and_then(Atom::as_str).ok_or_else(|| invalid("deconstruction requires text parameters"))
}

fn scoped_source(input: &Dictionary) -> Result<(&str, &Dictionary), ValueError> {
    let source = input.get("brep").and_then(Value::as_dictionary).ok_or_else(|| invalid("deconstruction requires original geometry"))?;
    let captured = parameter(input, "sourceHandle")?;
    if !captured.is_empty() && (captured.len() != 64 || !captured.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))) {
        return Err(invalid("sourceHandle must be an exact geometry handle"));
    }
    if source.schema() == Some("list") {
        if captured.is_empty() || source.len() > 600_001 {
            return Err(invalid("a bounded source collection requires a captured geometry handle"));
        }
        return Ok((captured, source));
    }
    let original = geometry_handle(input, "brep")?;
    if !captured.is_empty() && original != captured { return Err(invalid("selected source geometry has changed")); }
    Ok((original, source))
}

#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
pub(super) struct DeconstructOutput {
    reach: NativeTopologyReach,
    source_rank: usize,
    source_index: Option<u32>,
    gathering: bool,
    scope_ready: bool,
    metadata: Metadata,
    edge_labels: Option<LabelSelection>,
    face_labels: Option<LabelSelection>,
    selection: usize,
    position: usize,
    current: Option<TopologyOutput>,
    result: Option<Dictionary>,
}

impl DeconstructOutput {
    pub(super) fn new(session: &Session, input: &Dictionary) -> Result<Self, ValueError> {
        let (_, source) = scoped_source(input)?;
        for (port, _, _) in SELECTED {
            if parameter(input, port)?.len() > 16_000_000 {
                return Err(invalid("component selection exceeds its byte limit"));
            }
        }
        let gathering = source.schema() == Some("list");
        Ok(Self { reach: session.plan_original_topology_reach()?, source_rank: 0, source_index: (!gathering).then_some(0), gathering, scope_ready: false,
            metadata: Metadata::default(), edge_labels: Some(LabelSelection::default()), face_labels: Some(LabelSelection::default()), selection: 0, position: 0, current: None, result: None })
    }

    pub(super) fn phase(&self) -> &'static str {
        if self.gathering { "deconstruct-source" }
        else if !self.scope_ready { "deconstruct-reach" }
        else if self.selection < SELECTED.len() { "deconstruct-labels" }
        else { self.current.as_ref().map_or("deconstruct-components", TopologyOutput::phase) }
    }

    pub(super) fn staged_claim(&self) -> Option<&str> { self.current.as_ref().and_then(TopologyOutput::staged_claim) }

    pub(super) fn close_claim(&mut self, session: &Session, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        self.current.as_mut().expect("deconstruct claim has an active component list").close_claim(session, effect)
    }

    fn labels_mut(&mut self, index: usize) -> &mut Option<LabelSelection> {
        if index == 0 { &mut self.edge_labels } else { &mut self.face_labels }
    }

    fn labels(&self, index: usize) -> &LabelSelection {
        (if index == 0 { &self.edge_labels } else { &self.face_labels }).as_ref().expect("deconstruct labels under selection")
    }

    pub(super) fn demand(&self, session: &Session, input: &Dictionary, copy: usize) -> Result<RetirementDemand, ValueError> {
        let (handle, source) = scoped_source(input)?;
        let basic = RetirementDemand { depth: dictionary_depth(input).max(dictionary_depth(source)), ..Default::default() };
        if self.gathering { return Ok(basic); }
        if !self.scope_ready { return session.original_topology_reach_demand(&self.reach, handle, copy); }
        if let Some((_, _, kind)) = SELECTED.get(self.selection) {
            return self.labels(self.selection).demand(input, session, &self.reach, *kind, copy);
        }
        if self.metadata.field == 0 {
            return Ok(RetirementDemand { depth: basic.depth.max(session.original_query_depth_demand(handle)?), ..basic });
        }
        if self.metadata.field < Metadata::END {
            return self.metadata.demand(handle, copy);
        }
        if self.result.is_some() { return Ok(basic); }
        self.current.as_ref().expect("active deconstruct component list").demand(session, Some(&self.reach), copy)
    }

    pub(super) fn step(&mut self, session: &Session, input: &Dictionary, effect: &mut RetainedAllocationEffect<'_>) -> Result<(Option<Dictionary>, Progress), ValueError> {
        let grant = effect.grant();
        let pending = (None, Progress::default());
        if !retained_clone_grant_admits(grant, self.demand(session, input, grant.maximum_copy_bytes)?) { return Ok(pending); }
        let (handle, source) = scoped_source(input)?;
        if self.gathering {
            if !effect.performed(1)? { return Ok(pending); }
            if let Some((key, value)) = source.entry_at_rank(self.source_rank) {
                if key != neural_engine::SCHEMA_KEY {
                    let index = (key.len() <= 10 && (key == "0" || !key.starts_with('0')))
                        .then(|| key.parse::<u32>().ok()).flatten().filter(|_| key.bytes().all(|b| b.is_ascii_digit()))
                        .ok_or_else(|| invalid("source collection indices must be canonical integers"))?;
                    if geometry_atom_handle(Some(value))? == handle && self.source_index.replace(index).is_some() {
                        return Err(invalid("selected source geometry is ambiguous in its collection"));
                    }
                }
                self.source_rank += 1;
            } else {
                if self.source_index.is_none() { return Err(invalid("selected source geometry no longer exists in its collection")); }
                self.gathering = false;
            }
            return Ok((None, item()));
        }
        if !self.scope_ready {
            let (ready, receipt) = session.step_original_topology_reach(&mut self.reach, handle, effect)?;
            self.scope_ready = ready;
            return Ok((None, receipt));
        }
        if let Some((port, _, kind)) = SELECTED.get(self.selection) {
            let labels = if self.selection == 0 { &mut self.edge_labels } else { &mut self.face_labels };
            let labels = labels.as_mut().expect("deconstruct labels under selection");
            let receipt = labels.step(parameter(input, port)?, session, &self.reach, *kind, effect)?;
            if labels.ready() { self.selection += 1; }
            return Ok((None, receipt));
        }
        if self.metadata.field == 0 {
            if !effect.performed(1)? { return Ok(pending); }
            self.metadata.kind = session.original_topology_kind(handle)?;
            self.metadata.field = 1;
            return Ok((None, item()));
        }
        if self.metadata.field < Metadata::END {
            let receipt = self.metadata.step(handle, self.source_index.expect("selected source index"), effect)?;
            if self.metadata.field == Metadata::END { self.result = self.metadata.output.take(); }
            return Ok((None, receipt));
        }
        if self.result.is_some() {
            if !effect.performed(1)? { return Ok(pending); }
            let output = self.result.take().expect("deconstruct accumulated output");
            let previous = if let Some(current) = &mut self.current {
                if let Some(&kind) = TOPOLOGY.get(self.position) {
                    current.reset(kind.channel(), kind, self.reach.count(kind), None, output)
                } else if let Some(&(_, channel, kind)) = SELECTED.get(self.position - TOPOLOGY.len()) {
                    let index = self.position - TOPOLOGY.len();
                    let labels = if index == 0 { self.edge_labels.take() } else { self.face_labels.take() };
                    current.reset(channel, kind, 0, labels, output)
                } else {
                    current.reset_empty("errors", output)
                }
            } else {
                let kind = TOPOLOGY[self.position];
                self.current = Some(TopologyOutput::reached(kind, self.reach.count(kind), output));
                None
            };
            if let Some(previous) = previous {
                let index = self.position - TOPOLOGY.len() - 1;
                *self.labels_mut(index) = Some(previous);
            }
            return Ok((None, item()));
        }
        let current = self.current.as_mut().expect("active deconstruct component list");
        let (value, receipt) = current.step(session, Some(&self.reach), effect)?;
        if let Some(value) = value {
            self.position += 1;
            if self.position == TOPOLOGY.len() + SELECTED.len() + 1 { return Ok((Some(value), receipt)); }
            self.result = Some(value);
        }
        return Ok((None, receipt));
    }
}

#[derive(Default, semio_framework_value::RetireOwned)]
#[value(send_only)]
struct Metadata {
    field: u8,
    kind: &'static str,
    key: String,
    text: String,
    inner: Option<Dictionary>,
    output: Option<Dictionary>,
    entry: Option<BudgetedOwnedEntry>,
    closing: bool,
}

impl Metadata {
    const END: u8 = 8;

    fn field<'a>(&self, handle: &'a str) -> (&'static str, Option<&'a str>) {
        match self.field {
            1 => ("$schema", Some("geometry")), 2 => ("handle", Some(handle)), 3 => ("kind", Some(self.kind)),
            4 => ("brepOut", None), 5 => ("$schema", Some("number")), 6 => ("value", None), 7 => ("sourceIndex", None),
            _ => unreachable!(),
        }
    }

    fn demand(&self, handle: &str, copy: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(entry) = &self.entry {
            return if self.closing { entry.next_close_demands(copy) } else { entry.next_demands(copy) };
        }
        let (key, value) = self.field(handle);
        if self.key.len() < key.len() { return Ok(text_demand(&self.key, key)); }
        Ok(value.filter(|value| self.text.len() < value.len()).map_or(RetirementDemand { depth: 1, ..Default::default() }, |value| text_demand(&self.text, value)))
    }

    fn step(&mut self, handle: &str, index: u32, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        let copy = effect.grant().maximum_copy_bytes;
        if let Some(entry) = &mut self.entry {
            if self.closing {
                return match entry.close_step(copy, effect) {
                    RetainedWorkPoll::Pending { .. } => Ok(entry.step_progress()),
                    RetainedWorkPoll::Complete(outcome) => {
                        let receipt = outcome.result.map_err(|error| error.into_value_error())?.progress();
                        if entry.terminal_is_empty() { self.entry = None; self.closing = false; self.field += 1; }
                        Ok(receipt)
                    }
                };
            }
            let poll = entry.step(copy, effect);
            let receipt = entry.step_progress();
            if let RetainedWorkPoll::Complete(outcome) = poll {
                if let Some(dictionary) = outcome.result.map_err(|error| error.into_value_error())? {
                    if matches!(self.field, 4 | 7) { self.output = Some(dictionary); } else { self.inner = Some(dictionary); }
                    self.closing = true;
                }
            }
            return Ok(receipt);
        }
        let (key, value) = self.field(handle);
        if self.key.len() < key.len() { return write_text(&mut self.key, key, effect); }
        if let Some(value) = value.filter(|value| self.text.len() < value.len()) { return write_text(&mut self.text, value, effect); }
        if !effect.performed(1)? { return Ok(Progress::default()); }
        let (base, value) = match self.field {
            4 | 7 => (self.output.take().unwrap_or_default(), Value::Dictionary(self.inner.take().unwrap())),
            6 => (self.inner.take().unwrap(), Value::Atom(Atom::Decimal(f64::from(index)))),
            _ => (self.inner.take().unwrap_or_default(), Value::Atom(Atom::String(take(&mut self.text)))),
        };
        self.entry = Some(BudgetedOwnedEntry::new(base, take(&mut self.key), value));
        Ok(item())
    }
}
