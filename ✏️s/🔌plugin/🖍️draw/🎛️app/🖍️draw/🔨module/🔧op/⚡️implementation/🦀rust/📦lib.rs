//! ⚡ Draw app — operation enum + laws (constitutional: op).

use draw::{DrawCamera, DrawDocument, DrawLayerNode, DrawTextBody, DrawTransform, DrawTraceParams, FillStyle, PathSegment, StrokeStyle};
use draw_engine::{
    clone_draw_layer_node, extract_layer_node, find_draw_layer, find_draw_layer_location, hex_to_rgba, insert_layer, layer_base, layer_base_mut, mutate_draw_layer,
    remove_layer_from_tree, update_layer_in_tree,
};
use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};

//#region 🔖Types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum DrawOperation {
    SetLayerVisible {
        layer_id: String,
        visible: bool,
    },
    SetLayerLocked {
        layer_id: String,
        locked: bool,
    },
    SetLayerOpacity {
        layer_id: String,
        opacity: f64,
    },
    SetLayerBlendMode {
        layer_id: String,
        blend_mode: String,
    },
    SetLayerName {
        layer_id: String,
        name: String,
    },
    SetLayerTransform {
        layer_id: String,
        #[dsl(block)]
        transform: DrawTransform,
    },
    SetFill {
        layer_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[dsl(statements, block)]
        fill: Option<FillStyle>,
    },
    SetStroke {
        layer_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[dsl(block)]
        stroke: Option<StrokeStyle>,
    },
    SetBooleanOperation {
        layer_id: String,
        boolean_operation: String,
    },
    SetTraceParams {
        layer_id: String,
        #[dsl(block)]
        params: DrawTraceParams,
    },
    AddLayer {
        #[serde(skip_serializing_if = "Option::is_none")]
        parent_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<usize>,
        #[dsl(statements)]
        layer: Box<DrawLayerNode>,
    },
    DuplicateLayer {
        layer_id: String,
    },
    RemoveLayer {
        layer_id: String,
    },
    ReorderLayer {
        layer_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        parent_id: Option<String>,
        index: usize,
    },
    SetCamera {
        #[dsl(block)]
        camera: DrawCamera,
    },
    SetDocument {
        #[dsl(block)]
        document: DrawDocument,
    },
}

fn apply_draw_edit_operation(doc: &DrawDocument, edit: &DrawOperation) -> DrawDocument {
    match edit {
        DrawOperation::SetDocument { document } => document.clone(),
        DrawOperation::SetCamera { camera } => DrawDocument { camera: camera.clone(), ..doc.clone() },
        DrawOperation::SetLayerVisible { layer_id, visible } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).visible = *visible;
        }),
        DrawOperation::SetLayerLocked { layer_id, locked } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).locked = *locked;
        }),
        DrawOperation::SetLayerOpacity { layer_id, opacity } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).opacity = *opacity;
        }),
        DrawOperation::SetLayerBlendMode { layer_id, blend_mode } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).blend_mode = blend_mode.clone();
        }),
        DrawOperation::SetLayerName { layer_id, name } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).name = name.clone();
        }),
        DrawOperation::SetLayerTransform { layer_id, transform } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).transform = transform.clone();
        }),
        DrawOperation::SetFill { layer_id, fill } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).attributes.fill = fill.clone();
        }),
        DrawOperation::SetStroke { layer_id, stroke } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).attributes.stroke = stroke.clone();
        }),
        DrawOperation::SetBooleanOperation { layer_id, boolean_operation } => mutate_draw_layer(doc, layer_id, |layer| {
            if let DrawLayerNode::Boolean(boolean) = layer {
                boolean.operation = boolean_operation.clone();
            }
        }),
        DrawOperation::SetTraceParams { layer_id, params } => mutate_draw_layer(doc, layer_id, |layer| {
            if let DrawLayerNode::Trace(trace) = layer {
                trace.params = params.clone();
            }
        }),
        DrawOperation::AddLayer { parent_id, index, layer } => {
            let mut next = doc.clone();
            let at = index.unwrap_or(next.layers.len());
            insert_layer(&mut next.layers, parent_id.as_deref(), at, layer.as_ref().clone());
            next
        }
        DrawOperation::DuplicateLayer { layer_id: source_id } => {
            if let Some(layer) = find_draw_layer(doc, source_id).cloned() {
                let duplicate = clone_draw_layer_node(&layer, " copy");
                let mut next = doc.clone();
                if let Some(location) = find_draw_layer_location(doc, source_id) {
                    insert_layer(&mut next.layers, location.parent_id.as_deref(), location.index + 1, duplicate);
                } else {
                    next.layers.push(duplicate);
                }
                next
            } else {
                doc.clone()
            }
        }
        DrawOperation::RemoveLayer { layer_id } => {
            let mut next = doc.clone();
            remove_layer_from_tree(&mut next.layers, layer_id);
            next
        }
        DrawOperation::ReorderLayer { layer_id, parent_id, index } => {
            let mut next = doc.clone();
            if let Some(node) = extract_layer_node(&mut next.layers, layer_id) {
                insert_layer(&mut next.layers, parent_id.as_deref(), *index, node);
            }
            next
        }
    }
}

pub fn draw_op_for_layer_field(doc: &DrawDocument, layer_id: &str, field: &str, value: &serde_json::Value) -> Option<DrawOperation> {
    let layer = find_draw_layer(doc, layer_id)?;
    let operation = match field {
        "name" => DrawOperation::SetLayerName { layer_id: layer_id.into(), name: value.as_str().unwrap_or("").into() },
        "opacity" => DrawOperation::SetLayerOpacity { layer_id: layer_id.into(), opacity: value.as_f64().unwrap_or(1.0) },
        "visible" => DrawOperation::SetLayerVisible { layer_id: layer_id.into(), visible: value.as_bool().unwrap_or(true) },
        "locked" => DrawOperation::SetLayerLocked { layer_id: layer_id.into(), locked: value.as_bool().unwrap_or(false) },
        "blendMode" => DrawOperation::SetLayerBlendMode { layer_id: layer_id.into(), blend_mode: value.as_str().unwrap_or("normal").into() },
        "booleanOperation" => DrawOperation::SetBooleanOperation { layer_id: layer_id.into(), boolean_operation: value.as_str().unwrap_or("union").into() },
        "transformX" | "transformY" | "transformScaleX" | "transformScaleY" | "transformRotation" => {
            let mut transform = layer_base(layer).transform.clone();
            match field {
                "transformX" => transform.x = value.as_f64().unwrap_or(0.0),
                "transformY" => transform.y = value.as_f64().unwrap_or(0.0),
                "transformScaleX" => transform.scale_x = value.as_f64().unwrap_or(1.0),
                "transformScaleY" => transform.scale_y = value.as_f64().unwrap_or(1.0),
                _ => transform.rotation = value.as_f64().unwrap_or(0.0),
            }
            DrawOperation::SetLayerTransform { layer_id: layer_id.into(), transform }
        }
        "fillColor" => {
            let alpha = layer_base(layer)
                .attributes
                .fill
                .as_ref()
                .map(|fill| match fill {
                    FillStyle::Solid { color } => color[3],
                    FillStyle::LinearGradient { .. } | FillStyle::RadialGradient { .. } => 1.0,
                })
                .unwrap_or(1.0);
            DrawOperation::SetFill { layer_id: layer_id.into(), fill: Some(FillStyle::Solid { color: hex_to_rgba(value.as_str().unwrap_or("#000000"), alpha) }) }
        }
        "strokeWidth" => {
            let stroke = layer_base(layer).attributes.stroke.clone().unwrap_or(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 1.0, cap: "butt".into(), join: "miter".into(), dash: None });
            DrawOperation::SetStroke { layer_id: layer_id.into(), stroke: Some(StrokeStyle { width: value.as_f64().unwrap_or(1.0), ..stroke }) }
        }
        "traceThreshold" => {
            let DrawLayerNode::Trace(trace) = layer else { return None };
            let mut params = trace.params.clone();
            params.threshold = value.as_f64().unwrap_or(0.5);
            DrawOperation::SetTraceParams { layer_id: layer_id.into(), params }
        }
        "traceSimplify" => {
            let DrawLayerNode::Trace(trace) = layer else { return None };
            let mut params = trace.params.clone();
            params.simplify_epsilon = value.as_f64().unwrap_or(1.5);
            DrawOperation::SetTraceParams { layer_id: layer_id.into(), params }
        }
        _ => return None,
    };
    Some(operation)
}

pub fn patch_layer_field(doc: &DrawDocument, layer_id: &str, field: &str, value: &serde_json::Value) -> DrawDocument {
    match draw_op_for_layer_field(doc, layer_id, field, value) {
        Some(operation) => apply_draw_edit_operation(doc, &operation),
        None => doc.clone(),
    }
}
//#endregion 🔖Types

//#region 🔖Vcs
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawLayerBasePatch {
    pub visible: Option<bool>,
    pub locked: Option<bool>,
    pub name: Option<String>,
    pub opacity: Option<f64>,
    pub blend_mode: Option<String>,
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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawDiff {
    pub document: Option<DrawDocument>,
    pub camera: Option<DrawCamera>,
    pub layer_patches: Vec<DrawLayerTreePatch>,
    pub layers_removed: Vec<String>,
    pub layers_added: Vec<DrawLayerTreeAdd>,
}

impl OperationDiff<DrawDocument> for DrawDiff {
    fn apply(&self, projection: &DrawDocument) -> DrawDocument {
        let mut next = projection.clone();
        if let Some(document) = &self.document {
            return document.clone();
        }
        if let Some(camera) = &self.camera {
            next.camera = camera.clone();
        }
        for patch in &self.layer_patches {
            update_layer_in_tree(&mut next.layers, &patch.layer_id, &mut |layer| {
                let base = layer_base_mut(layer);
                if let Some(visible) = patch.base.visible {
                    base.visible = visible;
                }
                if let Some(locked) = patch.base.locked {
                    base.locked = locked;
                }
                if let Some(name) = &patch.base.name {
                    base.name = name.clone();
                }
                if let Some(opacity) = patch.base.opacity {
                    base.opacity = opacity;
                }
                if let Some(blend_mode) = &patch.base.blend_mode {
                    base.blend_mode = blend_mode.clone();
                }
            });
        }
        for layer_id in &self.layers_removed {
            remove_layer_from_tree(&mut next.layers, layer_id);
        }
        for add in &self.layers_added {
            let index = add.index.unwrap_or(next.layers.len());
            insert_layer(&mut next.layers, add.parent_id.as_deref(), index, add.layer.clone());
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.document.is_some() {
            *self = other;
            return;
        }
        if other.camera.is_some() {
            self.camera = other.camera;
        }
        self.layer_patches.extend(other.layer_patches);
        self.layers_removed.extend(other.layers_removed);
        self.layers_added.extend(other.layers_added);
    }
}

impl Operation<DrawDocument> for DrawOperation {
    type Diff = DrawDiff;

    fn diff(&self, _projection: &DrawDocument) -> DrawDiff {
        match self {
            DrawOperation::SetDocument { document } => DrawDiff { document: Some(document.clone()), ..Default::default() },
            DrawOperation::SetCamera { camera } => DrawDiff { camera: Some(camera.clone()), ..Default::default() },
            DrawOperation::SetLayerVisible { layer_id, visible } => DrawDiff { layer_patches: vec![DrawLayerTreePatch { layer_id: layer_id.clone(), base: DrawLayerBasePatch { visible: Some(*visible), ..Default::default() } }], ..Default::default() },
            DrawOperation::SetLayerLocked { layer_id, locked } => DrawDiff { layer_patches: vec![DrawLayerTreePatch { layer_id: layer_id.clone(), base: DrawLayerBasePatch { locked: Some(*locked), ..Default::default() } }], ..Default::default() },
            DrawOperation::SetLayerName { layer_id, name } => DrawDiff { layer_patches: vec![DrawLayerTreePatch { layer_id: layer_id.clone(), base: DrawLayerBasePatch { name: Some(name.clone()), ..Default::default() } }], ..Default::default() },
            DrawOperation::SetLayerOpacity { layer_id, opacity } => DrawDiff { layer_patches: vec![DrawLayerTreePatch { layer_id: layer_id.clone(), base: DrawLayerBasePatch { opacity: Some(*opacity), ..Default::default() } }], ..Default::default() },
            DrawOperation::SetLayerBlendMode { layer_id, blend_mode } => {
                DrawDiff { layer_patches: vec![DrawLayerTreePatch { layer_id: layer_id.clone(), base: DrawLayerBasePatch { blend_mode: Some(blend_mode.clone()), ..Default::default() } }], ..Default::default() }
            }
            DrawOperation::AddLayer { parent_id, index, layer } => DrawDiff { layers_added: vec![DrawLayerTreeAdd { parent_id: parent_id.clone(), index: *index, layer: layer.as_ref().clone() }], ..Default::default() },
            DrawOperation::RemoveLayer { layer_id } => DrawDiff { layers_removed: vec![layer_id.clone()], ..Default::default() },
            _ => DrawDiff { document: Some(apply_draw_edit_operation(_projection, self)), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &DrawDocument) -> Vec<Self> {
        vec![DrawOperation::SetDocument { document: projection.clone() }]
    }
}
//#endregion 🔖Vcs

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use draw::{DrawEllipse, DrawCircle, DrawLine, DrawPolygon, DrawShapeBody, DrawGroupBody, DrawImageAsset, DrawArtboard, GradientStop, DRAW_DOCUMENT_SCHEMA};
    use draw_engine::{
        create_draw_boolean_layer, create_draw_group_layer, create_draw_image_layer, create_draw_path_layer, create_draw_shape_layer_rect, create_draw_trace_layer,
        default_draw_document, default_layer_base, empty_draw_projection, layer_id,
    };

    fn representative_draw_document() -> DrawDocument {
        let mut assets = std::collections::BTreeMap::new();
        assets.insert("src-1".to_string(), DrawImageAsset { mime: "image/png".into(), data: "aGVsbG8=".into(), width: Some(8), height: Some(8) });

        let mut rect_shape = create_draw_shape_layer_rect("Rect");
        if let DrawLayerNode::Shape(shape) = &mut rect_shape {
            shape.base.attributes.fill = Some(FillStyle::LinearGradient {
                x1: 0.0,
                y1: 0.0,
                x2: 10.0,
                y2: 10.0,
                stops: vec![GradientStop { offset: 0.0, color: [1.0, 0.0, 0.0, 1.0] }, GradientStop { offset: 1.0, color: [0.0, 0.0, 1.0, 1.0] }],
            });
            shape.base.attributes.stroke = Some(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 1.5, cap: "round".into(), join: "round".into(), dash: Some(vec![2.0, 4.0]) });
        }
        let rect_id = layer_id(&rect_shape).to_string();

        let line_shape = DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("Line"), shape_kind: "line".into(), rect: None, ellipse: None, circle: None, line: Some(DrawLine { x1: 0.0, y1: 0.0, x2: 5.0, y2: 5.0 }), polygon: None });
        let line_id = layer_id(&line_shape).to_string();

        let polygon_shape = DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("Polygon"), shape_kind: "polygon".into(), rect: None, ellipse: None, circle: None, line: None, polygon: Some(DrawPolygon { points: vec![[0.0, 0.0], [1.0, 0.0], [0.5, 1.0]] }) });

        let mut radial_circle = DrawShapeBody { base: default_layer_base("RadialCircle"), shape_kind: "circle".into(), rect: None, ellipse: None, circle: Some(DrawCircle { cx: 1.0, cy: 2.0, r: 3.0 }), line: None, polygon: None };
        radial_circle.base.attributes.fill = Some(FillStyle::RadialGradient { cx: 1.0, cy: 2.0, r: 3.0, stops: vec![GradientStop { offset: 0.0, color: [1.0, 1.0, 1.0, 1.0] }, GradientStop { offset: 1.0, color: [0.0, 0.0, 0.0, 0.0] }] });
        let radial_circle = DrawLayerNode::Shape(radial_circle);

        let path_layer = create_draw_path_layer(
            "Path",
            vec![
                PathSegment::Move { to: [0.0, 0.0] },
                PathSegment::Line { to: [1.0, 0.0] },
                PathSegment::Quad { ctrl: [1.0, 1.0], to: [2.0, 1.0] },
                PathSegment::Cubic { ctrl1: [2.0, 2.0], ctrl2: [3.0, 2.0], to: [3.0, 3.0] },
                PathSegment::Arc { rx: 2.0, ry: 2.0, rotation: 0.0, large_arc: false, sweep: true, to: [1.0, -1.0] },
                PathSegment::Close,
            ],
        );

        let text_layer = DrawLayerNode::Text(DrawTextBody { base: default_layer_base("Label"), x: 4.0, y: 5.0, content: "semio \"draw\"\ndsl".into(), size: 12.0 });
        let image_layer = create_draw_image_layer("Image", "src-1");
        let trace_layer = create_draw_trace_layer("Trace", "src-1");
        let boolean_layer = create_draw_boolean_layer("Boolean", "xor", vec![rect_id.clone(), line_id]);

        let ellipse_shape = DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("Ellipse"), shape_kind: "ellipse".into(), rect: None, ellipse: Some(DrawEllipse { cx: 1.0, cy: 2.0, rx: 3.0, ry: 4.0 }), circle: None, line: None, polygon: None });
        let group_layer = DrawLayerNode::Group(DrawGroupBody { base: default_layer_base("Group \"nested\""), children: vec![ellipse_shape, radial_circle] });

        DrawDocument {
            schema: DRAW_DOCUMENT_SCHEMA.into(),
            id: "dsl-fixture".into(),
            title: Some("DSL Fixture \"Quotes\" \\ backslash".into()),
            camera: DrawCamera { x: 12.5, y: -3.0, zoom: 2.25 },
            layers: vec![rect_shape, line_shape, polygon_shape, path_layer, text_layer, image_layer, trace_layer, boolean_layer, group_layer],
            assets: Some(assets),
            artboard: Some(DrawArtboard { width: 640.0, height: 480.0 }),
        }
    }

    #[test]
    fn apply_add_and_patch_layer() {
        let doc = empty_draw_projection();
        let layer = create_draw_shape_layer_rect("Rect");
        let id = layer_id(&layer).to_string();
        let next = apply_draw_edit_operation(&doc, &DrawOperation::AddLayer { parent_id: None, index: None, layer: Box::new(layer) });
        assert_eq!(next.layers.len(), 2);
        let renamed = apply_draw_edit_operation(&next, &DrawOperation::SetLayerName { layer_id: id.clone(), name: "Box".into() });
        assert_eq!(find_draw_layer(&renamed, &id).map(|layer| layer_base(layer).name.as_str()), Some("Box"));
    }

    #[test]
    fn op_text_round_trips_every_draw_operation_variant() {
        store::test_support::assert_op_line_round_trip(&DrawOperation::SetLayerVisible { layer_id: "layer-1".into(), visible: false });
        store::test_support::assert_op_line_round_trip(&DrawOperation::SetLayerLocked { layer_id: "layer-1".into(), locked: true });
        store::test_support::assert_op_line_round_trip(&DrawOperation::SetLayerOpacity { layer_id: "layer-1".into(), opacity: 0.42 });
        store::test_support::assert_op_line_round_trip(&DrawOperation::SetLayerBlendMode { layer_id: "layer-1".into(), blend_mode: "multiply".into() });
        store::test_support::assert_op_line_round_trip(&DrawOperation::SetLayerName { layer_id: "layer-1".into(), name: "New \"Name\"\nline2".into() });
        store::test_support::assert_op_line_round_trip(&DrawOperation::SetLayerTransform { layer_id: "layer-1".into(), transform: DrawTransform { x: 1.0, y: -2.0, scale_x: 1.5, scale_y: 0.5, rotation: 0.3 } });
        store::test_support::assert_op_line_round_trip(&DrawOperation::SetFill { layer_id: "layer-1".into(), fill: None });
        store::test_support::assert_op_line_round_trip(&DrawOperation::SetFill { layer_id: "layer-1".into(), fill: Some(FillStyle::Solid { color: [0.1, 0.2, 0.3, 1.0] }) });
        store::test_support::assert_op_line_round_trip(&DrawOperation::SetStroke { layer_id: "layer-1".into(), stroke: None });
        store::test_support::assert_op_line_round_trip(&DrawOperation::SetStroke { layer_id: "layer-1".into(), stroke: Some(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 2.0, cap: "butt".into(), join: "bevel".into(), dash: Some(vec![1.0, 2.0, 3.0]) }) });
        store::test_support::assert_op_line_round_trip(&DrawOperation::SetBooleanOperation { layer_id: "layer-1".into(), boolean_operation: "intersection".into() });
        store::test_support::assert_op_line_round_trip(&DrawOperation::SetTraceParams { layer_id: "layer-1".into(), params: DrawTraceParams { threshold: 0.33, simplify_epsilon: 1.1 } });
        store::test_support::assert_op_line_round_trip(&DrawOperation::AddLayer { parent_id: None, index: None, layer: Box::new(create_draw_shape_layer_rect("Added")) });
        store::test_support::assert_op_line_round_trip(&DrawOperation::AddLayer { parent_id: Some("group-1".into()), index: Some(2), layer: Box::new(create_draw_group_layer("Nested")) });
        store::test_support::assert_op_line_round_trip(&DrawOperation::DuplicateLayer { layer_id: "layer-1".into() });
        store::test_support::assert_op_line_round_trip(&DrawOperation::RemoveLayer { layer_id: "layer-1".into() });
        store::test_support::assert_op_line_round_trip(&DrawOperation::ReorderLayer { layer_id: "layer-1".into(), parent_id: None, index: 0 });
        store::test_support::assert_op_line_round_trip(&DrawOperation::ReorderLayer { layer_id: "layer-1".into(), parent_id: Some("group-1".into()), index: 3 });
        store::test_support::assert_op_line_round_trip(&DrawOperation::SetCamera { camera: DrawCamera { x: 10.0, y: 20.0, zoom: 1.5 } });
        store::test_support::assert_op_line_round_trip(&DrawOperation::SetDocument { document: representative_draw_document() });
    }

    #[test]
    fn draw_op_for_layer_field_maps_every_known_field_and_rejects_unknown_field_or_missing_layer() {
        let rect = create_draw_shape_layer_rect("Rect");
        let rect_id = layer_id(&rect).to_string();
        let boolean = create_draw_boolean_layer("Bool", "union", Vec::new());
        let boolean_id = layer_id(&boolean).to_string();
        let trace = create_draw_trace_layer("Trace", "src");
        let trace_id = layer_id(&trace).to_string();
        let mut doc = default_draw_document("field-ops", None);
        doc.layers = vec![rect, boolean, trace];

        assert!(matches!(draw_op_for_layer_field(&doc, &rect_id, "name", &serde_json::json!("New")), Some(DrawOperation::SetLayerName { name, .. }) if name == "New"));
        assert!(matches!(draw_op_for_layer_field(&doc, &rect_id, "opacity", &serde_json::json!(0.4)), Some(DrawOperation::SetLayerOpacity { opacity, .. }) if (opacity - 0.4).abs() < 1e-9));
        assert!(matches!(draw_op_for_layer_field(&doc, &rect_id, "visible", &serde_json::json!(false)), Some(DrawOperation::SetLayerVisible { visible, .. }) if !visible));
        assert!(matches!(draw_op_for_layer_field(&doc, &rect_id, "locked", &serde_json::json!(true)), Some(DrawOperation::SetLayerLocked { locked, .. }) if locked));
        assert!(matches!(draw_op_for_layer_field(&doc, &rect_id, "blendMode", &serde_json::json!("multiply")), Some(DrawOperation::SetLayerBlendMode { blend_mode, .. }) if blend_mode == "multiply"));
        assert!(matches!(draw_op_for_layer_field(&doc, &boolean_id, "booleanOperation", &serde_json::json!("xor")), Some(DrawOperation::SetBooleanOperation { boolean_operation, .. }) if boolean_operation == "xor"));

        for field in ["transformX", "transformY", "transformScaleX", "transformScaleY", "transformRotation"] {
            assert!(matches!(draw_op_for_layer_field(&doc, &rect_id, field, &serde_json::json!(5.0)), Some(DrawOperation::SetLayerTransform { .. })));
        }

        assert!(matches!(draw_op_for_layer_field(&doc, &rect_id, "fillColor", &serde_json::json!("#00ff00")), Some(DrawOperation::SetFill { fill: Some(FillStyle::Solid { color }), .. }) if color == [0.0, 1.0, 0.0, 1.0]));

        doc = mutate_draw_layer(&doc, &rect_id, |layer| {
            layer_base_mut(layer).attributes.fill = Some(FillStyle::Solid { color: [0.0, 0.0, 0.0, 0.25] });
        });
        assert!(matches!(draw_op_for_layer_field(&doc, &rect_id, "fillColor", &serde_json::json!("#00ff00")), Some(DrawOperation::SetFill { fill: Some(FillStyle::Solid { color }), .. }) if color[3] == 0.25));

        doc = mutate_draw_layer(&doc, &rect_id, |layer| {
            layer_base_mut(layer).attributes.fill = Some(FillStyle::LinearGradient { x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0, stops: Vec::new() });
        });
        assert!(matches!(draw_op_for_layer_field(&doc, &rect_id, "fillColor", &serde_json::json!("#00ff00")), Some(DrawOperation::SetFill { fill: Some(FillStyle::Solid { color }), .. }) if color[3] == 1.0));

        assert!(matches!(draw_op_for_layer_field(&doc, &rect_id, "strokeWidth", &serde_json::json!(3.0)), Some(DrawOperation::SetStroke { stroke: Some(stroke), .. }) if stroke.width == 3.0 && stroke.cap == "butt"));
        doc = mutate_draw_layer(&doc, &rect_id, |layer| {
            layer_base_mut(layer).attributes.stroke = Some(StrokeStyle { color: [1.0, 1.0, 1.0, 1.0], width: 1.0, cap: "round".into(), join: "round".into(), dash: None });
        });
        assert!(matches!(draw_op_for_layer_field(&doc, &rect_id, "strokeWidth", &serde_json::json!(9.0)), Some(DrawOperation::SetStroke { stroke: Some(stroke), .. }) if stroke.width == 9.0 && stroke.cap == "round"));

        assert!(draw_op_for_layer_field(&doc, &rect_id, "traceThreshold", &serde_json::json!(0.7)).is_none());
        assert!(matches!(draw_op_for_layer_field(&doc, &trace_id, "traceThreshold", &serde_json::json!(0.7)), Some(DrawOperation::SetTraceParams { params, .. }) if params.threshold == 0.7));
        assert!(matches!(draw_op_for_layer_field(&doc, &trace_id, "traceSimplify", &serde_json::json!(2.5)), Some(DrawOperation::SetTraceParams { params, .. }) if params.simplify_epsilon == 2.5));

        assert!(draw_op_for_layer_field(&doc, &rect_id, "unknownField", &serde_json::json!(1)).is_none());
        assert!(draw_op_for_layer_field(&doc, "missing-layer", "name", &serde_json::json!("x")).is_none());
    }

    #[test]
    fn patch_layer_field_applies_mapped_field_and_returns_clone_for_unmapped_field_or_missing_layer() {
        let rect = create_draw_shape_layer_rect("Rect");
        let rect_id = layer_id(&rect).to_string();
        let mut doc = default_draw_document("patch-field", None);
        doc.layers = vec![rect];

        let patched = patch_layer_field(&doc, &rect_id, "opacity", &serde_json::json!(0.2));
        assert_eq!(find_draw_layer(&patched, &rect_id).map(|layer| layer_base(layer).opacity), Some(0.2));

        let unchanged = patch_layer_field(&doc, &rect_id, "unmapped", &serde_json::json!(1));
        assert_eq!(unchanged, doc);

        let unchanged_missing = patch_layer_field(&doc, "missing", "opacity", &serde_json::json!(0.1));
        assert_eq!(unchanged_missing, doc);
    }

    #[test]
    fn apply_draw_edit_operation_covers_remaining_variants() {
        let child = create_draw_shape_layer_rect("Child");
        let child_id = layer_id(&child).to_string();
        let mut group = create_draw_group_layer("Group");
        if let DrawLayerNode::Group(body) = &mut group {
            body.children.push(child);
        }
        let group_id = layer_id(&group).to_string();
        let mut doc = default_draw_document("apply-ops", None);
        doc.layers = vec![group];

        let with_camera = apply_draw_edit_operation(&doc, &DrawOperation::SetCamera { camera: DrawCamera { x: 5.0, y: 6.0, zoom: 2.0 } });
        assert_eq!(with_camera.camera, DrawCamera { x: 5.0, y: 6.0, zoom: 2.0 });

        let with_lock = apply_draw_edit_operation(&doc, &DrawOperation::SetLayerLocked { layer_id: child_id.clone(), locked: true });
        assert!(find_draw_layer(&with_lock, &child_id).map(|layer| layer_base(layer).locked).unwrap());

        let with_blend = apply_draw_edit_operation(&doc, &DrawOperation::SetLayerBlendMode { layer_id: child_id.clone(), blend_mode: "screen".into() });
        assert_eq!(find_draw_layer(&with_blend, &child_id).map(|layer| layer_base(layer).blend_mode.clone()), Some("screen".to_string()));

        let new_transform = DrawTransform { x: 1.0, y: 2.0, scale_x: 1.0, scale_y: 1.0, rotation: 0.0 };
        let with_transform = apply_draw_edit_operation(&doc, &DrawOperation::SetLayerTransform { layer_id: child_id.clone(), transform: new_transform.clone() });
        assert_eq!(find_draw_layer(&with_transform, &child_id).map(|layer| layer_base(layer).transform.clone()), Some(new_transform));

        let with_fill = apply_draw_edit_operation(&doc, &DrawOperation::SetFill { layer_id: child_id.clone(), fill: Some(FillStyle::Solid { color: [1.0, 0.0, 0.0, 1.0] }) });
        assert!(find_draw_layer(&with_fill, &child_id).map(|layer| layer_base(layer).attributes.fill.is_some()).unwrap());

        let boolean = create_draw_boolean_layer("Bool", "union", Vec::new());
        let boolean_id = layer_id(&boolean).to_string();
        doc.layers.push(boolean);
        let with_bool_op = apply_draw_edit_operation(&doc, &DrawOperation::SetBooleanOperation { layer_id: boolean_id.clone(), boolean_operation: "xor".into() });
        let DrawLayerNode::Boolean(bool_body) = find_draw_layer(&with_bool_op, &boolean_id).unwrap() else { panic!("expected boolean") };
        assert_eq!(bool_body.operation, "xor");
        let no_op_bool = apply_draw_edit_operation(&doc, &DrawOperation::SetBooleanOperation { layer_id: child_id.clone(), boolean_operation: "xor".into() });
        assert_eq!(no_op_bool, doc);

        let trace = create_draw_trace_layer("Trace", "src");
        let trace_id = layer_id(&trace).to_string();
        doc.layers.push(trace);
        let new_params = DrawTraceParams { threshold: 0.9, simplify_epsilon: 3.3 };
        let with_trace_params = apply_draw_edit_operation(&doc, &DrawOperation::SetTraceParams { layer_id: trace_id.clone(), params: new_params.clone() });
        let DrawLayerNode::Trace(trace_body) = find_draw_layer(&with_trace_params, &trace_id).unwrap() else { panic!("expected trace") };
        assert_eq!(trace_body.params, new_params);

        let added_layer = create_draw_shape_layer_rect("Added");
        let added_id = layer_id(&added_layer).to_string();
        let with_add = apply_draw_edit_operation(&doc, &DrawOperation::AddLayer { parent_id: Some(group_id.clone()), index: Some(0), layer: Box::new(added_layer) });
        assert!(find_draw_layer(&with_add, &added_id).is_some());
        let DrawLayerNode::Group(added_group) = find_draw_layer(&with_add, &group_id).unwrap() else { panic!("expected group") };
        assert_eq!(added_group.children.len(), 2);

        let dup_missing = apply_draw_edit_operation(&doc, &DrawOperation::DuplicateLayer { layer_id: "missing".into() });
        assert_eq!(dup_missing, doc);

        let with_dup = apply_draw_edit_operation(&doc, &DrawOperation::DuplicateLayer { layer_id: child_id.clone() });
        let DrawLayerNode::Group(dup_group) = find_draw_layer(&with_dup, &group_id).unwrap() else { panic!("expected group") };
        assert_eq!(dup_group.children.len(), 2);
        assert_ne!(layer_id(&dup_group.children[1]), child_id);

        let with_remove = apply_draw_edit_operation(&doc, &DrawOperation::RemoveLayer { layer_id: child_id.clone() });
        let DrawLayerNode::Group(remaining_group) = find_draw_layer(&with_remove, &group_id).unwrap() else { panic!("expected group") };
        assert!(remaining_group.children.is_empty());

        let with_reorder = apply_draw_edit_operation(&doc, &DrawOperation::ReorderLayer { layer_id: boolean_id.clone(), parent_id: Some(group_id.clone()), index: 0 });
        let DrawLayerNode::Group(reordered_group) = find_draw_layer(&with_reorder, &group_id).unwrap() else { panic!("expected group") };
        assert!(reordered_group.children.iter().any(|child| layer_id(child) == boolean_id));

        let reorder_missing = apply_draw_edit_operation(&doc, &DrawOperation::ReorderLayer { layer_id: "missing".into(), parent_id: None, index: 0 });
        assert_eq!(reorder_missing, doc);
    }

    #[test]
    fn draw_operation_diff_apply_absorb_and_backwards_round_trip() {
        let rect = create_draw_shape_layer_rect("Rect");
        let rect_id = layer_id(&rect).to_string();
        let mut doc = default_draw_document("diff-test", None);
        doc.layers = vec![rect];

        let add_op = DrawOperation::AddLayer { parent_id: None, index: None, layer: Box::new(create_draw_shape_layer_rect("New")) };
        let add_diff = add_op.diff(&doc);
        let after_add = add_diff.apply(&doc);
        assert_eq!(after_add.layers.len(), 2);

        let camera_op = DrawOperation::SetCamera { camera: DrawCamera { x: 3.0, y: 4.0, zoom: 1.5 } };
        let camera_diff = camera_op.diff(&doc);
        assert_eq!(camera_diff.apply(&doc).camera, DrawCamera { x: 3.0, y: 4.0, zoom: 1.5 });

        let remove_op = DrawOperation::RemoveLayer { layer_id: rect_id.clone() };
        let remove_diff = remove_op.diff(&doc);
        assert!(remove_diff.apply(&doc).layers.is_empty());

        let visible_op = DrawOperation::SetLayerVisible { layer_id: rect_id.clone(), visible: false };
        let visible_diff = visible_op.diff(&doc);
        let after_visible = visible_diff.apply(&doc);
        assert!(!find_draw_layer(&after_visible, &rect_id).map(|layer| layer_base(layer).visible).unwrap());

        let fill_op = DrawOperation::SetFill { layer_id: rect_id.clone(), fill: Some(FillStyle::Solid { color: [1.0, 1.0, 1.0, 1.0] }) };
        let fill_diff = fill_op.diff(&doc);
        assert_eq!(fill_diff.document, Some(apply_draw_edit_operation(&doc, &fill_op)));

        let backwards = fill_op.backwards(&doc);
        assert_eq!(backwards.len(), 1);
        assert!(matches!(&backwards[0], DrawOperation::SetDocument { document } if *document == doc));

        let mut absorb_target = DrawDiff {
            camera: Some(DrawCamera { x: 1.0, y: 1.0, zoom: 1.0 }),
            layer_patches: vec![DrawLayerTreePatch { layer_id: rect_id.clone(), base: DrawLayerBasePatch { visible: Some(false), ..Default::default() } }],
            ..Default::default()
        };
        let more_patches = DrawDiff { layer_patches: vec![DrawLayerTreePatch { layer_id: "other".into(), base: DrawLayerBasePatch { locked: Some(true), ..Default::default() } }], ..Default::default() };
        absorb_target.absorb(more_patches);
        assert_eq!(absorb_target.layer_patches.len(), 2);
        assert_eq!(absorb_target.camera, Some(DrawCamera { x: 1.0, y: 1.0, zoom: 1.0 }));

        let document_override = DrawDiff { document: Some(doc.clone()), ..Default::default() };
        absorb_target.absorb(document_override);
        assert_eq!(absorb_target.document, Some(doc.clone()));
        assert_eq!(absorb_target.camera, None);
    }

    #[test]
    fn draw_operation_parse_op_reports_error_for_unknown_operation_name() {
        use protocol::OpText;
        let err = DrawOperation::parse_op("bogusOperation layerId=layer-1").unwrap_err();
        assert!(err.message.contains("unknown operation line"), "unexpected error message: {}", err.message);
    }

}
//#endregion 🧪Tests
