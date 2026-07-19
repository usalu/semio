//! 🏙️ FEM 3D document model and element library on `vcs`.

#[cfg(test)]
use fem_core::ElementResult;
use fem_core::{analyses, Bar3, Dof, Element, Frame3, Model, NodalLoad, Node, Support};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(target_arch = "wasm32")]
use vcs::create_document_vcs_envelope;
use vcs::{DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff};

pub const FEM_3D_SCHEMA: &str = "fem.3d";

// #region 🔖Document
/// 📍 A structural node: a stable id and a global position, plain SI meters.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemNode {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 🔩 A two-node member: an axial `Bar` or a full 6-DOF `Frame` with a local-axis `roll` angle (radians).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FemElement {
    #[serde(rename_all = "camelCase")]
    Bar { id: String, start: String, end: String, material_id: String, section_id: String },
    #[serde(rename_all = "camelCase")]
    Frame { id: String, start: String, end: String, material_id: String, section_id: String, roll: f64 },
}

/// 🧱 Linear-elastic isotropic material: Young's modulus `e`, shear modulus `g` (Pa), and density
/// `rho` (kg/m³, drives self-weight via `Bar3`/`Frame3`'s `mass()`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemMaterial {
    pub id: String,
    pub name: String,
    pub e: f64,
    pub g: f64,
    pub rho: f64,
}

/// 📐 Cross-section properties: area (m²), second moments of area about local y/z (m⁴), torsion constant (m⁴).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemSection {
    pub id: String,
    pub name: String,
    pub area: f64,
    pub iy: f64,
    pub iz: f64,
    pub j: f64,
}

/// 🔒 A support: the subset of a node's DOFs restrained to zero displacement.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemSupport {
    pub id: String,
    pub node_id: String,
    pub fixed: Vec<Dof>,
}

/// 🏋️ A concentrated load on one node's global DOF, part of a `FemLoadCase`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemNodalLoad {
    pub id: String,
    pub node_id: String,
    pub dof: Dof,
    pub value: f64,
}

/// 📦 A named set of nodal loads solved together, plus an optional self-weight contribution; v0
/// `fem_3d` scope has no member UDLs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemLoadCase {
    pub id: String,
    pub name: String,
    pub loads: Vec<FemNodalLoad>,
    pub self_weight: bool,
}

/// 📦 A linear combination of load cases — `(case_id, factor)` terms superposed from already-solved
/// case results.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemCombination {
    pub id: String,
    pub name: String,
    pub terms: Vec<(String, f64)>,
}

/// ⚙️ Analysis settings: mode/factor counts for modal and buckling analyses, plus a deformation
/// display scale for the UI layer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemAnalysisSettings {
    pub modal_count: usize,
    pub buckling_count: usize,
    pub deformation_scale: f64,
}

impl Default for FemAnalysisSettings {
    fn default() -> Self {
        Self { modal_count: 3, buckling_count: 3, deformation_scale: 50.0 }
    }
}

/// 🎥 Opaque camera state string; the plugin layer owns and interprets its shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemCamera {
    pub json: String,
}

impl Default for FemCamera {
    fn default() -> Self {
        Self { json: "{}".to_string() }
    }
}

/// 🧾 Persistent fem-3d document — nodes, members, catalogs, supports and load cases plus camera state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Fem3dDocument {
    pub nodes: Vec<FemNode>,
    pub elements: Vec<FemElement>,
    pub materials: Vec<FemMaterial>,
    pub sections: Vec<FemSection>,
    pub supports: Vec<FemSupport>,
    pub load_cases: Vec<FemLoadCase>,
    pub combinations: Vec<FemCombination>,
    pub analysis: FemAnalysisSettings,
    pub camera: FemCamera,
}

/// 🪪 A `FemElement`'s stable id, across its `Bar`/`Frame` variants.
fn element_id(element: &FemElement) -> &str {
    match element {
        FemElement::Bar { id, .. } | FemElement::Frame { id, .. } => id,
    }
}
// #endregion 🔖Document

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

// #region 🔖Ops
/// 🩹 Sparse fem-3d diff over every document collection plus the scalar camera field.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem3dDiff {
    pub nodes: NodesDiff,
    pub elements: ElementsDiff,
    pub materials: MaterialsDiff,
    pub sections: SectionsDiff,
    pub supports: SupportsDiff,
    pub load_cases: LoadCasesDiff,
    pub combinations: CombinationsDiff,
    pub camera: Option<FemCamera>,
    pub analysis: Option<FemAnalysisSettings>,
}

impl OperationDiff<Fem3dDocument> for Fem3dDiff {
    fn apply(&self, projection: &Fem3dDocument) -> Fem3dDocument {
        let mut next = projection.clone();
        apply_nodes_diff(&mut next.nodes, &self.nodes);
        apply_elements_diff(&mut next.elements, &self.elements);
        apply_materials_diff(&mut next.materials, &self.materials);
        apply_sections_diff(&mut next.sections, &self.sections);
        apply_supports_diff(&mut next.supports, &self.supports);
        apply_load_cases_diff(&mut next.load_cases, &self.load_cases);
        apply_combinations_diff(&mut next.combinations, &self.combinations);
        if let Some(camera) = &self.camera {
            next.camera = camera.clone();
        }
        if let Some(analysis) = &self.analysis {
            next.analysis = analysis.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        self.nodes.removed.extend(other.nodes.removed);
        self.nodes.set.extend(other.nodes.set);
        self.elements.removed.extend(other.elements.removed);
        self.elements.set.extend(other.elements.set);
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
        if other.camera.is_some() {
            self.camera = other.camera;
        }
        if other.analysis.is_some() {
            self.analysis = other.analysis;
        }
    }
}

/// 🧮 Fem-3d operation: id-keyed collection edits over nodes/elements/materials/sections/supports/load
/// cases, plus the scalar camera, each with a true inverse via `backwards`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum Fem3dOp {
    SetNode { index: usize, node: FemNode },
    RemoveNode { id: String },
    SetElement { index: usize, element: FemElement },
    RemoveElement { id: String },
    SetMaterial { index: usize, material: FemMaterial },
    RemoveMaterial { id: String },
    SetSection { index: usize, section: FemSection },
    RemoveSection { id: String },
    SetSupport { index: usize, support: FemSupport },
    RemoveSupport { id: String },
    SetLoadCase { index: usize, load_case: FemLoadCase },
    RemoveLoadCase { id: String },
    SetCombination { index: usize, combination: FemCombination },
    RemoveCombination { id: String },
    SetCamera { camera: FemCamera },
    SetAnalysisSettings { settings: FemAnalysisSettings },
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

fn support_index(doc: &Fem3dDocument, id: &str) -> Option<usize> {
    doc.supports.iter().position(|entry| entry.id == id)
}

fn load_case_index(doc: &Fem3dDocument, id: &str) -> Option<usize> {
    doc.load_cases.iter().position(|entry| entry.id == id)
}

fn combination_index(doc: &Fem3dDocument, id: &str) -> Option<usize> {
    doc.combinations.iter().position(|entry| entry.id == id)
}

impl Operation<Fem3dDocument> for Fem3dOp {
    type Diff = Fem3dDiff;

    fn diff(&self, _projection: &Fem3dDocument) -> Fem3dDiff {
        let mut diff = Fem3dDiff::default();
        match self {
            Fem3dOp::SetNode { index, node } => diff.nodes.set.push((*index, node.clone())),
            Fem3dOp::RemoveNode { id } => diff.nodes.removed.push(id.clone()),
            Fem3dOp::SetElement { index, element } => diff.elements.set.push((*index, element.clone())),
            Fem3dOp::RemoveElement { id } => diff.elements.removed.push(id.clone()),
            Fem3dOp::SetMaterial { index, material } => diff.materials.set.push((*index, material.clone())),
            Fem3dOp::RemoveMaterial { id } => diff.materials.removed.push(id.clone()),
            Fem3dOp::SetSection { index, section } => diff.sections.set.push((*index, section.clone())),
            Fem3dOp::RemoveSection { id } => diff.sections.removed.push(id.clone()),
            Fem3dOp::SetSupport { index, support } => diff.supports.set.push((*index, support.clone())),
            Fem3dOp::RemoveSupport { id } => diff.supports.removed.push(id.clone()),
            Fem3dOp::SetLoadCase { index, load_case } => diff.load_cases.set.push((*index, load_case.clone())),
            Fem3dOp::RemoveLoadCase { id } => diff.load_cases.removed.push(id.clone()),
            Fem3dOp::SetCombination { index, combination } => diff.combinations.set.push((*index, combination.clone())),
            Fem3dOp::RemoveCombination { id } => diff.combinations.removed.push(id.clone()),
            Fem3dOp::SetCamera { camera } => diff.camera = Some(camera.clone()),
            Fem3dOp::SetAnalysisSettings { settings } => diff.analysis = Some(settings.clone()),
        }
        diff
    }

    fn backwards(&self, projection: &Fem3dDocument) -> Vec<Self> {
        match self {
            Fem3dOp::SetNode { node, .. } => match node_index(projection, &node.id) {
                Some(index) => vec![Fem3dOp::SetNode { index, node: projection.nodes[index].clone() }],
                None => vec![Fem3dOp::RemoveNode { id: node.id.clone() }],
            },
            Fem3dOp::RemoveNode { id } => node_index(projection, id).map(|index| vec![Fem3dOp::SetNode { index, node: projection.nodes[index].clone() }]).unwrap_or_default(),
            Fem3dOp::SetElement { element, .. } => match element_index(projection, element_id(element)) {
                Some(index) => vec![Fem3dOp::SetElement { index, element: projection.elements[index].clone() }],
                None => vec![Fem3dOp::RemoveElement { id: element_id(element).to_string() }],
            },
            Fem3dOp::RemoveElement { id } => element_index(projection, id).map(|index| vec![Fem3dOp::SetElement { index, element: projection.elements[index].clone() }]).unwrap_or_default(),
            Fem3dOp::SetMaterial { material, .. } => match material_index(projection, &material.id) {
                Some(index) => vec![Fem3dOp::SetMaterial { index, material: projection.materials[index].clone() }],
                None => vec![Fem3dOp::RemoveMaterial { id: material.id.clone() }],
            },
            Fem3dOp::RemoveMaterial { id } => material_index(projection, id).map(|index| vec![Fem3dOp::SetMaterial { index, material: projection.materials[index].clone() }]).unwrap_or_default(),
            Fem3dOp::SetSection { section, .. } => match section_index(projection, &section.id) {
                Some(index) => vec![Fem3dOp::SetSection { index, section: projection.sections[index].clone() }],
                None => vec![Fem3dOp::RemoveSection { id: section.id.clone() }],
            },
            Fem3dOp::RemoveSection { id } => section_index(projection, id).map(|index| vec![Fem3dOp::SetSection { index, section: projection.sections[index].clone() }]).unwrap_or_default(),
            Fem3dOp::SetSupport { support, .. } => match support_index(projection, &support.id) {
                Some(index) => vec![Fem3dOp::SetSupport { index, support: projection.supports[index].clone() }],
                None => vec![Fem3dOp::RemoveSupport { id: support.id.clone() }],
            },
            Fem3dOp::RemoveSupport { id } => support_index(projection, id).map(|index| vec![Fem3dOp::SetSupport { index, support: projection.supports[index].clone() }]).unwrap_or_default(),
            Fem3dOp::SetLoadCase { load_case, .. } => match load_case_index(projection, &load_case.id) {
                Some(index) => vec![Fem3dOp::SetLoadCase { index, load_case: projection.load_cases[index].clone() }],
                None => vec![Fem3dOp::RemoveLoadCase { id: load_case.id.clone() }],
            },
            Fem3dOp::RemoveLoadCase { id } => load_case_index(projection, id).map(|index| vec![Fem3dOp::SetLoadCase { index, load_case: projection.load_cases[index].clone() }]).unwrap_or_default(),
            Fem3dOp::SetCombination { combination, .. } => match combination_index(projection, &combination.id) {
                Some(index) => vec![Fem3dOp::SetCombination { index, combination: projection.combinations[index].clone() }],
                None => vec![Fem3dOp::RemoveCombination { id: combination.id.clone() }],
            },
            Fem3dOp::RemoveCombination { id } => combination_index(projection, id).map(|index| vec![Fem3dOp::SetCombination { index, combination: projection.combinations[index].clone() }]).unwrap_or_default(),
            Fem3dOp::SetCamera { .. } => vec![Fem3dOp::SetCamera { camera: projection.camera.clone() }],
            Fem3dOp::SetAnalysisSettings { .. } => vec![Fem3dOp::SetAnalysisSettings { settings: projection.analysis.clone() }],
        }
    }
}
// #endregion 🔖Ops

pub type Fem3dEnvelope = DocumentVcsEnvelope<Fem3dDocument, Fem3dOp>;
pub type Fem3dStore = DocumentVcsStore<Fem3dDocument, Fem3dOp>;

pub fn empty_fem3d_projection() -> Fem3dDocument {
    Fem3dDocument::default()
}

// #region 🔖Bridge

// #region 🔖Errors
/// ⚠️ Everything that can go wrong resolving or solving a `Fem3dDocument`.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum Fem3dError {
    #[error("material not found: {0}")]
    MaterialNotFound(String),
    #[error("section not found: {0}")]
    SectionNotFound(String),
    #[error("node not found: {0}")]
    NodeNotFound(String),
    #[error("load case not found: {0}")]
    LoadCaseNotFound(String),
    #[error("mode index out of range: {0}")]
    ModeIndexOutOfRange(usize),
    #[error(transparent)]
    Fem(#[from] fem_core::FemError),
}
// #endregion 🔖Errors

/// 🧩 `resolve_geometry`'s resolved `(nodes, elements, supports)` triple.
type ResolvedGeometry = (Vec<Node>, Vec<Box<dyn Element>>, Vec<Support>);

/// 🌉 Resolves a `Fem3dDocument`'s nodes/elements/supports (materials/sections looked up by id) into
/// their `fem_core` equivalents — the geometry shared by both `build_model` (single frozen-signature
/// solve) and `fem3d_solve_all` (multi-case/combination solve).
fn resolve_geometry(doc: &Fem3dDocument) -> Result<ResolvedGeometry, Fem3dError> {
    let nodes: Vec<Node> = doc.nodes.iter().map(|node| Node { id: node.id.clone(), pos: [node.x, node.y, node.z] }).collect();
    let node_exists = |id: &str| doc.nodes.iter().any(|n| n.id == id);
    let mut elements: Vec<Box<dyn Element>> = Vec::with_capacity(doc.elements.len());
    for element in &doc.elements {
        match element {
            FemElement::Bar { id, start, end, material_id, section_id } => {
                let material = doc.materials.iter().find(|m| &m.id == material_id).ok_or_else(|| Fem3dError::MaterialNotFound(material_id.clone()))?;
                let section = doc.sections.iter().find(|s| &s.id == section_id).ok_or_else(|| Fem3dError::SectionNotFound(section_id.clone()))?;
                if !node_exists(start) {
                    return Err(Fem3dError::NodeNotFound(start.clone()));
                }
                if !node_exists(end) {
                    return Err(Fem3dError::NodeNotFound(end.clone()));
                }
                elements.push(Box::new(Bar3 { id: id.clone(), node_a: start.clone(), node_b: end.clone(), e: material.e, a: section.area, density: material.rho }));
            }
            FemElement::Frame { id, start, end, material_id, section_id, roll } => {
                let material = doc.materials.iter().find(|m| &m.id == material_id).ok_or_else(|| Fem3dError::MaterialNotFound(material_id.clone()))?;
                let section = doc.sections.iter().find(|s| &s.id == section_id).ok_or_else(|| Fem3dError::SectionNotFound(section_id.clone()))?;
                if !node_exists(start) {
                    return Err(Fem3dError::NodeNotFound(start.clone()));
                }
                if !node_exists(end) {
                    return Err(Fem3dError::NodeNotFound(end.clone()));
                }
                elements.push(Box::new(Frame3 { id: id.clone(), node_a: start.clone(), node_b: end.clone(), e: material.e, g: material.g, a: section.area, iy: section.iy, iz: section.iz, j: section.j, roll: *roll, density: material.rho }));
            }
        }
    }
    let supports = doc.supports.iter().map(|support| Support { node_id: support.node_id.clone(), fixed: support.fixed.clone() }).collect();
    Ok((nodes, elements, supports))
}

/// 🌉 Resolves a `Fem3dDocument` load case into a `fem_core::Model`: nodes, `Bar3`/`Frame3` elements
/// (materials/sections looked up by id), supports, and the named load case's nodal loads.
pub fn build_model(doc: &Fem3dDocument, case_id: &str) -> Result<Model, Fem3dError> {
    let (nodes, elements, supports) = resolve_geometry(doc)?;
    let mut model = Model { nodes, elements, supports, nodal_loads: Vec::new(), member_loads: Vec::new() };
    let case = doc.load_cases.iter().find(|c| c.id == case_id).ok_or_else(|| Fem3dError::LoadCaseNotFound(case_id.to_string()))?;
    for load in &case.loads {
        model.nodal_loads.push(NodalLoad { node_id: load.node_id.clone(), dof: load.dof, value: load.value });
    }
    Ok(model)
}

/// 🚀 Frozen entry point: builds the model for `case_id` and runs `fem_core::solve_linear_static`.
/// Consumed directly by `fem-plugin` — do not rename or change this signature.
pub fn fem3d_solve(doc: &Fem3dDocument, case_id: &str) -> Result<fem_core::StaticResult, String> {
    let model = build_model(doc, case_id).map_err(|e| e.to_string())?;
    fem_core::solve_linear_static(&model).map_err(|e| e.to_string())
}

/// 🌉 Builds an `AnalysisModel` plus one `analyses::LoadCase` per `doc.load_cases` entry and one
/// `analyses::Combination` per `doc.combinations` entry, solving them ALL at once via
/// `fem_core::analyses::solve_multi_case` (self-weight honored via `doc.materials`' `rho`, gravity
/// fixed at `[0.0, 0.0, -9.81]` — this crate is Z-up, per `FemNode`'s `{x,y,z}` fields and the existing
/// cantilever test's `Dof::Tz` tip load). Returns results keyed by case id ∪ combination id.
pub fn fem3d_solve_all(doc: &Fem3dDocument) -> Result<HashMap<String, fem_core::StaticResult>, Fem3dError> {
    let (nodes, elements, supports) = resolve_geometry(doc)?;
    let model = analyses::AnalysisModel { nodes, elements, supports };
    let cases: Vec<analyses::LoadCase> = doc
        .load_cases
        .iter()
        .map(|case| analyses::LoadCase {
            id: case.id.clone(),
            nodal_loads: case.loads.iter().map(|load| NodalLoad { node_id: load.node_id.clone(), dof: load.dof, value: load.value }).collect(),
            member_loads: Vec::new(),
            self_weight: case.self_weight,
        })
        .collect();
    let combinations: Vec<analyses::Combination> = doc.combinations.iter().map(|combination| analyses::Combination { id: combination.id.clone(), terms: combination.terms.clone() }).collect();
    analyses::solve_multi_case(&model, &cases, &combinations, [0.0, 0.0, -9.81]).map_err(Fem3dError::from)
}

// #region 🔖ModalBuckling
/// 🔢 Node-major, active-DOF-filtered ordering matching `fem_core::analyses::ModalResult`/
/// `BucklingResult`'s documented shape-vector layout — mirrors `fem_2d`'s identically named helper
/// (both are small local reimplementations of `analyses::build_dof_map`, which isn't `pub`).
fn mode_dof_order(nodes: &[Node], elements: &[Box<dyn Element>]) -> Vec<(String, Dof)> {
    let mut order = Vec::new();
    for node in nodes {
        let mut active: Vec<Dof> = Vec::new();
        for element in elements {
            if element.node_ids().iter().any(|id| id == &node.id) {
                for &dof in element.dofs_per_node() {
                    if !active.contains(&dof) {
                        active.push(dof);
                    }
                }
            }
        }
        active.sort_by_key(|d| d.index());
        for dof in active {
            order.push((node.id.clone(), dof));
        }
    }
    order
}

/// 🎵 Modal analysis: lowest `doc.analysis.modal_count` natural frequencies/mode shapes.
pub fn fem3d_modal(doc: &Fem3dDocument) -> Result<analyses::ModalResult, Fem3dError> {
    let (nodes, elements, supports) = resolve_geometry(doc)?;
    let model = analyses::AnalysisModel { nodes, elements, supports };
    analyses::modal(&model, doc.analysis.modal_count).map_err(Fem3dError::from)
}

/// 🌉 Richer modal entry point: solves the same modal analysis as `fem3d_modal` but also unpacks mode
/// `mode_index`'s shape into a per-node `[f64;6]` displacement map. Returns
/// `(frequency_hz, node_id -> displacement values)`.
pub fn fem3d_modal_mode_values(doc: &Fem3dDocument, mode_index: usize) -> Result<(f64, HashMap<String, [f64; 6]>), Fem3dError> {
    let (nodes, elements, supports) = resolve_geometry(doc)?;
    let order = mode_dof_order(&nodes, &elements);
    let model = analyses::AnalysisModel { nodes, elements, supports };
    let result = analyses::modal(&model, doc.analysis.modal_count)?;
    let freq = *result.frequencies_hz.get(mode_index).ok_or(Fem3dError::ModeIndexOutOfRange(mode_index))?;
    let shape = result.shapes.get(mode_index).ok_or(Fem3dError::ModeIndexOutOfRange(mode_index))?;
    let mut values: HashMap<String, [f64; 6]> = HashMap::new();
    for (i, (node_id, dof)) in order.iter().enumerate() {
        values.entry(node_id.clone()).or_insert([0.0; 6])[dof.index()] = shape.get(i);
    }
    Ok((freq, values))
}

/// 🌉 Shared buckling-case resolution for `fem3d_buckling`/`fem3d_buckling_mode_values`.
fn buckling_case(doc: &Fem3dDocument, case_id: &str) -> Result<analyses::LoadCase, Fem3dError> {
    let case = doc.load_cases.iter().find(|c| c.id == case_id).ok_or_else(|| Fem3dError::LoadCaseNotFound(case_id.to_string()))?;
    Ok(analyses::LoadCase { id: case.id.clone(), nodal_loads: case.loads.iter().map(|load| NodalLoad { node_id: load.node_id.clone(), dof: load.dof, value: load.value }).collect(), member_loads: Vec::new(), self_weight: case.self_weight })
}

/// 🏛️ Linear buckling: lowest `doc.analysis.buckling_count` load factors/mode shapes for `case_id`.
pub fn fem3d_buckling(doc: &Fem3dDocument, case_id: &str) -> Result<analyses::BucklingResult, Fem3dError> {
    let (nodes, elements, supports) = resolve_geometry(doc)?;
    let model = analyses::AnalysisModel { nodes, elements, supports };
    let case = buckling_case(doc, case_id)?;
    analyses::buckling(&model, &case, doc.analysis.buckling_count).map_err(Fem3dError::from)
}

/// 🌉 Richer buckling entry point: mirrors `fem3d_modal_mode_values` — solves the same buckling
/// analysis as `fem3d_buckling` but also unpacks mode `mode_index`'s shape into a per-node
/// displacement map. Returns `(load_factor, node_id -> displacement values)`.
pub fn fem3d_buckling_mode_values(doc: &Fem3dDocument, case_id: &str, mode_index: usize) -> Result<(f64, HashMap<String, [f64; 6]>), Fem3dError> {
    let (nodes, elements, supports) = resolve_geometry(doc)?;
    let order = mode_dof_order(&nodes, &elements);
    let case = buckling_case(doc, case_id)?;
    let model = analyses::AnalysisModel { nodes, elements, supports };
    let result = analyses::buckling(&model, &case, doc.analysis.buckling_count)?;
    let factor = *result.factors.get(mode_index).ok_or(Fem3dError::ModeIndexOutOfRange(mode_index))?;
    let shape = result.shapes.get(mode_index).ok_or(Fem3dError::ModeIndexOutOfRange(mode_index))?;
    let mut values: HashMap<String, [f64; 6]> = HashMap::new();
    for (i, (node_id, dof)) in order.iter().enumerate() {
        values.entry(node_id.clone()).or_insert([0.0; 6])[dof.index()] = shape.get(i);
    }
    Ok((factor, values))
}
// #endregion 🔖ModalBuckling
// #endregion 🔖Bridge

// #region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct Fem3dDocumentVcs {
        store: RefCell<Fem3dStore>,
    }

    #[wasm_bindgen]
    impl Fem3dDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<Fem3dDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Fem3dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Fem3dStore::new(envelope)
                }
                None => Fem3dStore::new(create_document_vcs_envelope(FEM_3D_SCHEMA, "fem3d", empty_fem3d_projection(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_json(command_json).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store.borrow().envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
}
// #endregion 🔖WasmBridge

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use vcs::apply_operation;

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
            materials: vec![FemMaterial { id: "steel".into(), name: "Steel".into(), e, g, rho: 7850.0 }],
            sections: vec![FemSection { id: "hea200".into(), name: "HEA200".into(), area: a, iy, iz, j }],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: Dof::ALL.to_vec() }],
            load_cases: vec![FemLoadCase { id: "point".into(), name: "Point Load".into(), loads: vec![FemNodalLoad { id: "l1".into(), node_id: "n2".into(), dof: Dof::Tz, value: -p }], self_weight: false }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
            camera: FemCamera::default(),
        };
        (doc, e, iy, l, p, iz)
    }

    /// 🔺 A free 3D joint needs at least 3 non-coplanar bars to be kinematically determinate — two
    /// bars only span a plane, leaving one direction with zero stiffness (a mechanism). Hence n4/b3.
    fn truss_fixture() -> Fem3dDocument {
        Fem3dDocument {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNode { id: "n2".into(), x: 2.0, y: 0.0, z: 0.0 }, FemNode { id: "n3".into(), x: 1.0, y: 1.0, z: 2.0 }, FemNode { id: "n4".into(), x: 1.0, y: -1.0, z: 0.0 }],
            elements: vec![
                FemElement::Bar { id: "b1".into(), start: "n1".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
                FemElement::Bar { id: "b2".into(), start: "n2".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
                FemElement::Bar { id: "b3".into(), start: "n4".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
            ],
            materials: vec![FemMaterial { id: "steel".into(), name: "Steel".into(), e: 210e9, g: 80.77e9, rho: 7850.0 }],
            sections: vec![FemSection { id: "rod".into(), name: "Rod".into(), area: 0.001, iy: 1e-6, iz: 1e-6, j: 1e-6 }],
            supports: vec![
                FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: Dof::ALL.to_vec() },
                FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: Dof::ALL.to_vec() },
                FemSupport { id: "s3".into(), node_id: "n4".into(), fixed: Dof::ALL.to_vec() },
            ],
            load_cases: vec![FemLoadCase { id: "drop".into(), name: "Drop".into(), loads: vec![FemNodalLoad { id: "l1".into(), node_id: "n3".into(), dof: Dof::Tz, value: -1000.0 }], self_weight: false }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
            camera: FemCamera::default(),
        }
    }
    // #endregion 🔖Fixtures

    // #region 🔖OpRoundTrip
    fn round_trip(projection: &Fem3dDocument, op: &Fem3dOp) -> Fem3dDocument {
        let forward = apply_operation(projection, op);
        let mut restored = forward.clone();
        for back in op.backwards(projection) {
            restored = apply_operation(&restored, &back);
        }
        assert_eq!(&restored, projection, "backwards() must restore the pre-op document");
        forward
    }

    #[test]
    fn node_set_and_remove_round_trip() {
        let base = empty_fem3d_projection();
        let node = FemNode { id: "n1".into(), x: 1.0, y: 2.0, z: 3.0 };
        let after_set = round_trip(&base, &Fem3dOp::SetNode { index: 0, node: node.clone() });
        assert_eq!(after_set.nodes, vec![node.clone()]);
        round_trip(&after_set, &Fem3dOp::RemoveNode { id: node.id });
    }

    #[test]
    fn element_set_and_remove_round_trip() {
        let (base, ..) = cantilever_fixture();
        let updated = FemElement::Frame { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.5 };
        let after_set = round_trip(&base, &Fem3dOp::SetElement { index: 0, element: updated });
        round_trip(&after_set, &Fem3dOp::RemoveElement { id: "e1".into() });
    }

    #[test]
    fn material_set_and_remove_round_trip() {
        let (base, ..) = cantilever_fixture();
        let material = FemMaterial { id: "steel".into(), name: "Steel Updated".into(), e: 200e9, g: 79e9, rho: 7900.0 };
        let after_set = round_trip(&base, &Fem3dOp::SetMaterial { index: 0, material });
        round_trip(&after_set, &Fem3dOp::RemoveMaterial { id: "steel".into() });
    }

    #[test]
    fn section_set_and_remove_round_trip() {
        let (base, ..) = cantilever_fixture();
        let section = FemSection { id: "hea200".into(), name: "HEA200 Updated".into(), area: 0.006, iy: 4e-5, iz: 1.5e-5, j: 7e-7 };
        let after_set = round_trip(&base, &Fem3dOp::SetSection { index: 0, section });
        round_trip(&after_set, &Fem3dOp::RemoveSection { id: "hea200".into() });
    }

    #[test]
    fn support_set_and_remove_round_trip() {
        let (base, ..) = cantilever_fixture();
        let support = FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz] };
        let after_set = round_trip(&base, &Fem3dOp::SetSupport { index: 0, support });
        round_trip(&after_set, &Fem3dOp::RemoveSupport { id: "s1".into() });
    }

    #[test]
    fn load_case_set_and_remove_round_trip() {
        let (base, ..) = cantilever_fixture();
        let load_case = FemLoadCase { id: "point".into(), name: "Point Load Updated".into(), loads: vec![FemNodalLoad { id: "l1".into(), node_id: "n2".into(), dof: Dof::Tz, value: -9000.0 }], self_weight: false };
        let after_set = round_trip(&base, &Fem3dOp::SetLoadCase { index: 0, load_case });
        round_trip(&after_set, &Fem3dOp::RemoveLoadCase { id: "point".into() });
    }

    #[test]
    fn combination_set_and_remove_round_trip() {
        let (base, ..) = cantilever_fixture();
        let combination = FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![("point".into(), 1.35)] };
        let after_set = round_trip(&base, &Fem3dOp::SetCombination { index: 0, combination });
        round_trip(&after_set, &Fem3dOp::RemoveCombination { id: "uls".into() });
    }

    #[test]
    fn camera_set_round_trips() {
        let base = empty_fem3d_projection();
        round_trip(&base, &Fem3dOp::SetCamera { camera: FemCamera { json: "{\"zoom\":2}".into() } });
    }

    #[test]
    fn analysis_settings_set_round_trips() {
        let base = empty_fem3d_projection();
        let settings = FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 25.0 };
        round_trip(&base, &Fem3dOp::SetAnalysisSettings { settings });
    }
    // #endregion 🔖OpRoundTrip

    // #region 🔖BuildModel
    #[test]
    fn build_model_rejects_dangling_material() {
        let (mut doc, ..) = cantilever_fixture();
        if let FemElement::Frame { material_id, .. } = &mut doc.elements[0] {
            *material_id = "missing".into();
        }
        let err = build_model(&doc, "point").unwrap_err();
        assert!(err.to_string().contains("missing"), "error should name the dangling id: {err}");
    }

    #[test]
    fn build_model_rejects_dangling_section() {
        let (mut doc, ..) = cantilever_fixture();
        if let FemElement::Frame { section_id, .. } = &mut doc.elements[0] {
            *section_id = "missing".into();
        }
        let err = build_model(&doc, "point").unwrap_err();
        assert!(err.to_string().contains("missing"), "error should name the dangling id: {err}");
    }

    #[test]
    fn build_model_rejects_dangling_node() {
        let (mut doc, ..) = cantilever_fixture();
        if let FemElement::Frame { end, .. } = &mut doc.elements[0] {
            *end = "missing".into();
        }
        let err = build_model(&doc, "point").unwrap_err();
        assert!(err.to_string().contains("missing"), "error should name the dangling id: {err}");
    }
    // #endregion 🔖BuildModel

    // #region 🔖CantileverBenchmark
    #[test]
    fn cantilever_tip_load_matches_analytical_solution() {
        let (doc, e, iy, l, p, _iz) = cantilever_fixture();
        let result = fem3d_solve(&doc, "point").expect("solves");

        let expected_deflection = p * l.powi(3) / (3.0 * e * iy);
        let expected_rotation = p * l.powi(2) / (2.0 * e * iy);
        let expected_base_moment = p * l;

        let n2 = result.displacements.iter().find(|d| d.node_id == "n2").unwrap();
        let deflection = n2.values[Dof::Tz.index()].abs();
        let rotation = n2.values[Dof::Ry.index()].abs();
        assert!((deflection - expected_deflection).abs() / expected_deflection < 0.01, "deflection {deflection} vs {expected_deflection}");
        assert!((rotation - expected_rotation).abs() / expected_rotation < 0.01, "rotation {rotation} vs {expected_rotation}");

        let reaction_tz = result.reactions.iter().find(|r| r.node_id == "n1" && r.dof == Dof::Tz).unwrap();
        assert!((reaction_tz.value - p).abs() < p * 0.01 || (reaction_tz.value + p).abs() < p * 0.01, "reaction {}", reaction_tz.value);
        assert!((reaction_tz.value + (-p)).abs() < p * 0.01, "reaction + applied load should be ~0: {}", reaction_tz.value);

        let reaction_ry = result.reactions.iter().find(|r| r.node_id == "n1" && r.dof == Dof::Ry).unwrap();
        assert!((reaction_ry.value.abs() - expected_base_moment).abs() / expected_base_moment < 0.01, "base moment {} vs {}", reaction_ry.value, expected_base_moment);

        let (_, element_result) = result.elements.iter().find(|(id, _)| id == "e1").unwrap();
        match element_result {
            ElementResult::Beam { stations } => {
                let base = stations.first().unwrap();
                let tip = stations.last().unwrap();
                let base_tol = (expected_base_moment * 0.01).max(1.0);
                assert!((base.m.abs() - expected_base_moment).abs() < base_tol, "base moment {} vs {}", base.m, expected_base_moment);
                assert!(tip.m.abs() < base_tol, "tip moment should be ~0: {}", tip.m);
            }
            _ => panic!("expected beam result"),
        }
    }

    #[test]
    fn truss_3d_solve_is_finite_and_balanced() {
        let doc = truss_fixture();
        let result = fem3d_solve(&doc, "drop").expect("solves");
        for &v in &result.checks.reaction_sum {
            assert!(v.abs() < 1e-6, "reaction_sum should balance the applied load: {:?}", result.checks.reaction_sum);
        }
        for (_, element_result) in &result.elements {
            match element_result {
                ElementResult::Bar { n } => {
                    assert!(n.is_finite());
                    assert!(n.abs() > 1e-6, "bar force should be nonzero under load");
                }
                _ => panic!("expected bar result"),
            }
        }
    }

    #[test]
    fn fem3d_solve_unknown_case_id_errors() {
        let (doc, ..) = cantilever_fixture();
        let err = fem3d_solve(&doc, "missing-case").unwrap_err();
        assert!(err.contains("load case not found"), "error was: {err}");
    }
    // #endregion 🔖CantileverBenchmark

    // #region 🔖SolveAll
    #[test]
    fn fem3d_solve_all_returns_case_and_combination_results() {
        let (mut doc, ..) = cantilever_fixture();
        doc.load_cases.push(FemLoadCase { id: "point2".into(), name: "Point Load 2".into(), loads: vec![FemNodalLoad { id: "l2".into(), node_id: "n2".into(), dof: Dof::Tz, value: -2000.0 }], self_weight: false });
        doc.combinations = vec![FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![("point".into(), 1.35), ("point2".into(), 1.0)] }];

        let results = fem3d_solve_all(&doc).expect("solves");
        let mut keys: Vec<&String> = results.keys().collect();
        keys.sort();
        assert_eq!(keys, vec!["point", "point2", "uls"], "expected exactly the case and combination ids");

        let point = results.get("point").unwrap();
        let point2 = results.get("point2").unwrap();
        let uls = results.get("uls").unwrap();
        for pd in &point.displacements {
            let p2d = point2.displacements.iter().find(|d| d.node_id == pd.node_id).unwrap();
            let ud = uls.displacements.iter().find(|d| d.node_id == pd.node_id).unwrap();
            for k in 0..6 {
                let expected = 1.35 * pd.values[k] + 1.0 * p2d.values[k];
                assert!((ud.values[k] - expected).abs() < 1e-8, "combo displacement mismatch at {} dof {k}: {} vs {}", pd.node_id, ud.values[k], expected);
            }
        }
    }

    #[test]
    fn self_weight_case_produces_nonzero_reactions() {
        let (mut doc, _e, _iy, l, _p, _iz) = cantilever_fixture();
        let (area, rho) = (doc.sections[0].area, doc.materials[0].rho);
        doc.load_cases = vec![FemLoadCase { id: "self".into(), name: "Self Weight".into(), loads: vec![], self_weight: true }];

        let results = fem3d_solve_all(&doc).expect("solves");
        let result = results.get("self").unwrap();

        let total_tz_reaction: f64 = result.reactions.iter().filter(|r| r.dof == Dof::Tz).map(|r| r.value).sum();
        let expected = rho * area * l * 9.81;
        assert!(total_tz_reaction.abs() > 1e-6, "self-weight reaction should be nonzero");
        assert!((total_tz_reaction - expected).abs() / expected < 0.02, "reaction sum {total_tz_reaction} vs expected {expected}");
    }
    // #endregion 🔖SolveAll

    // #region 🔖ModalBuckling
    #[test]
    fn fem3d_modal_returns_requested_mode_count() {
        let (doc, ..) = cantilever_fixture();
        let result = fem3d_modal(&doc).expect("modal solves");
        assert_eq!(result.frequencies_hz.len(), doc.analysis.modal_count);
        for w in result.frequencies_hz.windows(2) {
            assert!(w[0] <= w[1], "frequencies should be ascending: {:?}", result.frequencies_hz);
        }
        for &f in &result.frequencies_hz {
            assert!(f.is_finite() && f >= 0.0, "frequency should be finite and non-negative: {f}");
        }
    }

    #[test]
    fn fem3d_modal_mode_values_returns_node_displacements() {
        let (doc, ..) = cantilever_fixture();
        let (freq, values) = fem3d_modal_mode_values(&doc, 0).expect("modal mode values solves");
        assert!(freq.is_finite() && freq >= 0.0);
        assert!(values.contains_key("n1"));
        assert!(values.contains_key("n2"));
    }

    #[test]
    fn fem3d_buckling_returns_requested_mode_count() {
        let (doc, ..) = cantilever_fixture();
        let result = fem3d_buckling(&doc, "point").expect("buckling solves");
        assert_eq!(result.factors.len(), doc.analysis.buckling_count);
        for &f in &result.factors {
            assert!(f.is_finite(), "buckling factor should be finite: {f}");
        }
    }

    #[test]
    fn fem3d_buckling_mode_values_returns_node_displacements() {
        let (doc, ..) = cantilever_fixture();
        let (factor, values) = fem3d_buckling_mode_values(&doc, "point", 0).expect("buckling mode values solves");
        assert!(factor.is_finite());
        assert!(values.contains_key("n1"));
        assert!(values.contains_key("n2"));
    }

    #[test]
    fn fem3d_buckling_unknown_case_errors() {
        let (doc, ..) = cantilever_fixture();
        let err = fem3d_buckling(&doc, "missing").err().expect("expected error");
        assert!(err.to_string().contains("load case not found"), "unexpected error: {err}");
    }
    // #endregion 🔖ModalBuckling

    #[test]
    fn example_fixture_parses() {
        let json = include_str!("../example/default.fem3d.json");
        let doc: Fem3dDocument = serde_json::from_str(json).expect("example fixture parses");
        assert_eq!(doc.nodes.len(), 2);
        assert_eq!(doc.elements.len(), 1);
        let result = fem3d_solve(&doc, "point").expect("example fixture solves");
        assert!(result.checks.residual_norm < 1e-6);

        let all_results = fem3d_solve_all(&doc).expect("example fixture solves all");
        assert!(all_results.contains_key("point"), "expected point case result");
        assert!(all_results.contains_key("point2"), "expected point2 case result");
        assert!(all_results.contains_key("uls"), "expected uls combination result");
    }
}
// #endregion 🔖Tests
