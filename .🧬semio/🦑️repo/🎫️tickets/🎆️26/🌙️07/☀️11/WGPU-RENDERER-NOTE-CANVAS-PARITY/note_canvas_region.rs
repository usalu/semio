//! 📝️ Direct DrawList painting for note-canvas, ported from note-canvas-host.tsx (framework/renderer/react).

//#region NoteCanvasModel
static NOTE_HOST_ID_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

fn create_note_host_id(prefix: &str) -> String {
    let next = NOTE_HOST_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
    format!("{prefix}-host-{next}")
}

#[derive(Clone, Copy, Debug, Default)]
struct NoteCameraF {
    x: f64,
    y: f64,
    zoom: f64,
}

impl From<NoteCameraJson> for NoteCameraF {
    fn from(camera: NoteCameraJson) -> Self {
        Self { x: camera.x, y: camera.y, zoom: camera.zoom }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NoteCameraJson {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default = "note_default_zoom")]
    zoom: f64,
}

fn note_default_zoom() -> f64 {
    1.0
}

impl Default for NoteCameraJson {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct NoteDocumentJson {
    schema: String,
    id: String,
    camera: NoteCameraJson,
    blocks: Vec<Value>,
    active_tool: Option<String>,
    grid_visible: Option<bool>,
    grid_spacing: Option<f64>,
    grid_subdivisions: Option<f64>,
    grid_opacity: Option<f64>,
    snap_enabled: Option<bool>,
    snap_grid_spacing: Option<f64>,
    pencil_width: Option<f64>,
    eraser_radius: Option<f64>,
    assets: HashMap<String, Value>,
}

impl Default for NoteDocumentJson {
    fn default() -> Self {
        Self {
            schema: "note.document".into(),
            id: "empty".into(),
            camera: NoteCameraJson::default(),
            blocks: Vec::new(),
            active_tool: Some("selectDirect".into()),
            grid_visible: None,
            grid_spacing: None,
            grid_subdivisions: None,
            grid_opacity: None,
            snap_enabled: None,
            snap_grid_spacing: None,
            pencil_width: None,
            eraser_radius: None,
            assets: HashMap::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct NoteBoundsF {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl NoteBoundsF {
    fn contains_point(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }

    fn intersects(&self, other: &NoteBoundsF) -> bool {
        self.x < other.x + other.w && self.x + self.w > other.x && self.y < other.y + other.h && self.y + self.h > other.y
    }
}

fn note_block_str<'a>(block: &'a Value, key: &str) -> &'a str {
    block.get(key).and_then(Value::as_str).unwrap_or("")
}

fn note_block_id(block: &Value) -> &str {
    note_block_str(block, "id")
}

fn note_block_kind(block: &Value) -> &str {
    note_block_str(block, "kind")
}

fn note_block_visible(block: &Value) -> bool {
    block.get("visible").and_then(Value::as_bool).unwrap_or(true)
}

fn note_block_locked(block: &Value) -> bool {
    block.get("locked").and_then(Value::as_bool).unwrap_or(false)
}

fn note_block_num(block: &Value, key: &str) -> f64 {
    block.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

fn note_block_bounds(block: &Value) -> NoteBoundsF {
    let x = note_block_num(block, "x");
    let y = note_block_num(block, "y");
    let w = note_block_num(block, "width");
    let h = note_block_num(block, "height");
    if note_block_kind(block) == "ink" {
        if let Some(points) = block.get("points").and_then(Value::as_array) {
            if !points.is_empty() {
                let mut min_x = f64::INFINITY;
                let mut min_y = f64::INFINITY;
                let mut max_x = f64::NEG_INFINITY;
                let mut max_y = f64::NEG_INFINITY;
                for point in points {
                    let px = point.get(0).and_then(Value::as_f64).unwrap_or(0.0);
                    let py = point.get(1).and_then(Value::as_f64).unwrap_or(0.0);
                    min_x = min_x.min(px);
                    min_y = min_y.min(py);
                    max_x = max_x.max(px);
                    max_y = max_y.max(py);
                }
                return NoteBoundsF {
                    x: x + min_x,
                    y: y + min_y,
                    w: (max_x - min_x).max(1.0),
                    h: (max_y - min_y).max(1.0),
                };
            }
        }
    }
    NoteBoundsF { x, y, w, h }
}

fn note_effective_bounds(block: &Value, overrides: &HashMap<String, Value>) -> NoteBoundsF {
    match overrides.get(note_block_id(block)) {
        Some(over) => note_block_bounds(over),
        None => note_block_bounds(block),
    }
}

fn flatten_note_blocks(blocks: &[Value]) -> Vec<&Value> {
    let mut out = Vec::new();
    fn visit<'a>(blocks: &'a [Value], out: &mut Vec<&'a Value>) {
        for block in blocks {
            out.push(block);
            if note_block_kind(block) == "group" {
                if let Some(children) = block.get("children").and_then(Value::as_array) {
                    visit(children, out);
                }
            }
        }
    }
    visit(blocks, &mut out);
    out
}

fn find_note_block<'a>(blocks: &'a [Value], id: &str) -> Option<&'a Value> {
    flatten_note_blocks(blocks).into_iter().find(|block| note_block_id(block) == id)
}

fn note_blocks_at_point<'a>(blocks: &'a [Value], overrides: &HashMap<String, Value>, x: f64, y: f64) -> Vec<&'a Value> {
    let mut flat = flatten_note_blocks(blocks);
    flat.reverse();
    flat.into_iter()
        .filter(|block| note_effective_bounds(block, overrides).contains_point(x, y))
        .collect()
}

fn note_blocks_intersecting_rect(blocks: &[Value], overrides: &HashMap<String, Value>, rect: NoteBoundsF) -> Vec<String> {
    flatten_note_blocks(blocks)
        .into_iter()
        .filter(|block| note_effective_bounds(block, overrides).intersects(&rect))
        .map(|block| note_block_id(block).to_string())
        .collect()
}

fn note_selection_bounds(blocks: &[Value], overrides: &HashMap<String, Value>, ids: &[String]) -> Option<NoteBoundsF> {
    let id_set: HashSet<&str> = ids.iter().map(String::as_str).collect();
    let selected: Vec<NoteBoundsF> = flatten_note_blocks(blocks)
        .into_iter()
        .filter(|block| id_set.contains(note_block_id(block)))
        .map(|block| note_effective_bounds(block, overrides))
        .collect();
    if selected.is_empty() {
        return None;
    }
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for bounds in &selected {
        min_x = min_x.min(bounds.x);
        min_y = min_y.min(bounds.y);
        max_x = max_x.max(bounds.x + bounds.w);
        max_y = max_y.max(bounds.y + bounds.h);
    }
    Some(NoteBoundsF { x: min_x, y: min_y, w: (max_x - min_x).max(1.0), h: (max_y - min_y).max(1.0) })
}

fn note_scale_value(v: f64, from_min: f64, from_size: f64, to_min: f64, to_size: f64) -> f64 {
    if from_size <= 0.0 {
        return to_min;
    }
    to_min + ((v - from_min) / from_size) * to_size
}

fn note_scaled_block(block: &Value, from: NoteBoundsF, to: NoteBoundsF) -> Value {
    let bounds = note_block_bounds(block);
    let next_x = note_scale_value(bounds.x, from.x, from.w, to.x, to.w);
    let next_y = note_scale_value(bounds.y, from.y, from.h, to.y, to.h);
    let next_w = (note_scale_value(bounds.x + bounds.w, from.x, from.w, to.x, to.w) - next_x).max(8.0);
    let next_h = (note_scale_value(bounds.y + bounds.h, from.y, from.h, to.y, to.h) - next_y).max(8.0);
    let mut cloned = block.clone();
    if let Some(obj) = cloned.as_object_mut() {
        obj.insert("x".into(), json!(next_x));
        obj.insert("y".into(), json!(next_y));
        obj.insert("width".into(), json!(next_w));
        obj.insert("height".into(), json!(next_h));
        if note_block_kind(block) == "ink" {
            let scale_x = if from.w > 0.0 { to.w / from.w } else { 1.0 };
            let scale_y = if from.h > 0.0 { to.h / from.h } else { 1.0 };
            if let Some(points) = block.get("points").and_then(Value::as_array) {
                let scaled: Vec<Value> = points
                    .iter()
                    .map(|p| {
                        let px = p.get(0).and_then(Value::as_f64).unwrap_or(0.0) * scale_x;
                        let py = p.get(1).and_then(Value::as_f64).unwrap_or(0.0) * scale_y;
                        json!([px, py])
                    })
                    .collect();
                obj.insert("points".into(), Value::Array(scaled));
            }
        }
    }
    cloned
}

fn note_resize_bounds(from: NoteBoundsF, handle: &str, dx: f64, dy: f64, min_size: f64) -> NoteBoundsF {
    let mut x = from.x;
    let mut y = from.y;
    let mut w = from.w;
    let mut h = from.h;
    if handle.contains('e') {
        w = (w + dx).max(min_size);
    }
    if handle.contains('w') {
        let next_w = (w - dx).max(min_size);
        x += w - next_w;
        w = next_w;
    }
    if handle.contains('s') {
        h = (h + dy).max(min_size);
    }
    if handle.contains('n') {
        let next_h = (h - dy).max(min_size);
        y += h - next_h;
        h = next_h;
    }
    NoteBoundsF { x, y, w, h }
}

fn note_snap_coordinate(v: f64, spacing: f64) -> f64 {
    if spacing <= 0.0 {
        v
    } else {
        (v / spacing).round() * spacing
    }
}

fn note_snap_point(x: f64, y: f64, spacing: f64) -> (f64, f64) {
    (note_snap_coordinate(x, spacing), note_snap_coordinate(y, spacing))
}

fn note_maybe_snap(doc: &NoteDocumentJson, x: f64, y: f64) -> (f64, f64) {
    if doc.snap_enabled.unwrap_or(false) {
        note_snap_point(x, y, doc.snap_grid_spacing.unwrap_or(8.0))
    } else {
        (x, y)
    }
}

fn note_block_with_position(block: &Value, x: f64, y: f64) -> Value {
    let mut cloned = block.clone();
    if let Some(obj) = cloned.as_object_mut() {
        obj.insert("x".into(), json!(x));
        obj.insert("y".into(), json!(y));
    }
    cloned
}

fn note_create_block(kind: &str, x: f64, y: f64) -> Value {
    let id = create_note_host_id(kind);
    match kind {
        "image" => json!({
            "id": id, "name": "Image", "kind": "image", "x": x, "y": y, "width": 240.0, "height": 160.0,
            "rotation": 0.0, "visible": true, "locked": false, "imageKey": "placeholder",
        }),
        "table" => json!({
            "id": id, "name": "Table", "kind": "table", "x": x, "y": y, "width": 320.0, "height": 160.0,
            "rotation": 0.0, "visible": true, "locked": false,
            "columns": ["A", "B", "C"],
            "rows": [
                [{"content": ""}, {"content": ""}, {"content": ""}],
                [{"content": ""}, {"content": ""}, {"content": ""}],
            ],
        }),
        "math" => json!({
            "id": id, "name": "Math", "kind": "math", "x": x, "y": y, "width": 200.0, "height": 80.0,
            "rotation": 0.0, "visible": true, "locked": false, "tex": "E = mc^2", "displayMode": true,
        }),
        "ink" => json!({
            "id": id, "name": "Ink", "kind": "ink", "x": x, "y": y, "width": 1.0, "height": 1.0,
            "rotation": 0.0, "visible": true, "locked": false, "points": [], "strokeWidth": 3.0, "color": [0.0, 0.0, 0.0, 1.0],
        }),
        "group" => json!({
            "id": id, "name": "Group", "kind": "group", "x": x, "y": y, "width": 280.0, "height": 120.0,
            "rotation": 0.0, "visible": true, "locked": false, "children": [],
        }),
        _ => json!({
            "id": id, "name": "Text", "kind": "text", "x": x, "y": y, "width": 280.0, "height": 120.0,
            "rotation": 0.0, "visible": true, "locked": false,
            "paragraphs": [{"runs": [{"text": ""}]}], "fontSize": 18.0, "fontWeight": "normal", "align": "left",
        }),
    }
}

fn note_text_plain(block: &Value) -> String {
    block
        .get("paragraphs")
        .and_then(Value::as_array)
        .map(|paragraphs| {
            paragraphs
                .iter()
                .map(|paragraph| {
                    paragraph
                        .get("runs")
                        .and_then(Value::as_array)
                        .map(|runs| runs.iter().filter_map(|run| run.get("text").and_then(Value::as_str)).collect::<String>())
                        .unwrap_or_default()
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default()
}

fn point_segment_distance(px: f64, py: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    if dx == 0.0 && dy == 0.0 {
        return ((px - x1).powi(2) + (py - y1).powi(2)).sqrt();
    }
    let t = (((px - x1) * dx + (py - y1) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
    ((px - (x1 + t * dx)).powi(2) + (py - (y1 + t * dy)).powi(2)).sqrt()
}

fn ink_points(block: &Value) -> Vec<(f64, f64)> {
    let bx = note_block_num(block, "x");
    let by = note_block_num(block, "y");
    block
        .get("points")
        .and_then(Value::as_array)
        .map(|points| {
            points
                .iter()
                .map(|p| {
                    let px = p.get(0).and_then(Value::as_f64).unwrap_or(0.0);
                    let py = p.get(1).and_then(Value::as_f64).unwrap_or(0.0);
                    (bx + px, by + py)
                })
                .collect()
        })
        .unwrap_or_default()
}

fn ink_hits_point(block: &Value, x: f64, y: f64, threshold: f64) -> bool {
    let points = ink_points(block);
    let stroke_width = note_block_num(block, "strokeWidth");
    if points.len() < 2 {
        return points.first().map(|p| ((x - p.0).powi(2) + (y - p.1).powi(2)).sqrt() <= threshold).unwrap_or(false);
    }
    points
        .windows(2)
        .any(|w| point_segment_distance(x, y, w[0].0, w[0].1, w[1].0, w[1].1) <= threshold + stroke_width / 2.0)
}

fn note_erase_ink_stroke_events(blocks: &[Value], x: f64, y: f64, threshold: f64) -> Vec<Value> {
    flatten_note_blocks(blocks)
        .into_iter()
        .filter(|block| note_block_kind(block) == "ink" && ink_hits_point(block, x, y, threshold))
        .map(|block| json!({ "op": "removeBlock", "blockId": note_block_id(block) }))
        .collect()
}

fn note_erase_ink_points_in_block(block: &Value, x: f64, y: f64, radius: f64) -> Vec<Value> {
    let bx = note_block_num(block, "x");
    let by = note_block_num(block, "y");
    let points = block.get("points").and_then(Value::as_array).cloned().unwrap_or_default();
    let mut kept_indices = Vec::new();
    for (index, point) in points.iter().enumerate() {
        let px = bx + point.get(0).and_then(Value::as_f64).unwrap_or(0.0);
        let py = by + point.get(1).and_then(Value::as_f64).unwrap_or(0.0);
        if ((px - x).powi(2) + (py - y).powi(2)).sqrt() > radius {
            kept_indices.push(index);
        }
    }
    if kept_indices.len() == points.len() {
        return vec![block.clone()];
    }
    if kept_indices.is_empty() {
        return Vec::new();
    }
    let mut runs: Vec<Vec<Value>> = Vec::new();
    let mut current: Vec<Value> = vec![points[kept_indices[0]].clone()];
    for window in kept_indices.windows(2) {
        if window[1] - window[0] > 1 {
            if current.len() >= 2 {
                runs.push(current);
            }
            current = vec![points[window[1]].clone()];
        } else {
            current.push(points[window[1]].clone());
        }
    }
    if current.len() >= 2 {
        runs.push(current);
    }
    let name = note_block_str(block, "name").to_string();
    runs.into_iter()
        .enumerate()
        .map(|(index, pts)| {
            let mut cloned = block.clone();
            if let Some(obj) = cloned.as_object_mut() {
                if index > 0 {
                    obj.insert("id".into(), json!(create_note_host_id("ink")));
                    obj.insert("name".into(), json!(format!("{name} fragment")));
                }
                obj.insert("points".into(), Value::Array(pts));
            }
            cloned
        })
        .collect()
}

fn note_erase_ink_points_events(blocks: &[Value], x: f64, y: f64, radius: f64) -> Vec<Value> {
    let mut events = Vec::new();
    for block in flatten_note_blocks(blocks) {
        if note_block_kind(block) != "ink" {
            continue;
        }
        let fragments = note_erase_ink_points_in_block(block, x, y, radius);
        if fragments.len() == 1 && fragments[0] == *block {
            continue;
        }
        events.push(json!({ "op": "removeBlock", "blockId": note_block_id(block) }));
        for fragment in fragments {
            events.push(json!({ "op": "addBlock", "block": fragment }));
        }
    }
    events
}

fn note_screen_to_world(camera: NoteCameraF, inner: Rect, sx: f32, sy: f32) -> (f64, f64) {
    let lx = (sx - inner.x) as f64;
    let ly = (sy - inner.y) as f64;
    ((lx - camera.x) / camera.zoom, (ly - camera.y) / camera.zoom)
}

fn note_world_to_screen(camera: NoteCameraF, inner: Rect, wx: f64, wy: f64) -> (f32, f32) {
    (inner.x + (wx * camera.zoom + camera.x) as f32, inner.y + (wy * camera.zoom + camera.y) as f32)
}

fn positive_mod_f32(v: f32, m: f32) -> f32 {
    if m <= 0.0 {
        0.0
    } else {
        ((v % m) + m) % m
    }
}
//#endregion NoteCanvasModel

//#region NoteCanvasState
fn note_current_camera(scene: &UiComponentSceneNode) -> NoteCameraF {
    let state = scene_state(&scene.surface_id);
    if let Some((x, y, zoom)) = state.note_camera {
        return NoteCameraF { x, y, zoom };
    }
    scene
        .note_canvas
        .as_ref()
        .and_then(|note| serde_json::from_str::<NoteDocumentJson>(&note.document_json).ok())
        .map(|doc| NoteCameraF::from(doc.camera))
        .unwrap_or_default()
}

fn note_events_json(events: &[Value]) -> String {
    Value::Array(events.to_vec()).to_string()
}

fn note_apply_events_cmd(scene: &UiComponentSceneNode, events: &[Value], phase: &str, select_ids: Option<&[String]>) -> CommandDescriptor {
    let mut args = json!({
        "surfaceId": scene.surface_id,
        "eventsJson": note_events_json(events),
        "phase": phase,
    });
    if let Some(ids) = select_ids {
        args["selectIds"] = json!(ids);
    }
    scene_cmd(scene, "applyNoteEvents", args)
}

fn note_set_selection_cmd(scene: &UiComponentSceneNode, ids: &[String]) -> CommandDescriptor {
    scene_cmd(scene, "setSelection", json!({ "surfaceId": scene.surface_id, "ids": ids }))
}

fn note_set_hover_cmd(scene: &UiComponentSceneNode, id: Option<&str>) -> CommandDescriptor {
    scene_cmd(scene, "setHover", json!({ "surfaceId": scene.surface_id, "id": id }))
}

fn note_set_camera_cmd(scene: &UiComponentSceneNode, camera: NoteCameraF) -> CommandDescriptor {
    scene_cmd(
        scene,
        "setCamera",
        json!({ "surfaceId": scene.surface_id, "camera": { "x": camera.x, "y": camera.y, "zoom": camera.zoom } }),
    )
}

const NOTE_RESIZE_HANDLES: [&str; 8] = ["nw", "n", "ne", "e", "se", "s", "sw", "w"];

fn note_resize_handle_screen_pos(handle: &str, sx: f32, sy: f32, w: f32, h: f32, size: f32) -> (f32, f32) {
    let half = size * 0.5;
    let x = if handle.contains('w') {
        sx - half
    } else if handle.contains('e') {
        sx + w - half
    } else {
        sx + w * 0.5 - half
    };
    let y = if handle.contains('n') {
        sy - half
    } else if handle.contains('s') {
        sy + h - half
    } else {
        sy + h * 0.5 - half
    };
    (x, y)
}

fn note_resize_handle_at(bounds: NoteBoundsF, camera: NoteCameraF, inner: Rect, sx: f32, sy: f32, hit_radius: f32) -> Option<&'static str> {
    let (bx, by) = note_world_to_screen(camera, inner, bounds.x, bounds.y);
    let w = (bounds.w * camera.zoom) as f32;
    let h = (bounds.h * camera.zoom) as f32;
    for handle in NOTE_RESIZE_HANDLES {
        let (hx, hy) = note_resize_handle_screen_pos(handle, bx, by, w, h, 8.0);
        let cx = hx + 4.0;
        let cy = hy + 4.0;
        if ((sx - cx).powi(2) + (sy - cy).powi(2)).sqrt() <= hit_radius {
            return Some(handle);
        }
    }
    None
}

/** @emoji 📝️ Pointer-down entry point for note-canvas: mirrors handlePointerDown in note-canvas-host.tsx. */
fn note_pointer_down(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, button: i16, shift: bool) -> Vec<CommandDescriptor> {
    let Some(note) = &scene.note_canvas else {
        return Vec::new();
    };
    if note.view_mode == "navigator" || !note.interactive {
        return Vec::new();
    }
    let doc: NoteDocumentJson = serde_json::from_str(&note.document_json).unwrap_or_default();
    let selected_ids: Vec<String> = serde_json::from_str(&note.selection_json).unwrap_or_default();
    let state = scene_state(&scene.surface_id);
    let camera = state
        .note_camera
        .map(|(cx, cy, cz)| NoteCameraF { x: cx, y: cy, zoom: cz })
        .unwrap_or_else(|| NoteCameraF::from(doc.camera.clone()));
    let tool = doc.active_tool.clone().unwrap_or_else(|| "selectDirect".into());
    let mut commands = Vec::new();

    let selection_bounds = note_selection_bounds(&doc.blocks, &state.note_overrides, &selected_ids);
    let show_handles = (tool == "selectDirect" || tool == "selectMarquee") && selection_bounds.is_some() && !selected_ids.is_empty();
    if button == 0 && show_handles {
        if let Some(bounds) = selection_bounds {
            if let Some(handle) = note_resize_handle_at(bounds, camera, inner, x, y, 8.0) {
                mutate_scene_state(&scene.surface_id, |s| {
                    s.drag = Some(SceneDrag {
                        mode: SceneDragMode::NoteResize {
                            handle: handle.to_string(),
                            from: bounds,
                            start_x: x,
                            start_y: y,
                            selected_ids: selected_ids.clone(),
                        },
                        button,
                    });
                });
                return commands;
            }
        }
    }

    if tool == "pan" || button == 1 {
        mutate_scene_state(&scene.surface_id, |s| {
            s.drag = Some(SceneDrag {
                mode: SceneDragMode::NotePan { start_x: x, start_y: y, camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom },
                button,
            });
        });
        return commands;
    }

    if button != 0 {
        return commands;
    }

    let (world_x, world_y) = note_screen_to_world(camera, inner, x, y);

    if tool == "eraserStroke" || tool == "eraserPoint" {
        let events = if tool == "eraserStroke" {
            note_erase_ink_stroke_events(&doc.blocks, world_x, world_y, 8.0)
        } else {
            note_erase_ink_points_events(&doc.blocks, world_x, world_y, doc.eraser_radius.unwrap_or(12.0))
        };
        mutate_scene_state(&scene.surface_id, |s| {
            s.drag = Some(SceneDrag { mode: SceneDragMode::NoteEraser { mode: tool.clone() }, button });
        });
        if !events.is_empty() {
            commands.push(note_apply_events_cmd(scene, &events, "begin", None));
        }
        return commands;
    }

    if tool == "selectMarquee" {
        mutate_scene_state(&scene.surface_id, |s| {
            s.drag = Some(SceneDrag { mode: SceneDragMode::NoteMarqueeDrag { start_x: x, start_y: y }, button });
            s.note_marquee_points = vec![(x, y)];
        });
        return commands;
    }

    if tool == "pencil" {
        let block = note_create_block("ink", world_x, world_y);
        let block_id = note_block_id(&block).to_string();
        mutate_scene_state(&scene.surface_id, |s| {
            s.note_overrides.insert(block_id.clone(), block.clone());
            s.drag = Some(SceneDrag { mode: SceneDragMode::NoteInk { block_id: block_id.clone() }, button });
        });
        commands.push(note_apply_events_cmd(scene, &[json!({ "op": "addBlock", "block": block })], "begin", Some(&[block_id])));
        return commands;
    }

    if tool == "text" || tool == "image" || tool == "table" || tool == "math" {
        let (px, py) = note_maybe_snap(&doc, world_x, world_y);
        let block = note_create_block(&tool, px, py);
        let block_id = note_block_id(&block).to_string();
        commands.push(note_apply_events_cmd(scene, &[json!({ "op": "addBlock", "block": block })], "atomic", Some(&[block_id])));
        return commands;
    }

    let hits = note_blocks_at_point(&doc.blocks, &state.note_overrides, world_x, world_y);
    let top = hits.first().copied();
    match top {
        Some(top_block) if !note_block_locked(top_block) => {
            if tool == "selectDirect" {
                let top_id = note_block_id(top_block).to_string();
                let next_selection = if shift {
                    let mut ids: Vec<String> = selected_ids.clone();
                    if !ids.contains(&top_id) {
                        ids.push(top_id.clone());
                    }
                    ids
                } else {
                    vec![top_id.clone()]
                };
                commands.push(note_set_selection_cmd(scene, &next_selection));
                let move_ids: Vec<String> = if selected_ids.contains(&top_id) { selected_ids.clone() } else { vec![top_id.clone()] };
                let mut origins = HashMap::new();
                for id in &move_ids {
                    if let Some(b) = find_note_block(&doc.blocks, id) {
                        let eff = state.note_overrides.get(id).unwrap_or(b);
                        origins.insert(id.clone(), (note_block_num(eff, "x"), note_block_num(eff, "y")));
                    }
                }
                mutate_scene_state(&scene.surface_id, |s| {
                    s.drag = Some(SceneDrag { mode: SceneDragMode::NoteMove { origins, start_x: x, start_y: y }, button });
                });
            }
        }
        _ => {
            if tool == "selectDirect" {
                commands.push(note_set_selection_cmd(scene, &[]));
            }
        }
    }
    commands
}

/** @emoji 📝️ Pointer-up entry point for note-canvas: commits the active gesture and finalizes marquee selection. */
fn note_pointer_up(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Vec<CommandDescriptor> {
    let mut commands = Vec::new();
    let state = scene_state(&scene.surface_id);
    let Some(drag) = state.drag.clone() else {
        return commands;
    };
    let doc: NoteDocumentJson = scene
        .note_canvas
        .as_ref()
        .map(|n| serde_json::from_str(&n.document_json).unwrap_or_default())
        .unwrap_or_default();
    match &drag.mode {
        SceneDragMode::NoteMove { origins, .. } => {
            let mut events = Vec::new();
            for id in origins.keys() {
                if let Some(block) = state.note_overrides.get(id).cloned().or_else(|| find_note_block(&doc.blocks, id).cloned()) {
                    let updated = if doc.snap_enabled.unwrap_or(false) {
                        let spacing = doc.snap_grid_spacing.unwrap_or(8.0);
                        let (sx, sy) = note_snap_point(note_block_num(&block, "x"), note_block_num(&block, "y"), spacing);
                        note_block_with_position(&block, sx, sy)
                    } else {
                        block
                    };
                    events.push(json!({ "op": "updateBlock", "blockId": id, "block": updated }));
                }
            }
            commands.push(note_apply_events_cmd(scene, &events, "commit", None));
        }
        SceneDragMode::NoteResize { selected_ids, .. } => {
            let mut events = Vec::new();
            for id in selected_ids {
                if let Some(block) = state.note_overrides.get(id).cloned() {
                    events.push(json!({ "op": "updateBlock", "blockId": id, "block": block }));
                }
            }
            commands.push(note_apply_events_cmd(scene, &events, "commit", None));
        }
        SceneDragMode::NoteInk { block_id } => {
            if let Some(block) = state.note_overrides.get(block_id).cloned() {
                commands.push(note_apply_events_cmd(scene, &[json!({ "op": "updateBlock", "blockId": block_id, "block": block })], "commit", None));
            } else {
                commands.push(note_apply_events_cmd(scene, &[], "commit", None));
            }
        }
        SceneDragMode::NoteEraser { .. } => {
            commands.push(note_apply_events_cmd(scene, &[], "commit", None));
        }
        SceneDragMode::NoteMarqueeDrag { start_x, start_y } => {
            let x0 = start_x.min(x);
            let y0 = start_y.min(y);
            let w = (x - start_x).abs();
            let h = (y - start_y).abs();
            if w >= 4.0 || h >= 4.0 {
                let camera = note_current_camera(scene);
                let (wx0, wy0) = note_screen_to_world(camera, inner, x0, y0);
                let (wx1, wy1) = note_screen_to_world(camera, inner, x0 + w, y0 + h);
                let world_rect = NoteBoundsF { x: wx0.min(wx1), y: wy0.min(wy1), w: (wx1 - wx0).abs(), h: (wy1 - wy0).abs() };
                let ids = note_blocks_intersecting_rect(&doc.blocks, &state.note_overrides, world_rect);
                commands.push(note_set_selection_cmd(scene, &ids));
            }
        }
        _ => {}
    }
    mutate_scene_state(&scene.surface_id, |s| {
        s.drag = None;
        s.note_marquee_points.clear();
    });
    commands
}

/** @emoji 📝️ Pointer-move hover entry point for note-canvas: mirrors the `!dragState` hover branch of handlePointerMove. */
fn note_hover_move(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Vec<CommandDescriptor> {
    let Some(note) = &scene.note_canvas else {
        return Vec::new();
    };
    if note.view_mode == "navigator" || !note.interactive {
        return Vec::new();
    }
    let doc: NoteDocumentJson = serde_json::from_str(&note.document_json).unwrap_or_default();
    let camera = note_current_camera(scene);
    let (wx, wy) = note_screen_to_world(camera, inner, x, y);
    let state = scene_state(&scene.surface_id);
    let hits = note_blocks_at_point(&doc.blocks, &state.note_overrides, wx, wy);
    let top_id = hits.first().map(|block| note_block_id(block).to_string());
    if note.hovered_id.as_deref() == top_id.as_deref() {
        return Vec::new();
    }
    vec![note_set_hover_cmd(scene, top_id.as_deref())]
}

/** @emoji 📝️ Wheel entry point for note-canvas: zoom-at-cursor, mirrors handleWheel in note-canvas-host.tsx. */
fn note_wheel(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, delta: f32) -> Vec<CommandDescriptor> {
    let Some(note) = &scene.note_canvas else {
        return Vec::new();
    };
    if note.view_mode == "navigator" {
        return Vec::new();
    }
    let camera = note_current_camera(scene);
    let zoom_factor: f64 = if delta < 0.0 { 1.08 } else { 0.92 };
    let next_zoom = (camera.zoom * zoom_factor).clamp(0.1, 8.0);
    let (wx, wy) = note_screen_to_world(camera, inner, x, y);
    let next = NoteCameraF {
        x: (x - inner.x) as f64 - wx * next_zoom,
        y: (y - inner.y) as f64 - wy * next_zoom,
        zoom: next_zoom,
    };
    mutate_scene_state(&scene.surface_id, |s| {
        s.note_camera = Some((next.x, next.y, next.zoom));
    });
    vec![note_set_camera_cmd(scene, next)]
}
//#endregion NoteCanvasState

//#region NoteCanvasRender
fn note_draw_rect_outline(draw: &mut semio_framework_ui_wgpu::DrawList, x: f32, y: f32, w: f32, h: f32, color: Rgba, width: f32) {
    draw.push_line(x, y, x + w, y, color, width);
    draw.push_line(x + w, y, x + w, y + h, color, width);
    draw.push_line(x + w, y + h, x, y + h, color, width);
    draw.push_line(x, y + h, x, y, color, width);
}

fn note_draw_grid(draw: &mut semio_framework_ui_wgpu::DrawList, camera: NoteCameraF, inner: Rect, theme: &Theme, spacing: f64, subdivisions: u32, opacity: f64) {
    let major_px = (spacing * camera.zoom) as f32;
    if major_px < 2.0 {
        return;
    }
    let minor_px = major_px / subdivisions.max(1) as f32;
    let offset_x = positive_mod_f32(camera.x as f32, major_px);
    let offset_y = positive_mod_f32(camera.y as f32, major_px);
    let color = theme.separator.with_alpha((theme.separator.a * opacity as f32).max(0.05));
    let minor_color = color.with_alpha(color.a * 0.55);

    let mut wx = inner.x + positive_mod_f32(offset_x, major_px) - major_px;
    while wx < inner.x + inner.w {
        if subdivisions > 1 {
            for s in 1..subdivisions {
                let mx = wx + s as f32 * minor_px;
                if mx >= inner.x && mx <= inner.x + inner.w {
                    draw.push_line(mx, inner.y, mx, inner.y + inner.h, minor_color, 0.5);
                }
            }
        }
        if wx >= inner.x && wx <= inner.x + inner.w {
            draw.push_line(wx, inner.y, wx, inner.y + inner.h, color, 1.0);
        }
        wx += major_px;
    }
    let mut wy = inner.y + positive_mod_f32(offset_y, major_px) - major_px;
    while wy < inner.y + inner.h {
        if subdivisions > 1 {
            for s in 1..subdivisions {
                let my = wy + s as f32 * minor_px;
                if my >= inner.y && my <= inner.y + inner.h {
                    draw.push_line(inner.x, my, inner.x + inner.w, my, minor_color, 0.5);
                }
            }
        }
        if wy >= inner.y && wy <= inner.y + inner.h {
            draw.push_line(inner.x, wy, inner.x + inner.w, wy, color, 1.0);
        }
        wy += major_px;
    }
}

fn note_draw_table(ctx: &mut FrameworkWidgetContext<'_>, block: &Value, sx: f32, sy: f32, w: f32, h: f32, theme: &Theme) {
    let columns: Vec<String> = block
        .get("columns")
        .and_then(Value::as_array)
        .map(|c| c.iter().filter_map(|v| v.as_str().map(str::to_string)).collect())
        .unwrap_or_default();
    let rows: Vec<Vec<String>> = block
        .get("rows")
        .and_then(Value::as_array)
        .map(|rows| {
            rows.iter()
                .map(|row| {
                    row.as_array()
                        .map(|cells| cells.iter().map(|cell| cell.get("content").and_then(Value::as_str).unwrap_or("").to_string()).collect())
                        .unwrap_or_default()
                })
                .collect()
        })
        .unwrap_or_default();
    let col_count = columns.len().max(1);
    let row_count = rows.len() + 1;
    let col_w = w / col_count as f32;
    let row_h = h / row_count as f32;
    let font = theme.font_size_small.min(row_h * 0.6).max(6.0);
    for (index, label) in columns.iter().enumerate() {
        draw_text(ctx, label, sx + index as f32 * col_w + 3.0, sy + row_h * 0.7, font, theme.text_muted);
    }
    for (row_index, row) in rows.iter().enumerate() {
        let ry = sy + (row_index + 1) as f32 * row_h;
        for (col_index, cell) in row.iter().enumerate() {
            draw_text(ctx, cell, sx + col_index as f32 * col_w + 3.0, ry + row_h * 0.7, font, theme.text);
        }
    }
    for index in 0..=col_count {
        let x = sx + index as f32 * col_w;
        ctx.draw.push_line(x, sy, x, sy + h, theme.separator, 0.5);
    }
    for index in 0..=row_count {
        let y = sy + index as f32 * row_h;
        ctx.draw.push_line(sx, y, sx + w, y, theme.separator, 0.5);
    }
}

fn note_draw_image(ctx: &mut FrameworkWidgetContext<'_>, scene: &UiComponentSceneNode, block: &Value, doc: &NoteDocumentJson, sx: f32, sy: f32, w: f32, h: f32) {
    let theme = ctx.theme;
    let image_key = note_block_str(block, "imageKey");
    if let Some(asset) = doc.assets.get(image_key) {
        let mime = asset.get("mime").and_then(Value::as_str).unwrap_or("image/png");
        let data = asset.get("data").and_then(Value::as_str).unwrap_or("");
        let data_url = if data.starts_with("data:") { data.to_string() } else { format!("data:{mime};base64,{data}") };
        if let Some(key) = queue_canvas_image_upload(&scene.surface_id, note_block_id(block), &data_url) {
            ctx.draw.push_raster_quad(&key, [sx, sy, w.max(1.0), h.max(1.0)], [0.0, 0.0, 1.0, 1.0], 1.0);
            return;
        }
    }
    draw_text(ctx, image_key, sx + 6.0, sy + h * 0.5, theme.font_size_small, theme.text_muted);
}

fn note_draw_block(
    ctx: &mut FrameworkWidgetContext<'_>,
    scene: &UiComponentSceneNode,
    block: &Value,
    camera: NoteCameraF,
    inner: Rect,
    doc: &NoteDocumentJson,
    selected: bool,
    hovered: bool,
) {
    let theme = ctx.theme;
    let kind = note_block_kind(block);
    let bounds = note_block_bounds(block);
    let (sx, sy) = note_world_to_screen(camera, inner, bounds.x, bounds.y);
    let w = (bounds.w * camera.zoom) as f32;
    let h = (bounds.h * camera.zoom) as f32;

    if kind == "ink" {
        let points = ink_points(block);
        if points.len() >= 2 {
            let color = block
                .get("color")
                .and_then(Value::as_array)
                .map(|c| {
                    let get = |i: usize| c.get(i).and_then(Value::as_f64).unwrap_or(0.0) as f32;
                    Rgba::new(get(0), get(1), get(2), get(3))
                })
                .unwrap_or(Rgba::new(0.0, 0.0, 0.0, 1.0));
            let stroke_width = (note_block_num(block, "strokeWidth") as f32 * camera.zoom as f32).max(1.0);
            let screen_points: Vec<(f32, f32)> = points.iter().map(|p| note_world_to_screen(camera, inner, p.0, p.1)).collect();
            for pair in screen_points.windows(2) {
                ctx.draw.push_line(pair[0].0, pair[0].1, pair[1].0, pair[1].1, color, stroke_width);
            }
        }
        return;
    }

    let bg = theme.panel;
    ctx.draw.push_rounded([sx, sy, w.max(4.0), h.max(4.0)], bg.with_alpha(0.92), theme.border_radius.min(6.0));

    match kind {
        "text" => {
            let text = note_text_plain(block);
            let font_size = (note_block_num(block, "fontSize").max(8.0) as f32 * camera.zoom as f32).max(6.0);
            draw_text_wrapped(ctx, &text, sx + 6.0, sy + 4.0, (w - 12.0).max(1.0), font_size, theme.text);
        }
        "math" => {
            let tex = note_block_str(block, "tex");
            draw_text(ctx, tex, sx + 8.0, sy + h * 0.5 + 4.0, theme.font_size_body.max(8.0), theme.text);
        }
        "table" => note_draw_table(ctx, block, sx, sy, w.max(4.0), h.max(4.0), theme),
        "image" => note_draw_image(ctx, scene, block, doc, sx, sy, w.max(4.0), h.max(4.0)),
        "group" => {
            let children_len = block.get("children").and_then(Value::as_array).map(Vec::len).unwrap_or(0);
            draw_text(ctx, &format!("Group · {children_len} children"), sx + 6.0, sy + 16.0, theme.font_size_small, theme.text_muted);
        }
        _ => {}
    }

    let border = if selected {
        theme.accent
    } else if hovered {
        theme.accent.with_alpha(theme.accent.a * 0.6)
    } else {
        theme.panel_border
    };
    let border_w = if selected { 2.0 } else { 1.0 };
    note_draw_rect_outline(ctx.draw, sx, sy, w.max(4.0), h.max(4.0), border, border_w);
}

fn note_draw_selection_chrome(draw: &mut semio_framework_ui_wgpu::DrawList, theme: &Theme, camera: NoteCameraF, inner: Rect, bounds: NoteBoundsF, show_handles: bool) {
    let (sx, sy) = note_world_to_screen(camera, inner, bounds.x, bounds.y);
    let w = (bounds.w * camera.zoom) as f32;
    let h = (bounds.h * camera.zoom) as f32;
    note_draw_rect_outline(draw, sx, sy, w, h, theme.accent, 1.5);
    if !show_handles {
        return;
    }
    let handle_size = 8.0;
    for handle in NOTE_RESIZE_HANDLES {
        let (hx, hy) = note_resize_handle_screen_pos(handle, sx, sy, w, h, handle_size);
        draw.push_rounded([hx, hy, handle_size, handle_size], theme.background, 1.0);
        note_draw_rect_outline(draw, hx, hy, handle_size, handle_size, theme.accent, 1.0);
    }
}

fn render_note_canvas(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, gpu: &mut semio_framework_ui_wgpu::GpuContext) {
    let _ = gpu;
    let theme = ctx.theme;
    let Some(note) = &scene.note_canvas else {
        return render_placeholder("note-canvas", bounds, ctx);
    };
    let doc: NoteDocumentJson = serde_json::from_str(&note.document_json).unwrap_or_default();
    let selected_ids: Vec<String> = serde_json::from_str(&note.selection_json).unwrap_or_default();
    let selected_set: HashSet<&str> = selected_ids.iter().map(String::as_str).collect();
    let hovered_id = note.hovered_id.clone();
    let is_navigator = note.view_mode == "navigator";
    let inner = bounds;

    let state = scene_state(&scene.surface_id);
    let camera = state.note_camera.map(|(x, y, zoom)| NoteCameraF { x, y, zoom }).unwrap_or_else(|| NoteCameraF::from(doc.camera.clone()));

    ctx.draw.push_solid([inner.x, inner.y, inner.w, inner.h], theme.canvas_clear);
    ctx.draw.push_scissor(inner);

    if doc.grid_visible.unwrap_or(true) && !is_navigator {
        note_draw_grid(ctx.draw, camera, inner, theme, doc.grid_spacing.unwrap_or(32.0), doc.grid_subdivisions.unwrap_or(4.0).max(1.0) as u32, doc.grid_opacity.unwrap_or(0.35));
    }

    let overrides = state.note_overrides.clone();
    let blocks = flatten_note_blocks(&doc.blocks);
    for block in blocks.iter().copied() {
        let effective = overrides.get(note_block_id(block)).unwrap_or(block);
        if !note_block_visible(effective) {
            continue;
        }
        let id = note_block_id(block);
        let selected = selected_set.contains(id);
        let hovered = hovered_id.as_deref() == Some(id);
        note_draw_block(ctx, scene, effective, camera, inner, &doc, selected, hovered);
    }

    let selection_bounds = note_selection_bounds(&doc.blocks, &overrides, &selected_ids);
    let tool = doc.active_tool.clone().unwrap_or_else(|| "selectDirect".into());
    let show_handles = !is_navigator && (tool == "selectDirect" || tool == "selectMarquee") && selection_bounds.is_some() && !selected_ids.is_empty();
    if let Some(sel) = selection_bounds {
        note_draw_selection_chrome(ctx.draw, theme, camera, inner, sel, show_handles);
    }

    if state.note_marquee_points.len() >= 2 {
        let points: Vec<[f32; 2]> = state.note_marquee_points.iter().map(|p| [p.0, p.1]).collect();
        semio_framework_ui_wgpu::paint_selection_marquee(ctx.draw, theme, false, false, &points, false);
    }

    ctx.draw.pop_scissor();

    ctx.input.register_hit(HitTarget {
        rect: inner,
        event: None,
        control_id: Some(scene.surface_id.clone()),
        kind: HitKind::Generic,
        drag_axis: None,
        drag_data: None,
    });
}
//#endregion NoteCanvasRender

//#region NoteCanvasTests
#[cfg(test)]
mod note_canvas_tests {
    use super::*;

    fn sample_block(id: &str, x: f64, y: f64, w: f64, h: f64) -> Value {
        json!({
            "id": id, "name": "Text", "kind": "text", "x": x, "y": y, "width": w, "height": h,
            "rotation": 0.0, "visible": true, "locked": false,
            "paragraphs": [], "fontSize": 18.0, "fontWeight": "normal", "align": "left",
        })
    }

    #[test]
    fn hit_test_prefers_topmost_block() {
        let blocks = vec![sample_block("a", 0.0, 0.0, 100.0, 100.0), sample_block("b", 20.0, 20.0, 100.0, 100.0)];
        let overrides = HashMap::new();
        let hits = note_blocks_at_point(&blocks, &overrides, 50.0, 50.0);
        assert_eq!(note_block_id(hits[0]), "b");
    }

    #[test]
    fn hit_test_misses_outside_bounds() {
        let blocks = vec![sample_block("a", 0.0, 0.0, 10.0, 10.0)];
        let overrides = HashMap::new();
        assert!(note_blocks_at_point(&blocks, &overrides, 50.0, 50.0).is_empty());
    }

    #[test]
    fn resize_bounds_east_handle_grows_width_only() {
        let from = NoteBoundsF { x: 0.0, y: 0.0, w: 100.0, h: 50.0 };
        let to = note_resize_bounds(from, "e", 20.0, 0.0, 8.0);
        assert_eq!(to, NoteBoundsF { x: 0.0, y: 0.0, w: 120.0, h: 50.0 });
    }

    #[test]
    fn resize_bounds_northwest_handle_moves_origin() {
        let from = NoteBoundsF { x: 10.0, y: 10.0, w: 100.0, h: 100.0 };
        let to = note_resize_bounds(from, "nw", -10.0, -10.0, 8.0);
        assert_eq!(to, NoteBoundsF { x: 0.0, y: 0.0, w: 110.0, h: 110.0 });
    }

    #[test]
    fn resize_bounds_respects_minimum_size() {
        let from = NoteBoundsF { x: 0.0, y: 0.0, w: 20.0, h: 20.0 };
        let to = note_resize_bounds(from, "e", -100.0, 0.0, 8.0);
        assert_eq!(to.w, 8.0);
    }

    #[test]
    fn screen_world_roundtrip() {
        let camera = NoteCameraF { x: 12.0, y: -8.0, zoom: 1.5 };
        let inner = Rect::new(100.0, 40.0, 400.0, 300.0);
        let (wx, wy) = note_screen_to_world(camera, inner, 250.0, 150.0);
        let (sx, sy) = note_world_to_screen(camera, inner, wx, wy);
        assert!((sx - 250.0).abs() < 0.01);
        assert!((sy - 150.0).abs() < 0.01);
    }

    #[test]
    fn snap_rounds_to_nearest_grid_cell() {
        assert_eq!(note_snap_coordinate(13.0, 8.0), 16.0);
        assert_eq!(note_snap_coordinate(3.0, 8.0), 0.0);
    }

    #[test]
    fn ink_block_bounds_from_points() {
        let block = json!({
            "id": "i1", "kind": "ink", "x": 10.0, "y": 10.0, "width": 1.0, "height": 1.0,
            "points": [[0.0, 0.0], [5.0, 10.0], [-5.0, 2.0]], "strokeWidth": 3.0, "color": [0, 0, 0, 1],
        });
        let bounds = note_block_bounds(&block);
        assert_eq!(bounds.x, 5.0);
        assert_eq!(bounds.y, 10.0);
        assert_eq!(bounds.w, 10.0);
        assert_eq!(bounds.h, 10.0);
    }
}
//#endregion NoteCanvasTests
