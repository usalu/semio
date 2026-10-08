//! 🧬️ Equation diff schema — sparse edits over the artifact: graph scalars, id-keyed node and edge row deltas, a positional
//! point-edit list, label-addressed expression-node kind patches and the re-minted derived child handles.

use crate::standards::v1::subsets::any::schema::snapshot::{EquationExprSnapshot, EquationNodeKind, EquationNodeLabel};
use crate::{EquationComputedChild, EquationEdge, EquationGeometry, EquationGraph, EquationNode, EquationNotationChild, EquationPoint, EquationResultsChild, EquationSnapshot};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the equation artifact; every slot is absent unless the mutation owns it. A state leaf yields its
/// sparse graph/point slots and re-mints the derived handles it changes (`crate::equation_state_diff`). There is no whole
/// `graph`/`geometry`/`equation` slot: nodes and edges are id-keyed row deltas, points a positional edit list and the
/// expression tree label-addressed node-kind patches.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.mathematical.equation")]
pub struct EquationDiff {
    #[state(artifact)]
    pub directed: Option<bool>,
    #[state(artifact)]
    pub algorithm: Option<String>,
    #[state(artifact)]
    pub algorithm_seed: Option<EquationOptionalSeed>,
    #[state(artifact)]
    pub nodes: Option<EquationNodesDelta>,
    #[state(artifact)]
    pub edges: Option<EquationEdgesDelta>,
    #[state(artifact)]
    pub points: Option<EquationPointsDelta>,
    #[state(artifact)]
    pub notation: Option<EquationNotationChild>,
    #[state(artifact)]
    pub results: Option<EquationResultsChild>,
    #[state(artifact)]
    pub computed: Option<EquationComputedChild>,
    #[state(artifact)]
    pub equation: Option<EquationExprDiff>,
}

// 🌱️ Hand-written, not derived — `notation`/`results`/`computed` are `Option<store::ArtifactChild<S>>`, a generic framework
// handle `#[derive(ToValue, FromValue)]` cannot route through (fan-out playbook trap #3); absent slots are omitted.
impl ToValue for EquationDiff {
    fn to_value(&self) -> DslValue {
        let mut entries: Vec<(String, DslValue)> = Vec::new();
        macro_rules! put {
            ($field:ident, $key:literal) => {
                if let Some(value) = &self.$field {
                    entries.push(($key.to_string(), ToValue::to_value(value)));
                }
            };
        }
        put!(directed, "directed");
        put!(algorithm, "algorithm");
        put!(algorithm_seed, "algorithmSeed");
        put!(nodes, "nodes");
        put!(edges, "edges");
        put!(points, "points");
        put!(notation, "notation");
        put!(results, "results");
        put!(computed, "computed");
        put!(equation, "equation");
        DslValue::object(entries)
    }
}
impl FromValue for EquationDiff {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut result = Self::default();
        let mut seen: Vec<String> = Vec::new();
        for (key, value) in DslValue::into_object(value)? {
            if seen.contains(&key) {
                return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("duplicate Equation diff field {key}")));
            }
            match key.as_str() {
                "directed" => result.directed = Some(FromValue::from_value(value)?),
                "algorithm" => result.algorithm = Some(FromValue::from_value(value)?),
                "algorithmSeed" => result.algorithm_seed = Some(FromValue::from_value(value)?),
                "nodes" => result.nodes = Some(FromValue::from_value(value)?),
                "edges" => result.edges = Some(FromValue::from_value(value)?),
                "points" => result.points = Some(FromValue::from_value(value)?),
                "notation" => result.notation = Some(FromValue::from_value(value)?),
                "results" => result.results = Some(FromValue::from_value(value)?),
                "computed" => result.computed = Some(FromValue::from_value(value)?),
                "equation" => result.equation = Some(FromValue::from_value(value)?),
                _ => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown Equation diff field {key}"))),
            }
            seen.push(key);
        }
        Ok(result)
    }
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🧱️ Carries the optional algorithm seed as a present slot, so clearing it stays distinct from leaving it untouched on every wire.
#[derive(Clone, Debug, Default, PartialEq, ToValueDerive, FromValueDerive)]
#[value(rename_all = "camelCase", default)]
pub struct EquationOptionalSeed {
    pub value: Option<String>,
}

/// 🩹 Field patch of one graph node; every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, ToValueDerive, FromValueDerive)]
#[value(rename_all = "camelCase", default)]
pub struct EquationNodePatch {
    pub id: String,
    pub label: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

/// 🩹 Field patch of one graph edge.
#[derive(Clone, Debug, Default, PartialEq, ToValueDerive, FromValueDerive)]
#[value(rename_all = "camelCase", default)]
pub struct EquationEdgePatch {
    pub id: String,
    pub source: Option<String>,
    pub target: Option<String>,
}

/// 🧩 Id-keyed row delta of the graph nodes.
#[derive(Clone, Debug, Default, PartialEq, ToValueDerive, FromValueDerive)]
#[value(rename_all = "camelCase", default)]
pub struct EquationNodesDelta {
    pub added: Vec<EquationNode>,
    pub removed: Vec<String>,
    pub patched: Vec<EquationNodePatch>,
    pub reordered: Option<Vec<String>>,
}

/// 🧩 Id-keyed row delta of the graph edges.
#[derive(Clone, Debug, Default, PartialEq, ToValueDerive, FromValueDerive)]
#[value(rename_all = "camelCase", default)]
pub struct EquationEdgesDelta {
    pub added: Vec<EquationEdge>,
    pub removed: Vec<String>,
    pub patched: Vec<EquationEdgePatch>,
    pub reordered: Option<Vec<String>>,
}

/// ✏️ One positional edit of the point cloud (points carry no identity, only an index); edits apply in order.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive)]
#[value(tag = "op", rename_all = "camelCase")]
pub enum EquationPointEdit {
    Insert { at: u32, point: EquationPoint },
    Remove { at: u32 },
    Set { at: u32, point: EquationPoint },
}

/// 📍️ Ordered positional edits of the point cloud.
#[derive(Clone, Debug, Default, PartialEq, ToValueDerive, FromValueDerive)]
#[value(rename_all = "camelCase", default)]
pub struct EquationPointsDelta {
    pub edits: Vec<EquationPointEdit>,
}

/// 🌳️ Replaces the kind of the expression node addressed by `label` (a stable address, never a positional path).
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive)]
#[value(rename_all = "camelCase")]
pub struct EquationKindPatch {
    pub label: EquationNodeLabel,
    pub kind: EquationNodeKind,
}

/// 🌳️ Sparse expression-tree delta: label-addressed kind patches and the label allocator.
#[derive(Clone, Debug, Default, PartialEq, ToValueDerive, FromValueDerive)]
#[value(rename_all = "camelCase", default)]
pub struct EquationExprDiff {
    pub kinds: Vec<EquationKindPatch>,
    pub next_label: Option<u64>,
}
//#endregion 🔖️DeltaHelpers

//#region 🧺️KeyedDelta
/// 🧺️ Id-keyed ordered-collection delta (`added`/`removed`/`patched`/`reordered`) and its algebra: apply, composition (create∘delete
/// cancels, delete∘create replaces, patch∘patch composes, patch∘create folds), negative delta and state delta.
pub trait KeyedDelta: Sized {
    type Row: Clone;
    type Patch: Clone;
    fn added(&self) -> &[Self::Row];
    fn removed(&self) -> &[String];
    fn patched(&self) -> &[Self::Patch];
    fn reordered(&self) -> Option<&[String]>;
    fn assemble(added: Vec<Self::Row>, removed: Vec<String>, patched: Vec<Self::Patch>, reordered: Option<Vec<String>>) -> Self;
    fn row_key(row: &Self::Row) -> &str;
    fn patch_key(patch: &Self::Patch) -> &str;
    fn patch_fold(patch: &Self::Patch, row: &mut Self::Row) -> Result<(), protocol::MutationApplyError>;
    fn patch_compose(first: &Self::Patch, later: &Self::Patch) -> Self::Patch;
    fn patch_inverse(patch: &Self::Patch, base: &Self::Row) -> Self::Patch;
    fn patch_between(base: &Self::Row, other: &Self::Row) -> Option<Self::Patch>;
    fn patch_is_empty(patch: &Self::Patch) -> bool;
}

fn keyed_error(code: &str, message: &str, at: [&str; 2]) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(code, message).at(at)
}

pub fn keyed_apply<D: KeyedDelta>(rows: &[D::Row], delta: &D) -> Result<Vec<D::Row>, protocol::MutationApplyError> {
    let key = D::row_key;
    for (index, id) in delta.removed().iter().enumerate() {
        if delta.removed()[..index].contains(id) {
            return Err(keyed_error("mutation.apply.duplicate-target", "row is removed more than once", ["removed", &index.to_string()]));
        }
        if !rows.iter().any(|row| key(row) == id) {
            return Err(keyed_error("mutation.apply.missing-target", "removed row does not exist", ["removed", &index.to_string()]));
        }
    }
    let mut next: Vec<D::Row> = rows.iter().filter(|row| !delta.removed().iter().any(|id| id == key(row))).cloned().collect();
    for (index, row) in delta.added().iter().enumerate() {
        if next.iter().any(|existing| key(existing) == key(row)) {
            return Err(keyed_error("mutation.apply.duplicate-target", "added row identity already exists", ["added", &index.to_string()]));
        }
        next.push(row.clone());
    }
    for (index, patch) in delta.patched().iter().enumerate() {
        if delta.patched()[..index].iter().any(|earlier| D::patch_key(earlier) == D::patch_key(patch)) {
            return Err(keyed_error("mutation.apply.duplicate-target", "row is patched more than once", ["patched", &index.to_string()]));
        }
        let row = next.iter_mut().find(|row| key(row) == D::patch_key(patch)).ok_or_else(|| keyed_error("mutation.apply.missing-target", "patched row does not exist", ["patched", &index.to_string()]))?;
        D::patch_fold(patch, row).map_err(|error| error.under(["patched".to_string(), index.to_string()]))?;
    }
    let Some(order) = delta.reordered() else { return Ok(next) };
    if order.len() != next.len() || order.iter().enumerate().any(|(index, id)| order[..index].contains(id) || !next.iter().any(|row| key(row) == id)) {
        return Err(protocol::MutationApplyError::new("mutation.apply.invalid-order", "reorder must be a complete unique permutation").at(["reordered".to_string()]));
    }
    Ok(order.iter().filter_map(|id| next.iter().find(|row| key(row) == id).cloned()).collect())
}

fn canonical<D: KeyedDelta>(mut added: Vec<D::Row>, mut removed: Vec<String>, mut patched: Vec<D::Patch>, reordered: Option<Vec<String>>) -> D {
    if let Some(order) = &reordered {
        added.sort_by_key(|row| order.iter().position(|id| id == D::row_key(row)).unwrap_or(usize::MAX));
    }
    let reordered = reordered.filter(|order| {
        let tail = added.len();
        !(order.len() <= tail + 1 && order.len() >= tail && order[order.len() - tail..].iter().map(String::as_str).eq(added.iter().map(D::row_key)))
    });
    removed.sort();
    removed.dedup();
    patched.retain(|patch| !D::patch_is_empty(patch));
    patched.sort_by(|left, right| D::patch_key(left).cmp(D::patch_key(right)));
    D::assemble(added, removed, patched, reordered)
}

/// ➕️ Normal form of `first` then `later`: create∘delete cancels, delete∘create replaces, patch∘patch composes, patch∘create folds.
pub fn keyed_absorb<D: KeyedDelta>(first: &D, later: &D) -> D {
    let mut added: Vec<D::Row> = first.added().to_vec();
    let mut removed: Vec<String> = first.removed().to_vec();
    let mut patched: Vec<D::Patch> = Vec::new();
    for patch in first.patched() {
        match added.iter_mut().find(|row| D::row_key(row) == D::patch_key(patch)) {
            Some(row) => {
                let _ = D::patch_fold(patch, row);
            }
            None => patched.push(patch.clone()),
        }
    }
    for id in later.removed() {
        if let Some(position) = added.iter().position(|row| D::row_key(row) == id) {
            added.remove(position);
        } else {
            patched.retain(|patch| D::patch_key(patch) != id);
            if !removed.contains(id) {
                removed.push(id.clone());
            }
        }
    }
    added.extend(later.added().iter().cloned());
    for patch in later.patched() {
        let key = D::patch_key(patch);
        if let Some(row) = added.iter_mut().find(|row| D::row_key(row) == key) {
            let _ = D::patch_fold(patch, row);
        } else if let Some(existing) = patched.iter_mut().find(|existing| D::patch_key(existing) == key) {
            *existing = D::patch_compose(existing, patch);
        } else {
            patched.push(patch.clone());
        }
    }
    let reordered = match (later.reordered(), first.reordered()) {
        (Some(order), _) => Some(order.to_vec()),
        (None, Some(order)) => Some(order.iter().filter(|id| !later.removed().contains(id)).cloned().chain(later.added().iter().map(|row| D::row_key(row).to_string())).collect()),
        (None, None) => None,
    };
    canonical::<D>(added, removed, patched, reordered)
}

fn ids_after<D: KeyedDelta>(base: &[String], delta: &D) -> Vec<String> {
    match delta.reordered() {
        Some(order) => order.to_vec(),
        None => base.iter().filter(|id| !delta.removed().contains(id)).cloned().chain(delta.added().iter().map(|row| D::row_key(row).to_string())).collect(),
    }
}

/// 🔁️ The negative delta: removes what `delta` added, restores what it removed, undoes its patches, restores the base order.
pub fn keyed_inverse<D: KeyedDelta>(delta: &D, base: &[D::Row]) -> D {
    let base_ids: Vec<String> = base.iter().map(|row| D::row_key(row).to_string()).collect();
    let removed: Vec<String> = delta.added().iter().map(|row| D::row_key(row).to_string()).collect();
    let added: Vec<D::Row> = delta.removed().iter().filter_map(|id| base.iter().find(|row| D::row_key(row) == id).cloned()).collect();
    let patched: Vec<D::Patch> = delta
        .patched()
        .iter()
        .filter(|patch| !removed.iter().any(|id| id == D::patch_key(patch)))
        .filter_map(|patch| base.iter().find(|row| D::row_key(row) == D::patch_key(patch)).map(|row| D::patch_inverse(patch, row)))
        .collect();
    let after = ids_after(&base_ids, delta);
    let natural: Vec<String> = after.iter().filter(|id| !removed.contains(id)).cloned().chain(added.iter().map(|row| D::row_key(row).to_string())).collect();
    let reordered = (natural != base_ids).then_some(base_ids);
    canonical::<D>(added, removed, patched, reordered)
}

/// 🧭️ The delta turning `base` into `other` (sync/import only).
pub fn keyed_between<D: KeyedDelta>(base: &[D::Row], other: &[D::Row]) -> D {
    let base_ids: Vec<String> = base.iter().map(|row| D::row_key(row).to_string()).collect();
    let other_ids: Vec<String> = other.iter().map(|row| D::row_key(row).to_string()).collect();
    let removed: Vec<String> = base_ids.iter().filter(|id| !other_ids.contains(id)).cloned().collect();
    let added: Vec<D::Row> = other.iter().filter(|row| !base_ids.iter().any(|id| id == D::row_key(row))).cloned().collect();
    let patched: Vec<D::Patch> = other.iter().filter_map(|row| base.iter().find(|candidate| D::row_key(candidate) == D::row_key(row)).and_then(|candidate| D::patch_between(candidate, row))).collect();
    let natural: Vec<String> = base_ids.iter().filter(|id| !removed.contains(id)).cloned().chain(added.iter().map(|row| D::row_key(row).to_string())).collect();
    let reordered = (natural != other_ids).then_some(other_ids);
    canonical::<D>(added, removed, patched, reordered)
}

pub fn keyed_is_empty<D: KeyedDelta>(delta: &D) -> bool {
    delta.added().is_empty() && delta.removed().is_empty() && delta.patched().iter().all(D::patch_is_empty) && delta.reordered().is_none()
}
//#endregion 🧺️KeyedDelta

macro_rules! keyed_delta_impl {
    ($delta:ident, $row:ty, $patch:ty, $row_key:ident, $patch_key:ident, $fold:expr, $compose:expr, $inverse:expr, $patch_between:expr, $is_empty:expr) => {
        impl KeyedDelta for $delta {
            type Row = $row;
            type Patch = $patch;
            fn added(&self) -> &[$row] {
                &self.added
            }
            fn removed(&self) -> &[String] {
                &self.removed
            }
            fn patched(&self) -> &[$patch] {
                &self.patched
            }
            fn reordered(&self) -> Option<&[String]> {
                self.reordered.as_deref()
            }
            fn assemble(added: Vec<$row>, removed: Vec<String>, patched: Vec<$patch>, reordered: Option<Vec<String>>) -> Self {
                Self { added, removed, patched, reordered }
            }
            fn row_key(row: &$row) -> &str {
                row.$row_key.as_str()
            }
            fn patch_key(patch: &$patch) -> &str {
                patch.$patch_key.as_str()
            }
            fn patch_fold(patch: &$patch, row: &mut $row) -> Result<(), protocol::MutationApplyError> {
                ($fold)(patch, row);
                Ok(())
            }
            fn patch_compose(first: &$patch, later: &$patch) -> $patch {
                ($compose)(first, later)
            }
            fn patch_inverse(patch: &$patch, base: &$row) -> $patch {
                ($inverse)(patch, base)
            }
            fn patch_between(base: &$row, other: &$row) -> Option<$patch> {
                ($patch_between)(base, other)
            }
            fn patch_is_empty(patch: &$patch) -> bool {
                ($is_empty)(patch)
            }
        }
    };
}

keyed_delta_impl!(
    EquationNodesDelta,
    EquationNode,
    EquationNodePatch,
    id,
    id,
    |patch: &EquationNodePatch, row: &mut EquationNode| {
        if let Some(label) = &patch.label {
            row.label = label.clone();
        }
        if let Some(x) = patch.x {
            row.x = x;
        }
        if let Some(y) = patch.y {
            row.y = y;
        }
    },
    |first: &EquationNodePatch, later: &EquationNodePatch| EquationNodePatch { id: first.id.clone(), label: later.label.clone().or_else(|| first.label.clone()), x: later.x.or(first.x), y: later.y.or(first.y) },
    |patch: &EquationNodePatch, base: &EquationNode| EquationNodePatch { id: patch.id.clone(), label: patch.label.as_ref().map(|_| base.label.clone()), x: patch.x.map(|_| base.x), y: patch.y.map(|_| base.y) },
    |base: &EquationNode, other: &EquationNode| {
        let patch = EquationNodePatch { id: other.id.clone(), label: (base.label != other.label).then(|| other.label.clone()), x: (base.x != other.x).then_some(other.x), y: (base.y != other.y).then_some(other.y) };
        (patch.label.is_some() || patch.x.is_some() || patch.y.is_some()).then_some(patch)
    },
    |patch: &EquationNodePatch| patch.label.is_none() && patch.x.is_none() && patch.y.is_none()
);

keyed_delta_impl!(
    EquationEdgesDelta,
    EquationEdge,
    EquationEdgePatch,
    id,
    id,
    |patch: &EquationEdgePatch, row: &mut EquationEdge| {
        if let Some(source) = &patch.source {
            row.source = source.clone();
        }
        if let Some(target) = &patch.target {
            row.target = target.clone();
        }
    },
    |first: &EquationEdgePatch, later: &EquationEdgePatch| EquationEdgePatch { id: first.id.clone(), source: later.source.clone().or_else(|| first.source.clone()), target: later.target.clone().or_else(|| first.target.clone()) },
    |patch: &EquationEdgePatch, base: &EquationEdge| EquationEdgePatch { id: patch.id.clone(), source: patch.source.as_ref().map(|_| base.source.clone()), target: patch.target.as_ref().map(|_| base.target.clone()) },
    |base: &EquationEdge, other: &EquationEdge| {
        let patch = EquationEdgePatch { id: other.id.clone(), source: (base.source != other.source).then(|| other.source.clone()), target: (base.target != other.target).then(|| other.target.clone()) };
        (patch.source.is_some() || patch.target.is_some()).then_some(patch)
    },
    |patch: &EquationEdgePatch| patch.source.is_none() && patch.target.is_none()
);

//#region 🔖️PointEdits
fn point_edit_error(code: &str, message: &str, index: usize) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(code, message).at(["edits".to_string(), index.to_string()])
}

/// ▶️ Applies the positional edits in order onto `points`.
pub fn points_after(points: &[EquationPoint], edits: &[EquationPointEdit]) -> Result<Vec<EquationPoint>, protocol::MutationApplyError> {
    let mut next = points.to_vec();
    for (index, edit) in edits.iter().enumerate() {
        match edit {
            EquationPointEdit::Insert { at, point } if *at as usize <= next.len() => next.insert(*at as usize, point.clone()),
            EquationPointEdit::Insert { .. } => return Err(point_edit_error("mutation.apply.invalid-index", "point insert index is out of range", index)),
            EquationPointEdit::Remove { at } if (*at as usize) < next.len() => {
                next.remove(*at as usize);
            }
            EquationPointEdit::Remove { .. } => return Err(point_edit_error("mutation.apply.missing-target", "removed point does not exist", index)),
            EquationPointEdit::Set { at, point } if (*at as usize) < next.len() => next[*at as usize] = point.clone(),
            EquationPointEdit::Set { .. } => return Err(point_edit_error("mutation.apply.missing-target", "set point does not exist", index)),
        }
    }
    Ok(next)
}

fn merge_point_edits(top: &EquationPointEdit, next: &EquationPointEdit) -> Option<Option<EquationPointEdit>> {
    use EquationPointEdit::{Insert, Remove, Set};
    match (top, next) {
        (Insert { at, .. }, Remove { at: removed }) if at == removed => Some(None),
        (Insert { at, .. }, Set { at: set, point }) if at == set => Some(Some(Insert { at: *at, point: point.clone() })),
        (Set { at, .. }, Set { at: set, point }) if at == set => Some(Some(Set { at: *at, point: point.clone() })),
        (Set { at, .. }, Remove { at: removed }) if at == removed => Some(Some(Remove { at: *at })),
        (Remove { at }, Insert { at: inserted, point }) if at == inserted => Some(Some(Set { at: *at, point: point.clone() })),
        _ => None,
    }
}

/// ➕️ Canonical form of an edit sequence: adjacent edits of one index fold (insert∘remove cancels, insert∘set inserts the last point,
/// set∘set keeps the last point, remove∘insert sets) and runs of sets sort by index.
pub fn canonical_point_edits(edits: Vec<EquationPointEdit>) -> Vec<EquationPointEdit> {
    let mut stack: Vec<EquationPointEdit> = Vec::new();
    for edit in edits {
        let mut pending = Some(edit);
        while let Some(current) = pending.take() {
            match stack.last().and_then(|top| merge_point_edits(top, &current)) {
                Some(merged) => {
                    stack.pop();
                    pending = merged;
                }
                None => stack.push(current),
            }
        }
    }
    let mut result: Vec<EquationPointEdit> = Vec::new();
    let mut run: std::collections::BTreeMap<u32, EquationPoint> = std::collections::BTreeMap::new();
    for edit in stack {
        match edit {
            EquationPointEdit::Set { at, point } => {
                run.insert(at, point);
            }
            other => {
                result.extend(std::mem::take(&mut run).into_iter().map(|(at, point)| EquationPointEdit::Set { at, point }));
                result.push(other);
            }
        }
    }
    result.extend(run.into_iter().map(|(at, point)| EquationPointEdit::Set { at, point }));
    result
}

fn points_inverse(edits: &[EquationPointEdit], base: &[EquationPoint]) -> Vec<EquationPointEdit> {
    let mut state = base.to_vec();
    let mut inverse = Vec::new();
    for edit in edits {
        match edit {
            EquationPointEdit::Insert { at, point } if *at as usize <= state.len() => {
                state.insert(*at as usize, point.clone());
                inverse.push(EquationPointEdit::Remove { at: *at });
            }
            EquationPointEdit::Remove { at } if (*at as usize) < state.len() => {
                let removed = state.remove(*at as usize);
                inverse.push(EquationPointEdit::Insert { at: *at, point: removed });
            }
            EquationPointEdit::Set { at, point } if (*at as usize) < state.len() => {
                let old = std::mem::replace(&mut state[*at as usize], point.clone());
                inverse.push(EquationPointEdit::Set { at: *at, point: old });
            }
            _ => {}
        }
    }
    inverse.reverse();
    canonical_point_edits(inverse)
}

/// 🧭️ The positional edits turning `base` into `replacement`: the common prefix and suffix stay, the middle is set pairwise and the
/// surplus is removed or inserted.
pub fn points_replacing(base: &[EquationPoint], replacement: &[EquationPoint]) -> EquationPointsDelta {
    let prefix = base.iter().zip(replacement).take_while(|(left, right)| left == right).count();
    let suffix = base[prefix..].iter().rev().zip(replacement[prefix..].iter().rev()).take_while(|(left, right)| left == right).count();
    let (old, new) = (&base[prefix..base.len() - suffix], &replacement[prefix..replacement.len() - suffix]);
    let common = old.len().min(new.len());
    let mut edits: Vec<EquationPointEdit> = (0..common).filter(|index| old[*index] != new[*index]).map(|index| EquationPointEdit::Set { at: (prefix + index) as u32, point: new[index].clone() }).collect();
    edits.extend((common..old.len()).map(|_| EquationPointEdit::Remove { at: (prefix + common) as u32 }));
    edits.extend((common..new.len()).map(|index| EquationPointEdit::Insert { at: (prefix + index) as u32, point: new[index].clone() }));
    EquationPointsDelta { edits: canonical_point_edits(edits) }
}

/// 🧭️ The sparse graph slots turning `base` into `replacement`: the scalars that differ plus the node and edge row deltas.
pub fn graph_replacing(base: &EquationGraph, replacement: &EquationGraph) -> EquationDiff {
    let nodes = keyed_between::<EquationNodesDelta>(&base.nodes, &replacement.nodes);
    let edges = keyed_between::<EquationEdgesDelta>(&base.edges, &replacement.edges);
    EquationDiff {
        directed: (base.directed != replacement.directed).then_some(replacement.directed),
        algorithm: (base.algorithm != replacement.algorithm).then(|| replacement.algorithm.clone()),
        algorithm_seed: (base.algorithm_seed != replacement.algorithm_seed).then(|| EquationOptionalSeed { value: replacement.algorithm_seed.clone() }),
        nodes: (!keyed_is_empty(&nodes)).then_some(nodes),
        edges: (!keyed_is_empty(&edges)).then_some(edges),
        ..Default::default()
    }
}
//#endregion 🔖️PointEdits

//#region 🔖️Apply
impl EquationDiff {
    /// 🧮️ The `(graph, geometry)` this diff leaves behind `base` — the state the derived child handles are minted from.
    pub fn state_after(&self, base: &EquationSnapshot) -> protocol::MutationApplyResult<(EquationGraph, EquationGeometry)> {
        let mut graph = base.graph.clone();
        if let Some(directed) = self.directed {
            graph.directed = directed;
        }
        if let Some(algorithm) = &self.algorithm {
            graph.algorithm = algorithm.clone();
        }
        if let Some(seed) = &self.algorithm_seed {
            graph.algorithm_seed = seed.value.clone();
        }
        if let Some(delta) = &self.nodes {
            graph.nodes = keyed_apply(&graph.nodes, delta).map_err(|error| error.under(["nodes"]))?;
        }
        if let Some(delta) = &self.edges {
            graph.edges = keyed_apply(&graph.edges, delta).map_err(|error| error.under(["edges"]))?;
        }
        let points = match &self.points {
            Some(delta) => points_after(&base.geometry.points, &delta.edits).map_err(|error| error.under(["points"]))?,
            None => base.geometry.points.clone(),
        };
        Ok((graph, EquationGeometry { points }))
    }
}

impl EquationExprDiff {
    fn is_empty(&self) -> bool {
        self.kinds.is_empty() && self.next_label.is_none()
    }

    fn canonical(mut self) -> Self {
        let mut kinds: Vec<EquationKindPatch> = Vec::new();
        for patch in std::mem::take(&mut self.kinds) {
            match kinds.iter_mut().find(|existing| existing.label == patch.label) {
                Some(slot) => *slot = patch,
                None => kinds.push(patch),
            }
        }
        kinds.sort_by_key(|patch| patch.label.0);
        Self { kinds, next_label: self.next_label }
    }
}

impl MutationDiff<EquationSnapshot> for EquationDiff {
    fn apply(&self, snapshot: &EquationSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<EquationSnapshot> {
        let (graph, geometry) = self.state_after(snapshot)?;
        let mut next = snapshot.clone();
        next.graph = graph;
        next.geometry = geometry;
        if let Some(notation) = &self.notation {
            next.notation = notation.clone();
        }
        if let Some(results) = &self.results {
            next.results = results.clone();
        }
        if let Some(computed) = &self.computed {
            next.computed = computed.clone();
        }
        if let Some(expr) = &self.equation {
            for (index, patch) in expr.kinds.iter().enumerate() {
                if !next.equation.replace(patch.label, &patch.kind) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "patched expression node does not exist").at(["equation".to_string(), "kinds".to_string(), index.to_string()]));
                }
            }
            if let Some(next_label) = expr.next_label {
                next.equation.next_label = next_label;
            }
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(directed);
        take!(algorithm);
        take!(algorithm_seed);
        take!(notation);
        take!(results);
        take!(computed);
        self.nodes = match (self.nodes.take(), other.nodes) {
            (Some(first), Some(later)) => Some(keyed_absorb(&first, &later)),
            (first, later) => later.or(first),
        };
        self.edges = match (self.edges.take(), other.edges) {
            (Some(first), Some(later)) => Some(keyed_absorb(&first, &later)),
            (first, later) => later.or(first),
        };
        self.points = match (self.points.take(), other.points) {
            (Some(mut first), Some(later)) => {
                first.edits.extend(later.edits);
                first.edits = canonical_point_edits(first.edits);
                Some(first)
            }
            (first, later) => later.or(first),
        };
        self.equation = match (self.equation.take(), other.equation) {
            (Some(mut first), Some(later)) => {
                first.kinds.extend(later.kinds);
                first.next_label = later.next_label.or(first.next_label);
                Some(first.canonical())
            }
            (first, later) => later.or(first),
        };
    }
}

impl protocol::DiffAlgebra<EquationSnapshot> for EquationDiff {
    fn inverse(&self, base: &EquationSnapshot) -> Self {
        Self {
            directed: self.directed.map(|_| base.graph.directed),
            algorithm: self.algorithm.as_ref().map(|_| base.graph.algorithm.clone()),
            algorithm_seed: self.algorithm_seed.as_ref().map(|_| EquationOptionalSeed { value: base.graph.algorithm_seed.clone() }),
            nodes: self.nodes.as_ref().map(|delta| keyed_inverse(delta, &base.graph.nodes)),
            edges: self.edges.as_ref().map(|delta| keyed_inverse(delta, &base.graph.edges)),
            points: self.points.as_ref().map(|delta| EquationPointsDelta { edits: points_inverse(&delta.edits, &base.geometry.points) }),
            notation: self.notation.as_ref().map(|_| base.notation.clone()),
            results: self.results.as_ref().map(|_| base.results.clone()),
            computed: self.computed.as_ref().map(|_| base.computed.clone()),
            equation: self.equation.as_ref().map(|expr| {
                EquationExprDiff {
                    kinds: expr.kinds.iter().filter_map(|patch| base.equation.find(patch.label).map(|node| EquationKindPatch { label: patch.label, kind: node.kind.clone() })).collect(),
                    next_label: expr.next_label.map(|_| base.equation.next_label),
                }
                .canonical()
            }),
        }
    }

    fn between(base: &EquationSnapshot, other: &EquationSnapshot) -> Self {
        let nodes = keyed_between::<EquationNodesDelta>(&base.graph.nodes, &other.graph.nodes);
        let edges = keyed_between::<EquationEdgesDelta>(&base.graph.edges, &other.graph.edges);
        let points = points_replacing(&base.geometry.points, &other.geometry.points);
        let expr = (base.equation != other.equation).then(|| EquationExprDiff {
            kinds: (base.equation.expr != other.equation.expr).then(|| EquationKindPatch { label: other.equation.expr.label, kind: other.equation.expr.kind.clone() }).into_iter().collect(),
            next_label: (base.equation.next_label != other.equation.next_label).then_some(other.equation.next_label),
        });
        Self {
            directed: (base.graph.directed != other.graph.directed).then_some(other.graph.directed),
            algorithm: (base.graph.algorithm != other.graph.algorithm).then(|| other.graph.algorithm.clone()),
            algorithm_seed: (base.graph.algorithm_seed != other.graph.algorithm_seed).then(|| EquationOptionalSeed { value: other.graph.algorithm_seed.clone() }),
            nodes: (!keyed_is_empty(&nodes)).then_some(nodes),
            edges: (!keyed_is_empty(&edges)).then_some(edges),
            points: (!points.edits.is_empty()).then_some(points),
            notation: (base.notation != other.notation).then(|| other.notation.clone()),
            results: (base.results != other.results).then(|| other.results.clone()),
            computed: (base.computed != other.computed).then(|| other.computed.clone()),
            equation: expr,
        }
    }

    fn is_empty(&self) -> bool {
        self.directed.is_none()
            && self.algorithm.is_none()
            && self.algorithm_seed.is_none()
            && self.nodes.as_ref().is_none_or(keyed_is_empty)
            && self.edges.as_ref().is_none_or(keyed_is_empty)
            && self.points.as_ref().is_none_or(|delta| delta.edits.is_empty())
            && self.notation.is_none()
            && self.results.is_none()
            && self.computed.is_none()
            && self.equation.as_ref().is_none_or(EquationExprDiff::is_empty)
    }
}
//#endregion 🔖️Apply

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
