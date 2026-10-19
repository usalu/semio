//! 🧊️ Preview geometry of the preview evaluation: the channel items, the mesh roles, the fit, the marker meshes and the retained mesh handles. See [`super`].

use super::*;

//#region 🧊️Geometry
/// ⏱️ Face/edge units one `tessellate` STEP may spend — the granularity at which a cancel can land
/// inside the kernel, never the round trip's own bound.
pub const PREVIEW_TESSELLATE_STEP_BUDGET: u32 = 24;

/// ⏱️ Wall-clock microseconds one `tessellate` ROUND TRIP may spend stepping before it answers.
///
/// ⚖️ This is the bound that actually matters, because a unit is not a unit of time: on this kernel
/// one step measured 29 µs and another 5.9 s. Bounding only the units made five consecutive
/// microsecond-long steps each pay a WHOLE `flowEvalTick` round trip — seconds apiece in a served
/// build — which is most of why `box-fillet-preview` needed eight round trips to paint and never
/// finished inside a patience window (`📓️preview-mesh-delivery-2026-09-12.md`).
pub const PREVIEW_TESSELLATE_STEP_WALL_MICROS: u64 = semio_framework_os_flow::mesh::TESSELLATE_STEP_WALL_MICROS;

/// 🔬️ The deflection one LOD step asks the kernel for — the ONE mapping both surfaces read, so an
/// editor mesh and a viewer mesh of the same handle at the same LOD are byte-identical.
pub fn preview_tolerance(lod_mode: &str) -> f64 {
    match lod_mode {
        "coarse" => 0.15,
        "fine" => 0.02,
        _ => 0.05,
    }
}

/// 🧊️ The flow-domain id of the geometry extension whose kernel tessellates every preview handle.
/// It is NOT an invocation address: the host resolves an extension actor by the CONTRIBUTING
/// PLUGIN's id, which [`semio_framework_os_flow::flow_extension_invocation_address`] — the ONE
/// translation surface — looks up from the live contribution table.
pub const GENERATION_3D_GEOMETRY_EXTENSION_ID: &str = "brep";

/// 🧊️ The geometry kernel's invocation address, resolved through the one translation surface every
/// producer and every status projection shares. Both callers own the same typed miss, so an
/// unaddressable kernel reaches a preview window as `phase: "faulted"` with its own English/German
/// label instead of a per-tick `eprintln!` nobody reads.
pub fn geometry_extension_address() -> Result<String, semio_framework_os_flow::FlowExtensionAddressMiss> {
    semio_framework_os_flow::flow_extension_invocation_address(GENERATION_3D_GEOMETRY_EXTENSION_ID)
}

pub fn is_brep_geometry_handle(handle: &str) -> bool {
    if handle.is_empty() {
        return false;
    }
    if handle.starts_with("solid-")
        || handle.starts_with("shell-")
        || handle.starts_with("face-")
        || handle.starts_with("wire-")
        || handle.starts_with("edge-")
        || handle.starts_with("vertex-")
        || handle.starts_with("compound-")
        || handle.starts_with("curve-")
        || handle.starts_with("surface-")
    {
        return true;
    }
    handle.len() == 64 && handle.as_bytes().iter().all(u8::is_ascii_hexdigit)
}

/// 🔌️ Point/vector geometry synthesized without a kernel round-trip, for a math-style output
/// channel that carries `x`/`y`/`z` coordinates instead of a brep handle.
#[derive(Clone, Debug, PartialEq)]
pub enum PreviewInlineGeometry {
    Point { x: f64, y: f64, z: f64 },
    Vector { x: f64, y: f64, z: f64 },
    Mesh { preview: String, data: String },
}

/// 🔌️ One previewable value found on one output channel of one widget — the channel-aware
/// replacement for unordered handle flattening, so preview instance ids can be channel-qualified
/// (`{widgetId}@{channel}#{index}`).
#[derive(Clone, Debug, PartialEq)]
pub struct PreviewChannelItem {
    pub channel: String,
    pub index: usize,
    pub handle: String,
    pub inline: Option<PreviewInlineGeometry>,
}

/// 🔎️ A `$schema: "list"` dictionary's entries in index order (`"0"`, `"1"`, …) — the wire form
/// flow dictionary lists actually take (an object with numeric-string keys, not a JSON array), so
/// ordering has to be recovered by parsing the keys rather than trusting map iteration.
fn preview_channel_list_entries(map: &semio_framework_pack_json::Object) -> Vec<&semio_framework_pack_json::Value> {
    let mut entries: Vec<(usize, &semio_framework_pack_json::Value)> = map.iter().filter_map(|(key, value)| key.parse::<usize>().ok().map(|index| (index, value))).collect();
    entries.sort_by_key(|(index, _)| *index);
    entries.into_iter().map(|(_, value)| value).collect()
}

/// 🔎️ Depth-first walk of one channel's evaluated value, emitting one [`PreviewChannelItem`] per
/// geometry-bearing leaf in encounter order. Arrays and `$schema: "list"` dictionaries recurse; a
/// handle passing [`is_brep_geometry_handle`] or an `x`/`y`/`z` point/vector is a leaf; everything
/// else is pure data and yields nothing.
fn collect_preview_channel_items(channel: &str, value: &semio_framework_pack_json::Value, index: &mut usize, items: &mut Vec<PreviewChannelItem>) {
    match value {
        semio_framework_pack_json::Value::Object(map) => {
            if map.get("$schema").and_then(semio_framework_pack_json::Value::as_str) == Some("mesh") {
                if let (Some(preview), Some(data)) = (map.get("preview").and_then(semio_framework_pack_json::Value::as_str), map.get("data").and_then(semio_framework_pack_json::Value::as_str)) {
                    items.push(PreviewChannelItem { channel: channel.into(), index: *index, handle: String::new(), inline: Some(PreviewInlineGeometry::Mesh { preview: preview.into(), data: data.into() }) });
                    *index += 1;
                }
                return;
            }
            if let Some(handle) = map.get("handle").and_then(semio_framework_pack_json::Value::as_str) {
                if is_brep_geometry_handle(handle) {
                    items.push(PreviewChannelItem { channel: channel.into(), index: *index, handle: handle.into(), inline: None });
                    *index += 1;
                    return;
                }
            }
            if map.get("$schema").and_then(semio_framework_pack_json::Value::as_str) == Some("list") {
                for entry in preview_channel_list_entries(map) {
                    collect_preview_channel_items(channel, entry, index, items);
                }
                return;
            }
            let coords = ["x", "y", "z"].into_iter().map(|key| map.get(key).and_then(semio_framework_pack_json::Value::as_f64)).collect::<Option<Vec<_>>>();
            if let Some(coords) = coords {
                let (x, y, z) = (coords[0], coords[1], coords[2]);
                let inline = if map.get("$schema").and_then(semio_framework_pack_json::Value::as_str) == Some("vector") { PreviewInlineGeometry::Vector { x, y, z } } else { PreviewInlineGeometry::Point { x, y, z } };
                items.push(PreviewChannelItem { channel: channel.into(), index: *index, handle: String::new(), inline: Some(inline) });
                *index += 1;
            }
        }
        semio_framework_pack_json::Value::Array(list) => {
            for entry in list {
                collect_preview_channel_items(channel, entry, index, items);
            }
        }
        _ => {}
    }
}

/// 🔌️ Channel-by-channel enumeration of one widget's preview-bearing values: sorted `"out"` channel
/// keys (falling back to `"in"` only when the widget has no `"out"` at all), each walked depth-first
/// into its geometry-bearing leaves.
pub fn preview_channel_items_for_widget(eval: &semio_framework_pack_json::Value, widget_id: &str) -> Vec<PreviewChannelItem> {
    let Some(widget_eval) = eval.get(widget_id) else {
        return Vec::new();
    };
    let Some(channels) = widget_eval.get("out").or_else(|| widget_eval.get("in")) else {
        return Vec::new();
    };
    let Some(map) = channels.as_object() else {
        return Vec::new();
    };
    let mut keys: Vec<&str> = map.iter().map(|(key, _)| key).collect();
    keys.sort();
    let mut items = Vec::new();
    for key in keys {
        let mut index = 0usize;
        if let Some(value) = map.get(key) {
            collect_preview_channel_items(key, value, &mut index, &mut items);
        }
    }
    items
}

/// 🪪️ Every preview-bearing widget id of a host_snapshot, in declaration order.
pub fn preview_widget_ids(host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot) -> Vec<String> {
    host_snapshot.widgets.iter().filter(|widget| semio_s_artifact_procedural_3d_model::widget_previews(widget)).map(|widget| semio_s_artifact_procedural_3d_model::widget_id(widget).to_string()).collect()
}

pub fn mesh_has_preview_geometry(data: &MeshData) -> bool {
    (!data.indices.is_empty() && data.positions.len() >= 9) || data.edge_positions.len() >= 6 || (data.positions.len() >= 3 && data.indices.is_empty())
}

/// 🏷️ Every role a published preview mesh may declare, in the order a reader ranks them: a body
/// with triangles, a body that is only a polyline, the axis cross of a point channel, the arrow of
/// a vector channel.
pub const PREVIEW_MESH_ROLES: [&str; 4] = ["solid", "wire", "point", "vector"];

/// 🏷️ What ONE published preview mesh is, stamped into the payload beside its geometry so a reader
/// never has to re-derive it from the arrays.
///
/// ⚖️ A preview publishes one mesh per geometry-bearing output CHANNEL, so a document whose graph
/// previews three nodes publishes a solid beside the wire it was extruded from and the vector that
/// drove it — three meshes for one visible body. Counting entries therefore says nothing about how
/// many solids are on screen, which is exactly the reconciliation the runtime oracles could not
/// make (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-mesh-oracle-2026-09-14.md`).
///
/// 🎨️ The role is what the payload CARRIES, not what the channel was named: `apply_show_mode_mesh`
/// strips triangles in `wireframe` mode, and a solid published without them really is a wire on
/// screen. The inline markers keep their own role because their arrays are indistinguishable from a
/// tessellated polyline.
pub fn preview_mesh_role(inline: Option<&PreviewInlineGeometry>, data: &MeshData) -> &'static str {
    match inline {
        Some(PreviewInlineGeometry::Point { .. }) => "point",
        Some(PreviewInlineGeometry::Vector { .. }) => "vector",
        _ if !data.indices.is_empty() => "solid",
        _ if !data.edge_positions.is_empty() => "wire",
        _ => "point",
    }
}

/// 📦️ The axis-aligned extent of everything ONE published preview payload puts on screen, read back
/// off the payload itself (`positions` plus `edgePositions`, so a wire-only example has an extent
/// too) rather than recomputed from the handles.
///
/// ⚖️ This is the delivery's own extent, and it is deliberately not `expect.boundingBox*`: the
/// committed `expect` box is the FINE tessellation the native lane measures, while the surface
/// paints the preview LOD, which inscribes it. The example fixtures commit both
/// (`delivery.boundingBoxMin/Max` beside `expect.boundingBoxMin/Max`), which is what lets the
/// browser oracle and the native lane grade the same number instead of an envelope
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️boot-camera-framing-2026-09-15.md`).
pub fn preview_payload_bounds(meshes_json: &str) -> Option<([f64; 3], [f64; 3])> {
    let parsed = semio_framework_pack_json::parse(meshes_json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?;
    let entries = parsed.as_array()?;
    let mut minimum = [f64::INFINITY; 3];
    let mut maximum = [f64::NEG_INFINITY; 3];
    for entry in entries {
        let Some(data) = entry.get("data") else { continue };
        for lane in ["positions", "edgePositions"] {
            let Some(values) = data.get(lane).and_then(semio_framework_pack_json::Value::as_array) else { continue };
            for point in values.chunks_exact(3) {
                for axis in 0..3 {
                    let Some(value) = point[axis].as_f64() else { continue };
                    minimum[axis] = minimum[axis].min(value);
                    maximum[axis] = maximum[axis].max(value);
                }
            }
        }
    }
    (0..3).all(|axis| minimum[axis].is_finite() && maximum[axis].is_finite()).then_some((minimum, maximum))
}

/// 🎯️ Headroom the preview leaves around the delivered bounds — the render hosts' own margin rides
/// on top of the exact fitting distance, so this padding only has to say "do not hug the box".
pub const PREVIEW_FIT_PADDING: f64 = 1.12;

/// 🎯️ Which DOCUMENT the preview is framing, as a number the render hosts key their one-shot fit on.
///
/// ⚖️ Hashed over the graph's TOPOLOGY (widget ids, their kinds, the synapse wiring) and never over
/// parameter values: switching example replaces the whole graph, while dragging a slider keeps it,
/// so this is exactly the line between "frame the example the user just opened" and "yank the camera
/// out from under an edit". Every re-evaluation of one example answers the same revision, which is
/// what makes the hosts' framing one-shot at all
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️boot-camera-framing-2026-09-15.md`).
pub fn preview_fit_revision(host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot) -> u32 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    let mut eat = |text: &str| {
        for byte in text.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    };
    eat(&host_snapshot.schema);
    for widget in &host_snapshot.widgets {
        eat(semio_s_artifact_procedural_3d_model::widget_id(widget));
        eat(widget_kind_tag(widget));
    }
    for synapse in &host_snapshot.synapses {
        eat(&synapse.from);
        eat(&synapse.from_port);
        eat(&synapse.to);
        eat(&synapse.to_port);
    }
    (hash >> 32) as u32
}

/// 🏷️ The kind tag [`preview_fit_revision`] hashes per widget — what the node IS, never what it is
/// currently set to.
fn widget_kind_tag(widget: &semio_framework_artifact_flow_flow::Widget) -> &str {
    use semio_framework_artifact_flow_flow::Widget;
    match widget {
        Widget::Neuron { neuron_kind, .. } => neuron_kind.as_str(),
        Widget::InputSlider { .. } => "input.slider",
        Widget::OutputPreview { .. } => "output.preview",
        _ => "widget",
    }
}

/// 🎯️ The `World3dScene.fit_json` every generation3d preview window publishes: frame the delivered
/// bounds once per document, never again while that document is on screen.
pub fn preview_fit_json(host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, meshes_json: &str) -> String {
    semio_framework_ui_declaration::ui::world3d_fit_json(preview_fit_revision(host_snapshot), PREVIEW_FIT_PADDING, preview_payload_bounds(meshes_json))
}

/// 🔌️ Half-extent (world units) of the axis cross drawn for a [`PreviewInlineGeometry::Point`].
const PREVIEW_POINT_MARKER_HALF_EXTENT: f64 = 0.05;

/// 🔌️ A small axis cross at `(x, y, z)` — the point-channel preview marker, built without a kernel
/// round-trip. Carries both `positions` (so `"points"` show mode still has something to draw once
/// [`apply_show_mode_mesh`] strips `edge_positions`) and the cross itself as edges.
pub fn point_marker_mesh(x: f64, y: f64, z: f64) -> MeshData {
    let (x, y, z) = (x as f32, y as f32, z as f32);
    let e = PREVIEW_POINT_MARKER_HALF_EXTENT as f32;
    MeshData { positions: vec![x, y, z], edge_positions: vec![x - e, y, z, x + e, y, z, x, y - e, z, x, y + e, z, x, y, z - e, x, y, z + e], ..Default::default() }
}

/// 🔌️ A line segment from the world origin to `(x, y, z)` — the vector-channel preview marker,
/// built without a kernel round-trip.
pub fn vector_marker_mesh(x: f64, y: f64, z: f64) -> MeshData {
    let (x, y, z) = (x as f32, y as f32, z as f32);
    MeshData { positions: vec![0.0, 0.0, 0.0, x, y, z], edge_positions: vec![0.0, 0.0, 0.0, x, y, z], ..Default::default() }
}

/// 👁️ The shading mode decides which mesh channels survive into the payload at all, so
/// wireframe/points really are cheaper.
pub fn apply_show_mode_mesh(mut data: MeshData, show_mode: &str) -> MeshData {
    let show_mode = match show_mode {
        "solid" | "shaded" | "shaded+edges" | "wireframe" | "points" => show_mode,
        _ => "shaded",
    };
    match show_mode {
        "wireframe" => {
            data.positions.clear();
            data.normals.clear();
            data.indices.clear();
            data.face_ids.clear();
            data
        }
        "points" => {
            data.indices.clear();
            data.normals.clear();
            data.edge_positions.clear();
            data
        }
        _ => data,
    }
}

/// 🎒️ Decodes a base64 `pack` mesh body the extension shipped back — the single decode seam every
/// preview reader goes through.
pub fn decode_preview_mesh_pack(base64_body: &str) -> Option<MeshData> {
    let bytes = semio_framework_os_flow::mesh::decode_base64(base64_body).ok()?;
    semio_framework_os_flow::mesh::decode_mesh_pack(&bytes).ok()
}

/// 🧵️ The mesh the CHAIN produced for this handle: the retained session's resolved `pack` body,
/// decoded into typed arrays. `None` while the tessellate round trip is still in flight, and `None`
/// forever once the handle carries diagnostics.
pub fn session_preview_mesh(handle: &str, session: &FlowEvalSession) -> Option<MeshData> {
    let data = decode_preview_mesh_pack(session.preview_mesh_pack(handle)?)?;
    mesh_has_preview_geometry(&data).then_some(data)
}

/// 🧵️ Rendering consumes only mesh output completed by the retained preview chain.
pub fn mesh_data_for_preview_handle(handle: &str, _tolerance: f64, session: Option<&FlowEvalSession>) -> Option<MeshData> {
    session_preview_mesh(handle, session?)
}

/// 🔁️ The converged walk's brep handle for the same preview channel leaf — used while a slider edit
/// has already minted a new handle but its tessellation has not landed.
/// 🔁 When the painted handle has no pack yet, the one OTHER packed handle this session still
/// holds — true after convergence superseded the handle and retired the old one from every eval text.
fn superseded_preview_mesh_handle(session: &FlowEvalSession, painted_handle: &str) -> Option<String> {
    let mut candidates = session
        .preview_packed_handles()
        .into_iter()
        .filter(|handle| handle != painted_handle && session_preview_mesh(handle, session).is_some())
        .collect::<Vec<_>>();
    match candidates.len() {
        0 => None,
        1 => Some(candidates.remove(0)),
        _ => None,
    }
}

fn converged_preview_handle(session: &FlowEvalSession, _host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, widget_id: &str, channel: &str, index: usize) -> Option<String> {
    let converged = session.converged_eval_json();
    if converged.is_empty() {
        return None;
    }
    let eval = semio_framework_pack_json::parse(converged, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| semio_framework_pack_json::Value::Object(semio_framework_pack_json::Object::new()));
    preview_channel_items_for_widget(&eval, widget_id)
        .into_iter()
        .find(|item| item.channel == channel && item.index == index)
        .map(|item| item.handle)
        .filter(|handle| !handle.is_empty())
}

/// 🧵️ Mesh lookup for one preview channel while a live evaluation is in flight: the new handle's
/// pack when it exists, otherwise the last converged handle's retained pack — never a second kernel
/// round trip on geometry the session already tessellated.
pub fn mesh_data_for_session_preview_channel(
    handle: &str,
    widget_id: &str,
    channel: &str,
    index: usize,
    tolerance: f64,
    session: &FlowEvalSession,
    host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot,
) -> Option<MeshData> {
    if let Some(data) = mesh_data_for_preview_handle(handle, tolerance, Some(session)) {
        return Some(data);
    }
    if session.preview_diagnostics(handle).is_some() {
        return None;
    }
    if let Some(fallback) = converged_preview_handle(session, host_snapshot, widget_id, channel, index).filter(|fallback| fallback != handle) {
        if let Some(data) = session_preview_mesh(&fallback, session) {
            return Some(data);
        }
    }
    superseded_preview_mesh_handle(session, handle).and_then(|fallback| session_preview_mesh(&fallback, session))
}

/// 📏️ Fingerprint of whichever mesh pack a preview channel would paint — the live handle's or the
/// converged fallback's while tessellation is outstanding.
pub fn preview_mesh_pack_fingerprint(
    session: &FlowEvalSession,
    host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot,
    widget_id: &str,
    item: &PreviewChannelItem,
) -> usize {
    if item.handle.is_empty() {
        return 0;
    }
    if let Some(pack) = session.preview_mesh_pack(&item.handle) {
        return pack.len();
    }
    if session.preview_diagnostics(&item.handle).is_some() {
        return 0;
    }
    if let Some(fallback) = converged_preview_handle(session, host_snapshot, widget_id, &item.channel, item.index).filter(|fallback| *fallback != item.handle) {
        if let Some(pack) = session.preview_mesh_pack(&fallback) {
            return pack.len();
        }
    }
    superseded_preview_mesh_handle(session, &item.handle)
        .and_then(|fallback| session.preview_mesh_pack(&fallback))
        .map_or(0, |pack| pack.len())
}

/// 📏️ What the preview would paint from `eval` given the session's current mesh packs — changes when
/// `flowTessellateResolve` lands even though the evaluation text did not move one byte.
fn preview_mesh_residency_digest(session: &FlowEvalSession, host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, eval: &semio_framework_pack_json::Value) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for (id, channel) in semio_s_artifact_procedural_3d_mesh_bridge::retained_geometry_channels(host_snapshot) {
        for item in preview_channel_items_for_widget(eval, &id).into_iter().filter(|item| channel.as_ref().is_none_or(|channel| *channel == item.channel)) {
            if item.handle.is_empty() {
                continue;
            }
            preview_mesh_pack_fingerprint(session, host_snapshot, &id, &item).hash(&mut hasher);
        }
    }
    hasher.finish()
}

/// 📤️ Whether the addressed preview window owes a fresh publication. The painted evaluation text is
/// the primary signal; mesh residency is the second — tessellation can finish without moving eval
/// json, and a signature that looked only at bytes pinned the first (empty or stale) table forever
/// (`📓️slider-mesh-supersession-fallback-2026-09-15.md`, viewer `preview_mesh_signature`).
pub fn preview_eval_publication_for(session: &mut FlowEvalSession, host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, retained_eval: Option<&str>) -> FlowEvalPublication {
    let painted = session.painted_eval_json().to_string();
    let digest = if painted.is_empty() {
        0
    } else {
        semio_framework_pack_json::parse(&painted, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|eval| preview_mesh_residency_digest(session, host_snapshot, &eval)).unwrap_or(0)
    };
    let publication = session.eval_publication_for(retained_eval);
    match publication {
        FlowEvalPublication::Changed(eval) => {
            session.note_published_preview_mesh_digest(digest);
            FlowEvalPublication::Changed(eval)
        }
        FlowEvalPublication::Retained if session.published_preview_mesh_digest() == digest => FlowEvalPublication::Retained,
        FlowEvalPublication::Retained => {
            session.note_published_preview_mesh_digest(digest);
            FlowEvalPublication::Changed((!painted.is_empty()).then_some(painted))
        }
    }
}

/// 🧹 Every brep handle whose tessellation pack must stay live across a handle supersession.
pub fn preview_mesh_retention_handles(eval: &semio_framework_pack_json::Value, host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, session: &FlowEvalSession) -> HashSet<String> {
    let mut live = HashSet::new();
    let mut channel_missing_pack = false;
    for widget in &host_snapshot.widgets {
        let id = semio_s_artifact_procedural_3d_model::widget_id(widget).to_string();
        for item in preview_channel_items_for_widget(eval, &id) {
            if item.handle.is_empty() {
                continue;
            }
            live.insert(item.handle.clone());
            if session_preview_mesh(&item.handle, session).is_none() && session.preview_diagnostics(&item.handle).is_none() {
                channel_missing_pack = true;
                if let Some(fallback) = converged_preview_handle(session, host_snapshot, &id, &item.channel, item.index) {
                    if fallback != item.handle {
                        live.insert(fallback);
                    }
                }
                if let Some(fallback) = superseded_preview_mesh_handle(session, &item.handle) {
                    live.insert(fallback);
                }
            }
        }
    }
    if channel_missing_pack {
        for handle in session.preview_packed_handles() {
            live.insert(handle);
        }
    }
    live
}

/// 🧊 Geometry handles on preview widgets that still need an extension tessellate. Takes the ALREADY
/// PARSED evaluation: its caller parses the same document one line earlier to collect the live
/// handle set, and re-parsing a whole eval session per tick is the cost this path exists to avoid.
pub fn pending_preview_tessellate_handles(eval: &semio_framework_pack_json::Value, host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, session: &FlowEvalSession) -> Vec<String> {
    let mut handles = Vec::new();
    for (id, channel) in semio_s_artifact_procedural_3d_mesh_bridge::retained_geometry_channels(host_snapshot) {
        for handle in preview_channel_items_for_widget(eval, &id).into_iter().filter(|item| channel.as_ref().is_none_or(|channel| *channel == item.channel)).filter_map(|item| (!item.handle.is_empty()).then_some(item.handle)) {
            if session.preview_diagnostics(&handle).is_some() {
                continue;
            }
            if session_preview_mesh(&handle, session).is_none() {
                handles.push(handle);
            }
        }
    }
    handles
}

/// 📨 Extension invocations that tessellate preview handles inside the owning brep extension kernel.
/// Each one names `flowTessellateResolve` as its response action, so the SDK's request registry
/// mints the `req`, parks the continuation and dispatches the mesh JSON straight back into the
/// addressed surface — a hand-minted `RequestId` owns no registry slot, so every tessellation result
/// would be discarded and the 3d preview could never paint.
pub fn preview_tessellate_invocations(window_id: &str, window_kind_id: &str, session: &mut FlowEvalSession, host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, tolerance: f64, grant: semio_framework_value::RetainedCloneGrant) -> Result<Vec<ExtensionInvocation>, semio_framework_value::ValueError> {
    let Ok(geometry_extension_address) = geometry_extension_address() else {
        return Ok(Vec::new());
    };
    let tolerance_bits = tolerance.to_bits();
    let (live, pending) = {
        let eval_json = session.painted_eval_json();
        if eval_json.is_empty() {
            return Ok(Vec::new());
        }
        let eval = semio_framework_pack_json::parse(eval_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| semio_framework_pack_json::Value::Object(semio_framework_pack_json::Object::new()));
        let live = preview_mesh_retention_handles(&eval, host_snapshot, session);
        (live, pending_preview_tessellate_handles(&eval, host_snapshot, session))
    };
    retain_preview_meshes(session, live, grant)?;
    let mut invocations = Vec::new();
    for handle in pending {
        let node_hash = semio_framework_os_flow::preview_tessellate_node_hash(&handle, tolerance_bits);
        if session.note_pending_tessellate(node_hash, handle.clone()) {
            let mut request_object = semio_framework_pack_json::Object::new();
            request_object.insert("handle", semio_framework_pack_json::Value::String(handle));
            request_object.insert("tolerance", semio_framework_pack_json::Value::from(tolerance));
            request_object.insert("nodeHash", semio_framework_pack_json::Value::from(node_hash));
            request_object.insert("budget", semio_framework_pack_json::Value::from(u64::from(PREVIEW_TESSELLATE_STEP_BUDGET)));
            request_object.insert("wallMicros", semio_framework_pack_json::Value::from(PREVIEW_TESSELLATE_STEP_WALL_MICROS));
            request_object.insert("windowId", semio_framework_pack_json::Value::String(window_id.to_string()));
            request_object.insert("windowKindId", semio_framework_pack_json::Value::String(window_kind_id.to_string()));
            let request_json = semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Object(request_object));
            invocations.push(ExtensionInvocation::new(geometry_extension_address.clone(), "tessellate", request_json, "flowTessellateResolve"));
        }
    }
    Ok(invocations)
}

/// 🧹️ Retains exactly the live roster of preview meshes, one granted retention turn at a time, and refuses a retention that stops making progress.
pub fn retain_preview_meshes(session: &mut FlowEvalSession, live: HashSet<String>, grant: semio_framework_value::RetainedCloneGrant) -> Result<(), semio_framework_value::ValueError> {
    session.begin_retain_preview_meshes(live.into_iter().collect()).map_err(|(error, _)| error)?;
    while !session.preview_retention_terminal_is_empty() {
        session.retain_preview_meshes_step(grant)?;
        if session.preview_retention_step_progress() == Default::default() && !session.preview_retention_terminal_is_empty() {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "preview mesh retention was granted too little to advance"));
        }
    }
    Ok(())
}
//#endregion 🧊️Geometry
