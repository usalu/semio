//! 🩹️ FEM 3D artifact — sparse collection diffs + the whole-document `Fem3dDiff` (constitutional: op's
//! diff half). Every id-keyed document collection (nodes/elements/materials/sections/solids/supports/
//! load cases/combinations) shares one generic shape via `HasId`/`CollectionDiff<T>`/
//! `apply_collection_diff`/`index_of` rather than eight hand-duplicated `apply_*_diff` functions — a
//! DRY refactor over the pre-migration `fem3d_op` crate, whose JSON wire shape (`{"removed": [...],
//! "set": [[index, value], ...]}`) is unchanged since `CollectionDiff<T>`'s field names are identical to
//! every old per-type struct's.

use crate::artifacts::fem3d::{element_id, Fem3dDocument, FemAnalysisSettings, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemSection, FemSolid, FemSupport};
use protocol::OperationDiff;
use serde::{Deserialize, Serialize};

// #region 🔖️HasId
/// 🪪️ A document collection entry's stable id — the common shape `index_of`/`apply_collection_diff`
/// generalize over.
pub trait HasId {
    fn id(&self) -> &str;
}

impl HasId for FemNode {
    fn id(&self) -> &str {
        &self.id
    }
}

impl HasId for FemElement {
    fn id(&self) -> &str {
        element_id(self)
    }
}

impl HasId for FemMaterial {
    fn id(&self) -> &str {
        &self.id
    }
}

impl HasId for FemSection {
    fn id(&self) -> &str {
        &self.id
    }
}

impl HasId for FemSolid {
    fn id(&self) -> &str {
        &self.id
    }
}

impl HasId for FemSupport {
    fn id(&self) -> &str {
        &self.id
    }
}

impl HasId for FemLoadCase {
    fn id(&self) -> &str {
        &self.id
    }
}

impl HasId for FemCombination {
    fn id(&self) -> &str {
        &self.id
    }
}

/// 🔎️ The position of the entry whose `HasId::id()` equals `id`, if any.
pub fn index_of<T: HasId>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|entry| entry.id() == id)
}
// #endregion 🔖️HasId

// #region 🔖️Collections
/// 🩹️ Sparse id-keyed collection diff — removals plus id-or-index `set`s (replace when the id already
/// exists, else insert at the recorded index). Mirrors `procedural_2d`'s `WidgetsDiff` pattern so
/// disjoint edits from concurrent peers merge cleanly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionDiff<T> {
    pub removed: Vec<String>,
    pub set: Vec<(usize, T)>,
}

/// 🈳️ Hand-written rather than derived: `#[derive(Default)]` would demand `T: Default`, which no
/// document entity type satisfies (`FemElement`/`FemLoad` are enums with no natural empty value), even
/// though an empty diff never needs a `T` at all.
impl<T> Default for CollectionDiff<T> {
    fn default() -> Self {
        Self { removed: Vec::new(), set: Vec::new() }
    }
}

impl<T> CollectionDiff<T> {
    /// 🧬️ Concatenates `other`'s removals/sets onto `self`'s — the per-collection half of
    /// `Fem3dDiff::absorb`.
    fn absorb(&mut self, other: CollectionDiff<T>) {
        self.removed.extend(other.removed);
        self.set.extend(other.set);
    }
}

pub type NodesDiff = CollectionDiff<FemNode>;
pub type ElementsDiff = CollectionDiff<FemElement>;
pub type MaterialsDiff = CollectionDiff<FemMaterial>;
pub type SectionsDiff = CollectionDiff<FemSection>;
pub type SolidsDiff = CollectionDiff<FemSolid>;
pub type SupportsDiff = CollectionDiff<FemSupport>;
pub type LoadCasesDiff = CollectionDiff<FemLoadCase>;
pub type CombinationsDiff = CollectionDiff<FemCombination>;

/// 🔧️ Applies a `CollectionDiff<T>` to `items` in place: every `removed` id drops its entry, then every
/// `set` either replaces the existing entry with the same id or inserts at the recorded index (clamped
/// to the current length).
pub fn apply_collection_diff<T: HasId + Clone>(items: &mut Vec<T>, diff: &CollectionDiff<T>) {
    for id in &diff.removed {
        items.retain(|entry| entry.id() != id);
    }
    for (index, item) in &diff.set {
        match index_of(items, item.id()) {
            Some(pos) => items[pos] = item.clone(),
            None => items.insert((*index).min(items.len()), item.clone()),
        }
    }
}
// #endregion 🔖️Collections

// #region 🔖️Fem3dDiff
/// 🩹️ Sparse fem-3d diff over every document collection (camera is session-only runtime state, not a
/// document field — see `Fem3dConfig::camera` in the app's `🎚️config`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem3dDiff {
    /// 🌍️ Whole-document replacement (example import / reset); wins over every granular field below.
    pub document: Option<Fem3dDocument>,
    pub nodes: NodesDiff,
    pub elements: ElementsDiff,
    pub materials: MaterialsDiff,
    pub sections: SectionsDiff,
    pub solids: SolidsDiff,
    pub supports: SupportsDiff,
    pub load_cases: LoadCasesDiff,
    pub combinations: CombinationsDiff,
    pub analysis: Option<FemAnalysisSettings>,
}

impl OperationDiff<Fem3dDocument> for Fem3dDiff {
    fn apply(&self, projection: &Fem3dDocument) -> Fem3dDocument {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        apply_collection_diff(&mut next.nodes, &self.nodes);
        apply_collection_diff(&mut next.elements, &self.elements);
        apply_collection_diff(&mut next.materials, &self.materials);
        apply_collection_diff(&mut next.sections, &self.sections);
        apply_collection_diff(&mut next.solids, &self.solids);
        apply_collection_diff(&mut next.supports, &self.supports);
        apply_collection_diff(&mut next.load_cases, &self.load_cases);
        apply_collection_diff(&mut next.combinations, &self.combinations);
        if let Some(analysis) = &self.analysis {
            next.analysis = analysis.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.document.is_some() {
            *self = Fem3dDiff { document: other.document, ..Default::default() };
            return;
        }
        self.nodes.absorb(other.nodes);
        self.elements.absorb(other.elements);
        self.materials.absorb(other.materials);
        self.sections.absorb(other.sections);
        self.solids.absorb(other.solids);
        self.supports.absorb(other.supports);
        self.load_cases.absorb(other.load_cases);
        self.combinations.absorb(other.combinations);
        if other.analysis.is_some() {
            self.analysis = other.analysis;
        }
    }
}
// #endregion 🔖️Fem3dDiff

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_collection_diff_inserts_and_replaces_by_id() {
        let mut nodes: Vec<FemNode> = vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 }];
        let diff = NodesDiff { removed: vec![], set: vec![(1, FemNode { id: "n2".into(), x: 1.0, y: 1.0, z: 1.0 })] };
        apply_collection_diff(&mut nodes, &diff);
        assert_eq!(nodes.len(), 2);
        let replace = NodesDiff { removed: vec![], set: vec![(0, FemNode { id: "n1".into(), x: 9.0, y: 9.0, z: 9.0 })] };
        apply_collection_diff(&mut nodes, &replace);
        assert_eq!(nodes[0].x, 9.0);
    }

    #[test]
    fn apply_collection_diff_removes_by_id() {
        let mut nodes: Vec<FemNode> = vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 }];
        let diff = NodesDiff { removed: vec!["n1".into()], set: vec![] };
        apply_collection_diff(&mut nodes, &diff);
        assert!(nodes.is_empty());
    }

    #[test]
    fn document_diff_wins_over_granular_changes_on_absorb() {
        let base = Fem3dDocument::default();
        let mut diff = Fem3dDiff { analysis: Some(FemAnalysisSettings { modal_count: 1, buckling_count: 1, deformation_scale: 1.0 }), ..Default::default() };
        diff.absorb(Fem3dDiff { document: Some(base.clone()), ..Default::default() });
        assert_eq!(diff.apply(&base), base);
    }
}
// #endregion 🧪️Tests
