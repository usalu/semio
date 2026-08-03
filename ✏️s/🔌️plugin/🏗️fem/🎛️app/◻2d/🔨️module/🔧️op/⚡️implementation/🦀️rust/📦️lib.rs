//! ⚡️ FEM 2D app — operation enum + laws (constitutional: op).

use fem2d::{element_id, Fem2dDocument, FemAnalysisSettings, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};
use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};
use store::{DocumentEnvelope, DocumentStore};

// #region 🔖️Collections
/// 🪪️ Stable-id accessor shared by every id-keyed document collection entry.
trait HasId {
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

fn index_of<T: HasId>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}
// #endregion 🔖️Collections

// #region 🔖️Operations
/// 🩹️ Sparse fem-2d diff over every document collection (camera is session-only runtime state, not a
/// document field — see `Fem2dPlayApp::camera` in the ui crate).
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

impl OperationDiff<Fem2dDocument> for Fem2dDiff {
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

/// 🧮️ Fem-2d operation: id-keyed document-collection edits, each with a true inverse computed from
/// the pre-operation projection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Fem2dOperation {
    SetNode { index: usize, #[dsl(block)] node: FemNode },
    RemoveNode { id: String },
    SetElement { index: usize, #[dsl(statements)] element: Box<FemElement> },
    RemoveElement { id: String },
    SetMaterial { index: usize, #[dsl(block)] material: FemMaterial },
    RemoveMaterial { id: String },
    SetSection { index: usize, #[dsl(block)] section: FemSection },
    RemoveSection { id: String },
    SetSupport { index: usize, #[dsl(block)] support: FemSupport },
    RemoveSupport { id: String },
    SetLoadCase { index: usize, #[dsl(block)] load_case: FemLoadCase },
    RemoveLoadCase { id: String },
    SetRegion { index: usize, #[dsl(block)] region: FemRegion },
    RemoveRegion { id: String },
    SetCombination { index: usize, #[dsl(block)] combination: FemCombination },
    RemoveCombination { id: String },
    SetAnalysisSettings { #[dsl(block)] settings: FemAnalysisSettings },
    /// 🌍️ Replaces the whole document (example import / reset).
    SetDocument { #[dsl(block)] document: Fem2dDocument },
}

impl Operation<Fem2dDocument> for Fem2dOperation {
    type Diff = Fem2dDiff;

    fn diff(&self, _projection: &Fem2dDocument) -> Fem2dDiff {
        let mut diff = Fem2dDiff::default();
        match self {
            Fem2dOperation::SetNode { index, node } => diff.nodes.set.push((*index, node.clone())),
            Fem2dOperation::RemoveNode { id } => diff.nodes.removed.push(id.clone()),
            Fem2dOperation::SetElement { index, element } => diff.elements.set.push((*index, (**element).clone())),
            Fem2dOperation::RemoveElement { id } => diff.elements.removed.push(id.clone()),
            Fem2dOperation::SetMaterial { index, material } => diff.materials.set.push((*index, material.clone())),
            Fem2dOperation::RemoveMaterial { id } => diff.materials.removed.push(id.clone()),
            Fem2dOperation::SetSection { index, section } => diff.sections.set.push((*index, section.clone())),
            Fem2dOperation::RemoveSection { id } => diff.sections.removed.push(id.clone()),
            Fem2dOperation::SetSupport { index, support } => diff.supports.set.push((*index, support.clone())),
            Fem2dOperation::RemoveSupport { id } => diff.supports.removed.push(id.clone()),
            Fem2dOperation::SetLoadCase { index, load_case } => diff.load_cases.set.push((*index, load_case.clone())),
            Fem2dOperation::RemoveLoadCase { id } => diff.load_cases.removed.push(id.clone()),
            Fem2dOperation::SetRegion { index, region } => diff.regions.set.push((*index, region.clone())),
            Fem2dOperation::RemoveRegion { id } => diff.regions.removed.push(id.clone()),
            Fem2dOperation::SetCombination { index, combination } => diff.combinations.set.push((*index, combination.clone())),
            Fem2dOperation::RemoveCombination { id } => diff.combinations.removed.push(id.clone()),
            Fem2dOperation::SetAnalysisSettings { settings } => diff.analysis = Some(settings.clone()),
            Fem2dOperation::SetDocument { document } => diff.document = Some(document.clone()),
        }
        diff
    }

    fn backwards(&self, projection: &Fem2dDocument) -> Vec<Self> {
        match self {
            Fem2dOperation::SetNode { node, .. } => match index_of(&projection.nodes, &node.id) {
                Some(index) => vec![Fem2dOperation::SetNode { index, node: projection.nodes[index].clone() }],
                None => vec![Fem2dOperation::RemoveNode { id: node.id.clone() }],
            },
            Fem2dOperation::RemoveNode { id } => index_of(&projection.nodes, id).map(|index| vec![Fem2dOperation::SetNode { index, node: projection.nodes[index].clone() }]).unwrap_or_default(),
            Fem2dOperation::SetElement { element, .. } => match index_of(&projection.elements, element_id(element)) {
                Some(index) => vec![Fem2dOperation::SetElement { index, element: Box::new(projection.elements[index].clone()) }],
                None => vec![Fem2dOperation::RemoveElement { id: element_id(element).to_string() }],
            },
            Fem2dOperation::RemoveElement { id } => index_of(&projection.elements, id).map(|index| vec![Fem2dOperation::SetElement { index, element: Box::new(projection.elements[index].clone()) }]).unwrap_or_default(),
            Fem2dOperation::SetMaterial { material, .. } => match index_of(&projection.materials, &material.id) {
                Some(index) => vec![Fem2dOperation::SetMaterial { index, material: projection.materials[index].clone() }],
                None => vec![Fem2dOperation::RemoveMaterial { id: material.id.clone() }],
            },
            Fem2dOperation::RemoveMaterial { id } => index_of(&projection.materials, id).map(|index| vec![Fem2dOperation::SetMaterial { index, material: projection.materials[index].clone() }]).unwrap_or_default(),
            Fem2dOperation::SetSection { section, .. } => match index_of(&projection.sections, &section.id) {
                Some(index) => vec![Fem2dOperation::SetSection { index, section: projection.sections[index].clone() }],
                None => vec![Fem2dOperation::RemoveSection { id: section.id.clone() }],
            },
            Fem2dOperation::RemoveSection { id } => index_of(&projection.sections, id).map(|index| vec![Fem2dOperation::SetSection { index, section: projection.sections[index].clone() }]).unwrap_or_default(),
            Fem2dOperation::SetSupport { support, .. } => match index_of(&projection.supports, &support.id) {
                Some(index) => vec![Fem2dOperation::SetSupport { index, support: projection.supports[index].clone() }],
                None => vec![Fem2dOperation::RemoveSupport { id: support.id.clone() }],
            },
            Fem2dOperation::RemoveSupport { id } => index_of(&projection.supports, id).map(|index| vec![Fem2dOperation::SetSupport { index, support: projection.supports[index].clone() }]).unwrap_or_default(),
            Fem2dOperation::SetLoadCase { load_case, .. } => match index_of(&projection.load_cases, &load_case.id) {
                Some(index) => vec![Fem2dOperation::SetLoadCase { index, load_case: projection.load_cases[index].clone() }],
                None => vec![Fem2dOperation::RemoveLoadCase { id: load_case.id.clone() }],
            },
            Fem2dOperation::RemoveLoadCase { id } => index_of(&projection.load_cases, id).map(|index| vec![Fem2dOperation::SetLoadCase { index, load_case: projection.load_cases[index].clone() }]).unwrap_or_default(),
            Fem2dOperation::SetRegion { region, .. } => match index_of(&projection.regions, &region.id) {
                Some(index) => vec![Fem2dOperation::SetRegion { index, region: projection.regions[index].clone() }],
                None => vec![Fem2dOperation::RemoveRegion { id: region.id.clone() }],
            },
            Fem2dOperation::RemoveRegion { id } => index_of(&projection.regions, id).map(|index| vec![Fem2dOperation::SetRegion { index, region: projection.regions[index].clone() }]).unwrap_or_default(),
            Fem2dOperation::SetCombination { combination, .. } => match index_of(&projection.combinations, &combination.id) {
                Some(index) => vec![Fem2dOperation::SetCombination { index, combination: projection.combinations[index].clone() }],
                None => vec![Fem2dOperation::RemoveCombination { id: combination.id.clone() }],
            },
            Fem2dOperation::RemoveCombination { id } => index_of(&projection.combinations, id).map(|index| vec![Fem2dOperation::SetCombination { index, combination: projection.combinations[index].clone() }]).unwrap_or_default(),
            Fem2dOperation::SetAnalysisSettings { .. } => vec![Fem2dOperation::SetAnalysisSettings { settings: projection.analysis.clone() }],
            Fem2dOperation::SetDocument { .. } => vec![Fem2dOperation::SetDocument { document: projection.clone() }],
        }
    }
}

pub type Fem2dEnvelope = DocumentEnvelope<Fem2dDocument, Fem2dOperation>;
pub type Fem2dStore = DocumentStore<Fem2dDocument, Fem2dOperation>;
// #endregion 🔖️Operations

// #region 🔖️ConfigOperations
/// @emoji 🧮️ B1: `fem2d_engine::Fem2dConfig`'s operation enum — one variant per settled interaction
/// (mirrors the pre-B1 `Fem2dPlayApp` `RefCell` field writes), plus a generic `Snapshot` every variant's
/// `backwards()` returns — mirrors `shooting_op::ShootingConfigOperation`'s identical B1 pilot recipe:
/// since a config-only dispatch is a plain `Apply` (not an `AmendLast`), each tick is its own distinct,
/// real config edit, and "undo this tick" is exactly "restore the whole-config snapshot from just
/// before it". `Operation::Diff` is the WHOLE `Fem2dConfig` (not a granular patch type): `diff()`
/// returns "the full config after this op", and `OperationDiff<Fem2dConfig>::apply` for `Fem2dConfig`
/// itself (`fem2d_engine`) just returns that snapshot verbatim, ignoring `base`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Fem2dConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: fem2d_engine::Fem2dConfig,
    },
    /// 👁️ Was the `setResultDisplay` view action writing `Fem2dPlayApp::result_display`.
    #[dsl(key = "result-display")]
    SetResultDisplay { source_id: Option<String>, mode: String, mode_index: u32 },
    /// 🎥️ Was the `setCamera` view action writing `Fem2dPlayApp::camera`.
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: fem2d::FemCamera,
    },
    /// 🗣️ Was read off the deleted `ViewState::locale` in `app_labels`.
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<fem2d_engine::Fem2dConfig> for Fem2dConfigOperation {
    type Diff = fem2d_engine::Fem2dConfig;

    fn diff(&self, base: &fem2d_engine::Fem2dConfig) -> fem2d_engine::Fem2dConfig {
        let mut next = base.clone();
        match self {
            Fem2dConfigOperation::Snapshot { config } => return config.clone(),
            Fem2dConfigOperation::SetResultDisplay { source_id, mode, mode_index } => {
                next.result_source_id = source_id.clone();
                next.result_mode = mode.clone();
                next.result_mode_index = *mode_index;
            }
            Fem2dConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            Fem2dConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &fem2d_engine::Fem2dConfig) -> Vec<Self> {
        vec![Fem2dConfigOperation::Snapshot { config: base.clone() }]
    }
}
// #endregion 🔖️ConfigOperations

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
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![fem2d::FemDof::Tx, fem2d::FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: vec![fem2d::FemDof::Ty] }],
            load_cases: vec![FemLoadCase { id: "dead".into(), name: "dead".into(), loads: vec![fem2d::FemLoad::MemberUdl { id: "l1".into(), element_id: "e1".into(), wx: 0.0, wy: -10000.0 }], self_weight: false }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }

    /// 🟩️ A 4x2m rectangular region (steel, 0.02m thick, 1m mesh) whose 4 corners are pre-placed as
    /// document nodes — mirrors `fem2d_engine`'s identically named fixture.
    fn rectangle_region_doc() -> Fem2dDocument {
        Fem2dDocument {
            nodes: vec![FemNode { id: "c0".into(), x: 0.0, y: 0.0 }, FemNode { id: "c1".into(), x: 4.0, y: 0.0 }, FemNode { id: "c2".into(), x: 4.0, y: 2.0 }, FemNode { id: "c3".into(), x: 0.0, y: 2.0 }],
            elements: vec![],
            regions: vec![FemRegion { id: "r1".into(), name: "slab".into(), outline: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]], holes: vec![], thickness: 0.02, material_id: "steel".into(), mesh_size: 1.0 }],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![],
            supports: vec![FemSupport { id: "s1".into(), node_id: "c0".into(), fixed: vec![fem2d::FemDof::Tx, fem2d::FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "c1".into(), fixed: vec![fem2d::FemDof::Tx, fem2d::FemDof::Ty] }],
            load_cases: vec![FemLoadCase { id: "self".into(), name: "self weight".into(), loads: vec![], self_weight: true }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }
    // #endregion 🔖️Fixtures

    // #region 🔖️OpRoundTrip
    fn round_trip(projection: &Fem2dDocument, operation: &Fem2dOperation) -> Fem2dDocument {
        let forward = vcs::apply_operation(projection, operation);
        let mut restored = forward.clone();
        for back in operation.backwards(projection) {
            restored = vcs::apply_operation(&restored, &back);
        }
        assert_eq!(&restored, projection, "backwards() must restore the pre-operation document");
        forward
    }

    #[test]
    fn node_op_round_trips() {
        let base = simply_supported_beam_doc();
        let after = round_trip(&base, &Fem2dOperation::SetNode { index: 0, node: FemNode { id: "n1".into(), x: 1.0, y: 1.0 } });
        assert_eq!(after.nodes[0].x, 1.0);
        round_trip(&base, &Fem2dOperation::RemoveNode { id: "n1".into() });
    }

    #[test]
    fn element_op_round_trips() {
        let base = simply_supported_beam_doc();
        let updated = FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() };
        round_trip(&base, &Fem2dOperation::SetElement { index: 0, element: Box::new(updated) });
        round_trip(&base, &Fem2dOperation::RemoveElement { id: "e1".into() });
    }

    #[test]
    fn material_op_round_trips() {
        let base = simply_supported_beam_doc();
        round_trip(&base, &Fem2dOperation::SetMaterial { index: 0, material: FemMaterial { id: "steel".into(), name: "steel".into(), e: 200e9, nu: 0.3, rho: 7850.0 } });
        round_trip(&base, &Fem2dOperation::RemoveMaterial { id: "steel".into() });
    }

    #[test]
    fn section_op_round_trips() {
        let base = simply_supported_beam_doc();
        round_trip(&base, &Fem2dOperation::SetSection { index: 0, section: FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.01, iy: 1e-4 } });
        round_trip(&base, &Fem2dOperation::RemoveSection { id: "ipe300".into() });
    }

    #[test]
    fn support_op_round_trips() {
        let base = simply_supported_beam_doc();
        round_trip(&base, &Fem2dOperation::SetSupport { index: 0, support: FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![fem2d::FemDof::Ty] } });
        round_trip(&base, &Fem2dOperation::RemoveSupport { id: "s1".into() });
    }

    #[test]
    fn load_case_op_round_trips() {
        let base = simply_supported_beam_doc();
        round_trip(&base, &Fem2dOperation::SetLoadCase { index: 0, load_case: FemLoadCase { id: "dead".into(), name: "dead 2".into(), loads: vec![], self_weight: true } });
        round_trip(&base, &Fem2dOperation::RemoveLoadCase { id: "dead".into() });
    }

    #[test]
    fn region_op_round_trips() {
        let base = rectangle_region_doc();
        let updated = FemRegion { id: "r1".into(), name: "slab v2".into(), outline: vec![[0.0, 0.0], [5.0, 0.0], [5.0, 2.0], [0.0, 2.0]], holes: vec![], thickness: 0.03, material_id: "steel".into(), mesh_size: 0.5 };
        let after = round_trip(&base, &Fem2dOperation::SetRegion { index: 0, region: updated });
        assert_eq!(after.regions[0].thickness, 0.03);
        round_trip(&base, &Fem2dOperation::RemoveRegion { id: "r1".into() });
    }

    #[test]
    fn combination_op_round_trips() {
        let mut base = simply_supported_beam_doc();
        base.combinations.push(FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![fem2d::FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }] });
        let updated = FemCombination { id: "uls".into(), name: "ULS v2".into(), terms: vec![fem2d::FemCombinationTerm { case_id: "dead".into(), factor: 1.4 }] };
        let after = round_trip(&base, &Fem2dOperation::SetCombination { index: 0, combination: updated });
        assert_eq!(after.combinations[0].terms[0].factor, 1.4);
        round_trip(&base, &Fem2dOperation::RemoveCombination { id: "uls".into() });
    }

    #[test]
    fn analysis_settings_op_round_trips() {
        let base = simply_supported_beam_doc();
        let after = round_trip(&base, &Fem2dOperation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } });
        assert_eq!(after.analysis.modal_count, 5);
    }

    #[test]
    fn document_op_round_trips() {
        let base = simply_supported_beam_doc();
        let replacement = rectangle_region_doc();
        let after = round_trip(&base, &Fem2dOperation::SetDocument { document: replacement.clone() });
        assert_eq!(after, replacement);
    }

    #[test]
    fn document_diff_absorb_wins_over_granular_changes() {
        let base = simply_supported_beam_doc();
        let replacement = rectangle_region_doc();
        let mut diff = Fem2dOperation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 1, buckling_count: 1, deformation_scale: 1.0 } }.diff(&base);
        diff.absorb(Fem2dOperation::SetDocument { document: replacement.clone() }.diff(&base));
        assert_eq!(diff.apply(&base), replacement);
    }
    // #endregion 🔖️OpRoundTrip

    // #region 🔖️OpText
    #[test]
    fn fem2d_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetNode { index: 0, node: FemNode { id: "n1".into(), x: 1.0, y: 2.0 } });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::RemoveNode { id: "n1".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetElement {
            index: 0,
            element: Box::new(FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }),
        });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetElement {
            index: 0,
            element: Box::new(FemElement::Bar { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "rod".into() }),
        });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::RemoveElement { id: "e1".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetMaterial { index: 0, material: FemMaterial { id: "steel".into(), name: "Steel S235".into(), e: 210e9, nu: 0.3, rho: 7850.0 } });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::RemoveMaterial { id: "steel".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetSection { index: 0, section: FemSection { id: "ipe300".into(), name: "IPE 300".into(), area: 0.005381, iy: 8.356e-5 } });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::RemoveSection { id: "ipe300".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetSupport { index: 0, support: FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![fem2d::FemDof::Tx, fem2d::FemDof::Ty] } });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::RemoveSupport { id: "s1".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetLoadCase {
            index: 0,
            load_case: FemLoadCase {
                id: "dead".into(),
                name: "Dead Load".into(),
                loads: vec![
                    fem2d::FemLoad::Nodal { id: "l1".into(), node_id: "n1".into(), dof: fem2d::FemDof::Ty, value: -1000.0 },
                    fem2d::FemLoad::MemberUdl { id: "l2".into(), element_id: "e1".into(), wx: 0.0, wy: -5000.0 },
                    fem2d::FemLoad::Area { id: "l3".into(), region_id: "r1".into(), pressure: 800.0 },
                ],
                self_weight: true,
            },
        });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::RemoveLoadCase { id: "dead".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetRegion {
            index: 0,
            region: FemRegion {
                id: "r1".into(),
                name: "Slab".into(),
                outline: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]],
                holes: vec![vec![[1.0, 1.0], [2.0, 1.0], [2.0, 1.5]]],
                thickness: 0.02,
                material_id: "steel".into(),
                mesh_size: 0.5,
            },
        });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::RemoveRegion { id: "r1".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetCombination { index: 0, combination: FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![fem2d::FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }, fem2d::FemCombinationTerm { case_id: "live".into(), factor: 1.5 }] } });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::RemoveCombination { id: "uls".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetDocument { document: simply_supported_beam_doc() });
    }
    // #endregion 🔖️OpText

    // #region 🔖️ConfigOperations
    #[test]
    fn config_operation_backwards_always_restores_the_pre_operation_snapshot() {
        let base = fem2d_engine::Fem2dConfig::default();
        let camera = fem2d::FemCamera { x: 1.0, y: 2.0, zoom: 3.0 };
        let op = Fem2dConfigOperation::SetCamera { camera: camera.clone() };
        let next = op.diff(&base);
        assert_eq!(next.camera, camera);
        let backwards = op.backwards(&base);
        assert_eq!(backwards, vec![Fem2dConfigOperation::Snapshot { config: base.clone() }]);
        assert_eq!(backwards[0].diff(&next), base);
    }

    #[test]
    fn set_result_display_config_operation_round_trips() {
        let base = fem2d_engine::Fem2dConfig::default();
        let op = Fem2dConfigOperation::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 2 };
        let next = op.diff(&base);
        assert_eq!(next.result_source_id.as_deref(), Some("dead"));
        assert_eq!(next.result_mode, "modal");
        assert_eq!(next.result_mode_index, 2);
    }

    #[test]
    fn set_locale_config_operation_round_trips() {
        let base = fem2d_engine::Fem2dConfig::default();
        let op = Fem2dConfigOperation::SetLocale { value: "de-DE".into() };
        let next = op.diff(&base);
        assert_eq!(next.locale, "de-DE");
    }

    #[test]
    fn fem2d_config_operation_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&Fem2dConfigOperation::Snapshot { config: fem2d_engine::Fem2dConfig::default() });
        store::test_support::assert_op_line_round_trip(&Fem2dConfigOperation::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 1 });
        store::test_support::assert_op_line_round_trip(&Fem2dConfigOperation::SetCamera { camera: fem2d::FemCamera { x: 1.0, y: 2.0, zoom: 1.5 } });
        store::test_support::assert_op_line_round_trip(&Fem2dConfigOperation::SetLocale { value: "de-DE".into() });
    }
    // #endregion 🔖️ConfigOperations
}
// #endregion 🧪️Tests
