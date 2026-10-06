"""🧱️ S5-PUZZLE wave B3 (design §22.32 a, F16, F21, F24) — puzzle 2d guest, all or nothing.

`python3 <this file> --check | --apply | --restore`

- §22.32 (a): the non-drag board tools are ONE `statechart!` machine (`board_tool`) driven on the window's gesture slot,
  one release = ONE `ToolTransaction` of `<appId>#<kind>`; the select tool is a `GestureChart` on the same slot — the
  private `Puzzle2dSelectTool` wrapper, its persisted window-transient state, the preview fold, the utility-switch
  retire and the whole `host_event` arm (`TimeTravelFrozen` included) are deleted.
- F21: a committed selection leaf states its inputs at the `precision` its input schema declares.
- F16: the `vortex` domain declares `HierarchyProvider::Topology` and the app supplies it.
- F24: law — a flush of transient rows alone upserts no history row.

Every anchor must resolve exactly as stated on the live tree before any file is written. `--apply` keeps the pre-wave and
post-wave text of every file under `🗑️generated/s5-puzzle/b3/` so `--restore` can refuse a file a peer changed since.
"""

import hashlib
import json
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[6]
STAGED = HERE / "🧪️s5-puzzle-b3"
KEPT = HERE / "🗑️generated" / "s5-puzzle" / "b3"
PUZZLE = "✏️s/🔌️plugins/🧩️puzzle"
P2 = f"{PUZZLE}/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any"
ED = f"{P2}/✏️editor"
E = f"{ED}/🦀️.rs"
SELECT = f"{ED}/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs"
BOARD = f"{ED}/🎮️commands/🎲️apply-board-events/🦀️.rs"
MUTATIONS = f"{P2}/🧬️schema/🧬️mutations/🦀️.rs"
RETAINED = f"{PUZZLE}/🎮️commands/🧵️retained/🦀️.rs"
WINDOW = f"{ED}/🪟️window/🦀️.rs"
WINDOW_JSON = f"{ED}/🪟️window/🧬️schema/🔣️.json"
WINDOW_TS = f"{ED}/🪟️window/🧬️schema/🟦️.ts"
CONFIG = f"{ED}/🎚️config/🦀️.rs"
GATE = f"{PUZZLE}/📦️packages/🟦️typescript/📜️script.ts"
SELECT_LAWS = f"{ED}/🧪️tests/🧪️select-tool/🦀️.rs"
TRANSACTION_LAWS = f"{ED}/🧪️tests/🧪️select-tool-transactions/🦀️.rs"
NEW = {
    f"{ED}/🧫️fixtures/🧫️board-tools/🔣️.json": "corpus.json",
    f"{ED}/🧬️schema/🔣️board-tools/🔣️.json": "corpus-schema.json",
    f"{ED}/🧪️tests/🧪️board-tools/🐍️.py": "oracle.py",
    f"{ED}/🧪️tests/🧪️board-tools/🦀️.rs": "board-tools-law.rs",
}
DETACHED = "&semio_framework_plugin::app::GestureSlot::detached()"


class Tree:
    def __init__(self):
        self.before = {}
        self.after = {}

    def text(self, path):
        if path not in self.after:
            self.before[path] = self.after[path] = (ROOT / path).read_text(encoding="utf-8")
        return self.after[path]

    def sub(self, path, old, new, count=1):
        text = self.text(path)
        found = text.count(old)
        if found != count:
            sys.exit(f"{path}: anchor resolves {found} times, expected {count}:\n{old[:160]}")
        self.after[path] = text.replace(old, new)

    def line(self, lines, needle, start=0):
        hits = [index for index in range(start, len(lines)) if needle in lines[index]]
        if not hits or (start == 0 and len(hits) != 1):
            return None, len(hits)
        return hits[0], len(hits)

    def cut(self, path, first, last, replacement, keep_last=False):
        lines = self.text(path).split("\n")
        begin, found = self.line(lines, first)
        if begin is None:
            sys.exit(f"{path}: cut start resolves {found} times, expected 1:\n{first}")
        end, found = self.line(lines, last, begin + 1)
        if end is None:
            sys.exit(f"{path}: cut end not found after its start:\n{last}")
        tail = lines[end:] if keep_last else lines[end + 1 :]
        self.after[path] = "\n".join(lines[:begin] + ([replacement.removesuffix("\n")] if replacement else []) + tail)

    def tail(self, path, first, replacement):
        lines = self.text(path).split("\n")
        begin, found = self.line(lines, first)
        if begin is None:
            sys.exit(f"{path}: tail start resolves {found} times, expected 1:\n{first}")
        self.after[path] = "\n".join(lines[:begin]) + "\n" + replacement.rstrip("\n") + "\n"


BOARD_IMPORTS_OLD = """use crate::editor::puzzle2d::modes::edit::windows::{detail, overview, selection};
use crate::editor::puzzle2d::panels::{artifact, inspection};
use crate::editor::puzzle2d::{apply_brush_place_payload, delete_selection_from_host_snapshot, puzzle2d_push_target_region, puzzle2d_relocate_target_region, puzzle2d_selection_write, Puzzle2dActionCtx, Puzzle2dScene, Puzzle2dSelectionRecord};
use semio_framework::kernel::UiDirtyScope;
use serde_json::{json, Value};
"""
BOARD_IMPORTS_NEW = """#![allow(unexpected_cfgs)]

use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select::PUZZLE2D_EDITOR_APP_ID;
use crate::editor::puzzle2d::modes::edit::windows::{detail, overview, selection};
use crate::editor::puzzle2d::panels::{artifact, inspection};
use crate::editor::puzzle2d::{apply_brush_place_payload, delete_selection_from_host_snapshot, puzzle2d_push_target_region, puzzle2d_relocate_target_region, puzzle2d_selection_write, Puzzle2dActionCtx, Puzzle2dScene, Puzzle2dSelectionRecord};
use crate::standards::v1::subsets::any::schema::mutations::{puzzle2d_document_delta_operations, Puzzle2dMutation};
use machine::Command;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_tool_machine::{GestureChart, GesturePhase, ToolYield};
use serde_json::{json, Value};
use std::sync::{Arc, OnceLock};
"""
BOARD_TOOL = r'''//#region 🛠️BoardTool
/// 🧰️ The board gestures whose press and stream stay engine-local — the board paints its own rubber band, ghost or wire
/// and reports only the release, as rows of ONE kind: an area painted (`regionCreate`, already grid-snapped by the
/// engine), a region grip dragged (`regionResize`, an absolute pose a locked region refuses), a brush stamped
/// (`brushPlace`), a wire dropped on a handle (`edgeCreate`), a wire cut (`edgeDelete`) and a delete of whatever the id
/// names (`nodeDelete`: a node, a handle, an edge or a target region). Each is a one-step tool: its release yields the
/// document leaves inside ONE `ToolTransaction` of `<appId>#<kind>`; a gesture the board cancels sends no row, and a
/// release that changes nothing leaves zero trace.
pub const PUZZLE2D_BOARD_TOOLS: [&str; 6] = ["regionCreate", "regionResize", "brushPlace", "edgeCreate", "edgeDelete", "nodeDelete"];

/// 🧱️ Folds ONE board-tool row into `fixture` — the one reducer the board tool's release and a guest-side host's
/// replayed rows ([`apply_board_events_from_json`]) share. Answers whether `name` is a board-tool kind.
pub fn puzzle2d_fold_board_row(fixture: &mut Value, name: &str, payload: &Value) -> bool {
    let read = |key: &str| payload.get(key).and_then(Value::as_f64).filter(|value| value.is_finite());
    let id = payload.get("id").and_then(Value::as_str).filter(|id| !id.is_empty());
    match name {
        "regionCreate" => {
            if let (Some(x), Some(y), Some(width), Some(height)) = (read("x"), read("y"), read("width"), read("height")) {
                if width > 0.0 && height > 0.0 {
                    puzzle2d_push_target_region(fixture, x, y, width, height);
                }
            }
        }
        "regionResize" => {
            if let (Some(id), Some(x), Some(y), Some(width), Some(height)) = (id, read("x"), read("y"), read("width"), read("height")) {
                puzzle2d_relocate_target_region(fixture, id, &json!({ "position": [x, y], "size": [width, height] }));
            }
        }
        "brushPlace" => apply_brush_place_payload(fixture, payload),
        "edgeCreate" => {
            if let Some(edges) = fixture.get_mut("edges").and_then(Value::as_array_mut) {
                edges.push(payload.clone());
            }
        }
        "nodeDelete" => {
            if let Some(id) = id {
                delete_selection_from_host_snapshot(fixture, &[id.to_string()]);
                crate::editor::puzzle2d::delete_target_regions_from_fixture(fixture, &[id.to_string()]);
            }
        }
        "edgeDelete" => {
            if let (Some(id), Some(edges)) = (id, fixture.get_mut("edges").and_then(Value::as_array_mut)) {
                edges.retain(|edge| edge.get("id").and_then(Value::as_str) != Some(id));
            }
        }
        _ => return false,
    }
    true
}

/// 📦️ What a release does: the document after it and the leaves that take its base there.
#[derive(Debug)]
pub struct BoardToolOutcome {
    pub after: Arc<Value>,
    pub yields: Vec<(String, Puzzle2dMutation)>,
}

/// 🧾️ One release as a tool event: the document it folds on and the row payloads of its ONE kind, in board order. What
/// it yields is derived once, however often the chart asks.
#[derive(Clone, Debug)]
pub struct BoardToolRelease {
    base: Arc<Value>,
    kind: &'static str,
    rows: Arc<[Value]>,
    outcome: Arc<OnceLock<BoardToolOutcome>>,
}

impl BoardToolRelease {
    /// 🎬️ The release of `kind` whose row payloads are `rows`, on `base`.
    pub fn new(base: Arc<Value>, kind: &'static str, rows: Vec<Value>) -> Self {
        Self { base, kind, rows: rows.into(), outcome: Arc::new(OnceLock::new()) }
    }

    /// 🧮️ The release folded on its base: every row through [`puzzle2d_fold_board_row`], then the granular leaves
    /// between the base and the folded document, keyed `<kind>:<index>`. A fold the document does not decode after is
    /// refused whole: it yields nothing and says so on the trace channel.
    pub fn outcome(&self) -> &BoardToolOutcome {
        self.outcome.get_or_init(|| {
            let mut after = self.base.as_ref().clone();
            for payload in self.rows.iter() {
                puzzle2d_fold_board_row(&mut after, self.kind, payload);
            }
            match puzzle2d_document_delta_operations(&self.base, &after) {
                Ok(leaves) if !leaves.is_empty() => BoardToolOutcome { after: Arc::new(after), yields: leaves.into_iter().enumerate().map(|(index, leaf)| (format!("{}:{index}", self.kind), leaf)).collect() },
                Ok(_) => BoardToolOutcome { after: Arc::clone(&self.base), yields: Vec::new() },
                Err(refusal) => {
                    eprintln!("[TRACE] puzzle2d board tool {} refused its release: {refusal}", self.kind);
                    BoardToolOutcome { after: Arc::clone(&self.base), yields: Vec::new() }
                }
            }
        })
    }
}

/// 🪶️ The board tool keeps nothing between dispatches: its gesture lives in the board engine until the release.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BoardToolContext;

fn board_tool_context(input: BoardToolContext) -> BoardToolContext {
    input
}

fn release_applies(_context: &BoardToolContext, event: Option<&board_tool::Event>) -> bool {
    matches!(event, Some(board_tool::Event::Release(release)) if !release.outcome().yields.is_empty())
}

fn yield_release(_context: &mut BoardToolContext, event: Option<&board_tool::Event>, sink: &mut Vec<Command<board_tool::BoardTool>>) {
    let Some(board_tool::Event::Release(release)) = event else { return };
    sink.extend(release.outcome().yields.iter().cloned().map(|(key, mutation)| Command::Effect(ToolYield::upsert(key, mutation))));
    sink.push(Command::Effect(ToolYield::Commit));
}

machine::statechart! {
    machine board_tool {
        context: BoardToolContext;
        event Event { Release(BoardToolRelease) }
        input: BoardToolContext;
        output: ();
        effect: ToolYield<Puzzle2dMutation>;
        context_from_input: board_tool_context;
        initial: idle;
        state idle {
            on Release if release_applies => idle do yield_release;
        }
    }
}

/// 🧷️ The board tool's host: its chart declares no timer, no invoke and no foreign effect, so every duty is empty.
pub struct BoardToolHost;

impl machine::Host<board_tool::BoardTool> for BoardToolHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<Puzzle2dMutation>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: machine::TimerId, _delay_ms: u64) {}
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: machine::TimerId) {}
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        semio_framework_job::default_now_ms().unwrap_or(0)
    }
}

/// 🧭️ The board tool on the framework gesture runner: a release is a one-shot of the window's gesture slot — it ends
/// whatever gesture the window held and never holds one itself, so there is nothing to restore, and it folds on any
/// base.
impl GestureChart for board_tool::BoardTool {
    type Tick = BoardToolRelease;
    type Host = BoardToolHost;

    const BASE_BOUND: bool = false;

    fn tool(verb: &str) -> String {
        format!("{PUZZLE2D_EDITOR_APP_ID}#{verb}")
    }

    fn host() -> BoardToolHost {
        BoardToolHost
    }

    fn input() -> BoardToolContext {
        BoardToolContext
    }

    fn restore(_entries: &[(String, Puzzle2dMutation)], _context: &semio_framework_value::DslValue) -> Option<BoardToolContext> {
        None
    }

    fn event(phase: GesturePhase, _at_rest: bool, tick: Option<BoardToolRelease>) -> Option<board_tool::Event> {
        tick.filter(|_| phase == GesturePhase::Once).map(board_tool::Event::Release)
    }
}
//#endregion 🛠️BoardTool

/// 🎲️ Folds one engine event batch into the scene and answers its gesture records — what a guest-side host's own rows
/// replay through (`apply_host_events`). A `gesture` row never touches the scene: it is a parametric input the select
/// tool yields as ONE transaction; a board-tool row folds through [`puzzle2d_fold_board_row`]; selection is
/// framework-owned (see [`board_selection_write`]).
pub fn apply_board_events_from_json(events_json: &str, envelope: &mut Puzzle2dScene) -> Vec<Puzzle2dSelectionRecord> {
    let Ok(events) = serde_json::from_str::<Vec<Value>>(events_json) else {
        return Vec::new();
    };
    let mut records = Vec::new();
    for event in events {
        let Some(name) = event.get("name").and_then(Value::as_str) else {
            continue;
        };
        let payload = event.get("payload").cloned().unwrap_or(Value::Null);
        match name {
            "gesture" => records.extend(Puzzle2dSelectionRecord::from_gesture(&payload)),
            "brushCandidates" => fold_brush_candidates(envelope, &payload),
            name => {
                puzzle2d_fold_board_row(&mut envelope.fixture, name, &payload);
            }
        }
    }
    records
}

/// 🖌️ The brush's candidate page a `brushCandidates` row carries — window-transient runtime state, never the document.
fn fold_brush_candidates(envelope: &mut Puzzle2dScene, payload: &Value) {
    if let Some(candidates) = payload.get("candidates").and_then(Value::as_array) {
        envelope.runtime.brush_candidates = candidates.iter().map(semio_framework_value::DslValue::from).collect();
    }
    if let Some(source) = payload.get("sourceHandleId").and_then(Value::as_str) {
        envelope.runtime.brush_candidate_source_handle_id = source.to_string();
    }
    if let Some(index) = payload.get("index").and_then(Value::as_u64) {
        envelope.runtime.brush_candidate_index = index as usize;
    }
}
'''
BOARD_VERB = r'''/// 🎰️ One board flush. Its last `select` row is the framework selection write; its `gesture` records commit through
/// the select tool; its board-tool rows commit through the board tool, one release per run of one kind — every document
/// change of a flush is a tool transaction, never a scene delta. A record whose every target is locked is refused at
/// the tool with one sentence; the selection write of the same batch still lands. A flush of transient rows alone (a
/// hover, a selection, a candidate page) yields no operation at all.
pub fn apply_board_events(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let Some(events_json) = args.and_then(|value| value.get("eventsJson")).and_then(|value| value.as_str()) else {
        return;
    };
    let events = serde_json::from_str::<Vec<Value>>(events_json).ok();
    let scope = events.as_deref().map_or(UiDirtyScope::Full, puzzle2d_board_events_scope);
    *ctx.ui_scope = scope.clone();
    let absent = Value::Null;
    let mut records = Vec::new();
    let mut rows = Vec::new();
    for event in events.as_deref().unwrap_or_default() {
        let payload = event.get("payload").unwrap_or(&absent);
        match event.get("name").and_then(Value::as_str) {
            Some("gesture") => records.extend(Puzzle2dSelectionRecord::from_gesture(payload)),
            Some("brushCandidates") => fold_brush_candidates(ctx.scene, payload),
            Some(name) => rows.extend(PUZZLE2D_BOARD_TOOLS.iter().find(|kind| **kind == name).map(|kind| (*kind, payload.clone()))),
            None => {}
        }
    }
    let released = ctx.release_board(&rows);
    if let Some(write) = events.as_deref().and_then(|events| board_selection_write(events, released.as_deref().unwrap_or(&ctx.scene.fixture))) {
        ctx.interaction_writes.push(write);
    }
    if !records.is_empty() || ctx.gesture.open().is_some() {
        ctx.commit_selection(overview::utilities::select::UTILITY_ID, records);
    }
    if !ctx.interaction_writes.is_empty() && matches!(*ctx.ui_scope, UiDirtyScope::None) {
        *ctx.ui_scope = scope;
    }
}
'''
CTX_TOOLS = r'''    /// 🛠️ Commits `records` through the select tool machine as ONE tool transaction of this action — the
    /// parametric selection leaves plus the connections their drops land, yielded as `verb`. A request whose
    /// every target is locked is refused at the tool with the one lock sentence and leaves zero trace; a request
    /// with a movable target is yielded whole, and its leaf reports the locked rest as `mutation.partial`.
    pub fn commit_selection(&mut self, verb: &str, records: Vec<Puzzle2dSelectionRecord>) {
        self.transform_selection(verb, GesturePhase::Once, records);
    }

    /// 🌊️ Drives this window's select tool through ONE dispatch of a selection transform on the window's gesture slot.
    /// `Once` commits the records as one transaction; `Stream` upserts them into the window's open transaction —
    /// opening it on the first tick — which the runtime keeps and overlays on the document every render reads; `Commit`
    /// folds the final records in and commits the whole gesture as ONE edit; `Abort` drops the open gesture with zero
    /// trace. Another verb, a one-shot, a moved base and every host fact (a blur, a lost capture, a utility switch, a
    /// frozen document) end an open gesture in the slot — this action maps none of them.
    pub fn transform_selection(&mut self, verb: &str, phase: GesturePhase, records: Vec<Puzzle2dSelectionRecord>) {
        if matches!(phase, GesturePhase::Abort(_)) && self.gesture.open().is_none() {
            *self.ui_scope = UiDirtyScope::None;
            return;
        }
        let base = self.base.typed();
        let refused = records.iter().any(|record| record.refused_as_locked(base));
        let request = (!matches!(phase, GesturePhase::Abort(_))).then(|| SelectToolRequest { base: self.base.shared_typed(), proximity_radius: self.scene.runtime.proximity_radius, records });
        let committed = self.gesture.drive::<ChartGesture<select_utility::select_tool::SelectTool>>(None, verb, phase, request, self.authoring_seed).ok().flatten();
        match committed {
            Some((transaction, mutations)) => self.publish_transaction(transaction, mutations),
            None if refused && self.gesture.open().is_none() => {
                self.notice(|labels| labels.selection_locked.as_str());
                *self.ui_scope = UiDirtyScope::None;
            }
            None => {}
        }
    }

    /// 🧾️ Publishes one committed tool transaction of this action: its leaves join the action's ONE edit, which the
    /// first transaction the action commits stamps.
    fn publish_transaction(&mut self, transaction: protocol::TransactionRef, mutations: Vec<Puzzle2dMutation>) {
        if self.transaction.is_none() && !self.authoring_seed.is_empty() {
            *self.transaction = Some(transaction);
        }
        self.artifact_mutations.extend(mutations);
    }

    /// 🧱️ Commits the board-tool rows of ONE flush (`rows`: kind and payload, in board order): every run of one kind is
    /// one release of that tool — ONE `ToolTransaction` of `<appId>#<kind>` on the window's gesture slot, folded on the
    /// document the runs before it left. Answers the document after every release, `None` when no row changed it.
    pub fn release_board(&mut self, rows: &[(&'static str, Value)]) -> Option<std::sync::Arc<Value>> {
        let mut after: Option<std::sync::Arc<Value>> = None;
        for run in rows.chunk_by(|left, right| left.0 == right.0) {
            let base = after.clone().unwrap_or_else(|| self.base.shared_value());
            let release = apply_board_events::BoardToolRelease::new(base, run[0].0, run.iter().map(|row| row.1.clone()).collect());
            let committed = self.gesture.drive::<ChartGesture<apply_board_events::board_tool::BoardTool>>(None, run[0].0, GesturePhase::Once, Some(release.clone()), self.authoring_seed).ok().flatten();
            if let Some((transaction, mutations)) = committed {
                self.publish_transaction(transaction, mutations);
                after = Some(std::sync::Arc::clone(&release.outcome().after));
            }
        }
        after
    }
'''
TOPOLOGY = r'''    /// 🗺️ The `vortex` domain's whole pickable universe, in board order: every node with its handles under it, every
    /// edge and every target region (a region answers to the node granularity, as `puzzle2d_selection_targets`
    /// classifies it) — what `selectAll` enumerates and what keeps a selection inside the document. A host fact never
    /// reaches this editor: the window's gesture slot ends an open gesture on a blur, a lost capture, a utility switch,
    /// a closing window, a frozen document and a moved base.
    fn interaction_topology(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle2dConfig>) -> Result<semio_framework_plugin::InteractionTopology, semio_framework_value::ValueError> {
        let document = doc.snapshot.typed();
        let mut ordered = Vec::new();
        for node in &document.nodes {
            ordered.push(semio_framework_plugin::TopologyNode { id: node.id.clone(), granularity: PUZZLE2D_GRANULARITY_NODE.into(), parent: None });
            ordered.extend(node.handles.iter().map(|handle| semio_framework_plugin::TopologyNode { id: handle.id.clone(), granularity: PUZZLE2D_GRANULARITY_HANDLE.into(), parent: Some(node.id.clone()) }));
        }
        ordered.extend(document.edges.iter().map(|edge| semio_framework_plugin::TopologyNode { id: edge.id.clone(), granularity: PUZZLE2D_GRANULARITY_EDGE.into(), parent: None }));
        ordered.extend(document.target_regions.iter().map(|region| semio_framework_plugin::TopologyNode { id: region.id.clone(), granularity: PUZZLE2D_GRANULARITY_NODE.into(), parent: None }));
        Ok(semio_framework_plugin::InteractionTopology { domains: std::collections::BTreeMap::from([(PUZZLE2D_INTERACTION_DOMAIN.to_string(), semio_framework_plugin::DomainTopology { ordered })]) })
    }

'''
PRECISION = r'''
//#region 🎚️DeclaredPrecision
/// 🪚️ `mutation` with every numeric input rounded to the `precision` its input schema declares for it
/// (`x-semio-ui.precision`, decimal places) — what a gesture commits, so the row label, the history editor's value and
/// its stepper state one number on every host (a host that measures a drag in `f32` records 59.99996 for 60). An input
/// that declares no precision keeps its value.
pub fn puzzle2d_declared_precision(mutation: Puzzle2dMutation) -> Puzzle2dMutation {
    let Some(schema) = Mutation::<Puzzle2dSnapshot>::input_schema(&mutation).and_then(|schema| serde_json::from_str::<Value>(schema).ok()) else {
        return mutation;
    };
    let mut payload = Value::from(Mutation::<Puzzle2dSnapshot>::payload_value(&mutation));
    let mut rounded = false;
    for (name, declared) in schema.get("properties").and_then(Value::as_object).into_iter().flatten() {
        let Some(precision) = declared.get("x-semio-ui").and_then(|ui| ui.get("precision")).and_then(Value::as_u64) else { continue };
        let Some(value) = payload.get(name.as_str()).and_then(Value::as_f64) else { continue };
        let scale = 10f64.powi(precision.min(15) as i32);
        let quantized = (value * scale).round() / scale;
        if quantized != value || (quantized == 0.0 && value.is_sign_negative()) {
            payload[name.as_str()] = Value::from(if quantized == 0.0 { 0.0 } else { quantized });
            rounded = true;
        }
    }
    if !rounded {
        return mutation;
    }
    Mutation::<Puzzle2dSnapshot>::with_payload_value(&mutation, semio_framework_value::DslValue::from(&payload)).unwrap_or(mutation)
}
//#endregion 🎚️DeclaredPrecision
'''
SHARED = r'''
    /// 🤝️ The legacy play projection as a shared root — what an owned tool event carries without copying the document.
    pub fn shared_value(&self) -> std::sync::Arc<Value> {
        std::sync::Arc::clone(self.value.get_or_init(|| std::sync::Arc::new(Value::from(semio_framework_value::ToValue::to_value(self.typed.as_ref())))))
    }

    /// 🫱️ The typed authority as a shared root, for the same reason.
    pub fn shared_typed(&self) -> std::sync::Arc<Puzzle2dSnapshot> {
        std::sync::Arc::clone(&self.typed)
    }
'''
SELECT_PHASE = r'''/// 🎚️ Reads a transform verb's `phase` (`stream` | `commit` | `abort`, absent = one-shot) and an abort's `reason`
/// (`blur`, `captureLost`, `baseMoved`, `frozen`, `retired`; absent = `tool`); `None` for an unknown one.
pub fn puzzle2d_gesture_phase(args: Option<&Value>) -> Option<GesturePhase> {
    let text = |key: &str| args.and_then(|args| args.get(key)).and_then(Value::as_str);
    GesturePhase::parse(text("phase"), text("reason"))
}
'''
SELECT_CHART = r'''/// 🧭️ The select tool on the framework gesture runner: a streamed transform is the window's ONE gesture in its
/// gesture slot — started, resumed, persisted, previewed and ended there, never by this editor. A resumed stream
/// recovers its net transform from the open transaction's one leaf and whether its drop connects from the context it
/// persisted.
impl GestureChart for select_tool::SelectTool {
    type Tick = SelectToolRequest;
    type Host = SelectToolHost;

    fn tool(verb: &str) -> String {
        format!("{PUZZLE2D_EDITOR_APP_ID}#{verb}")
    }

    fn host() -> SelectToolHost {
        SelectToolHost
    }

    fn input() -> SelectToolContext {
        SelectToolContext::default()
    }

    fn restore(entries: &[(String, Puzzle2dMutation)], context: &semio_framework_value::DslValue) -> Option<SelectToolContext> {
        let connect = matches!(context, semio_framework_value::DslValue::Bool(true));
        Some(SelectToolContext { stream: entries.iter().find(|(key, _)| key == PUZZLE2D_SELECT_TOOL_LEAF_KEY).and_then(|(_, leaf)| Puzzle2dSelectionRecord::from_leaf(leaf, connect)) })
    }

    fn context(context: &SelectToolContext) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::Bool(context.stream.as_ref().is_some_and(|stream| stream.connect))
    }

    fn event(phase: GesturePhase, at_rest: bool, tick: Option<SelectToolRequest>) -> Option<select_tool::Event> {
        let request = tick?;
        match phase {
            GesturePhase::Stream => Some(select_tool::Event::Stream(request)),
            GesturePhase::Commit if !at_rest => Some(select_tool::Event::Finish(request)),
            GesturePhase::Once | GesturePhase::Commit => Some(select_tool::Event::Records(request)),
            GesturePhase::Abort(_) => None,
        }
    }
}
'''


def edit(tree):
    # mutations: declared precision + shared roots
    tree.sub(MUTATIONS, "//#endregion 🔖️HandleGeometry\n", "//#endregion 🔖️HandleGeometry\n" + PRECISION)
    tree.sub(MUTATIONS, "    /// 🧬️ The typed authority, without materializing the legacy projection.\n", SHARED.lstrip("\n") + "\n    /// 🧬️ The typed authority, without materializing the legacy projection.\n")

    # retained layer: the job context reaches a work object
    tree.sub(RETAINED, "    fn bind_view_state(&mut self, _view_state: Option<ViewModel>) {}\n", "    fn bind_view_state(&mut self, _view_state: Option<ViewModel>) {}\n    /// 🖐️ The admission's job context, bound once per construction — a work object that drives its window's gesture\n    /// reads the gesture slot from it. Default no-op.\n    fn bind_job_context(&mut self, _context: Arc<semio_framework_plugin::app::ArtifactOwnedToolJobContext<A>>) {}\n")

    # board tool
    tree.sub(BOARD, BOARD_IMPORTS_OLD, BOARD_IMPORTS_NEW)
    tree.cut(BOARD, "/// 🎲️ Folds one engine event batch into the scene and answers its gesture records.", "/// 🐢️ `UiDirtyScope.windowBodies`/`.panelBodies` are matched against", "\n" + BOARD_TOOL + "\n", keep_last=True)
    tree.tail(BOARD, "/// 🎲️ One board flush: folds its rows into the scene", BOARD_VERB)

    # select tool: chart on the gesture runner, the private wrapper and its persisted state gone
    tree.sub(SELECT, "use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle2d_mutation, connect_handles_in_proximity, drag_selection, puzzle2d_handle_distance, rotate_selection, scale_selection, Puzzle2dMutation};", "use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle2d_mutation, connect_handles_in_proximity, drag_selection, puzzle2d_declared_precision, puzzle2d_handle_distance, rotate_selection, scale_selection, Puzzle2dMutation};")
    tree.sub(SELECT, "use semio_framework_tool_machine::{ToolAbortReason, ToolMachineRunner, ToolRefusal, ToolStep, ToolTransaction, ToolTransactionState, ToolYield};", "use semio_framework_tool_machine::{GestureChart, GesturePhase, ToolMachineRunner, ToolStep, ToolYield};")
    tree.cut(SELECT, "/// 🎚️ Where one dispatch of a selection transform sits in a select-tool gesture", "/// 🔑️ The transaction key of a gesture's parametric leaf", SELECT_PHASE + "\n", keep_last=True)
    tree.cut(SELECT, "/// 🧷️ One provisional entry of a persisted select-tool transaction", "/// 🛠️ Runs `records` through a select tool at rest as ONE one-shot transaction on `clock`", SELECT_CHART + "\n", keep_last=True)
    tree.cut(SELECT, "/// 👁️ The document a window paints while its select tool holds an open transaction", "/// 🧮️ What the select tool yields for `records` on `base`, keyed", "", keep_last=True)
    tree.sub(SELECT, "        let leaf = record.mutation();\n        if !record.applies_to(&state) || apply_puzzle2d_mutation(&mut state, &leaf).is_err() {", "        let leaf = puzzle2d_declared_precision(record.mutation());\n        if !record.applies_to(&state) || !Puzzle2dSelectionRecord::from_leaf(&leaf, false).is_some_and(|committed| committed.moves()) || apply_puzzle2d_mutation(&mut state, &leaf).is_err() {")
    tree.sub(SELECT, "/// again.\npub fn puzzle2d_selection_yields(", "/// again. Every leaf is committed at the precision its inputs declare ([`puzzle2d_declared_precision`]); a motion that\n/// rounds to the identity yields nothing.\npub fn puzzle2d_selection_yields(")

    # editor: slot threading, tools on the slot, topology, no host_event, no preview fold, no retire
    tree.sub(E, "select::{puzzle2d_selection_pivot, Puzzle2dSelectPhase, Puzzle2dSelectTool, Puzzle2dSelectionMotion", "select::{puzzle2d_gesture_phase, puzzle2d_selection_pivot, Puzzle2dSelectionMotion")
    tree.sub(E, "use semio_framework_tool_machine::{ToolAbortReason, ToolStep};\n", "use semio_framework_tool_machine::{ChartGesture, GesturePhase};\n")
    tree.sub(E, "/// `Flat` hierarchy (no parent/child structure was ever modeled for it).\n", "/// whose topology the app supplies (`interaction_topology`): nodes, their handles, edges and target regions.\n")
    tree.sub(E, "hierarchy: HierarchyProvider::Flat,", "hierarchy: HierarchyProvider::Topology,")
    tree.sub(E, "    pub transaction: &'a mut Option<protocol::TransactionRef>,\n}", "    pub transaction: &'a mut Option<protocol::TransactionRef>,\n    /// 🖐️ The dispatching window's gesture slot: every tool of this action drives the window's ONE gesture through it.\n    pub gesture: &'a semio_framework_plugin::app::GestureSlot<Puzzle2dMutation>,\n}")
    tree.cut(E, "    /// 🛠️ Commits `records` through the select tool machine as ONE tool transaction of this action", "        self.scene.runtime.select_tool = tool.persist();", CTX_TOOLS.rstrip("\n").rsplit("\n", 1)[0], keep_last=False)
    tree.cut(E, "        let fixture = match runtime.select_tool.as_ref().filter(|_| active_utility == select_utility::UTILITY_ID) {", "        };", "        let fixture = doc.snapshot.value().clone();")
    tree.sub(E, "    ephemeral: Option<EphemeralEmit<EditorApp<Puzzle2dPlayApp>>>,\n}\n\nimpl Puzzle2dWindowCommandWork {", "    ephemeral: Option<EphemeralEmit<EditorApp<Puzzle2dPlayApp>>>,\n    context: Option<std::sync::Arc<semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle2dPlayApp>>>>,\n}\n\nimpl Puzzle2dWindowCommandWork {")
    tree.sub(E, "window_transient: None, ephemeral: None }", "window_transient: None, ephemeral: None, context: None }")
    tree.cut(E, "/// 🪟️ Whether `action` may publish its window's transient: a retained verb only when its exact publication contract", "/// 📐️ A canonical document revision as lowercase hex", "", keep_last=True)
    tree.sub(E, "impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle2dPlayApp>> for Puzzle2dWindowCommandWork {\n    fn tool_id(&self) -> &'static str {\n        self.tool_id\n    }\n", "impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle2dPlayApp>> for Puzzle2dWindowCommandWork {\n    fn tool_id(&self) -> &'static str {\n        self.tool_id\n    }\n\n    fn bind_job_context(&mut self, context: std::sync::Arc<semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle2dPlayApp>>>) {\n        self.context = Some(context);\n    }\n")
    tree.sub(E, "        let (emit, ephemeral) = puzzle2d_dispatch_emit(", "        let detached = semio_framework_plugin::app::GestureSlot::detached();\n        let gesture = self.context.as_ref().map_or(&detached, |context| context.gesture());\n        let (emit, ephemeral) = puzzle2d_dispatch_emit(")
    for path in (E, GATE):
        tree.sub(path, "&selection, &self.authoring_seed, &self.base_revision, None)?", "&selection, &self.authoring_seed, &self.base_revision, gesture, None)?")
        tree.sub(path, "authoring_seed, &base_revision, doc.operation_optional().cloned())", f"authoring_seed, &base_revision, {DETACHED}, doc.operation_optional().cloned())")
    tree.sub(E, "    base_revision: &str,\n    operation: Option<semio_framework_plugin::AppOperationContext>,\n", "    base_revision: &str,\n    gesture: &semio_framework_plugin::app::GestureSlot<Puzzle2dMutation>,\n    operation: Option<semio_framework_plugin::AppOperationContext>,\n")
    tree.sub(E, "            base_revision,\n            transaction: &mut transaction,\n", "            base_revision,\n            transaction: &mut transaction,\n            gesture,\n")
    tree.cut(E, "    // 🛠️ A window that left the select utility retires the select tool: its in-flight gesture aborts with zero trace", "    let retired_gesture = active_utility != select_utility::UTILITY_ID", "")
    tree.sub(E, "    // actual document mutation (would silently desync remote clients' UI from the committed operation), nor with a\n    // retired gesture whose preview must leave the window.\n    if (!operations.is_empty() || retired_gesture) && matches!(ui_scope, UiDirtyScope::None) {", "    // actual document mutation (would silently desync remote clients' UI from the committed operation).\n    if !operations.is_empty() && matches!(ui_scope, UiDirtyScope::None) {")
    tree.sub(E, "    // 🛠️ A committed select-tool transaction stamps every op of this ONE edit — the yielded leaves and any\n    // scene delta the same batch carried — so history lists the gesture as one row whose inputs stay editable.\n", "    // 🛠️ The first tool transaction an action commits stamps every op of its ONE edit — the yielded leaves and any\n    // scene delta the same dispatch carried — so history lists the gesture as one row whose inputs stay editable.\n")
    tree.cut(E, "    /// 📨️ Every host event ends the window's open select-tool gesture with zero trace under the reason the tool records:", "        Some(Puzzle2dCommand::TranslateSelection { window_id: Some(event.window_id().to_string()), args: Some(json!({ \"phase\": \"abort\"", TOPOLOGY.rstrip("\n").rsplit("\n", 1)[0], keep_last=False)
    tree.sub(E, "        work.bind_view_state(request.context.view_state.clone());\n", "        work.bind_view_state(request.context.view_state.clone());\n        work.bind_job_context(std::sync::Arc::clone(&request.context));\n")
    tree.sub(E, "#[cfg(test)]\n#[path = \"🧪️tests/🧪️select-tool-transactions/🦀️.rs\"]\nmod select_tool_transaction_tests;\n", "#[cfg(test)]\n#[path = \"🧪️tests/🧪️select-tool-transactions/🦀️.rs\"]\nmod select_tool_transaction_tests;\n\n/// 🧱️ The board tools' laws — every board gesture that is not a selection transform is ONE tool transaction of its\n/// tool, a transient flush is no history row, and the `vortex` topology names every board entity.\n#[cfg(test)]\n#[path = \"🧪️tests/🧪️board-tools/🦀️.rs\"]\nmod board_tool_tests;\n")

    # the three transform verbs read the framework phase
    for name, old_use, new_use in (
        ("🚀️translate-selection", "use crate::editor::puzzle2d::{Puzzle2dActionCtx, Puzzle2dSelectPhase, Puzzle2dSelectionRecord};", "use crate::editor::puzzle2d::{puzzle2d_gesture_phase, Puzzle2dActionCtx, Puzzle2dSelectionRecord};"),
        ("🔄️rotate-selection", "use crate::editor::puzzle2d::{puzzle2d_selection_pivot, Puzzle2dActionCtx, Puzzle2dSelectPhase, Puzzle2dSelectionMotion, Puzzle2dSelectionRecord};", "use crate::editor::puzzle2d::{puzzle2d_gesture_phase, puzzle2d_selection_pivot, Puzzle2dActionCtx, Puzzle2dSelectionMotion, Puzzle2dSelectionRecord};"),
        ("📏️scale-selection", "use crate::editor::puzzle2d::{puzzle2d_selection_pivot, Puzzle2dActionCtx, Puzzle2dSelectPhase, Puzzle2dSelectionMotion, Puzzle2dSelectionRecord};", "use crate::editor::puzzle2d::{puzzle2d_gesture_phase, puzzle2d_selection_pivot, Puzzle2dActionCtx, Puzzle2dSelectionMotion, Puzzle2dSelectionRecord};"),
    ):
        path = f"{ED}/🎮️commands/{name}/🦀️.rs"
        tree.sub(path, old_use + "\n", new_use + "\nuse semio_framework_tool_machine::GesturePhase;\n")
        tree.sub(path, "    let Some(phase) = Puzzle2dSelectPhase::from_args(args) else { return };", "    let Some(phase) = puzzle2d_gesture_phase(args) else { return };")
        tree.sub(path, "matches!(phase, Puzzle2dSelectPhase::Once | Puzzle2dSelectPhase::Stream)", "matches!(phase, GesturePhase::Once | GesturePhase::Stream)")
    tree.sub(f"{ED}/🎮️commands/🚀️translate-selection/🦀️.rs", "/// `phase: \"abort\"` (with a `reason`) drops it with zero trace — the app's own answer to a host event\n/// (`Puzzle2dPlayApp::host_event`), which no host sends itself.\n", "/// `phase: \"abort\"` (with a `reason`) drops it with zero trace; a host fact (a blur, a lost capture, a frozen\n/// document) ends it in the window's gesture slot without any verb.\n")

    # window transient and runtime no longer carry a gesture
    tree.sub(WINDOW, "use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select::Puzzle2dSelectToolState;\n", "")
    tree.cut(WINDOW, "    /// 🛠️ The window's in-flight select-tool gesture: statechart configuration and open transaction", "    pub select_tool: Option<Box<Puzzle2dSelectToolState>>,", "")
    tree.sub(WINDOW, "suggestion_menu, select_tool });", "suggestion_menu });")
    tree.sub(WINDOW, "    let select_tool = transient.select_tool.as_ref().map(semio_framework_value::ToValue::to_value);\n", "")
    tree.sub(WINDOW, "    pending.try_reserve(transient.brush_candidates.len() + 1).ok()?;", "    pending.try_reserve(transient.brush_candidates.len()).ok()?;")
    tree.sub(WINDOW, "    pending.extend(select_tool.iter());\n", "")
    tree.sub(WINDOW, "        select_tool: transient.select_tool.as_deref().cloned(),\n", "")
    tree.sub(WINDOW, "            select_tool: runtime.select_tool.clone().map(Box::new),\n", "")
    tree.cut(CONFIG, "    /// 🛠️ The window's in-flight select-tool gesture (its open transaction), ridden from and back to the window", "    pub select_tool: Option<", "")
    tree.sub(CONFIG, "            select_tool: None,\n", "")
    tree.cut(WINDOW_TS, "/** 🛠️ A window's in-flight select-tool gesture between dispatches", "export interface Puzzle2dWindowTransient {", "", keep_last=True)
    tree.sub(WINDOW_TS, "  selectTool?: Puzzle2dSelectToolState | null;\n", "")
    schema = json.loads(tree.text(WINDOW_JSON))
    if "Puzzle2dSelectToolState" not in schema["definitions"]:
        sys.exit(f"{WINDOW_JSON}: the select-tool state definition is already gone")
    text = tree.text(WINDOW_JSON)
    tree.sub(WINDOW_JSON, ",\n        \"selectTool\": {\"anyOf\": [{\"$ref\": \"#/definitions/Puzzle2dSelectToolState\"}, {\"type\": \"null\"}], \"x-semio-state\": \"window-transient\"}\n", "\n")
    lines = tree.text(WINDOW_JSON).split("\n")
    begin = [index for index, line in enumerate(lines) if line == "    \"Puzzle2dSelectToolState\": {"]
    if len(begin) != 1:
        sys.exit(f"{WINDOW_JSON}: the definition header resolves {len(begin)} times")
    end = next(index for index in range(begin[0] + 1, len(lines)) if lines[index] in ("    },", "    }"))
    kept = lines[: begin[0]] + lines[end + 1 :]
    if lines[end] == "    }":
        last = max(index for index in range(begin[0]) if kept[index].strip())
        if not kept[last].endswith(","):
            sys.exit(f"{WINDOW_JSON}: the definition before the removed one does not end with a comma")
        kept[last] = kept[last][:-1]
    tree.after[WINDOW_JSON] = "\n".join(kept)
    after = json.loads(tree.after[WINDOW_JSON])
    del schema["definitions"]["Puzzle2dSelectToolState"]
    for definition in schema["definitions"].values():
        definition.get("properties", {}).pop("selectTool", None)
    if after != schema:
        sys.exit(f"{WINDOW_JSON}: the text edit is not exactly the removal of the select-tool state")
    del text

    # laws
    laws = (STAGED / "select-tool-laws.rs").read_text(encoding="utf-8")
    tree.sub(SELECT_LAWS, "use protocol::Mutation as _;\n", "use protocol::Mutation as _;\nuse semio_framework_tool_machine::{drive_chart_gesture, ChartGesture, GestureDrive, GestureState, GestureTool, ToolAbortReason, ToolRefusal};\n")
    tree.sub(SELECT_LAWS, "//! all-locked, identity, missing and empty requests leave zero trace, and a streamed gesture persists, resumes\n//! by stable ids and commits as ONE transaction or aborts with zero trace.\n", "//! all-locked, identity, missing and empty requests leave zero trace, a streamed gesture is the window's ONE gesture on\n//! the framework runner (resumed by stable ids, committed as ONE transaction or dropped with zero trace), and a committed\n//! leaf states its inputs at their declared precision.\n")
    lines = tree.text(SELECT_LAWS).split("\n")
    begin = [index for index, line in enumerate(lines) if line == "fn a_streamed_gesture_spans_dispatches_and_commits_one_transaction() {"]
    end = [index for index, line in enumerate(lines) if line == "fn a_persisted_gesture_round_trips_the_window_transient_wire() {"]
    if len(begin) != 1 or len(end) != 1 or lines[begin[0] - 1] != "#[test]":
        sys.exit(f"{SELECT_LAWS}: the streamed-gesture laws do not resolve")
    close = next(index for index in range(end[0], len(lines)) if lines[index] == "}")
    tree.after[SELECT_LAWS] = "\n".join(lines[: begin[0] - 1] + [laws.rstrip("\n")] + lines[close + 1 :])
    lines = tree.text(TRANSACTION_LAWS).split("\n")
    begin = [index for index, line in enumerate(lines) if line.startswith("/// 🌊️ The window transient a streamed `translateSelection` of `left` by `dx` persisted on `revision`.")]
    end = [index for index, line in enumerate(lines) if line == "fn a_one_shot_interrupting_a_threaded_gesture_commits_only_itself() {"]
    if len(begin) != 1 or len(end) != 1:
        sys.exit(f"{TRANSACTION_LAWS}: the threaded laws do not resolve")
    close = next(index for index in range(end[0], len(lines)) if lines[index] == "}")
    tree.after[TRANSACTION_LAWS] = "\n".join(lines[: begin[0]] + lines[close + 1 :])
    tree.sub(TRANSACTION_LAWS, "&selection, \"seed-3\", \"rev-1\", None).expect(\"emit\");", f"&selection, \"seed-3\", \"rev-1\", {DETACHED}, None).expect(\"emit\");")
    tree.sub(TRANSACTION_LAWS, "seed, \"rev-1\", None).expect(\"emit\").0;", f"seed, \"rev-1\", {DETACHED}, None).expect(\"emit\").0;")

    for path, staged in NEW.items():
        if (ROOT / path).exists():
            sys.exit(f"{path}: already exists")
        tree.before[path] = None
        tree.after[path] = (STAGED / staged).read_text(encoding="utf-8")


def digest(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else ""
    if mode == "--restore":
        manifest = json.loads((KEPT / "manifest.json").read_text(encoding="utf-8"))
        for path, entry in manifest.items():
            current = (ROOT / path).read_text(encoding="utf-8") if (ROOT / path).exists() else None
            if current is None or digest(current) != entry["after"]:
                sys.exit(f"{path}: changed since the wave landed — restore refused, nothing written")
        for path, entry in manifest.items():
            if entry["before"] is None:
                (ROOT / path).unlink()
            else:
                (ROOT / path).write_text((KEPT / entry["before"]).read_text(encoding="utf-8"), encoding="utf-8")
        print(f"restored {len(manifest)} files")
        return
    if mode not in ("--check", "--apply"):
        sys.exit(__doc__)
    tree = Tree()
    edit(tree)
    changed = {path: text for path, text in tree.after.items() if tree.before[path] != text}
    for path, text in changed.items():
        before = tree.before[path]
        print(f"{'new ' if before is None else ''}{len(text.splitlines()) - (0 if before is None else len(before.splitlines())):+5d} lines  {path.split('/')[-3]}/{path.split('/')[-2]}/{path.split('/')[-1]}")
    if mode == "--check":
        print(f"{len(changed)} files would change")
        return
    KEPT.mkdir(parents=True, exist_ok=True)
    manifest = {}
    for index, (path, text) in enumerate(changed.items()):
        before = tree.before[path]
        name = None if before is None else f"{index:02d}.before"
        if before is not None:
            (KEPT / name).write_text(before, encoding="utf-8")
        manifest[path] = {"before": name, "after": digest(text)}
    (KEPT / "manifest.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=1), encoding="utf-8")
    for path, text in changed.items():
        (ROOT / path).parent.mkdir(parents=True, exist_ok=True)
        (ROOT / path).write_text(text, encoding="utf-8")
    print(f"applied {len(changed)} files")


if __name__ == "__main__":
    main()
