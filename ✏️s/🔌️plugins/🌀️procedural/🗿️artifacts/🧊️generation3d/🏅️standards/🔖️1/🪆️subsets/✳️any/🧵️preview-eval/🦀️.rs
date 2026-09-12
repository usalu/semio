//! 🧵️ Surface-neutral preview evaluation chain — the ONE `flowEvalTick` law both generation3d
//! surfaces run.
//!
//! ⏱️ The law, in order: an attached preview roster arms `flowEvalTick` addressed at that preview
//! window → the tick advances the retained [`FlowEvalSession`] and emits an
//! [`semio_framework_plugin::ExtensionInvocation`] (`evaluate` while operators are still pending,
//! then `tessellate`) or re-arms itself → `flowEvalResolve` / `flowTessellateResolve` fold the
//! answer back and re-arm → the tick publishes the evaluation into the addressed window's retained
//! transient → the window's own render reads it. No surface may run a synchronous
//! `FlowEvalSession::tick` loop instead: the brep/math operators are CONTRIBUTED by the host at
//! runtime (`setContributions`) and are never linked into the guest, so an in-guest tick can only
//! ever fault (`📓️audit-window-inventory-2026-09-12.md` §4 P0 item 1).
//!
//! 🪆️ This module lives at the SUBSET level, beside `✏️editor` and `👁️viewer` rather than inside
//! either, because both run the identical chain and `policyViewerPurityBreaches` forbids a viewer
//! file reaching through `::editor::`. Everything a surface owns alone — its config type, its
//! interaction marks, its payload assembly, its window chrome — stays on that surface; everything
//! the chain itself is made of lives here exactly once (CLAUDE.md: repeated code MUST be close to
//! each other). Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.

use semio_framework_os_flow::{flow_host_with_session, FlowEvalPublication, FlowEvalSession};
use semio_framework_plugin::{Effect, ExtensionInvocation, MeshData, ViewModel};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🪟️Addressing
/// 🪟️ The evaluation tick names the preview window that OWNS the evaluation it advances.
///
/// The tick publishes into one window's retained window transient, so the retained route is
/// window-scoped and its work refuses any command that does not name that window. The chain is
/// entirely self-dispatched — a surface's `pending_effects` arms the first tick and every
/// tick/resolve re-arms the next through `Effect::DispatchAction` — and an effect carries no window
/// of its own: the shell redispatches it under whichever window is current. Carrying the id ON THE
/// PAYLOAD is how `retained_window_transient_target` can capture the preview window's transient
/// authority (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "flow-eval-tick")]
pub struct FlowEvalTick {
    pub window_id: String,
    pub window_kind_id: String,
}

/// ✅️ One `evaluate` round trip's answer, echoed back onto the response action by
/// `reactor::extension_response_args` — including the window address, so the re-armed tick keeps
/// addressing the preview window that owns this evaluation.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "flow-eval-resolve")]
pub struct FlowEvalResolve {
    pub window_id: String,
    pub window_kind_id: String,
    pub node_hash: u64,
    pub output_json: String,
    /// 🪪️ Which extension this answer came from, echoed back off the request's own correlation.
    #[value(default)]
    pub extension_id: String,
    /// ✅️ The SDK's own outcome flag (`reactor::extension_response_args`), present on every answer
    /// the runtime builds. Defaulted so a hand-authored fixture may still address only the fields it
    /// cares about.
    #[value(default)]
    pub ok: bool,
    /// 💥 The decoded fault of a refused answer, verbatim from the host completion the SDK unpacked.
    #[value(default)]
    pub fault_code: String,
    #[value(default)]
    pub fault_message: String,
}

/// 🔺️ One budgeted `tessellate` round trip's answer, carrying the same echoed window address so a
/// resumable tessellation re-arms the tick on the window that owns the mesh it is building.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "flow-tessellate-resolve")]
pub struct FlowTessellateResolve {
    pub window_id: String,
    pub window_kind_id: String,
    pub node_hash: u64,
    pub output_json: String,
}

/// 🧯️ The geometry extension's `tessellateCancel` answer, echoed back onto the cancel's own response
/// action with the window address it was asked under. `retired` is how many kernel jobs the
/// extension actor actually retired — publication-free bookkeeping, kept because a cancel that
/// retires nothing on an actor that was supposed to be busy is the one symptom that distinguishes
/// "the gesture reached the kernel" from "the gesture reached only this process".
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "flow-tessellate-cancel-resolve")]
pub struct FlowTessellateCancelResolve {
    pub window_id: String,
    pub window_kind_id: String,
    #[value(default)]
    pub output_json: String,
    #[value(default)]
    pub ok: bool,
}

/// 🛑️ The user's explicit "stop computing this preview" gesture, addressed at the preview window
/// whose evaluation it stops. The window address is not decoration: the `tessellateCancel` round
/// trip this gesture emits must come back to the SAME window (`reactor::extension_response_args`
/// echoes a request's own fields onto the response action), and a surface that cancels one of two
/// open previews may not silently stop the other.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "cancel-preview-eval")]
pub struct CancelPreviewEval {
    #[value(default)]
    pub window_id: String,
    #[value(default)]
    pub window_kind_id: String,
}

/// 🛑️ The verb a preview window's status contract names as its `cancelAction`. The shell learns the
/// cancel affordance from the surface's own published status and from nothing else
/// (`🌐️World3dHost/🟦️.tsx` `declareSurfaceCancelAction`), so every surface that publishes this
/// status must also DECLARE this command — see `🧫️fixtures/🛑️preview-cancel.json` `surfaces`.
pub const PREVIEW_CANCEL_ACTION_ID: &str = "cancelPreviewEval";

/// ⏱️ Units ONE budgeted `evaluate` step spends before the round trip re-checks its wall deadline.
/// Mirrors `flow_extension_sdk::EVALUATE_STEP_BUDGET`; declared here because the REQUESTER picks
/// the budget its own interactivity needs, and the extension only honours it.
pub const EVALUATE_STEP_BUDGET: u64 = 8;

/// ⌛️ Wall-clock allowance for ONE `evaluate` round trip. Deliberately far below the host shard
/// watchdog's 16 s silence threshold AND below its 5 s ordinary heartbeat window: a worker that
/// returns to its event loop every 2 s cannot be mistaken for a dead one, which is exactly the
/// mistake that killed every actor on shard 0 when `brep.bool.cut` ran 16 s inside one turn
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub const EVALUATE_STEP_WALL_MICROS: u64 = 2_000_000;

/// 🔁️ The self-redispatch every hop of the chain arms, addressed to the SAME preview window.
pub fn rearm(window_id: &str, window_kind_id: &str, req: u64) -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(req), action: "flowEvalTick".into(), args: Some(window_args(window_id, window_kind_id)), delay_ms: 0 }
}

/// 🚧️ Whether an unfinished evaluation of `fixture` may arm another tick at all.
///
/// A graph whose operator kinds no contributed extension serves faults
/// `flow.extension-not-contributed`, and NOTHING a tick does can change that: the evaluator answers
/// `unknown kind`, the error dictionary is never cached, and the next tick recomputes the identical
/// miss. The pre-contribution chain therefore used to re-arm itself forever — and because every
/// settle drives a host `refreshUi`, the spin also starved the very push that would have fixed it
/// (45 s of live console with zero `[DEBUG] contributions …` lines). The ONE thing that resumes
/// this chain is `setContributions`, whose route invalidates the retained session against the moved
/// registry generation and re-arms one tick per attached preview window
/// (`📓️contributions-rearm-2026-09-10.md`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// 🪪️ This supersedes `📓️fault-arm-symmetry-2026-09-10.md`'s "the fault arm is symmetric with the
/// ok arm": that report is about how a fault VALUE is encoded on the extension-result wire (pack,
/// not JSON) and says nothing about who owes the next tick. Symmetry of encoding is not symmetry of
/// continuation — a fault nothing in this process can clear owes no continuation at all.
pub fn may_rearm(fixture: &semio_framework_artifact_flow_flow::FlowFixture) -> bool {
    semio_framework_os_flow::unserved_flow_operator_kinds(fixture).is_empty()
}

/// 🪟️ The one argument object every hop of the chain carries, on the redispatch and on the
/// extension request alike — `reactor::extension_response_args` echoes an invocation request's own
/// fields back onto the response action, so the window address survives the round trip without the
/// SDK ever learning what it means.
pub fn window_args(window_id: &str, window_kind_id: &str) -> dsl::DslValue {
    dsl::DslValue::object([
        ("windowId".to_string(), dsl::DslValue::String(window_id.to_string())),
        ("windowKindId".to_string(), dsl::DslValue::String(window_kind_id.to_string())),
    ])
}

/// 🧵️ Re-arms every attached preview window's chain — what a gesture owes after it changed what the
/// evaluation would produce (`setActiveExample`, `setContributions`, the generation commands, every
/// viewer view command). Arming from the gesture's own emit makes the restart a consequence of the
/// gesture rather than of a host refresh that may be narrowed, coalesced or lost.
pub fn rearm_attached_previews(session: &mut FlowEvalSession, windows: &[(&str, &str)]) -> Vec<Effect> {
    windows.iter().filter(|(window_id, _)| session.arm_window_tick(window_id)).map(|(window_id, window_kind_id)| rearm(window_id, window_kind_id, 105)).collect()
}

/// 🪟️ The preview window kind this surface recognises, or `None` — the surface hands in its own
/// roster of preview kind ids (`procedural-preview` + `generation3d-generate-preview` for the
/// editor, `procedural-view-preview` for the viewer) and the answer is the `'static` id the window
/// transient owner is keyed by.
pub fn preview_kind(kind: &str, preview_kinds: &[&'static str]) -> Option<&'static str> {
    preview_kinds.iter().copied().find(|candidate| *candidate == kind)
}

/// 🪟️ Every attached preview window, in roster order — the windows a `flowEvalTick` chain can be
/// addressed to. Each holds its OWN retained evaluation publication, so each gets its own armed
/// chain; an empty answer means no surface is mounted yet and NOTHING may be armed at all.
pub fn attached_preview_windows<'a>(view: Option<&'a ViewModel>, preview_kinds: &[&'static str]) -> Vec<(&'a str, &'static str)> {
    view.map(|view| view.window_instances.iter().filter_map(|window| preview_kind(&window.window_kind_id, preview_kinds).map(|kind| (window.id.as_str(), kind))).collect()).unwrap_or_default()
}
//#endregion 🪟️Addressing

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
pub const PREVIEW_TESSELLATE_STEP_WALL_MICROS: u64 = semio_framework_os_flow::brep_geometry::TESSELLATE_STEP_WALL_MICROS;

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
    // Blake3 hex digests minted by `BrepKernel::mint` (no kind prefix).
    handle.len() == 64 && handle.as_bytes().iter().all(u8::is_ascii_hexdigit)
}

/// 🔌️ Point/vector geometry synthesized without a kernel round-trip, for a math-style output
/// channel that carries `x`/`y`/`z` coordinates instead of a brep handle.
#[derive(Clone, Debug, PartialEq)]
pub enum PreviewInlineGeometry {
    Point { x: f64, y: f64, z: f64 },
    Vector { x: f64, y: f64, z: f64 },
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
fn preview_channel_list_entries(map: &dsl::json::Object) -> Vec<&dsl::json::Value> {
    let mut entries: Vec<(usize, &dsl::json::Value)> = map.iter().filter_map(|(key, value)| key.parse::<usize>().ok().map(|index| (index, value))).collect();
    entries.sort_by_key(|(index, _)| *index);
    entries.into_iter().map(|(_, value)| value).collect()
}

/// 🔎️ Depth-first walk of one channel's evaluated value, emitting one [`PreviewChannelItem`] per
/// geometry-bearing leaf in encounter order. Arrays and `$schema: "list"` dictionaries recurse; a
/// handle passing [`is_brep_geometry_handle`] or an `x`/`y`/`z` point/vector is a leaf; everything
/// else is pure data and yields nothing.
fn collect_preview_channel_items(channel: &str, value: &dsl::json::Value, index: &mut usize, items: &mut Vec<PreviewChannelItem>) {
    match value {
        dsl::json::Value::Object(map) => {
            if let Some(handle) = map.get("handle").and_then(dsl::json::Value::as_str) {
                if is_brep_geometry_handle(handle) {
                    items.push(PreviewChannelItem { channel: channel.into(), index: *index, handle: handle.into(), inline: None });
                    *index += 1;
                    return;
                }
            }
            if map.get("$schema").and_then(dsl::json::Value::as_str) == Some("list") {
                for entry in preview_channel_list_entries(map) {
                    collect_preview_channel_items(channel, entry, index, items);
                }
                return;
            }
            let coords = ["x", "y", "z"].into_iter().map(|key| map.get(key).and_then(dsl::json::Value::as_f64)).collect::<Option<Vec<_>>>();
            if let Some(coords) = coords {
                let (x, y, z) = (coords[0], coords[1], coords[2]);
                let inline = if map.get("$schema").and_then(dsl::json::Value::as_str) == Some("vector") { PreviewInlineGeometry::Vector { x, y, z } } else { PreviewInlineGeometry::Point { x, y, z } };
                items.push(PreviewChannelItem { channel: channel.into(), index: *index, handle: String::new(), inline: Some(inline) });
                *index += 1;
            }
        }
        dsl::json::Value::Array(list) => {
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
pub fn preview_channel_items_for_widget(eval: &dsl::json::Value, widget_id: &str) -> Vec<PreviewChannelItem> {
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

/// 👁️ Whether a widget contributes preview geometry at all. A `Neuron` carries its own author-set
/// `preview` toggle; an `OutputPreview` is a preview by construction; a `Cluster` has no toggle of
/// its own, so its contract output channels always preview, which is the only way a grouped
/// sub-graph's geometry reaches the 3D world at all.
pub fn widget_previews(widget: &semio_framework_artifact_flow_flow::Widget) -> bool {
    matches!(widget, semio_framework_artifact_flow_flow::Widget::Neuron { preview: true, .. } | semio_framework_artifact_flow_flow::Widget::OutputPreview { .. } | semio_framework_artifact_flow_flow::Widget::Cluster { .. })
}

/// 🪪️ Every preview-bearing widget id of a fixture, in declaration order.
pub fn preview_widget_ids(fixture: &semio_framework_artifact_flow_flow::FlowFixture) -> Vec<String> {
    fixture.widgets.iter().filter(|widget| widget_previews(widget)).map(|widget| crate::widget_id(widget).to_string()).collect()
}

pub fn mesh_has_preview_geometry(data: &MeshData) -> bool {
    (!data.indices.is_empty() && data.positions.len() >= 9) || data.edge_positions.len() >= 6 || (data.positions.len() >= 3 && data.indices.is_empty())
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
    let bytes = semio_framework_os_flow::brep_geometry::decode_base64(base64_body).ok()?;
    semio_framework_os_flow::brep_geometry::decode_mesh_pack(&bytes).ok()
}

/// 🧵️ The mesh the CHAIN produced for this handle: the retained session's resolved `pack` body,
/// decoded into typed arrays. `None` while the tessellate round trip is still in flight, and `None`
/// forever once the handle carries diagnostics.
pub fn session_preview_mesh(handle: &str, session: &FlowEvalSession) -> Option<MeshData> {
    let data = decode_preview_mesh_pack(session.preview_mesh_pack(handle)?)?;
    mesh_has_preview_geometry(&data).then_some(data)
}

/// 🧵️ Pure per-render mesh lookup: the chain's own answer first, and only a SESSION-FREE caller
/// (the mesh-export bridge, a schema test) falls back to tessellating in-process — a served guest
/// links no geometry kernel, so that fallback is never the live path.
pub fn mesh_data_for_preview_handle(handle: &str, tolerance: f64, session: Option<&FlowEvalSession>) -> Option<MeshData> {
    if let Some(session) = session {
        if let Some(data) = session_preview_mesh(handle, session) {
            return Some(data);
        }
        if session.preview_diagnostics(handle).is_some() {
            return None;
        }
    }
    let data = semio_framework_os_flow::tessellate_geometry(handle, tolerance).ok()?;
    mesh_has_preview_geometry(&data).then_some(data)
}

/// 🧊 Geometry handles on preview widgets that still need an extension tessellate. Takes the ALREADY
/// PARSED evaluation: its caller parses the same document one line earlier to collect the live
/// handle set, and re-parsing a whole eval session per tick is the cost this path exists to avoid.
pub fn pending_preview_tessellate_handles(eval: &dsl::json::Value, fixture: &semio_framework_artifact_flow_flow::FlowFixture, session: &FlowEvalSession) -> Vec<String> {
    let mut handles = Vec::new();
    for id in preview_widget_ids(fixture) {
        for handle in preview_channel_items_for_widget(eval, &id).into_iter().filter_map(|item| (!item.handle.is_empty()).then_some(item.handle)) {
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
pub fn preview_tessellate_invocations(window_id: &str, window_kind_id: &str, session: &mut FlowEvalSession, fixture: &semio_framework_artifact_flow_flow::FlowFixture, tolerance: f64) -> Vec<ExtensionInvocation> {
    let Ok(geometry_extension_address) = geometry_extension_address() else {
        return Vec::new();
    };
    let tolerance_bits = tolerance.to_bits();
    let (live, pending) = {
        let eval_json = session.eval_json();
        if eval_json.is_empty() {
            return Vec::new();
        }
        let eval = dsl::json::parse(eval_json).unwrap_or_else(|_| dsl::json::Value::Object(dsl::json::Object::new()));
        let mut live = std::collections::HashSet::new();
        for widget in &fixture.widgets {
            let id = crate::widget_id(widget).to_string();
            for item in preview_channel_items_for_widget(&eval, &id) {
                if !item.handle.is_empty() {
                    live.insert(item.handle);
                }
            }
        }
        (live, pending_preview_tessellate_handles(&eval, fixture, session))
    };
    session.retain_preview_meshes(&live);
    let mut invocations = Vec::new();
    for handle in pending {
        let node_hash = semio_framework_os_flow::preview_tessellate_node_hash(&handle, tolerance_bits);
        if session.note_pending_tessellate(node_hash, handle.clone()) {
            let mut request_object = dsl::json::Object::new();
            request_object.insert("handle", dsl::json::Value::String(handle));
            request_object.insert("tolerance", dsl::json::Value::from(tolerance));
            request_object.insert("nodeHash", dsl::json::Value::from(node_hash));
            request_object.insert("budget", dsl::json::Value::from(u64::from(PREVIEW_TESSELLATE_STEP_BUDGET)));
            request_object.insert("wallMicros", dsl::json::Value::from(PREVIEW_TESSELLATE_STEP_WALL_MICROS));
            request_object.insert("chunk", dsl::json::Value::from(u64::from(session.next_tessellate_chunk(node_hash))));
            request_object.insert("windowId", dsl::json::Value::String(window_id.to_string()));
            request_object.insert("windowKindId", dsl::json::Value::String(window_kind_id.to_string()));
            let request_json = dsl::json::to_string(&dsl::json::Value::Object(request_object));
            invocations.push(ExtensionInvocation::new(geometry_extension_address.clone(), "tessellate", request_json, "flowTessellateResolve"));
        }
    }
    invocations
}
//#endregion 🧊️Geometry

//#region ⏱️Tick
/// ⏱️ What one evaluation tick owes its caller: the self-redispatch effects, the extension work it
/// declared, and whether the addressed window's retained publication actually changed.
pub struct FlowEvalTickOutcome {
    pub effects: Vec<Effect>,
    pub extension_invocations: Vec<ExtensionInvocation>,
    pub publication: FlowEvalPublication,
}

/// 🧮️ ONE evaluation tick of the shared chain, for ANY surface.
///
/// ⏱️ The tick's own wall cost is recorded into `semio_framework_os_flow`'s evaluation-step ledger
/// ONLY while `semio_framework_job::runtime_diagnostics_enabled()` is armed — this is the ONE app
/// work step the 8 ms interactive ceiling governs, and a normal boot must not pay two clock reads
/// per tick to measure it.
///
/// 📤️ `retained_eval` is the evaluation the addressed window ALREADY holds — the tick republishes
/// only when it differs, so redispatch ticks that move no node allocate and retire nothing.
pub fn evaluate_tick(
    window_id: &str,
    window_kind_id: &str,
    fixture: &semio_framework_artifact_flow_flow::FlowFixture,
    tolerance: f64,
    session: &mut FlowEvalSession,
    retained_eval: Option<&str>,
) -> FlowEvalTickOutcome {
    let started_us = semio_framework_job::runtime_diagnostics_enabled().then(semio_framework_job::default_now_us).flatten();
    // ▶️ The armed tick is now RUNNING, so its latch is free for whatever THIS tick decides to arm.
    session.begin_window_tick(window_id);
    let mut host = flow_host_with_session(fixture, session);
    let more = session.tick(&mut host);
    let pending_extension_eval = host.take_pending_extension_eval();
    host.retire_cold();
    let mut extension_invocations = Vec::new();
    if let Some(pending) = pending_extension_eval {
        // 🪪️ `extensionId` is CORRELATION, not payload: `reactor::extension_response_args` echoes the
        // request's own fields back onto `flowEvalResolve`, and a refused answer has to be able to
        // name which extension refused it — the tick's geometry address is a different extension
        // from the one an operator hop was routed to (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        let request_json = dsl::json::to_json_string(&dsl::DslValue::object([
            ("operatorId".to_string(), dsl::DslValue::String(pending.operator_id.clone())),
            ("inputJson".to_string(), dsl::DslValue::String(pending.input_json.clone())),
            ("nodeHash".to_string(), dsl::DslValue::uint(pending.node_hash)),
            // ⏱️ The budget the extension's `evaluate` step honours. `nodeHash` doubles as the key
            // its retained job is resumed by, so an identical re-emitted request continues the SAME
            // evaluation instead of restarting it — the exact resumption mechanism budgeted
            // `tessellate` already uses (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
            ("budget".to_string(), dsl::DslValue::uint(EVALUATE_STEP_BUDGET)),
            ("wallMicros".to_string(), dsl::DslValue::uint(EVALUATE_STEP_WALL_MICROS)),
            ("windowId".to_string(), dsl::DslValue::String(window_id.to_string())),
            ("windowKindId".to_string(), dsl::DslValue::String(window_kind_id.to_string())),
            ("extensionId".to_string(), dsl::DslValue::String(pending.extension_id.clone())),
        ]));
        extension_invocations.push(ExtensionInvocation::new(pending.extension_id, "evaluate", request_json, "flowEvalResolve"));
    } else if !more {
        extension_invocations.extend(preview_tessellate_invocations(window_id, window_kind_id, session, fixture, tolerance));
    }
    // ⏳️ A tick that parked extension work owes NO re-arm: the answers own the continuation, and a
    // second tick on the same window would recompute the identical pending request and park a
    // duplicate. A tick that parked nothing re-arms itself only while the live registry can still
    // serve this graph ([`may_rearm`]) AND the latch admits one — an uncontributed graph publishes
    // what it could compute and stops (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    session.note_window_tick_outcome(window_id, more);
    let effects = if !extension_invocations.is_empty() {
        session.note_window_extensions_in_flight(window_id, extension_invocations.len());
        Vec::new()
    } else if more && !may_rearm(fixture) {
        // 🚧️ Nothing this process can do clears an uncontributed operator kind, so the chain gives
        // up rather than leaving the refresh poll a standing debt to re-arm every turn.
        session.abandon_window_tick(window_id);
        Vec::new()
    } else if more && session.arm_window_tick(window_id) {
        vec![rearm(window_id, window_kind_id, 103)]
    } else {
        Vec::new()
    };
    let publication = session.eval_publication_for(retained_eval);
    if let (Some(started_us), Some(finished_us)) = (started_us, started_us.and_then(|_| semio_framework_job::default_now_us())) {
        semio_framework_os_flow::record_flow_eval_step(finished_us.saturating_sub(started_us));
    }
    FlowEvalTickOutcome { effects, extension_invocations, publication }
}

/// ✅️ Folds one `evaluate` answer into the retained session and re-arms the addressed chain — once.
///
/// 🚧️ An answer that could NOT be folded is a fault settle, not slow work: the host answers a
/// faulted `invokeExtension` with an empty `outputJson`, the node cache keeps no entry, and the next
/// tick would park the identical request and fault again at the host's own cadence forever. Such a
/// settle owes no continuation, for the same reason [`may_rearm`] gives an uncontributed graph.
pub fn resolve_eval(payload: &FlowEvalResolve, session: &mut FlowEvalSession) -> Vec<Effect> {
    // ⏱️ A budgeted `evaluate` answers an ENVELOPE, not an out dictionary: a step that spent its
    // wall allowance without finishing says `done: false` and parks its job under this same
    // `nodeHash`, so the chain owes one more identical round trip and nothing may be seeded yet.
    // Folding it here (rather than in every surface) keeps the whole budget law in the chain
    // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️extension-evaluate-budget-2026-09-12.md`).
    let outcome = session.resolve_preview_eval(payload.node_hash, &payload.output_json);
    if let semio_framework_os_flow::PreviewEvalOutcome::Working = outcome {
        note_eval_answer_fault(payload, session);
        let discharged = session.settle_window_extension(&payload.window_id);
        let armed = session.arm_window_tick(&payload.window_id);
        return if discharged || armed { vec![rearm(&payload.window_id, &payload.window_kind_id, 102)] } else { Vec::new() };
    }
    // 🛑 A cancelled evaluation owes NOTHING: the gesture already quiesced every latch, and arming
    // here would restart the chain the user stopped.
    if let semio_framework_os_flow::PreviewEvalOutcome::Cancelled = outcome {
        session.settle_window_extension(&payload.window_id);
        return Vec::new();
    }
    let output_json = match &outcome {
        semio_framework_os_flow::PreviewEvalOutcome::Complete { output_json } => output_json.as_str(),
        _ => "",
    };
    let seeded = session.seed_node_cache(payload.node_hash, output_json).is_ok();
    if !seeded {
        eprintln!("flowEvalResolve could not seed the node cache for nodeHash={} ({} output bytes)", payload.node_hash, output_json.len());
    }
    // 💥 What the surface publishes has to be the fault this answer actually carried, not the
    // addressing miss that preceded the install. The SDK already decoded it onto the response
    // action; retaining it here is the only thing between that and the preview's status object.
    note_eval_answer_fault(payload, session);
    if !seeded {
        session.abandon_window_tick(&payload.window_id);
    }
    let discharged = session.settle_window_extension(&payload.window_id);
    let armed = seeded && session.arm_window_tick(&payload.window_id);
    if discharged || armed {
        vec![rearm(&payload.window_id, &payload.window_kind_id, 102)]
    } else {
        Vec::new()
    }
}

/// 💥 Retains (or forgets) the evaluate fault one answer carried. Publication only — no arming.
fn note_eval_answer_fault(payload: &FlowEvalResolve, session: &mut FlowEvalSession) {
    if payload.ok || payload.fault_code.is_empty() {
        session.clear_extension_evaluate_fault();
        return;
    }
    session.note_extension_evaluate_fault(semio_framework_os_flow::ExtensionEvaluateFault {
        extension_id: payload.extension_id.clone(),
        capability: "evaluate".to_string(),
        code: payload.fault_code.clone(),
        message: payload.fault_message.clone(),
    });
}

/// 🛑️ The whole cancel gesture, for ANY surface: retire what THIS process owns, then reach the
/// kernel jobs it does not.
///
/// 🚪️ The two halves are not interchangeable. `FlowEvalSession::cancel_preview_evaluation` empties
/// the guest's pending table, freezes its progress ledger, resets its chunk cursors and quiesces its
/// arming latches — all inside the generation3d app's own wasm instance. The `TessellationJob`s
/// themselves live in the geometry extension's instance, behind its own process-global registry, and
/// the guest's linked copy of `brep_geometry::cancel_all_tessellations` can only ever see an EMPTY
/// registry from here (two components, two globals). The extension's `tessellateCancel` capability
/// is the only door onto them, so the gesture emits one invocation through it and folds the answer
/// on [`resolve_tessellate_cancel`] (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// An unaddressable geometry extension emits nothing: there is no actor to tell, and the local half
/// has already happened.
pub fn cancel_preview_eval(payload: &CancelPreviewEval, session: &mut FlowEvalSession) -> Vec<ExtensionInvocation> {
    cancel_preview_eval_for(payload, session, geometry_extension_address())
}

/// 🛑️ The gesture itself, over an ALREADY RESOLVED geometry address — pure in both its inputs, for
/// the same reason [`crate::editor::generation3d::preview_progress_status_json_for`] is: proving the
/// unaddressable branch by uninstalling the contribution poisons the flow catalogue's cache lock for
/// every later test in the binary.
pub fn cancel_preview_eval_for(payload: &CancelPreviewEval, session: &mut FlowEvalSession, address: Result<String, semio_framework_os_flow::FlowExtensionAddressMiss>) -> Vec<ExtensionInvocation> {
    session.cancel_preview_evaluation(&payload.window_id);
    let Ok(address) = address else {
        return Vec::new();
    };
    let request_json = FlowEvalSession::preview_cancel_invocation_request_json(&payload.window_id, &payload.window_kind_id);
    // ⏱️ TWO doors, because there are two registries of retained kernel work in that actor and each
    // names its own: `tessellateCancel` retires the mesh jobs, `evaluateCancel` retires the parked
    // budgeted operator evaluations (a `brep.bool.cut` mid-validation is in the second and in
    // neither of the others). Both answers land on the same fold, which arms nothing
    // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️extension-evaluate-budget-2026-09-12.md`).
    vec![
        ExtensionInvocation::new(address.clone(), "evaluateCancel", FlowEvalSession::preview_eval_cancel_invocation_request_json(&payload.window_id, &payload.window_kind_id), "flowTessellateCancelResolve"),
        ExtensionInvocation::new(address, "tessellateCancel", request_json, "flowTessellateCancelResolve"),
    ]
}

/// 🧯️ Folds the `tessellateCancel` answer. Deliberately arms NOTHING and publishes nothing: the
/// gesture already left every latch quiescent, and re-arming here would restart the chain the user
/// stopped. The answer is only ever observed.
pub fn resolve_tessellate_cancel(payload: &FlowTessellateCancelResolve, session: &mut FlowEvalSession) {
    let _ = session;
    if !payload.ok {
        eprintln!("flowTessellateCancelResolve: geometry extension refused the cancel for window {}", payload.window_id);
    }
}

/// ✅️ Folds one budgeted `tessellate` round trip into the retained session. A step that neither
/// finished the mesh nor received its last body chunk re-arms the tick chain, which is what turns a
/// one-shot synchronous tessellation into a resumable job the user can watch and stop.
pub fn resolve_tessellate(payload: &FlowTessellateResolve, session: &mut FlowEvalSession) -> Vec<Effect> {
    let outcome = session.resolve_preview_tessellate(payload.node_hash, &payload.output_json);
    let discharged = session.settle_window_extension(&payload.window_id);
    let armed = outcome.needs_another_round_trip() && session.arm_window_tick(&payload.window_id);
    if discharged || armed {
        vec![rearm(&payload.window_id, &payload.window_kind_id, 107)]
    } else {
        Vec::new()
    }
}
//#endregion ⏱️Tick

//#region 📈️Status
/// 📈️ The ONE status object EVERY generation3d World3d preview window publishes — edit mode's
/// `procedural-preview`, generate mode's `generation3d-generate-preview` and the viewer's
/// `procedural-view-preview`. One projection, one schema, one place.
///
/// 🐛️ It used to live on `✏️editor`, so the viewer's preview published a `World3dScene` with
/// `status_json: None`: meshes rendered, but `data-status-json` carried no `phase`, no `progress`
/// and no `cancellable`, i.e. a read-only surface could watch nothing and stop nothing
/// (`🗑️generated/journey-7/results.json`, rows `view:*`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
/// A viewer file may not reach through `::editor::` (`policyViewerPurityBreaches`), so the fix is
/// the same one the tick chain got: the projection moves HERE and both surfaces bind it.
///
/// 🔒️ `session` is OPTIONAL because a surface's marks-free `render` entry point is handed no
/// retained session at all. A window with no session still publishes the full contract — idle
/// phase, zero progress, nothing to cancel — rather than the empty status that was the whole defect.

/// 📈️ The per-widget half: the evaluation's own `error`, or the `widgetErrors` map of every widget
/// whose evaluation carries one. Surface-neutral — it reads the evaluation text and the fixture the
/// surface is looking at, nothing else.
pub fn preview_status_json(eval_json: &str, fixture: &semio_framework_artifact_flow_flow::FlowFixture) -> Option<String> {
    let eval = dsl::json::parse(eval_json).ok()?;
    if eval.get("error").and_then(dsl::json::Value::as_str).is_some() {
        let mut error_object = dsl::json::Object::new();
        error_object.insert("error", eval.get("error").cloned().unwrap_or(dsl::json::Value::Null));
        return Some(dsl::json::to_string(&dsl::json::Value::Object(error_object)));
    }
    let mut errors = dsl::json::Object::new();
    for widget in &fixture.widgets {
        let id = crate::widget_id(widget).to_string();
        let Some(entry) = eval.get(&id) else { continue };
        if let Some(error) = entry.get("error").and_then(dsl::json::Value::as_str) {
            errors.insert(id, dsl::json::Value::String(error.to_string()));
        }
    }
    if errors.is_empty() {
        None
    } else {
        let mut wrapper = dsl::json::Object::new();
        wrapper.insert("widgetErrors", dsl::json::Value::Object(errors));
        Some(dsl::json::to_string(&dsl::json::Value::Object(wrapper)))
    }
}

/// 🧩️ Folds one status object's keys onto another's, right-hand side winning.
fn merge_status_json(computing: Option<String>, preview_status: Option<String>) -> Option<String> {
    match (computing, preview_status) {
        (Some(c), Some(p)) => {
            let mut computing_object = dsl::json::parse(&c).ok().and_then(|value| value.as_object().cloned()).unwrap_or_else(|| {
                let mut fallback = dsl::json::Object::new();
                fallback.insert("computing", dsl::json::Value::Bool(true));
                fallback
            });
            let preview_object = dsl::json::parse(&p).ok().and_then(|value| value.as_object().cloned()).unwrap_or_default();
            for (key, value) in preview_object.iter() {
                computing_object.insert(key, value.clone());
            }
            Some(dsl::json::to_string(&dsl::json::Value::Object(computing_object)))
        }
        (Some(c), None) => Some(c),
        (None, Some(p)) => Some(p),
        (None, None) => None,
    }
}

/// 👁️ Merges the session's live "still computing" flag, the resumable tessellation's progress and a
/// fresh [`preview_status_json`] result into the one status object a preview window publishes.
pub fn preview_scene_status_json(session: Option<&FlowEvalSession>, preview_status: Option<String>) -> Option<String> {
    let computing = session.is_some_and(FlowEvalSession::pending).then(|| r#"{"computing":true}"#.to_string());
    merge_status_json(merge_status_json(computing, Some(preview_progress_status_json(session))), preview_status)
}

/// 📈 The schema-first tessellation progress object: `phase` (wire tag) and its `phaseLabel`
/// English/German pair, the monotone `progress` counters and ratio, `cancellable` (drives the
/// `cancelPreviewEval` affordance) and any typed validate-gate `diagnostics`.
///
/// 🪪️ An unaddressable geometry kernel outranks whatever the tessellation ledger last recorded:
/// nothing was ever invoked, so the ledger's `idle` is a lie the surface cannot act on. The miss is
/// resolved through the SAME [`geometry_extension_address`] the invocation producer uses — read
/// live, never cached — and published as `phase: "faulted"` plus a `fault` object naming the flow
/// extension id and every `<manifest id> → <owning plugin id>` translation the session does carry.
pub fn preview_progress_status_json(session: Option<&FlowEvalSession>) -> String {
    preview_progress_status_json_for(session, geometry_extension_address())
}

/// 📈 The projection itself, over an ALREADY RESOLVED geometry address — pure in both its inputs, so
/// the unaddressable branch is provable without mutating the process-global contribution table (a
/// test that uninstalls and reinstalls it poisons the flow catalogue's cache lock and leaves the
/// neural registry unretired for every later test in the binary).
pub fn preview_progress_status_json_for(session: Option<&FlowEvalSession>, address: Result<String, semio_framework_os_flow::FlowExtensionAddressMiss>) -> String {
    let status = session.map(FlowEvalSession::preview_tessellate_status).unwrap_or_default();
    // ⏱️ The budgeted-evaluation ledger — the half of a preview's work the tessellation ledger is
    // structurally blind to, because a long operator admits no tessellation until it has finished
    // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️extension-evaluate-budget-2026-09-12.md`).
    let eval_status = session.map(FlowEvalSession::preview_eval_status).unwrap_or_default();
    // 🛑 The explicit gesture OUTRANKS the tessellation ledger, which knows nothing about a cancel
    // raised while the chain was still in its `evaluate` round trips (no tessellation admitted yet,
    // so no progress row to stamp) — that case used to publish `phase: "idle"` and swallow the
    // gesture whole (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    let phase = if address.is_err() {
        semio_framework_os_flow::PreviewTessellatePhase::Faulted
    } else if session.is_some_and(FlowEvalSession::preview_cancelled) {
        semio_framework_os_flow::PreviewTessellatePhase::Cancelled
    } else {
        status.phase
    };
    // 🛑 `cancellable` is "is there work a cancel would stop", and the tessellation ledger answers
    // only half of that: an `evaluate` round trip — the slow half of a boolean preview — admits no
    // tessellation at all, so `status.phase` reads `idle` and `in_flight` reads 0 throughout it. The
    // session's own in-flight extension count and its scheduled-tick flag are the other half, and a
    // cancel that has already landed is never cancellable again.
    let work_in_flight = status.is_cancellable() || eval_status.is_cancellable() || session.is_some_and(|session| session.extensions_in_flight() > 0) || session.is_some_and(FlowEvalSession::pending);
    let cancellable = work_in_flight && !session.is_some_and(FlowEvalSession::preview_cancelled) && address.is_ok();
    // ⏱️ While a budgeted evaluation is the ONLY work outstanding, the surface names THAT phase —
    // the tessellation ledger would say `idle` for the whole of it.
    let show_eval_phase = eval_status.in_flight > 0 && status.in_flight == 0 && matches!(phase, semio_framework_os_flow::PreviewTessellatePhase::Idle);
    let (phase_tag, english, german) = if show_eval_phase {
        let (english, german) = eval_status.phase.labels();
        (eval_status.phase.tag(), english, german)
    } else {
        let (english, german) = phase.labels();
        (phase.tag(), english, german)
    };
    let mut label = dsl::json::Object::new();
    label.insert("en", dsl::json::Value::String(english.to_string()));
    label.insert("de", dsl::json::Value::String(german.to_string()));
    let mut progress = dsl::json::Object::new();
    progress.insert("unitsDone", dsl::json::Value::from(u64::from(status.units_done)));
    progress.insert("unitsTotal", dsl::json::Value::from(u64::from(status.units_total)));
    progress.insert("facesDone", dsl::json::Value::from(u64::from(status.faces_done)));
    progress.insert("facesTotal", dsl::json::Value::from(u64::from(status.faces_total)));
    // ⏱️ `inFlight` counts BOTH kinds of outstanding kernel work. A boolean preview spends its
    // whole slow half in `evaluate` round trips that admit no tessellation, so the tessellation
    // ledger alone published `inFlight: 0` while the kernel was busy for sixteen seconds
    // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    progress.insert("inFlight", dsl::json::Value::from(u64::from(status.in_flight.saturating_add(eval_status.in_flight))));
    progress.insert("evalUnitsDone", dsl::json::Value::from(u64::from(eval_status.units_done)));
    progress.insert("evalUnitsTotal", dsl::json::Value::from(u64::from(eval_status.units_total)));
    progress.insert("ratio", dsl::json::Value::from(if eval_status.in_flight > 0 && status.in_flight == 0 { eval_status.ratio() } else { status.ratio() }));
    let mut object = dsl::json::Object::new();
    object.insert("phase", dsl::json::Value::String(phase_tag.to_string()));
    object.insert("phaseLabel", dsl::json::Value::Object(label));
    object.insert("progress", dsl::json::Value::Object(progress));
    object.insert("cancellable", dsl::json::Value::Bool(cancellable));
    object.insert("cancelAction", dsl::json::Value::String(PREVIEW_CANCEL_ACTION_ID.to_string()));
    // 💥 A live evaluate fault OUTRANKS the ledger phase for the same reason an addressing miss
    // does — the surface must state what it is actually living with. It can only exist once the
    // address resolved (an unaddressable extension is never invoked), so the two fault objects are
    // mutually exclusive by construction, and a contribution install clears this one
    // (`FlowEvalSession::invalidate_for_flow_extension_registry`) before the first tick settles.
    if let Some(evaluate_fault) = session.and_then(FlowEvalSession::extension_evaluate_fault) {
        let (english, german) = evaluate_fault.labels();
        let mut message = dsl::json::Object::new();
        message.insert("en", dsl::json::Value::String(english));
        message.insert("de", dsl::json::Value::String(german));
        let mut fault = dsl::json::Object::new();
        fault.insert("code", dsl::json::Value::String(semio_framework_os_flow::ExtensionEvaluateFault::CODE.to_string()));
        fault.insert("extensionId", dsl::json::Value::String(evaluate_fault.extension_id.clone()));
        fault.insert("capability", dsl::json::Value::String(evaluate_fault.capability.clone()));
        fault.insert("faultCode", dsl::json::Value::String(evaluate_fault.code.clone()));
        fault.insert("faultMessage", dsl::json::Value::String(evaluate_fault.message.clone()));
        fault.insert("message", dsl::json::Value::Object(message));
        let (faulted_en, faulted_de) = semio_framework_os_flow::PreviewTessellatePhase::Faulted.labels();
        let mut faulted_label = dsl::json::Object::new();
        faulted_label.insert("en", dsl::json::Value::String(faulted_en.to_string()));
        faulted_label.insert("de", dsl::json::Value::String(faulted_de.to_string()));
        object.insert("phase", dsl::json::Value::String(semio_framework_os_flow::PreviewTessellatePhase::Faulted.tag().to_string()));
        object.insert("phaseLabel", dsl::json::Value::Object(faulted_label));
        object.insert("cancellable", dsl::json::Value::Bool(false));
        object.insert("fault", dsl::json::Value::Object(fault));
    } else if let Err(miss) = address {
        let (english, german) = miss.labels();
        let mut message = dsl::json::Object::new();
        message.insert("en", dsl::json::Value::String(english));
        message.insert("de", dsl::json::Value::String(german));
        let mut fault = dsl::json::Object::new();
        fault.insert("code", dsl::json::Value::String(semio_framework_os_flow::FlowExtensionAddressMiss::CODE.to_string()));
        fault.insert("extensionId", dsl::json::Value::String(miss.extension_id));
        fault.insert("message", dsl::json::Value::Object(message));
        fault.insert(
            "contributed",
            dsl::json::Value::Array(
                miss.contributed
                    .into_iter()
                    .map(|(extension_id, plugin_id)| {
                        let mut entry = dsl::json::Object::new();
                        entry.insert("extensionId", dsl::json::Value::String(extension_id));
                        entry.insert("pluginId", dsl::json::Value::String(plugin_id));
                        dsl::json::Value::Object(entry)
                    })
                    .collect(),
            ),
        );
        object.insert("fault", dsl::json::Value::Object(fault));
    }
    if status.diagnostics > 0 {
        object.insert("diagnosticCount", dsl::json::Value::from(u64::from(status.diagnostics)));
        let entries: Vec<dsl::json::Value> = session
            .map(FlowEvalSession::preview_diagnostic_entries)
            .unwrap_or_default()
            .into_iter()
            .map(|(handle, issues)| {
                let mut entry = dsl::json::Object::new();
                entry.insert("handle", dsl::json::Value::String(handle.to_string()));
                entry.insert("issues", dsl::json::parse(issues).unwrap_or(dsl::json::Value::Array(Vec::new())));
                dsl::json::Value::Object(entry)
            })
            .collect();
        object.insert("diagnostics", dsl::json::Value::Array(entries));
    }
    dsl::json::to_string(&dsl::json::Value::Object(object))
}

/// 🐞️ The observable counters a preview window stamps onto its status so the browser probe can read
/// back off `data-status-json` what actually reached the scene, without a screenshot.
pub struct PreviewStatusDebug<'a> {
    pub eval_json: &'a str,
    pub meshes_json: &'a str,
    pub instances_json: &'a str,
}

/// 📈️ THE preview-window status projection: progress + widget errors + debug counters + the
/// optional authored hint a window shows while it has nothing to paint. All three World3d preview
/// windows publish exactly this, so a probe, a shell pane and a law can name one shape.
pub fn preview_window_status_json(session: Option<&FlowEvalSession>, widget_status: Option<String>, debug: &PreviewStatusDebug<'_>, hint: Option<&str>) -> Option<String> {
    let base = preview_scene_status_json(session, widget_status);
    let mut object = base.as_deref().and_then(|text| dsl::json::parse(text).ok()).and_then(|value| value.as_object().cloned()).unwrap_or_else(dsl::json::Object::new);
    let mut debug_object = dsl::json::Object::new();
    debug_object.insert("evalLen", dsl::json::Value::from(debug.eval_json.len()));
    debug_object.insert("meshesLen", dsl::json::Value::from(debug.meshes_json.len()));
    debug_object.insert("instancesLen", dsl::json::Value::from(debug.instances_json.len()));
    debug_object.insert("evalHead", dsl::json::Value::String(debug.eval_json.chars().take(240).collect::<String>()));
    object.insert("debug", dsl::json::Value::Object(debug_object));
    if let Some(hint) = hint {
        object.insert("hint", dsl::json::Value::String(hint.to_string()));
    }
    Some(dsl::json::to_string(&dsl::json::Value::Object(object)))
}
//#endregion 📈️Status

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
