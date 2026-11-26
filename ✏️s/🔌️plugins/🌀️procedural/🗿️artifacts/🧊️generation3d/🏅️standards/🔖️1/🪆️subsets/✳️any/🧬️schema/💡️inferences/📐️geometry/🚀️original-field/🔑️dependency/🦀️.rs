//! 🔑️ Funds the dependency digest of a source widget, its authored incoming wires and its typed parent hashes.
use super::*;
use protocol::{ArtifactCanonicalJson, ArtifactCanonicalJsonCursor, ArtifactCanonicalJsonNode, ArtifactCanonicalJsonText, ArtifactCanonicalJsonTree, OriginalDependencyHasher, OriginalDependencyStep};
use semio_framework_value::RetirementDemand;

struct Tree<'a>(&'a dyn ArtifactCanonicalJsonTree);
impl Tree<'_> {
    fn at(&self, path: &[usize]) -> Result<&dyn ArtifactCanonicalJsonTree, ValueError> {
        path.iter().try_fold(self.0, |node, ordinal| node.canonical_tree_child(*ordinal))
    }
}
impl ArtifactCanonicalJson for Tree<'_> {
    fn canonical_json_node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'_>, ValueError> { self.at(path)?.canonical_tree_node() }
    fn canonical_json_key(&self, path: &[usize], index: usize) -> Result<ArtifactCanonicalJsonText<'_>, ValueError> { self.at(path)?.canonical_tree_key(index) }
}

enum Phase { Widget, FindWire, Wire, FollowingWire, Unsupported }

struct DependencyWork {
    cursor: ArtifactCanonicalJsonCursor,
    hash: OriginalDependencyHasher,
    phase: Phase,
    wire: usize,
    offset: usize,
}

impl InferencePending for DependencyWork {
    fn cancel(&mut self) {}
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn close_original(&mut self, _cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> { Ok(true) }
    fn original_box_release_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
    fn original_retirement_demands(&self, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand { release_bytes: size_of::<Self>(), depth: 1, ..Default::default() })
    }
}

impl DependencyWork {
    fn stream(&mut self, source: &dyn ArtifactCanonicalJsonTree, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let mut grant = cx.retained_grant();
        let demand = self.cursor.next_encode_demand();
        if demand.depth > self.cursor.path.len() {
            post(cx, RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
            self.phase = Phase::Unsupported;
            return Ok(false);
        }
        let output_bound = self.cursor.next_encode_output_bound();
        if demand.depth > grant.maximum_depth || grant.maximum_copy_bytes < demand.copy_bytes.saturating_add(output_bound) { return Ok(false); }
        let mut bytes = [0;64];
        let length = if output_bound == 0 { 1 } else { ((grant.maximum_copy_bytes - demand.copy_bytes) / 2).clamp(1, bytes.len()) };
        grant.maximum_copy_bytes -= if output_bound == 0 { 0 } else { length };
        let step = match self.cursor.encode_chunk_admitted(&Tree(source), &mut bytes[..length], grant) {
            Ok(step) => step,
            Err(_) => {
                post(cx, RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
                self.phase = Phase::Unsupported;
                return Ok(false);
            }
        };
        let mut receipt = step.ownership.progress();
        receipt.copied_bytes += step.written_bytes;
        post(cx, receipt)?;
        self.hash.append(&bytes[..step.written_bytes]);
        Ok(self.cursor.is_complete())
    }

    fn advance(&mut self, snapshot: &Generation3dSnapshot, widget: &Widget, cx: &mut StepContext<'_>) -> Result<OriginalDependencyStep, OriginalTurnFailure> {
        let grant = cx.retained_grant();
        match self.phase {
            Phase::Widget => {
                if self.stream(widget, cx)? { self.phase = Phase::FindWire; }
            }
            Phase::FindWire => {
                let Some(wire) = snapshot.host_snapshot.synapses.get(self.wire) else {
                    let bytes = size_of::<OriginalDependencyHasher>() + 32;
                    if grant.maximum_copy_bytes < bytes { return Ok(OriginalDependencyStep::Pending); }
                    post(cx, RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
                    return Ok(OriginalDependencyStep::Ready(self.hash.finish()));
                };
                let key = widget_id(widget).as_bytes();
                let target = wire.to.as_bytes();
                let available = grant.maximum_copy_bytes.min(64) / 2;
                let extent = if key.len() == target.len() { (key.len() - self.offset).min(available) } else { 0 };
                if key.len() == target.len() && self.offset < key.len() && extent == 0 { return Ok(OriginalDependencyStep::Pending); }
                post(cx, RetainedCloneProgress { copied_items: 1, copied_bytes: extent * 2, ..Default::default() })?;
                let end = self.offset + extent;
                if key.len() != target.len() || key[self.offset..end] != target[self.offset..end] {
                    self.wire += 1;
                    self.offset = 0;
                } else if end == key.len() {
                    self.phase = Phase::FollowingWire;
                } else {
                    self.offset = end;
                }
            }
            Phase::FollowingWire => {
                if grant.maximum_copy_bytes <= ArtifactCanonicalJsonCursor::restart_copy_bytes() { return Ok(OriginalDependencyStep::Pending); }
                let Some(mut receipt) = self.cursor.restart_admitted(grant)? else { return Ok(OriginalDependencyStep::Pending); };
                receipt.copied_bytes += 1;
                post(cx, receipt)?;
                self.hash.append(&[0]);
                self.phase = Phase::Wire;
            }
            Phase::Wire => {
                let wire = snapshot.host_snapshot.synapses.get(self.wire).ok_or_else(|| invalid("original dependency wire disappeared"))?;
                if self.stream(wire, cx)? {
                    self.wire += 1;
                    self.offset = 0;
                    self.phase = Phase::FindWire;
                }
            }
            Phase::Unsupported => return Ok(OriginalDependencyStep::Uncacheable),
        }
        Ok(OriginalDependencyStep::Pending)
    }
}

pub(in super::super) fn step(snapshot: &Generation3dSnapshot, ordinal: usize, pending: &mut Option<Box<dyn InferencePending>>, cx: &mut StepContext<'_>) -> Result<OriginalDependencyStep, InferenceWorkFailure> {
    if !open(cx) { return Ok(OriginalDependencyStep::Pending); }
    if pending.is_none() {
        let bytes = size_of::<DependencyWork>();
        if cx.retained_grant().maximum_capacity_bytes < bytes { return Ok(OriginalDependencyStep::Pending); }
        *pending = Some(Box::new(DependencyWork {
            cursor: Default::default(),
            hash: OriginalDependencyHasher::new(Generation3dGeometry::FIELD_ID, Generation3dGeometry::SCHEMA_VERSION),
            phase: Phase::Widget, wire: 0, offset: 0,
        }));
        post(cx, RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: bytes, ..Default::default() })?;
        return Ok(OriginalDependencyStep::Pending);
    }
    let widget = snapshot.host_snapshot.widgets.get(ordinal).ok_or_else(|| invalid("original dependency widget disappeared"))?;
    let owner = pending.as_mut().and_then(|owner| owner.as_any_mut().downcast_mut::<DependencyWork>()).ok_or_else(|| invalid("original dependency cursor received another pending owner"))?;
    Ok(owner.advance(snapshot, widget, cx)?)
}

pub(in super::super) fn weight(evaluation: &SharedEvaluation) -> Option<usize> {
    if evaluation.outputs.len() > 8 { return None; }
    let entries = evaluation.outputs.iter().try_fold(96usize, |bytes, (port, value)| {
        if matches!(value, GeometryValue::List(_)) { return None; }
        bytes.checked_add(port.len())?.checked_add(value.byte_len())
    })?;
    match evaluation.fault.as_ref() {
        Some(fault) => entries.checked_add(fault.code.len())?.checked_add(fault.message.en.len())?.checked_add(fault.message.de.len()),
        None => Some(entries),
    }
}
