//! 🏙️ FEM 3D document model and element library on `vcs`.

use fem_core::{Bar3, Dof, Element, ElementResult, Frame3, Model, NodalLoad, Node, Support};
use serde::{Deserialize, Serialize};
use vcs::{create_document_vcs_envelope, DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff};

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

/// 🧱 Linear-elastic isotropic material: Young's modulus `e` and shear modulus `g`, in Pascals.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemMaterial {
    pub id: String,
    pub name: String,
    pub e: f64,
    pub g: f64,
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

/// 📦 A named set of nodal loads solved together; v0 `fem_3d` scope has no member UDLs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemLoadCase {
    pub id: String,
    pub name: String,
    pub loads: Vec<FemNodalLoad>,
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
    pub camera: Option<FemCamera>,
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
        if let Some(camera) = &self.camera {
            next.camera = camera.clone();
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
        if other.camera.is_some() {
            self.camera = other.camera;
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
    SetCamera { camera: FemCamera },
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
            Fem3dOp::SetCamera { camera } => diff.camera = Some(camera.clone()),
        }
        diff
    }

    fn backwards(&self, projection: &Fem3dDocument) -> Vec<Self> {
        match self {
            Fem3dOp::SetNode { node, .. } => match node_index(projection, &node.id) {
                Some(index) => vec![Fem3dOp::SetNode { index, node: projection.nodes[index].clone() }],
                None => vec![Fem3dOp::RemoveNode { id: node.id.clone() }],
            },
            Fem3dOp::RemoveNode { id } => node_index(projection, id)
                .map(|index| vec![Fem3dOp::SetNode { index, node: projection.nodes[index].clone() }])
                .unwrap_or_default(),
            Fem3dOp::SetElement { element, .. } => match element_index(projection, element_id(element)) {
                Some(index) => vec![Fem3dOp::SetElement { index, element: projection.elements[index].clone() }],
                None => vec![Fem3dOp::RemoveElement { id: element_id(element).to_string() }],
            },
            Fem3dOp::RemoveElement { id } => element_index(projection, id)
                .map(|index| vec![Fem3dOp::SetElement { index, element: projection.elements[index].clone() }])
                .unwrap_or_default(),
            Fem3dOp::SetMaterial { material, .. } => match material_index(projection, &material.id) {
                Some(index) => vec![Fem3dOp::SetMaterial { index, material: projection.materials[index].clone() }],
                None => vec![Fem3dOp::RemoveMaterial { id: material.id.clone() }],
            },
            Fem3dOp::RemoveMaterial { id } => material_index(projection, id)
                .map(|index| vec![Fem3dOp::SetMaterial { index, material: projection.materials[index].clone() }])
                .unwrap_or_default(),
            Fem3dOp::SetSection { section, .. } => match section_index(projection, &section.id) {
                Some(index) => vec![Fem3dOp::SetSection { index, section: projection.sections[index].clone() }],
                None => vec![Fem3dOp::RemoveSection { id: section.id.clone() }],
            },
            Fem3dOp::RemoveSection { id } => section_index(projection, id)
                .map(|index| vec![Fem3dOp::SetSection { index, section: projection.sections[index].clone() }])
                .unwrap_or_default(),
            Fem3dOp::SetSupport { support, .. } => match support_index(projection, &support.id) {
                Some(index) => vec![Fem3dOp::SetSupport { index, support: projection.supports[index].clone() }],
                None => vec![Fem3dOp::RemoveSupport { id: support.id.clone() }],
            },
            Fem3dOp::RemoveSupport { id } => support_index(projection, id)
                .map(|index| vec![Fem3dOp::SetSupport { index, support: projection.supports[index].clone() }])
                .unwrap_or_default(),
            Fem3dOp::SetLoadCase { load_case, .. } => match load_case_index(projection, &load_case.id) {
                Some(index) => vec![Fem3dOp::SetLoadCase { index, load_case: projection.load_cases[index].clone() }],
                None => vec![Fem3dOp::RemoveLoadCase { id: load_case.id.clone() }],
            },
            Fem3dOp::RemoveLoadCase { id } => load_case_index(projection, id)
                .map(|index| vec![Fem3dOp::SetLoadCase { index, load_case: projection.load_cases[index].clone() }])
                .unwrap_or_default(),
            Fem3dOp::SetCamera { .. } => vec![Fem3dOp::SetCamera { camera: projection.camera.clone() }],
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
/// 🌉 Resolves a `Fem3dDocument` load case into a `fem_core::Model`: nodes, `Bar3`/`Frame3` elements
/// (materials/sections looked up by id), supports, and the named load case's nodal loads.
pub fn build_model(doc: &Fem3dDocument, case_id: &str) -> Result<Model, String> {
    let mut model = Model::default();
    for node in &doc.nodes {
        model.nodes.push(Node { id: node.id.clone(), pos: [node.x, node.y, node.z] });
    }
    let node_exists = |id: &str| doc.nodes.iter().any(|n| n.id == id);
    for element in &doc.elements {
        match element {
            FemElement::Bar { id, start, end, material_id, section_id } => {
                let material = doc
                    .materials
                    .iter()
                    .find(|m| &m.id == material_id)
                    .ok_or_else(|| format!("material not found: {material_id}"))?;
                let section = doc
                    .sections
                    .iter()
                    .find(|s| &s.id == section_id)
                    .ok_or_else(|| format!("section not found: {section_id}"))?;
                if !node_exists(start) {
                    return Err(format!("node not found: {start}"));
                }
                if !node_exists(end) {
                    return Err(format!("node not found: {end}"));
                }
                model.elements.push(Box::new(Bar3 {
                    id: id.clone(),
                    node_a: start.clone(),
                    node_b: end.clone(),
                    e: material.e,
                    a: section.area,
                }));
            }
            FemElement::Frame { id, start, end, material_id, section_id, roll } => {
                let material = doc
                    .materials
                    .iter()
                    .find(|m| &m.id == material_id)
                    .ok_or_else(|| format!("material not found: {material_id}"))?;
                let section = doc
                    .sections
                    .iter()
                    .find(|s| &s.id == section_id)
                    .ok_or_else(|| format!("section not found: {section_id}"))?;
                if !node_exists(start) {
                    return Err(format!("node not found: {start}"));
                }
                if !node_exists(end) {
                    return Err(format!("node not found: {end}"));
                }
                model.elements.push(Box::new(Frame3 {
                    id: id.clone(),
                    node_a: start.clone(),
                    node_b: end.clone(),
                    e: material.e,
                    g: material.g,
                    a: section.area,
                    iy: section.iy,
                    iz: section.iz,
                    j: section.j,
                    roll: *roll,
                }));
            }
        }
    }
    for support in &doc.supports {
        model.supports.push(Support { node_id: support.node_id.clone(), fixed: support.fixed.clone() });
    }
    let case = doc.load_cases.iter().find(|c| c.id == case_id).ok_or_else(|| format!("load case not found: {case_id}"))?;
    for load in &case.loads {
        model.nodal_loads.push(NodalLoad { node_id: load.node_id.clone(), dof: load.dof, value: load.value });
    }
    Ok(model)
}

/// 🚀 Frozen entry point: builds the model for `case_id` and runs `fem_core::solve_linear_static`.
/// Consumed directly by `fem-plugin` — do not rename or change this signature.
pub fn fem3d_solve(doc: &Fem3dDocument, case_id: &str) -> Result<fem_core::StaticResult, String> {
    let model = build_model(doc, case_id)?;
    fem_core::solve_linear_static(&model).map_err(|e| e.to_string())
}
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
                None => {
                    Fem3dStore::new(create_document_vcs_envelope(FEM_3D_SCHEMA, "fem3d", empty_fem3d_projection(), None))
                }
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
            nodes: vec![
                FemNode { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 },
                FemNode { id: "n2".into(), x: l, y: 0.0, z: 0.0 },
            ],
            elements: vec![FemElement::Frame {
                id: "e1".into(),
                start: "n1".into(),
                end: "n2".into(),
                material_id: "steel".into(),
                section_id: "hea200".into(),
                roll: 0.0,
            }],
            materials: vec![FemMaterial { id: "steel".into(), name: "Steel".into(), e, g }],
            sections: vec![FemSection { id: "hea200".into(), name: "HEA200".into(), area: a, iy, iz, j }],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: Dof::ALL.to_vec() }],
            load_cases: vec![FemLoadCase {
                id: "point".into(),
                name: "Point Load".into(),
                loads: vec![FemNodalLoad { id: "l1".into(), node_id: "n2".into(), dof: Dof::Tz, value: -p }],
            }],
            camera: FemCamera::default(),
        };
        (doc, e, iy, l, p, iz)
    }

    /// 🔺 A free 3D joint needs at least 3 non-coplanar bars to be kinematically determinate — two
    /// bars only span a plane, leaving one direction with zero stiffness (a mechanism). Hence n4/b3.
    fn truss_fixture() -> Fem3dDocument {
        Fem3dDocument {
            nodes: vec![
                FemNode { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 },
                FemNode { id: "n2".into(), x: 2.0, y: 0.0, z: 0.0 },
                FemNode { id: "n3".into(), x: 1.0, y: 1.0, z: 2.0 },
                FemNode { id: "n4".into(), x: 1.0, y: -1.0, z: 0.0 },
            ],
            elements: vec![
                FemElement::Bar { id: "b1".into(), start: "n1".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
                FemElement::Bar { id: "b2".into(), start: "n2".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
                FemElement::Bar { id: "b3".into(), start: "n4".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
            ],
            materials: vec![FemMaterial { id: "steel".into(), name: "Steel".into(), e: 210e9, g: 80.77e9 }],
            sections: vec![FemSection { id: "rod".into(), name: "Rod".into(), area: 0.001, iy: 1e-6, iz: 1e-6, j: 1e-6 }],
            supports: vec![
                FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: Dof::ALL.to_vec() },
                FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: Dof::ALL.to_vec() },
                FemSupport { id: "s3".into(), node_id: "n4".into(), fixed: Dof::ALL.to_vec() },
            ],
            load_cases: vec![FemLoadCase {
                id: "drop".into(),
                name: "Drop".into(),
                loads: vec![FemNodalLoad { id: "l1".into(), node_id: "n3".into(), dof: Dof::Tz, value: -1000.0 }],
            }],
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
        round_trip(&after_set, &Fem3dOp::RemoveNode { id: node.id.clone() });
    }

    #[test]
    fn element_set_and_remove_round_trip() {
        let (base, ..) = cantilever_fixture();
        let updated = FemElement::Frame {
            id: "e1".into(),
            start: "n1".into(),
            end: "n2".into(),
            material_id: "steel".into(),
            section_id: "hea200".into(),
            roll: 0.5,
        };
        let after_set = round_trip(&base, &Fem3dOp::SetElement { index: 0, element: updated });
        round_trip(&after_set, &Fem3dOp::RemoveElement { id: "e1".into() });
    }

    #[test]
    fn material_set_and_remove_round_trip() {
        let (base, ..) = cantilever_fixture();
        let material = FemMaterial { id: "steel".into(), name: "Steel Updated".into(), e: 200e9, g: 79e9 };
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
        let load_case = FemLoadCase {
            id: "point".into(),
            name: "Point Load Updated".into(),
            loads: vec![FemNodalLoad { id: "l1".into(), node_id: "n2".into(), dof: Dof::Tz, value: -9000.0 }],
        };
        let after_set = round_trip(&base, &Fem3dOp::SetLoadCase { index: 0, load_case });
        round_trip(&after_set, &Fem3dOp::RemoveLoadCase { id: "point".into() });
    }

    #[test]
    fn camera_set_round_trips() {
        let base = empty_fem3d_projection();
        round_trip(&base, &Fem3dOp::SetCamera { camera: FemCamera { json: "{\"zoom\":2}".into() } });
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
        assert!(err.contains("missing"), "error should name the dangling id: {err}");
    }

    #[test]
    fn build_model_rejects_dangling_section() {
        let (mut doc, ..) = cantilever_fixture();
        if let FemElement::Frame { section_id, .. } = &mut doc.elements[0] {
            *section_id = "missing".into();
        }
        let err = build_model(&doc, "point").unwrap_err();
        assert!(err.contains("missing"), "error should name the dangling id: {err}");
    }

    #[test]
    fn build_model_rejects_dangling_node() {
        let (mut doc, ..) = cantilever_fixture();
        if let FemElement::Frame { end, .. } = &mut doc.elements[0] {
            *end = "missing".into();
        }
        let err = build_model(&doc, "point").unwrap_err();
        assert!(err.contains("missing"), "error should name the dangling id: {err}");
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

    #[test]
    fn example_fixture_parses() {
        let json = include_str!("../example/default.fem3d.json");
        let doc: Fem3dDocument = serde_json::from_str(json).expect("example fixture parses");
        assert_eq!(doc.nodes.len(), 2);
        assert_eq!(doc.elements.len(), 1);
        let result = fem3d_solve(&doc, "point").expect("example fixture solves");
        assert!(result.checks.residual_norm < 1e-6);
    }
}
// #endregion 🔖Tests
