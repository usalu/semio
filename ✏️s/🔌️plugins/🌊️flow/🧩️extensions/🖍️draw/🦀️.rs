//! 🖊️ Flow draw module: 2D vector-graphics operators backed by [`flow_extension_sdk::DrawingStore`].

use flow_extension_sdk::with_drawing_kernel as with_kernel;
use flow_extension_sdk::{DrawingHandle, DrawingKernel, DrawingStore, FillStyle, GradientStop, LineCap, LineJoin, StrokeStyle};
use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, FieldSpec, Operator, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType};
use semio_framework_2d::{DrawingError, Vec2};

// #region 🔖️Helpers

fn map_kernel_error(error: DrawingError) -> EvalError {
    EvalError::InvalidInput(error.to_string())
}

fn kind_label(kind: flow_extension_sdk::DrawingKind) -> &'static str {
    match kind {
        flow_extension_sdk::DrawingKind::Rect => "rect",
        flow_extension_sdk::DrawingKind::Ellipse => "ellipse",
        flow_extension_sdk::DrawingKind::Circle => "circle",
        flow_extension_sdk::DrawingKind::Line => "line",
        flow_extension_sdk::DrawingKind::Polygon => "polygon",
        flow_extension_sdk::DrawingKind::Path => "path",
        flow_extension_sdk::DrawingKind::Text => "text",
        flow_extension_sdk::DrawingKind::Group => "group",
    }
}

fn drawing_dict(kernel: &DrawingStore, handle: &DrawingHandle) -> Result<Dictionary, EvalError> {
    let kind = kernel.kind(handle).map_err(map_kernel_error)?;
    Ok(Dictionary::with_schema("draw.drawing").insert("handle", Value::Atom(Atom::String(handle.as_str().to_string()))).insert("kind", Value::Atom(Atom::String(kind_label(kind).into()))))
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
            Ok([dict.get("x").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0), dict.get("y").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0)])
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

#[allow(
    clippy::too_many_arguments,
    reason = "positional operator-metadata builder mirroring this file's registration table shape (id/name/abbr/icon/summary/inputs/outputs/group columns); restructuring into a params struct would only churn call sites with no behavior change"
)]
fn operator_info(id: &str, name: &str, abbr: &str, icon: &str, summary: &str, inputs: Vec<ChannelSpec>, outputs: Vec<ChannelSpec>, group: &[&str]) -> OperatorInfo {
    OperatorInfo {
        id: id.into(),
        extension: "draw".into(),
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

// #endregion 🔖️Helpers

// #region 🔖️ShapeMutations
struct ShapeRect;
impl Operator for ShapeRect {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let x = read_channel_number(input, "x")?;
            let y = read_channel_number(input, "y")?;
            let width = read_channel_number(input, "width")?;
            let height = read_channel_number(input, "height")?;
            let handle = k.rect(x, y, width, height).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct ShapeEllipse;
impl Operator for ShapeEllipse {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let cx = read_channel_number(input, "cx")?;
            let cy = read_channel_number(input, "cy")?;
            let rx = read_channel_number(input, "rx")?;
            let ry = read_channel_number(input, "ry")?;
            let handle = k.ellipse(cx, cy, rx, ry).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct ShapeCircle;
impl Operator for ShapeCircle {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let cx = read_channel_number(input, "cx")?;
            let cy = read_channel_number(input, "cy")?;
            let r = read_channel_number(input, "r")?;
            let handle = k.circle(cx, cy, r).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct ShapeLine;
impl Operator for ShapeLine {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let x1 = read_channel_number(input, "x1")?;
            let y1 = read_channel_number(input, "y1")?;
            let x2 = read_channel_number(input, "x2")?;
            let y2 = read_channel_number(input, "y2")?;
            let handle = k.line(x1, y1, x2, y2).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct ShapePolygon;
impl Operator for ShapePolygon {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let points = read_point_list(input, "points")?;
            let handle = k.polygon(&points).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖️ShapeMutations

// #region 🔖️PathMutations
struct PathPolyline;
impl Operator for PathPolyline {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let points = read_point_list(input, "points")?;
            let handle = k.polyline_path(&points).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct PathRect;
impl Operator for PathRect {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let x = read_channel_number(input, "x")?;
            let y = read_channel_number(input, "y")?;
            let width = read_channel_number(input, "width")?;
            let height = read_channel_number(input, "height")?;
            let handle = k.rect_path(x, y, width, height).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖️PathMutations

// #region 🔖️StyleMutations
struct StyleFill;
impl Operator for StyleFill {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let drawing = read_drawing(input, "drawing")?;
            let color = read_rgba(input, "color")?;
            let handle = k.set_fill(&drawing, FillStyle::Solid { color }).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct StyleStroke;
impl Operator for StyleStroke {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let drawing = read_drawing(input, "drawing")?;
            let color = read_rgba(input, "color")?;
            let width = read_channel_number(input, "width").unwrap_or(1.0);
            let stroke = StrokeStyle { color, width, cap: LineCap::Butt, join: LineJoin::Miter, dash: Vec::new() };
            let handle = k.set_stroke(&drawing, stroke).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖️StyleMutations

// #region 🔖️XformMutations
struct XformTranslate;
impl Operator for XformTranslate {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let drawing = read_drawing(input, "drawing")?;
            let dx = read_channel_number(input, "dx")?;
            let dy = read_channel_number(input, "dy")?;
            let handle = k.translate(&drawing, dx, dy).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct XformRotate;
impl Operator for XformRotate {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let drawing = read_drawing(input, "drawing")?;
            let angle = read_channel_number(input, "angle")?;
            let handle = k.rotate(&drawing, angle).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct XformScale;
impl Operator for XformScale {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let drawing = read_drawing(input, "drawing")?;
            let sx = read_channel_number(input, "sx")?;
            let sy = read_channel_number(input, "sy").unwrap_or(sx);
            let handle = k.scale(&drawing, sx, sy).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖️XformMutations

// #region 🔖️GroupMutations
struct GroupMerge;
impl Operator for GroupMerge {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let a = read_drawing(input, "a")?;
            let b = read_drawing(input, "b")?;
            let handle = k.group(&[a, b]).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖️GroupMutations

// #region 🔖️BoolMutations
struct BoolUnion;
impl Operator for BoolUnion {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let a = read_drawing(input, "a")?;
            let b = read_drawing(input, "b")?;
            let handle = k.bool_union(&a, &b).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct BoolDifference;
impl Operator for BoolDifference {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let a = read_drawing(input, "a")?;
            let b = read_drawing(input, "b")?;
            let handle = k.bool_difference(&a, &b).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}

struct BoolIntersection;
impl Operator for BoolIntersection {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let a = read_drawing(input, "a")?;
            let b = read_drawing(input, "b")?;
            let handle = k.bool_intersection(&a, &b).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖️BoolMutations

// #region 🔖️TextMutations
struct DrawText;
impl Operator for DrawText {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let x = read_channel_number(input, "x")?;
            let y = read_channel_number(input, "y")?;
            let content = read_text(input, "text")?;
            let size = read_channel_number(input, "size").unwrap_or(16.0);
            let handle = k.text(x, y, &content, size).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖️TextMutations

// #region 🔖️GradientMutations
struct GradientLinear;
impl Operator for GradientLinear {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let drawing = read_drawing(input, "drawing")?;
            let x1 = read_channel_number(input, "x1")?;
            let y1 = read_channel_number(input, "y1")?;
            let x2 = read_channel_number(input, "x2")?;
            let y2 = read_channel_number(input, "y2")?;
            let stops = vec![GradientStop { offset: 0.0, color: read_rgba(input, "start")? }, GradientStop { offset: 1.0, color: read_rgba(input, "end")? }];
            let handle = k.linear_gradient_fill(&drawing, x1, y1, x2, y2, &stops).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖️GradientMutations

// #region 🔖️ClipMutations
struct ClipApply;
impl Operator for ClipApply {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let target = read_drawing(input, "target")?;
            let clip = read_drawing(input, "clip")?;
            let handle = k.apply_clip(&target, &clip).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖️ClipMutations

/// 📦️ Registers all draw operators.
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
        operator_info(
            "draw.shape.rect",
            "Rect",
            "Rct",
            "emoji:▭️",
            "Axis-aligned rectangle",
            vec![number_channel("x", "draw.shape.rect", 0.0), number_channel("y", "draw.shape.rect", 0.0), number_channel("width", "draw.shape.rect", 10.0), number_channel("height", "draw.shape.rect", 10.0)],
            vec![out_drawing("Rectangle")],
            shape,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(ShapeRect) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.shape.ellipse",
            "Ellipse",
            "Ell",
            "emoji:⬭️",
            "Ellipse",
            vec![number_channel("cx", "draw.shape.ellipse", 0.0), number_channel("cy", "draw.shape.ellipse", 0.0), number_channel("rx", "draw.shape.ellipse", 10.0), number_channel("ry", "draw.shape.ellipse", 5.0)],
            vec![out_drawing("Ellipse")],
            shape,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(ShapeEllipse) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.shape.circle",
            "Circle",
            "Cir",
            "emoji:⚪️",
            "Circle",
            vec![number_channel("cx", "draw.shape.circle", 0.0), number_channel("cy", "draw.shape.circle", 0.0), number_channel("r", "draw.shape.circle", 5.0)],
            vec![out_drawing("Circle")],
            shape,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(ShapeCircle) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.shape.line",
            "Line",
            "Lin",
            "emoji:╱️",
            "Line segment",
            vec![number_channel("x1", "draw.shape.line", 0.0), number_channel("y1", "draw.shape.line", 0.0), number_channel("x2", "draw.shape.line", 10.0), number_channel("y2", "draw.shape.line", 10.0)],
            vec![out_drawing("Line")],
            shape,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(ShapeLine) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.shape.polygon", "Polygon", "Pol", "emoji:⬡️", "Closed polygon", vec![list_channel("points", "draw.shape.polygon")], vec![out_drawing("Polygon")], shape),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(ShapePolygon) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.path.polyline", "Polyline", "Pln", "emoji:〰", "Open polyline path", vec![list_channel("points", "draw.path.polyline")], vec![out_drawing("PolylinePath")], paths),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(PathPolyline) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.path.rect",
            "Rect Path",
            "Rph",
            "emoji:▭️",
            "Rectangle path",
            vec![number_channel("x", "draw.path.rect", 0.0), number_channel("y", "draw.path.rect", 0.0), number_channel("width", "draw.path.rect", 10.0), number_channel("height", "draw.path.rect", 10.0)],
            vec![out_drawing("RectPath")],
            paths,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(PathRect) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.style.fill",
            "Fill",
            "Fil",
            "emoji:🪣️",
            "Solid fill",
            vec![
                drawing_channel("drawing", "draw.style.fill"),
                number_channel("colorR", "draw.style.fill", 1.0),
                number_channel("colorG", "draw.style.fill", 1.0),
                number_channel("colorB", "draw.style.fill", 1.0),
                number_channel("colorA", "draw.style.fill", 1.0),
            ],
            vec![out_drawing("FilledDrawing")],
            style,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(StyleFill) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.style.stroke",
            "Stroke",
            "Str",
            "emoji:🖌️",
            "Stroke outline",
            vec![
                drawing_channel("drawing", "draw.style.stroke"),
                number_channel("width", "draw.style.stroke", 1.0),
                number_channel("colorR", "draw.style.stroke", 0.0),
                number_channel("colorG", "draw.style.stroke", 0.0),
                number_channel("colorB", "draw.style.stroke", 0.0),
                number_channel("colorA", "draw.style.stroke", 1.0),
            ],
            vec![out_drawing("StrokedDrawing")],
            style,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(StyleStroke) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.xform.translate",
            "Translate",
            "Trn",
            "emoji:↔",
            "Translate drawing",
            vec![drawing_channel("drawing", "draw.xform.translate"), number_channel("dx", "draw.xform.translate", 0.0), number_channel("dy", "draw.xform.translate", 0.0)],
            vec![out_drawing("TranslatedDrawing")],
            xform,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(XformTranslate) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.xform.rotate", "Rotate", "Rot", "emoji:🔄️", "Rotate drawing", vec![drawing_channel("drawing", "draw.xform.rotate"), number_channel("angle", "draw.xform.rotate", 0.0)], vec![out_drawing("RotatedDrawing")], xform),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(XformRotate) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.xform.scale",
            "Scale",
            "Scl",
            "emoji:↕️",
            "Scale drawing",
            vec![drawing_channel("drawing", "draw.xform.scale"), number_channel("sx", "draw.xform.scale", 1.0), number_channel("sy", "draw.xform.scale", 1.0)],
            vec![out_drawing("ScaledDrawing")],
            xform,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(XformScale) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.group.merge", "Merge", "Mrg", "emoji:🗂️", "Merge drawings into a group", vec![drawing_channel("a", "draw.group.merge"), drawing_channel("b", "draw.group.merge")], vec![out_drawing("MergedGroup")], group),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(GroupMerge) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.bool.union", "Union", "Uni", "emoji:∪", "Boolean union", vec![drawing_channel("a", "draw.bool.union"), drawing_channel("b", "draw.bool.union")], vec![out_drawing("UnionDrawing")], boolean),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(BoolUnion) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.bool.difference", "Difference", "Dif", "emoji:−", "Boolean difference", vec![drawing_channel("a", "draw.bool.difference"), drawing_channel("b", "draw.bool.difference")], vec![out_drawing("DifferenceDrawing")], boolean),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(BoolDifference) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.bool.intersection",
            "Intersection",
            "Int",
            "emoji:∩",
            "Boolean intersection",
            vec![drawing_channel("a", "draw.bool.intersection"), drawing_channel("b", "draw.bool.intersection")],
            vec![out_drawing("IntersectionDrawing")],
            boolean,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(BoolIntersection) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.text",
            "Text",
            "Txt",
            "emoji:🔤️",
            "Text label",
            vec![number_channel("x", "draw.text", 0.0), number_channel("y", "draw.text", 0.0), text_channel("text", "draw.text"), number_channel("size", "draw.text", 16.0)],
            vec![out_drawing("TextDrawing")],
            text,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(DrawText) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info(
            "draw.gradient.linear",
            "Linear Gradient",
            "Lgr",
            "emoji:🌈️",
            "Linear gradient fill",
            vec![
                drawing_channel("drawing", "draw.gradient.linear"),
                number_channel("x1", "draw.gradient.linear", 0.0),
                number_channel("y1", "draw.gradient.linear", 0.0),
                number_channel("x2", "draw.gradient.linear", 10.0),
                number_channel("y2", "draw.gradient.linear", 0.0),
                number_channel("startR", "draw.gradient.linear", 1.0),
                number_channel("startG", "draw.gradient.linear", 0.0),
                number_channel("startB", "draw.gradient.linear", 0.0),
                number_channel("startA", "draw.gradient.linear", 1.0),
                number_channel("endR", "draw.gradient.linear", 0.0),
                number_channel("endG", "draw.gradient.linear", 0.0),
                number_channel("endB", "draw.gradient.linear", 1.0),
                number_channel("endA", "draw.gradient.linear", 1.0),
            ],
            vec![out_drawing("GradientDrawing")],
            gradient,
        ),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(GradientLinear) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.clip.apply", "Clip", "Clp", "emoji:✂️", "Apply clip path", vec![drawing_channel("target", "draw.clip.apply"), drawing_channel("clip", "draw.clip.apply")], vec![out_drawing("ClippedDrawing")], clip),
        vec![OperatorImpl { schemas: vec![], operator: Box::new(ClipApply) }],
        &["draw.drawing"],
    );
    registry.finalize();
}

// #region 🔖️Manifest
/// 📦️ Flow extension manifest JSON contributed to host catalogues.
pub fn extension_manifest_json() -> String {
    use flow_extension_sdk::build_manifest_json;
    build_manifest_json("draw", "Draw", "0.1.0", &neural_engine::ColdOwner::new(module_registry()), vec!["onStartup".into()], vec![], vec![], vec![])
}

/// 🌊️ Builds an in-process operator registry for this extension.
pub fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖️Manifest

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

// #region 🔖️ExtensionGuest
#[cfg(feature = "component-guest")]
mod extension_guest {
    use super::module_registry;
    use flow_extension_sdk::{build_manifest_json, evaluate_invoke_json, flow_extension_topic_contribution};
    use semio_framework::{Fault, FaultCode, FaultOrigin};
    use semio_framework_plugin::{ExecutionMode, ExtensionBundle};

    const FLOW_APP_ID: &str = "flow-play";
    const PROCEDURAL3D_APP_ID: &str = "procedural3d-play";
    const EXTENSION_ID: &str = "draw";
    const EXTENSION_LABEL: &str = "Draw";

    // 🚫️async: E1 pure — `extension_exports!` calls `bundle` outside an async context (macro requires
    // a plain sync fn). `.mode`/`.contributes_topic`/`.handler` are still `async fn` in
    // `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (out of this packet's
    // path_scope); bridged via `semio_framework::io::resolve_ready` — see this packet's lease-request.
    // See R9.
    fn bundle() -> ExtensionBundle {
        let manifest_json = build_manifest_json("draw", "Draw", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![]);
        let flow_topic = flow_extension_topic_contribution(FLOW_APP_ID, EXTENSION_ID, EXTENSION_LABEL, "draw", &manifest_json);
        let procedural3d_topic = flow_extension_topic_contribution(PROCEDURAL3D_APP_ID, EXTENSION_ID, EXTENSION_LABEL, "draw", &manifest_json);
        let bundle = ExtensionBundle::new("flow-extension-draw", "Draw", "0.1.0").extends("flow");
        let bundle = bundle.mode(ExecutionMode::Linked);
        let bundle = bundle.contributes_topic(flow_topic.topic, flow_topic.payload);
        let bundle = bundle.contributes_topic(procedural3d_topic.topic, procedural3d_topic.payload);
        bundle.handler("evaluate", |req| {
            evaluate_invoke_json(&module_registry(), req).map_err(|err| Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.evaluate.bad-request"), err))
        })
    }

    #[cfg(test)]
    include!("🧪️tests/🔬️extension-guest-standalone/🦀️.rs");

    semio_framework_plugin::extension_exports!(bundle);
}
// #endregion 🔖️ExtensionGuest
