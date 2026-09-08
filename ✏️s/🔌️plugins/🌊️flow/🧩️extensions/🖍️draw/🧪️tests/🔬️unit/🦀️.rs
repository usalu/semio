
use super::*;
use flow_extension_sdk::{boolean_segments_json, build_manifest_json, dispose_drawing, export_pdf_json, export_svg_json, render_scene_json, retain_drawing_handles, trace_bitmap_json};

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

#[semio_framework_async_macros::async_test]
async fn rect_operator_creates_drawing() {
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
    let handle = drawing.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap_or("");
    assert_eq!(handle.len(), 64, "drawing handles are the hex-encoded 32-byte content key, not a prefixed counter");
    assert!(handle.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
}

#[semio_framework_async_macros::async_test]
async fn manifest_lists_draw_operators() {
    let json = build_manifest_json("draw", "Draw", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![]);
    assert!(json.contains("draw.shape.rect"));
    assert!(json.contains("draw.drawing"));
}

#[semio_framework_async_macros::async_test]
async fn render_scene_json_returns_nodes() {
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
    points.iter().enumerate().fold(Dictionary::with_schema("list"), |list, (index, (x, y))| list.insert(index.to_string(), Value::Dictionary(Dictionary::new().insert("x", Value::Atom(Atom::Decimal(*x))).insert("y", Value::Atom(Atom::Decimal(*y))))))
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

#[semio_framework_async_macros::async_test]
async fn ellipse_operator_creates_drawing() {
    let input =
        Dictionary::new().insert("cx", Value::Dictionary(number_dictionary(5.0))).insert("cy", Value::Dictionary(number_dictionary(5.0))).insert("rx", Value::Dictionary(number_dictionary(3.0))).insert("ry", Value::Dictionary(number_dictionary(2.0)));
    let out = ShapeEllipse.evaluate(&input).unwrap();
    assert_eq!(drawing_kind_of(&out), "ellipse");
}

#[semio_framework_async_macros::async_test]
async fn circle_operator_creates_drawing() {
    let input = Dictionary::new().insert("cx", Value::Dictionary(number_dictionary(0.0))).insert("cy", Value::Dictionary(number_dictionary(0.0))).insert("r", Value::Dictionary(number_dictionary(4.0)));
    let out = ShapeCircle.evaluate(&input).unwrap();
    assert_eq!(drawing_kind_of(&out), "circle");
}

#[semio_framework_async_macros::async_test]
async fn line_operator_creates_drawing() {
    let input = Dictionary::new()
        .insert("x1", Value::Dictionary(number_dictionary(0.0)))
        .insert("y1", Value::Dictionary(number_dictionary(0.0)))
        .insert("x2", Value::Dictionary(number_dictionary(10.0)))
        .insert("y2", Value::Dictionary(number_dictionary(10.0)));
    let out = ShapeLine.evaluate(&input).unwrap();
    assert_eq!(drawing_kind_of(&out), "line");
}

#[semio_framework_async_macros::async_test]
async fn polygon_operator_creates_drawing_from_points() {
    let input = Dictionary::new().insert("points", Value::Dictionary(point_list(&[(0.0, 0.0), (10.0, 0.0), (5.0, 10.0)])));
    let out = ShapePolygon.evaluate(&input).unwrap();
    assert_eq!(drawing_kind_of(&out), "polygon");
}

#[semio_framework_async_macros::async_test]
async fn polygon_operator_errors_with_fewer_than_three_points() {
    let input = Dictionary::new().insert("points", Value::Dictionary(point_list(&[(0.0, 0.0), (10.0, 0.0)])));
    assert!(matches!(ShapePolygon.evaluate(&input), Err(EvalError::InvalidInput(_))));
}

#[semio_framework_async_macros::async_test]
async fn polyline_path_operator_creates_open_path() {
    let input = Dictionary::new().insert("points", Value::Dictionary(point_list(&[(0.0, 0.0), (10.0, 0.0), (10.0, 10.0)])));
    let out = PathPolyline.evaluate(&input).unwrap();
    assert_eq!(drawing_kind_of(&out), "path");
}

#[semio_framework_async_macros::async_test]
async fn polyline_path_operator_errors_with_fewer_than_two_points() {
    let input = Dictionary::new().insert("points", Value::Dictionary(point_list(&[(0.0, 0.0)])));
    assert!(matches!(PathPolyline.evaluate(&input), Err(EvalError::InvalidInput(_))));
}

#[semio_framework_async_macros::async_test]
async fn rect_path_operator_creates_path() {
    let input = Dictionary::new()
        .insert("x", Value::Dictionary(number_dictionary(0.0)))
        .insert("y", Value::Dictionary(number_dictionary(0.0)))
        .insert("width", Value::Dictionary(number_dictionary(5.0)))
        .insert("height", Value::Dictionary(number_dictionary(5.0)));
    let out = PathRect.evaluate(&input).unwrap();
    assert_eq!(drawing_kind_of(&out), "path");
}

#[semio_framework_async_macros::async_test]
async fn fill_operator_applies_solid_color() {
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

#[semio_framework_async_macros::async_test]
async fn stroke_operator_defaults_width_when_missing() {
    let _guard = kernel_read_guard();
    let handle = make_rect(0.0, 0.0, 5.0, 5.0);
    let out = StyleStroke.evaluate(&drawing_input("drawing", &handle)).unwrap();
    assert_eq!(drawing_kind_of(&out), "rect");
}

#[semio_framework_async_macros::async_test]
async fn translate_operator_moves_drawing() {
    let _guard = kernel_read_guard();
    let handle = make_rect(0.0, 0.0, 5.0, 5.0);
    let input = drawing_input("drawing", &handle).insert("dx", Value::Dictionary(number_dictionary(3.0))).insert("dy", Value::Dictionary(number_dictionary(-2.0)));
    let out = XformTranslate.evaluate(&input).unwrap();
    assert_ne!(drawing_handle_of(&out), handle);
}

#[semio_framework_async_macros::async_test]
async fn rotate_operator_rotates_drawing() {
    let _guard = kernel_read_guard();
    let handle = make_rect(0.0, 0.0, 5.0, 5.0);
    let input = drawing_input("drawing", &handle).insert("angle", Value::Dictionary(number_dictionary(45.0)));
    let out = XformRotate.evaluate(&input).unwrap();
    assert_ne!(drawing_handle_of(&out), handle);
}

#[semio_framework_async_macros::async_test]
async fn scale_operator_defaults_sy_to_sx_when_missing() {
    let _guard = kernel_read_guard();
    let handle = make_rect(0.0, 0.0, 5.0, 5.0);
    let input = drawing_input("drawing", &handle).insert("sx", Value::Dictionary(number_dictionary(2.0)));
    let out = XformScale.evaluate(&input).unwrap();
    assert_ne!(drawing_handle_of(&out), handle);
}

#[semio_framework_async_macros::async_test]
async fn group_merge_operator_combines_two_drawings() {
    let _guard = kernel_read_guard();
    let a = make_rect(0.0, 0.0, 5.0, 5.0);
    let b = make_rect(10.0, 10.0, 5.0, 5.0);
    let input = with_drawing(drawing_input("a", &a), "b", &b);
    let out = GroupMerge.evaluate(&input).unwrap();
    assert_eq!(drawing_kind_of(&out), "group");
}

#[semio_framework_async_macros::async_test]
async fn bool_union_operator_combines_two_drawings() {
    let _guard = kernel_read_guard();
    let a = make_rect(0.0, 0.0, 5.0, 5.0);
    let b = make_rect(2.0, 2.0, 5.0, 5.0);
    let input = with_drawing(drawing_input("a", &a), "b", &b);
    let out = BoolUnion.evaluate(&input).unwrap();
    assert_eq!(drawing_kind_of(&out), "path");
}

#[semio_framework_async_macros::async_test]
async fn bool_difference_operator_combines_two_drawings() {
    let _guard = kernel_read_guard();
    let a = make_rect(0.0, 0.0, 5.0, 5.0);
    let b = make_rect(2.0, 2.0, 5.0, 5.0);
    let input = with_drawing(drawing_input("a", &a), "b", &b);
    let out = BoolDifference.evaluate(&input).unwrap();
    assert_eq!(drawing_kind_of(&out), "path");
}

#[semio_framework_async_macros::async_test]
async fn bool_intersection_operator_combines_two_drawings() {
    let _guard = kernel_read_guard();
    let a = make_rect(0.0, 0.0, 5.0, 5.0);
    let b = make_rect(2.0, 2.0, 5.0, 5.0);
    let input = with_drawing(drawing_input("a", &a), "b", &b);
    let out = BoolIntersection.evaluate(&input).unwrap();
    assert_eq!(drawing_kind_of(&out), "path");
}

#[semio_framework_async_macros::async_test]
async fn text_operator_creates_drawing_with_default_size() {
    let input =
        Dictionary::new().insert("x", Value::Dictionary(number_dictionary(0.0))).insert("y", Value::Dictionary(number_dictionary(0.0))).insert("text", Value::Dictionary(Dictionary::new().insert("value", Value::Atom(Atom::String("hi".into())))));
    let out = DrawText.evaluate(&input).unwrap();
    assert_eq!(drawing_kind_of(&out), "text");
}

#[semio_framework_async_macros::async_test]
async fn gradient_linear_operator_creates_drawing() {
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

#[semio_framework_async_macros::async_test]
async fn clip_apply_operator_creates_drawing() {
    let _guard = kernel_read_guard();
    let target = make_rect(0.0, 0.0, 10.0, 10.0);
    let clip = make_rect(2.0, 2.0, 4.0, 4.0);
    let input = with_drawing(drawing_input("target", &target), "clip", &clip);
    let out = ClipApply.evaluate(&input).unwrap();
    assert_eq!(drawing_kind_of(&out), "rect");
}

#[semio_framework_async_macros::async_test]
async fn export_svg_json_returns_svg_for_known_handle() {
    let _guard = kernel_read_guard();
    let handle = make_rect(0.0, 0.0, 5.0, 5.0);
    let json = pack::json::parse(&export_svg_json(&handle)).unwrap();
    assert!(json.get("svg").and_then(|v| v.as_str()).is_some_and(|svg| svg.contains("svg")));
}

#[semio_framework_async_macros::async_test]
async fn export_svg_json_returns_error_for_unknown_handle() {
    let json = pack::json::parse(&export_svg_json("drawing-missing-999")).unwrap();
    assert!(json.get("error").is_some());
}

#[semio_framework_async_macros::async_test]
async fn export_pdf_json_returns_pdf_for_known_handle() {
    let _guard = kernel_read_guard();
    let handle = make_rect(0.0, 0.0, 5.0, 5.0);
    let json = pack::json::parse(&export_pdf_json(&handle)).unwrap();
    assert!(json.get("pdf").and_then(|v| v.as_str()).is_some_and(|pdf| !pdf.is_empty()));
}

#[semio_framework_async_macros::async_test]
async fn export_pdf_json_returns_error_for_unknown_handle() {
    let json = pack::json::parse(&export_pdf_json("drawing-missing-999")).unwrap();
    assert!(json.get("error").is_some());
}

#[semio_framework_async_macros::async_test]
async fn render_scene_json_returns_error_for_unknown_handle() {
    let json = pack::json::parse(&render_scene_json("drawing-missing-999")).unwrap();
    assert!(json.get("error").is_some());
}

#[semio_framework_async_macros::async_test]
async fn dispose_drawing_removes_the_handle() {
    let _guard = kernel_write_guard();
    let handle = make_rect(0.0, 0.0, 5.0, 5.0);
    dispose_drawing(&handle);
    let json = pack::json::parse(&render_scene_json(&handle)).unwrap();
    assert!(json.get("error").is_some());
}

#[semio_framework_async_macros::async_test]
async fn retain_drawing_handles_disposes_unreferenced_drawings() {
    let _guard = kernel_write_guard();
    let kept = make_rect(0.0, 0.0, 5.0, 5.0);
    let dropped = make_rect(1.0, 1.0, 5.0, 5.0);
    retain_drawing_handles(&[kept.clone()]);
    let kept_json = pack::json::parse(&render_scene_json(&kept)).unwrap();
    let dropped_json = pack::json::parse(&render_scene_json(&dropped)).unwrap();
    assert!(kept_json.get("nodes").is_some());
    assert!(dropped_json.get("error").is_some());
}

#[semio_framework_async_macros::async_test]
async fn trace_bitmap_json_returns_segments_for_a_filled_mask() {
    let mask = vec![255u8; 16];
    let json = pack::json::parse(&trace_bitmap_json(4, 4, &mask, 0.5, 0.0)).unwrap();
    assert!(json.get("segments").is_some());
}

#[semio_framework_async_macros::async_test]
async fn boolean_segments_json_unions_two_traced_masks() {
    let mask = vec![255u8; 16];
    let segments_json = trace_bitmap_json(4, 4, &mask, 0.5, 0.0);
    let result = pack::json::parse(&boolean_segments_json(&segments_json, &segments_json, "union")).unwrap();
    assert!(result.get("segments").is_some());
}

#[semio_framework_async_macros::async_test]
async fn boolean_segments_json_reports_malformed_json_input() {
    let result = pack::json::parse(&boolean_segments_json("not json", "{}", "union")).unwrap();
    assert!(result.get("error").is_some());
}

#[semio_framework_async_macros::async_test]
async fn boolean_segments_json_propagates_upstream_error() {
    let upstream_error = pack::json::to_string(&pack::json::object([("error".to_string(), pack::json::Value::from("upstream boom"))]));
    let result = pack::json::parse(&boolean_segments_json(&upstream_error, "{\"segments\":[]}", "union")).unwrap();
    assert_eq!(result.get("error").and_then(|v| v.as_str()), Some("upstream boom"));
}

#[semio_framework_async_macros::async_test]
async fn boolean_segments_json_reports_missing_segments_field() {
    let result = pack::json::parse(&boolean_segments_json("{}", "{\"segments\":[]}", "union")).unwrap();
    assert_eq!(result.get("error").and_then(|v| v.as_str()), Some("missing segments"));
}

#[semio_framework_async_macros::async_test]
async fn read_channel_number_errors_when_key_missing() {
    let input = Dictionary::new();
    assert!(matches!(read_channel_number(&input, "x"), Err(EvalError::MissingInput(ref key)) if key == "x"));
}

#[semio_framework_async_macros::async_test]
async fn read_text_errors_when_key_missing() {
    let input = Dictionary::new();
    assert!(matches!(read_text(&input, "text"), Err(EvalError::MissingInput(ref key)) if key == "text"));
}

#[semio_framework_async_macros::async_test]
async fn read_drawing_errors_when_handle_missing() {
    let input = Dictionary::new().insert("drawing", Value::Dictionary(Dictionary::new()));
    assert!(matches!(read_drawing(&input, "drawing"), Err(EvalError::MissingInput(ref key)) if key == "drawing.handle"));
}

#[semio_framework_async_macros::async_test]
async fn read_point_list_errors_when_entry_is_not_a_point() {
    let list = Dictionary::with_schema("list").insert("0", Value::Atom(Atom::Decimal(1.0)));
    let input = Dictionary::new().insert("points", Value::Dictionary(list));
    assert!(matches!(read_point_list(&input, "points"), Err(EvalError::InvalidInput(_))));
}

#[semio_framework_async_macros::async_test]
async fn bundle_contributes_draw_for_flow_and_procedural3d_play() {
    use flow_extension_sdk::build_manifest_json;
    use semio_framework_plugin::{ExtensionBundle, extension_activate, extension_invoke, extension_manifest, install_extension_bundle};

    let manifest_json = build_manifest_json("draw", "Draw", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![]);
    let flow_topic = flow_extension_sdk::flow_extension_topic_contribution("flow-play", "draw", "Draw", "draw", &manifest_json);
    let procedural3d_topic = flow_extension_sdk::flow_extension_topic_contribution("procedural3d-play", "draw", "Draw", "draw", &manifest_json);
    let bundle = ExtensionBundle::new("flow-extension-draw", "Draw", "0.1.0")
        .extends("flow")
        .contributes_topic(flow_topic.topic, flow_topic.payload)
        .contributes_topic(procedural3d_topic.topic, procedural3d_topic.payload)
        .handler("evaluate", |req| Ok(flow_extension_sdk::evaluate_invoke_json(&module_registry(), req).unwrap()));
    install_extension_bundle(bundle).await;
    let installed = extension_manifest().await;
    assert_eq!(installed.topic_contributions.len(), 2);
    assert_eq!(installed.topic_contributions[0].topic, "flow.extension");
    assert_eq!(installed.topic_contributions[1].topic, "flow.extension");
    let _ = extension_manifest().await;
    extension_activate().await.expect("activate");
    let _ = extension_invoke;
}
