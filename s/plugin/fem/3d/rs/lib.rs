//! 🏙️ FEM 3D document model and element library on `vcs`.

#[cfg(test)]
use fem_core::ElementResult;
use fem_core::{analyses, Bar3, Dof, Element, Frame3, MemberUdl, Model, NodalLoad, Node, Support};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
#[cfg(target_arch = "wasm32")]
use store::create_document_envelope;
use protocol::{Operation, OperationDiff};
use store::{DocumentEnvelope, DocumentStore};

pub const FEM_3D_SCHEMA: &str = "fem.3d";

// #region 🔖Document
/// 🔁 fem_3d's own DSL-printable mirror of `fem_core::Dof` — `fem_core::Dof` can't derive
/// `dsl::DslScalar` from outside its own crate (the orphan rule blocks implementing a foreign
/// `dsl::DslField` for a foreign `fem_core::Dof`), so every DOF-typed field in the `Fem3dDocument`
/// grammar (`FemSupport::fixed`, `FemLoad::Nodal::dof`) uses this local tag instead, converting
/// to/from `fem_core::Dof` at the `fem_core::Model`/`Support`/`NodalLoad` boundary (see
/// `resolve_geometry`, `translate_loads`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
pub enum FemDof {
    #[dsl(key = "Tx")]
    Tx,
    #[dsl(key = "Ty")]
    Ty,
    #[dsl(key = "Tz")]
    Tz,
    #[dsl(key = "Rx")]
    Rx,
    #[dsl(key = "Ry")]
    Ry,
    #[dsl(key = "Rz")]
    Rz,
}

impl FemDof {
    pub const ALL: [FemDof; 6] = [FemDof::Tx, FemDof::Ty, FemDof::Tz, FemDof::Rx, FemDof::Ry, FemDof::Rz];
}

impl From<Dof> for FemDof {
    fn from(dof: Dof) -> Self {
        match dof {
            Dof::Tx => FemDof::Tx,
            Dof::Ty => FemDof::Ty,
            Dof::Tz => FemDof::Tz,
            Dof::Rx => FemDof::Rx,
            Dof::Ry => FemDof::Ry,
            Dof::Rz => FemDof::Rz,
        }
    }
}

impl From<FemDof> for Dof {
    fn from(dof: FemDof) -> Self {
        match dof {
            FemDof::Tx => Dof::Tx,
            FemDof::Ty => Dof::Ty,
            FemDof::Tz => Dof::Tz,
            FemDof::Rx => Dof::Rx,
            FemDof::Ry => Dof::Ry,
            FemDof::Rz => Dof::Rz,
        }
    }
}

/// 📍 A structural node: a stable id and a global position, plain SI meters.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "node")]
pub struct FemNode {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 🔩 A two-node member: an axial `Bar` or a full 6-DOF `Frame` with a local-axis `roll` angle (radians).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FemElement {
    #[serde(rename_all = "camelCase")]
    Bar { id: String, start: String, end: String, material_id: String, section_id: String },
    #[serde(rename_all = "camelCase")]
    Frame { id: String, start: String, end: String, material_id: String, section_id: String, roll: f64 },
}

/// 🧱 Linear-elastic isotropic material: Young's modulus `e`, shear modulus `g` (Pa), Poisson's ratio
/// `nu` (dimensionless, drives `Tet4` solid elements), and density `rho` (kg/m³, drives self-weight via
/// `Bar3`/`Frame3`/`Tet4`'s `mass()`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "material")]
pub struct FemMaterial {
    pub id: String,
    pub name: String,
    pub e: f64,
    pub g: f64,
    pub nu: f64,
    pub rho: f64,
}

/// 📐 Cross-section properties: area (m²), second moments of area about local y/z (m⁴), torsion constant (m⁴).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "section")]
pub struct FemSection {
    pub id: String,
    pub name: String,
    pub area: f64,
    pub iy: f64,
    pub iz: f64,
    pub j: f64,
}

/// 🔒 A support: the subset of a node's DOFs restrained to zero displacement.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "support")]
pub struct FemSupport {
    pub id: String,
    pub node_id: String,
    pub fixed: Vec<FemDof>,
}

/// 🏋️ A load — a concentrated nodal force/moment, a member UDL on a `Bar`/`Frame` element, or a normal
/// pressure (Pa) over a meshed `FemSolid`'s top face, simplified as a uniform global `-Z` nodal load
/// (see `area_load_nodal_loads_3d`) — mirrors `fem_2d::FemLoad`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FemLoad {
    #[serde(rename_all = "camelCase")]
    Nodal { id: String, node_id: String, dof: FemDof, value: f64 },
    #[serde(rename_all = "camelCase")]
    MemberUdl { id: String, element_id: String, wx: f64, wy: f64, wz: f64 },
    #[serde(rename_all = "camelCase")]
    Area { id: String, solid_id: String, pressure: f64 },
}

/// 🪪 A `FemLoad`'s stable id, across every variant.
pub fn load_id(load: &FemLoad) -> &str {
    match load {
        FemLoad::Nodal { id, .. } | FemLoad::MemberUdl { id, .. } | FemLoad::Area { id, .. } => id,
    }
}

/// 📦 A named set of loads applied together for one analysis run, optionally including self-weight.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "loadcase")]
pub struct FemLoadCase {
    pub id: String,
    pub name: String,
    #[dsl(statements, block)]
    pub loads: Vec<FemLoad>,
    pub self_weight: bool,
}

/// 📦 A linear combination of load cases — case id → factor terms superposed from already-solved
/// case results. `BTreeMap` (not `Vec<(String, f64)>`, which the DSL engine has no primitive for)
/// keyed by case id — duplicates collapse to the last value, which never happened in practice anyway.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "combination")]
pub struct FemCombination {
    pub id: String,
    pub name: String,
    pub terms: BTreeMap<String, f64>,
}

/// ⚙️ Analysis settings: mode/factor counts for modal and buckling analyses, plus a deformation
/// display scale for the UI layer. `deformation_scale` exaggerates the STATIC results view's real
/// (meter-scale) displacements only; modal/buckling mode shapes are dimensionless (mass/Kg-
/// orthonormalized) and the viewer normalizes them to a fixed fraction of the model's own extent
/// instead of using this factor.
// No `#[dsl(keyword = ...)]` here: the only field embedding this type (`Fem3dDocument::analysis`,
// `Fem3dOperation::SetAnalysisSettings::settings`) is itself `#[dsl(block)]`, which already supplies
// the bare leading keyword from the FIELD's own name — an inner keyword too would double it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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

/// 🧱 A meshed continuum solid — a polygon footprint (with optional holes) extruded upward from
/// `base_z` by `height` across `layers` equal-height layers, filled with `Tet4` elements at solve time
/// (see `resolve_geometry`) — mirrors `fem_2d::FemRegion`, extended into 3D via `fem_core::mesh`'s
/// extrusion + tet-splitting.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "solid")]
pub struct FemSolid {
    pub id: String,
    pub name: String,
    pub outline: Vec<[f64; 2]>,
    pub holes: Vec<Vec<[f64; 2]>>,
    pub base_z: f64,
    pub height: f64,
    pub layers: usize,
    pub mesh_size: f64,
    pub material_id: String,
}

/// 🎥 Opaque camera state string; the plugin layer owns and interprets its shape. No
/// `#[dsl(keyword = ...)]`: every field embedding this type is itself `#[dsl(block)]` (see
/// `FemAnalysisSettings`'s doc comment above for why that means the keyword stays off here).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "fem3d", layout = "lines")]
pub struct Fem3dDocument {
    #[dsl(table)]
    pub nodes: Vec<FemNode>,
    #[dsl(statements, block)]
    pub elements: Vec<FemElement>,
    #[dsl(table)]
    pub materials: Vec<FemMaterial>,
    #[dsl(table)]
    pub sections: Vec<FemSection>,
    #[dsl(table)]
    pub solids: Vec<FemSolid>,
    #[dsl(table)]
    pub supports: Vec<FemSupport>,
    #[dsl(table)]
    pub load_cases: Vec<FemLoadCase>,
    #[dsl(table)]
    pub combinations: Vec<FemCombination>,
    #[dsl(block)]
    pub analysis: FemAnalysisSettings,
    #[dsl(block)]
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
/// 🩹 Sparse fem-3d diff over every document collection plus the scalar camera field.
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
    pub camera: Option<FemCamera>,
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
        if let Some(camera) = &self.camera {
            next.camera = camera.clone();
        }
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
    SetCamera {
        #[dsl(block)]
        camera: FemCamera,
    },
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
            Fem3dOperation::SetCamera { camera } => diff.camera = Some(camera.clone()),
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
            Fem3dOperation::SetCamera { .. } => vec![Fem3dOperation::SetCamera { camera: projection.camera.clone() }],
            Fem3dOperation::SetAnalysisSettings { .. } => vec![Fem3dOperation::SetAnalysisSettings { settings: projection.analysis.clone() }],
            Fem3dOperation::SetDocument { .. } => vec![Fem3dOperation::SetDocument { document: projection.clone() }],
        }
    }
}
// #endregion 🔖Operations

pub type Fem3dEnvelope = DocumentEnvelope<Fem3dDocument, Fem3dOperation>;
pub type Fem3dStore = DocumentStore<Fem3dDocument, Fem3dOperation>;

pub fn empty_fem3d_projection() -> Fem3dDocument {
    Fem3dDocument::default()
}

// #region 🔖Dsl
// `Fem3dDocument`'s `store::DocumentDsl` and `Fem3dOperation`'s `store::OpText` are generated by
// `#[derive(dsl::DslDocument)]`/`#[derive(dsl::DslOps)]` on the type definitions in
// `🔖Document`/`🔖Operations` above — the engine's `dsl_schema` grammar replaces this crate's own
// hand-rolled lexer/parser/printer.
// #endregion 🔖Dsl

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
    #[error("unknown solid id: {0}")]
    UnknownSolidId(String),
    #[error("solid {solid_id} failed to mesh: {reason}")]
    MeshFailed { solid_id: String, reason: String },
    #[error("load case not found: {0}")]
    LoadCaseNotFound(String),
    #[error("mode index out of range: {0}")]
    ModeIndexOutOfRange(usize),
    #[error(transparent)]
    Fem(#[from] fem_core::FemError),
}
// #endregion 🔖Errors

// #region 🔖SolidMeshing
/// 📐 Unsigned area of triangle `(p0, p1, p2)` via the shoelace formula — mirrors `fem_2d`'s helper of
/// the same purpose.
fn triangle_area_2d(p0: [f64; 2], p1: [f64; 2], p2: [f64; 2]) -> f64 {
    (0.5 * ((p1[0] - p0[0]) * (p2[1] - p0[1]) - (p2[0] - p0[0]) * (p1[1] - p0[1]))).abs()
}

/// 🌐 One meshed `FemSolid`'s resolved geometry, reused by `resolve_geometry`'s caller for area-load
/// tributary-area computation. `node_ids`/`points` cover the FULL volume mesh (every layer); the top
/// surface (needed for `FemLoad::Area`) is exactly `top_footprint_tris` over `top_footprint_points`
/// (the ORIGINAL flat triangulation, since `extrude_tri_mesh` numbers each layer's points in that same
/// order — see its doc — so the top layer's node ids are `node_ids[top_offset + footprint_point_index]`).
struct MeshedSolid {
    solid_id: String,
    node_ids: Vec<String>,
    top_offset: usize,
    top_footprint_points: Vec<[f64; 2]>,
    top_footprint_tris: Vec<[u32; 3]>,
}

/// 🧩 `resolve_geometry`'s resolved `(nodes, elements, meshed solids, supports)` quadruple.
type ResolvedGeometry = (Vec<Node>, Vec<Box<dyn Element>>, Vec<MeshedSolid>, Vec<Support>);

/// 🌉 Resolves a `Fem3dDocument`'s nodes/elements/supports (materials/sections looked up by id) plus
/// every `FemSolid` meshed into `Tet4` elements (footprint triangulated via `fem_core::mesh::triangulate`,
/// extruded via `extrude_tri_mesh`, split via `split_to_tets` — mirrors `fem_2d::build_nodes_and_elements`'s
/// `Tri3Cst` region meshing) — the geometry shared by `build_model`, `fem3d_solve_all`, modal, and
/// buckling. A solid boundary point coinciding (within `1e-9`, all of x/y/z) with an existing document
/// node reuses that node's id; otherwise a node is synthesized once per unique mesh point as
/// `{solid_id}_m{point_index}`.
fn resolve_geometry(doc: &Fem3dDocument) -> Result<ResolvedGeometry, Fem3dError> {
    let mut nodes: Vec<Node> = doc.nodes.iter().map(|node| Node { id: node.id.clone(), pos: [node.x, node.y, node.z] }).collect();
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

    let mut meshed_solids = Vec::with_capacity(doc.solids.len());
    for solid in &doc.solids {
        let material = doc.materials.iter().find(|m| m.id == solid.material_id).ok_or_else(|| Fem3dError::MaterialNotFound(solid.material_id.clone()))?;
        let domain = fem_core::mesh::PlanarDomain { outer: solid.outline.clone(), holes: solid.holes.clone() };
        let opts = fem_core::mesh::MeshOpts { max_edge: solid.mesh_size, min_angle_deg: 20.0 };
        let tri_mesh = fem_core::mesh::triangulate(&domain, &opts).map_err(|e| Fem3dError::MeshFailed { solid_id: solid.id.clone(), reason: e.to_string() })?;
        let layers = solid.layers.max(1);
        let volume_mesh = fem_core::mesh::extrude_tri_mesh(&tri_mesh, solid.height, layers);
        let tet_mesh = fem_core::mesh::split_to_tets(&volume_mesh);
        let points: Vec<[f64; 3]> = tet_mesh.points.iter().map(|p| [p[0], p[1], p[2] + solid.base_z]).collect();

        let mut node_ids = Vec::with_capacity(points.len());
        for (point_index, p) in points.iter().enumerate() {
            let id = match doc.nodes.iter().find(|n| (n.x - p[0]).abs() < 1e-9 && (n.y - p[1]).abs() < 1e-9 && (n.z - p[2]).abs() < 1e-9) {
                Some(n) => n.id.clone(),
                None => {
                    let synthetic_id = format!("{}_m{}", solid.id, point_index);
                    nodes.push(Node { id: synthetic_id.clone(), pos: *p });
                    synthetic_id
                }
            };
            node_ids.push(id);
        }

        for (cell_index, cell) in tet_mesh.cells.iter().enumerate() {
            let fem_core::mesh::Cell::Tet4(t) = cell else { continue };
            let tet_nodes = [node_ids[t[0] as usize].clone(), node_ids[t[1] as usize].clone(), node_ids[t[2] as usize].clone(), node_ids[t[3] as usize].clone()];
            elements.push(Box::new(fem_core::elements3d::Tet4 { id: format!("{}_c{}", solid.id, cell_index), nodes: tet_nodes, e: material.e, nu: material.nu, density: material.rho }));
        }

        // The LAST extrusion layer's points are the top surface — `extrude_tri_mesh` numbers points
        // layer-by-layer in `tri_mesh.points`' own order (see its doc), so index `top_offset + i` is
        // footprint point `i`'s top-layer node, and `tri_mesh.tris` is directly the top surface's
        // triangulation (that layer's wedges pass their top cap through `split_to_tets` unsplit).
        let top_offset = layers * tri_mesh.points.len();
        meshed_solids.push(MeshedSolid { solid_id: solid.id.clone(), node_ids, top_offset, top_footprint_points: tri_mesh.points, top_footprint_tris: tri_mesh.tris });
    }

    let supports = doc.supports.iter().map(|support| Support { node_id: support.node_id.clone(), fixed: support.fixed.iter().map(|dof| Dof::from(*dof)).collect() }).collect();
    Ok((nodes, elements, meshed_solids, supports))
}

/// 🌬️ Converts a `FemLoad::Area` (uniform pressure, Pa) into per-node global `-Z` nodal loads on a
/// solid's TOP surface — `pressure * tributaryArea` at each top node, tributary area `(1/3)` of the
/// summed area of every top-surface triangle touching that node. Mirrors `fem_2d::area_load_nodal_loads`.
fn area_load_nodal_loads_3d(solid: &MeshedSolid, pressure: f64) -> Vec<NodalLoad> {
    let mut tributary: HashMap<String, f64> = HashMap::new();
    for tri in &solid.top_footprint_tris {
        let area = triangle_area_2d(solid.top_footprint_points[tri[0] as usize], solid.top_footprint_points[tri[1] as usize], solid.top_footprint_points[tri[2] as usize]);
        for &idx in tri {
            let node_id = solid.node_ids[solid.top_offset + idx as usize].clone();
            *tributary.entry(node_id).or_insert(0.0) += area / 3.0;
        }
    }
    tributary.into_iter().map(|(node_id, trib)| NodalLoad { node_id, dof: Dof::Tz, value: -pressure * trib }).collect()
}

/// 🌬️ Translates one `FemLoadCase`'s loads into `(nodal_loads, member_loads)`, resolving `Area` loads
/// against the already-meshed `solids` — shared by `build_model`, `fem3d_solve_all`, and buckling's
/// reference-case resolution.
fn translate_loads(loads: &[FemLoad], solids: &[MeshedSolid]) -> Result<(Vec<NodalLoad>, Vec<(String, MemberUdl)>), Fem3dError> {
    let mut nodal_loads = Vec::new();
    let mut member_loads = Vec::new();
    for load in loads {
        match load {
            FemLoad::Nodal { node_id, dof, value, .. } => nodal_loads.push(NodalLoad { node_id: node_id.clone(), dof: Dof::from(*dof), value: *value }),
            FemLoad::MemberUdl { element_id, wx, wy, wz, .. } => member_loads.push((element_id.clone(), MemberUdl { wx: *wx, wy: *wy, wz: *wz })),
            FemLoad::Area { solid_id, pressure, .. } => {
                let solid = solids.iter().find(|s| &s.solid_id == solid_id).ok_or_else(|| Fem3dError::UnknownSolidId(solid_id.clone()))?;
                nodal_loads.extend(area_load_nodal_loads_3d(solid, *pressure));
            }
        }
    }
    Ok((nodal_loads, member_loads))
}
// #endregion 🔖SolidMeshing

/// 🌉 Resolves a `Fem3dDocument` load case into a `fem_core::Model`: nodes, `Bar3`/`Frame3`/`Tet4`
/// elements (materials/sections looked up by id), supports, and the named load case's translated loads.
pub fn build_model(doc: &Fem3dDocument, case_id: &str) -> Result<Model, Fem3dError> {
    let (nodes, elements, solids, supports) = resolve_geometry(doc)?;
    let case = doc.load_cases.iter().find(|c| c.id == case_id).ok_or_else(|| Fem3dError::LoadCaseNotFound(case_id.to_string()))?;
    let (nodal_loads, member_loads) = translate_loads(&case.loads, &solids)?;
    Ok(Model { nodes, elements, supports, nodal_loads, member_loads })
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
    let (nodes, elements, solids, supports) = resolve_geometry(doc)?;
    let model = analyses::AnalysisModel { nodes, elements, supports };
    let mut cases = Vec::with_capacity(doc.load_cases.len());
    for case in &doc.load_cases {
        let (nodal_loads, member_loads) = translate_loads(&case.loads, &solids)?;
        cases.push(analyses::LoadCase { id: case.id.clone(), nodal_loads, member_loads, self_weight: case.self_weight });
    }
    let combinations: Vec<analyses::Combination> =
        doc.combinations.iter().map(|combination| analyses::Combination { id: combination.id.clone(), terms: combination.terms.iter().map(|(id, factor)| (id.clone(), *factor)).collect() }).collect();
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
    let (nodes, elements, _solids, supports) = resolve_geometry(doc)?;
    let model = analyses::AnalysisModel { nodes, elements, supports };
    analyses::modal(&model, doc.analysis.modal_count).map_err(Fem3dError::from)
}

/// 🌉 Richer modal entry point: solves the same modal analysis as `fem3d_modal` but also unpacks mode
/// `mode_index`'s shape into a per-node `[f64;6]` displacement map. Returns
/// `(frequency_hz, node_id -> displacement values)`.
pub fn fem3d_modal_mode_values(doc: &Fem3dDocument, mode_index: usize) -> Result<(f64, HashMap<String, [f64; 6]>), Fem3dError> {
    let (nodes, elements, _solids, supports) = resolve_geometry(doc)?;
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

/// 🌉 Shared buckling-case resolution for `fem3d_buckling`/`fem3d_buckling_mode_values`, mirroring
/// `fem2d`'s `buckling_inputs` — translates the named case's loads (incl. `Area` against `solids`).
fn buckling_case(doc: &Fem3dDocument, case_id: &str, solids: &[MeshedSolid]) -> Result<analyses::LoadCase, Fem3dError> {
    let case = doc.load_cases.iter().find(|c| c.id == case_id).ok_or_else(|| Fem3dError::LoadCaseNotFound(case_id.to_string()))?;
    let (nodal_loads, member_loads) = translate_loads(&case.loads, solids)?;
    Ok(analyses::LoadCase { id: case.id.clone(), nodal_loads, member_loads, self_weight: case.self_weight })
}

/// 🏛️ Linear buckling: lowest `doc.analysis.buckling_count` load factors/mode shapes for `case_id`.
pub fn fem3d_buckling(doc: &Fem3dDocument, case_id: &str) -> Result<analyses::BucklingResult, Fem3dError> {
    let (nodes, elements, solids, supports) = resolve_geometry(doc)?;
    let case = buckling_case(doc, case_id, &solids)?;
    let model = analyses::AnalysisModel { nodes, elements, supports };
    analyses::buckling(&model, &case, doc.analysis.buckling_count).map_err(Fem3dError::from)
}

/// 🌉 Richer buckling entry point: mirrors `fem3d_modal_mode_values` — solves the same buckling
/// analysis as `fem3d_buckling` but also unpacks mode `mode_index`'s shape into a per-node
/// displacement map. Returns `(load_factor, node_id -> displacement values)`.
pub fn fem3d_buckling_mode_values(doc: &Fem3dDocument, case_id: &str, mode_index: usize) -> Result<(f64, HashMap<String, [f64; 6]>), Fem3dError> {
    let (nodes, elements, solids, supports) = resolve_geometry(doc)?;
    let order = mode_dof_order(&nodes, &elements);
    let case = buckling_case(doc, case_id, &solids)?;
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

// #region 🔖SolidMeshPreview
/// 🗺️ One meshed solid's cheap preview geometry — the full volume mesh (points/tets) plus its outer
/// boundary triangulation (via `fem_core::mesh::boundary_faces`) for surface rendering, WITHOUT
/// building any `fem_core::Element`. Mirrors `fem_2d::RegionMesh`/`fem2d_mesh_preview`.
pub struct SolidMesh {
    pub solid_id: String,
    pub points: Vec<[f64; 3]>,
    pub tets: Vec<[u32; 4]>,
    pub boundary_tris: Vec<[u32; 3]>,
    pub node_ids: Vec<String>,
}

/// 🗺️ Triangulates+extrudes+tet-splits every `FemSolid` in `doc` (same deterministic
/// `fem_core::mesh` calls as `resolve_geometry`, so tet indices line up with `"{solid_id}_c{i}"`
/// element ids) and returns just the geometry plus its outer surface — cheap enough for every render.
pub fn fem3d_mesh_preview(doc: &Fem3dDocument) -> Result<Vec<SolidMesh>, Fem3dError> {
    let mut out = Vec::with_capacity(doc.solids.len());
    for solid in &doc.solids {
        let domain = fem_core::mesh::PlanarDomain { outer: solid.outline.clone(), holes: solid.holes.clone() };
        let opts = fem_core::mesh::MeshOpts { max_edge: solid.mesh_size, min_angle_deg: 20.0 };
        let tri_mesh = fem_core::mesh::triangulate(&domain, &opts).map_err(|e| Fem3dError::MeshFailed { solid_id: solid.id.clone(), reason: e.to_string() })?;
        let volume_mesh = fem_core::mesh::extrude_tri_mesh(&tri_mesh, solid.height, solid.layers.max(1));
        let tet_mesh = fem_core::mesh::split_to_tets(&volume_mesh);
        let points: Vec<[f64; 3]> = tet_mesh.points.iter().map(|p| [p[0], p[1], p[2] + solid.base_z]).collect();
        let node_ids = points
            .iter()
            .enumerate()
            .map(|(point_index, p)| match doc.nodes.iter().find(|n| (n.x - p[0]).abs() < 1e-9 && (n.y - p[1]).abs() < 1e-9 && (n.z - p[2]).abs() < 1e-9) {
                Some(n) => n.id.clone(),
                None => format!("{}_m{}", solid.id, point_index),
            })
            .collect();
        let tets: Vec<[u32; 4]> = tet_mesh.cells.iter().filter_map(|c| match c { fem_core::mesh::Cell::Tet4(t) => Some(*t), _ => None }).collect();
        let boundary_mesh = fem_core::mesh::VolumeMesh { points: points.clone(), cells: tet_mesh.cells };
        let boundary_tris = fem_core::mesh::boundary_faces(&boundary_mesh);
        out.push(SolidMesh { solid_id: solid.id.clone(), points, tets, boundary_tris, node_ids });
    }
    Ok(out)
}

/// 🎨 Nodal-averaged von Mises stress for `case_id`'s solved result, keyed by node id — the
/// document-layer bridge to `fem_core::analyses::nodal_averaged_scalar`, mirroring `fem_2d`'s
/// `fem2d_nodal_von_mises`, feeding `fem-plugin`'s solid stress contour rendering.
pub fn fem3d_nodal_von_mises(doc: &Fem3dDocument, case_id: &str) -> Result<HashMap<String, f64>, Fem3dError> {
    let (nodes, elements, _solids, supports) = resolve_geometry(doc)?;
    let model = analyses::AnalysisModel { nodes, elements, supports };
    let results = fem3d_solve_all(doc)?;
    let result = results.get(case_id).ok_or_else(|| Fem3dError::LoadCaseNotFound(case_id.to_string()))?;
    Ok(analyses::nodal_averaged_scalar(&model, result, analyses::StressScalar::VonMises))
}
// #endregion 🔖SolidMeshPreview
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
                None => Fem3dStore::new(create_document_envelope(FEM_3D_SCHEMA, "fem3d", empty_fem3d_projection(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_text(command_text).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_binary(command_bytes).map_err(|e| JsValue::from_str(&e.to_string()))
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
use store::{create_document_envelope, DocumentDsl, DocumentCommand};

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
            load_cases: vec![FemLoadCase { id: "point".into(), name: "Point Load".into(), loads: vec![FemLoad::Nodal { id: "l1".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -p }], self_weight: false }],
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
            materials: vec![FemMaterial { id: "steel".into(), name: "Steel".into(), e: 210e9, g: 80.77e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "rod".into(), name: "Rod".into(), area: 0.001, iy: 1e-6, iz: 1e-6, j: 1e-6 }],
            solids: vec![],
            supports: vec![
                FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: FemDof::ALL.to_vec() },
                FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: FemDof::ALL.to_vec() },
                FemSupport { id: "s3".into(), node_id: "n4".into(), fixed: FemDof::ALL.to_vec() },
            ],
            load_cases: vec![FemLoadCase { id: "drop".into(), name: "Drop".into(), loads: vec![FemLoad::Nodal { id: "l1".into(), node_id: "n3".into(), dof: FemDof::Tz, value: -1000.0 }], self_weight: false }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
            camera: FemCamera::default(),
        }
    }
    // #endregion 🔖Fixtures

    // #region 🔖OpRoundTrip
    fn round_trip(projection: &Fem3dDocument, operation: &Fem3dOperation) -> Fem3dDocument {
        let forward = apply_operation(projection, operation);
        let mut restored = forward.clone();
        for back in operation.backwards(projection) {
            restored = apply_operation(&restored, &back);
        }
        assert_eq!(&restored, projection, "backwards() must restore the pre-operation document");
        forward
    }

    #[test]
    fn node_set_and_remove_round_trip() {
        let base = empty_fem3d_projection();
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
        let load_case = FemLoadCase { id: "point".into(), name: "Point Load Updated".into(), loads: vec![FemLoad::Nodal { id: "l1".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -9000.0 }], self_weight: false };
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
    fn camera_set_round_trips() {
        let base = empty_fem3d_projection();
        round_trip(&base, &Fem3dOperation::SetCamera { camera: FemCamera { json: "{\"zoom\":2}".into() } });
    }

    #[test]
    fn analysis_settings_set_round_trips() {
        let base = empty_fem3d_projection();
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
        let mut diff = Fem3dOperation::SetCamera { camera: FemCamera { json: "{\"zoom\":2}".into() } }.diff(&base);
        diff.absorb(Fem3dOperation::SetDocument { document: replacement.clone() }.diff(&base));
        assert_eq!(diff.apply(&base), replacement);
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
        doc.load_cases.push(FemLoadCase { id: "point2".into(), name: "Point Load 2".into(), loads: vec![FemLoad::Nodal { id: "l2".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -2000.0 }], self_weight: false });
        doc.combinations = vec![FemCombination { id: "uls".into(), name: "ULS".into(), terms: BTreeMap::from([("point".into(), 1.35), ("point2".into(), 1.0)]) }];

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

    /// 🌬️ A `FemLoad::MemberUdl` on the cantilever fixture's `Frame3`: base shear must equal the
    /// classical `wL` total, same benchmark `elements3d::tests::frame3_udl_cantilever_matches_hand_calc`
    /// checks headlessly, now exercised through the document bridge's load translation.
    #[test]
    fn member_udl_load_matches_total_wl() {
        let (mut doc, _e, _iy, l, _p, _iz) = cantilever_fixture();
        let w = 800.0;
        doc.load_cases = vec![FemLoadCase { id: "udl".into(), name: "UDL".into(), loads: vec![FemLoad::MemberUdl { id: "u1".into(), element_id: "e1".into(), wx: 0.0, wy: 0.0, wz: -w }], self_weight: false }];
        let results = fem3d_solve_all(&doc).expect("solves");
        let result = results.get("udl").unwrap();
        let total_tz_reaction: f64 = result.reactions.iter().filter(|r| r.dof == Dof::Tz).map(|r| r.value).sum();
        let expected = w * l;
        assert!((total_tz_reaction - expected).abs() / expected < 1e-6, "reaction sum {total_tz_reaction} vs expected {expected}");
    }
    // #endregion 🔖SolveAll

    // #region 🔖Solids
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
            camera: FemCamera::default(),
        }
    }

    #[test]
    fn solid_op_round_trips() {
        let base = solid_slab_doc();
        let updated = FemSolid { id: "sol1".into(), name: "Slab Updated".into(), outline: base.solids[0].outline.clone(), holes: vec![], base_z: 0.0, height: 0.8, layers: 2, mesh_size: 0.5, material_id: "concrete".into() };
        let after_set = round_trip(&base, &Fem3dOperation::SetSolid { index: 0, solid: updated });
        assert_eq!(after_set.solids[0].height, 0.8);
        round_trip(&after_set, &Fem3dOperation::RemoveSolid { id: "sol1".into() });
    }

    #[test]
    fn solid_self_weight_matches_total_mass_times_gravity() {
        let doc = solid_slab_doc();
        let results = fem3d_solve_all(&doc).expect("solid self-weight solves");
        let result = results.get("self").unwrap();
        let total_tz_reaction: f64 = result.reactions.iter().filter(|r| r.dof == Dof::Tz).map(|r| r.value).sum();
        let (footprint_area, height, rho, g) = (2.0 * 1.0, 0.5, 2400.0, 9.81);
        let expected = rho * footprint_area * height * g;
        assert!((total_tz_reaction - expected).abs() / expected < 1e-6, "reaction sum {total_tz_reaction} vs expected {expected}");
    }

    /// ⚖️ A uniform pressure over the solid's top face must balance EXACTLY (mesh-independent, since
    /// tributary-area nodal loads sum to `pressure * footprintArea` regardless of triangulation) —
    /// possible only now that `fem_3d` meshes solids at all.
    #[test]
    fn solid_area_load_matches_pressure_times_footprint_area() {
        let mut doc = solid_slab_doc();
        doc.load_cases = vec![FemLoadCase { id: "pressure".into(), name: "Pressure".into(), loads: vec![FemLoad::Area { id: "a1".into(), solid_id: "sol1".into(), pressure: 8000.0 }], self_weight: false }];
        let results = fem3d_solve_all(&doc).expect("solid pressure load solves");
        let result = results.get("pressure").unwrap();
        let total_tz_reaction: f64 = result.reactions.iter().filter(|r| r.dof == Dof::Tz).map(|r| r.value).sum();
        let expected = 8000.0 * 2.0 * 1.0;
        assert!((total_tz_reaction - expected).abs() / expected < 1e-6, "reaction sum {total_tz_reaction} vs expected {expected}");
    }

    #[test]
    fn fem3d_mesh_preview_returns_solid_tets_and_boundary() {
        let doc = solid_slab_doc();
        let previews = fem3d_mesh_preview(&doc).expect("mesh preview succeeds");
        assert_eq!(previews.len(), 1);
        assert_eq!(previews[0].solid_id, "sol1");
        assert!(!previews[0].tets.is_empty(), "expected at least one tet");
        assert!(!previews[0].boundary_tris.is_empty(), "expected boundary triangles");
        assert_eq!(previews[0].node_ids.len(), previews[0].points.len());
        for corner_id in ["sc0", "sc1", "sc2", "sc3"] {
            assert!(previews[0].node_ids.contains(&corner_id.to_string()), "expected corner {corner_id} to be reused");
        }
    }

    #[test]
    fn fem3d_nodal_von_mises_returns_finite_values_for_solid() {
        let mut doc = solid_slab_doc();
        doc.load_cases = vec![FemLoadCase { id: "pressure".into(), name: "Pressure".into(), loads: vec![FemLoad::Area { id: "a1".into(), solid_id: "sol1".into(), pressure: 8000.0 }], self_weight: false }];
        let averaged = fem3d_nodal_von_mises(&doc, "pressure").expect("nodal von mises solves");
        assert!(!averaged.is_empty());
        for v in averaged.values() {
            assert!(v.is_finite() && *v >= 0.0, "von mises {v} should be finite and non-negative");
        }
    }
    // #endregion 🔖Solids

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

    // #region 🔖DslAndOpText
    #[test]
    fn fem3d_dsl_round_trips_fixture_documents() {
        store::test_support::assert_dsl_round_trip(&empty_fem3d_projection());
        store::test_support::assert_dsl_pack_equivalence(&empty_fem3d_projection());
        let (cantilever, ..) = cantilever_fixture();
        store::test_support::assert_dsl_round_trip(&cantilever);
        store::test_support::assert_dsl_pack_equivalence(&cantilever);
        store::test_support::assert_dsl_round_trip(&truss_fixture());
        store::test_support::assert_dsl_pack_equivalence(&truss_fixture());
        store::test_support::assert_dsl_round_trip(&solid_slab_doc());
        store::test_support::assert_dsl_pack_equivalence(&solid_slab_doc());
        let mut with_combination = cantilever;
        with_combination.combinations.push(FemCombination { id: "uls".into(), name: "ULS".into(), terms: BTreeMap::from([("point".into(), 1.35)]) });
        store::test_support::assert_dsl_round_trip(&with_combination);
        store::test_support::assert_dsl_pack_equivalence(&with_combination);
    }

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
                    FemLoad::Nodal { id: "l1".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -5000.0 },
                    FemLoad::MemberUdl { id: "l2".into(), element_id: "e1".into(), wx: 0.0, wy: 0.0, wz: -800.0 },
                    FemLoad::Area { id: "l3".into(), solid_id: "sol1".into(), pressure: 800.0 },
                ],
                self_weight: true,
            },
        });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::RemoveLoadCase { id: "point".into() });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::SetCombination { index: 0, combination: FemCombination { id: "uls".into(), name: "ULS".into(), terms: BTreeMap::from([("point".into(), 1.35), ("live".into(), 1.5)]) } });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::RemoveCombination { id: "uls".into() });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::SetCamera { camera: FemCamera { json: "{\"zoom\":2}".into() } });
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } });
        let (cantilever, ..) = cantilever_fixture();
        store::test_support::assert_op_line_round_trip(&Fem3dOperation::SetDocument { document: cantilever });
    }

    #[test]
    fn fem3d_document_text_round_trips_through_the_store() {
        let mut store = Fem3dStore::new(create_document_envelope(FEM_3D_SCHEMA, "fem3d", empty_fem3d_projection(), None));
        let (cantilever, ..) = cantilever_fixture();
        store.dispatch(DocumentCommand::Apply { operations: vec![Fem3dOperation::SetDocument { document: cantilever }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
    // #endregion 🔖DslAndOpText

    #[test]
    fn example_fixture_parses() {
        let dsl = include_str!("../example/default.fem3d");
        let doc: Fem3dDocument = Fem3dDocument::parse_dsl(dsl).expect("example fixture parses");
        assert_eq!(doc.nodes.len(), 16);
        assert_eq!(doc.elements.len(), 16);
        assert_eq!(doc.solids.len(), 1);
        let result = fem3d_solve(&doc, "dead").expect("example fixture solves");
        assert!(result.checks.residual_norm < 1e-6);

        let all_results = fem3d_solve_all(&doc).expect("example fixture solves all");
        assert!(all_results.contains_key("dead"), "expected dead case result");
        assert!(all_results.contains_key("live"), "expected live case result");
        assert!(all_results.contains_key("uls"), "expected uls combination result");
        let dead = all_results.get("dead").expect("expected dead case result (solid area load + member UDL + self-weight)");
        assert!(dead.checks.residual_norm < 1e-6, "residual {}", dead.checks.residual_norm);

        let previews = fem3d_mesh_preview(&doc).expect("mesh preview succeeds");
        assert_eq!(previews.len(), 1);
        assert!(!previews[0].tets.is_empty(), "expected at least one tet");
        assert!(!previews[0].boundary_tris.is_empty(), "expected boundary triangles");

        let averaged = fem3d_nodal_von_mises(&doc, "dead").expect("nodal von mises solves");
        assert!(!averaged.is_empty(), "expected at least one averaged nodal value");
        for v in averaged.values() {
            assert!(v.is_finite() && *v >= 0.0, "von mises {v} should be finite and non-negative");
        }

        let buckling = fem3d_buckling(&doc, "dead").expect("buckling resolves for the dead case's compressed column");
        assert!(buckling.factors[0].is_finite() && buckling.factors[0] > 1.0, "expected an illustrative (finite, >1) load factor: {:?}", buckling.factors);
    }

}
// #endregion 🔖Tests
