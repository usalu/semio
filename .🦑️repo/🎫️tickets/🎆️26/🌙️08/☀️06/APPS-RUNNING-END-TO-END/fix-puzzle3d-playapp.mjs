import { readFileSync, writeFileSync } from "node:fs";

const APP = readFileSync("/tmp/semio-puzzle-3d.txt", "utf8").trim();
const BACKUP =
  "/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/OS-EXCLUSIVE-STATE-AUTHORITY/backup-✏️s__🔌️plugins__🧩️puzzle__🎛️apps__🧊️3d__🦀️component.rs";

const current = readFileSync(APP, "utf8");
const backup = readFileSync(BACKUP, "utf8");

function extractRegion(src) {
  const start = src.indexOf("//#region 🔖️PlayApp");
  const end = src.indexOf("//#endregion 🔖️PlayApp");
  if (start < 0 || end < 0) throw new Error("PlayApp region missing");
  return { start, end: end + "//#endregion 🔖️PlayApp".length, body: src.slice(start, end + "//#endregion 🔖️PlayApp".length) };
}

const cur = extractRegion(current);
let body = extractRegion(backup).body;

// 1) Restore struct with interior-mutable session fields (shared via thread_local instance below).
const structBlock = `#[derive(Default)]
pub struct Puzzle3dPlayApp {
    precompute: std::cell::RefCell<Puzzle3dPrecomputeSession>,
    transform_drag_active: std::cell::RefCell<bool>,
    transform_base: std::cell::RefCell<Option<Puzzle3dFixture>>,
    transform_scratch: std::cell::RefCell<Option<Puzzle3dFixture>>,
    preview_seq: std::cell::RefCell<u64>,
    fill_display_memo: std::sync::Mutex<Option<FillDisplayMemo>>,
    geometry_cache: std::sync::Mutex<Option<(u64, (Value, Value))>>,
    document_sections_cache: std::sync::Mutex<Option<(u64, Vec<UiTreeSectionNode>)>>,
    mesh_registry: std::cell::RefCell<std::collections::BTreeMap<String, Value>>,
}
`;

body = body.replace(
  /pub #\[derive\(Default, Clone, Copy\)\]\nstruct Puzzle3dPlayApp;\n\nimpl Default for Puzzle3dPlayApp\n(?:    \}\n)?\}\n\nimpl Puzzle3dPlayApp\n        let \(_, instances, meshes\) = cache\.as_ref\(\)\.expect\("geometry cache populated"\);\n        \(instances\.clone\(\), meshes\.clone\(\)\)\n    \}/,
  `${structBlock}
thread_local! {
    static PUZZLE3D_PLAY_APP: Puzzle3dPlayApp = Puzzle3dPlayApp::default();
}

fn puzzle3d_play_app<R>(f: impl FnOnce(&Puzzle3dPlayApp) -> R) -> R {
    PUZZLE3D_PLAY_APP.with(f)
}

impl Puzzle3dPlayApp {
    fn geometry_jsons(&self, fixture: &Puzzle3dFixture) -> (Value, Value) {
        let fingerprint = main::fixture_geometry_fingerprint(fixture);
        let mut cache = self.geometry_cache.lock().expect("geometry cache");
        if cache.as_ref().is_none_or(|(fp, _)| *fp != fingerprint) {
            let (instances, meshes) = main::geometry_jsons(fixture);
            *cache = Some((fingerprint, (instances, meshes)));
        }
        let (_, instances, meshes) = {
            let (fp, pair) = cache.as_ref().expect("geometry cache populated");
            (*fp, pair.0.clone(), pair.1.clone())
        };
        let _ = instances; // placate if tuple shape differs — fixed below
        cache.as_ref().expect("geometry cache populated").1.clone()
    }`,
);

// Fix botched geometry_jsons if main::geometry_jsons API unknown — simplify to always recompute.
body = body.replace(
  /fn geometry_jsons\(&self, fixture: &Puzzle3dFixture\) -> \(Value, Value\) \{[\s\S]*?cache\.as_ref\(\)\.expect\("geometry cache populated"\)\.1\.clone\(\)\n    \}/,
  `fn geometry_jsons(&self, fixture: &Puzzle3dFixture) -> (Value, Value) {
        let fingerprint = main::fixture_geometry_fingerprint(fixture);
        let mut cache = self.geometry_cache.lock().expect("geometry cache");
        if cache.as_ref().is_none_or(|(fp, _)| *fp != fingerprint) {
            *cache = Some((fingerprint, main::fixture_geometry_jsons(fixture)));
        }
        cache.as_ref().expect("geometry cache populated").1.clone()
    }`,
);

// 2) Fix DocumentApp impl opening brace
body = body.replace("impl DocumentApp for Puzzle3dPlayApp\n\n", "impl DocumentApp for Puzzle3dPlayApp {\n\n");

// 3) Route associated-fn entrypoints through thread_local instance
body = body.replace(
  "Ok(Puzzle3dPlayApp::default().handle_action_impl(command.action_id(), command.args(), command.window_id(), doc, cfg.projection))",
  "Ok(puzzle3d_play_app(|app| app.handle_action_impl(command.action_id(), command.args(), command.window_id(), doc, cfg.projection)))",
);
body = body.replace(/let app = Puzzle3dPlayApp::default\(\);\n/g, "let mut out = None;\n        puzzle3d_play_app(|app| {\n");

// The above replace for render blocks is too naive. Do targeted replacements instead.
body = body.replace(
  `fn render(body_key: &str, doc: &DocumentView<'_, Puzzle3dPlayProjection>, cfg: &ConfigView<'_, Puzzle3dConfig>) -> UiNode {
        let app = Puzzle3dPlayApp::default();`,
  `fn render(body_key: &str, doc: &DocumentView<'_, Puzzle3dPlayProjection>, cfg: &ConfigView<'_, Puzzle3dConfig>) -> UiNode {
        puzzle3d_play_app(|app| {`,
);
// Need to close the closure before the end of render - find return paths. Safer: wrap whole body.
// Re-read approach - use a simpler pattern: replace Default() with thread_local borrow helper that returns owned clone? Can't clone RefCells easily.

writeFileSync("/tmp/semio-puzzle3d-playapp-partial.rs", body.slice(0, 2500));
console.log("partial head written; checking replacements...");
console.log("has struct fields", body.includes("precompute: std::cell::RefCell"));
console.log("has thread_local", body.includes("PUZZLE3D_PLAY_APP"));
console.log("has DocumentApp brace", /impl DocumentApp for Puzzle3dPlayApp \{/.test(body));
console.log("geometry_jsons", body.includes("fn geometry_jsons"));
console.log("default().handle", body.includes("Puzzle3dPlayApp::default().handle_action_impl"));
console.log("puzzle3d_play_app handle", body.includes("puzzle3d_play_app(|app| app.handle_action_impl"));
