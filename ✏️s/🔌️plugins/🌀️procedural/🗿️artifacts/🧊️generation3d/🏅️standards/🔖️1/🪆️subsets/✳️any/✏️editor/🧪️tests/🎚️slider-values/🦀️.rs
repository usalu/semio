//! 🎚️ What a MOVED slider must DELIVER — the correctness half of the gesture lane, over the whole
//! bundled roster.
//!
//! ⚖️ The oracle is independent of the chain under test: the same example graph is evaluated through
//! `with_host` at the released value and tessellated in-process at the same preview LOD, which is the
//! very path `📚️examples/🧪️tests/🧩️geometry` grades against `parry3d`. The chain's answer has to
//! travel the whole served route instead — one `nodeGraphEdit` per coalesced round trip, the
//! `previewEval` run's hops, the `brep` extension's `evaluate`/`tessellate` answers — and then equal
//! the oracle mesh for mesh and corner for corner.
//!
//! 🚦️ A value the kernel genuinely cannot build is NOT an exception to that: the oracle carries the
//! node's own `error`, and the chain must publish that error with its message rather than an empty
//! payload the surface reports as settled.
//!
//! @see `🧫️fixtures/🎚️slider-values.json` — the rows.
//! @see `📓️slider-reevaluation-correctness-2026-09-15.md`.

use super::*;
use crate::editor::generation3d::unit_tests::context::{self, app_with_registry};
use crate::standards::v1::subsets::any::schema::{example_snapshot, snapshot::Generation3dSnapshotRead, with_host};
use semio_framework_plugin::PluginApp;
use serde::Deserialize;
use std::collections::BTreeMap;

const SLIDER_VALUES_FIXTURE_JSON: &str = include_str!("../../../🧫️fixtures/🎚️slider-values.json");

//#region 📇️Fixture
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SliderValuesFixture {
    schema: String,
    lod_mode: String,
    bounds_tolerance: f64,
    /// 🤝️ Named defects this fixture GRADES but does not own — a row tagged with one must still
    /// fail, so the hand-on cannot rot into a silent pass when somebody fixes it.
    handed_on: BTreeMap<String, String>,
    rows: Vec<SliderExampleRow>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SliderExampleRow {
    example: String,
    slider: String,
    gestures: Vec<SliderGestureRow>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SliderGestureRow {
    row: String,
    kind: String,
    values: Vec<f64>,
    #[serde(default)]
    handed_on: Option<String>,
}

fn slider_values_fixture() -> SliderValuesFixture {
    let mut fixture: SliderValuesFixture = serde_json::from_str(SLIDER_VALUES_FIXTURE_JSON).expect("the slider-values fixture parses");
    assert_eq!(fixture.schema, "s.procedural.generation3d.slider-values/v1", "the slider-values fixture declares its own schema");
    // 🔬️ `SEMIO_SLIDER_EXAMPLES=a,b` narrows a DIAGNOSTIC run to those examples. The kernel's shape
    // store is process-wide, so "does this row still fail when nothing else is in the store?" is a
    // question a diagnosis has to be able to ask; an unset variable grades the whole roster.
    if let Ok(only) = std::env::var("SEMIO_SLIDER_EXAMPLES") {
        let wanted: Vec<&str> = only.split(',').map(str::trim).filter(|entry| !entry.is_empty()).collect();
        fixture.rows.retain(|row| wanted.contains(&row.example.as_str()));
        assert!(!fixture.rows.is_empty(), "SEMIO_SLIDER_EXAMPLES={only} names no row of the fixture");
    }
    fixture
}
//#endregion 📇️Fixture

//#region 🧿️Oracle
/// 🧿️ What the preview MUST carry for one slider value — the published mesh ids in payload order,
/// the payload's own extent, and the per-widget evaluation errors the value produces.
#[derive(Debug, Default, PartialEq)]
struct PreviewReading {
    mesh_ids: Vec<String>,
    bounds: Option<([f64; 3], [f64; 3])>,
    widget_errors: BTreeMap<String, String>,
}

impl PreviewReading {
    fn is_empty(&self) -> bool {
        self.mesh_ids.is_empty()
    }
}

/// 🧿️ The independent answer: the example's own graph with ONE slider moved, evaluated and
/// tessellated in this process. `session: None` is what makes it independent — the payload builder
/// falls back to the linked `tessellate_geometry` bridge instead of reading the chain's mesh packs.
fn oracle_reading(example_id: &str, slider_id: &str, value: f64, lod_mode: &str) -> PreviewReading {
    let read = Generation3dSnapshotRead::new(example_snapshot(example_id).expect("bundled example snapshot"));
    let config = Generation3dConfig { lod_mode: lod_mode.to_string(), ..Default::default() };
    with_host(&read.host_snapshot, |host| {
        host.set_slider_value(slider_id, value);
        let eval_json = host.evaluate().unwrap_or_default();
        let payload = crate::editor::generation3d::preview_payload(&eval_json, &host.host_snapshot, &config, None, &crate::editor::generation3d::PreviewInteractionMarks::default());
        let mut widget_errors = widget_errors_of(crate::preview_eval::preview_status_json(&eval_json, &host.host_snapshot).as_deref());
        if payload_mesh_ids(&payload.meshes_json).is_empty() && widget_errors.is_empty() {
            widget_errors.extend(tessellation_refusals(&eval_json, host, crate::preview_eval::preview_tolerance(lod_mode)));
        }
        PreviewReading { mesh_ids: payload_mesh_ids(&payload.meshes_json), bounds: crate::preview_eval::preview_payload_bounds(&payload.meshes_json), widget_errors }
    })
}

/// 🩺️ Why an evaluation that named no error still put nothing on screen: the kernel's own refusal of
/// each preview handle, read straight off the tessellator. Without this the oracle would call a value
/// "empty" where the product is entitled to call it faulted, and the law could not tell the two apart.
fn tessellation_refusals(eval_json: &str, host: &semio_framework_os_flow::FlowHost, tolerance: f64) -> BTreeMap<String, String> {
    let Ok(eval) = dsl::json::parse(eval_json) else { return BTreeMap::new() };
    let mut refusals = BTreeMap::new();
    for widget_id in crate::preview_eval::preview_widget_ids(&host.host_snapshot) {
        for item in crate::preview_eval::preview_channel_items_for_widget(&eval, &widget_id) {
            if item.handle.is_empty() {
                continue;
            }
            match semio_framework_os_flow::tessellate_geometry(&item.handle, tolerance) {
                Err(error) => {
                    refusals.insert(widget_id.clone(), error.to_string());
                }
                Ok(mesh) if !crate::preview_eval::mesh_has_preview_geometry(&mesh) => {
                    refusals.insert(widget_id.clone(), format!("the kernel tessellated {} into no geometry at all", item.handle));
                }
                Ok(_) => {}
            }
        }
    }
    refusals
}

/// 🧿️ Every value this fixture grades, answered ONCE and up front — keyed by example and by the
/// value's own bits, because a float is the key here and `f64` is not `Ord`.
fn oracle_table(fixture: &SliderValuesFixture) -> BTreeMap<(String, u64), PreviewReading> {
    let mut table = BTreeMap::new();
    for row in &fixture.rows {
        let mut values: Vec<f64> = vec![authored_slider_value(&row.example, &row.slider)];
        values.extend(row.gestures.iter().filter_map(|gesture| gesture.values.last().copied()));
        for value in values {
            table.entry((row.example.clone(), value.to_bits())).or_insert_with(|| oracle_reading(&row.example, &row.slider, value, &fixture.lod_mode));
        }
    }
    table
}

/// 🎚️ The value the example itself authors for one slider — the state the boot row is graded at.
fn authored_slider_value(example_id: &str, slider_id: &str) -> f64 {
    let read = Generation3dSnapshotRead::new(example_snapshot(example_id).expect("bundled example snapshot"));
    read.host_snapshot
        .widgets
        .iter()
        .find_map(|widget| match widget {
            semio_framework_artifact_flow_flow::Widget::InputSlider { id, value, .. } if id == slider_id => Some(*value),
            _ => None,
        })
        .unwrap_or_else(|| panic!("{example_id} authors no slider {slider_id}"))
}

fn payload_mesh_ids(meshes_json: &str) -> Vec<String> {
    let parsed: serde_json::Value = serde_json::from_str(meshes_json).expect("payload meshes json");
    parsed.as_array().cloned().unwrap_or_default().iter().filter_map(|entry| entry.get("id").and_then(serde_json::Value::as_str).map(str::to_string)).collect()
}

fn widget_errors_of(status_json: Option<&str>) -> BTreeMap<String, String> {
    let Some(text) = status_json else { return BTreeMap::new() };
    let parsed: serde_json::Value = serde_json::from_str(text).unwrap_or(serde_json::Value::Null);
    let mut errors = BTreeMap::new();
    if let Some(map) = parsed.get("widgetErrors").and_then(serde_json::Value::as_object) {
        for (key, value) in map {
            errors.insert(key.clone(), value.as_str().unwrap_or_default().to_string());
        }
    }
    if let Some(message) = parsed.get("error").and_then(serde_json::Value::as_str) {
        errors.insert("$graph".to_string(), message.to_string());
    }
    errors
}
//#endregion 🧿️Oracle

//#region 🚚️Delivery
/// 📈️ The whole published preview scene of one settled chain — what a renderer would paint.
#[derive(Debug, Default)]
struct DeliveredPreview {
    reading: PreviewReading,
    phase: String,
    computing: bool,
}

/// 🚚️ 🐛 The status's `diagnostics` row is `[{handle, issues:[{entity, code, message}]}]`
/// (`🧵️preview-eval/🦀️.rs`), one NESTING deeper than this reader used to walk — it read `entity` and
/// `message` off the outer entry, found neither, and recorded every validate-gate refusal as the
/// contentless `$kernel: ""`. A refusal read that way names nothing, so a row whose kernel genuinely
/// refuses the value looked like a silent empty payload.
fn delivered_preview(preview_body_json: &str) -> DeliveredPreview {
    let world: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(preview_body_json).expect("projected preview body must decode as an assembled world-3d scene");
    let status: serde_json::Value = serde_json::from_str(world.status_json.as_deref().unwrap_or("{}")).expect("preview status json");
    let mut widget_errors = widget_errors_of(world.status_json.as_deref());
    for entry in status.get("diagnostics").and_then(serde_json::Value::as_array).cloned().unwrap_or_default() {
        let handle = entry.get("handle").and_then(serde_json::Value::as_str).unwrap_or("$kernel");
        for issue in entry.get("issues").and_then(serde_json::Value::as_array).cloned().unwrap_or_default() {
            let entity = issue.get("entity").and_then(serde_json::Value::as_str).unwrap_or(handle).to_string();
            let code = issue.get("code").and_then(serde_json::Value::as_str).unwrap_or_default();
            let message = issue.get("message").and_then(serde_json::Value::as_str).unwrap_or_default();
            widget_errors.insert(entity, format!("[{code}] {message}").trim().to_string());
        }
    }
    DeliveredPreview {
        reading: PreviewReading { mesh_ids: payload_mesh_ids(&world.meshes_json), bounds: crate::preview_eval::preview_payload_bounds(&world.meshes_json), widget_errors },
        phase: status.get("phase").and_then(serde_json::Value::as_str).unwrap_or("<absent>").to_string(),
        computing: status.get("computing").and_then(serde_json::Value::as_bool).unwrap_or(false),
    }
}

/// 🎚️ The dispatch ONE coalesced round trip of a press carries, exactly as `useGraphSliderLanes`
/// builds it (`🕸️NodeGraph/🟦️.tsx`).
fn set_slider_operations(slider_id: &str, value: f64, gesture: &str, commit: bool) -> Generation3dCommand {
    let operations = serde_json::json!([{ "operation": "setSlider", "widgetId": slider_id, "value": value, "gesture": gesture, "commit": commit }]);
    Generation3dCommand::NodeGraphEdit(crate::editor::generation3d::commands::node_graph_edit::NodeGraphEdit { operations_json: operations.to_string() })
}

/// 🎚️ One press, driven the way the host lane drives it: a `drag` lets each round trip settle before
/// offering the next value, a `burst` lands every value of the press before any evaluation answers —
/// which is exactly how a value supersedes an evaluation already in flight.
async fn drive_slider_press(app: &mut context::Generation3dApp, flow_view: &semio_framework_plugin::ViewModel, slider_id: &str, values: &[f64], burst: bool, gesture: &str) {
    let last = values.len().saturating_sub(1);
    let mut carried = Vec::new();
    for (index, value) in values.iter().enumerate() {
        let receipt = context::dispatch_with_view(app, set_slider_operations(slider_id, *value, gesture, index == last), flow_view.clone()).await.expect("the slider dispatch lands");
        carried.extend(receipt.effects);
        if !burst {
            context::drive_preview_run(app, flow_view, &std::mem::take(&mut carried)).await;
        }
    }
    context::drive_preview_run(app, flow_view, &carried).await;
}
//#endregion 🚚️Delivery

//#region ⚖️Laws
/// ⚖️ LAW: a moved slider converges on the geometry its released value MAKES — on every bundled
/// example, for a drag inside the range, at both range ends and for a burst.
///
/// 🐛️ Before this law: six of the eight examples settled on an EMPTY payload reported as `idle`, a
/// burst ended in `PreviewTessellatePhase::Invalid` and never returned, and a range-end value made
/// the geometry vanish while the census said `1 of 7 nodes did not evaluate`
/// (`📓️slider-preview-update-2026-09-15.md` §7 items 1–2).
#[semio_framework_async_macros::async_test]
async fn every_moved_slider_converges_on_the_geometry_its_released_value_makes() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let fixture = slider_values_fixture();
    // 🧿️ Every oracle FIRST, before the app exists. The kernel's shape store is process-wide, so an
    // oracle taken between two gestures would be a second live evaluation competing for it — which is
    // exactly the interference this lane fixed and must not re-introduce into its own measurement.
    let oracles = oracle_table(&fixture);
    let mut app = app_with_registry().await;
    let (flow_view, preview_view) = context::shell_views(flow_window::GENERATION_3D_PLAY_WINDOW_MAIN, edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW);
    let mut faults: Vec<String> = Vec::new();
    for row in &fixture.rows {
        app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": row.example }).into()), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("setActiveExample dispatches");
        let switched = context::settle(&mut app).await;
        context::drive_preview_run(&mut app, &flow_view, &switched.effects).await;
        // 🪪️ The example's OWN authored value, before any slider moves — so a row that was already
        // broken on arrival is attributed to the example and never to the gesture.
        let authored = authored_slider_value(&row.example, &row.slider);
        let boot_oracle = &oracles[&(row.example.clone(), authored.to_bits())];
        let boot = delivered_preview(&context::render_with_view(&mut app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW, &preview_view).await);
        let boot_label = format!("{} · boot · {} = {authored}", row.example, row.slider);
        eprintln!("[DEBUG] slider delivery {boot_label}: meshes={:?} oracle={:?} phase={} errors={:?} oracleErrors={:?}", boot.reading.mesh_ids, boot_oracle.mesh_ids, boot.phase, boot.reading.widget_errors, boot_oracle.widget_errors);
        faults.extend(press_faults(&boot_label, boot_oracle, &boot, fixture.bounds_tolerance));
        for (index, gesture) in row.gestures.iter().enumerate() {
            let released = *gesture.values.last().expect("a press releases on a value");
            let oracle = &oracles[&(row.example.clone(), released.to_bits())];
            drive_slider_press(&mut app, &flow_view, &row.slider, &gesture.values, gesture.kind == "burst", &format!("{}-{index}", gesture.row)).await;
            let delivered = delivered_preview(&context::render_with_view(&mut app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW, &preview_view).await);
            let label = format!("{} · {} · {} = {released}", row.example, gesture.row, row.slider);
            eprintln!(
                "[DEBUG] slider delivery {label}: meshes={:?} oracle={:?} phase={} computing={} errors={:?} oracleErrors={:?}",
                delivered.reading.mesh_ids, oracle.mesh_ids, delivered.phase, delivered.computing, delivered.reading.widget_errors, oracle.widget_errors
            );
            let row_faults = press_faults(&label, oracle, &delivered, fixture.bounds_tolerance);
            match &gesture.handed_on {
                None => faults.extend(row_faults),
                Some(owner) => {
                    let reason = fixture.handed_on.get(owner).unwrap_or_else(|| panic!("{label} is handed on to {owner}, which the fixture does not declare"));
                    if row_faults.is_empty() {
                        faults.push(format!("{label}: converges now, but the fixture still hands it on to {owner} — delete the tag and the reason: {reason}"));
                    } else {
                        eprintln!("[DEBUG] handed on to {owner}: {}", row_faults.join(" | "));
                    }
                }
            }
        }
    }
    assert!(faults.is_empty(), "a moved slider did not converge on its value's geometry:\n{}", faults.join("\n"));
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// ⚖️ Everything one released value owes, as a list of what it failed rather than the first
/// assertion that fires — a per-example verdict is worth far more than a stack trace on row one.
fn press_faults(label: &str, oracle: &PreviewReading, delivered: &DeliveredPreview, bounds_tolerance: f64) -> Vec<String> {
    let mut faults = Vec::new();
    if delivered.computing {
        faults.push(format!("{label}: the chain settled but the surface still publishes `computing`"));
    }
    if !oracle.widget_errors.is_empty() {
        // 🚦️ A value the kernel cannot build owes a NAMED fault, not a silent empty payload. The
        // message need not be the oracle's word for word — the chain reaches the kernel through the
        // extension and the oracle in-process — but SOMETHING must name a node and say why.
        let named: Vec<&String> = delivered.reading.widget_errors.iter().filter(|(_, message)| !message.trim().is_empty()).map(|(widget, _)| widget).collect();
        if named.is_empty() {
            faults.push(format!(
                "{label}: the kernel cannot build this value ({:?}) and the surface names NOTHING — it publishes {} meshes under phase {} with errors {:?}",
                oracle.widget_errors,
                delivered.reading.mesh_ids.len(),
                delivered.phase,
                delivered.reading.widget_errors
            ));
        } else if oracle.mesh_ids.is_empty() && !delivered.reading.mesh_ids.is_empty() {
            faults.push(format!(
                "{label}: the kernel refuses this value ({:?}) but the preview still paints stale meshes {:?}",
                oracle.widget_errors,
                delivered.reading.mesh_ids
            ));
        }
        return faults;
    }
    if oracle.is_empty() {
        faults.push(format!("{label}: the ORACLE itself delivers nothing and names no error — the fixture row is not a geometry-bearing value"));
        return faults;
    }
    if delivered.reading.is_empty() {
        faults.push(format!("{label}: the preview settled EMPTY under phase {} while the value makes {:?}", delivered.phase, oracle.mesh_ids));
        return faults;
    }
    if delivered.reading.mesh_ids != oracle.mesh_ids {
        faults.push(format!("{label}: published {:?}, the value makes {:?}", delivered.reading.mesh_ids, oracle.mesh_ids));
    }
    match (delivered.reading.bounds, oracle.bounds) {
        (Some(delivered_box), Some(oracle_box)) => {
            for axis in 0..3 {
                if (delivered_box.0[axis] - oracle_box.0[axis]).abs() > bounds_tolerance || (delivered_box.1[axis] - oracle_box.1[axis]).abs() > bounds_tolerance {
                    faults.push(format!("{label}: delivered extent {delivered_box:?} is not the value's own {oracle_box:?}"));
                    break;
                }
            }
        }
        (delivered_box, oracle_box) => faults.push(format!("{label}: delivered extent {delivered_box:?} against the value's {oracle_box:?}")),
    }
    if delivered.phase == "invalid" || delivered.phase == "failed" {
        faults.push(format!("{label}: the preview carries the correct geometry but rests in phase {}", delivered.phase));
    }
    faults
}
//#endregion ⚖️Laws
