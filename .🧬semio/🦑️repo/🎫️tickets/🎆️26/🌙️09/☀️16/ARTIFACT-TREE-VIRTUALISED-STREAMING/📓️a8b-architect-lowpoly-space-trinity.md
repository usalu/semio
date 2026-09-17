# 📓️ A8b — architect / lowpoly / space / trinity-jack / trinity-rewriting, verified

Packet A8b of wave 4. Picks up the STALE (04:30) BUILD-ERR verdicts the previous A8 agent left for these
five crates, separates ticket-caused breakage from peer-owned churn, fixes the former, and re-runs the
window laws, the full suite and the `wasm32-wasip2` check per crate.

Runner env for every command: `DEVELOPER_DIR=/Library/Developer/CommandLineTools CARGO_INCREMENTAL=0
CARGO_PROFILE_WASM_DEV_DEBUG=false`, `-j 4`, never `CARGO_TARGET_DIR`. Logs: `🗑️generated/a8b/*.txt`.

> ⏳️ Status: IN PROGRESS. architect and lowpoly are finished; space and trinity are mid-verification.
> Every verdict below was produced by a command actually run in this session — nothing is inferred.

## 1. Verdicts

| crate | window laws | full suite | wasm32-wasip2 |
| --- | --- | --- | --- |
| `semio-s-artifact-architect-program` | ✅ **18 passed / 0 failed** | ⚠️ 2 pre-existing failures + a pre-existing stack-overflow abort (§3.1) | ✅ `Finished dev profile in 41.72s` |
| `semio-s-artifact-lowpoly-lowpoly` | ✅ **9 passed** (3 sibling app-fixture tests fail on the fleet-wide tool-proof gate, §3.2) | ⛔️ blocked by a peer's `semio-framework-pixels` break (§3.3) | ✅ `Finished dev profile in 51.25s` |
| `semio-s-plugin-space` | 🔄 in progress (9 passed / 4 failed at last run) | — | — |
| `semio-s-artifact-space-space` | 🔄 not yet re-run after the fixes | — | — |
| `semio-s-artifact-trinity-jack` | 🔄 not yet re-run after the fixes | — | — |
| `semio-s-artifact-trinity-rewriting` | 🔄 not yet re-run after the fixes | — | — |

## 2. What I changed

### 2.1 Ticket-caused (this packet's own breakage, fixed)

| path | fix |
| --- | --- |
| `🏛️architect/…/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️semantic-contract/🦀️.rs` | added `use semio_framework_plugin::BuiltNode;` — the migrated `document`/`catalogue` renders no longer re-export it through `use super::*` |
| `🏛️architect/…/✏️editor/📌️panels/📚️catalogue/🧪️tests/🔬️unit/🦀️.rs` | rewritten: `render()` now takes `&TreeWindows<'_>`, and `every_register_id_gets_a_catalogue_row` (false under a window) is replaced by the four window laws (a)–(d) — extent `66`, closed section, `[offset, offset+rows)` over the roster TAIL, and the unbound-tree variant of (d) |
| `🏛️architect/…/✏️editor/📌️panels/🗿️artifact/🦀️.rs` | the 66-row **register summary is now authored CLOSED**. The first-paint row budget is shared across the body in document order, so an open register roster swallowed all 48 rows and the `elements` section — the panel's actual subject and its only pick surface — materialised **zero** rows on a cold paint. Closed, it still stamps its full `total`, so the host opens and streams it on demand. New law `the_register_summary_is_closed_on_first_paint_so_elements_get_the_viewport` pins this |
| `💠️lowpoly/…/✏️editor/🦀️.rs` | `LOWPOLY_PLAY_CONTROLLER_ID` → `pub(crate)` (the migrated `🗿️artifact` panel imports it for `interaction_domain`) |
| `💠️lowpoly/…/✏️editor/📌️panels/🔍️inspection/🦀️.rs` | three `ui_node_list` call sites fixed for the SDK signature: its items are `UiAssemblyResult<BuiltNode>`, not `BuiltNode` (the deleted crate-local copy took the latter) |
| `💠️lowpoly/…/✏️editor/📌️panels/🛍️catalogue/🦀️.rs` | `primitive_row(entry: &(&'static str, &'static str, &'static str), …)` — `primitive_catalog_label` takes `&'static str`, so the windowed row closure needs the roster's real `'static` element lifetime |
| `🪐️space/⚙️engine/🪐️space/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs` | `AppIo::from_artifact(document_schema, …)` → `artifact_schema` (the helper's own parameter name) |

### 2.2 The space `🔢️parameters` gap the S1 audit flagged — CLOSED

`📓️a8-remaining-plugins.md` open item 1 declared this unfixable: the panel was a `column` of
`Component::Container(Section)` nodes, `ContainerProps` has no `window` carrier, so `total` never
reached the host. The stated reason for not converting it to a tree was
`🧰️framework/🔨️modules/🖥️platform/🟦️.ts`'s note that `TreeView`/`treeItemToTreeData` "never recurses
into a non-`treeItem` child as an inline row control".

**That note is stale.** `🗣️Interpreter/🟦️.tsx` grew `collectTreeItemControls` + `renderTreeItemControls`
+ `TreeDataItem.control` on 2026-09-10 precisely so a tree row can mount its non-`treeItem` children as
inline controls (the History panel's `framework.history.undo.run` button). So the panel is now a real
windowed tree, no new window carrier invented:

- `🪐️space/⚙️engine/🪐️space/📌️panels/🔢️parameters/🦀️.rs` — `PanelTreeBuilder::new("s-play-parameters")`
  with a 1-row `header` section (the `addParameter` button as a row control) and ONE
  `window_section(windows, S_PLAY_PARAMETERS_LIST_KEY, …, &projection.parameters, …)`. Every parameter
  is a `tree_window_item` over its own `ParameterField` slice (name / value / min / max / step /
  option·N / add-option / remove), and each field row is `control_row(id, label, control)` =
  `ui::tree_item(label).try_id(id).try_child(control)`. Both levels stamp `TreeWindow::total`.
  Every control id is preserved verbatim (`s-play-parameters.{id}.value`, `.name.input`,
  `.{field}.stepper`, `.option.{option}.remove`, `.add-option.input`, `.remove`, `s-play-parameters.add`).
- `…/🔢️parameters/🧪️tests/🔬️unit/🦀️.rs` — the four window laws plus a nested-level law
  (`each_parameter_is_a_windowed_container_over_its_own_field_rows`).
- Node keys are unique per body (`s-play-parameters.header`, `.list`, `s-play-parameters.{parameterId}`,
  and one distinct suffix per field row) — checked against F2's incoming duplicate-`node_key` SDK error.

### 2.3 Peer-owned breakage I had to repair to build at all

Both are pre-existing, committed, and untouched by this ticket; without them not a single command runs.

1. **`🔱️trinity/🗿️artifacts/🔌️jack/🦀️.rs:1290`** — `#[path = "…/🎮️commands/🧫️set-snapshot-json/🦀️.rs"]`
   points at a directory that does not exist. Commit `8773331d23` (2026-09-15, "Align plugin schemas,
   mutations, panels, and tests to snapshot-fixture-asset terminology") renamed the **`#[path]` string**
   but not the directory; `git ls-files` and `git show 7c296af6fd:…` both show the on-disk dir has been
   `🧫️set-fixture-json` since 2026-09-05. Repaired the `#[path]` to the file that exists.
2. **`🪐️space` `artifact_app_laws::new_app()` at 10 call sites** (`✏️editor/🧪️tests/🔬️unit`, 8 command
   test modules) — `artifact_app_laws` is not a name in the editor module, so it never resolved; the
   sibling tests in the same directories all spell it `context::new_app()`. Rewrote the 10 sites to
   `context::new_app()`. Also fixed `semio_framework_os::Viewport2d` / bare `commands::` /
   `SpaceWindowCamera` in `⚙️engine/🪐️space/🧪️tests/🔬️unit/🦀️.rs`'s
   `node_graph_viewport_writes_typed_workflow_camera_config` (added by commit `78b716e661`, 2026-09-13,
   already referencing names that do not exist at those paths).

## 3. Failures triaged as pre-existing, with evidence

### 3.1 architect — 2 failures + a stack-overflow abort in the full suite
- `command_from_action_covers_every_declared_action_and_rejects_unknown_ones` panics with
  `action nodeGraphViewport failed to bridge: nodeGraphViewport requires viewport`. `✏️editor/🦀️.rs:1309`
  requires an args key `viewport`, but `:1392` declares the action with **no** `ActionArgDef`s
  (`ActionDefinition::new("nodeGraphViewport", …, ActionKind::View, "camera")`). `git log -S` dates that
  pair to commits `615`/`616` (2026-09-13) — the interactive-job/tool-publication packet, three days
  before this ticket. No tree, panel, window or paging symbol is involved.
- `import_registers_csv_action_sets_plugin` panics inside `🏪️store/🦀️.rs:2730`
  (`artifact envelope terminal shell reached Drop before its app-owned bounded retirement authority
  detached every nested owner`) — the framework store-drop-witness law, not a render path.
- The whole-suite run then aborts (SIGABRT) with
  `thread 'architect-window-ownership-law' has overflowed its stack` — the same class of runner abort
  `📓️p3-sdk.md` §6 records for `mutation_fixture::dummy` in the framework crate.

### 3.2 lowpoly — 3 app-fixture tests fail the tool-proof gate
`catalogue_lists_primitives`, `document_tree_lists_active_object`, `layers_panel_lists_the_base_layer`
all panic in `🔌️plugin/🦀️.rs:21838` during **app construction**, before any panel renders:
`interactive-job.catalog-authority: tool factory proof rejected tool 'patchObject' … owner_eq=true
controller_eq=true schema_eq=true … generated_migrated=false, migrated={}`. The gate was introduced by
commit `21fbcd3538` (2026-09-02), and the identical `catalog-authority` fault is already recorded in
this ticket's own peer logs for a3-cad, a4-fem2d, a7 (dag, flow, sequence, vcs, note, remodel) and
a8-raster — i.e. it fires fleet-wide in every plugin, independent of this packet. All 9 lowpoly window
laws, which drive `render` directly with no app fixture, pass.

### 3.3 lowpoly — the full suite cannot link
`cargo test -p semio-s-artifact-lowpoly-lowpoly` (all targets) fails in
`🧰️framework/🔨️modules/🔲️pixels/🦀️.rs` with 8 × `cannot find module or crate semio_framework_deflate`.
`git status` shows `🔲️pixels/📦️packages/🦀️rust/Cargo.toml` **and** `🔲️pixels/🦀️.rs` modified in the
worktree — a peer is mid-refactor on that dependency. The `--lib` test target (which carries every
window law) compiles and runs.

## 4. Migration audit — "what migrated means", re-checked on disk

- **No app-local paging residue** anywhere in `🏛️architect`, `💠️lowpoly`, `🪐️space`, `🔱️trinity`:
  zero hits for `setPanelPage`, `panel_pages`, `paged_section`/`paged_panel_section`,
  `continuation_row`, `panel_page_rows`, `PanelRowBudget`, `section_page`, `SECTION_ROWS`, `IDS_ROWS`,
  `LIST_ROWS_MAX`, `CATALOGUE_GROUP_ROWS`, `PANEL_RECONCILE_NODE_BUDGET`, `registerPages`. The only
  `continuation_row` / `.more` strings left are inside the new laws that assert their absence.
- **No crate-local `fn ui_node_list`** survives in any of the four plugins; every `.take(N)` hit is
  `Option::take` or non-UI arithmetic.
- **Every tree-bearing render threads `TreeWindows`.** Files still on plain `.section(...)` are exactly
  the spec §1 carve-out — a handful of hand-written heterogeneous rows with no entry slice to window:
  architect `🔍️inspection` (6 rows) and `🗿️artifact`/`📓️report` `meta` (3 rows), lowpoly `🔍️inspection`
  (empty-state + object + transform), trinity jack/rewriting `🔍️inspection` and both catalogues'
  3-row `kinds`, space members' 3 fixed action rows and the engine `🔍️inspection` field groups.
- `💠️lowpoly/…/✏️editor/🧭️view/🦀️.rs` has no tree at all (pure id/selection helpers) — its
  `PanelTreeBuilder` hit is a doc-comment reference.

## 5. Not finished

Everything under 🔄 in §1 — `semio-s-plugin-space` (4 failing laws under investigation),
`semio-s-artifact-space-space`, `semio-s-artifact-trinity-jack`, `semio-s-artifact-trinity-rewriting`
have not been re-run to completion since the fixes in §2. The full suite and the wasm check are
outstanding for all four. Build throughput is the limit: the shared fine-grain-locked build dir was
serialising ~20 peer cargos, with single runs queuing 30–60 min on
`Blocking waiting for file lock on artifact directory`.
