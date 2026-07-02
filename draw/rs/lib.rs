//! ✏️ Draw document domain + typed VCS on `vcs`.

use vcs::{
    create_document_vcs_envelope, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff,
};
use serde::{Deserialize, Serialize};

pub const DRAW_DOCUMENT_SCHEMA: &str = "draw.document";

//#region 🔖Domain
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawTransform {
    pub x: f64,
    pub y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub rotation: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawLayerBase {
    pub id: String,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f64,
    pub blend_mode: String,
    pub transform: DrawTransform,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawShapeLayer {
    #[serde(flatten)]
    pub base: DrawLayerBase,
    pub kind: String,
    pub shape_kind: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawGroupLayer {
    #[serde(flatten)]
    pub base: DrawLayerBase,
    pub kind: String,
    pub children: Vec<DrawLayerNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DrawLayerNode {
    Shape(DrawShapeLayer),
    Group(DrawGroupLayer),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawDocument {
    pub schema: String,
    pub id: String,
    pub version: String,
    pub layers: Vec<DrawLayerNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_tool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera: Option<DrawCamera>,
}

pub type DrawEnvelope = DocumentVcsEnvelope<DrawDocument, DrawOp>;
pub type DrawStore = DocumentVcsStore<DrawDocument, DrawOp>;

pub fn default_draw_transform() -> DrawTransform {
    DrawTransform {
        x: 0.0,
        y: 0.0,
        scale_x: 1.0,
        scale_y: 1.0,
        rotation: 0.0,
    }
}

pub fn empty_draw_projection() -> DrawDocument {
    DrawDocument {
        schema: DRAW_DOCUMENT_SCHEMA.into(),
        id: "draw".into(),
        version: "1".into(),
        layers: Vec::new(),
        active_tool: Some("selectMarquee".into()),
        camera: Some(DrawCamera {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
        }),
    }
}
//#endregion 🔖Domain

//#region 🔖Ops
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum DrawOp {
    SetLayerVisible {
        layer_id: String,
        visible: bool,
    },
    SetLayerName {
        layer_id: String,
        name: String,
    },
    SetLayerOpacity {
        layer_id: String,
        opacity: f64,
    },
    SetActiveTool {
        tool: String,
    },
    SetCamera {
        camera: DrawCamera,
    },
    AddShapeLayer {
        #[serde(skip_serializing_if = "Option::is_none")]
        parent_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<usize>,
        layer: DrawShapeLayer,
    },
    RemoveLayer {
        layer_id: String,
    },
    SetDocument {
        document: DrawDocument,
    },
}

fn update_layer_in_tree(layers: &mut [DrawLayerNode], layer_id: &str, mutator: &mut impl FnMut(&mut DrawLayerNode)) -> bool {
    for layer in layers.iter_mut() {
        match layer {
            DrawLayerNode::Shape(shape) if shape.base.id == layer_id => {
                mutator(layer);
                return true;
            }
            DrawLayerNode::Group(group) if group.base.id == layer_id => {
                mutator(layer);
                return true;
            }
            DrawLayerNode::Group(group) => {
                if update_layer_in_tree(&mut group.children, layer_id, mutator) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn remove_layer_from_tree(layers: &mut Vec<DrawLayerNode>, layer_id: &str) -> bool {
    if let Some(index) = layers.iter().position(|layer| layer_id_matches(layer, layer_id)) {
        layers.remove(index);
        return true;
    }
    for layer in layers.iter_mut() {
        if let DrawLayerNode::Group(group) = layer {
            if remove_layer_from_tree(&mut group.children, layer_id) {
                return true;
            }
        }
    }
    false
}

fn layer_id_matches(layer: &DrawLayerNode, layer_id: &str) -> bool {
    match layer {
        DrawLayerNode::Shape(shape) => shape.base.id == layer_id,
        DrawLayerNode::Group(group) => group.base.id == layer_id,
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawLayerBasePatch {
    pub visible: Option<bool>,
    pub name: Option<String>,
    pub opacity: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawLayerTreePatch {
    pub layer_id: String,
    pub base: DrawLayerBasePatch,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawLayerTreeAdd {
    pub parent_id: Option<String>,
    pub index: Option<usize>,
    pub layer: DrawLayerNode,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawDiff {
    pub document: Option<DrawDocument>,
    pub active_tool: Option<Option<String>>,
    pub camera: Option<DrawCamera>,
    pub layer_patches: Vec<DrawLayerTreePatch>,
    pub layers_removed: Vec<String>,
    pub layers_added: Vec<DrawLayerTreeAdd>,
}

impl Default for DrawDiff {
    fn default() -> Self {
        Self {
            document: None,
            active_tool: None,
            camera: None,
            layer_patches: Vec::new(),
            layers_removed: Vec::new(),
            layers_added: Vec::new(),
        }
    }
}

impl OperationDiff<DrawDocument> for DrawDiff {
    fn apply(&self, projection: &DrawDocument) -> DrawDocument {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        if let Some(tool) = &self.active_tool {
            next.active_tool = tool.clone();
        }
        if let Some(camera) = &self.camera {
            next.camera = Some(camera.clone());
        }
        for patch in &self.layer_patches {
            let mut set = |layer: &mut DrawLayerNode| match layer {
                DrawLayerNode::Shape(shape) => {
                    if let Some(visible) = patch.base.visible {
                        shape.base.visible = visible;
                    }
                    if let Some(name) = &patch.base.name {
                        shape.base.name = name.clone();
                    }
                    if let Some(opacity) = patch.base.opacity {
                        shape.base.opacity = opacity;
                    }
                }
                DrawLayerNode::Group(group) => {
                    if let Some(visible) = patch.base.visible {
                        group.base.visible = visible;
                    }
                    if let Some(name) = &patch.base.name {
                        group.base.name = name.clone();
                    }
                    if let Some(opacity) = patch.base.opacity {
                        group.base.opacity = opacity;
                    }
                }
            };
            update_layer_in_tree(&mut next.layers, &patch.layer_id, &mut set);
        }
        for layer_id in &self.layers_removed {
            remove_layer_from_tree(&mut next.layers, layer_id);
        }
        for add in &self.layers_added {
            if let Some(parent_id) = &add.parent_id {
                insert_layer_in_parent(&mut next.layers, parent_id, add.index.unwrap_or(0), add.layer.clone());
            } else {
                let at = add.index.unwrap_or(next.layers.len());
                next.layers.insert(at.min(next.layers.len()), add.layer.clone());
            }
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.document.is_some() {
            *self = other;
            return;
        }
        if other.active_tool.is_some() {
            self.active_tool = other.active_tool;
        }
        if other.camera.is_some() {
            self.camera = other.camera;
        }
        self.layer_patches.extend(other.layer_patches);
        self.layers_removed.extend(other.layers_removed);
        self.layers_added.extend(other.layers_added);
    }
}

fn find_layer_base<'a>(layers: &'a [DrawLayerNode], layer_id: &str) -> Option<&'a DrawLayerBase> {
    for layer in layers {
        match layer {
            DrawLayerNode::Shape(shape) if shape.base.id == layer_id => return Some(&shape.base),
            DrawLayerNode::Group(group) if group.base.id == layer_id => return Some(&group.base),
            DrawLayerNode::Group(group) => {
                if let Some(base) = find_layer_base(&group.children, layer_id) {
                    return Some(base);
                }
            }
            _ => {}
        }
    }
    None
}

fn extract_layer_node(layers: &mut Vec<DrawLayerNode>, layer_id: &str) -> Option<DrawLayerNode> {
    if let Some(index) = layers.iter().position(|layer| layer_id_matches(layer, layer_id)) {
        return Some(layers.remove(index));
    }
    for layer in layers.iter_mut() {
        if let DrawLayerNode::Group(group) = layer {
            if let Some(node) = extract_layer_node(&mut group.children, layer_id) {
                return Some(node);
            }
        }
    }
    None
}

impl Operation<DrawDocument> for DrawOp {
    type Diff = DrawDiff;

    fn diff(&self, _projection: &DrawDocument) -> DrawDiff {
        match self {
            DrawOp::SetLayerVisible { layer_id, visible } => DrawDiff {
                layer_patches: vec![DrawLayerTreePatch {
                    layer_id: layer_id.clone(),
                    base: DrawLayerBasePatch {
                        visible: Some(*visible),
                        ..Default::default()
                    },
                }],
                ..Default::default()
            },
            DrawOp::SetLayerName { layer_id, name } => DrawDiff {
                layer_patches: vec![DrawLayerTreePatch {
                    layer_id: layer_id.clone(),
                    base: DrawLayerBasePatch {
                        name: Some(name.clone()),
                        ..Default::default()
                    },
                }],
                ..Default::default()
            },
            DrawOp::SetLayerOpacity { layer_id, opacity } => DrawDiff {
                layer_patches: vec![DrawLayerTreePatch {
                    layer_id: layer_id.clone(),
                    base: DrawLayerBasePatch {
                        opacity: Some(*opacity),
                        ..Default::default()
                    },
                }],
                ..Default::default()
            },
            DrawOp::SetActiveTool { tool } => DrawDiff {
                active_tool: Some(Some(tool.clone())),
                ..Default::default()
            },
            DrawOp::SetCamera { camera } => DrawDiff {
                camera: Some(camera.clone()),
                ..Default::default()
            },
            DrawOp::AddShapeLayer { parent_id, index, layer } => DrawDiff {
                layers_added: vec![DrawLayerTreeAdd {
                    parent_id: parent_id.clone(),
                    index: *index,
                    layer: DrawLayerNode::Shape(layer.clone()),
                }],
                ..Default::default()
            },
            DrawOp::RemoveLayer { layer_id } => DrawDiff {
                layers_removed: vec![layer_id.clone()],
                ..Default::default()
            },
            DrawOp::SetDocument { document } => DrawDiff {
                document: Some(document.clone()),
                ..Default::default()
            },
        }
    }

    fn backwards(&self, projection: &DrawDocument) -> Vec<Self> {
        match self {
            DrawOp::SetLayerVisible { layer_id, .. } => find_layer_base(&projection.layers, layer_id)
                .map(|base| {
                    vec![DrawOp::SetLayerVisible {
                        layer_id: layer_id.clone(),
                        visible: base.visible,
                    }]
                })
                .unwrap_or_default(),
            DrawOp::SetLayerName { layer_id, .. } => find_layer_base(&projection.layers, layer_id)
                .map(|base| {
                    vec![DrawOp::SetLayerName {
                        layer_id: layer_id.clone(),
                        name: base.name.clone(),
                    }]
                })
                .unwrap_or_default(),
            DrawOp::SetLayerOpacity { layer_id, .. } => find_layer_base(&projection.layers, layer_id)
                .map(|base| {
                    vec![DrawOp::SetLayerOpacity {
                        layer_id: layer_id.clone(),
                        opacity: base.opacity,
                    }]
                })
                .unwrap_or_default(),
            DrawOp::SetActiveTool { .. } => vec![DrawOp::SetActiveTool {
                tool: projection.active_tool.clone().unwrap_or_default(),
            }],
            DrawOp::SetCamera { .. } => vec![DrawOp::SetCamera {
                camera: projection.camera.clone().unwrap_or(DrawCamera {
                    x: 0.0,
                    y: 0.0,
                    zoom: 1.0,
                }),
            }],
            DrawOp::AddShapeLayer { layer, parent_id, index, .. } => vec![DrawOp::RemoveLayer {
                layer_id: layer.base.id.clone(),
            }],
            DrawOp::RemoveLayer { layer_id } => {
                let mut layers = projection.layers.clone();
                extract_layer_node(&mut layers, layer_id)
                    .map(|node| {
                        if let DrawLayerNode::Shape(shape) = node {
                            vec![DrawOp::AddShapeLayer {
                                parent_id: None,
                                index: None,
                                layer: shape,
                            }]
                        } else {
                            Vec::new()
                        }
                    })
                    .unwrap_or_default()
            }
            DrawOp::SetDocument { .. } => vec![DrawOp::SetDocument {
                document: projection.clone(),
            }],
        }
    }
}
//#endregion 🔖Ops

fn insert_layer_in_parent(layers: &mut [DrawLayerNode], parent_id: &str, index: usize, node: DrawLayerNode) -> bool {
    for layer in layers.iter_mut() {
        if let DrawLayerNode::Group(group) = layer {
            if group.base.id == parent_id {
                let at = index.min(group.children.len());
                group.children.insert(at, node);
                return true;
            }
            if insert_layer_in_parent(&mut group.children, parent_id, index, node.clone()) {
                return true;
            }
        }
    }
    false
}

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct DrawDocumentVcs {
        store: RefCell<DrawStore>,
    }

    #[wasm_bindgen]
    impl DrawDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<DrawDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: DrawEnvelope =
                        serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    DrawStore::new(envelope)
                }
                None => DrawStore::new(create_document_vcs_envelope(DRAW_DOCUMENT_SCHEMA, "draw", empty_draw_projection(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store
                .borrow_mut()
                .dispatch_json(command_json)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store
                .borrow()
                .projection_json()
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store
                .borrow()
                .envelope_json()
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
}
//#endregion 🔖WasmBridge

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_document_vcs_materializes() {
        let mut store = DrawStore::new(create_document_vcs_envelope(DRAW_DOCUMENT_SCHEMA, "draw", empty_draw_projection(), None));
        let layer = DrawShapeLayer {
            base: DrawLayerBase {
                id: "layer-1".into(),
                name: "Shape".into(),
                visible: true,
                locked: false,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: default_draw_transform(),
            },
            kind: "shape".into(),
            shape_kind: "rect".into(),
        };
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DrawOp::AddShapeLayer {
                    parent_id: None,
                    index: None,
                    layer: layer.clone(),
                }],
                description: None,
            })
            .expect("apply");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.layers.len(), 1);
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DrawOp::SetLayerName {
                    layer_id: "layer-1".into(),
                    name: "Renamed".into(),
                }],
                description: None,
            })
            .expect("rename");
        if let DrawLayerNode::Shape(shape) = &store.projection().expect("projection").layers[0] {
            assert_eq!(shape.base.name, "Renamed");
        } else {
            panic!("expected shape layer");
        }
    }
}
//#endregion 🧪Tests
