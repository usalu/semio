//! 📐 FEM 2D document model and element library on `vcs`.

use fem_core::{Bar2, BeamEb2, Dof, Element, ElementResult, MemberUdl, Model, NodalLoad, Node, Support};
use serde::{Deserialize, Serialize};
use vcs::{create_document_vcs_envelope, DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff};

pub const FEM_2D_SCHEMA: &str = "fem.2d";

// #region 🔖Document
/// 📍 A structural node in plan (x, y in meters).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemNode {
    pub id: String,
    pub x: f64,
    pub y: f64,
}

/// 🔩 A 2-node structural member — axial-only `Bar` or axial+bending `Beam`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FemElement {
    #[serde(rename_all = "camelCase")]
    Bar { id: String, start: String, end: String, material_id: String, section_id: String },
    #[serde(rename_all = "camelCase")]
    Beam { id: String, start: String, end: String, material_id: String, section_id: String },
}

/// 🪪 A `FemElement`'s stable id, across both variants.
pub fn element_id(element: &FemElement) -> &str {
    match element {
        FemElement::Bar { id, .. } | FemElement::Beam { id, .. } => id,
    }
}

/// 🧱 An isotropic material — Young's modulus `e` in Pascals.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemMaterial {
    pub id: String,
    pub name: String,
    pub e: f64,
}

/// 📏 A cross-section — area in m², strong-axis moment of inertia `iy` in m⁴.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemSection {
    pub id: String,
    pub name: String,
    pub area: f64,
    pub iy: f64,
}

/// 🔒 A support: the subset of a node's DOFs restrained to zero displacement.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemSupport {
    pub id: String,
    pub node_id: String,
    pub fixed: Vec<Dof>,
}

/// 🏋️ A load, either a concentrated nodal force/moment or a member UDL.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FemLoad {
    #[serde(rename_all = "camelCase")]
    Nodal { id: String, node_id: String, dof: Dof, value: f64 },
    #[serde(rename_all = "camelCase")]
    MemberUdl { id: String, element_id: String, wx: f64, wy: f64 },
}

/// 🪪 A `FemLoad`'s stable id, across both variants.
pub fn load_id(load: &FemLoad) -> &str {
    match load {
        FemLoad::Nodal { id, .. } | FemLoad::MemberUdl { id, .. } => id,
    }
}

/// 📦 A named set of loads applied together for one analysis run.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemLoadCase {
    pub id: String,
    pub name: String,
    pub loads: Vec<FemLoad>,
}

/// 🎥 The canvas camera (pan/zoom) for the plugin viewport.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for FemCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

/// 🧾 Persistent fem-2d document — nodes, members, materials/sections, supports and load cases.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem2dDocument {
    pub nodes: Vec<FemNode>,
    pub elements: Vec<FemElement>,
    pub materials: Vec<FemMaterial>,
    pub sections: Vec<FemSection>,
    pub supports: Vec<FemSupport>,
    pub load_cases: Vec<FemLoadCase>,
    pub camera: FemCamera,
}
// #endregion 🔖Document

// #region 🔖Collections
/// 🪪 Stable-id accessor shared by every id-keyed document collection entry.
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

/// 🩹 Sparse id-keyed collection diff — removals plus id-or-index `set`s (replace when the id
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

/// 🩹 Applies a sparse id-keyed diff to a collection in place — remove-by-id, then replace-by-id
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
// #endregion 🔖Collections

// #region 🔖Ops
/// 🩹 Sparse fem-2d diff over every document collection plus the scalar camera.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem2dDiff {
    pub nodes: NodesDiff,
    pub elements: ElementsDiff,
    pub materials: MaterialsDiff,
    pub sections: SectionsDiff,
    pub supports: SupportsDiff,
    pub load_cases: LoadCasesDiff,
    pub camera: Option<FemCamera>,
}

impl OperationDiff<Fem2dDocument> for Fem2dDiff {
    fn apply(&self, projection: &Fem2dDocument) -> Fem2dDocument {
        let mut next = projection.clone();
        apply_collection_diff(&mut next.nodes, &self.nodes.removed, &self.nodes.set);
        apply_collection_diff(&mut next.elements, &self.elements.removed, &self.elements.set);
        apply_collection_diff(&mut next.materials, &self.materials.removed, &self.materials.set);
        apply_collection_diff(&mut next.sections, &self.sections.removed, &self.sections.set);
        apply_collection_diff(&mut next.supports, &self.supports.removed, &self.supports.set);
        apply_collection_diff(&mut next.load_cases, &self.load_cases.removed, &self.load_cases.set);
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

/// 🧮 Fem-2d operation: id-keyed document-collection edits plus the scalar camera, each with a
/// true inverse computed from the pre-op projection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum Fem2dOp {
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

impl Operation<Fem2dDocument> for Fem2dOp {
    type Diff = Fem2dDiff;

    fn diff(&self, _projection: &Fem2dDocument) -> Fem2dDiff {
        let mut diff = Fem2dDiff::default();
        match self {
            Fem2dOp::SetNode { index, node } => diff.nodes.set.push((*index, node.clone())),
            Fem2dOp::RemoveNode { id } => diff.nodes.removed.push(id.clone()),
            Fem2dOp::SetElement { index, element } => diff.elements.set.push((*index, element.clone())),
            Fem2dOp::RemoveElement { id } => diff.elements.removed.push(id.clone()),
            Fem2dOp::SetMaterial { index, material } => diff.materials.set.push((*index, material.clone())),
            Fem2dOp::RemoveMaterial { id } => diff.materials.removed.push(id.clone()),
            Fem2dOp::SetSection { index, section } => diff.sections.set.push((*index, section.clone())),
            Fem2dOp::RemoveSection { id } => diff.sections.removed.push(id.clone()),
            Fem2dOp::SetSupport { index, support } => diff.supports.set.push((*index, support.clone())),
            Fem2dOp::RemoveSupport { id } => diff.supports.removed.push(id.clone()),
            Fem2dOp::SetLoadCase { index, load_case } => diff.load_cases.set.push((*index, load_case.clone())),
            Fem2dOp::RemoveLoadCase { id } => diff.load_cases.removed.push(id.clone()),
            Fem2dOp::SetCamera { camera } => diff.camera = Some(camera.clone()),
        }
        diff
    }

    fn backwards(&self, projection: &Fem2dDocument) -> Vec<Self> {
        match self {
            Fem2dOp::SetNode { node, .. } => match index_of(&projection.nodes, &node.id) {
                Some(index) => vec![Fem2dOp::SetNode { index, node: projection.nodes[index].clone() }],
                None => vec![Fem2dOp::RemoveNode { id: node.id.clone() }],
            },
            Fem2dOp::RemoveNode { id } => index_of(&projection.nodes, id)
                .map(|index| vec![Fem2dOp::SetNode { index, node: projection.nodes[index].clone() }])
                .unwrap_or_default(),
            Fem2dOp::SetElement { element, .. } => match index_of(&projection.elements, element_id(element)) {
                Some(index) => vec![Fem2dOp::SetElement { index, element: projection.elements[index].clone() }],
                None => vec![Fem2dOp::RemoveElement { id: element_id(element).to_string() }],
            },
            Fem2dOp::RemoveElement { id } => index_of(&projection.elements, id)
                .map(|index| vec![Fem2dOp::SetElement { index, element: projection.elements[index].clone() }])
                .unwrap_or_default(),
            Fem2dOp::SetMaterial { material, .. } => match index_of(&projection.materials, &material.id) {
                Some(index) => vec![Fem2dOp::SetMaterial { index, material: projection.materials[index].clone() }],
                None => vec![Fem2dOp::RemoveMaterial { id: material.id.clone() }],
            },
            Fem2dOp::RemoveMaterial { id } => index_of(&projection.materials, id)
                .map(|index| vec![Fem2dOp::SetMaterial { index, material: projection.materials[index].clone() }])
                .unwrap_or_default(),
            Fem2dOp::SetSection { section, .. } => match index_of(&projection.sections, &section.id) {
                Some(index) => vec![Fem2dOp::SetSection { index, section: projection.sections[index].clone() }],
                None => vec![Fem2dOp::RemoveSection { id: section.id.clone() }],
            },
            Fem2dOp::RemoveSection { id } => index_of(&projection.sections, id)
                .map(|index| vec![Fem2dOp::SetSection { index, section: projection.sections[index].clone() }])
                .unwrap_or_default(),
            Fem2dOp::SetSupport { support, .. } => match index_of(&projection.supports, &support.id) {
                Some(index) => vec![Fem2dOp::SetSupport { index, support: projection.supports[index].clone() }],
                None => vec![Fem2dOp::RemoveSupport { id: support.id.clone() }],
            },
            Fem2dOp::RemoveSupport { id } => index_of(&projection.supports, id)
                .map(|index| vec![Fem2dOp::SetSupport { index, support: projection.supports[index].clone() }])
                .unwrap_or_default(),
            Fem2dOp::SetLoadCase { load_case, .. } => match index_of(&projection.load_cases, &load_case.id) {
                Some(index) => vec![Fem2dOp::SetLoadCase { index, load_case: projection.load_cases[index].clone() }],
                None => vec![Fem2dOp::RemoveLoadCase { id: load_case.id.clone() }],
            },
            Fem2dOp::RemoveLoadCase { id } => index_of(&projection.load_cases, id)
                .map(|index| vec![Fem2dOp::SetLoadCase { index, load_case: projection.load_cases[index].clone() }])
                .unwrap_or_default(),
            Fem2dOp::SetCamera { .. } => vec![Fem2dOp::SetCamera { camera: projection.camera.clone() }],
        }
    }
}

pub type Fem2dEnvelope = DocumentVcsEnvelope<Fem2dDocument, Fem2dOp>;
pub type Fem2dStore = DocumentVcsStore<Fem2dDocument, Fem2dOp>;

pub fn empty_fem2d_projection() -> Fem2dDocument {
    Fem2dDocument::default()
}
// #endregion 🔖Ops

// #region 🔖Bridge
/// 🌉 Resolves a `Fem2dDocument` plus a named load case into a `fem_core::Model`, erroring
/// descriptively on any dangling material/section/node reference.
pub fn build_model(doc: &Fem2dDocument, case_id: &str) -> Result<Model, String> {
    let load_case =
        doc.load_cases.iter().find(|lc| lc.id == case_id).ok_or_else(|| format!("load case not found: {case_id}"))?;

    let node_exists = |id: &str| doc.nodes.iter().any(|n| n.id == id);
    let nodes: Vec<Node> = doc.nodes.iter().map(|n| Node { id: n.id.clone(), pos: [n.x, n.y, 0.0] }).collect();

    let mut elements: Vec<Box<dyn Element>> = Vec::with_capacity(doc.elements.len());
    for element in &doc.elements {
        let (id, start, end, material_id, section_id) = match element {
            FemElement::Bar { id, start, end, material_id, section_id } => (id, start, end, material_id, section_id),
            FemElement::Beam { id, start, end, material_id, section_id } => (id, start, end, material_id, section_id),
        };
        if !node_exists(start) {
            return Err(format!("unknown node id: {start}"));
        }
        if !node_exists(end) {
            return Err(format!("unknown node id: {end}"));
        }
        let material =
            doc.materials.iter().find(|m| &m.id == material_id).ok_or_else(|| format!("unknown material id: {material_id}"))?;
        let section =
            doc.sections.iter().find(|s| &s.id == section_id).ok_or_else(|| format!("unknown section id: {section_id}"))?;
        match element {
            FemElement::Bar { .. } => {
                elements.push(Box::new(Bar2 { id: id.clone(), start: start.clone(), end: end.clone(), e: material.e, area: section.area }));
            }
            FemElement::Beam { .. } => {
                elements.push(Box::new(BeamEb2 {
                    id: id.clone(),
                    start: start.clone(),
                    end: end.clone(),
                    e: material.e,
                    area: section.area,
                    iy: section.iy,
                }));
            }
        }
    }

    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.clone() }).collect();

    let mut nodal_loads = Vec::new();
    let mut member_loads = Vec::new();
    for load in &load_case.loads {
        match load {
            FemLoad::Nodal { node_id, dof, value, .. } => nodal_loads.push(NodalLoad { node_id: node_id.clone(), dof: *dof, value: *value }),
            FemLoad::MemberUdl { element_id, wx, wy, .. } => {
                member_loads.push((element_id.clone(), MemberUdl { wx: *wx, wy: *wy, wz: 0.0 }))
            }
        }
    }

    Ok(Model { nodes, elements, supports, nodal_loads, member_loads })
}

/// 🌉 Frozen public entry point: solves a `Fem2dDocument`'s named load case for linear-static
/// equilibrium. Signature is a contract consumed directly by `fem-plugin` — do not rename or
/// change it.
pub fn fem2d_solve(doc: &Fem2dDocument, case_id: &str) -> Result<fem_core::StaticResult, String> {
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
    pub struct Fem2dDocumentVcs {
        store: RefCell<Fem2dStore>,
    }

    #[wasm_bindgen]
    impl Fem2dDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<Fem2dDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Fem2dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Fem2dStore::new(envelope)
                }
                None => Fem2dStore::new(create_document_vcs_envelope(FEM_2D_SCHEMA, "fem2d", empty_fem2d_projection(), None)),
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
    fn simply_supported_beam_doc() -> Fem2dDocument {
        Fem2dDocument {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0 }, FemNode { id: "n2".into(), x: 6.0, y: 0.0 }],
            elements: vec![FemElement::Beam {
                id: "e1".into(),
                start: "n1".into(),
                end: "n2".into(),
                material_id: "steel".into(),
                section_id: "ipe300".into(),
            }],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9 }],
            sections: vec![FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 }],
            supports: vec![
                FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![Dof::Tx, Dof::Ty] },
                FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: vec![Dof::Ty] },
            ],
            load_cases: vec![FemLoadCase {
                id: "dead".into(),
                name: "dead".into(),
                loads: vec![FemLoad::MemberUdl { id: "l1".into(), element_id: "e1".into(), wx: 0.0, wy: -10000.0 }],
            }],
            camera: FemCamera::default(),
        }
    }

    fn simply_supported_beam_two_span_doc() -> Fem2dDocument {
        Fem2dDocument {
            nodes: vec![
                FemNode { id: "n1".into(), x: 0.0, y: 0.0 },
                FemNode { id: "n2".into(), x: 3.0, y: 0.0 },
                FemNode { id: "n3".into(), x: 6.0, y: 0.0 },
            ],
            elements: vec![
                FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() },
                FemElement::Beam { id: "e2".into(), start: "n2".into(), end: "n3".into(), material_id: "steel".into(), section_id: "ipe300".into() },
            ],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9 }],
            sections: vec![FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 }],
            supports: vec![
                FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![Dof::Tx, Dof::Ty] },
                FemSupport { id: "s2".into(), node_id: "n3".into(), fixed: vec![Dof::Ty] },
            ],
            load_cases: vec![FemLoadCase {
                id: "dead".into(),
                name: "dead".into(),
                loads: vec![
                    FemLoad::MemberUdl { id: "l1".into(), element_id: "e1".into(), wx: 0.0, wy: -10000.0 },
                    FemLoad::MemberUdl { id: "l2".into(), element_id: "e2".into(), wx: 0.0, wy: -10000.0 },
                ],
            }],
            camera: FemCamera::default(),
        }
    }

    fn truss_doc() -> Fem2dDocument {
        Fem2dDocument {
            nodes: vec![
                FemNode { id: "n1".into(), x: 0.0, y: 0.0 },
                FemNode { id: "n2".into(), x: 4.0, y: 0.0 },
                FemNode { id: "n3".into(), x: 4.0, y: 3.0 },
            ],
            elements: vec![
                FemElement::Bar { id: "e1".into(), start: "n1".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
                FemElement::Bar { id: "e2".into(), start: "n2".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
            ],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9 }],
            sections: vec![FemSection { id: "rod".into(), name: "rod".into(), area: 0.001, iy: 0.0 }],
            supports: vec![
                FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![Dof::Tx, Dof::Ty] },
                FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: vec![Dof::Tx, Dof::Ty] },
            ],
            load_cases: vec![FemLoadCase {
                id: "dead".into(),
                name: "dead".into(),
                loads: vec![
                    FemLoad::Nodal { id: "l1".into(), node_id: "n3".into(), dof: Dof::Ty, value: -1000.0 },
                    FemLoad::Nodal { id: "l2".into(), node_id: "n3".into(), dof: Dof::Tx, value: -500.0 },
                ],
            }],
            camera: FemCamera::default(),
        }
    }
    // #endregion 🔖Fixtures

    // #region 🔖OpRoundTrip
    fn round_trip(projection: &Fem2dDocument, op: &Fem2dOp) -> Fem2dDocument {
        let forward = apply_operation(projection, op);
        let mut restored = forward.clone();
        for back in op.backwards(projection) {
            restored = apply_operation(&restored, &back);
        }
        assert_eq!(&restored, projection, "backwards() must restore the pre-op document");
        forward
    }

    #[test]
    fn node_op_round_trips() {
        let base = simply_supported_beam_doc();
        let after = round_trip(&base, &Fem2dOp::SetNode { index: 0, node: FemNode { id: "n1".into(), x: 1.0, y: 1.0 } });
        assert_eq!(after.nodes[0].x, 1.0);
        round_trip(&base, &Fem2dOp::RemoveNode { id: "n1".into() });
    }

    #[test]
    fn element_op_round_trips() {
        let base = simply_supported_beam_doc();
        let updated = FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() };
        round_trip(&base, &Fem2dOp::SetElement { index: 0, element: updated });
        round_trip(&base, &Fem2dOp::RemoveElement { id: "e1".into() });
    }

    #[test]
    fn material_op_round_trips() {
        let base = simply_supported_beam_doc();
        round_trip(&base, &Fem2dOp::SetMaterial { index: 0, material: FemMaterial { id: "steel".into(), name: "steel".into(), e: 200e9 } });
        round_trip(&base, &Fem2dOp::RemoveMaterial { id: "steel".into() });
    }

    #[test]
    fn section_op_round_trips() {
        let base = simply_supported_beam_doc();
        round_trip(&base, &Fem2dOp::SetSection { index: 0, section: FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.01, iy: 1e-4 } });
        round_trip(&base, &Fem2dOp::RemoveSection { id: "ipe300".into() });
    }

    #[test]
    fn support_op_round_trips() {
        let base = simply_supported_beam_doc();
        round_trip(&base, &Fem2dOp::SetSupport { index: 0, support: FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![Dof::Ty] } });
        round_trip(&base, &Fem2dOp::RemoveSupport { id: "s1".into() });
    }

    #[test]
    fn load_case_op_round_trips() {
        let base = simply_supported_beam_doc();
        round_trip(&base, &Fem2dOp::SetLoadCase { index: 0, load_case: FemLoadCase { id: "dead".into(), name: "dead 2".into(), loads: vec![] } });
        round_trip(&base, &Fem2dOp::RemoveLoadCase { id: "dead".into() });
    }

    #[test]
    fn camera_op_round_trips() {
        let base = simply_supported_beam_doc();
        let after = round_trip(&base, &Fem2dOp::SetCamera { camera: FemCamera { x: 7.0, y: 8.0, zoom: 2.0 } });
        assert_eq!(after.camera.zoom, 2.0);
    }
    // #endregion 🔖OpRoundTrip

    // #region 🔖BuildModel
    #[test]
    fn build_model_reports_dangling_material() {
        let mut doc = simply_supported_beam_doc();
        doc.materials.clear();
        let err = build_model(&doc, "dead").unwrap_err();
        assert!(err.contains("material"), "unexpected error: {err}");
    }

    #[test]
    fn build_model_reports_dangling_section() {
        let mut doc = simply_supported_beam_doc();
        doc.sections.clear();
        let err = build_model(&doc, "dead").unwrap_err();
        assert!(err.contains("section"), "unexpected error: {err}");
    }

    #[test]
    fn build_model_reports_dangling_node() {
        let mut doc = simply_supported_beam_doc();
        doc.nodes.clear();
        let err = build_model(&doc, "dead").unwrap_err();
        assert!(err.contains("node"), "unexpected error: {err}");
    }
    // #endregion 🔖BuildModel

    // #region 🔖AnalyticalBenchmark
    #[test]
    fn simply_supported_beam_matches_analytical_udl_solution() {
        let doc = simply_supported_beam_doc();
        let result = fem2d_solve(&doc, "dead").expect("solves");

        for reaction in result.reactions.iter().filter(|r| r.dof == Dof::Tx) {
            assert!(reaction.value.abs() < 1e-6, "horizontal reaction {reaction:?} should be ~0 (no horizontal load)");
        }
        for reaction in result.reactions.iter().filter(|r| r.dof == Dof::Ty) {
            assert!((reaction.value - 30000.0).abs() < 1.0, "vertical reaction {reaction:?} not near 30000N");
        }

        let (_, ElementResult::Beam { stations }) = &result.elements[0] else { panic!("expected beam result") };
        let midspan = stations.iter().min_by(|a, b| (a.x - 3.0).abs().partial_cmp(&(b.x - 3.0).abs()).unwrap()).unwrap();
        assert!((midspan.m - 45000.0).abs() / 45000.0 < 0.01, "midspan moment {} not near 45000", midspan.m);
    }

    #[test]
    fn two_span_beam_matches_analytical_midspan_deflection_and_moment() {
        let doc = simply_supported_beam_two_span_doc();
        let result = fem2d_solve(&doc, "dead").expect("solves");

        for reaction in result.reactions.iter().filter(|r| r.dof == Dof::Tx) {
            assert!(reaction.value.abs() < 1e-6, "horizontal reaction {reaction:?} should be ~0 (no horizontal load)");
        }
        for reaction in result.reactions.iter().filter(|r| r.dof == Dof::Ty) {
            assert!((reaction.value - 30000.0).abs() < 1.0, "vertical reaction {reaction:?} not near 30000N");
        }

        let midspan_disp = result.displacements.iter().find(|d| d.node_id == "n2").unwrap();
        let expected = -0.009617;
        assert!((midspan_disp.values[Dof::Ty.index()] - expected).abs() / expected.abs() < 0.02, "midspan deflection {} not near {expected}", midspan_disp.values[Dof::Ty.index()]);

        let (_, ElementResult::Beam { stations }) = &result.elements[0] else { panic!("expected beam result") };
        let end_moment = stations.last().unwrap();
        assert!((end_moment.m - 45000.0).abs() / 45000.0 < 0.01, "end moment at midspan node {} not near 45000", end_moment.m);
    }
    // #endregion 🔖AnalyticalBenchmark

    // #region 🔖Truss
    #[test]
    fn truss_is_in_equilibrium_with_finite_bar_forces() {
        let doc = truss_doc();
        let result = fem2d_solve(&doc, "dead").expect("solves");

        assert!(result.checks.reaction_sum[Dof::Tx.index()].abs() < 1e-6);
        assert!((result.checks.reaction_sum[Dof::Ty.index()]).abs() < 1e-6);

        for (_, element_result) in &result.elements {
            let ElementResult::Bar { n } = element_result else { panic!("expected bar result") };
            assert!(n.is_finite() && *n != 0.0, "bar force {n} should be finite and nonzero");
        }
    }
    // #endregion 🔖Truss

    // #region 🔖UnknownCase
    #[test]
    fn unknown_load_case_returns_descriptive_error() {
        let doc = simply_supported_beam_doc();
        let err = fem2d_solve(&doc, "missing").unwrap_err();
        assert!(err.contains("load case not found"), "unexpected error: {err}");
    }
    // #endregion 🔖UnknownCase

    // #region 🔖ExampleFixture
    #[test]
    fn example_fixture_parses_and_solves() {
        let json = include_str!("../example/default.fem2d.json");
        let doc: Fem2dDocument = serde_json::from_str(json).expect("example fixture parses");
        assert_eq!(doc.nodes.len(), 2);
        assert_eq!(doc.elements.len(), 1);
        let result = fem2d_solve(&doc, "dead").expect("example fixture solves");
        assert!(result.checks.residual_norm < 1e-6);
    }
    // #endregion 🔖ExampleFixture
}
// #endregion 🔖Tests
