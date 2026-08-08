//! 🩹️ FEM 2D artifact — sparse document diff (constitutional: op, diff half).

use crate::artifacts::fem2d::{element_id, Fem2dDocument, FemAnalysisSettings, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


// #region 🔖️Collections
/// 🪪️ Stable-id accessor shared by every id-keyed document collection entry. `pub(crate)`: `index_of`
/// (below) is called cross-node from `crate::artifacts::fem2d::op`'s `Mutation::inverse` impl, and a
/// private trait cannot appear in a more-visible function's signature.
pub(crate) trait HasId {
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
impl HasId for FemRegion {
    fn id(&self) -> &str {
        &self.id
    }
}
impl HasId for FemCombination {
    fn id(&self) -> &str {
        &self.id
    }
}

/// 🩹️ Sparse id-keyed collection diff — removals plus id-or-index `set`s (replace when the id
/// already exists, else insert at the recorded index).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodesDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, FemNode)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, FemElement)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, FemMaterial)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, FemSection)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, FemSupport)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadCasesDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, FemLoadCase)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegionsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, FemRegion)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombinationsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, FemCombination)>,
}

/// 🩹️ Applies a sparse id-keyed diff to a collection in place — remove-by-id, then replace-by-id
/// or insert-at-index for each `set` entry.
fn apply_collection_diff<T: HasId + Clone>(items: &mut Vec<T>, removed: &[String], set: &[(usize, T)]) {
    for id in removed {
        items.retain(|item| item.id() != id);
    }
    for (index, item) in set {
        if let Some(pos) = items.iter().position(|entry| entry.id() == item.id()) {
            items[pos] = item.clone();
        } else {
            items.insert((*index).min(items.len()), item.clone());
        }
    }
}

/// 🔎️ `pub(crate)`: also called from `crate::artifacts::fem2d::op`'s `Mutation::inverse` impl to
/// locate the pre-operation index/value a removal or replace should invert to.
pub(crate) fn index_of<T: HasId>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}
// #endregion 🔖️Collections

// #region 🔖️Diff
/// 🩹️ Sparse fem-2d diff over every document collection (camera is session-only runtime state, not a
/// document field — see `Fem2dConfig::camera` in the app's `config.rs`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem2dDiff {
    /// 🌍️ Whole-document replacement (example import / reset); wins over every granular field below.
    pub document: Option<Fem2dDocument>,
    pub nodes: NodesDiff,
    pub elements: ElementsDiff,
    pub regions: RegionsDiff,
    pub materials: MaterialsDiff,
    pub sections: SectionsDiff,
    pub supports: SupportsDiff,
    pub load_cases: LoadCasesDiff,
    pub combinations: CombinationsDiff,
    pub analysis: Option<FemAnalysisSettings>,
}

impl MutationDiff<Fem2dDocument> for Fem2dDiff {
    fn apply(&self, projection: &Fem2dDocument) -> Fem2dDocument {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        apply_collection_diff(&mut next.nodes, &self.nodes.removed, &self.nodes.set);
        apply_collection_diff(&mut next.elements, &self.elements.removed, &self.elements.set);
        apply_collection_diff(&mut next.regions, &self.regions.removed, &self.regions.set);
        apply_collection_diff(&mut next.materials, &self.materials.removed, &self.materials.set);
        apply_collection_diff(&mut next.sections, &self.sections.removed, &self.sections.set);
        apply_collection_diff(&mut next.supports, &self.supports.removed, &self.supports.set);
        apply_collection_diff(&mut next.load_cases, &self.load_cases.removed, &self.load_cases.set);
        apply_collection_diff(&mut next.combinations, &self.combinations.removed, &self.combinations.set);
        if let Some(analysis) = &self.analysis {
            next.analysis = analysis.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.document.is_some() {
            *self = Fem2dDiff { document: other.document, ..Default::default() };
            return;
        }
        self.nodes.removed.extend(other.nodes.removed);
        self.nodes.set.extend(other.nodes.set);
        self.elements.removed.extend(other.elements.removed);
        self.elements.set.extend(other.elements.set);
        self.regions.removed.extend(other.regions.removed);
        self.regions.set.extend(other.regions.set);
        self.materials.removed.extend(other.materials.removed);
        self.materials.set.extend(other.materials.set);
        self.sections.removed.extend(other.sections.removed);
        self.sections.set.extend(other.sections.set);
        self.supports.removed.extend(other.supports.removed);
        self.supports.set.extend(other.supports.set);
        self.load_cases.removed.extend(other.load_cases.removed);
        self.load_cases.set.extend(other.load_cases.set);
        self.combinations.removed.extend(other.combinations.removed);
        self.combinations.set.extend(other.combinations.set);
        if other.analysis.is_some() {
            self.analysis = other.analysis;
        }
    }
}
// #endregion 🔖️Diff

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // #region 🔖️Fixtures
    fn simply_supported_beam_doc() -> Fem2dDocument {
        Fem2dDocument {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0 }, FemNode { id: "n2".into(), x: 6.0, y: 0.0 }],
            elements: vec![FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }],
            regions: vec![],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 }],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![crate::artifacts::fem2d::FemDof::Tx, crate::artifacts::fem2d::FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: vec![crate::artifacts::fem2d::FemDof::Ty] }],
            load_cases: vec![FemLoadCase { id: "dead".into(), name: "dead".into(), loads: vec![crate::artifacts::fem2d::FemLoad::MemberUdl { id: "l1".into(), element_id: "e1".into(), wx: 0.0, wy: -10000.0 }], self_weight: false }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }

    /// 🟩️ A 4x2m rectangular region (steel, 0.02m thick, 1m mesh) whose 4 corners are pre-placed as
    /// document nodes.
    fn rectangle_region_doc() -> Fem2dDocument {
        Fem2dDocument {
            nodes: vec![FemNode { id: "c0".into(), x: 0.0, y: 0.0 }, FemNode { id: "c1".into(), x: 4.0, y: 0.0 }, FemNode { id: "c2".into(), x: 4.0, y: 2.0 }, FemNode { id: "c3".into(), x: 0.0, y: 2.0 }],
            elements: vec![],
            regions: vec![FemRegion { id: "r1".into(), name: "slab".into(), outline: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]], holes: vec![], thickness: 0.02, material_id: "steel".into(), mesh_size: 1.0 }],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![],
            supports: vec![FemSupport { id: "s1".into(), node_id: "c0".into(), fixed: vec![crate::artifacts::fem2d::FemDof::Tx, crate::artifacts::fem2d::FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "c1".into(), fixed: vec![crate::artifacts::fem2d::FemDof::Tx, crate::artifacts::fem2d::FemDof::Ty] }],
            load_cases: vec![FemLoadCase { id: "self".into(), name: "self weight".into(), loads: vec![], self_weight: true }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }
    // #endregion 🔖️Fixtures

    #[test]
    fn document_diff_absorb_wins_over_granular_changes() {
        let base = simply_supported_beam_doc();
        let replacement = rectangle_region_doc();
        let mut diff = Fem2dDiff { analysis: Some(FemAnalysisSettings { modal_count: 1, buckling_count: 1, deformation_scale: 1.0 }), ..Default::default() };
        diff.absorb(Fem2dDiff { document: Some(replacement.clone()), ..Default::default() });
        assert_eq!(diff.apply(&base), replacement);
    }

    #[test]
    fn granular_diff_applies_sets_and_removals() {
        let base = simply_supported_beam_doc();
        let diff = Fem2dDiff { nodes: NodesDiff { removed: vec![], set: vec![(0, FemNode { id: "n1".into(), x: 1.0, y: 1.0 })] }, ..Default::default() };
        let after = diff.apply(&base);
        assert_eq!(after.nodes[0].x, 1.0);
        let removal = Fem2dDiff { nodes: NodesDiff { removed: vec!["n1".into()], set: vec![] }, ..Default::default() };
        let after_removal = removal.apply(&base);
        assert!(!after_removal.nodes.iter().any(|n| n.id == "n1"));
    }

    #[test]
    fn index_of_locates_the_pre_operation_position() {
        let base = simply_supported_beam_doc();
        assert_eq!(index_of(&base.nodes, "n1"), Some(0));
        assert_eq!(index_of(&base.nodes, "missing"), None);
    }
}
// #endregion 🧪️Tests

#[cfg(test)]
mod semio_grammar_conformance {
    use super::*;

    #[test]
    fn component_grammar_semio_is_grammar_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_GRAMMAR_SEMIO).expect("parse grammar.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Grammar);
        assert!(!COMPONENT_GRAMMAR_SEMIO.is_empty());
        let _ = COMPONENT_GRAMMAR_PATH;
    }
}

