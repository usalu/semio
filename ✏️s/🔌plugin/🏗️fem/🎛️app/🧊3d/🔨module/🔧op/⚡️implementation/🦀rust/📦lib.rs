//! ⚡ FEM 3D app — operation enum + laws (constitutional: op).

use fem3d::{element_id, Fem3dDocument, FemAnalysisSettings, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemSection, FemSolid, FemSupport};
use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};
use store::{DocumentEnvelope, DocumentStore};

// #region 🔖Collections
/// 🩹 Sparse id-keyed collection diff — removals plus id-or-index `set`s (replace when the id already
/// exists, else insert at the recorded index). Mirrors `procedural_2d`'s `WidgetsDiff` pattern so
/// disjoint edits from concurrent peers merge cleanly.
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
pub struct SolidsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, FemSolid)>,
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
pub struct CombinationsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, FemCombination)>,
}

fn apply_nodes_diff(nodes: &mut Vec<FemNode>, diff: &NodesDiff) {
    for id in &diff.removed {
        nodes.retain(|entry| &entry.id != id);
    }
    for (index, node) in &diff.set {
        if let Some(pos) = nodes.iter().position(|entry| entry.id == node.id) {
            nodes[pos] = node.clone();
        } else {
            nodes.insert((*index).min(nodes.len()), node.clone());
        }
    }
}

fn apply_elements_diff(elements: &mut Vec<FemElement>, diff: &ElementsDiff) {
    for id in &diff.removed {
        elements.retain(|entry| element_id(entry) != id);
    }
    for (index, element) in &diff.set {
        if let Some(pos) = elements.iter().position(|entry| element_id(entry) == element_id(element)) {
            elements[pos] = element.clone();
        } else {
            elements.insert((*index).min(elements.len()), element.clone());
        }
    }
}

fn apply_materials_diff(materials: &mut Vec<FemMaterial>, diff: &MaterialsDiff) {
    for id in &diff.removed {
        materials.retain(|entry| &entry.id != id);
    }
    for (index, material) in &diff.set {
        if let Some(pos) = materials.iter().position(|entry| entry.id == material.id) {
            materials[pos] = material.clone();
        } else {
            materials.insert((*index).min(materials.len()), material.clone());
        }
    }
}

fn apply_sections_diff(sections: &mut Vec<FemSection>, diff: &SectionsDiff) {
    for id in &diff.removed {
        sections.retain(|entry| &entry.id != id);
    }
    for (index, section) in &diff.set {
        if let Some(pos) = sections.iter().position(|entry| entry.id == section.id) {
            sections[pos] = section.clone();
        } else {
            sections.insert((*index).min(sections.len()), section.clone());
        }
    }
}

fn apply_solids_diff(solids: &mut Vec<FemSolid>, diff: &SolidsDiff) {
    for id in &diff.removed {
        solids.retain(|entry| &entry.id != id);
    }
    for (index, solid) in &diff.set {
        if let Some(pos) = solids.iter().position(|entry| entry.id == solid.id) {
            solids[pos] = solid.clone();
        } else {
            solids.insert((*index).min(solids.len()), solid.clone());
        }
    }
}

fn apply_supports_diff(supports: &mut Vec<FemSupport>, diff: &SupportsDiff) {
    for id in &diff.removed {
        supports.retain(|entry| &entry.id != id);
    }
    for (index, support) in &diff.set {
        if let Some(pos) = supports.iter().position(|entry| entry.id == support.id) {
            supports[pos] = support.clone();
        } else {
            supports.insert((*index).min(supports.len()), support.clone());
        }
    }
}

fn apply_load_cases_diff(load_cases: &mut Vec<FemLoadCase>, diff: &LoadCasesDiff) {
    for id in &diff.removed {
        load_cases.retain(|entry| &entry.id != id);
    }
    for (index, load_case) in &diff.set {
        if let Some(pos) = load_cases.iter().position(|entry| entry.id == load_case.id) {
            load_cases[pos] = load_case.clone();
        } else {
            load_cases.insert((*index).min(load_cases.len()), load_case.clone());
        }
    }
}

fn apply_combinations_diff(combinations: &mut Vec<FemCombination>, diff: &CombinationsDiff) {
    for id in &diff.removed {
        combinations.retain(|entry| &entry.id != id);
    }
    for (index, combination) in &diff.set {
        if let Some(pos) = combinations.iter().position(|entry| entry.id == combination.id) {
            combinations[pos] = combination.clone();
        } else {
            combinations.insert((*index).min(combinations.len()), combination.clone());
        }
    }
}
// #endregion 🔖Collections

// #region 🔖Operations
/// 🩹 Sparse fem-3d diff over every document collection (camera is session-only runtime state, not a
/// document field — see `Fem3dPlayApp::camera` in the ui crate).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem3dDiff {
    /// 🌍 Whole-document replacement (example import / reset); wins over every granular field below.
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
        apply_nodes_diff(&mut next.nodes, &self.nodes);
        apply_elements_diff(&mut next.elements, &self.elements);
        apply_materials_diff(&mut next.materials, &self.materials);
        apply_sections_diff(&mut next.sections, &self.sections);
        apply_solids_diff(&mut next.solids, &self.solids);
        apply_supports_diff(&mut next.supports, &self.supports);
        apply_load_cases_diff(&mut next.load_cases, &self.load_cases);
        apply_combinations_diff(&mut next.combinations, &self.combinations);
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
        self.nodes.removed.extend(other.nodes.removed);
        self.nodes.set.extend(other.nodes.set);
        self.elements.removed.extend(other.elements.removed);
        self.elements.set.extend(other.elements.set);
        self.materials.removed.extend(other.materials.removed);
        self.materials.set.extend(other.materials.set);
        self.sections.removed.extend(other.sections.removed);
        self.sections.set.extend(other.sections.set);
        self.solids.removed.extend(other.solids.removed);
        self.solids.set.extend(other.solids.set);
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

/// 🧮 Fem-3d operation: id-keyed collection edits over nodes/elements/materials/sections/supports/load
/// cases, each with a true inverse via `backwards`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Fem3dOperation {
    SetNode { index: usize, node: FemNode },
    RemoveNode { id: String },
    // `FemElement` is a `DslEnum` (tagged, data-carrying variants), not a `DslRecord`, so it has no
    // `DslField` impl of its own — a bare scalar field can't bind it directly. `#[dsl(statements)]`
    // on a `Box<T>` is the engine's "exactly one required tagged value" shape for that case.
    SetElement {
        index: usize,
        #[dsl(statements)]
        element: Box<FemElement>,
    },
    RemoveElement { id: String },
    SetMaterial { index: usize, material: FemMaterial },
    RemoveMaterial { id: String },
    SetSection { index: usize, section: FemSection },
    RemoveSection { id: String },
    SetSolid { index: usize, solid: FemSolid },
    RemoveSolid { id: String },
    SetSupport { index: usize, support: FemSupport },
    RemoveSupport { id: String },
    SetLoadCase { index: usize, load_case: FemLoadCase },
    RemoveLoadCase { id: String },
    SetCombination { index: usize, combination: FemCombination },
    RemoveCombination { id: String },
    SetAnalysisSettings {
        #[dsl(block)]
        settings: FemAnalysisSettings,
    },
    /// 🌍 Replaces the whole document (example import / reset).
    SetDocument {
        #[dsl(block)]
        document: Fem3dDocument,
    },
}

fn node_index(doc: &Fem3dDocument, id: &str) -> Option<usize> {
    doc.nodes.iter().position(|entry| entry.id == id)
}

fn element_index(doc: &Fem3dDocument, id: &str) -> Option<usize> {
    doc.elements.iter().position(|entry| element_id(entry) == id)
}

fn material_index(doc: &Fem3dDocument, id: &str) -> Option<usize> {
    doc.materials.iter().position(|entry| entry.id == id)
}

fn section_index(doc: &Fem3dDocument, id: &str) -> Option<usize> {
    doc.sections.iter().position(|entry| entry.id == id)
}

fn solid_index(doc: &Fem3dDocument, id: &str) -> Option<usize> {
    doc.solids.iter().position(|entry| entry.id == id)
}

fn support_index(doc: &Fem3dDocument, id: &str) -> Option<usize> {
    doc.supports.iter().position(|entry| entry.id == id)
}

fn load_case_index(doc: &Fem3dDocument, id: &str) -> Option<usize> {
    doc.load_cases.iter().position(|entry| entry.id == id)
}

fn combination_index(doc: &Fem3dDocument, id: &str) -> Option<usize> {
    doc.combinations.iter().position(|entry| entry.id == id)
}

impl Operation<Fem3dDocument> for Fem3dOperation {
    type Diff = Fem3dDiff;

    fn diff(&self, _projection: &Fem3dDocument) -> Fem3dDiff {
        let mut diff = Fem3dDiff::default();
        match self {
            Fem3dOperation::SetNode { index, node } => diff.nodes.set.push((*index, node.clone())),
            Fem3dOperation::RemoveNode { id } => diff.nodes.removed.push(id.clone()),
            Fem3dOperation::SetElement { index, element } => diff.elements.set.push((*index, element.as_ref().clone())),
            Fem3dOperation::RemoveElement { id } => diff.elements.removed.push(id.clone()),
            Fem3dOperation::SetMaterial { index, material } => diff.materials.set.push((*index, material.clone())),
            Fem3dOperation::RemoveMaterial { id } => diff.materials.removed.push(id.clone()),
            Fem3dOperation::SetSection { index, section } => diff.sections.set.push((*index, section.clone())),
            Fem3dOperation::RemoveSection { id } => diff.sections.removed.push(id.clone()),
            Fem3dOperation::SetSolid { index, solid } => diff.solids.set.push((*index, solid.clone())),
            Fem3dOperation::RemoveSolid { id } => diff.solids.removed.push(id.clone()),
            Fem3dOperation::SetSupport { index, support } => diff.supports.set.push((*index, support.clone())),
            Fem3dOperation::RemoveSupport { id } => diff.supports.removed.push(id.clone()),
            Fem3dOperation::SetLoadCase { index, load_case } => diff.load_cases.set.push((*index, load_case.clone())),
            Fem3dOperation::RemoveLoadCase { id } => diff.load_cases.removed.push(id.clone()),
            Fem3dOperation::SetCombination { index, combination } => diff.combinations.set.push((*index, combination.clone())),
            Fem3dOperation::RemoveCombination { id } => diff.combinations.removed.push(id.clone()),
            Fem3dOperation::SetAnalysisSettings { settings } => diff.analysis = Some(settings.clone()),
            Fem3dOperation::SetDocument { document } => diff.document = Some(document.clone()),
        }
        diff
    }

    fn backwards(&self, projection: &Fem3dDocument) -> Vec<Self> {
        match self {
            Fem3dOperation::SetNode { node, .. } => match node_index(projection, &node.id) {
                Some(index) => vec![Fem3dOperation::SetNode { index, node: projection.nodes[index].clone() }],
                None => vec![Fem3dOperation::RemoveNode { id: node.id.clone() }],
            },
            Fem3dOperation::RemoveNode { id } => node_index(projection, id).map(|index| vec![Fem3dOperation::SetNode { index, node: projection.nodes[index].clone() }]).unwrap_or_default(),
            Fem3dOperation::SetElement { element, .. } => match element_index(projection, element_id(element)) {
                Some(index) => vec![Fem3dOperation::SetElement { index, element: Box::new(projection.elements[index].clone()) }],
                None => vec![Fem3dOperation::RemoveElement { id: element_id(element).to_string() }],
            },
            Fem3dOperation::RemoveElement { id } => {
                element_index(projection, id).map(|index| vec![Fem3dOperation::SetElement { index, element: Box::new(projection.elements[index].clone()) }]).unwrap_or_default()
            }
            Fem3dOperation::SetMaterial { material, .. } => match material_index(projection, &material.id) {
                Some(index) => vec![Fem3dOperation::SetMaterial { index, material: projection.materials[index].clone() }],
                None => vec![Fem3dOperation::RemoveMaterial { id: material.id.clone() }],
            },
            Fem3dOperation::RemoveMaterial { id } => material_index(projection, id).map(|index| vec![Fem3dOperation::SetMaterial { index, material: projection.materials[index].clone() }]).unwrap_or_default(),
            Fem3dOperation::SetSection { section, .. } => match section_index(projection, &section.id) {
                Some(index) => vec![Fem3dOperation::SetSection { index, section: projection.sections[index].clone() }],
                None => vec![Fem3dOperation::RemoveSection { id: section.id.clone() }],
            },
            Fem3dOperation::RemoveSection { id } => section_index(projection, id).map(|index| vec![Fem3dOperation::SetSection { index, section: projection.sections[index].clone() }]).unwrap_or_default(),
            Fem3dOperation::SetSolid { solid, .. } => match solid_index(projection, &solid.id) {
                Some(index) => vec![Fem3dOperation::SetSolid { index, solid: projection.solids[index].clone() }],
                None => vec![Fem3dOperation::RemoveSolid { id: solid.id.clone() }],
            },
            Fem3dOperation::RemoveSolid { id } => solid_index(projection, id).map(|index| vec![Fem3dOperation::SetSolid { index, solid: projection.solids[index].clone() }]).unwrap_or_default(),
            Fem3dOperation::SetSupport { support, .. } => match support_index(projection, &support.id) {
                Some(index) => vec![Fem3dOperation::SetSupport { index, support: projection.supports[index].clone() }],
                None => vec![Fem3dOperation::RemoveSupport { id: support.id.clone() }],
            },
            Fem3dOperation::RemoveSupport { id } => support_index(projection, id).map(|index| vec![Fem3dOperation::SetSupport { index, support: projection.supports[index].clone() }]).unwrap_or_default(),
            Fem3dOperation::SetLoadCase { load_case, .. } => match load_case_index(projection, &load_case.id) {
                Some(index) => vec![Fem3dOperation::SetLoadCase { index, load_case: projection.load_cases[index].clone() }],
                None => vec![Fem3dOperation::RemoveLoadCase { id: load_case.id.clone() }],
            },
            Fem3dOperation::RemoveLoadCase { id } => load_case_index(projection, id).map(|index| vec![Fem3dOperation::SetLoadCase { index, load_case: projection.load_cases[index].clone() }]).unwrap_or_default(),
            Fem3dOperation::SetCombination { combination, .. } => match combination_index(projection, &combination.id) {
                Some(index) => vec![Fem3dOperation::SetCombination { index, combination: projection.combinations[index].clone() }],
                None => vec![Fem3dOperation::RemoveCombination { id: combination.id.clone() }],
            },
            Fem3dOperation::RemoveCombination { id } => combination_index(projection, id).map(|index| vec![Fem3dOperation::SetCombination { index, combination: projection.combinations[index].clone() }]).unwrap_or_default(),
            Fem3dOperation::SetAnalysisSettings { .. } => vec![Fem3dOperation::SetAnalysisSettings { settings: projection.analysis.clone() }],
            Fem3dOperation::SetDocument { .. } => vec![Fem3dOperation::SetDocument { document: projection.clone() }],
        }
    }
}
// #endregion 🔖Operations

pub type Fem3dEnvelope = DocumentEnvelope<Fem3dDocument, Fem3dOperation>;
pub type Fem3dStore = DocumentStore<Fem3dDocument, Fem3dOperation>;

// #region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use fem3d::FemDof;
    use std::collections::BTreeMap;

    // #region 🔖Fixtures
    fn cantilever_fixture() -> (Fem3dDocument, f64, f64, f64, f64, f64) {
        let e = 210e9;
        let g = 80.77e9;
        let a = 0.00538;
        let iy = 0.0000369;
        let iz = 0.0000133;
        let j = 0.00000060;
        let l = 3.0;
        let p = 5000.0;
        let doc = Fem3dDocument {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNode { id: "n2".into(), x: l, y: 0.0, z: 0.0 }],
            elements: vec![FemElement::Frame { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.0 }],
            materials: vec![FemMaterial { id: "steel".into(), name: "Steel".into(), e, g, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "hea200".into(), name: "HEA200".into(), area: a, iy, iz, j }],
            solids: vec![],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: FemDof::ALL.to_vec() }],
            load_cases: vec![FemLoadCase { id: "point".into(), name: "Point Load".into(), loads: vec![fem3d::FemLoad::Nodal { id: "l1".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -p }], self_weight: false }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        };
        (doc, e, iy, l, p, iz)
    }

    /// 🧱 A 2m x 1m x 0.5m slab footprint at the origin, meshed at `mesh_size`, with all 4 footprint
    /// corners as pre-placed document nodes fully fixed in translation (`Tet4` has no rotational DOF) —
    /// mirrors `fem_2d`'s `rectangle_region_doc` fixture pattern for `FemSolid`.
    fn solid_slab_doc() -> Fem3dDocument {
        Fem3dDocument {
            nodes: vec![
                FemNode { id: "sc0".into(), x: 0.0, y: 0.0, z: 0.0 },
                FemNode { id: "sc1".into(), x: 2.0, y: 0.0, z: 0.0 },
                FemNode { id: "sc2".into(), x: 2.0, y: 1.0, z: 0.0 },
                FemNode { id: "sc3".into(), x: 0.0, y: 1.0, z: 0.0 },
            ],
            elements: vec![],
            materials: vec![FemMaterial { id: "concrete".into(), name: "Concrete".into(), e: 30e9, g: 12.5e9, nu: 0.2, rho: 2400.0 }],
            sections: vec![],
            solids: vec![FemSolid { id: "sol1".into(), name: "Slab".into(), outline: vec![[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]], holes: vec![], base_z: 0.0, height: 0.5, layers: 1, mesh_size: 1.0, material_id: "concrete".into() }],
            supports: vec![
                FemSupport { id: "s1".into(), node_id: "sc0".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
                FemSupport { id: "s2".into(), node_id: "sc1".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
                FemSupport { id: "s3".into(), node_id: "sc2".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
                FemSupport { id: "s4".into(), node_id: "sc3".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
            ],
            load_cases: vec![FemLoadCase { id: "self".into(), name: "Self Weight".into(), loads: vec![], self_weight: true }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }
    // #endregion 🔖Fixtures

    // #region 🔖OpRoundTrip
    fn round_trip(projection: &Fem3dDocument, operation: &Fem3dOperation) -> Fem3dDocument {
        let forward = vcs::apply_operation(projection, operation);
        let mut restored = forward.clone();
        for back in operation.backwards(projection) {
            restored = vcs::apply_operation(&restored, &back);
        }
        assert_eq!(&restored, projection, "backwards() must restore the pre-operation document");
        forward
    }

    #[test]
    fn node_set_and_remove_round_trip() {
        let base = Fem3dDocument::default();
        let node = FemNode { id: "n1".into(), x: 1.0, y: 2.0, z: 3.0 };
        let after_set = round_trip(&base, &Fem3dOperation::SetNode { index: 0, node: node.clone() });
        assert_eq!(after_set.nodes, vec![node.clone()]);
        round_trip(&after_set, &Fem3dOperation::RemoveNode { id: node.id });
    }

    #[test]
    fn element_set_and_remove_round_trip() {
        let (base, ..) = cantilever_fixture();
        let updated = FemElement::Frame { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.5 };
        let after_set = round_trip(&base, &Fem3dOperation::SetElement { index: 0, element: Box::new(updated) });
        round_trip(&after_set, &Fem3dOperation::RemoveElement { id: "e1".into() });
    }

    #[test]
    fn material_set_and_remove_round_trip() {
        let (base, ..) = cantilever_fixture();
        let material = FemMaterial { id: "steel".into(), name: "Steel Updated".into(), e: 200e9, g: 79e9, nu: 0.3, rho: 7900.0 };
        let after_set = round_trip(&base, &Fem3dOperation::SetMaterial { index: 0, material });
        round_trip(&after_set, &Fem3dOperation::RemoveMaterial { id: "steel".into() });
    }

    #[test]
    fn section_set_and_remove_round_trip() {
        let (base, ..) = cantilever_fixture();
        let section = FemSection { id: "hea200".into(), name: "HEA200 Updated".into(), area: 0.006, iy: 4e-5, iz: 1.5e-5, j: 7e-7 };
        let after_set = round_trip(&base, &Fem3dOperation::SetSection { index: 0, section });
        round_trip(&after_set, &Fem3dOperation::RemoveSection { id: "hea200".into() });
    }

    #[test]
    fn support_set_and_remove_round_trip() {
        let (base, ..) = cantilever_fixture();
        let support = FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] };
        let after_set = round_trip(&base, &Fem3dOperation::SetSupport { index: 0, support });
        round_trip(&after_set, &Fem3dOperation::RemoveSupport { id: "s1".into() });
    }

    #[test]
    fn load_case_set_and_remove_round_trip() {
        let (base, ..) = cantilever_fixture();
        let load_case = FemLoadCase { id: "point".into(), name: "Point Load Updated".into(), loads: vec![fem3d::FemLoad::Nodal { id: "l1".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -9000.0 }], self_weight: false };
        let after_set = round_trip(&base, &Fem3dOperation::SetLoadCase { index: 0, load_case });
        round_trip(&after_set, &Fem3dOperation::RemoveLoadCase { id: "point".into() });
    }

    #[test]
    fn combination_set_and_remove_round_trip() {
        let (base, ..) = cantilever_fixture();
        let combination = FemCombination { id: "uls".into(), name: "ULS".into(), terms: BTreeMap::from([("point".into(), 1.35)]) };
        let after_set = round_trip(&base, &Fem3dOperation::SetCombination { index: 0, combination });
        round_trip(&after_set, &Fem3dOperation::RemoveCombination { id: "uls".into() });
    }

    #[test]
    fn analysis_settings_set_round_trips() {
        let base = Fem3dDocument::default();
        let settings = FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 25.0 };
        round_trip(&base, &Fem3dOperation::SetAnalysisSettings { settings });
    }

    #[test]
    fn document_op_round_trips() {
        let (base, ..) = cantilever_fixture();
        let replacement = solid_slab_doc();
        let after = round_trip(&base, &Fem3dOperation::SetDocument { document: replacement.clone() });
        assert_eq!(after, replacement);
    }

    #[test]
    fn document_diff_absorb_wins_over_granular_changes() {
        let (base, ..) = cantilever_fixture();
        let replacement = solid_slab_doc();
        let mut diff = Fem3dOperation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 1, buckling_count: 1, deformation_scale: 1.0 } }.diff(&base);
        diff.absorb(Fem3dOperation::SetDocument { document: replacement.clone() }.diff(&base));
        assert_eq!(diff.apply(&base), replacement);
    }

    #[test]
    fn solid_op_round_trips() {
        let base = solid_slab_doc();
        let updated = FemSolid { id: "sol1".into(), name: "Slab Updated".into(), outline: base.solids[0].outline.clone(), holes: vec![], base_z: 0.0, height: 0.8, layers: 2, mesh_size: 0.5, material_id: "concrete".into() };
        let after_set = round_trip(&base, &Fem3dOperation::SetSolid { index: 0, solid: updated });
        assert_eq!(after_set.solids[0].height, 0.8);
        round_trip(&after_set, &Fem3dOperation::RemoveSolid { id: "sol1".into() });
    }
    // #endregion 🔖OpRoundTrip

    // #region 🔖OpText
    #[test]
    fn fem3d_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::SetNode { index: 0, node: FemNode { id: "n1".into(), x: 1.0, y: 2.0, z: 3.0 } });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::RemoveNode { id: "n1".into() });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::SetElement {
            index: 0,
            element: Box::new(FemElement::Frame { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.5 }),
        });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::SetElement {
            index: 0,
            element: Box::new(FemElement::Bar { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "rod".into() }),
        });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::RemoveElement { id: "e1".into() });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::SetMaterial { index: 0, material: FemMaterial { id: "steel".into(), name: "Steel".into(), e: 210e9, g: 80.77e9, nu: 0.3, rho: 7850.0 } });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::RemoveMaterial { id: "steel".into() });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::SetSection { index: 0, section: FemSection { id: "hea200".into(), name: "HEA200".into(), area: 0.00538, iy: 3.69e-5, iz: 1.33e-5, j: 6.0e-7 } });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::RemoveSection { id: "hea200".into() });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::SetSolid {
            index: 0,
            solid: FemSolid {
                id: "sol1".into(),
                name: "Slab".into(),
                outline: vec![[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]],
                holes: vec![vec![[0.5, 0.25], [1.5, 0.25], [1.5, 0.75]]],
                base_z: 0.0,
                height: 0.5,
                layers: 2,
                mesh_size: 0.5,
                material_id: "concrete".into(),
            },
        });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::RemoveSolid { id: "sol1".into() });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::SetSupport { index: 0, support: FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: FemDof::ALL.to_vec() } });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::RemoveSupport { id: "s1".into() });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::SetLoadCase {
            index: 0,
            load_case: FemLoadCase {
                id: "point".into(),
                name: "Point Load".into(),
                loads: vec![
                    fem3d::FemLoad::Nodal { id: "l1".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -5000.0 },
                    fem3d::FemLoad::MemberUdl { id: "l2".into(), element_id: "e1".into(), wx: 0.0, wy: 0.0, wz: -800.0 },
                    fem3d::FemLoad::Area { id: "l3".into(), solid_id: "sol1".into(), pressure: 800.0 },
                ],
                self_weight: true,
            },
        });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::RemoveLoadCase { id: "point".into() });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::SetCombination { index: 0, combination: FemCombination { id: "uls".into(), name: "ULS".into(), terms: BTreeMap::from([("point".into(), 1.35), ("live".into(), 1.5)]) } });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::RemoveCombination { id: "uls".into() });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } });
        let (cantilever, ..) = cantilever_fixture();
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::SetDocument { document: cantilever });
    }
    // #endregion 🔖OpText
}
// #endregion 🧪Tests
