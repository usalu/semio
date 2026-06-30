//! 🖊️ Flow draw module: 2D vector-graphics operators backed by [`geometry_drawing_rs::DrawingStore`].

use geometry_drawing_engine::{
    block_on, DrawingError, DrawingHandle, DrawingKernel, FillStyle, GradientStop, LineCap, LineJoin, StrokeStyle, Vec2,
};
use geometry_drawing_rs::DrawingStore;
use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, FieldSpec, Operation, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType};
use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

static KERNEL: OnceLock<Mutex<DrawingStore>> = OnceLock::new();

fn kernel() -> &'static Mutex<DrawingStore> {
    KERNEL.get_or_init(|| Mutex::new(DrawingStore::new()))
}

// #region 🔖Helpers
fn with_kernel<T>(f: impl FnOnce(&mut DrawingStore) -> Result<T, EvalError>) -> Result<T, EvalError> {
    let mut guard = kernel().lock().map_err(|_| EvalError::InvalidInput("draw kernel lock poisoned".into()))?;
    f(&mut guard)
}

fn map_kernel_error(error: DrawingError) -> EvalError {
    EvalError::InvalidInput(error.to_string())
}

fn kind_label(kind: geometry_drawing_engine::DrawingKind) -> &'static str {
    match kind {
        geometry_drawing_engine::DrawingKind::Rect => "rect",
        geometry_drawing_engine::DrawingKind::Ellipse => "ellipse",
        geometry_drawing_engine::DrawingKind::Circle => "circle",
        geometry_drawing_engine::DrawingKind::Line => "line",
        geometry_drawing_engine::DrawingKind::Polygon => "polygon",
        geometry_drawing_engine::DrawingKind::Path => "path",
        geometry_drawing_engine::DrawingKind::Text => "text",
        geometry_drawing_engine::DrawingKind::Group => "group",
    }
}

fn drawing_dict(kernel: &DrawingStore, handle: &DrawingHandle) -> Result<Dictionary, EvalError> {
    let kind = block_on(kernel.kind(handle)).map_err(map_kernel_error)?;
    Ok(Dictionary::with_schema("draw.drawing")
        .insert("handle", Value::Atom(Atom::String(handle.as_str().to_string())))
        .insert("kind", Value::Atom(Atom::String(kind_label(kind).into()))))
}

fn number_dictionary(value: f64) -> Dictionary {
    Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value)))
}

fn text_dictionary(value: impl Into<String>) -> Dictionary {
    Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(value.into())))
}

fn read_channel_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    dict.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_text(input: &Dictionary, key: &str) -> Result<String, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    dict.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).map(str::to_string).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_drawing(input: &Dictionary, key: &str) -> Result<DrawingHandle, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    let handle = dict.get("handle").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).ok_or_else(|| EvalError::MissingInput(format!("{key}.handle")))?;
    Ok(DrawingHandle(handle.to_string()))
}

fn read_point_list(input: &Dictionary, key: &str) -> Result<Vec<Vec2>, EvalError> {
    let list = input.get(key).and_then(|value| value.as_dictionary()).filter(|dict| dict.schema() == Some("list")).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    let mut indices: Vec<usize> = list.keys().filter_map(|key| key.parse().ok()).collect();
    indices.sort_unstable();
    indices
        .into_iter()
        .map(|index| {
            let dict = list.get(&index.to_string()).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::InvalidInput(format!("{key}[{index}] must be a point")))?;
            Ok([
                dict.get("x").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0),
                dict.get("y").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0),
            ])
        })
        .collect()
}

fn read_rgba(input: &Dictionary, key: &str) -> Result<[f64; 4], EvalError> {
    Ok([
        read_channel_number(input, &format!("{key}R")).unwrap_or(0.0),
        read_channel_number(input, &format!("{key}G")).unwrap_or(0.0),
        read_channel_number(input, &format!("{key}B")).unwrap_or(0.0),
        read_channel_number(input, &format!("{key}A")).unwrap_or(1.0),
    ])
}

fn number_channel(id: &str, operator_id: &str, default: f64) -> ChannelSpec {
    ChannelSpec::number_default(id, default, &[operator_id])
}

fn drawing_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::requires(id, &[operator_id])
}

fn list_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::list(id, &[operator_id])
}

fn text_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::text_default(id, "", &[operator_id])
}

fn out_drawing(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("D", "Drw", "draw.drawing", full_name)
}

fn operator_info(id: &str, name: &str, abbr: &str, icon: &str, summary: &str, inputs: Vec<ChannelSpec>, outputs: Vec<ChannelSpec>, group: &[&str]) -> OperatorInfo {
    OperatorInfo {
        id: id.into(),
        module: "draw".into(),
        name: name.into(),
        abbreviation: abbr.into(),
        icon: icon.into(),
        summary: summary.into(),
        inputs,
        outputs,
        group: group.iter().map(|entry| (*entry).to_string()).collect(),
        ..Default::default()
    }
}

fn drawing_schema() -> Schema {
    Schema {
        id: "draw.drawing".into(),
        module: "draw".into(),
        name: "Drawing".into(),
        icon: "emoji:🖊️".into(),
        summary: "Opaque 2D drawing handle".into(),
        fields: vec![FieldSpec::new("handle", ValueType::Text), FieldSpec::new("kind", ValueType::Text)],
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖Helpers

// #region 🔖ShapeOps
struct ShapeRect;
impl Operation for ShapeRect {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let x = read_channel_number(input, "x")?;
            let y = read_channel_number(input, "y")?;
            let width = read_channel_number(input, "width")?;
            let height = read_channel_number(input, "height")?;
            let handle = block_on(k.rect(x, y, width, height)).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct ShapeEllipse;
impl Operation for ShapeEllipse {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let cx = read_channel_number(input, "cx")?;
            let cy = read_channel_number(input, "cy")?;
            let rx = read_channel_number(input, "rx")?;
            let ry = read_channel_number(input, "ry")?;
            let handle = block_on(k.ellipse(cx, cy, rx, ry)).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct ShapeCircle;
impl Operation for ShapeCircle {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let cx = read_channel_number(input, "cx")?;
            let cy = read_channel_number(input, "cy")?;
            let r = read_channel_number(input, "r")?;
            let handle = block_on(k.circle(cx, cy, r)).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct ShapeLine;
impl Operation for ShapeLine {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let x1 = read_channel_number(input, "x1")?;
            let y1 = read_channel_number(input, "y1")?;
            let x2 = read_channel_number(input, "x2")?;
            let y2 = read_channel_number(input, "y2")?;
            let handle = block_on(k.line(x1, y1, x2, y2)).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct ShapePolygon;
impl Operation for ShapePolygon {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let points = read_point_list(input, "points")?;
            let handle = block_on(k.polygon(&points)).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖ShapeOps

// #region 🔖PathOps
struct PathPolyline;
impl Operation for PathPolyline {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let points = read_point_list(input, "points")?;
            let handle = block_on(k.polyline_path(&points)).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct PathRect;
impl Operation for PathRect {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let x = read_channel_number(input, "x")?;
            let y = read_channel_number(input, "y")?;
            let width = read_channel_number(input, "width")?;
            let height = read_channel_number(input, "height")?;
            let handle = block_on(k.rect_path(x, y, width, height)).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖PathOps

// #region 🔖StyleOps
struct StyleFill;
impl Operation for StyleFill {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let drawing = read_drawing(input, "drawing")?;
            let color = read_rgba(input, "color")?;
            let handle = block_on(k.set_fill(&drawing, FillStyle::Solid { color })).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct StyleStroke;
impl Operation for StyleStroke {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let drawing = read_drawing(input, "drawing")?;
            let color = read_rgba(input, "color")?;
            let width = read_channel_number(input, "width").unwrap_or(1.0);
            let stroke = StrokeStyle { color, width, cap: LineCap::Butt, join: LineJoin::Miter, dash: Vec::new() };
            let handle = block_on(k.set_stroke(&drawing, stroke)).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖StyleOps

// #region 🔖XformOps
struct XformTranslate;
impl Operation for XformTranslate {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let drawing = read_drawing(input, "drawing")?;
            let dx = read_channel_number(input, "dx")?;
            let dy = read_channel_number(input, "dy")?;
            let handle = block_on(k.translate(&drawing, dx, dy)).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct XformRotate;
impl Operation for XformRotate {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let drawing = read_drawing(input, "drawing")?;
            let angle = read_channel_number(input, "angle")?;
            let handle = block_on(k.rotate(&drawing, angle)).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct XformScale;
impl Operation for XformScale {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let drawing = read_drawing(input, "drawing")?;
            let sx = read_channel_number(input, "sx")?;
            let sy = read_channel_number(input, "sy").unwrap_or(sx);
            let handle = block_on(k.scale(&drawing, sx, sy)).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖XformOps

// #region 🔖GroupOps
struct GroupMerge;
impl Operation for GroupMerge {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let a = read_drawing(input, "a")?;
            let b = read_drawing(input, "b")?;
            let handle = block_on(k.group(&[a, b])).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖GroupOps

// #region 🔖BoolOps
struct BoolUnion;
impl Operation for BoolUnion {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let a = read_drawing(input, "a")?;
            let b = read_drawing(input, "b")?;
            let handle = block_on(k.bool_union(&a, &b)).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct BoolDifference;
impl Operation for BoolDifference {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let a = read_drawing(input, "a")?;
            let b = read_drawing(input, "b")?;
            let handle = block_on(k.bool_difference(&a, &b)).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct BoolIntersection;
impl Operation for BoolIntersection {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let a = read_drawing(input, "a")?;
            let b = read_drawing(input, "b")?;
            let handle = block_on(k.bool_intersection(&a, &b)).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖BoolOps

// #region 🔖TextOps
struct DrawText;
impl Operation for DrawText {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let x = read_channel_number(input, "x")?;
            let y = read_channel_number(input, "y")?;
            let content = read_text(input, "text")?;
            let size = read_channel_number(input, "size").unwrap_or(16.0);
            let handle = block_on(k.text(x, y, &content, size)).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖TextOps

// #region 🔖GradientOps
struct GradientLinear;
impl Operation for GradientLinear {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let drawing = read_drawing(input, "drawing")?;
            let x1 = read_channel_number(input, "x1")?;
            let y1 = read_channel_number(input, "y1")?;
            let x2 = read_channel_number(input, "x2")?;
            let y2 = read_channel_number(input, "y2")?;
            let stops = vec![
                GradientStop { offset: 0.0, color: read_rgba(input, "start")? },
                GradientStop { offset: 1.0, color: read_rgba(input, "end")? },
            ];
            let handle = block_on(k.linear_gradient_fill(&drawing, x1, y1, x2, y2, &stops)).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖GradientOps

// #region 🔖ClipOps
struct ClipApply;
impl Operation for ClipApply {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let target = read_drawing(input, "target")?;
            let clip = read_drawing(input, "clip")?;
            let handle = block_on(k.apply_clip(&target, &clip)).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖ClipOps

/// 📦 Registers all draw operators.
pub fn register(registry: &mut Registry) {
    registry.register_schema(drawing_schema());
    let shape = &["Shapes"];
    let paths = &["Paths"];
    let style = &["Style"];
    let xform = &["Transform"];
    let group = &["Group"];
    let boolean = &["Boolean"];
    let text = &["Text"];
    let gradient = &["Gradient"];
    let clip = &["Clip"];

    registry.register_operator(
        operator_info("draw.shape.rect", "Rect", "Rct", "emoji:▭", "Axis-aligned rectangle", vec![number_channel("x", "draw.shape.rect", 0.0), number_channel("y", "draw.shape.rect", 0.0), number_channel("width", "draw.shape.rect", 10.0), number_channel("height", "draw.shape.rect", 10.0)], vec![out_drawing("Rectangle")], shape),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(ShapeRect) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.shape.ellipse", "Ellipse", "Ell", "emoji:⬭", "Ellipse", vec![number_channel("cx", "draw.shape.ellipse", 0.0), number_channel("cy", "draw.shape.ellipse", 0.0), number_channel("rx", "draw.shape.ellipse", 10.0), number_channel("ry", "draw.shape.ellipse", 5.0)], vec![out_drawing("Ellipse")], shape),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(ShapeEllipse) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.shape.circle", "Circle", "Cir", "emoji:⚪", "Circle", vec![number_channel("cx", "draw.shape.circle", 0.0), number_channel("cy", "draw.shape.circle", 0.0), number_channel("r", "draw.shape.circle", 5.0)], vec![out_drawing("Circle")], shape),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(ShapeCircle) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.shape.line", "Line", "Lin", "emoji:╱", "Line segment", vec![number_channel("x1", "draw.shape.line", 0.0), number_channel("y1", "draw.shape.line", 0.0), number_channel("x2", "draw.shape.line", 10.0), number_channel("y2", "draw.shape.line", 10.0)], vec![out_drawing("Line")], shape),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(ShapeLine) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.shape.polygon", "Polygon", "Pol", "emoji:⬡", "Closed polygon", vec![list_channel("points", "draw.shape.polygon")], vec![out_drawing("Polygon")], shape),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(ShapePolygon) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.path.polyline", "Polyline", "Pln", "emoji:〰️", "Open polyline path", vec![list_channel("points", "draw.path.polyline")], vec![out_drawing("PolylinePath")], paths),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(PathPolyline) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.path.rect", "Rect Path", "Rph", "emoji:▭", "Rectangle path", vec![number_channel("x", "draw.path.rect", 0.0), number_channel("y", "draw.path.rect", 0.0), number_channel("width", "draw.path.rect", 10.0), number_channel("height", "draw.path.rect", 10.0)], vec![out_drawing("RectPath")], paths),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(PathRect) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.style.fill", "Fill", "Fil", "emoji:🪣", "Solid fill", vec![drawing_channel("drawing", "draw.style.fill"), number_channel("colorR", "draw.style.fill", 0.0), number_channel("colorG", "draw.style.fill", 0.0), number_channel("colorB", "draw.style.fill", 0.0), number_channel("colorA", "draw.style.fill", 1.0)], vec![out_drawing("FilledDrawing")], style),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(StyleFill) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.style.stroke", "Stroke", "Str", "emoji:🖌️", "Stroke outline", vec![drawing_channel("drawing", "draw.style.stroke"), number_channel("width", "draw.style.stroke", 1.0), number_channel("colorR", "draw.style.stroke", 0.0), number_channel("colorG", "draw.style.stroke", 0.0), number_channel("colorB", "draw.style.stroke", 0.0), number_channel("colorA", "draw.style.stroke", 1.0)], vec![out_drawing("StrokedDrawing")], style),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(StyleStroke) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.xform.translate", "Translate", "Trn", "emoji:↔️", "Translate drawing", vec![drawing_channel("drawing", "draw.xform.translate"), number_channel("dx", "draw.xform.translate", 0.0), number_channel("dy", "draw.xform.translate", 0.0)], vec![out_drawing("TranslatedDrawing")], xform),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(XformTranslate) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.xform.rotate", "Rotate", "Rot", "emoji:🔄", "Rotate drawing", vec![drawing_channel("drawing", "draw.xform.rotate"), number_channel("angle", "draw.xform.rotate", 0.0)], vec![out_drawing("RotatedDrawing")], xform),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(XformRotate) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.xform.scale", "Scale", "Scl", "emoji:↕️", "Scale drawing", vec![drawing_channel("drawing", "draw.xform.scale"), number_channel("sx", "draw.xform.scale", 1.0), number_channel("sy", "draw.xform.scale", 1.0)], vec![out_drawing("ScaledDrawing")], xform),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(XformScale) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.group.merge", "Merge", "Mrg", "emoji:🗂️", "Merge drawings into a group", vec![drawing_channel("a", "draw.group.merge"), drawing_channel("b", "draw.group.merge")], vec![out_drawing("MergedGroup")], group),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(GroupMerge) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.bool.union", "Union", "Uni", "emoji:∪", "Boolean union", vec![drawing_channel("a", "draw.bool.union"), drawing_channel("b", "draw.bool.union")], vec![out_drawing("UnionDrawing")], boolean),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(BoolUnion) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.bool.difference", "Difference", "Dif", "emoji:−", "Boolean difference", vec![drawing_channel("a", "draw.bool.difference"), drawing_channel("b", "draw.bool.difference")], vec![out_drawing("DifferenceDrawing")], boolean),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(BoolDifference) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.bool.intersection", "Intersection", "Int", "emoji:∩", "Boolean intersection", vec![drawing_channel("a", "draw.bool.intersection"), drawing_channel("b", "draw.bool.intersection")], vec![out_drawing("IntersectionDrawing")], boolean),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(BoolIntersection) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.text", "Text", "Txt", "emoji:🔤", "Text label", vec![number_channel("x", "draw.text", 0.0), number_channel("y", "draw.text", 0.0), text_channel("text", "draw.text"), number_channel("size", "draw.text", 16.0)], vec![out_drawing("TextDrawing")], text),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(DrawText) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.gradient.linear", "Linear Gradient", "Lgr", "emoji:🌈", "Linear gradient fill", vec![drawing_channel("drawing", "draw.gradient.linear"), number_channel("x1", "draw.gradient.linear", 0.0), number_channel("y1", "draw.gradient.linear", 0.0), number_channel("x2", "draw.gradient.linear", 10.0), number_channel("y2", "draw.gradient.linear", 0.0), number_channel("startR", "draw.gradient.linear", 1.0), number_channel("startG", "draw.gradient.linear", 0.0), number_channel("startB", "draw.gradient.linear", 0.0), number_channel("startA", "draw.gradient.linear", 1.0), number_channel("endR", "draw.gradient.linear", 0.0), number_channel("endG", "draw.gradient.linear", 0.0), number_channel("endB", "draw.gradient.linear", 1.0), number_channel("endA", "draw.gradient.linear", 1.0)], vec![out_drawing("GradientDrawing")], gradient),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(GradientLinear) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.clip.apply", "Clip", "Clp", "emoji:✂️", "Apply clip path", vec![drawing_channel("target", "draw.clip.apply"), drawing_channel("clip", "draw.clip.apply")], vec![out_drawing("ClippedDrawing")], clip),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(ClipApply) }],
        &["draw.drawing"],
    );
    registry.finalize();
}

// #region 🔖Scene
/// 🧹 Retains only drawing handles referenced by the current evaluation outputs.
pub fn retain_drawing_handles(live: &[String]) {
    let live_set: HashSet<String> = live.iter().cloned().collect();
    if let Ok(mut guard) = kernel().lock() {
        guard.retain_sync(&live_set);
    }
}

/// 🎬 Flattens a drawing handle to JSON scene payload.
pub fn render_scene_json(handle: &str) -> String {
    kernel()
        .lock()
        .ok()
        .and_then(|store| {
            let drawing = DrawingHandle(handle.to_string());
            match block_on(store.flatten_scene(&drawing)) {
                Ok(scene) => Some(serde_json::to_string(&scene).unwrap_or_else(|_| "{}".into())),
                Err(error) => Some(serde_json::json!({ "error": error.to_string() }).to_string()),
            }
        })
        .unwrap_or_else(|| serde_json::json!({ "error": "draw kernel unavailable" }).to_string())
}

/// 📄 Exports a drawing handle as SVG JSON wrapper.
pub fn export_svg_json(handle: &str) -> String {
    kernel()
        .lock()
        .ok()
        .and_then(|store| {
            let drawing = DrawingHandle(handle.to_string());
            match block_on(store.export_svg(&drawing)) {
                Ok(svg) => Some(serde_json::json!({ "svg": svg }).to_string()),
                Err(error) => Some(serde_json::json!({ "error": error.to_string() }).to_string()),
            }
        })
        .unwrap_or_else(|| serde_json::json!({ "error": "draw kernel unavailable" }).to_string())
}

/// 📑 Exports a drawing handle as base64 PDF JSON wrapper.
pub fn export_pdf_json(handle: &str) -> String {
    kernel()
        .lock()
        .ok()
        .and_then(|store| {
            let drawing = DrawingHandle(handle.to_string());
            match block_on(store.export_pdf(&drawing)) {
                Ok(pdf) => Some(serde_json::json!({ "pdf": base64_encode(&pdf) }).to_string()),
                Err(error) => Some(serde_json::json!({ "error": error.to_string() }).to_string()),
            }
        })
        .unwrap_or_else(|| serde_json::json!({ "error": "draw kernel unavailable" }).to_string())
}

/// 🗑️ Disposes a drawing handle owned by the in-process draw kernel.
pub fn dispose_drawing(handle: &str) {
    if let Ok(mut store) = kernel().lock() {
        block_on(store.dispose(&DrawingHandle(handle.to_string())));
    }
}

/// 🔍 Autotraces a bitmap mask into path segments JSON.
pub fn trace_bitmap_json(width: u32, height: u32, mask: &[u8], threshold: f64, simplify_epsilon: f64) -> String {
    kernel()
        .lock()
        .ok()
        .and_then(|mut store| match block_on(store.trace_bitmap(width, height, mask, threshold, simplify_epsilon)) {
            Ok(handle) => match block_on(store.flatten_scene(&handle)) {
                Ok(scene) => {
                    let segments = scene
                        .nodes
                        .into_iter()
                        .find_map(|node| if let geometry_drawing_engine::DrawingNode::Path { segments } = node.node { Some(segments) } else { None });
                    segments.map(|segs| serde_json::json!({ "segments": segs }).to_string())
                }
                Err(error) => Some(serde_json::json!({ "error": error.to_string() }).to_string()),
            },
            Err(error) => Some(serde_json::json!({ "error": error.to_string() }).to_string()),
        })
        .unwrap_or_else(|| serde_json::json!({ "error": "draw kernel unavailable" }).to_string())
}

/// 🔀 Boolean-combines two path segment arrays.
pub fn boolean_segments_json(a_json: &str, b_json: &str, op: &str) -> String {
    let parse = |json: &str| -> Result<Vec<geometry_drawing_engine::PathSegment>, String> {
        let parsed: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
        if let Some(error) = parsed.get("error").and_then(|v| v.as_str()) {
            return Err(error.to_string());
        }
        let segments_value = parsed.get("segments").cloned().ok_or_else(|| "missing segments".to_string())?;
        serde_json::from_value(segments_value).map_err(|e| e.to_string())
    };
    kernel()
        .lock()
        .ok()
        .and_then(|store| match (parse(a_json), parse(b_json)) {
            (Ok(a), Ok(b)) => match block_on(store.boolean_segments(&a, &b, op)) {
                Ok(segments) => Some(serde_json::json!({ "segments": segments }).to_string()),
                Err(error) => Some(serde_json::json!({ "error": error.to_string() }).to_string()),
            },
            (Err(error), _) | (_, Err(error)) => Some(serde_json::json!({ "error": error }).to_string()),
        })
        .unwrap_or_else(|| serde_json::json!({ "error": "draw kernel unavailable" }).to_string())
}

fn base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(TABLE[((triple >> 18) & 63) as usize] as char);
        out.push(TABLE[((triple >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 { TABLE[((triple >> 6) & 63) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { TABLE[(triple & 63) as usize] as char } else { '=' });
    }
    out
}
// #endregion 🔖Scene

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow_module_wasm::build_manifest_json;

    #[test]
    fn rect_operator_creates_drawing() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("x", Value::Dictionary(number_dictionary(0.0)))
            .insert("y", Value::Dictionary(number_dictionary(0.0)))
            .insert("width", Value::Dictionary(number_dictionary(10.0)))
            .insert("height", Value::Dictionary(number_dictionary(20.0)));
        let out = reg.dispatch("draw.shape.rect", &input).unwrap();
        let drawing = out.get("draw.drawing").and_then(|v| v.as_dictionary()).expect("drawing channel");
        assert_eq!(drawing.schema(), Some("draw.drawing"));
        assert!(drawing.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap_or("").starts_with("drawing-"));
    }

    #[test]
    fn manifest_lists_draw_operators() {
        let json = build_manifest_json("draw", "Draw", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![]);
        assert!(json.contains("draw.shape.rect"));
        assert!(json.contains("draw.drawing"));
    }

    #[test]
    fn render_scene_json_returns_nodes() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("x", Value::Dictionary(number_dictionary(0.0)))
            .insert("y", Value::Dictionary(number_dictionary(0.0)))
            .insert("width", Value::Dictionary(number_dictionary(5.0)))
            .insert("height", Value::Dictionary(number_dictionary(5.0)));
        let out = reg.dispatch("draw.shape.rect", &input).unwrap();
        let handle = out.get("draw.drawing").and_then(|v| v.as_dictionary()).and_then(|d| d.get("handle")).and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap();
        let scene_json = render_scene_json(handle);
        assert!(scene_json.contains("nodes"));
    }
}
// #endregion 🔖Tests

// #region 🔖WasmExt
#[cfg(all(target_arch = "wasm32", feature = "standalone-wasm"))]
mod wasm_ext {
    use super::module_registry;
    use flow_module_wasm::{build_manifest_json, command_json, evaluate_json};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn manifest() -> String {
        build_manifest_json("draw", "Draw", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![])
    }

    #[wasm_bindgen]
    pub fn evaluate(kind_id: &str, input_json: &str) -> String {
        evaluate_json(&module_registry(), kind_id, input_json)
    }

    #[wasm_bindgen]
    pub fn command(command_id: &str, args_json: &str) -> String {
        command_json(command_id, args_json)
    }

    #[wasm_bindgen]
    pub fn activate() {}

    #[wasm_bindgen]
    pub fn deactivate() {}

    #[wasm_bindgen]
    pub fn render_scene(handle: &str) -> String {
        super::render_scene_json(handle)
    }

    #[wasm_bindgen]
    pub fn export_svg(handle: &str) -> String {
        super::export_svg_json(handle)
    }

    #[wasm_bindgen]
    pub fn export_pdf(handle: &str) -> String {
        super::export_pdf_json(handle)
    }

    #[wasm_bindgen]
    pub fn dispose(handle: &str) {
        super::dispose_drawing(handle);
    }
}
// #endregion 🔖WasmExt
