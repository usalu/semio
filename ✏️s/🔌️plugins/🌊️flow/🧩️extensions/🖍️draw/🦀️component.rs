//! 🖊️ Flow draw module: 2D vector-graphics operators backed by [`semio_s_2d::DrawingStore`].

use semio_s_2d::{block_on, DrawingError, DrawingHandle, DrawingStore, FillStyle, GradientStop, LineCap, LineJoin, StrokeStyle, Vec2};
use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, FieldSpec, Operation, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType};
use flow_extension_sdk::with_drawing_kernel as with_kernel;

// #region 🔖️Helpers

fn map_kernel_error(error: DrawingError) -> EvalError {
    EvalError::InvalidInput(error.to_string())
}

fn kind_label(kind: semio_s_2d::DrawingKind) -> &'static str {
    match kind {
        semio_s_2d::DrawingKind::Rect => "rect",
        semio_s_2d::DrawingKind::Ellipse => "ellipse",
        semio_s_2d::DrawingKind::Circle => "circle",
        semio_s_2d::DrawingKind::Line => "line",
        semio_s_2d::DrawingKind::Polygon => "polygon",
        semio_s_2d::DrawingKind::Path => "path",
        semio_s_2d::DrawingKind::Text => "text",
        semio_s_2d::DrawingKind::Group => "group",
    }
}

fn drawing_dict(kernel: &DrawingStore, handle: &DrawingHandle) -> Result<Dictionary, EvalError> {
    let kind = block_on(kernel.kind(handle)).map_err(map_kernel_error)?;
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

#[cfg(any(test, feature = "component-guest"))]
fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖️Helpers

// #region 🔖️ShapeOperations
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
// #endregion 🔖️ShapeOperations

// #region 🔖️PathOperations
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
// #endregion 🔖️PathOperations

// #region 🔖️StyleOperations
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
// #endregion 🔖️StyleOperations

// #region 🔖️XformOperations
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
// #endregion 🔖️XformOperations

// #region 🔖️GroupOperations
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
// #endregion 🔖️GroupOperations

// #region 🔖️BoolOperations
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
// #endregion 🔖️BoolOperations

// #region 🔖️TextOperations
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
// #endregion 🔖️TextOperations

// #region 🔖️GradientOperations
struct GradientLinear;
impl Operation for GradientLinear {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|k| {
            let drawing = read_drawing(input, "drawing")?;
            let x1 = read_channel_number(input, "x1")?;
            let y1 = read_channel_number(input, "y1")?;
            let x2 = read_channel_number(input, "x2")?;
            let y2 = read_channel_number(input, "y2")?;
            let stops = vec![GradientStop { offset: 0.0, color: read_rgba(input, "start")? }, GradientStop { offset: 1.0, color: read_rgba(input, "end")? }];
            let handle = block_on(k.linear_gradient_fill(&drawing, x1, y1, x2, y2, &stops)).map_err(map_kernel_error)?;
            Ok(channel_output("draw.drawing", drawing_dict(k, &handle)?))
        })
    }
}
// #endregion 🔖️GradientOperations

// #region 🔖️ClipOperations
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
// #endregion 🔖️ClipOperations

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
        vec![OperatorImpl { schemas: vec![], operation: Box::new(ShapeRect) }],
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
        vec![OperatorImpl { schemas: vec![], operation: Box::new(ShapeEllipse) }],
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
        vec![OperatorImpl { schemas: vec![], operation: Box::new(ShapeCircle) }],
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
        vec![OperatorImpl { schemas: vec![], operation: Box::new(ShapeLine) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.shape.polygon", "Polygon", "Pol", "emoji:⬡️", "Closed polygon", vec![list_channel("points", "draw.shape.polygon")], vec![out_drawing("Polygon")], shape),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(ShapePolygon) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.path.polyline", "Polyline", "Pln", "emoji:〰", "Open polyline path", vec![list_channel("points", "draw.path.polyline")], vec![out_drawing("PolylinePath")], paths),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(PathPolyline) }],
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
        vec![OperatorImpl { schemas: vec![], operation: Box::new(PathRect) }],
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
        vec![OperatorImpl { schemas: vec![], operation: Box::new(StyleFill) }],
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
        vec![OperatorImpl { schemas: vec![], operation: Box::new(StyleStroke) }],
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
        vec![OperatorImpl { schemas: vec![], operation: Box::new(XformTranslate) }],
        &["draw.drawing"],
    );
    registry.register_operator(
        operator_info("draw.xform.rotate", "Rotate", "Rot", "emoji:🔄️", "Rotate drawing", vec![drawing_channel("drawing", "draw.xform.rotate"), number_channel("angle", "draw.xform.rotate", 0.0)], vec![out_drawing("RotatedDrawing")], xform),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(XformRotate) }],
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
        vec![OperatorImpl { schemas: vec![], operation: Box::new(BoolIntersection) }],
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
        vec![OperatorImpl { schemas: vec![], operation: Box::new(DrawText) }],
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


// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow_extension_sdk::{
        boolean_segments_json, build_manifest_json, dispose_drawing, export_dwg_json, export_pdf_json, export_svg_json, import_dwg_json, render_scene_json,
        retain_drawing_handles, trace_bitmap_json,
    };

    fn number_dictionary(value: f64) -> Dictionary {
        Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value)))
    }

    /// 🚦️ The module kernel is a single process-wide static, so any test that creates a handle and later
    /// looks it up must hold a read lock — otherwise a concurrently-running `retain_drawing_handles` test
    /// (which purges every handle outside its live set) can dispose it mid-test.
    static KERNEL_TEST_LOCK: std::sync::RwLock<()> = std::sync::RwLock::new(());

    fn kernel_read_guard() -> std::sync::RwLockReadGuard<'static, ()> {
        KERNEL_TEST_LOCK.read().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn kernel_write_guard() -> std::sync::RwLockWriteGuard<'static, ()> {
        KERNEL_TEST_LOCK.write().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

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
    fn dwg_export_import_round_trips_a_rect() {
        let _guard = kernel_read_guard();
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("x", Value::Dictionary(number_dictionary(0.0)))
            .insert("y", Value::Dictionary(number_dictionary(0.0)))
            .insert("width", Value::Dictionary(number_dictionary(5.0)))
            .insert("height", Value::Dictionary(number_dictionary(5.0)));
        let out = reg.dispatch("draw.shape.rect", &input).unwrap();
        let handle = out.get("draw.drawing").and_then(|v| v.as_dictionary()).and_then(|d| d.get("handle")).and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap();

        let export_json: serde_json::Value = serde_json::from_str(&export_dwg_json(handle)).unwrap();
        let data = export_json.get("dwg").and_then(|v| v.as_str()).expect("dwg base64");
        assert!(!data.is_empty());

        let import_json: serde_json::Value = serde_json::from_str(&import_dwg_json(data)).unwrap();
        let imported_handle = import_json.get("handle").and_then(|v| v.as_str()).expect("imported handle");
        let scene_json = render_scene_json(imported_handle);
        assert!(scene_json.contains("nodes"));
    }

    #[test]
    fn render_scene_json_returns_nodes() {
        let _guard = kernel_read_guard();
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

    fn point_list(points: &[(f64, f64)]) -> Dictionary {
        points
            .iter()
            .enumerate()
            .fold(Dictionary::with_schema("list"), |list, (index, (x, y))| list.insert(index.to_string(), Value::Dictionary(Dictionary::new().insert("x", Value::Atom(Atom::Decimal(*x))).insert("y", Value::Atom(Atom::Decimal(*y))))))
    }

    fn drawing_handle_of(output: &Dictionary) -> String {
        output.get("draw.drawing").and_then(|v| v.as_dictionary()).and_then(|d| d.get("handle")).and_then(|v| v.as_atom()).and_then(|a| a.as_str()).expect("handle").to_string()
    }

    fn drawing_kind_of(output: &Dictionary) -> String {
        output.get("draw.drawing").and_then(|v| v.as_dictionary()).and_then(|d| d.get("kind")).and_then(|v| v.as_atom()).and_then(|a| a.as_str()).expect("kind").to_string()
    }

    fn with_drawing(input: Dictionary, key: &str, handle: &str) -> Dictionary {
        input.insert(key, Value::Dictionary(Dictionary::new().insert("handle", Value::Atom(Atom::String(handle.to_string())))))
    }

    fn drawing_input(key: &str, handle: &str) -> Dictionary {
        with_drawing(Dictionary::new(), key, handle)
    }

    fn make_rect(x: f64, y: f64, width: f64, height: f64) -> String {
        let input = Dictionary::new()
            .insert("x", Value::Dictionary(number_dictionary(x)))
            .insert("y", Value::Dictionary(number_dictionary(y)))
            .insert("width", Value::Dictionary(number_dictionary(width)))
            .insert("height", Value::Dictionary(number_dictionary(height)));
        drawing_handle_of(&ShapeRect.evaluate(&input).unwrap())
    }

    #[test]
    fn ellipse_operator_creates_drawing() {
        let input = Dictionary::new()
            .insert("cx", Value::Dictionary(number_dictionary(5.0)))
            .insert("cy", Value::Dictionary(number_dictionary(5.0)))
            .insert("rx", Value::Dictionary(number_dictionary(3.0)))
            .insert("ry", Value::Dictionary(number_dictionary(2.0)));
        let out = ShapeEllipse.evaluate(&input).unwrap();
        assert_eq!(drawing_kind_of(&out), "ellipse");
    }

    #[test]
    fn circle_operator_creates_drawing() {
        let input = Dictionary::new().insert("cx", Value::Dictionary(number_dictionary(0.0))).insert("cy", Value::Dictionary(number_dictionary(0.0))).insert("r", Value::Dictionary(number_dictionary(4.0)));
        let out = ShapeCircle.evaluate(&input).unwrap();
        assert_eq!(drawing_kind_of(&out), "circle");
    }

    #[test]
    fn line_operator_creates_drawing() {
        let input = Dictionary::new()
            .insert("x1", Value::Dictionary(number_dictionary(0.0)))
            .insert("y1", Value::Dictionary(number_dictionary(0.0)))
            .insert("x2", Value::Dictionary(number_dictionary(10.0)))
            .insert("y2", Value::Dictionary(number_dictionary(10.0)));
        let out = ShapeLine.evaluate(&input).unwrap();
        assert_eq!(drawing_kind_of(&out), "line");
    }

    #[test]
    fn polygon_operator_creates_drawing_from_points() {
        let input = Dictionary::new().insert("points", Value::Dictionary(point_list(&[(0.0, 0.0), (10.0, 0.0), (5.0, 10.0)])));
        let out = ShapePolygon.evaluate(&input).unwrap();
        assert_eq!(drawing_kind_of(&out), "polygon");
    }

    #[test]
    fn polygon_operator_errors_with_fewer_than_three_points() {
        let input = Dictionary::new().insert("points", Value::Dictionary(point_list(&[(0.0, 0.0), (10.0, 0.0)])));
        assert!(matches!(ShapePolygon.evaluate(&input), Err(EvalError::InvalidInput(_))));
    }

    #[test]
    fn polyline_path_operator_creates_open_path() {
        let input = Dictionary::new().insert("points", Value::Dictionary(point_list(&[(0.0, 0.0), (10.0, 0.0), (10.0, 10.0)])));
        let out = PathPolyline.evaluate(&input).unwrap();
        assert_eq!(drawing_kind_of(&out), "path");
    }

    #[test]
    fn polyline_path_operator_errors_with_fewer_than_two_points() {
        let input = Dictionary::new().insert("points", Value::Dictionary(point_list(&[(0.0, 0.0)])));
        assert!(matches!(PathPolyline.evaluate(&input), Err(EvalError::InvalidInput(_))));
    }

    #[test]
    fn rect_path_operator_creates_path() {
        let input = Dictionary::new()
            .insert("x", Value::Dictionary(number_dictionary(0.0)))
            .insert("y", Value::Dictionary(number_dictionary(0.0)))
            .insert("width", Value::Dictionary(number_dictionary(5.0)))
            .insert("height", Value::Dictionary(number_dictionary(5.0)));
        let out = PathRect.evaluate(&input).unwrap();
        assert_eq!(drawing_kind_of(&out), "path");
    }

    #[test]
    fn fill_operator_applies_solid_color() {
        let _guard = kernel_read_guard();
        let handle = make_rect(0.0, 0.0, 5.0, 5.0);
        let input = with_drawing(Dictionary::new(), "drawing", &handle)
            .insert("colorR", Value::Dictionary(number_dictionary(1.0)))
            .insert("colorG", Value::Dictionary(number_dictionary(0.5)))
            .insert("colorB", Value::Dictionary(number_dictionary(0.25)))
            .insert("colorA", Value::Dictionary(number_dictionary(1.0)));
        let out = StyleFill.evaluate(&input).unwrap();
        assert_eq!(drawing_kind_of(&out), "rect");
    }

    #[test]
    fn stroke_operator_defaults_width_when_missing() {
        let _guard = kernel_read_guard();
        let handle = make_rect(0.0, 0.0, 5.0, 5.0);
        let out = StyleStroke.evaluate(&drawing_input("drawing", &handle)).unwrap();
        assert_eq!(drawing_kind_of(&out), "rect");
    }

    #[test]
    fn translate_operator_moves_drawing() {
        let _guard = kernel_read_guard();
        let handle = make_rect(0.0, 0.0, 5.0, 5.0);
        let input = drawing_input("drawing", &handle).insert("dx", Value::Dictionary(number_dictionary(3.0))).insert("dy", Value::Dictionary(number_dictionary(-2.0)));
        let out = XformTranslate.evaluate(&input).unwrap();
        assert_ne!(drawing_handle_of(&out), handle);
    }

    #[test]
    fn rotate_operator_rotates_drawing() {
        let _guard = kernel_read_guard();
        let handle = make_rect(0.0, 0.0, 5.0, 5.0);
        let input = drawing_input("drawing", &handle).insert("angle", Value::Dictionary(number_dictionary(45.0)));
        let out = XformRotate.evaluate(&input).unwrap();
        assert_ne!(drawing_handle_of(&out), handle);
    }

    #[test]
    fn scale_operator_defaults_sy_to_sx_when_missing() {
        let _guard = kernel_read_guard();
        let handle = make_rect(0.0, 0.0, 5.0, 5.0);
        let input = drawing_input("drawing", &handle).insert("sx", Value::Dictionary(number_dictionary(2.0)));
        let out = XformScale.evaluate(&input).unwrap();
        assert_ne!(drawing_handle_of(&out), handle);
    }

    #[test]
    fn group_merge_operator_combines_two_drawings() {
        let _guard = kernel_read_guard();
        let a = make_rect(0.0, 0.0, 5.0, 5.0);
        let b = make_rect(10.0, 10.0, 5.0, 5.0);
        let input = with_drawing(drawing_input("a", &a), "b", &b);
        let out = GroupMerge.evaluate(&input).unwrap();
        assert_eq!(drawing_kind_of(&out), "group");
    }

    #[test]
    fn bool_union_operator_combines_two_drawings() {
        let _guard = kernel_read_guard();
        let a = make_rect(0.0, 0.0, 5.0, 5.0);
        let b = make_rect(2.0, 2.0, 5.0, 5.0);
        let input = with_drawing(drawing_input("a", &a), "b", &b);
        let out = BoolUnion.evaluate(&input).unwrap();
        assert_eq!(drawing_kind_of(&out), "path");
    }

    #[test]
    fn bool_difference_operator_combines_two_drawings() {
        let _guard = kernel_read_guard();
        let a = make_rect(0.0, 0.0, 5.0, 5.0);
        let b = make_rect(2.0, 2.0, 5.0, 5.0);
        let input = with_drawing(drawing_input("a", &a), "b", &b);
        let out = BoolDifference.evaluate(&input).unwrap();
        assert_eq!(drawing_kind_of(&out), "path");
    }

    #[test]
    fn bool_intersection_operator_combines_two_drawings() {
        let _guard = kernel_read_guard();
        let a = make_rect(0.0, 0.0, 5.0, 5.0);
        let b = make_rect(2.0, 2.0, 5.0, 5.0);
        let input = with_drawing(drawing_input("a", &a), "b", &b);
        let out = BoolIntersection.evaluate(&input).unwrap();
        assert_eq!(drawing_kind_of(&out), "path");
    }

    #[test]
    fn text_operator_creates_drawing_with_default_size() {
        let input =
            Dictionary::new().insert("x", Value::Dictionary(number_dictionary(0.0))).insert("y", Value::Dictionary(number_dictionary(0.0))).insert("text", Value::Dictionary(Dictionary::new().insert("value", Value::Atom(Atom::String("hi".into())))));
        let out = DrawText.evaluate(&input).unwrap();
        assert_eq!(drawing_kind_of(&out), "text");
    }

    #[test]
    fn gradient_linear_operator_creates_drawing() {
        let _guard = kernel_read_guard();
        let handle = make_rect(0.0, 0.0, 5.0, 5.0);
        let mut input = drawing_input("drawing", &handle);
        for key in ["x1", "y1", "x2", "y2"] {
            input = input.insert(key, Value::Dictionary(number_dictionary(0.0)));
        }
        for key in ["startR", "startG", "startB", "startA", "endR", "endG", "endB", "endA"] {
            input = input.insert(key, Value::Dictionary(number_dictionary(1.0)));
        }
        let out = GradientLinear.evaluate(&input).unwrap();
        assert_eq!(drawing_kind_of(&out), "rect");
    }

    #[test]
    fn clip_apply_operator_creates_drawing() {
        let _guard = kernel_read_guard();
        let target = make_rect(0.0, 0.0, 10.0, 10.0);
        let clip = make_rect(2.0, 2.0, 4.0, 4.0);
        let input = with_drawing(drawing_input("target", &target), "clip", &clip);
        let out = ClipApply.evaluate(&input).unwrap();
        assert_eq!(drawing_kind_of(&out), "rect");
    }

    #[test]
    fn export_svg_json_returns_svg_for_known_handle() {
        let _guard = kernel_read_guard();
        let handle = make_rect(0.0, 0.0, 5.0, 5.0);
        let json: serde_json::Value = serde_json::from_str(&export_svg_json(&handle)).unwrap();
        assert!(json.get("svg").and_then(|v| v.as_str()).is_some_and(|svg| svg.contains("svg")));
    }

    #[test]
    fn export_svg_json_returns_error_for_unknown_handle() {
        let json: serde_json::Value = serde_json::from_str(&export_svg_json("drawing-missing-999")).unwrap();
        assert!(json.get("error").is_some());
    }

    #[test]
    fn export_pdf_json_returns_pdf_for_known_handle() {
        let _guard = kernel_read_guard();
        let handle = make_rect(0.0, 0.0, 5.0, 5.0);
        let json: serde_json::Value = serde_json::from_str(&export_pdf_json(&handle)).unwrap();
        assert!(json.get("pdf").and_then(|v| v.as_str()).is_some_and(|pdf| !pdf.is_empty()));
    }

    #[test]
    fn export_pdf_json_returns_error_for_unknown_handle() {
        let json: serde_json::Value = serde_json::from_str(&export_pdf_json("drawing-missing-999")).unwrap();
        assert!(json.get("error").is_some());
    }

    #[test]
    fn render_scene_json_returns_error_for_unknown_handle() {
        let json: serde_json::Value = serde_json::from_str(&render_scene_json("drawing-missing-999")).unwrap();
        assert!(json.get("error").is_some());
    }

    #[test]
    fn import_dwg_json_rejects_invalid_base64() {
        let json: serde_json::Value = serde_json::from_str(&import_dwg_json("not-@@-base64!!")).unwrap();
        assert!(json.get("error").and_then(|v| v.as_str()).unwrap_or_default().contains("base64"));
    }

    #[test]
    fn dispose_drawing_removes_the_handle() {
        let _guard = kernel_read_guard();
        let handle = make_rect(0.0, 0.0, 5.0, 5.0);
        dispose_drawing(&handle);
        let json: serde_json::Value = serde_json::from_str(&render_scene_json(&handle)).unwrap();
        assert!(json.get("error").is_some());
    }

    #[test]
    fn retain_drawing_handles_disposes_unreferenced_drawings() {
        let _guard = kernel_write_guard();
        let kept = make_rect(0.0, 0.0, 5.0, 5.0);
        let dropped = make_rect(1.0, 1.0, 5.0, 5.0);
        retain_drawing_handles(&[kept.clone()]);
        let kept_json: serde_json::Value = serde_json::from_str(&render_scene_json(&kept)).unwrap();
        let dropped_json: serde_json::Value = serde_json::from_str(&render_scene_json(&dropped)).unwrap();
        assert!(kept_json.get("nodes").is_some());
        assert!(dropped_json.get("error").is_some());
    }

    #[test]
    fn trace_bitmap_json_returns_segments_for_a_filled_mask() {
        let mask = vec![255u8; 16];
        let json: serde_json::Value = serde_json::from_str(&trace_bitmap_json(4, 4, &mask, 0.5, 0.0)).unwrap();
        assert!(json.get("segments").is_some());
    }

    #[test]
    fn boolean_segments_json_unions_two_traced_masks() {
        let mask = vec![255u8; 16];
        let segments_json = trace_bitmap_json(4, 4, &mask, 0.5, 0.0);
        let result: serde_json::Value = serde_json::from_str(&boolean_segments_json(&segments_json, &segments_json, "union")).unwrap();
        assert!(result.get("segments").is_some());
    }

    #[test]
    fn boolean_segments_json_reports_malformed_json_input() {
        let result: serde_json::Value = serde_json::from_str(&boolean_segments_json("not json", "{}", "union")).unwrap();
        assert!(result.get("error").is_some());
    }

    #[test]
    fn boolean_segments_json_propagates_upstream_error() {
        let upstream_error = serde_json::json!({ "error": "upstream boom" }).to_string();
        let result: serde_json::Value = serde_json::from_str(&boolean_segments_json(&upstream_error, "{\"segments\":[]}", "union")).unwrap();
        assert_eq!(result.get("error").and_then(|v| v.as_str()), Some("upstream boom"));
    }

    #[test]
    fn boolean_segments_json_reports_missing_segments_field() {
        let result: serde_json::Value = serde_json::from_str(&boolean_segments_json("{}", "{\"segments\":[]}", "union")).unwrap();
        assert_eq!(result.get("error").and_then(|v| v.as_str()), Some("missing segments"));
    }

    #[test]
    fn read_channel_number_errors_when_key_missing() {
        let input = Dictionary::new();
        assert!(matches!(read_channel_number(&input, "x"), Err(EvalError::MissingInput(ref key)) if key == "x"));
    }

    #[test]
    fn read_text_errors_when_key_missing() {
        let input = Dictionary::new();
        assert!(matches!(read_text(&input, "text"), Err(EvalError::MissingInput(ref key)) if key == "text"));
    }

    #[test]
    fn read_drawing_errors_when_handle_missing() {
        let input = Dictionary::new().insert("drawing", Value::Dictionary(Dictionary::new()));
        assert!(matches!(read_drawing(&input, "drawing"), Err(EvalError::MissingInput(ref key)) if key == "drawing.handle"));
    }

    #[test]
    fn read_point_list_errors_when_entry_is_not_a_point() {
        let list = Dictionary::with_schema("list").insert("0", Value::Atom(Atom::Decimal(1.0)));
        let input = Dictionary::new().insert("points", Value::Dictionary(list));
        assert!(matches!(read_point_list(&input, "points"), Err(EvalError::InvalidInput(_))));
    }

    #[test]
    fn bundle_contributes_draw_for_flow_and_procedural3d_play() {
        use flow_extension_sdk::{build_manifest_json, evaluate_json};
        use semio_framework_core::Contribution;
        use semio_framework_plugin::{extension_activate, extension_invoke, extension_manifest, install_extension_bundle, ExtensionBundle};

        let manifest_json = build_manifest_json("draw", "Draw", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![]);
        let bundle = ExtensionBundle::new("draw", "Draw", "0.1.0")
            .extends("flow")
            .contributes(Contribution::FlowExtension {
                app_id: "flow-play".into(),
                extension_id: "draw".into(),
                label: "Draw".into(),
                icon_id: "draw".into(),
                manifest_json: manifest_json.clone(),
            })
            .contributes(Contribution::FlowExtension {
                app_id: "procedural3d-play".into(),
                extension_id: "draw".into(),
                label: "Draw".into(),
                icon_id: "draw".into(),
                manifest_json,
            })
            .handler("evaluate", |req| {
                #[derive(serde::Deserialize)]
                #[serde(rename_all = "camelCase")]
                struct EvaluateRequest {
                    operator_id: String,
                    input_json: String,
                }
                let request: EvaluateRequest = serde_json::from_slice(req).unwrap();
                Ok(evaluate_json(&module_registry(), &request.operator_id, &request.input_json).into_bytes())
            });
        let installed = install_extension_bundle(bundle).expect("install");
        assert_eq!(installed.contributions.len(), 2);
        assert!(matches!(installed.contributions[0], Contribution::FlowExtension { .. }));
        assert!(matches!(installed.contributions[1], Contribution::FlowExtension { .. }));
        let _ = extension_manifest();
        extension_activate().expect("activate");
        let _ = extension_invoke;
    }

}
// #endregion 🔖️Tests


// #region 🔖️ExtensionGuest
#[cfg(feature = "component-guest")]
mod extension_guest {
    use super::module_registry;
    use flow_extension_sdk::{build_manifest_json, evaluate_json};
    use semio_framework_core::{Contribution, Fault, FaultCode, FaultOrigin};
    use semio_framework_plugin::ExtensionBundle;
    use serde::Deserialize;

    const FLOW_APP_ID: &str = "flow-play";
    const PROCEDURAL3D_APP_ID: &str = "procedural3d-play";

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct EvaluateRequest {
        operator_id: String,
        input_json: String,
    }

    fn flow_extension_contribution(app_id: &str, manifest_json: String) -> Contribution {
        Contribution::FlowExtension {
            app_id: app_id.into(),
            extension_id: "draw".into(),
            label: "Draw".into(),
            icon_id: "draw".into(),
            manifest_json,
        }
    }

    fn bundle() -> ExtensionBundle {
        let manifest_json = build_manifest_json("draw", "Draw", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![]);
        ExtensionBundle::new("draw", "Draw", "0.1.0")
            .extends("flow")
            .contributes(flow_extension_contribution(FLOW_APP_ID, manifest_json.clone()))
            .contributes(flow_extension_contribution(PROCEDURAL3D_APP_ID, manifest_json))
            .handler("evaluate", |req| {
                let request: EvaluateRequest = serde_json::from_slice(req).map_err(|err| {
                    Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.evaluate.bad-request"), err.to_string())
                })?;
                Ok(evaluate_json(&module_registry(), &request.operator_id, &request.input_json).into_bytes())
            })
    }

    semio_framework_plugin::extension_exports!(bundle);
}
// #endregion 🔖️ExtensionGuest
