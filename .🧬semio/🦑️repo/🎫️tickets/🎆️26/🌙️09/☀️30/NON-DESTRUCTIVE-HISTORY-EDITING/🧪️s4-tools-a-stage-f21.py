"""🧳️ Rule-45 staging of audit item F21 (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, `📓️s4-tools-a-report.md` S4.5b).

Two compile-atomic groups of shared-crate edits, re-derived from the CURRENT tree by anchored replacements so they can land
after peers moved the files:

- `drive`: `drive_gesture` answers `Result<GestureDrive, ToolRefusal>` (a refused start or tick is the dispatch's refusal,
  with no effect at all; an unrestorable gesture is dropped with zero trace), its fixture law `🧪️tests/🧪️gesture-drive-law`
  (replacing the ad-hoc `🌊️GestureLaws` unit region) and every caller: fem 2d/3d gumball, lowpoly paint, raster paint
  stroke, generation3d gumball, wfc bitmap brush, mapping the refusal to its `toolTransaction.*` fault.
- `schema`: `#[derive(MutationLeaf)]` looks a leaf's referenced documents up in its own tree AND the plugins its crate's
  `[dependencies]` name by path (twin of the landed repo-test gate `mutationSchemaSearchRoots`), so layout's
  `change-data-fields` publishes forms' dictionary schema beside it; plus its fixture law.
- `emit`: `Emit::child_node_drag` (node-drag machine + composed child in one call) replacing `Emit::node_drag_child` and the
  redundant `ui_scope: Full` / `Nothing` matches in flow, dag, wires and sequence (x2).

Usage (cwd: repo root):
  python3 🧪️s4-tools-a-stage-f21.py              stage both groups under 🗑️generated/s4-tools-a/staged/ (copies + .patch)
  python3 🧪️s4-tools-a-stage-f21.py --land drive  write the group into the tree (only at "CARGO OPEN", with its check)
"""

import difflib
import pathlib
import sys

REPO = pathlib.Path(__file__).resolve().parents[7]
STAGED = pathlib.Path(__file__).resolve().parent / "🗑️generated/s4-tools-a/staged"
TM = "🧰️framework/🔨️modules/🛠️tool-machine"
PLUGIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
P = "✏️s/🔌️plugins"
FEM2 = f"{P}/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor"
FEM3 = f"{P}/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor"
LOWPOLY = f"{P}/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
RASTER = f"{P}/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
GEN3D = f"{P}/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
BITMAP = f"{P}/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
FLOW = f"{P}/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
DAG = f"{P}/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
WIRES = f"{P}/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
SEQUENCE = f"{P}/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"

OLD_DRIVE = '''/// 🚂️ Drives one window's streamed tool through ONE dispatch. `Once` commits `tick` as one transaction; `Stream` upserts
/// it into the window's open transaction (opening it on the first tick), `Commit` folds it in and commits the whole
/// gesture, `Abort` drops the open gesture with zero trace. An open gesture another verb or a one-shot interrupts is
/// aborted `captureLost`; one whose base moved under it is aborted `baseMoved`, and a stream tick or commit that found
/// it is dropped with it.
pub fn drive_gesture<T: GestureTool>(persisted: Option<&T::Gesture>, verb: &str, phase: GesturePhase, tick: Option<T::Tick>, authoring_seed: &str, base_revision: &str) -> GestureDrive<T::Gesture, T::Mutation> {
    let dropped = || GestureDrive { committed: None, next: persisted.map(|_| None) };
    let open = match (persisted.and_then(|gesture| T::resume(gesture).ok()), phase) {
        (Some(mut tool), GesturePhase::Abort(reason)) => {
            tool.abort(reason);
            return dropped();
        }
        (None, GesturePhase::Abort(_)) => return dropped(),
        (Some(mut tool), _) if tool.base_revision() != base_revision => {
            tool.abort(ToolAbortReason::BaseMoved);
            if phase != GesturePhase::Once {
                return dropped();
            }
            None
        }
        (Some(mut tool), _) if tool.verb() != verb || phase == GesturePhase::Once => {
            tool.abort(ToolAbortReason::CaptureLost);
            None
        }
        (open, _) => open,
    };
    let Some(mut tool) = open.or_else(|| T::start(verb, authoring_seed, base_revision).ok()) else { return dropped() };
    let step = tool.send(phase, tick);
    let gesture = tool.persist();
    let next = (gesture.as_ref() != persisted).then_some(gesture);
    match step {
        Ok(ToolStep::Committed(reference, mutations)) => GestureDrive { committed: Some((reference, mutations)), next },
        Ok(_) | Err(_) => GestureDrive { committed: None, next },
    }
}'''

NEW_DRIVE = '''/// 🚂️ Drives one window's streamed tool through ONE dispatch (law `🧫️fixtures/🧫️gesture-drive-law`). `Once` commits
/// `tick` as one transaction; `Stream` upserts it into the window's open transaction (opening it on the first tick),
/// `Commit` folds it in and commits the whole gesture, `Abort` drops the open gesture with zero trace. A gesture whose base
/// moved under it is aborted `baseMoved` (a stream tick or commit that found it is dropped with it, a one-shot commits
/// fresh); otherwise another verb or a one-shot interrupts it (`captureLost`). A gesture its tool cannot restore is dropped
/// with zero trace and the dispatch runs from rest; a refused start or tick is the dispatch's refusal, with no effect at all.
pub fn drive_gesture<T: GestureTool>(persisted: Option<&T::Gesture>, verb: &str, phase: GesturePhase, tick: Option<T::Tick>, authoring_seed: &str, base_revision: &str) -> Result<GestureDrive<T::Gesture, T::Mutation>, ToolRefusal> {
    let dropped = || GestureDrive { committed: None, next: persisted.map(|_| None) };
    let (open, interrupted) = match (persisted.and_then(|gesture| T::resume(gesture).ok()), phase) {
        (Some(mut tool), GesturePhase::Abort(reason)) => {
            tool.abort(reason);
            return Ok(dropped());
        }
        (None, GesturePhase::Abort(_)) => return Ok(dropped()),
        (Some(mut tool), _) if tool.base_revision() != base_revision && phase != GesturePhase::Once => {
            tool.abort(ToolAbortReason::BaseMoved);
            return Ok(dropped());
        }
        (Some(tool), _) if tool.base_revision() != base_revision => (None, Some((tool, ToolAbortReason::BaseMoved))),
        (Some(tool), _) if tool.verb() != verb || phase == GesturePhase::Once => (None, Some((tool, ToolAbortReason::CaptureLost))),
        (open, _) => (open, None),
    };
    let mut tool = match open {
        Some(tool) => tool,
        None => T::start(verb, authoring_seed, base_revision)?,
    };
    let step = tool.send(phase, tick)?;
    if let Some((mut interrupted, reason)) = interrupted {
        interrupted.abort(reason);
    }
    let gesture = tool.persist();
    let next = (gesture.as_ref() != persisted).then_some(gesture);
    let committed = match step {
        ToolStep::Committed(reference, mutations) => Some((reference, mutations)),
        ToolStep::Idle | ToolStep::Open | ToolStep::Aborted(..) | ToolStep::Empty(_) => None,
    };
    Ok(GestureDrive { committed, next })
}'''

GESTURE_DRIVE_LAW_RS = '''//! 🌊️ The shared streamed-gesture runner (`drive_gesture`) against `🧫️fixtures/🧫️gesture-drive-law`, authored by an
//! independent Python model of its counting tool (`🧪️s4-tools-a-gesture-drive-law.py` of ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING) and judged in TypeScript against xstate and fast-check: the phase argument
//! table, and one dispatch per row from the window's persisted gesture with the abort reasons the drive called.

use super::*;
use serde_json::{json, Value};
use std::cell::RefCell;

const GESTURE_DRIVE_LAW: &str = include_str!("../../🧫️fixtures/🧫️gesture-drive-law/🔣️.json");

thread_local! {
    static ABORTED: RefCell<Vec<ToolAbortReason>> = const { RefCell::new(Vec::new()) };
}

fn law() -> Value {
    serde_json::from_str(GESTURE_DRIVE_LAW).expect("the gesture-drive law parses")
}

fn text(value: &Value) -> &str {
    value.as_str().expect("a string")
}

/// 📜️ The counting tool's contract the fixture names: the verb `start` refuses and the code of each refusal.
struct Contract {
    refuse_start_verb: String,
    start: ToolRefusal,
    resume: ToolRefusal,
    send: ToolRefusal,
}

fn contract() -> &'static Contract {
    static CONTRACT: std::sync::OnceLock<Contract> = std::sync::OnceLock::new();
    CONTRACT.get_or_init(|| {
        let tool = law()["tool"].clone();
        let refusal = |key: &str| ToolRefusal::parse(text(&tool[key])).expect("a known refusal");
        Contract { refuse_start_verb: text(&tool["refuseStartVerb"]).to_string(), start: refusal("startRefusal"), resume: refusal("resumeRefusal"), send: refusal("sendRefusal") }
    })
}

/// 🧾️ The counting tool's persisted open gesture; `corrupt` marks one its `resume` refuses.
#[derive(Clone, Debug, PartialEq)]
struct LawGesture {
    verb: String,
    base: String,
    ticks: Vec<i64>,
    corrupt: bool,
}

impl LawGesture {
    fn from_json(value: &Value) -> Option<Self> {
        value.is_object().then(|| Self {
            verb: text(&value["verb"]).to_string(),
            base: text(&value["base"]).to_string(),
            ticks: value["ticks"].as_array().expect("ticks").iter().map(|tick| tick.as_i64().expect("an integer tick")).collect(),
            corrupt: value["corrupt"].as_bool().unwrap_or(false),
        })
    }

    fn to_json(&self) -> Value {
        let mut gesture = json!({ "verb": self.verb, "base": self.base, "ticks": self.ticks });
        if self.corrupt {
            gesture["corrupt"] = json!(true);
        }
        gesture
    }
}

/// 🧮️ The fixture's counting tool: a stream tick accumulates (open while it has ticks), a one-shot or commit folds its
/// tick in and commits the ticks, an abort clears them; `send` refuses a negative tick.
struct LawTool {
    verb: String,
    base: String,
    ticks: Vec<i64>,
}

impl GestureTool for LawTool {
    type Gesture = LawGesture;
    type Tick = i64;
    type Mutation = i64;

    fn start(verb: &str, _authoring_seed: &str, base_revision: &str) -> Result<Self, ToolRefusal> {
        if verb == contract().refuse_start_verb {
            return Err(contract().start);
        }
        Ok(Self { verb: verb.into(), base: base_revision.into(), ticks: Vec::new() })
    }

    fn resume(gesture: &LawGesture) -> Result<Self, ToolRefusal> {
        if gesture.corrupt {
            return Err(contract().resume);
        }
        Ok(Self { verb: gesture.verb.clone(), base: gesture.base.clone(), ticks: gesture.ticks.clone() })
    }

    fn verb(&self) -> &str {
        &self.verb
    }

    fn base_revision(&self) -> &str {
        &self.base
    }

    fn abort(&mut self, reason: ToolAbortReason) {
        ABORTED.with(|aborted| aborted.borrow_mut().push(reason));
        self.ticks.clear();
    }

    fn send(&mut self, phase: GesturePhase, tick: Option<i64>) -> Result<ToolStep<i64>, ToolRefusal> {
        if tick.is_some_and(|tick| tick < 0) {
            return Err(contract().send);
        }
        self.ticks.extend(tick);
        Ok(match phase {
            _ if self.ticks.is_empty() => ToolStep::Idle,
            GesturePhase::Stream => ToolStep::Open,
            GesturePhase::Once | GesturePhase::Commit | GesturePhase::Abort(_) => ToolStep::Committed(TransactionRef { id: format!("{}#1", self.verb), tool: self.verb.clone() }, std::mem::take(&mut self.ticks)),
        })
    }

    fn persist(self) -> Option<LawGesture> {
        (!self.ticks.is_empty()).then(|| LawGesture { verb: self.verb, base: self.base, ticks: self.ticks, corrupt: false })
    }
}

/// 🏃️ One dispatch through `drive_gesture` from the window's persisted gesture, reported in the fixture's outcome shape.
fn outcome(persisted: Option<&LawGesture>, dispatch: &Value) -> Value {
    ABORTED.with(|aborted| aborted.borrow_mut().clear());
    let verb = text(&dispatch["verb"]);
    let phase = GesturePhase::parse(dispatch["phase"].as_str(), dispatch["reason"].as_str()).expect("a known phase");
    let result = drive_gesture::<LawTool>(persisted, verb, phase, dispatch["tick"].as_i64(), "seed", text(&dispatch["base"]));
    let aborted = ABORTED.with(|aborted| aborted.borrow().clone());
    assert!(aborted.len() <= 1, "one dispatch aborts at most one gesture: {aborted:?}");
    let aborted = aborted.first().map_or(Value::Null, |reason| json!(reason.as_str()));
    match result {
        Err(refusal) => json!({ "aborted": aborted, "committed": null, "next": "unchanged", "refused": refusal.code() }),
        Ok(drive) => json!({
            "aborted": aborted,
            "committed": drive.committed.map_or(Value::Null, |(reference, ticks)| {
                assert_eq!(reference.tool, verb, "a commit is stamped by the dispatching verb's transaction");
                json!(ticks)
            }),
            "next": match drive.next {
                None => json!("unchanged"),
                Some(None) => json!("cleared"),
                Some(Some(gesture)) => gesture.to_json(),
            },
            "refused": null,
        }),
    }
}

/// 🗣️ LAW: a gesture verb's `phase`/`reason` arguments parse like the fixture's table (absent phase = one-shot, absent
/// reason = `tool`, unknown words refused).
#[test]
fn the_phase_arguments_parse_like_the_fixture() {
    for row in law()["phases"].as_array().expect("phases") {
        let parsed = match GesturePhase::parse(row["args"]["phase"].as_str(), row["args"]["reason"].as_str()) {
            None => Value::Null,
            Some(GesturePhase::Once) => json!({ "kind": "once" }),
            Some(GesturePhase::Stream) => json!({ "kind": "stream" }),
            Some(GesturePhase::Commit) => json!({ "kind": "commit" }),
            Some(GesturePhase::Abort(reason)) => json!({ "kind": "abort", "reason": reason.as_str() }),
        };
        assert_eq!(parsed, row["phase"], "{}", row["args"]);
    }
}

/// 🎞️ LAW: every fixture row drives to its expected outcome — one-shots, streams, commits and aborts at rest, open and
/// unrestorable; moved bases, verb switches and one-shot interruptions; refused starts and ticks with no effect at all.
#[test]
fn every_fixture_row_drives_to_its_expected_outcome() {
    for row in law()["rows"].as_array().expect("rows") {
        let persisted = LawGesture::from_json(&row["persisted"]);
        assert_eq!(outcome(persisted.as_ref(), &row["dispatch"]), row["expected"], "{}", row["name"]);
    }
}
'''


DERIVE = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive"

OLD_REFERENCED = """/// 🔗️ Every schema document the payload schema at `payload_schema` references by an absolute `$id` (fragment stripped),
/// transitively, among the `🧬️schema` JSON documents of its search tree ([`mutation_schema_search_root`]) — what the runtime
/// publishes beside the leaf (`MutationLeaf::PAYLOAD_SCHEMA_DOCUMENTS`), so an input that `$ref`s another facet of its scope
/// resolves. A reference the tree does not hold is left to the OS-wide registry, which framework scopes publish themselves.
/// Sorted, never the payload schema itself.
fn mutation_leaf_referenced_documents(payload_schema: &Path) -> Result<Vec<PathBuf>, String> {
    let index = mutation_schema_document_index(&mutation_schema_search_root(payload_schema));"""

NEW_REFERENCED = """/// 🔗️ Every schema document the payload schema at `payload_schema` references by an absolute `$id` (fragment stripped),
/// transitively, among the `🧬️schema` JSON documents of its search trees ([`mutation_schema_search_roots`]: its own, then the
/// plugins its crate depends on) — what the runtime publishes beside the leaf (`MutationLeaf::PAYLOAD_SCHEMA_DOCUMENTS`), so an
/// input that `$ref`s another facet of its scope resolves. A reference no tree holds is left to the OS-wide registry, which
/// framework scopes publish themselves. Sorted, never the payload schema itself.
fn mutation_leaf_referenced_documents(payload_schema: &Path) -> Result<Vec<PathBuf>, String> {
    let indexes: Vec<_> = mutation_schema_search_roots(payload_schema).iter().map(|root| mutation_schema_document_index(root)).collect();"""

OLD_SEARCH_END = """    under("🔌️plugins").or_else(|| under("🔨️modules")).or(owner).or_else(|| payload_schema.parent()).unwrap_or(payload_schema).to_path_buf()
}
"""

NEW_SEARCH_END = """    under("🔌️plugins").or_else(|| under("🔨️modules")).or(owner).or_else(|| payload_schema.parent()).unwrap_or(payload_schema).to_path_buf()
}

/// 🧶️ The trees a leaf's referenced documents are looked up in, own first: its own ([`mutation_schema_search_root`]), then the
/// plugin of every path dependency its crate declares ([`mutation_leaf_crate_manifest`]) — a leaf references across exactly the
/// plugins its types are built from (twin: `mutationSchemaSearchRoots` of the repo test platform).
fn mutation_schema_search_roots(payload_schema: &Path) -> Vec<PathBuf> {
    let manifest = mutation_leaf_crate_manifest(payload_schema);
    let text = manifest.as_deref().and_then(|manifest| fs::read_to_string(manifest).ok());
    let dependencies = manifest.zip(text).map(|(manifest, text)| mutation_manifest_path_dependencies(&manifest, &text)).unwrap_or_default();
    let plugins = dependencies.into_iter().filter_map(|dependency| dependency.ancestors().find(|ancestor| ancestor.parent().and_then(Path::file_name).is_some_and(|name| name == "🔌️plugins")).map(Path::to_path_buf));
    std::iter::once(mutation_schema_search_root(payload_schema)).chain(plugins).fold(Vec::new(), |mut roots, root| {
        if !roots.contains(&root) {
            roots.push(root);
        }
        roots
    })
}

/// 🏗️ The crate manifest a leaf's types are built by: the nearest `Cargo.toml` above the payload schema, beside a directory or in
/// its `📦️packages/🦀️rust` (twin: `mutationLeafCrateManifest`).
fn mutation_leaf_crate_manifest(payload_schema: &Path) -> Option<PathBuf> {
    payload_schema.ancestors().skip(1).flat_map(|directory| [directory.join("Cargo.toml"), directory.join("📦️packages").join("🦀️rust").join("Cargo.toml")]).find(|manifest| manifest.is_file())
}

/// 🔩️ The `path` of every inline `[dependencies]` entry of the crate manifest at `manifest` whose text is `text`, resolved beside it
/// and normalized lexically, in manifest order (twin: `mutationManifestPathDependencies`; law `🧫️fixtures/🧫️manifest-path-dependencies`).
fn mutation_manifest_path_dependencies(manifest: &Path, text: &str) -> Vec<PathBuf> {
    let directory = manifest.parent().unwrap_or(manifest);
    let mut dependencies = false;
    let mut found = Vec::new();
    for line in text.lines().map(str::trim) {
        if line.starts_with('[') {
            dependencies = line == "[dependencies]";
            continue;
        }
        let Some((_, table)) = line.split_once('=').filter(|_| dependencies) else { continue };
        let path = table.trim().strip_prefix('{').and_then(|table| table.split(',').find_map(|entry| entry.trim().strip_prefix("path")?.trim_start().strip_prefix('=')?.trim_start().strip_prefix('"')?.split('"').next()));
        if let Some(path) = path {
            found.push(directory.join(path).components().fold(PathBuf::new(), |mut normal, component| {
                match component {
                    Component::ParentDir => {
                        normal.pop();
                    }
                    Component::CurDir => {}
                    other => normal.push(other),
                }
                normal
            }));
        }
    }
    found
}
"""

DERIVE_LAW = """

/// 🔩️ LAW: the crate-manifest reader takes exactly the inline `path` values of the `[dependencies]` table, resolved beside the
/// manifest and normalized lexically, like the language-agnostic cases (judged against Bun's TOML parser in the TS twin).
#[test]
fn reads_the_path_dependencies_of_a_crate_manifest_like_the_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️manifest-path-dependencies/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let paths: Vec<PathBuf> = case["paths"].as_array().unwrap().iter().map(|path| PathBuf::from(path.as_str().unwrap())).collect();
        assert_eq!(mutation_manifest_path_dependencies(Path::new(case["manifest"].as_str().unwrap()), case["text"].as_str().unwrap()), paths, "{}", case["name"]);
    }
}
"""


def tm_unit_cut(text):
    start = text.index("\n//#region 🌊️GestureLaws\n")
    end = text.index("//#endregion 🌊️GestureLaws\n") + len("//#endregion 🌊️GestureLaws\n")
    return text[:start] + text[end:]


def expect_calls(prefix, message, count):
    """🪝️ Appends `.expect(message)` after each balanced call starting with `prefix`; `count` pins how many."""

    def apply(text):
        out, cursor, found = [], 0, 0
        while (start := text.find(prefix, cursor)) != -1:
            depth, index = 0, start + len(prefix) - 1
            while True:
                depth += {"(": 1, ")": -1}.get(text[index], 0)
                if depth == 0:
                    break
                index += 1
            out.append(text[cursor : index + 1] + f'.expect("{message}")')
            cursor, found = index + 1, found + 1
        if found != count:
            raise SystemExit(f"{prefix}: {found} calls, expected {count}")
        return "".join(out) + text[cursor:]

    return apply


def replace(old, new):
    def apply(text):
        if text.count(old) != 1:
            raise SystemExit(f"anchor matched {text.count(old)} times: {old[:90]!r}")
        return text.replace(old, new)

    return apply


FAULT = 'semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, refusal.code(), "{}")'

GROUPS = {
    "drive": {
        f"{TM}/🦀️.rs": [
            replace(OLD_DRIVE, NEW_DRIVE),
            replace('#[path = "🧪️tests/🧪️node-graph-edit-rows/🦀️.rs"]\nmod node_graph_edit_rows_tests;\n', '#[path = "🧪️tests/🧪️node-graph-edit-rows/🦀️.rs"]\nmod node_graph_edit_rows_tests;\n#[cfg(test)]\n#[path = "🧪️tests/🧪️gesture-drive-law/🦀️.rs"]\nmod gesture_drive_law_tests;\n'),
        ],
        f"{TM}/🧪️tests/🔬️unit/🦀️.rs": [tm_unit_cut],
        f"{TM}/🧪️tests/🧪️gesture-drive-law/🦀️.rs": GESTURE_DRIVE_LAW_RS,
        f"{FEM2}/🫧️transient/🦀️.rs": [
            replace(
                "/// ([`semio_framework_tool_machine::drive_gesture`]) against the gesture this transient holds for that window.\npub fn fem_gumball_drive<T: GestureTool<Gesture = FemGumballGesture>>(transient: &FemGumballTransient, window: &str, verb: &str, phase: GesturePhase, tick: Option<T::Tick>, authoring_seed: &str, base_revision: &str) -> FemGumballDrive<T::Mutation> {\n    let drive = semio_framework_tool_machine::drive_gesture::<T>(transient.gestures.get(window), verb, phase, tick, authoring_seed, base_revision);\n    FemGumballDrive { committed: drive.committed, transient: drive.next.map(|gesture| transient.with_gesture(window, gesture)) }\n}",
                "/// ([`semio_framework_tool_machine::drive_gesture`]) against the gesture this transient holds for that window; a refused\n/// start or tick raises its tool-transaction fault (`toolTransaction.closed` | `toolTransaction.unclosed`).\npub fn fem_gumball_drive<T: GestureTool<Gesture = FemGumballGesture>>(\n    transient: &FemGumballTransient,\n    window: &str,\n    verb: &str,\n    phase: GesturePhase,\n    tick: Option<T::Tick>,\n    authoring_seed: &str,\n    base_revision: &str,\n) -> Result<FemGumballDrive<T::Mutation>, semio_framework_plugin::Fault> {\n    let drive = semio_framework_tool_machine::drive_gesture::<T>(transient.gestures.get(window), verb, phase, tick, authoring_seed, base_revision)\n        .map_err(|refusal| "
                + FAULT.format("the gumball tool refused the dispatch")
                + ")?;\n    Ok(FemGumballDrive { committed: drive.committed, transient: drive.next.map(|gesture| transient.with_gesture(window, gesture)) })\n}",
            )
        ],
        f"{FEM2}/🎮️commands/🧭️gumball/🦀️.rs": [
            replace("&operation.authoring_seed, &base_revision);\n    let emit", "&operation.authoring_seed, &base_revision)?;\n    let emit"),
            replace('GesturePhase::Once, tick, &seed, "");', 'GesturePhase::Once, tick, &seed, "")?;'),
        ],
        f"{FEM3}/🎮️commands/🧭️gumball/🦀️.rs": [
            replace("&operation.authoring_seed, &base_revision);\n    let emit", "&operation.authoring_seed, &base_revision)?;\n    let emit"),
            replace('GesturePhase::Once, tick, &seed, "");', 'GesturePhase::Once, tick, &seed, "")?;'),
        ],
        f"{FEM2}/🕹️interaction/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs": [expect_calls("crate::editor::fem2d::transient::fem_gumball_drive::<Fem2dGumballTool>(", "the gumball tool accepts the dispatch", 1)],
        f"{FEM3}/🕹️interaction/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs": [expect_calls("    fem_gumball_drive::<Fem3dGumballTool>(", "the gumball tool accepts the dispatch", 1)],
        f"{LOWPOLY}/🖌️session/🦀️.rs": [
            replace(
                "/// ([`semio_framework_tool_machine::drive_gesture`]) against the gesture this transient holds for that window.\npub fn lowpoly_paint_drive(transient: &LowpolyTransient, window: &str, phase: GesturePhase, tick: Option<LowpolyMutation>, authoring_seed: &str, base_revision: &str) -> LowpolyPaintDrive {\n    let drive = semio_framework_tool_machine::drive_gesture::<LowpolyPaintTool>(transient.paint(window), LOWPOLY_PAINT_VERB, phase, tick, authoring_seed, base_revision);\n    LowpolyPaintDrive { committed: drive.committed, transient: drive.next.map(|gesture| transient.with_paint(window, gesture)) }\n}",
                "/// ([`semio_framework_tool_machine::drive_gesture`]) against the gesture this transient holds for that window; a refused\n/// start or tick raises its tool-transaction fault (`toolTransaction.closed` | `toolTransaction.unclosed`).\npub fn lowpoly_paint_drive(transient: &LowpolyTransient, window: &str, phase: GesturePhase, tick: Option<LowpolyMutation>, authoring_seed: &str, base_revision: &str) -> Result<LowpolyPaintDrive, semio_framework_plugin::Fault> {\n    let drive = semio_framework_tool_machine::drive_gesture::<LowpolyPaintTool>(transient.paint(window), LOWPOLY_PAINT_VERB, phase, tick, authoring_seed, base_revision)\n        .map_err(|refusal| "
                + FAULT.format("the paint tool refused the dispatch")
                + ")?;\n    Ok(LowpolyPaintDrive { committed: drive.committed, transient: drive.next.map(|gesture| transient.with_paint(window, gesture)) })\n}",
            )
        ],
        f"{LOWPOLY}/🎮️commands/🖌️paint/🦀️.rs": [replace("&operation.authoring_seed, &base_revision);\n    let emit", "&operation.authoring_seed, &base_revision)?;\n    let emit")],
        f"{RASTER}/🎮️commands/🖌️paint-stroke/🦀️.rs": [replace('let drive = drive_gesture::<RasterPaintTool>(open, press, phase, Some(tick), seed, "");', 'let drive = drive_gesture::<RasterPaintTool>(open, press, phase, Some(tick), seed, "").map_err(|refusal| ' + FAULT.format("the paint tool refused the stroke") + ")?;")],
        f"{GEN3D}/🎮️commands/🧭️transforms/🦀️.rs": [
            replace("    /// the base no longer splices drops the gesture and refuses with the gumball's named code.\n", "    /// the base no longer splices drops the gesture and refuses with the gumball's named code; a refused start or tick\n    /// raises its tool-transaction fault and changes nothing.\n"),
            replace(
                "        let drive = drive_gesture::<Generation3dGumballTool>(self.open.get(request.window), request.verb, request.phase, Some(tick), request.authoring_seed, &base_revision);\n",
                "        let drive = match drive_gesture::<Generation3dGumballTool>(self.open.get(request.window), request.verb, request.phase, Some(tick), request.authoring_seed, &base_revision) {\n            Ok(drive) => drive,\n            Err(refusal) => {\n                retire_rows(splice);\n                return Err(Fault::new(FaultOrigin::App, refusal.code(), \"the gumball tool refused the dispatch\"));\n            }\n        };\n",
            ),
        ],
        f"{GEN3D}/🎮️commands/🧭️transforms/🧪️tests/🔬️unit/🦀️.rs": [
            expect_calls("drive_gesture::<Generation3dGumballTool>(", "the gumball accepts the dispatch", 4),
            replace('"seed", "base").expect("the gumball accepts the dispatch").next.flatten().expect("the tick keeps the gesture open");', '"seed", "base")\n        .expect("the gumball accepts the dispatch")\n        .next\n        .flatten()\n        .expect("the tick keeps the gesture open");'),
        ],
        f"{BITMAP}/🎭️modes/✏️edit/🪟️windows/🖼️input/🪛️utilities/🖌️brush/🦀️.rs": [
            replace(
                "/// stroke that paints no cell and a cancel all leave zero trace.\npub fn bitmap_brush_dispatch(phase: GesturePhase, request: BrushToolRequest, transient: &BitmapInputWindowTransient, authoring_seed: &str) -> (Option<(protocol::TransactionRef, Vec<BitmapMutation>)>, BitmapInputWindowTransient) {\n    let drive = drive_gesture::<BitmapBrushTool>(transient.brush.as_deref(), BITMAP_PAINT_STROKE_VERB, phase, Some(request), authoring_seed, \"\");\n    match drive.committed {\n        Some(committed) => (Some(committed), BitmapInputWindowTransient { brush: None }),\n        None => (None, BitmapInputWindowTransient { brush: drive.next.map_or_else(|| transient.brush.clone(), |next| next.map(Box::new)) }),\n    }\n}",
                "/// stroke that paints no cell and a cancel all leave zero trace; a refused start or tick raises its tool-transaction fault.\npub fn bitmap_brush_dispatch(\n    phase: GesturePhase,\n    request: BrushToolRequest,\n    transient: &BitmapInputWindowTransient,\n    authoring_seed: &str,\n) -> Result<(Option<(protocol::TransactionRef, Vec<BitmapMutation>)>, BitmapInputWindowTransient), semio_framework_plugin::Fault> {\n    let drive = drive_gesture::<BitmapBrushTool>(transient.brush.as_deref(), BITMAP_PAINT_STROKE_VERB, phase, Some(request), authoring_seed, \"\")\n        .map_err(|refusal| "
                + FAULT.format("the brush refused the stroke")
                + ")?;\n    Ok(match drive.committed {\n        Some(committed) => (Some(committed), BitmapInputWindowTransient { brush: None }),\n        None => (None, BitmapInputWindowTransient { brush: drive.next.map_or_else(|| transient.brush.clone(), |next| next.map(Box::new)) }),\n    })\n}",
            )
        ],
        f"{BITMAP}/🦀️.rs": [replace("bitmap_brush_dispatch(phase, BrushToolRequest::on(doc.snapshot, points, color), window, seed);", "bitmap_brush_dispatch(phase, BrushToolRequest::on(doc.snapshot, points, color), window, seed)?;")],
        f"{BITMAP}/🧪️tests/🧪️brush-tool/🦀️.rs": [expect_calls("bitmap_brush_dispatch(", "the brush accepts the dispatch", 21)],
    },
    "schema": {
        f"{DERIVE}/🦀️.rs": [
            replace(OLD_REFERENCED, NEW_REFERENCED),
            replace("        let Some(path) = index.get(&id) else { continue };\n", "        let Some(path) = indexes.iter().find_map(|index| index.get(&id)) else { continue };\n"),
            replace(OLD_SEARCH_END, NEW_SEARCH_END),
        ],
        f"{DERIVE}/🧪️tests/🔬️mutation-leaf-derive/🦀️.rs": [lambda text: text.rstrip("\n") + DERIVE_LAW],
    },
    "emit": {
        PLUGIN: [
            replace(
                """        /// 🪆️ A released node drag (`semio_framework_tool_machine::node_drag_emit`) whose leaves land in the owned child
        /// `slot` member `child_id` (design §12): ONE composite group whose member edit carries the transaction, the child
        /// leaves plainly for a view without command authority, or nothing.
        pub fn node_drag_child<S, M>(drag: semio_framework_tool_machine::NodeDragEmit<M>, slot: &str, child_id: &str) -> Self
        where
            M: protocol::SemanticMutation<S> + ::protocol::OpBinary,
        {
            match drag {""",
                """        /// 🤏️ ONE released node drag of `leaves` through the node-drag machine (`semio_framework_tool_machine::node_drag_emit`:
        /// tool `<app_id>#<verb>`, press `gesture`, ref minted from `authoring_seed`) landing in the owned child `slot` member
        /// `child_id` (design §12, §13.3): ONE composite group whose member edit carries the transaction, the child leaves
        /// plainly for a view without command authority, or nothing when no leaf lands.
        pub fn child_node_drag<S, M>(app_id: &str, verb: &str, authoring_seed: &str, gesture: &str, leaves: Vec<M>, slot: &str, child_id: &str) -> Self
        where
            M: protocol::SemanticMutation<S> + ::protocol::OpBinary + Clone + 'static,
        {
            match semio_framework_tool_machine::node_drag_emit(app_id, verb, authoring_seed, gesture, leaves) {""",
            )
        ],
        f"{FLOW}/🎭️modes/✏️edit/🛠️tools/✋️drag/🦀️.rs": [
            replace("use semio_framework::kernel::UiDirtyScope;\n", ""),
            replace(
                """pub fn flow_drag_tool(verb: &str, authoring_seed: &str, base: &SemioFlowSnapshot, prepared: Vec<SemioFlowMutation>, records: &[NodeDragRecord]) -> NodeDragEmit<SemioFlowMutation> {
    let gesture = records.first().map_or(verb, |record| record.gesture_id.as_str());
    node_drag_emit(FLOW_EDITOR_APP_ID, verb, authoring_seed, gesture, prepared.into_iter().chain(flow_drag_leaves(base, records).into_iter().map(SemioFlowMutation::DragNodes)).collect())
}

/// 🧾️ The emit one release publishes on the `content` child `child_id` ([`Emit::node_drag_child`], design §12): ONE
/// composite group whose member edit carries the transaction and is labelled from its leaves, the leaves plainly for a
/// view without command authority, or the empty emit when nothing landed.
pub fn flow_drag_tool_emit(child_id: &str, verb: &str, authoring_seed: &str, base: &SemioFlowSnapshot, prepared: Vec<SemioFlowMutation>, records: &[NodeDragRecord]) -> Emit<FlowMutation, NoConfigMutation> {
    match flow_drag_tool(verb, authoring_seed, base, prepared, records) {
        NodeDragEmit::Nothing => Emit::default(),
        drag => Emit { ui_scope: UiDirtyScope::Full, ..Emit::node_drag_child::<SemioFlowSnapshot, _>(drag, "content", child_id) },
    }
}""",
                """pub fn flow_drag_tool(verb: &str, authoring_seed: &str, base: &SemioFlowSnapshot, prepared: Vec<SemioFlowMutation>, records: &[NodeDragRecord]) -> NodeDragEmit<SemioFlowMutation> {
    let (gesture, leaves) = flow_drag_release(verb, base, prepared, records);
    node_drag_emit(FLOW_EDITOR_APP_ID, verb, authoring_seed, gesture, leaves)
}

/// 🧾️ The emit one release publishes on the `content` child `child_id` ([`Emit::child_node_drag`], design §12): ONE
/// composite group whose member edit carries the transaction and is labelled from its leaves, the leaves plainly for a
/// view without command authority, or the empty emit when nothing landed.
pub fn flow_drag_tool_emit(child_id: &str, verb: &str, authoring_seed: &str, base: &SemioFlowSnapshot, prepared: Vec<SemioFlowMutation>, records: &[NodeDragRecord]) -> Emit<FlowMutation, NoConfigMutation> {
    let (gesture, leaves) = flow_drag_release(verb, base, prepared, records);
    Emit::child_node_drag::<SemioFlowSnapshot, _>(FLOW_EDITOR_APP_ID, verb, authoring_seed, gesture, leaves, "content", child_id)
}

/// 🧺️ What one release drags: the press the first record names (else `verb`), and `prepared` followed by the `drag-nodes`
/// leaves `records` mean on `base`.
fn flow_drag_release<'a>(verb: &'a str, base: &SemioFlowSnapshot, prepared: Vec<SemioFlowMutation>, records: &'a [NodeDragRecord]) -> (&'a str, Vec<SemioFlowMutation>) {
    (records.first().map_or(verb, |record| record.gesture_id.as_str()), prepared.into_iter().chain(flow_drag_leaves(base, records).into_iter().map(SemioFlowMutation::DragNodes)).collect())
}""",
            ),
        ],
        f"{DAG}/🎮️commands/✏️node-graph-edit/🦀️.rs": [
            replace("use semio_framework_tool_machine::{node_drag_emit, node_graph_edit_rows, ", "use semio_framework_tool_machine::{node_graph_edit_rows, "),
            replace(
                """    let drag = node_drag_emit(DAG_PLAY_APP_ID, verb, doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str()), gesture, leaves);
    Emit::node_drag_child::<semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot, _>(drag, "content", &doc.snapshot.content.child_id)""",
                """    let authoring_seed = doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str());
    Emit::child_node_drag::<semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot, _>(DAG_PLAY_APP_ID, verb, authoring_seed, gesture, leaves, "content", &doc.snapshot.content.child_id)""",
            ),
        ],
        f"{WIRES}/🦀️.rs": [
            replace("use semio_framework_tool_machine::node_drag_emit;\n", ""),
            replace(
                "    Emit::node_drag_child::<crate::SemioGraphSnapshot, _>(node_drag_emit(WIRES_PLAY_APP_ID, WIRES_NODE_DRAG_VERB, &operation.authoring_seed, gesture, vec![leaf]), crate::WIRES_CONTENT_SLOT, &snapshot.content.child_id)",
                "    Emit::child_node_drag::<crate::SemioGraphSnapshot, _>(WIRES_PLAY_APP_ID, WIRES_NODE_DRAG_VERB, &operation.authoring_seed, gesture, vec![leaf], crate::WIRES_CONTENT_SLOT, &snapshot.content.child_id)",
            ),
        ],
        f"{SEQUENCE}/🦀️.rs": [
            replace("use semio_framework_tool_machine::{node_drag_emit, NodeDragEmit, NodeGraphEditRow};\n", "use semio_framework_tool_machine::NodeGraphEditRow;\n"),
            replace(
                """            Some(gesture) => match node_drag_emit(SEQUENCE_PLAY_APP_ID, node_graph_edit::NODE_GRAPH_EDIT_VERB, authoring_seed, &gesture, leaves) {
                NodeDragEmit::Nothing => Emit::default(),
                drag => Emit { ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Emit::node_drag_child::<SemioFlowSnapshot, _>(drag, "content", &snapshot.content.child_id) },
            },""",
                """            Some(gesture) => Emit::child_node_drag::<SemioFlowSnapshot, _>(SEQUENCE_PLAY_APP_ID, node_graph_edit::NODE_GRAPH_EDIT_VERB, authoring_seed, &gesture, leaves, "content", &snapshot.content.child_id),""",
            ),
        ],
        f"{SEQUENCE}/🎮️commands/🕸️node-graph/🦀️.rs": [
            replace("use semio_framework_tool_machine::{node_drag_emit, NodeDragRecord, NodeGraphEditRow};\n", "use semio_framework_tool_machine::{NodeDragRecord, NodeGraphEditRow};\n"),
            replace(
                """        let drag = node_drag_emit(SEQUENCE_PLAY_APP_ID, NODE_GRAPH_EDIT_VERB, authoring_seed, gesture, leaves);
        Ok(Emit { ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Emit::node_drag_child::<SemioFlowSnapshot, _>(drag, "content", &doc.snapshot.content.child_id) })""",
                """        Ok(Emit::child_node_drag::<SemioFlowSnapshot, _>(SEQUENCE_PLAY_APP_ID, NODE_GRAPH_EDIT_VERB, authoring_seed, gesture, leaves, "content", &doc.snapshot.content.child_id))""",
            ),
        ],
    },
}


def derive(group):
    """🧪️ Every file of `group` as (path, current text or None, staged text), all anchors checked against the tree."""
    staged = []
    for path, edit in GROUPS[group].items():
        file = REPO / path
        current = file.read_text(encoding="utf-8") if file.exists() else None
        if isinstance(edit, str):
            if current is not None and current != edit:
                raise SystemExit(f"{path} already exists with other content")
            staged.append((path, current, edit))
            continue
        if current is None:
            raise SystemExit(f"{path} is gone")
        text = current
        for apply in edit:
            text = apply(text)
        staged.append((path, current, text))
    return staged


def main():
    if "--land" in sys.argv:
        group = sys.argv[sys.argv.index("--land") + 1]
        for path, _, text in derive(group):
            target = REPO / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(text, encoding="utf-8")
            print(f"landed {path}")
        return
    for group in GROUPS:
        patch = []
        for path, current, text in derive(group):
            copy = STAGED / group / path
            copy.parent.mkdir(parents=True, exist_ok=True)
            copy.write_text(text, encoding="utf-8")
            patch += difflib.unified_diff((current or "").splitlines(keepends=True), text.splitlines(keepends=True), f"a/{path}" if current is not None else "/dev/null", f"b/{path}")
        (STAGED / f"{group}.patch").write_text("".join(patch), encoding="utf-8")
        print(f"staged {group}: {len(GROUPS[group])} files, {sum(1 for line in patch if line.startswith(('+', '-')) and not line.startswith(('+++', '---')))} changed lines -> {(STAGED / f'{group}.patch').relative_to(REPO)}")


if __name__ == "__main__":
    main()
