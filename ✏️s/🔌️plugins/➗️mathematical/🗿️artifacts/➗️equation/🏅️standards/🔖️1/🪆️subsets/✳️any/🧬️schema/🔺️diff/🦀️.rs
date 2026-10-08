//! 🧬️ Equation diff schema — sparse edits over the artifact: graph scalars, positional node and edge row deltas, a positional
//! point-edit list and label-addressed expression-node kind patches. The derived `notation`/`results`/`computed` child handles are
//! never carried: `apply` re-derives them from the graph and points it leaves behind.

use crate::standards::v1::subsets::any::schema::snapshot::{EquationExprSnapshot, EquationNodeKind, EquationNodeLabel};
use crate::{EquationEdge, EquationGeometry, EquationNode, EquationPoint, EquationSnapshot};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the equation artifact; every slot is absent unless the mutation owns it. A state leaf yields its
/// sparse graph/point slots only. There is no whole
/// `graph`/`geometry`/`equation` slot: nodes and edges are positional row deltas, points a positional edit list and the
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
    pub equation: Option<EquationExprDiff>,
}

// 🌱️ Hand-written, not derived: absent slots are omitted from the wire.
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
    pub label: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

/// 🩹 Field patch of one graph edge.
#[derive(Clone, Debug, Default, PartialEq, ToValueDerive, FromValueDerive)]
#[value(rename_all = "camelCase", default)]
pub struct EquationEdgePatch {
    pub source: Option<String>,
    pub target: Option<String>,
}

protocol::list_delta! {
    /// 🧩 Positional row delta of the graph nodes.
    pub EquationNodesDelta { removal: EquationNodeRemoval, insertion: EquationNodeInsertion, relocation: EquationNodeRelocation, modification: EquationNodesModification, row: EquationNode, patch: EquationNodePatch, key: id, values_only }
}

protocol::list_delta! {
    /// 🧩 Positional row delta of the graph edges.
    pub EquationEdgesDelta { removal: EquationEdgeRemoval, insertion: EquationEdgeInsertion, relocation: EquationEdgeRelocation, modification: EquationEdgesModification, row: EquationEdge, patch: EquationEdgePatch, key: id, values_only }
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






impl protocol::list_delta::RowPatch<EquationNode> for EquationNodePatch {
    fn commit_into(&self, row: &mut EquationNode, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(label) = &self.label {
            row.label = label.clone();
        }
        if let Some(x) = self.x {
            row.x = x;
        }
        if let Some(y) = self.y {
            row.y = y;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.label = later.label.or(self.label.take());
        self.x = later.x.or(self.x);
        self.y = later.y.or(self.y);
    }
    fn inverse(&self, row: &EquationNode) -> Self {
        Self { label: self.label.as_ref().map(|_| row.label.clone()), x: self.x.map(|_| row.x), y: self.y.map(|_| row.y) }
    }
    fn is_empty(&self) -> bool {
        self.label.is_none() && self.x.is_none() && self.y.is_none()
    }
}

impl protocol::list_delta::RowPatch<EquationEdge> for EquationEdgePatch {
    fn commit_into(&self, row: &mut EquationEdge, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(source) = &self.source {
            row.source = source.clone();
        }
        if let Some(target) = &self.target {
            row.target = target.clone();
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.source = later.source.or(self.source.take());
        self.target = later.target.or(self.target.take());
    }
    fn inverse(&self, row: &EquationEdge) -> Self {
        Self { source: self.source.as_ref().map(|_| row.source.clone()), target: self.target.as_ref().map(|_| row.target.clone()) }
    }
    fn is_empty(&self) -> bool {
        self.source.is_none() && self.target.is_none()
    }
}

//#region 🔖️PointEdits
fn point_edit_error(code: &str, message: &str, index: usize) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(code, message).at(["edits".to_string(), index.to_string()])
}

/// ▶️ Applies the positional edits in order onto `points`.
pub fn points_after(points: &[EquationPoint], edits: &[EquationPointEdit]) -> Result<Vec<EquationPoint>, protocol::MutationApplyError> {
    let mut next: Vec<EquationPoint> = Vec::new();
    next.extend_from_slice(points);
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

/// ↩️ The edits that turn the point list `edits` leaves behind back into `base`, read point by point off `base` by origin: inserted
/// points are removed (highest index first), removed base points are reinserted at their base index (lowest first) and every
/// surviving base point a `Set` touched is set back to its base position.
fn points_inverse(edits: &[EquationPointEdit], base: &[EquationPoint]) -> Vec<EquationPointEdit> {
    let mut slots: Vec<Option<usize>> = (0..base.len()).map(Some).collect();
    let mut touched = std::collections::BTreeSet::new();
    for edit in edits {
        match edit {
            EquationPointEdit::Insert { at, .. } if *at as usize <= slots.len() => slots.insert(*at as usize, None),
            EquationPointEdit::Remove { at } if (*at as usize) < slots.len() => {
                slots.remove(*at as usize);
            }
            EquationPointEdit::Set { at, .. } => {
                if let Some(Some(origin)) = slots.get(*at as usize) {
                    touched.insert(*origin);
                }
            }
            _ => {}
        }
    }
    let survivors: Vec<usize> = slots.iter().flatten().copied().collect();
    let removals = slots.iter().enumerate().rev().filter(|(_, origin)| origin.is_none()).map(|(index, _)| EquationPointEdit::Remove { at: index as u32 });
    let reinserts = base.iter().enumerate().filter(|(index, _)| !survivors.contains(index)).map(|(index, point)| EquationPointEdit::Insert { at: index as u32, point: point.clone() });
    let restores = touched.iter().filter(|origin| survivors.contains(origin)).map(|origin| EquationPointEdit::Set { at: *origin as u32, point: base[*origin].clone() });
    canonical_point_edits(removals.chain(reinserts).chain(restores).collect())
}

//#endregion 🔖️PointEdits

//#region 🔖️Apply
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
    fn apply(&self, snapshot: &EquationSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<EquationSnapshot> {
        let mut next = snapshot.clone();
        if let Some(directed) = self.directed {
            next.graph.directed = directed;
        }
        if let Some(algorithm) = &self.algorithm {
            next.graph.algorithm = algorithm.clone();
        }
        if let Some(seed) = &self.algorithm_seed {
            next.graph.algorithm_seed = seed.value.clone();
        }
        if let Some(delta) = &self.nodes {
            next.graph.nodes = delta.commit_onto(&next.graph.nodes, capability).map_err(|error| error.under(["nodes"]))?;
        }
        if let Some(delta) = &self.edges {
            next.graph.edges = delta.commit_onto(&next.graph.edges, capability).map_err(|error| error.under(["edges"]))?;
        }
        if let Some(delta) = &self.points {
            next.geometry = EquationGeometry { points: points_after(&snapshot.geometry.points, &delta.edits).map_err(|error| error.under(["points"]))? };
        }
        if self.directed.is_some() || self.algorithm.is_some() || self.algorithm_seed.is_some() || self.nodes.is_some() || self.edges.is_some() || self.points.is_some() {
            (next.notation, next.results, next.computed) = crate::equation_children(&next.graph, &next.geometry);
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
        self.nodes = match (self.nodes.take(), other.nodes) {
            (Some(mut first), Some(later)) => {
                first.absorb(later);
                Some(first)
            }
            (first, later) => later.or(first),
        };
        self.edges = match (self.edges.take(), other.edges) {
            (Some(mut first), Some(later)) => {
                first.absorb(later);
                Some(first)
            }
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
            nodes: self.nodes.as_ref().map(|delta| delta.inverse(&base.graph.nodes)),
            edges: self.edges.as_ref().map(|delta| delta.inverse(&base.graph.edges)),
            points: self.points.as_ref().map(|delta| EquationPointsDelta { edits: points_inverse(&delta.edits, &base.geometry.points) }),
            equation: self.equation.as_ref().map(|expr| {
                EquationExprDiff {
                    kinds: expr.kinds.iter().filter_map(|patch| base.equation.find(patch.label).map(|node| EquationKindPatch { label: patch.label, kind: node.kind.clone() })).collect(),
                    next_label: expr.next_label.map(|_| base.equation.next_label),
                }
                .canonical()
            }),
        }
    }

    fn is_empty(&self) -> bool {
        self.directed.is_none()
            && self.algorithm.is_none()
            && self.algorithm_seed.is_none()
            && self.nodes.as_ref().is_none_or(EquationNodesDelta::is_empty)
            && self.edges.as_ref().is_none_or(EquationEdgesDelta::is_empty)
            && self.points.as_ref().is_none_or(|delta| delta.edits.is_empty())
            && self.equation.as_ref().is_none_or(EquationExprDiff::is_empty)
    }
}
//#endregion 🔖️Apply

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
