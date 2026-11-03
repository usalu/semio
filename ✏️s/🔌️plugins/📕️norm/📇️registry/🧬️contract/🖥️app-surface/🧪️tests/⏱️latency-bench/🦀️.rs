//! ⏱️ Latency bench of one norm editor on a scaled document (architecture §5.7 and §10.2): the drag preview from gesture tick to projected scene, the checks re-evaluation after a commit and the incremental live-scene rebuild against the full one. Timings use `Instant` like the framework's own latency gates; the report is a JSON file per artifact, written into the ticket (or `SEMIO_LATENCY_REPORT_DIR`). The thresholds assert in release builds only. See [`norm_latency_bench`].

use crate::direct_manipulation::{is_placement_write, placement_edits, writes_for};
use crate::{commit_writes, gesture_handles, insert_value_at_path, is_view_write, lane_events, live_set_fields, set_value_at_path, Gesture, LiveInvocation, LiveView, NormEdit, NormLiveEditor, NormRetainedEditor, NormTransient};
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_s_plugin_norm_compliance::{CheckReport, NormFamily};
use semio_s_plugin_norm_structural_scene::projection_kit::placements_of;
use semio_s_plugin_norm_structural_scene::{EntityRef, Handle, StructuralScene};
use std::time::Instant;

pub const MIN_ENTITIES: usize = 200;
pub const DRAG_P95_MS: f64 = 12.0;
pub const COMMIT_P95_MS: f64 = 400.0;
pub const COLD_MS: f64 = 1000.0;
const TOUCH_STEPS: usize = 60;

/// 👆️ The frame budget of a touch drag in milliseconds: the strictest of the touch device classes (the tablet's), because a finger moves handles at the display rate.
pub fn touch_drag_budget_ms() -> f64 {
    use crate::device_profiles::device_profile;
    use crate::viewport3d::config::DeviceClass;
    [DeviceClass::Mobile, DeviceClass::Tablet].into_iter().map(|class| f64::from(device_profile(class).frame_budget_ms)).fold(f64::INFINITY, f64::min)
}
const DRAG_TICKS: usize = 120;
const COMMITS: usize = 30;
const REPORT_DIR_ENV: &str = "SEMIO_LATENCY_REPORT_DIR";
const TICKET_REPORT_DIR: &str = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🗑️generated/realtime-qa";

/// 📊️ Order statistics of one series of timings in milliseconds.
#[derive(Clone, Debug, PartialEq)]
pub struct Stats {
    pub samples: usize,
    pub p50: f64,
    pub p95: f64,
    pub max: f64,
}

impl Stats {
    pub fn of(mut ms: Vec<f64>) -> Self {
        ms.sort_by(f64::total_cmp);
        let at = |percent: usize| ms.get((ms.len() * percent / 100).min(ms.len().saturating_sub(1))).copied().unwrap_or(0.0);
        Self { samples: ms.len(), p50: at(50), p95: at(95), max: ms.last().copied().unwrap_or(0.0) }
    }

    fn value(&self) -> DslValue {
        DslValue::object([("samples", DslValue::uint(self.samples as u64)), ("p50Ms", DslValue::float(self.p50)), ("p95Ms", DslValue::float(self.p95)), ("maxMs", DslValue::float(self.max))].map(|(key, value)| (key.to_string(), value)))
    }
}

/// 🧾️ The measured figures of one artifact.
#[derive(Clone, Debug, PartialEq)]
pub struct LatencyReport {
    pub variant: String,
    pub release: bool,
    pub entities: usize,
    pub drag_preview: Stats,
    pub commit_resolve: Stats,
    pub commit_checks: Stats,
    pub touch_drag: Stats,
    pub touch_nudge: Stats,
    pub scene_incremental: Stats,
    pub scene_full: Stats,
    pub scenes_agree: bool,
    pub cold_drag_ms: f64,
    pub cold_checks_ms: f64,
    pub cold_scene_ms: f64,
}

impl LatencyReport {
    fn commit_total_p95(&self) -> f64 {
        self.commit_resolve.p95 + self.commit_checks.p95
    }

    fn cold_total(&self) -> f64 {
        self.cold_checks_ms + self.cold_scene_ms
    }

    /// 🎯️ The failed gates, empty when the document meets §5.7: drag p95 under 12 ms, commit to final checks under 400 ms, incremental scene no slower than the full one.
    pub fn failures(&self) -> Vec<String> {
        let mut failed = Vec::new();
        if !(self.drag_preview.p95 < DRAG_P95_MS) {
            failed.push(format!("drag preview p95 {:.2} ms is not under {DRAG_P95_MS} ms", self.drag_preview.p95));
        }
        if !(self.commit_total_p95() < COMMIT_P95_MS) {
            failed.push(format!("commit to final checks p95 {:.1} ms is not under {COMMIT_P95_MS} ms", self.commit_total_p95()));
        }
        if !(self.touch_drag.p95 < touch_drag_budget_ms()) {
            failed.push(format!("touch drag p95 {:.2} ms (recogniser, preview and scene) is not under the touch frame budget {} ms", self.touch_drag.p95, touch_drag_budget_ms()));
        }
        if !(self.touch_nudge.p95 < COMMIT_P95_MS) {
            failed.push(format!("nudge button to final checks p95 {:.1} ms is not under {COMMIT_P95_MS} ms", self.touch_nudge.p95));
        }
        if !(self.cold_drag_ms < COLD_MS) {
            failed.push(format!("cold first drag tick {:.1} ms is not under {COLD_MS} ms", self.cold_drag_ms));
        }
        if !(self.cold_total() < COLD_MS) {
            failed.push(format!("cold first evaluation (checks plus scene) {:.1} ms is not under {COLD_MS} ms", self.cold_total()));
        }
        if !(self.scene_incremental.p95 <= self.scene_full.p95) {
            failed.push(format!("incremental scene p95 {:.2} ms is slower than the full rebuild {:.2} ms", self.scene_incremental.p95, self.scene_full.p95));
        }
        failed
    }

    fn value(&self, failures: &[String]) -> DslValue {
        let text = |value: &str| DslValue::String(value.to_string());
        let ratio = if self.scene_incremental.p50 > 0.0 { self.scene_full.p50 / self.scene_incremental.p50 } else { 0.0 };
        DslValue::object(
            [
                ("variant", text(&self.variant)),
                ("build", text(if self.release { "release" } else { "debug" })),
                ("entities", DslValue::uint(self.entities as u64)),
                ("dragPreview", self.drag_preview.value()),
                ("dragPreviewTargetP95Ms", DslValue::float(DRAG_P95_MS)),
                ("touchDrag", self.touch_drag.value()),
                ("touchDragTargetP95Ms", DslValue::float(touch_drag_budget_ms())),
                ("touchNudge", self.touch_nudge.value()),
                ("commitResolve", self.commit_resolve.value()),
                ("commitChecks", self.commit_checks.value()),
                ("commitTotalP95Ms", DslValue::float(self.commit_total_p95())),
                ("commitTargetP95Ms", DslValue::float(COMMIT_P95_MS)),
                ("sceneIncremental", self.scene_incremental.value()),
                ("sceneFull", self.scene_full.value()),
                ("sceneFullOverIncrementalP50", DslValue::float(ratio)),
                ("coldDragMs", DslValue::float(self.cold_drag_ms)),
                ("coldChecksMs", DslValue::float(self.cold_checks_ms)),
                ("coldSceneMs", DslValue::float(self.cold_scene_ms)),
                ("coldTargetMs", DslValue::float(COLD_MS)),
                ("scenesAgree", DslValue::Bool(self.scenes_agree)),
                ("asserted", DslValue::Bool(self.release)),
                ("failures", DslValue::Array(failures.iter().map(|failure| text(failure)).collect())),
            ]
            .map(|(key, value)| (key.to_string(), value)),
        )
    }

    /// 📣️ One machine-readable `[latency] <variant>/<scenario>` line per gated figure on stderr, printed always (pass or fail, release or debug); scenarios `cold-drag` and `cold-evaluation` (first tick and first checks plus scene on empty caches, one sample), `drag` (a different value every tick), `commit` (a different real edit every round: p50 the sum of the p50s of its two stages, p95 the sum of the p95s).
    pub fn print(&self) {
        let commit_p50 = self.commit_resolve.p50 + self.commit_checks.p50;
        let (variant, cold_drag, cold_evaluation) = (&self.variant, self.cold_drag_ms, self.cold_total());
        eprintln!("[latency] {variant}/cold-drag p50_ms={cold_drag:.3} p95_ms={cold_drag:.3} budget_ms={COLD_MS:.1}");
        eprintln!("[latency] {variant}/cold-evaluation p50_ms={cold_evaluation:.3} p95_ms={cold_evaluation:.3} budget_ms={COLD_MS:.1}");
        eprintln!("[latency] {}/drag p50_ms={:.3} p95_ms={:.3} budget_ms={DRAG_P95_MS:.1}", self.variant, self.drag_preview.p50, self.drag_preview.p95);
        eprintln!("[latency] {}/commit p50_ms={commit_p50:.3} p95_ms={:.3} budget_ms={COMMIT_P95_MS:.1}", self.variant, self.commit_total_p95());
        eprintln!("[latency] {}/touch-drag p50_ms={:.3} p95_ms={:.3} budget_ms={:.1}", self.variant, self.touch_drag.p50, self.touch_drag.p95, touch_drag_budget_ms());
        eprintln!("[latency] {}/touch-nudge p50_ms={:.3} p95_ms={:.3} budget_ms={COMMIT_P95_MS:.1}", self.variant, self.touch_nudge.p50, self.touch_nudge.p95);
    }

    /// 💾️ Writes `<variant>.json` into the report directory; an unwritable directory is skipped, never fatal.
    pub fn write(&self) {
        let directory = std::env::var_os(REPORT_DIR_ENV).map(std::path::PathBuf::from).or_else(|| std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().find(|root| root.join(".🧬semio").is_dir()).map(|root| root.join(TICKET_REPORT_DIR)));
        let Some(directory) = directory else { return };
        let json = semio_framework_pack_json::to_json_string(&self.value(&self.failures()));
        let _ = std::fs::create_dir_all(&directory).and_then(|()| std::fs::write(directory.join(format!("{}.json", self.variant)), json));
    }
}

fn collections(tree: &DslValue) -> Vec<(String, usize)> {
    let DslValue::Object(fields) = tree else { return Vec::new() };
    fields
        .iter()
        .filter(|(name, _)| name != "placements")
        .filter_map(|(name, value)| match value {
            DslValue::Array(rows) if rows.iter().any(|row| matches!(row.get("id"), Some(DslValue::String(_)))) => Some((name.clone(), rows.len())),
            _ => None,
        })
        .collect()
}

fn replicated(tree: &DslValue, collection: &str, copies: usize) -> DslValue {
    let mut out = tree.clone();
    if let DslValue::Object(fields) = &mut out {
        for (name, value) in fields.iter_mut() {
            let (true, DslValue::Array(rows)) = (name == collection, value) else { continue };
            let originals = rows.clone();
            for copy in 1..=copies {
                rows.extend(originals.iter().map(|row| match row {
                    DslValue::Object(entries) => DslValue::object(entries.iter().map(|(key, value)| match value {
                        DslValue::String(id) if key == "id" => (key.clone(), DslValue::String(format!("{id}~{copy}"))),
                        other => (key.clone(), other.clone()),
                    })),
                    other => other.clone(),
                }));
            }
        }
    }
    out
}

/// 📈️ The default document grown to at least [`MIN_ENTITIES`] scene entities by cloning the rows of every collection that adds entities (ids suffixed `~n`, references untouched); the largest document reached when no collection can.
pub fn scaled<A: NormLiveEditor>(seed: &A::Snapshot) -> A::Snapshot
where
    A::Snapshot: ToValue + FromValue + Clone,
{
    let decode = |tree: &DslValue| <A::Snapshot as FromValue>::from_value(tree.clone()).ok();
    let count = |tree: &DslValue| decode(tree).map_or(0, |doc| A::live_entities(&doc).len());
    let mut tree = seed.to_value();
    let mut entities = count(&tree);
    for (name, rows) in collections(&tree) {
        if entities >= MIN_ENTITIES {
            break;
        }
        let candidate = replicated(&tree, &name, (MIN_ENTITIES * 5 / 4).div_ceil(rows.max(1)));
        let grown = count(&candidate);
        if grown > entities {
            tree = candidate;
            entities = grown;
        }
    }
    decode(&tree).unwrap_or_else(|| seed.clone())
}

fn gesture_json(entity: &EntityRef, handle: &Handle, s: f64, origin: f64, step: &str) -> String {
    format!(r#"{{"entity":{{"kind":"{}","id":"{}"}},"handle":"{}","s":{s},"origin":{origin},{step}}}"#, crate::viewport3d::picking::granularity(entity.kind), entity.id, handle.id)
}

fn probe<A: NormLiveEditor>(doc: &A::Snapshot) -> Option<(EntityRef, Handle, f64, f64)> {
    A::live_entities(doc).into_iter().find_map(|entity| {
        A::live_handles(doc, std::slice::from_ref(&entity)).into_iter().find_map(|handle| {
            let lo = handle.limits.as_ref().map_or(0.0, |limits| limits.clamp(-1.0e9));
            let hi = handle.limits.as_ref().map_or(1.0, |limits| limits.clamp(1.0e9));
            (writes_for(&handle, lo) != writes_for(&handle, hi)).then(|| (entity.clone(), handle, lo, hi))
        })
    })
}

fn applied<A: NormLiveEditor>(doc: &A::Snapshot, handles: &[Handle], gesture: &Gesture) -> Option<A::Snapshot>
where
    A::Snapshot: ToValue + FromValue,
{
    let writes = commit_writes(handles, gesture)?;
    let (placed, plain): (Vec<_>, Vec<_>) = writes.into_iter().filter(|write| !is_view_write(write)).partition(is_placement_write);
    let mut tree = doc.to_value();
    for edit in placement_edits(&placements_of(doc), &crate::standing_of::<A>(doc, handles), &placed).into_iter().chain(plain.into_iter().map(|write| NormEdit::SetField { path: write.path, value: write.value })) {
        match edit {
            NormEdit::SetField { path, value } => set_value_at_path(&mut tree, &path, value).ok()?,
            NormEdit::InsertItem { path, index, value } => insert_value_at_path(&mut tree, &path, index, value).ok()?,
            NormEdit::RemoveItem { .. } | NormEdit::MoveItem { .. } => return None,
        }
    }
    <A::Snapshot as FromValue>::from_value(tree).ok()
}

/// ⏱️ Measures `A` on its scaled `seed` document, which the caller states explicitly (a snapshot with a normative choice has no default). `scene` is the incremental live scene of the editor, `full` the projection from scratch.
pub fn measure<A: NormLiveEditor>(variant: &str, seed: &A::Snapshot, scene: impl Fn(&A::Snapshot, &CheckReport) -> StructuralScene, full: impl Fn(&A::Snapshot, &CheckReport) -> StructuralScene) -> LatencyReport
where
    A::Snapshot: Clone + ToValue + FromValue,
{
    let doc = scaled::<A>(seed);
    let entities = A::live_entities(&doc).len();
    assert!(entities >= MIN_ENTITIES, "{variant}: the scaled document has {entities} entities, the bench needs {MIN_ENTITIES}");
    let (entity, handle, lo, hi) = probe::<A>(&doc).unwrap_or_else(|| panic!("{variant}: the scaled document offers a handle with a free range"));
    let handles = gesture_handles::<A>(&doc, &Gesture::parse(&gesture_json(&entity, &handle, hi, lo, r#""phase":"stream""#)).expect("gesture json"));
    let at = |i: usize| lo + (hi - lo) * (i + 1) as f64 / (DRAG_TICKS + 12) as f64;

    let started = Instant::now();
    let committed = <<A as NormRetainedEditor>::Family as NormFamily>::live_report(&doc);
    let cold_checks_ms = started.elapsed().as_secs_f64() * 1000.0;
    let started = Instant::now();
    std::hint::black_box(scene(&doc, &committed).entities.len());
    let cold_scene_ms = started.elapsed().as_secs_f64() * 1000.0;
    let mut drag = Vec::new();
    let mut cold_drag_ms = 0.0;
    for i in 0..DRAG_TICKS + 10 {
        let stream = gesture_json(&entity, &handle, at(i), lo, r#""phase":"stream""#);
        let started = Instant::now();
        let invocation = LiveInvocation::set_fields("[]", Some(&stream), None).expect("gesture json");
        let transient = lane_events::<A>(&invocation, &doc, &NormTransient::default()).into_iter().fold(NormTransient::default(), NormTransient::apply);
        let shown = LiveView::<<A as NormRetainedEditor>::Family>::with_lanes(doc.clone(), None, &[], &transient);
        let projected = scene(shown.host.document(), shown.host.report());
        let elapsed = started.elapsed().as_secs_f64() * 1000.0;
        std::hint::black_box(projected.entities.len());
        match i {
            0 => cold_drag_ms = elapsed,
            10.. => drag.push(elapsed),
            _ => {}
        }
    }

    let mut touch = semio_framework::interaction::gesture::TouchSession::default();
    touch.down(0.0, 1, 200.0, 200.0);
    let mut touch_drag = Vec::new();
    for i in 0..TOUCH_STEPS + 10 {
        let started = Instant::now();
        let steps = touch.moved(10.0 + i as f64, 1, 200.0 + 3.0 * (i as f64 + 3.0), 200.0);
        let stream = gesture_json(&entity, &handle, at(i), lo, r#""phase":"stream""#);
        let invocation = LiveInvocation::set_fields("[]", Some(&stream), None).expect("gesture json");
        let transient = lane_events::<A>(&invocation, &doc, &NormTransient::default()).into_iter().fold(NormTransient::default(), NormTransient::apply);
        let shown = LiveView::<<A as NormRetainedEditor>::Family>::with_lanes(doc.clone(), None, &[], &transient);
        let projected = scene(shown.host.document(), shown.host.report());
        let elapsed = started.elapsed().as_secs_f64() * 1000.0;
        std::hint::black_box((steps.len(), projected.entities.len()));
        if i >= 10 {
            touch_drag.push(elapsed);
        }
    }
    touch.up(1000.0, 1);
    let mut touch_nudge = Vec::new();
    for i in 0..COMMITS {
        let to = lo + (hi - lo) * (i as f64 + 0.37) / (COMMITS as f64 + 1.0);
        let button = gesture_json(&entity, &handle, to, lo, r#""phase":"commit","ctrl":true"#);
        let gesture = Gesture::parse(&button).expect("gesture json");
        let started = Instant::now();
        let emitted = live_set_fields::<A>(&doc, "[]", Some(&button), None).expect("the nudge button commits");
        std::hint::black_box(emitted.artifact_mutations.len());
        let Some(next) = applied::<A>(&doc, &handles, &gesture) else { continue };
        let report = <<A as NormRetainedEditor>::Family as NormFamily>::live_report(&next);
        std::hint::black_box(scene(&next, &report).entities.len());
        touch_nudge.push(started.elapsed().as_secs_f64() * 1000.0);
    }
    assert!(!touch_nudge.is_empty(), "{variant}: no nudge button commit of the probe handle applied to the scaled document");

    let (mut resolve, mut checks, mut incremental, mut complete) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let mut agree = true;
    for i in 0..COMMITS {
        let commit = gesture_json(&entity, &handle, lo + (hi - lo) * (i as f64 + 0.37) / (COMMITS as f64 + 1.0), lo, r#""phase":"commit""#);
        let gesture = Gesture::parse(&commit).expect("gesture json");
        let started = Instant::now();
        let emitted = live_set_fields::<A>(&doc, "[]", Some(&commit), None).expect("the release commits");
        resolve.push(started.elapsed().as_secs_f64() * 1000.0);
        std::hint::black_box(emitted.artifact_mutations.len());
        let Some(next) = applied::<A>(&doc, &handles, &gesture) else { continue };
        let started = Instant::now();
        let report = <<A as NormRetainedEditor>::Family as NormFamily>::live_report(&next);
        checks.push(started.elapsed().as_secs_f64() * 1000.0);
        let started = Instant::now();
        let warm = scene(&next, &report);
        incremental.push(started.elapsed().as_secs_f64() * 1000.0);
        let started = Instant::now();
        let cold = full(&next, &report);
        complete.push(started.elapsed().as_secs_f64() * 1000.0);
        agree &= warm.entities.len() == cold.entities.len();
    }
    assert!(!checks.is_empty(), "{variant}: no commit of the probe handle applied to the scaled document");
    LatencyReport {
        variant: variant.to_string(),
        release: !cfg!(debug_assertions),
        entities,
        drag_preview: Stats::of(drag),
        commit_resolve: Stats::of(resolve),
        commit_checks: Stats::of(checks),
        touch_drag: Stats::of(touch_drag),
        touch_nudge: Stats::of(touch_nudge),
        scene_incremental: Stats::of(incremental),
        scene_full: Stats::of(complete),
        scenes_agree: agree,
        cold_drag_ms,
        cold_checks_ms,
        cold_scene_ms,
    }
}

/// 🌱 The seed of an artifact whose snapshot carries no normative choice and therefore has a default.
pub fn default_seed<A: NormLiveEditor>() -> A::Snapshot
where
    A::Snapshot: Default,
{
    A::Snapshot::default()
}

/// 🚦️ Measures, writes the JSON report and asserts the gates in release builds.
pub fn gate<A: NormLiveEditor>(variant: &str, seed: &A::Snapshot, scene: impl Fn(&A::Snapshot, &CheckReport) -> StructuralScene, full: impl Fn(&A::Snapshot, &CheckReport) -> StructuralScene)
where
    A::Snapshot: Clone + ToValue + FromValue,
{
    let report = measure::<A>(variant, seed, scene, full);
    report.print();
    report.write();
    let failures = report.failures();
    assert!(!report.release || failures.is_empty(), "{variant}: {failures:?}");
}

/// 🧪️ Declares the `latency_bench` test module of an editor: `scene` is `fn(&Snapshot, &CheckReport) -> StructuralScene` (the incremental live scene), `full` the same from scratch. Run with the `:test` filter `latency_bench` on a release build; the report lands in the ticket's `🗑️generated/realtime-qa`.
#[macro_export]
macro_rules! norm_latency_bench {
    ($app:ty, variant = $variant:expr, seed = $seed:expr, scene = $scene:expr, full = $full:expr) => {
        #[cfg(test)]
        mod latency_bench {
            use super::*;

            #[test]
            fn the_realtime_gates_hold_on_a_scaled_document_and_the_report_is_written() {
                $crate::latency_bench::gate::<$app>($variant, &$seed, $scene, $full);
            }
        }
    };
    ($app:ty, variant = $variant:expr, scene = $scene:expr, full = $full:expr) => {
        $crate::norm_latency_bench!($app, variant = $variant, seed = $crate::latency_bench::default_seed::<$app>(), scene = $scene, full = $full);
    };
}
