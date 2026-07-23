//! 🖼️ Raster plugin — declarative raster board bundled as a hot-swappable WASM component.

//#region 🔖Domain
/// 🖼️ Raster's own document/VCS domain model (layer tree, ops, diffs) kept app-owned while
/// `RasterHost`/`RasterSession` (the generic paint-canvas hosting mechanism) live in
/// `framework_surface_paint`.
pub(crate) mod domain {
    use std::collections::HashMap;

    //#region 🔖DocumentVcs
    use vcs::{Operation, OperationDiff};

    pub const RASTER_DOCUMENT_SCHEMA: &str = "raster.document";

    fn default_one() -> f64 {
        1.0
    }

    fn default_true() -> bool {
        true
    }
    
    //#region 🔖Projection
    /// 🎞️ Non-destructive raster document: a nested layer tree (pixel/group/adjustment) over a pannable
    /// camera, plus embedded image assets. This is the authoritative projection shared by the wasm
    /// compositor bridge and the `raster-plugin` `DocumentApp`. Ephemeral tool/brush/selection state
    /// lives in the plugin's app struct, never here.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RasterCamera {
        #[serde(default)]
        pub x: f64,
        #[serde(default)]
        pub y: f64,
        #[serde(default = "default_one")]
        pub zoom: f64,
    }
    
    impl Default for RasterCamera {
        fn default() -> Self {
            Self { x: 0.0, y: 0.0, zoom: 1.0 }
        }
    }
    
    fn one_f32() -> f32 {
        1.0
    }
    
    fn default_blend() -> String {
        "normal".into()
    }
    
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RasterTransform {
        #[serde(default)]
        pub x: f64,
        #[serde(default)]
        pub y: f64,
        #[serde(default = "default_one")]
        pub scale_x: f64,
        #[serde(default = "default_one")]
        pub scale_y: f64,
        #[serde(default)]
        pub rotation: f64,
    }
    
    impl Default for RasterTransform {
        fn default() -> Self {
            Self { x: 0.0, y: 0.0, scale_x: 1.0, scale_y: 1.0, rotation: 0.0 }
        }
    }
    
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RasterLayerMask {
        #[serde(default = "default_true")]
        pub enabled: bool,
        #[serde(default = "default_true")]
        pub linked: bool,
        #[serde(default)]
        pub invert: bool,
        pub width: Option<u32>,
        pub height: Option<u32>,
    }
    
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase")]
    pub enum RasterLayerNode {
        #[serde(rename = "pixel", rename_all = "camelCase")]
        Pixel {
            id: String,
            name: String,
            #[serde(default = "default_true")]
            visible: bool,
            #[serde(default = "one_f32")]
            opacity: f32,
            #[serde(default = "default_blend")]
            blend_mode: String,
            #[serde(default)]
            transform: RasterTransform,
            mask: Option<RasterLayerMask>,
            width: Option<u32>,
            height: Option<u32>,
            image_key: Option<String>,
        },
        #[serde(rename = "group", rename_all = "camelCase")]
        Group {
            id: String,
            name: String,
            #[serde(default = "default_true")]
            visible: bool,
            #[serde(default = "one_f32")]
            opacity: f32,
            #[serde(default = "default_blend")]
            blend_mode: String,
            #[serde(default)]
            transform: RasterTransform,
            mask: Option<RasterLayerMask>,
            children: Vec<RasterLayerNode>,
        },
        #[serde(rename = "adjustment", rename_all = "camelCase")]
        Adjustment {
            id: String,
            name: String,
            #[serde(default = "default_true")]
            visible: bool,
            #[serde(default = "one_f32")]
            opacity: f32,
            #[serde(default = "default_blend")]
            blend_mode: String,
            #[serde(default)]
            transform: RasterTransform,
            adjustment_kind: String,
        },
    }
    
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RasterImageAsset {
        pub mime: String,
        pub data: String,
    }
    
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RasterProjection {
        pub schema: String,
        pub id: String,
        #[serde(default)]
        pub title: Option<String>,
        #[serde(default)]
        pub camera: RasterCamera,
        #[serde(default)]
        pub layers: Vec<RasterLayerNode>,
        #[serde(default)]
        pub assets: HashMap<String, RasterImageAsset>,
    }
    
    pub fn empty_raster_projection() -> RasterProjection {
        RasterProjection {
            schema: RASTER_DOCUMENT_SCHEMA.into(),
            id: "raster".into(),
            title: Some("Untitled".into()),
            camera: RasterCamera::default(),
            layers: Vec::new(),
            assets: HashMap::new(),
        }
    }
    //#endregion 🔖Projection
    
    //#region 🔖Tree
    pub fn layer_node_id(layer: &RasterLayerNode) -> &str {
        match layer {
            RasterLayerNode::Pixel { id, .. } | RasterLayerNode::Group { id, .. } | RasterLayerNode::Adjustment { id, .. } => id,
        }
    }
    
    pub fn layer_name(layer: &RasterLayerNode) -> &str {
        match layer {
            RasterLayerNode::Pixel { name, .. } | RasterLayerNode::Group { name, .. } | RasterLayerNode::Adjustment { name, .. } => name,
        }
    }
    
    pub fn layer_visible(layer: &RasterLayerNode) -> bool {
        match layer {
            RasterLayerNode::Pixel { visible, .. } | RasterLayerNode::Group { visible, .. } | RasterLayerNode::Adjustment { visible, .. } => *visible,
        }
    }
    
    pub fn layer_opacity(layer: &RasterLayerNode) -> f32 {
        match layer {
            RasterLayerNode::Pixel { opacity, .. } | RasterLayerNode::Group { opacity, .. } | RasterLayerNode::Adjustment { opacity, .. } => *opacity,
        }
    }
    
    pub fn find_layer<'a>(layers: &'a [RasterLayerNode], target_id: &str) -> Option<&'a RasterLayerNode> {
        for layer in layers {
            if layer_node_id(layer) == target_id {
                return Some(layer);
            }
            if let RasterLayerNode::Group { children, .. } = layer {
                if let Some(found) = find_layer(children, target_id) {
                    return Some(found);
                }
            }
        }
        None
    }
    
    /// 🧭 Finds a layer's parent-group id (`None` at the root) and its index among its siblings.
    pub fn locate_layer(layers: &[RasterLayerNode], target_id: &str) -> Option<(Option<String>, usize)> {
        fn walk(layers: &[RasterLayerNode], parent: Option<&str>, target_id: &str) -> Option<(Option<String>, usize)> {
            for (index, layer) in layers.iter().enumerate() {
                if layer_node_id(layer) == target_id {
                    return Some((parent.map(str::to_string), index));
                }
                if let RasterLayerNode::Group { id, children, .. } = layer {
                    if let Some(found) = walk(children, Some(id), target_id) {
                        return Some(found);
                    }
                }
            }
            None
        }
        walk(layers, None, target_id)
    }
    
    pub fn flatten_raster_layers(layers: &[RasterLayerNode]) -> Vec<&RasterLayerNode> {
        let mut out = Vec::new();
        fn visit<'a>(layers: &'a [RasterLayerNode], out: &mut Vec<&'a RasterLayerNode>) {
            for layer in layers {
                out.push(layer);
                if let RasterLayerNode::Group { children, .. } = layer {
                    visit(children, out);
                }
            }
        }
        visit(layers, &mut out);
        out
    }
    
    fn remove_layer_from_tree(layers: &mut Vec<RasterLayerNode>, target_id: &str) -> Option<RasterLayerNode> {
        if let Some(index) = layers.iter().position(|layer| layer_node_id(layer) == target_id) {
            return Some(layers.remove(index));
        }
        for layer in layers.iter_mut() {
            if let RasterLayerNode::Group { children, .. } = layer {
                if let Some(removed) = remove_layer_from_tree(children, target_id) {
                    return Some(removed);
                }
            }
        }
        None
    }
    
    fn insert_layer(layers: &mut Vec<RasterLayerNode>, parent_id: Option<&str>, index: usize, layer: RasterLayerNode) {
        match parent_id {
            None => {
                let at = index.min(layers.len());
                layers.insert(at, layer);
            }
            Some(parent_id) => {
                for node in layers.iter_mut() {
                    if let RasterLayerNode::Group { id, children, .. } = node {
                        if id == parent_id {
                            let at = index.min(children.len());
                            children.insert(at, layer);
                            return;
                        }
                        insert_layer(children, Some(parent_id), index, layer.clone());
                    }
                }
            }
        }
    }
    
    fn apply_layer_patch(node: &mut RasterLayerNode, patch: &RasterLayerPatch) -> RasterLayerPatch {
        let mut inverse = RasterLayerPatch::default();
        match node {
            RasterLayerNode::Pixel { name, visible, opacity, blend_mode, transform, width, height, .. } => {
                if let Some(value) = &patch.name {
                    inverse.name = Some(name.clone());
                    *name = value.clone();
                }
                if let Some(value) = patch.visible {
                    inverse.visible = Some(*visible);
                    *visible = value;
                }
                if let Some(value) = patch.opacity {
                    inverse.opacity = Some(*opacity);
                    *opacity = value;
                }
                if let Some(value) = &patch.blend_mode {
                    inverse.blend_mode = Some(blend_mode.clone());
                    *blend_mode = value.clone();
                }
                if let Some(value) = patch.transform_x {
                    inverse.transform_x = Some(transform.x);
                    transform.x = value;
                }
                if let Some(value) = patch.transform_y {
                    inverse.transform_y = Some(transform.y);
                    transform.y = value;
                }
                if let Some(value) = patch.width {
                    inverse.width = Some(width.unwrap_or(512));
                    *width = Some(value);
                }
                if let Some(value) = patch.height {
                    inverse.height = Some(height.unwrap_or(512));
                    *height = Some(value);
                }
            }
            RasterLayerNode::Group { name, visible, opacity, blend_mode, transform, .. } => {
                if let Some(value) = &patch.name {
                    inverse.name = Some(name.clone());
                    *name = value.clone();
                }
                if let Some(value) = patch.visible {
                    inverse.visible = Some(*visible);
                    *visible = value;
                }
                if let Some(value) = patch.opacity {
                    inverse.opacity = Some(*opacity);
                    *opacity = value;
                }
                if let Some(value) = &patch.blend_mode {
                    inverse.blend_mode = Some(blend_mode.clone());
                    *blend_mode = value.clone();
                }
                if let Some(value) = patch.transform_x {
                    inverse.transform_x = Some(transform.x);
                    transform.x = value;
                }
                if let Some(value) = patch.transform_y {
                    inverse.transform_y = Some(transform.y);
                    transform.y = value;
                }
            }
            RasterLayerNode::Adjustment { name, visible, opacity, blend_mode, adjustment_kind, .. } => {
                if let Some(value) = &patch.name {
                    inverse.name = Some(name.clone());
                    *name = value.clone();
                }
                if let Some(value) = patch.visible {
                    inverse.visible = Some(*visible);
                    *visible = value;
                }
                if let Some(value) = patch.opacity {
                    inverse.opacity = Some(*opacity);
                    *opacity = value;
                }
                if let Some(value) = &patch.blend_mode {
                    inverse.blend_mode = Some(blend_mode.clone());
                    *blend_mode = value.clone();
                }
                if let Some(value) = &patch.adjustment_kind {
                    inverse.adjustment_kind = Some(adjustment_kind.clone());
                    *adjustment_kind = value.clone();
                }
            }
        }
        inverse
    }
    
    fn patch_layer_in_tree(layers: &mut [RasterLayerNode], target_id: &str, patch: &RasterLayerPatch) -> Option<RasterLayerPatch> {
        for layer in layers.iter_mut() {
            if layer_node_id(layer) == target_id {
                return Some(apply_layer_patch(layer, patch));
            }
            if let RasterLayerNode::Group { children, .. } = layer {
                if let Some(inverse) = patch_layer_in_tree(children, target_id, patch) {
                    return Some(inverse);
                }
            }
        }
        None
    }
    //#endregion 🔖Tree
    
    //#region 🔖Ops
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RasterLayerPatch {
        pub name: Option<String>,
        pub visible: Option<bool>,
        pub opacity: Option<f32>,
        pub blend_mode: Option<String>,
        pub transform_x: Option<f64>,
        pub transform_y: Option<f64>,
        pub width: Option<u32>,
        pub height: Option<u32>,
        pub adjustment_kind: Option<String>,
    }
    
    /// 🧩 One atomic tree mutation — the building block of {@link RasterDiff}, kept ordered so a diff can
    /// coalesce several edits (e.g. a multi-layer patch) while still inverting each mechanically.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(tag = "step", rename_all = "camelCase")]
    pub enum RasterStep {
        AddLayer { parent_id: Option<String>, index: usize, layer: RasterLayerNode },
        RemoveLayer { layer_id: String },
        PatchLayer { layer_id: String, patch: RasterLayerPatch },
        MoveLayer { layer_id: String, parent_id: Option<String>, index: usize },
    }
    
    fn apply_step(layers: &mut Vec<RasterLayerNode>, step: &RasterStep) {
        match step {
            RasterStep::AddLayer { parent_id, index, layer } => insert_layer(layers, parent_id.as_deref(), *index, layer.clone()),
            RasterStep::RemoveLayer { layer_id } => {
                remove_layer_from_tree(layers, layer_id);
            }
            RasterStep::PatchLayer { layer_id, patch } => {
                patch_layer_in_tree(layers, layer_id, patch);
            }
            RasterStep::MoveLayer { layer_id, parent_id, index } => {
                if let Some(node) = remove_layer_from_tree(layers, layer_id) {
                    insert_layer(layers, parent_id.as_deref(), *index, node);
                }
            }
        }
    }
    
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(tag = "op", rename_all = "camelCase")]
    pub enum RasterOp {
        AddLayer { parent_id: Option<String>, index: usize, layer: RasterLayerNode },
        RemoveLayer { layer_id: String },
        PatchLayer { layer_id: String, patch: RasterLayerPatch },
        MoveLayer { layer_id: String, parent_id: Option<String>, index: usize },
        SetCamera { camera: RasterCamera },
        ReplaceDocument { document: RasterProjection },
    }
    
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RasterDiff {
        pub steps: Vec<RasterStep>,
        pub camera: Option<RasterCamera>,
        pub replace: Option<Box<RasterProjection>>,
    }
    
    impl OperationDiff<RasterProjection> for RasterDiff {
        fn apply(&self, projection: &RasterProjection) -> RasterProjection {
            let mut next = self.replace.as_ref().map(|document| (**document).clone()).unwrap_or_else(|| projection.clone());
            for step in &self.steps {
                apply_step(&mut next.layers, step);
            }
            if let Some(camera) = &self.camera {
                next.camera = camera.clone();
            }
            next
        }
    
        fn absorb(&mut self, other: Self) {
            if let Some(replace) = other.replace {
                self.replace = Some(replace);
                self.steps.clear();
            }
            self.steps.extend(other.steps);
            if other.camera.is_some() {
                self.camera = other.camera;
            }
        }
    }
    
    fn step_diff(step: RasterStep) -> RasterDiff {
        RasterDiff { steps: vec![step], ..Default::default() }
    }
    
    impl Operation<RasterProjection> for RasterOp {
        type Diff = RasterDiff;
    
        fn diff(&self, _projection: &RasterProjection) -> RasterDiff {
            match self {
                RasterOp::AddLayer { parent_id, index, layer } => {
                    step_diff(RasterStep::AddLayer { parent_id: parent_id.clone(), index: *index, layer: layer.clone() })
                }
                RasterOp::RemoveLayer { layer_id } => step_diff(RasterStep::RemoveLayer { layer_id: layer_id.clone() }),
                RasterOp::PatchLayer { layer_id, patch } => {
                    step_diff(RasterStep::PatchLayer { layer_id: layer_id.clone(), patch: patch.clone() })
                }
                RasterOp::MoveLayer { layer_id, parent_id, index } => {
                    step_diff(RasterStep::MoveLayer { layer_id: layer_id.clone(), parent_id: parent_id.clone(), index: *index })
                }
                RasterOp::SetCamera { camera } => RasterDiff { camera: Some(camera.clone()), ..Default::default() },
                RasterOp::ReplaceDocument { document } => RasterDiff { replace: Some(Box::new(document.clone())), ..Default::default() },
            }
        }
    
        fn backwards(&self, projection: &RasterProjection) -> Vec<Self> {
            match self {
                RasterOp::AddLayer { layer, .. } => vec![RasterOp::RemoveLayer { layer_id: layer_node_id(layer).to_string() }],
                RasterOp::RemoveLayer { layer_id } => match (locate_layer(&projection.layers, layer_id), find_layer(&projection.layers, layer_id)) {
                    (Some((parent_id, index)), Some(layer)) => vec![RasterOp::AddLayer { parent_id, index, layer: layer.clone() }],
                    _ => Vec::new(),
                },
                RasterOp::PatchLayer { layer_id, patch } => {
                    let mut probe = projection.layers.clone();
                    match patch_layer_in_tree(&mut probe, layer_id, patch) {
                        Some(inverse) => vec![RasterOp::PatchLayer { layer_id: layer_id.clone(), patch: inverse }],
                        None => Vec::new(),
                    }
                }
                RasterOp::MoveLayer { layer_id, .. } => match locate_layer(&projection.layers, layer_id) {
                    Some((parent_id, index)) => vec![RasterOp::MoveLayer { layer_id: layer_id.clone(), parent_id, index }],
                    None => Vec::new(),
                },
                RasterOp::SetCamera { .. } => vec![RasterOp::SetCamera { camera: projection.camera.clone() }],
                RasterOp::ReplaceDocument { .. } => vec![RasterOp::ReplaceDocument { document: projection.clone() }],
            }
        }
    }
    
    pub type RasterEnvelope = vcs::DocumentVcsEnvelope<RasterProjection, RasterOp>;
    pub type RasterStore = vcs::DocumentVcsStore<RasterProjection, RasterOp>;
    //#endregion 🔖Ops
    
    //#region 🔖WasmDocumentVcs
    #[cfg(target_arch = "wasm32")]
    use std::cell::RefCell;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen::prelude::*;
    
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub struct RasterDocumentVcs {
        store: RefCell<RasterStore>,
    }
    
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    impl RasterDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: &str) -> Result<RasterDocumentVcs, JsValue> {
            let envelope: RasterEnvelope =
                serde_json::from_str(envelope_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(Self {
                store: RefCell::new(RasterStore::new(envelope)),
            })
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
    //#endregion 🔖WasmDocumentVcs
    
    //#region 🧪DocumentVcsTests
    #[cfg(test)]
    mod raster_vcs_tests {
        use super::*;
        use vcs::{apply_operation, create_document_vcs_envelope, DocumentVcsCommand};
    
        fn pixel_layer(id: &str, name: &str) -> RasterLayerNode {
            RasterLayerNode::Pixel {
                id: id.into(),
                name: name.into(),
                visible: true,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: None,
                width: Some(512),
                height: Some(512),
                image_key: None,
            }
        }
    
        fn round_trip(projection: &RasterProjection, op: &RasterOp) -> RasterProjection {
            let forward = apply_operation(projection, op);
            let mut restored = forward.clone();
            for back in op.backwards(projection) {
                restored = apply_operation(&restored, &back);
            }
            assert_eq!(&restored, projection, "backwards() must restore the pre-op projection");
            forward
        }
    
        #[test]
        fn add_remove_patch_layer_round_trip() {
            let projection = empty_raster_projection();
            let added = round_trip(&projection, &RasterOp::AddLayer { parent_id: None, index: 0, layer: pixel_layer("l1", "Base") });
            assert_eq!(added.layers.len(), 1);
            let patched = round_trip(
                &added,
                &RasterOp::PatchLayer { layer_id: "l1".into(), patch: RasterLayerPatch { name: Some("Renamed".into()), visible: Some(false), ..Default::default() } },
            );
            assert_eq!(layer_name(&patched.layers[0]), "Renamed");
            assert!(!layer_visible(&patched.layers[0]));
            let removed = round_trip(&patched, &RasterOp::RemoveLayer { layer_id: "l1".into() });
            assert!(removed.layers.is_empty());
        }
    
        #[test]
        fn move_layer_into_group_round_trip() {
            let mut projection = empty_raster_projection();
            projection.layers.push(RasterLayerNode::Group {
                id: "g1".into(),
                name: "Group".into(),
                visible: true,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: None,
                children: Vec::new(),
            });
            projection.layers.push(pixel_layer("l1", "Base"));
            let moved = round_trip(&projection, &RasterOp::MoveLayer { layer_id: "l1".into(), parent_id: Some("g1".into()), index: 0 });
            let RasterLayerNode::Group { children, .. } = &moved.layers[0] else { panic!("expected group") };
            assert_eq!(children.len(), 1);
            assert_eq!(layer_node_id(&children[0]), "l1");
        }
    
        #[test]
        fn set_camera_and_replace_round_trip() {
            let projection = empty_raster_projection();
            let next = round_trip(&projection, &RasterOp::SetCamera { camera: RasterCamera { x: 4.0, y: 5.0, zoom: 2.0 } });
            assert_eq!(next.camera.zoom, 2.0);
            let mut replacement = empty_raster_projection();
            replacement.layers.push(pixel_layer("l9", "Replaced"));
            let replaced = round_trip(&projection, &RasterOp::ReplaceDocument { document: replacement.clone() });
            assert_eq!(replaced, replacement);
        }
    
        #[test]
        fn store_applies_layer_add() {
            let mut store = RasterStore::new(create_document_vcs_envelope(
                RASTER_DOCUMENT_SCHEMA,
                "raster",
                empty_raster_projection(),
                None,
            ));
            store
                .dispatch(DocumentVcsCommand::Apply {
                    operations: vec![RasterOp::AddLayer { parent_id: None, index: 0, layer: pixel_layer("l1", "Base") }],
                    description: None,
                })
                .expect("apply");
            assert_eq!(store.projection().expect("projection").layers.len(), 1);
        }
    }
    //#endregion 🧪DocumentVcsTests
    //#endregion 🔖DocumentVcs
}
//#endregion 🔖Domain


use semio_framework_plugin::{SurfaceKind,
    build_paint_2d_scene, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree,
    ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_stack_vertical,
    ui_text, ActionArgDef, ActionArgOption, ActionDefinition, ActionEmit, ActionKind, App, ActionDescriptor,
    AppLabelsOverlay, AppLabelsOverlayExt, DocumentApp, DocumentView, MediaClass, MediaForm, MediaType, OsMediaCapability, OsMediaFormat, PanelGroup, PanelTreeBuilder,
    Paint2dScene, ResourceKindSpec, UtilityCategory, UtilityDefinition, WindowMeasure, is_de_locale, localized_label_map, resolve_labels,
    selection_ids, tree_item_with_action,
    UiInspectorFieldGroup, UiNode, UiPresence, UiSectionNode, UiTreeItemNode, ViewState,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    create_default_layout, SET_ACTIVE_UTILITY_ACTION_ID,
};
use crate::domain::{
    empty_raster_projection, find_layer, flatten_raster_layers, layer_name, layer_node_id, layer_visible,
    RasterCamera, RasterImageAsset, RasterLayerNode, RasterLayerPatch, RasterOp,
    RasterProjection as RasterDocument, RasterTransform,
};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU32, Ordering};

//#region 🔖Constants
const RASTER_PLAY_APP_ID: &str = "raster-play";
const RASTER_PLAY_CONTROLLER_ID: &str = "raster-play";
const RASTER_PLAY_SURFACE_COMPOSITE: &str = "raster.play.composite";
const RASTER_PLAY_SURFACE_NAVIGATOR: &str = "raster.play.navigator";
const RASTER_PLAY_BODY_COMPOSITE: &str = "raster.play.composite";
const RASTER_PLAY_BODY_NAVIGATOR: &str = "raster.play.navigator";
const RASTER_PLAY_BODY_LAYERS: &str = "raster.play.layers";
const RASTER_PLAY_BODY_CATALOGUE: &str = "raster.play.catalogue";
const RASTER_PLAY_BODY_MASKS: &str = "raster.play.masks";
const RASTER_PLAY_BODY_PROPERTIES: &str = "raster.play.properties";
const RASTER_PLAY_WINDOW_COMPOSITE: &str = "raster-composite";
const RASTER_PLAY_WINDOW_NAVIGATOR: &str = "raster-navigator";
const RASTER_PLAY_MASKS_TAB_ID: &str = "raster.panel.masks";
const RASTER_DOCUMENT_SCHEMA: &str = "raster.document";
const RASTER_TREE_PREFIX: &str = "raster-play-layers";
/// 🧰 Fallback utility when the host has not yet asserted a session active utility for the composite window.
const RASTER_DEFAULT_UTILITY: &str = "selectMarquee";

const SEMIO_EXAMPLE_JSON: &str = include_str!("../../example/semio.raster.json");

static RASTER_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
//#endregion 🔖Constants

//#region 🔖Document
/// 🎛️ Ephemeral view state (selection, hover, utility/brush settings, navigator viewport) held in the
/// app struct — never in the document — so it stays out of undo history and off the op channel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RasterPlayRuntime {
    selected_ids: Vec<String>,
    hovered_id: Option<String>,
    brush_size: f32,
    brush_opacity: f32,
    composite_viewport: Option<RasterViewportSize>,
}

impl RasterPlayRuntime {
    fn new() -> Self {
        Self {
            selected_ids: Vec::new(),
            hovered_id: None,
            brush_size: 24.0,
            brush_opacity: 1.0,
            composite_viewport: None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RasterViewportSize {
    width: f64,
    height: f64,
}

fn create_raster_id(prefix: &str) -> String {
    let next = RASTER_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{next}")
}

fn create_pixel_layer(name: &str, width: u32, height: u32) -> RasterLayerNode {
    RasterLayerNode::Pixel {
        id: create_raster_id("layer"),
        name: name.into(),
        visible: true,
        opacity: 1.0,
        blend_mode: "normal".into(),
        transform: RasterTransform::default(),
        mask: None,
        width: Some(width),
        height: Some(height),
        image_key: None,
    }
}

fn create_group_layer() -> RasterLayerNode {
    RasterLayerNode::Group {
        id: create_raster_id("group"),
        name: "Group".into(),
        visible: true,
        opacity: 1.0,
        blend_mode: "normal".into(),
        transform: RasterTransform::default(),
        mask: None,
        children: Vec::new(),
    }
}

fn create_adjustment_layer() -> RasterLayerNode {
    RasterLayerNode::Adjustment {
        id: create_raster_id("adjust"),
        name: "Adjustment".into(),
        visible: true,
        opacity: 1.0,
        blend_mode: "normal".into(),
        transform: RasterTransform::default(),
        adjustment_kind: "brightnessContrast".into(),
    }
}

fn create_layer_of_kind(kind: &str) -> RasterLayerNode {
    match kind {
        "group" => create_group_layer(),
        "adjustment" => create_adjustment_layer(),
        _ => create_pixel_layer("Layer", 512, 512),
    }
}

fn empty_raster_document() -> RasterDocument {
    let mut document = empty_raster_projection();
    document.id = "empty".into();
    document.layers = vec![create_pixel_layer("Background", 512, 512)];
    document
}

fn layer_row_id(layer: &RasterLayerNode) -> String {
    let segment = match layer {
        RasterLayerNode::Group { .. } => "group",
        RasterLayerNode::Adjustment { .. } => "adjustment",
        RasterLayerNode::Pixel { .. } => "layer",
    };
    format!("{RASTER_TREE_PREFIX}.{segment}.{}", layer_node_id(layer))
}

fn layer_id_from_tree_row_id(row_id: &str) -> Option<String> {
    row_id
        .strip_prefix(&format!("{RASTER_TREE_PREFIX}."))
        .and_then(|rest| rest.split('.').nth(1))
        .map(str::to_string)
}

fn mask_row_id(target_id: &str) -> String {
    format!("{RASTER_TREE_PREFIX}.mask.{target_id}")
}

/// 📄 Duplicates a layer subtree with freshly minted ids (a new document node, not an op inverse).
fn clone_layer(layer: &RasterLayerNode) -> RasterLayerNode {
    match layer {
        RasterLayerNode::Pixel { name, visible, opacity, blend_mode, transform, mask, width, height, image_key, .. } => {
            RasterLayerNode::Pixel {
                id: create_raster_id("layer"),
                name: format!("{name} copy"),
                visible: *visible,
                opacity: *opacity,
                blend_mode: blend_mode.clone(),
                transform: transform.clone(),
                mask: mask.clone(),
                width: *width,
                height: *height,
                image_key: image_key.clone(),
            }
        }
        RasterLayerNode::Group { name, visible, opacity, blend_mode, transform, mask, children, .. } => RasterLayerNode::Group {
            id: create_raster_id("group"),
            name: format!("{name} copy"),
            visible: *visible,
            opacity: *opacity,
            blend_mode: blend_mode.clone(),
            transform: transform.clone(),
            mask: mask.clone(),
            children: children.iter().map(clone_layer).collect(),
        },
        RasterLayerNode::Adjustment { name, visible, opacity, blend_mode, transform, adjustment_kind, .. } => {
            RasterLayerNode::Adjustment {
                id: create_raster_id("adjust"),
                name: format!("{name} copy"),
                visible: *visible,
                opacity: *opacity,
                blend_mode: blend_mode.clone(),
                transform: transform.clone(),
                adjustment_kind: adjustment_kind.clone(),
            }
        }
    }
}

/// 🩹 Builds a sparse {@link RasterLayerPatch} for a `patchLayer`/`patchLayers` field write.
fn layer_patch_for_field(field: &str, value: &Value, prior: &RasterLayerNode) -> Option<RasterLayerPatch> {
    let mut patch = RasterLayerPatch::default();
    let opacity_of = crate::domain::layer_opacity(prior) as f64;
    match field {
        "name" => patch.name = Some(value.as_str().unwrap_or("").into()),
        "visible" => patch.visible = Some(value.as_bool().unwrap_or_else(|| !layer_visible(prior))),
        "opacity" => patch.opacity = Some(value.as_f64().unwrap_or(opacity_of) as f32),
        "blendMode" => patch.blend_mode = Some(value.as_str().unwrap_or("normal").into()),
        "transformX" => patch.transform_x = Some(value.as_f64().unwrap_or(0.0)),
        "transformY" => patch.transform_y = Some(value.as_f64().unwrap_or(0.0)),
        "width" => patch.width = Some(value.as_u64().unwrap_or(512) as u32),
        "height" => patch.height = Some(value.as_u64().unwrap_or(512) as u32),
        "adjustmentKind" => patch.adjustment_kind = Some(value.as_str().unwrap_or("brightnessContrast").into()),
        _ => return None,
    }
    Some(patch)
}
//#endregion 🔖Document

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the raster app; one field per label makes every locale combination compile-checked.
semio_framework_plugin::app_labels! {
    struct RasterPlayLabels {
        masks: &'static str = en: "Masks", de: "Masken";
        no_masks: &'static str = en: "No masks", de: "Keine Masken";
        mask_suffix: &'static str = en: "mask", de: "Maske";
        add_pixel: &'static str = en: "Add Pixel", de: "Pixel hinzufügen";
        add_group: &'static str = en: "Add Group", de: "Gruppe hinzufügen";
        layer_kinds: &'static str = en: "Layer kinds", de: "Ebenenarten";
        layer: &'static str = en: "Layer", de: "Ebene";
        catalogue_pixel: &'static str = en: "pixel — paintable bitmap layer", de: "pixel — bearbeitbare Bitmap-Ebene";
        catalogue_group: &'static str = en: "group — nested layer stack", de: "group — verschachtelter Ebenenstapel";
        catalogue_adjustment: &'static str = en: "adjustment — non-destructive filter", de: "adjustment — zerstörungsfreier Filter";
        window_composite: &'static str = en: "Composite", de: "Komposit";
        window_navigator: &'static str = en: "Navigator", de: "Navigator";
        name: &'static str = en: "Name", de: "Name";
        opacity: &'static str = en: "Opacity", de: "Deckkraft";
        mixed: &'static str = en: "Mixed", de: "Gemischt";
        schema_prefix: &'static str = en: "Schema", de: "Schema";
        brush_prefix: &'static str = en: "Brush", de: "Pinsel";
    }
}

//#region 🔖CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_raster_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the
/// command palette and Actions rail get a translated label without threading locale through the builder chain.
fn raster_action_labels(is_de: bool) -> HashMap<String, String> {
    localized_label_map(is_de, &[
        ("addLayer", "Add Layer", "Ebene hinzufügen"),
        ("setDocument", "Set Document", "Dokument festlegen"),
        ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
        ("setCamera", "Set Camera", "Kamera festlegen"),
        ("setCameraZoom", "Set Camera Zoom", "Kamerazoom festlegen"),
        ("setLayerVisible", "Set Layer Visible", "Ebenensichtbarkeit festlegen"),
        ("toggleLayerVisible", "Toggle Layer Visible", "Ebenensichtbarkeit umschalten"),
        ("dropLayerKind", "Drop Layer Kind", "Ebenenart ablegen"),
        ("deleteLayer", "Delete Layer", "Ebene löschen"),
        ("duplicateLayer", "Duplicate Layer", "Ebene duplizieren"),
        ("patchLayer", "Patch Layer", "Ebene aktualisieren"),
        ("patchLayers", "Patch Layers", "Ebenen aktualisieren"),
        ("moveLayer", "Move Layer", "Ebene verschieben"),
        ("selectAll", "Select All", "Alles auswählen"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
        ("setHover", "Set Hover", "Hover festlegen"),
        ("setBrushSize", "Set Brush Size", "Pinselgröße festlegen"),
        ("setBrushOpacity", "Set Brush Opacity", "Pinseldeckkraft festlegen"),
        ("setCompositeViewport", "Set Composite Viewport", "Komposit-Ansichtsfenster festlegen"),
    ])
}

/// 🗣️ (utility id) -> localized utility bar button label, for every `.utility(...)` declared in `create_raster_app`.
fn raster_utility_labels(is_de: bool) -> HashMap<String, String> {
    localized_label_map(is_de, &[
        ("selectMarquee", "Marquee Select", "Rahmenauswahl"),
        ("paintBrush", "Brush", "Pinsel"),
        ("paintEraser", "Eraser", "Radiergummi"),
    ])
}
//#endregion 🔖CommandLabels
//#endregion 🔖Terminology

//#region 🔖Panels
fn play_action(controller_id: &str, action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: controller_id.into(),
        action: action.into(),
        args,
    }
}

fn selection_from_view(view_state: &ViewState) -> Vec<String> {
    view_state
        .selection_json
        .as_ref()
        .and_then(|json| serde_json::from_str::<Value>(json).ok())
        .and_then(|value| {
            value.as_array().map(|items| {
                items
                    .iter()
                    .filter_map(|item| item.as_str().map(str::to_string))
                    .collect()
            })
        })
        .unwrap_or_default()
}

fn selection_from_runtime(runtime: &RasterPlayRuntime, view_state: &ViewState) -> Vec<String> {
    if !runtime.selected_ids.is_empty() {
        return runtime.selected_ids.clone();
    }
    selection_from_view(view_state)
}

fn layer_tree_item(layer: &RasterLayerNode) -> UiTreeItemNode {
    let nested = match layer {
        RasterLayerNode::Group { children, .. } => {
            if children.is_empty() {
                None
            } else {
                Some(children.iter().map(layer_tree_item).collect())
            }
        }
        _ => None,
    };
    let description = match layer {
        RasterLayerNode::Pixel { .. } => "pixel",
        RasterLayerNode::Group { .. } => "group",
        RasterLayerNode::Adjustment { .. } => "adjustment",
    };
    let icon_id = match layer {
        RasterLayerNode::Pixel { .. } => "image",
        RasterLayerNode::Group { .. } => "folder",
        RasterLayerNode::Adjustment { .. } => "sliders-horizontal",
    };
    UiTreeItemNode {
        icon_id: Some(icon_id.into()),
        default_open: Some(matches!(layer, RasterLayerNode::Group { .. })),
        draggable: Some(true),
        items: nested,
        dimmed: if layer_visible(layer) { None } else { Some(true) },
        ..tree_item_with_action(
            layer_row_id(layer),
            layer_name(layer),
            Some(description.into()),
            play_action(RASTER_PLAY_CONTROLLER_ID, "setSelection", Some(json!({ "ids": [layer_node_id(layer)] }))),
        )
    }
}

fn render_layers_panel(document: &RasterDocument, runtime: &RasterPlayRuntime, view_state: &ViewState, labels: &RasterPlayLabels) -> UiNode {
    let action_rows = vec![
        UiTreeItemNode {
            icon_id: Some("image".into()),
            ..tree_item_with_action(
                format!("{RASTER_TREE_PREFIX}.add.pixel"),
                labels.add_pixel,
                None,
                play_action(RASTER_PLAY_CONTROLLER_ID, "addLayer", Some(json!({ "kind": "pixel" }))),
            )
        },
        UiTreeItemNode {
            icon_id: Some("folder-plus".into()),
            ..tree_item_with_action(
                format!("{RASTER_TREE_PREFIX}.add.group"),
                labels.add_group,
                None,
                play_action(RASTER_PLAY_CONTROLLER_ID, "addLayer", Some(json!({ "kind": "group" }))),
            )
        },
    ];
    let layer_items: Vec<UiTreeItemNode> = document.layers.iter().map(layer_tree_item).collect();
    let selected_ids: Vec<String> = selection_from_runtime(runtime, view_state)
        .iter()
        .filter_map(|id| find_layer(&document.layers, id).map(layer_row_id))
        .collect();
    let highlighted_ids: Vec<String> = runtime
        .hovered_id
        .as_deref()
        .and_then(|id| find_layer(&document.layers, id))
        .map(|layer| vec![layer_row_id(layer)])
        .unwrap_or_default();
    PanelTreeBuilder::new(RASTER_TREE_PREFIX)
        .section(RASTER_TREE_PREFIX, Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()), true, [action_rows, layer_items].concat())
        .selected(selected_ids)
        .highlighted(highlighted_ids)
        .selection_change(play_action(RASTER_PLAY_CONTROLLER_ID, "setSelection", None))
        .build()
}

fn render_masks_panel(document: &RasterDocument, runtime: &RasterPlayRuntime, view_state: &ViewState, labels: &RasterPlayLabels) -> UiNode {
    let mut items = Vec::new();
    fn collect_masks(layer: &RasterLayerNode, items: &mut Vec<UiTreeItemNode>, labels: &RasterPlayLabels) {
        if let RasterLayerNode::Pixel { id, name, mask, .. }
        | RasterLayerNode::Group { id, name, mask, .. } = layer
        {
            if mask.as_ref().is_some_and(|mask| mask.enabled) {
                items.push(UiTreeItemNode {
                    icon_id: Some("scan".into()),
                    ..tree_item_with_action(
                        mask_row_id(id),
                        format!("{name} {}", labels.mask_suffix),
                        Some("mask".into()),
                        play_action(RASTER_PLAY_CONTROLLER_ID, "setSelection", Some(json!({ "ids": [id] }))),
                    )
                });
            }
        }
        if let RasterLayerNode::Group { children, .. } = layer {
            for child in children {
                collect_masks(child, items, labels);
            }
        }
    }
    for layer in &document.layers {
        collect_masks(layer, &mut items, labels);
    }
    PanelTreeBuilder::new(RASTER_TREE_PREFIX)
        .section_or_placeholder("raster-play-masks", Some(labels.masks.into()), true, items, labels.no_masks)
        .selected(
            selection_from_runtime(runtime, view_state)
                .iter()
                .map(|id| mask_row_id(id))
                .collect(),
        )
        .build()
}

fn render_catalogue_panel(labels: &RasterPlayLabels) -> UiNode {
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "raster-catalogue".into(),
        label: Some(labels.layer_kinds.into()),
        default_open: Some(true),
        presence: UiPresence::default(),
        children: vec![
            ui_text(labels.catalogue_pixel),
            ui_text(labels.catalogue_group),
            ui_text(labels.catalogue_adjustment),
        ],
    }])
}

fn render_properties_panel(document: &RasterDocument, runtime: &RasterPlayRuntime, view_state: &ViewState, labels: &RasterPlayLabels) -> UiNode {
    let selected = selection_from_runtime(runtime, view_state);
    let layers: Vec<&RasterLayerNode> = selected
        .iter()
        .filter_map(|id| find_layer(&document.layers, id))
        .collect();
    if layers.is_empty() {
        return ui_stack_vertical(vec![
            ui_text(format!("{}: {}", labels.schema_prefix, document.schema)),
            ui_text(format!("{}: {} @ {}", labels.brush_prefix, runtime.brush_size, runtime.brush_opacity)),
        ]);
    }
    let names: Vec<String> = layers.iter().map(|layer| layer_name(*layer).into()).collect();
    let opacities: Vec<f64> = layers.iter().map(|layer| crate::domain::layer_opacity(layer) as f64).collect();
    let mixed_name = ui_inspector_mixed_text(&names);
    let mixed_opacity = ui_inspector_mixed_number(&opacities);
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { presence: UiPresence::default(),
        id: "raster-properties.layer".into(),
        label: labels.layer.into(),
        default_open: Some(true),
        fields: vec![
            ui_inspector_readonly_field(
                "raster-properties.name",
                labels.name,
                mixed_name.placeholder.unwrap_or(mixed_name.value),
            ),
            ui_inspector_readonly_field(
                "raster-properties.opacity",
                labels.opacity,
                if mixed_opacity.uniform {
                    mixed_opacity.value.to_string()
                } else {
                    labels.mixed.to_string()
                },
            ),
        ],
    }])
}
//#endregion 🔖Panels

//#region 🔖Render
/// 📡 Document JSON for the WASM compositor, omitting embedded assets/camera/utility/brush — mirrors premigration `rasterDocumentToSyncJson`.
fn document_sync_json(document: &RasterDocument) -> String {
    let mut value = serde_json::to_value(document).unwrap_or(Value::Null);
    if let Value::Object(ref mut map) = value {
        map.remove("assets");
        map.remove("camera");
        map.remove("brushSize");
        map.remove("brushOpacity");
    }
    value.to_string()
}

fn raster_scene(document: &RasterDocument, runtime: &RasterPlayRuntime, active_utility: &str, view_mode: &str) -> Paint2dScene {
    Paint2dScene {
        document_sync_json: document_sync_json(document),
        assets_json: serde_json::to_string(&document.assets).unwrap_or_else(|_| "{}".into()),
        camera_json: serde_json::to_string(&document.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into()),
        selection_json: serde_json::to_string(&runtime.selected_ids).unwrap_or_else(|_| "[]".into()),
        hovered_id: runtime.hovered_id.clone(),
        active_utility: active_utility.into(),
        brush_size: runtime.brush_size as f64,
        brush_opacity: runtime.brush_opacity as f64,
        view_mode: view_mode.into(),
        composite_viewport_json: runtime
            .composite_viewport
            .as_ref()
            .map(|viewport| serde_json::to_string(viewport).unwrap_or_else(|_| "{}".into())),
    }
}

fn render_composite_scene(document: &RasterDocument, runtime: &RasterPlayRuntime, active_utility: &str) -> UiNode {
    build_paint_2d_scene(RASTER_PLAY_SURFACE_COMPOSITE, RASTER_PLAY_CONTROLLER_ID, raster_scene(document, runtime, active_utility, "composite"))
}

fn render_navigator_scene(document: &RasterDocument, runtime: &RasterPlayRuntime, active_utility: &str) -> UiNode {
    build_paint_2d_scene(RASTER_PLAY_SURFACE_NAVIGATOR, RASTER_PLAY_CONTROLLER_ID, raster_scene(document, runtime, active_utility, "navigator"))
}
//#endregion 🔖Render

//#region 🔖RasterPlayApp
#[derive(Default)]
struct RasterPlayApp {
    runtime: RasterPlayRuntime,
}

impl RasterPlayApp {
    /// 🩹 Builds `PatchLayer` ops for a `patchLayer`/`patchLayers` field write across ids.
    fn patch_layer_ops(&self, document: &RasterDocument, layer_ids: &[String], field: &str, value: &Value) -> Vec<RasterOp> {
        layer_ids
            .iter()
            .filter_map(|layer_id| {
                let prior = find_layer(&document.layers, layer_id)?;
                let patch = layer_patch_for_field(field, value, prior)?;
                Some(RasterOp::PatchLayer { layer_id: layer_id.clone(), patch })
            })
            .collect()
    }
}

impl Default for RasterPlayRuntime {
    fn default() -> Self {
        Self::new()
    }
}

fn raster_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    play_action(RASTER_PLAY_CONTROLLER_ID, action, args)
}

fn raster_paint_utility_options(runtime: &RasterPlayRuntime, utility: &str, label: &str) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("raster-utility-options-{utility}"),
        label: label.into(),
        default_open: Some(true),
        active_utility_id: Some(utility.into()),
        children: vec![
            WindowMeasure::Slider {
                id: format!("raster-{utility}-size"),
                label: Some("Size".into()),
                value: runtime.brush_size as f64,
                min: 1.0,
                max: 128.0,
                step: Some(1.0),
                ready: None,
                loading: None,
                waiting: None,
                on_change: raster_action("setBrushSize", None),
                },
            WindowMeasure::Slider {
                id: format!("raster-{utility}-opacity"),
                label: Some("Opacity".into()),
                value: runtime.brush_opacity as f64,
                min: 0.0,
                max: 1.0,
                step: Some(0.05),
                ready: None,
                loading: None,
                waiting: None,
                on_change: raster_action("setBrushOpacity", None),
                },
        ],
    }
}

fn raster_window_measures(runtime: &RasterPlayRuntime) -> Vec<WindowMeasure> {
    vec![
        raster_paint_utility_options(runtime, "paintBrush", "Brush"),
        raster_paint_utility_options(runtime, "paintEraser", "Eraser"),
    ]
}

impl DocumentApp for RasterPlayApp {
    type Projection = RasterDocument;
    type Op = RasterOp;

    fn app_id(&self) -> &str {
        RASTER_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        RASTER_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> RasterDocument {
        empty_raster_document()
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, RasterDocument>,
        _view_state: &ViewState,
    ) -> ActionEmit<RasterOp> {
        let document = doc.projection;
        match action {
            // 👁️ View actions — mutate ephemeral runtime, emit no ops.
            "setBrushSize" => {
                if let Some(size) = args.and_then(|value| value.get("value").or_else(|| value.get("brushSize"))).and_then(|value| value.as_f64()) {
                    self.runtime.brush_size = size as f32;
                }
                ActionEmit::default()
            }
            "setBrushOpacity" => {
                if let Some(opacity) = args.and_then(|value| value.get("value").or_else(|| value.get("opacity"))).and_then(|value| value.as_f64()) {
                    self.runtime.brush_opacity = (opacity as f32).clamp(0.0, 1.0);
                }
                ActionEmit::default()
            }
            SET_ACTIVE_UTILITY_ACTION_ID => {
                // 🧰 Host-owned utility switch: the active utility lives in session view state (never the
                // document). There is no plugin-side paint scratch to clear — brush strokes are painted
                // host-side in the WASM canvas — so this simply acknowledges with no ops or history.
                ActionEmit::default()
            }
            "setSelection" => {
                self.runtime.selected_ids = selection_ids(args);
                ActionEmit::default()
            }
            "setHover" => {
                self.runtime.hovered_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(str::to_string);
                ActionEmit::default()
            }
            "setCompositeViewport" => {
                if let (Some(width), Some(height)) = (
                    args.and_then(|value| value.get("width")).and_then(|value| value.as_f64()),
                    args.and_then(|value| value.get("height")).and_then(|value| value.as_f64()),
                ) {
                    self.runtime.composite_viewport = Some(RasterViewportSize { width, height });
                }
                ActionEmit::default()
            }
            "selectAll" => {
                self.runtime.selected_ids = flatten_raster_layers(&document.layers)
                    .into_iter()
                    .map(|layer| layer_node_id(layer).to_string())
                    .collect();
                ActionEmit::default()
            }
            // 📷 Camera — a coalesced scalar op so a pan/zoom gesture is one undo step.
            "setCamera" | "setCameraZoom" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")).and_then(|value| serde_json::from_value::<RasterCamera>(value.clone()).ok()) {
                    return ActionEmit { ops: vec![RasterOp::SetCamera { camera }], coalesce_key: Some("camera".into()), ..Default::default() };
                }
                if let Some(zoom) = args.and_then(|value| value.get("zoom")).and_then(|value| value.as_f64()) {
                    let camera = RasterCamera { zoom, ..document.camera.clone() };
                    return ActionEmit { ops: vec![RasterOp::SetCamera { camera }], coalesce_key: Some("camera".into()), ..Default::default() };
                }
                ActionEmit::default()
            }
            // ✏️ Operations — dispatched as VCS operations with a true inverse.
            "setActiveExample" => {
                let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                let replacement = if example_id == "semio" {
                    serde_json::from_str::<RasterDocument>(SEMIO_EXAMPLE_JSON).unwrap_or_else(|_| empty_raster_document())
                } else {
                    empty_raster_document()
                };
                self.runtime.selected_ids.clear();
                ActionEmit::ops(vec![RasterOp::ReplaceDocument { document: replacement }])
            }
            "setDocument" => match args.and_then(|value| value.get("document")).and_then(|value| serde_json::from_value::<RasterDocument>(value.clone()).ok()) {
                Some(replacement) => ActionEmit::ops(vec![RasterOp::ReplaceDocument { document: replacement }]),
                None => ActionEmit::default(),
            },
            "setLayerVisible" | "toggleLayerVisible" => {
                let Some(target_id) = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()) else {
                    return ActionEmit::default();
                };
                let Some(layer) = find_layer(&document.layers, target_id) else { return ActionEmit::default() };
                let visible = args
                    .and_then(|value| value.get("visible"))
                    .and_then(|value| value.as_bool())
                    .unwrap_or_else(|| !layer_visible(layer));
                ActionEmit::ops(vec![RasterOp::PatchLayer {
                    layer_id: target_id.into(),
                    patch: RasterLayerPatch { visible: Some(visible), ..Default::default() },
                }])
            }
            "addLayer" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("pixel");
                let layer = create_layer_of_kind(kind);
                self.runtime.selected_ids = vec![layer_node_id(&layer).to_string()];
                ActionEmit::ops(vec![RasterOp::AddLayer { parent_id: None, index: document.layers.len(), layer }])
            }
            "dropLayerKind" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("pixel");
                let layer = create_layer_of_kind(kind);
                self.runtime.selected_ids = vec![layer_node_id(&layer).to_string()];
                ActionEmit::ops(vec![RasterOp::AddLayer { parent_id: None, index: document.layers.len(), layer }])
            }
            "deleteLayer" => {
                let Some(target_id) = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()) else {
                    return ActionEmit::default();
                };
                if find_layer(&document.layers, target_id).is_none() {
                    return ActionEmit::default();
                }
                self.runtime.selected_ids.retain(|id| id != target_id);
                ActionEmit::ops(vec![RasterOp::RemoveLayer { layer_id: target_id.into() }])
            }
            "duplicateLayer" => {
                let Some(target_id) = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()) else {
                    return ActionEmit::default();
                };
                match find_layer(&document.layers, target_id) {
                    Some(layer) => {
                        let copy = clone_layer(layer);
                        self.runtime.selected_ids = vec![layer_node_id(&copy).to_string()];
                        ActionEmit::ops(vec![RasterOp::AddLayer { parent_id: None, index: document.layers.len(), layer: copy }])
                    }
                    None => ActionEmit::default(),
                }
            }
            "patchLayer" => {
                let layer_id = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()).unwrap_or("");
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args
                    .and_then(|value| value.get("value"))
                    .or_else(|| args.and_then(|value| value.get("pressed")))
                    .cloned()
                    .unwrap_or(Value::Null);
                if layer_id.is_empty() || field.is_empty() {
                    return ActionEmit::default();
                }
                ActionEmit::ops(self.patch_layer_ops(document, &[layer_id.to_string()], field, &value))
            }
            "patchLayers" => {
                let layer_ids: Vec<String> = args
                    .and_then(|value| value.get("layerIds"))
                    .and_then(|value| value.as_array())
                    .map(|values| values.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect())
                    .unwrap_or_default();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args
                    .and_then(|value| value.get("value"))
                    .or_else(|| args.and_then(|value| value.get("pressed")))
                    .cloned()
                    .unwrap_or(Value::Null);
                if field.is_empty() {
                    return ActionEmit::default();
                }
                ActionEmit::ops(self.patch_layer_ops(document, &layer_ids, field, &value))
            }
            "moveLayer" => {
                let Some(layer_id) = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()) else {
                    return ActionEmit::default();
                };
                if find_layer(&document.layers, layer_id).is_none() {
                    return ActionEmit::default();
                }
                let target_row_id = args.and_then(|value| value.get("targetRowId")).and_then(|value| value.as_str()).unwrap_or("raster-play-layers");
                let drop_position = args.and_then(|value| value.get("dropPosition")).and_then(|value| value.as_str()).unwrap_or("after");
                let parent_id = layer_id_from_tree_row_id(target_row_id).and_then(|id| {
                    find_layer(&document.layers, &id).and_then(|entry| matches!(entry, RasterLayerNode::Group { .. }).then_some(id))
                });
                let index = if drop_position == "before" {
                    0
                } else if let Some(parent) = &parent_id {
                    match find_layer(&document.layers, parent) {
                        Some(RasterLayerNode::Group { children, .. }) => children.len(),
                        _ => 0,
                    }
                } else {
                    document.layers.len()
                };
                ActionEmit::ops(vec![RasterOp::MoveLayer { layer_id: layer_id.into(), parent_id, index }])
            }
            _ => ActionEmit::default(),
        }
    }

    fn window_measures(&self, _doc: &DocumentView<'_, RasterDocument>, _view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
        let measures = raster_window_measures(&self.runtime);
        HashMap::from([(RASTER_PLAY_WINDOW_COMPOSITE.into(), measures)])
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, RasterDocument>, view_state: &ViewState) -> UiNode {
        let document = doc.projection;
        let labels = resolve_labels::<RasterPlayLabels>(view_state);
        let active_utility = view_state.active_utility_id.as_deref().unwrap_or(RASTER_DEFAULT_UTILITY);
        match body_key {
            RASTER_PLAY_BODY_COMPOSITE => render_composite_scene(document, &self.runtime, active_utility),
            RASTER_PLAY_BODY_NAVIGATOR => render_navigator_scene(document, &self.runtime, active_utility),
            RASTER_PLAY_BODY_LAYERS => render_layers_panel(document, &self.runtime, view_state, labels),
            RASTER_PLAY_BODY_MASKS => render_masks_panel(document, &self.runtime, view_state, labels),
            RASTER_PLAY_BODY_CATALOGUE => render_catalogue_panel(labels),
            RASTER_PLAY_BODY_PROPERTIES => render_properties_panel(document, &self.runtime, view_state, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = resolve_labels::<RasterPlayLabels>(view_state);
        let is_de = is_de_locale(view_state);
        AppLabelsOverlay::default()
            .window_kind_label(RASTER_PLAY_WINDOW_COMPOSITE, labels.window_composite)
            .window_kind_label(RASTER_PLAY_WINDOW_NAVIGATOR, labels.window_navigator)
            .panel_tab_label(RASTER_PLAY_MASKS_TAB_ID, labels.masks)
            .mode_label("edit", if is_de { "Bearbeiten" } else { "Edit" })
            .action_labels(raster_action_labels(is_de))
            .utility_labels(raster_utility_labels(is_de))
            .example_labels(HashMap::from([("semio".to_string(), "Semio".to_string())]))
    }
}
//#endregion 🔖RasterPlayApp

//#region 🔖Manifest
/// 🛠️ An internal (non-palette) action declaration — the panel/pointer/gesture-bound vocabulary
/// dispatched by the layer tree, catalogue drops, camera and inspector, never a palette command.
fn raster_internal_action(id: &str, label: &str, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new(id, label, kind) }
}

/// 🧰 One composite-window utility declaration; ids must stay host-compatible (`paint*` prefix paints,
/// `paintEraser` erases, `selectMarquee` selects) because the scene's active utility feeds `RasterHost`.
fn raster_utility(id: &str, label: &str, icon: &str, group: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(category), ..UtilityDefinition::new(id, label, icon) }
}

fn create_raster_app() -> App {
    App::from_builder(
        App::builder(RASTER_PLAY_APP_ID, "Raster").document(["semio", "raster"])
            .resource_kind(ResourceKindSpec {
                id: "2d.raster".into(),
                name: "2D Raster".into(),
                source_format: "raster.document".into(),
                component_kind: "raster".into(),
                dimension: "2d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
                schema: "raster.document".into(),
                export_formats: vec![OsMediaFormat::Svg, OsMediaFormat::Png],
                import_formats: vec![OsMediaFormat::Svg, OsMediaFormat::Png],
            })
            .icon_id("raster")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(RASTER_PLAY_WINDOW_COMPOSITE, "Composite", RASTER_PLAY_BODY_COMPOSITE, SurfaceKind::Paint2d)
            .window_kind(RASTER_PLAY_WINDOW_NAVIGATOR, "Navigator", RASTER_PLAY_BODY_NAVIGATOR, SurfaceKind::Paint2d)
            .default_layout(create_default_layout(
                &[RASTER_PLAY_WINDOW_COMPOSITE.into(), RASTER_PLAY_WINDOW_NAVIGATOR.into()],
                "row",
                Some(&[72.0, 28.0]),
                Some(&["Composite".into(), "Navigator".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                RASTER_PLAY_BODY_LAYERS,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                RASTER_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(RASTER_PLAY_MASKS_TAB_ID, "Masks", PanelGroup::Workbench, RASTER_PLAY_BODY_MASKS)
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                RASTER_PLAY_BODY_PROPERTIES,
            )
            // ✏️ Palette-visible content operations.
            .operation("addLayer", "Add Layer")
            .operation("setDocument", "Set Document")
            .operation("setActiveExample", "Set Active Example")
            // 🔧 Internal content operations — layer-tree / catalogue-drop / camera / inspector bound.
            .action_with(raster_internal_action("setCamera", "Set Camera", ActionKind::Operation))
            .action_with(raster_internal_action("setCameraZoom", "Set Camera Zoom", ActionKind::Operation))
            .action_with(raster_internal_action("setLayerVisible", "Set Layer Visible", ActionKind::Operation))
            .action_with(raster_internal_action("toggleLayerVisible", "Toggle Layer Visible", ActionKind::Operation))
            .action_with(raster_internal_action("dropLayerKind", "Drop Layer Kind", ActionKind::Operation))
            .action_with(raster_internal_action("deleteLayer", "Delete Layer", ActionKind::Operation))
            .action_with(raster_internal_action("duplicateLayer", "Duplicate Layer", ActionKind::Operation))
            .action_with(raster_internal_action("patchLayer", "Patch Layer", ActionKind::Operation))
            .action_with(raster_internal_action("patchLayers", "Patch Layers", ActionKind::Operation))
            .action_with(raster_internal_action("moveLayer", "Move Layer", ActionKind::Operation))
            // 👁️ Ephemeral view state — selection, hover, live brush controls, navigator viewport.
            .view_action("selectAll", "Select All")
            .action_with(raster_internal_action("setSelection", "Set Selection", ActionKind::View))
            .action_with(raster_internal_action("setHover", "Set Hover", ActionKind::View))
            .action_with(raster_internal_action("setBrushSize", "Set Brush Size", ActionKind::View))
            .action_with(raster_internal_action("setBrushOpacity", "Set Brush Opacity", ActionKind::View))
            .action_with(raster_internal_action("setCompositeViewport", "Set Composite Viewport", ActionKind::View))
            // 📝 Staged palette-form arguments for the two palette operations.
            .action_args("addLayer", vec![
                ActionArgDef::select("kind", "Layer Kind", vec![
                    ActionArgOption::new("pixel", "Pixel"),
                    ActionArgOption::new("group", "Group"),
                    ActionArgOption::new("adjustment", "Adjustment"),
                ]).required().default_value("pixel"),
            ])
            .action_args("setDocument", vec![
                ActionArgDef::text("document", "Document"),
            ])
            // 🧰 Composite-window utilities — one exclusive set, active utility host-owned (never a document op).
            .utility(raster_utility("selectMarquee", "Marquee Select", "square-dashed", "Select", UtilityCategory::Selection))
            .utility(raster_utility("paintBrush", "Brush", "brush", "Paint", UtilityCategory::Utilities))
            .utility(raster_utility("paintEraser", "Eraser", "eraser", "Paint", UtilityCategory::Utilities))
            .window_kind_utilities(RASTER_PLAY_WINDOW_COMPOSITE, vec![
                "selectMarquee".into(), "paintBrush".into(), "paintEraser".into(),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("semio", "Semio", SEMIO_EXAMPLE_JSON)
    .program("raster", "Raster", "2d.raster")
}

fn raster_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    semio_framework_os::title_card_svg(value, "Raster", 1024, 1024)
}

/// 📥 Rasterizes a DWG drawing's flat SVG projection into a single-layer raster document.
fn raster_document_json_from_dwg(drawing: &semio_framework_os::DwgDrawing) -> Result<Value, String> {
    let (svg, width, height) = semio_framework_os::dwg_drawing_to_svg(drawing)?;
    let data = semio_framework_os::rasterize_svg_to_png_base64(&svg, width, height)?;
    let asset_key = create_raster_id("dwg-asset");
    let mut layer = create_pixel_layer("DWG Import", width, height);
    if let RasterLayerNode::Pixel { image_key, .. } = &mut layer {
        *image_key = Some(asset_key.clone());
    }
    let mut assets = HashMap::new();
    assets.insert(asset_key, RasterImageAsset { mime: "image/png".into(), data });
    let document = RasterDocument {
        schema: RASTER_DOCUMENT_SCHEMA.into(),
        id: create_raster_id("dwg-import"),
        title: Some("DWG Import".into()),
        camera: RasterCamera::default(),
        layers: vec![layer],
        assets,
    };
    serde_json::to_value(&document).map_err(|error| error.to_string())
}

fn register_raster_exports() {
    semio_framework_os::register_2d_export_handlers("2d.raster", "raster", raster_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("2d.raster", raster_document_json_from_dwg);
}

semio_framework_plugin::semio_plugin! {
    id: "raster", label: "Raster", version: "0.1.0",
    setup: register_raster_exports,
    apps: [ create_raster_app => RasterPlayApp ],
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp};
    use vcs::MemoryBackbone;

    fn semio_app() -> VcsDocumentApp<RasterPlayApp> {
        let mut app = testkit::new_app::<RasterPlayApp>();
        let document: RasterDocument = serde_json::from_str(SEMIO_EXAMPLE_JSON).expect("semio raster json");
        app.load_document(
            &serde_json::to_string(&vcs::create_document_vcs_envelope::<RasterDocument, RasterOp>(
                RASTER_DOCUMENT_SCHEMA,
                "raster",
                document,
                None,
            ))
            .unwrap(),
        )
        .expect("load semio");
        app
    }

    #[test]
    fn renders_raster_scene() {
        let mut app = testkit::new_app::<RasterPlayApp>();
        let node = app.render(RASTER_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("raster"));
    }

    #[test]
    fn renders_navigator_scene() {
        let mut app = testkit::new_app::<RasterPlayApp>();
        let json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_NAVIGATOR, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains("\"componentKind\":\"raster\""));
        assert!(json.contains("\"viewMode\":\"navigator\""));
    }

    #[test]
    fn parses_semio_example_document() {
        let document: RasterDocument = serde_json::from_str(SEMIO_EXAMPLE_JSON).expect("semio raster json");
        assert!(!document.layers.is_empty());
    }

    #[test]
    fn imports_dwg_polyline_into_raster_document() {
        let mut drawing = semio_framework_os::DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        drawing.entities.push(semio_framework_os::DwgEntity {
            layer,
            color: semio_framework_os::DwgColor::ByLayer,
            geometry: semio_framework_os::DwgGeometry::LwPolyline {
                closed: true,
                elevation: 0.0,
                vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]],
                bulges: vec![0.0, 0.0, 0.0, 0.0],
            },
        });
        drawing.extmin = [0.0, 0.0, 0.0];
        drawing.extmax = [10.0, 10.0, 0.0];
        let value = raster_document_json_from_dwg(&drawing).expect("dwg import");
        let document: RasterDocument = serde_json::from_value(value).expect("valid raster document");
        assert_eq!(document.layers.len(), 1);
        let RasterLayerNode::Pixel { image_key, .. } = &document.layers[0] else {
            panic!("expected pixel layer");
        };
        let asset_key = image_key.as_ref().expect("image key set");
        let asset = document.assets.get(asset_key).expect("asset present");
        assert_eq!(asset.mime, "image/png");
        assert!(!asset.data.is_empty());
    }

    #[test]
    fn imports_empty_dwg_into_blank_raster_document() {
        let drawing = semio_framework_os::DwgDrawing::default();
        let value = raster_document_json_from_dwg(&drawing).expect("empty dwg import");
        let document: RasterDocument = serde_json::from_value(value).expect("valid raster document");
        assert_eq!(document.layers.len(), 1);
        let RasterLayerNode::Pixel { image_key, width, height, .. } = &document.layers[0] else {
            panic!("expected pixel layer");
        };
        assert_eq!(*width, Some(1));
        assert_eq!(*height, Some(1));
        let asset_key = image_key.as_ref().expect("image key set");
        let asset = document.assets.get(asset_key).expect("asset present");
        assert!(!asset.data.is_empty());
    }

    #[test]
    fn empty_document_background_layer_has_identity_scale() {
        let document = empty_raster_document();
        let json = document_sync_json(&document);
        assert!(json.contains(r#""scaleX":1.0"#), "expected identity scale in {json}");
        assert!(json.contains(r#""scaleY":1.0"#), "expected identity scale in {json}");
        assert!(!json.contains(r#""scaleX":0.0"#), "layer must not collapse to zero size");
    }

    #[test]
    fn renders_layers_tree() {
        let mut app = semio_app();
        let json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_LAYERS, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains("\"type\":\"tree\""));
        assert!(json.contains("Backdrop"));
    }

    #[test]
    fn raster_labels_resolve_native_english_by_default() {
        let mut app = testkit::new_app::<RasterPlayApp>();
        let layers_json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_LAYERS, None, &ViewState::default()).expect("render")).unwrap();
        assert!(layers_json.contains("Add Pixel"));
        assert!(layers_json.contains("Add Group"));
        let masks_json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_MASKS, None, &ViewState::default()).expect("render")).unwrap();
        assert!(masks_json.contains("Masks"));
        assert!(masks_json.contains("No masks"));
        let catalogue_json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render")).unwrap();
        assert!(catalogue_json.contains("Layer kinds"));
        let properties_json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_PROPERTIES, None, &ViewState::default()).expect("render")).unwrap();
        assert!(properties_json.contains("Schema:"));
    }

    #[test]
    fn raster_labels_resolve_german_locale() {
        let mut app = testkit::new_app::<RasterPlayApp>();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let layers_json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_LAYERS, None, &view_state).expect("render")).unwrap();
        assert!(layers_json.contains("Pixel hinzufügen"));
        assert!(layers_json.contains("Gruppe hinzufügen"));
        let masks_json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_MASKS, None, &view_state).expect("render")).unwrap();
        assert!(masks_json.contains("Masken"));
        assert!(masks_json.contains("Keine Masken"));
        let catalogue_json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_CATALOGUE, None, &view_state).expect("render")).unwrap();
        assert!(catalogue_json.contains("Ebenenarten"));
    }

    #[test]
    fn composite_scene_syncs_document_and_assets() {
        let mut app = semio_app();
        let json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains("\"componentKind\":\"raster\""));
        assert!(json.contains("\"viewMode\":\"composite\""));
        assert!(!json.contains("\"assetsJson\":\"{}\""), "semio fixture has embedded assets");
        let document: RasterDocument = serde_json::from_str(SEMIO_EXAMPLE_JSON).unwrap();
        let sync_json = document_sync_json(&document);
        assert!(!sync_json.contains("\"assets\""), "sync json must omit assets");
        assert!(!sync_json.contains("\"camera\""), "sync json must omit camera");
    }

    #[test]
    fn set_hover_highlights_layer_row_via_runtime() {
        let mut app = semio_app();
        let layer_id = layer_node_id(&app.projection().expect("projection").layers[0]).to_string();
        let result = app.handle_action("setHover", Some(&json!({ "id": layer_id })), &ViewState::default(), &testkit::meta("local")).expect("hover");
        assert!(result.operations.is_empty(), "hover is a view action and emits no ops");
        let json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_LAYERS, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains("\"highlightedIds\":[\"raster-play-layers."));
    }

    #[test]
    fn set_composite_viewport_feeds_navigator_scene() {
        let mut app = testkit::new_app::<RasterPlayApp>();
        app.handle_action("setCompositeViewport", Some(&json!({ "width": 640.0, "height": 480.0 })), &ViewState::default(), &testkit::meta("local")).expect("viewport");
        let json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_NAVIGATOR, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains("compositeViewportJson"));
        assert!(json.contains(r#"\"width\":640.0"#));
        assert!(json.contains(r#"\"height\":480.0"#));
    }

    #[test]
    fn add_layer_action_appends_and_undo_removes() {
        let mut app = testkit::new_app::<RasterPlayApp>();
        let before = app.projection().expect("projection").layers.len();
        app.handle_action("addLayer", Some(&json!({ "kind": "group" })), &ViewState::default(), &testkit::meta("local")).expect("add");
        let projection = app.projection().expect("projection");
        assert_eq!(projection.layers.len(), before + 1);
        assert!(matches!(projection.layers.last().unwrap(), RasterLayerNode::Group { .. }));
        app.handle_action("undo", None, &ViewState::default(), &testkit::meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").layers.len(), before);
    }

    #[test]
    fn patch_layer_renames_and_toggles_visibility_round_trip() {
        let mut app = testkit::new_app::<RasterPlayApp>();
        let layer_id = layer_node_id(&app.projection().expect("projection").layers[0]).to_string();
        app.handle_action("patchLayer", Some(&json!({ "layerId": layer_id, "field": "name", "value": "Renamed" })), &ViewState::default(), &testkit::meta("local")).expect("rename");
        assert_eq!(layer_name(&app.projection().expect("projection").layers[0]), "Renamed");
        app.handle_action("toggleLayerVisible", Some(&json!({ "layerId": layer_id })), &ViewState::default(), &testkit::meta("local")).expect("toggle");
        assert!(!layer_visible(&app.projection().expect("projection").layers[0]));
        app.handle_action("undo", None, &ViewState::default(), &testkit::meta("local")).expect("undo toggle");
        assert!(layer_visible(&app.projection().expect("projection").layers[0]));
    }

    #[test]
    fn move_layer_into_group() {
        let mut app = testkit::new_app::<RasterPlayApp>();
        app.handle_action("addLayer", Some(&json!({ "kind": "group" })), &ViewState::default(), &testkit::meta("local")).expect("add group");
        let (group_id, pixel_id) = {
            let projection = app.projection().expect("projection");
            let group = projection.layers.iter().find(|layer| matches!(layer, RasterLayerNode::Group { .. })).unwrap();
            let pixel = projection.layers.iter().find(|layer| matches!(layer, RasterLayerNode::Pixel { .. })).unwrap();
            (layer_node_id(group).to_string(), layer_node_id(pixel).to_string())
        };
        let target_row = format!("{RASTER_TREE_PREFIX}.group.{group_id}");
        app.handle_action("moveLayer", Some(&json!({ "layerId": pixel_id, "targetRowId": target_row })), &ViewState::default(), &testkit::meta("local")).expect("move");
        let projection = app.projection().expect("projection");
        let RasterLayerNode::Group { children, .. } = projection.layers.iter().find(|layer| layer_node_id(layer) == group_id).unwrap() else {
            panic!("expected group");
        };
        assert_eq!(children.len(), 1);
        assert_eq!(layer_node_id(&children[0]), pixel_id);
    }

    /// 🧪 The definitional merge proof: A adds a layer while B renames the background layer — disjoint
    /// tree edits on one backbone that must both survive on both instances.
    #[test]
    fn two_instances_converge_disjoint_layer_edits_via_backbone() {
        let mut instance_a = testkit::new_app::<RasterPlayApp>();
        let mut instance_b = testkit::new_app::<RasterPlayApp>();
        // Seed both from an identical base projection (a background layer with a fixed id) so B's
        // rename targets the same layer A holds — per-instance `initial_projection` mints fresh ids.
        let mut base = empty_raster_projection();
        base.layers = vec![RasterLayerNode::Pixel {
            id: "bg".into(),
            name: "Background".into(),
            visible: true,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: RasterTransform::default(),
            mask: None,
            width: Some(512),
            height: Some(512),
            image_key: None,
        }];
        let base_envelope = serde_json::to_string(&vcs::create_document_vcs_envelope::<RasterDocument, RasterOp>(
            RASTER_DOCUMENT_SCHEMA,
            "raster",
            base,
            None,
        ))
        .unwrap();
        instance_a.load_document(&base_envelope).expect("load a");
        instance_b.load_document(&base_envelope).expect("load b");
        let background_id = "bg".to_string();
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://raster-convergence", "mem://raster-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        instance_a.handle_action("addLayer", Some(&json!({ "kind": "pixel" })), &ViewState::default(), &testkit::meta("actor-a")).expect("a adds layer");
        instance_b.handle_action("patchLayer", Some(&json!({ "layerId": background_id, "field": "name", "value": "Renamed By B" })), &ViewState::default(), &testkit::meta("actor-b")).expect("b renames");

        instance_a.handle_action("commitCheckpoint", None, &ViewState::default(), &testkit::meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &ViewState::default(), &testkit::meta("actor-b")).expect("pump b");

        let projection_a = instance_a.projection().expect("projection a");
        let projection_b = instance_b.projection().expect("projection b");
        assert_eq!(projection_a.layers.len(), 2, "A keeps its added layer");
        assert_eq!(projection_b.layers.len(), 2, "B converges on A's added layer");
        assert_eq!(layer_name(&projection_a.layers[0]), "Renamed By B", "A converges on B's rename");
        assert_eq!(layer_name(&projection_b.layers[0]), "Renamed By B", "B keeps its rename");
    }

    #[test]
    fn ingest_operations_is_idempotent() {
        testkit::assert_ingest_idempotent::<RasterPlayApp, usize>(
            "addLayer",
            Some(&json!({ "kind": "pixel" })),
            |app| app.projection().unwrap().layers.len(),
        );
    }

    #[test]
    fn set_active_utility_switch_emits_no_ops_and_reads_from_view_state() {
        let mut app = testkit::new_app_with_registry::<RasterPlayApp>(create_raster_app);
        let before = app.projection().expect("projection");
        let view = ViewState { active_utility_id: Some("paintBrush".into()), ..ViewState::default() };
        // Switching utilities is the framework View action: no document ops, nothing to sync/undo.
        let result = app
            .handle_action(SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "paintBrush" })), &view, &testkit::meta("local"))
            .expect("switch utility");
        assert!(result.operations.is_empty(), "utility switching never emits document ops");
        assert_eq!(app.projection().expect("projection"), before, "utility switching does not mutate the document");
        // The composite scene reads the host-owned active utility from session view state, not the runtime.
        let json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_COMPOSITE, None, &view).expect("render")).unwrap();
        assert!(json.contains("\"activeUtility\":\"paintBrush\""), "scene reflects host-owned active utility: {json}");
    }

    #[test]
    fn utility_registry_declares_utilities_scoped_to_the_composite_window() {
        let definition = create_raster_app().definition;
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(utility_ids, ["selectMarquee", "paintBrush", "paintEraser"]);
        // The marquee carries the Selection category; the paint utilities are Tools.
        let selects: Vec<&str> = definition.utilities.iter().filter(|utility| utility.category == Some(UtilityCategory::Selection)).map(|utility| utility.id.as_str()).collect();
        assert_eq!(selects, ["selectMarquee"]);
        let composite = definition.window_kinds.iter().find(|window| window.id == RASTER_PLAY_WINDOW_COMPOSITE).expect("composite window");
        assert_eq!(composite.utilities.len(), definition.utilities.len(), "every utility is scoped to the composite window kind");
        // The framework auto-injects the setActiveUtility View action once utilities are declared; no doc op survives.
        assert!(definition.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID && matches!(action.kind, ActionKind::View)));
        assert!(!definition.actions.iter().any(|action| action.id == "setActiveUtility" && !matches!(action.kind, ActionKind::View)));
    }
}
//#endregion 🧪Tests
