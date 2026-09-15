# 🛂️ Wave W0-B: manifest declaration, action injection, projection, React progress-measure removal

Lane W0-B of `📋️tool-run-contract.md` (§2.4, §2.5, §3.5, §3.6, §5, §6). Status: **landed, verified**. The plugin-builder injection test passed after W0-D fixed its test file (§3.3).

Paths are relative to the repo root. `M` = `🧰️framework/🔨️modules/🛂️manifest`.

## 0. Layering check

- `🧅️layering.json` is a ratchet on how often **framework** files reference **implementation** areas (`areaLayers`: `✏️s`, `🌎️hub`, `♻️mit-bestand`). `🧰️framework` → `🧰️framework` references are not counted, so the manifest may depend on `⏯️tool-run`.
- The manifest is mounted only in crate `semio-framework` (`🧰️framework/📦️packages/🦀️rust`). That crate now depends on `semio-framework-tool-run`.
- No cycle: tool-run depends only on `os-kernel` and `ui`, and neither depends on `semio-framework`. Native and wasm32-wasip2 checks both compile.
- **Crate name:** the contract's `semio-framework-manifest` does not exist; the crate is `semio-framework`.

## 1. What changed

### Manifest crate (`M/🦀️.rs`)

- **Region `🔖️ToolRun` (`:1191-1242`):**
  - Explicit re-export of the tool-run types and constants (see §2).
  - `app_declares_tool_run` (`:1198`), `tool_run_action_definitions` (`:1208`) and the private `tool_run_action_arg` (`:1229`).
  - Mounts the test module `tool_run_actions_tests` (`:1241`).
- `UtilityDefinition.run` (`:1377`) and `ToolDefinition.run` (`:1568`), both `Option<ToolRunDefinition>` with `serde`/`value` `default` + `skip_serializing_if`. `new()` sets `run: None`.
- Every other construction site in the repo already uses `..X::new(..)`, so no plugin needed touching.

### Crate glue and scripts (`🧰️framework/📦️packages/🦀️rust/`)

- `Cargo.toml`: `semio-framework-tool-run = { workspace = true }`.
- `📜️script.ts`: new command `test-tool-run-actions`, which runs the bun ajv/TS test and then `cargo test --lib manifest::tool_run_actions_tests`.
- `📋️project.json`: target `test-tool-run-actions`.

### Schema (`M/🧬️schema/🔣️.json`, insertions only)

New `$defs`:
- `ToolRunDefinition`: a `$ref` to `https://json.schemas.assets.semio-tech.com/framework/tool-run/schema.json#/$defs/ToolRunDefinition`.
- `ToolDefinition` and `UtilityDefinition`: their wire shape, with `label` as a `$ref` to tool-run `LocalizedLabel` and `run` as a `$ref` to `ToolRunDefinition`.
- `ToolRunActionsFixture`.

### Fixture (new)

`M/🧫️fixtures/⏯️tool-run-actions.json` is language-agnostic. It holds:
- the 7 action rows: id, icon, keys or `panelChord`, EN/DE label, and args (id, schema, required, EN/DE label);
- the framework chords that are already reserved;
- 4 injection cases: no tools, a tool without `run`, a tool with `run`, a utility with `run`.

### Tests (new)

- `M/🧪️tests/🔬️tool-run-actions/🦀️.rs`, 4 Rust tests:
  - injection happens exactly when `run` is declared;
  - the fixture rows equal the output, and the output equals `ToolRunAction` ids, args, chords and labels;
  - **chord law**;
  - `ToolDefinition`/`UtilityDefinition` with `run` round-trip through serde_json and the first-party value codec, and the definition passes `validate()`.
- `M/🧪️tests/🔬️tool-run-actions/🟦️.ts`, 4 bun tests. The oracle is **ajv** (strict), which validates:
  - the fixture itself;
  - hostile `run` declarations through the cross-schema `$ref`: empty stages, unknown trace kind, reserved reason code 0xFF00, extra field, misspelled `runs`;
  - hostile fixture rows (an unknown action id, and dismiss carrying both `keys` and `panelChord`).
  It also checks `appDeclaresToolRun` and that the fixture equals the tool-run TS `TOOL_RUN_ACTIONS`/`TOOL_RUN_LABELS`.
- `M/🧪️tests/🔬️app-label/🦀️.rs`:
  - `app_with` is now `pub(super)` so it can be reused;
  - the typegen row count goes 186 → 194 (`:1477`).

### Manifest TS mirror (`M/🟦️.ts`)

- Removed the `MeasureProgressStep`/`MeasureProgressStepKind` imports and re-exports.
- Added generated re-exports: `ToolRunDefinition`, `ToolRunStageDefinition`, `ToolRunCounterDefinition`, `ToolRunReasonDefinition`, `ToolRunRebasePolicy`, `ToolRunReconfigurePolicy`, `ToolRunTraceKind`, `ToolRunVerdict`, `JobKindId` (`:602-612`).
- Added an explicit re-export from `../⏯️tool-run/🟦️.ts` of the action ids, the chords, `TOOL_RUN_ACTIONS` and `ToolRunActionId`.
- Added `appDeclaresToolRun` (`:634`).

### Projection (`🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs`)

- **Removed** the rows `MeasureProgressStep` and `MeasureProgressStepKind`, and the `{ "kind": "progress", … }` arm of `WindowMeasure`.
- **Added rows:** `JobKindId` (`:869`), `ToolRunCounterDefinition`, `ToolRunDefinition`, `ToolRunReasonDefinition`, `ToolRunRebasePolicy`, `ToolRunReconfigurePolicy`, `ToolRunStageDefinition`, `ToolRunTraceKind`, `ToolRunVerdict` (`:1201-1263`), and `UiProgressNode` (`:1641`).
- `ToolDefinition` (`:1170`) and `UtilityDefinition` (`:1723`) gain `run?: ToolRunDefinition`.
- Regenerated `M/🤖️generated/🪪️manifest/🟦️.ts` with `bun nx run @semio-tech/framework-rs:generate`. The repo auto-commit already picked the output up, so it shows no diff against HEAD. It contains `ToolRunDefinition` and `UiProgressNode` and no `MeasureProgressStep`.

### React progress-measure removal

- `…/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx`: removed the `WindowMeasureProgress` import, `windowMeasureProgressControl`, and both `measure.kind === "progress"` arms (tree items and `renderWindowMeasure`).
- `…/🛠️ShellHelpers/🎚️measure-controls/🟦️.tsx`:
  - removed `WindowMeasureProgress`, `MEASURE_PROGRESS_STEP_CLASS`, `MEASURE_PROGRESS_STEPS_SHOWN` and `MEASURE_PROGRESS_INDETERMINATE_SHARE`;
  - removed the imports of the `MeasureProgressStep*` types and `useLabel`;
  - updated the header docstring.
- `…/🧑‍🎨engine/🧪️tests/🎚️window-measure-controls/🟦️.tsx`: removed the Progress fixture, the `cancelAction` and the whole `⏳️ the progress measure` describe block.

## 2. Public API as landed

```rust
// semio_framework (manifest), re-exported at the crate root
pub use semio_framework_tool_run::{JobKindId, ToolRunAction, ToolRunCounterDefinition, ToolRunDefinition, ToolRunDefinitionError,
    ToolRunReasonDefinition, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunStageDefinition, ToolRunTraceKind, ToolRunVerdict,
    TOOL_RUN_ABORT_ACTION_ID, TOOL_RUN_ACTION_IDS, TOOL_RUN_DISMISS_ACTION_ID, TOOL_RUN_DISMISS_CHORD, TOOL_RUN_FINALIZE_ACTION_ID,
    TOOL_RUN_PAUSE_ACTION_ID, TOOL_RUN_RESUME_ACTION_ID, TOOL_RUN_START_ACTION_ID, TOOL_RUN_STEP_ACTION_ID};
pub struct ToolDefinition { pub id, pub label, pub icon_id, pub keys: Option<String>, pub run: Option<ToolRunDefinition> }
pub struct UtilityDefinition { …, pub allows_actions_while_active: bool, pub run: Option<ToolRunDefinition> }
pub fn app_declares_tool_run(app: &AppDefinition) -> bool;
pub fn tool_run_action_definitions(app: &AppDefinition) -> Vec<ActionDefinition>; // [] unless a tool or utility declares run
```

```ts
export type ToolRunDefinition /* … */; export function appDeclaresToolRun(app): boolean;
export { TOOL_RUN_*_ACTION_ID, TOOL_RUN_*_CHORD, TOOL_RUN_ACTION_IDS, TOOL_RUN_ACTIONS, type ToolRunActionId } from "../⏯️tool-run/🟦️.ts";
```

### Injected actions

The actions come out in `ToolRunAction::ALL` order.

**Shared by all seven:**
- `ActionKind::History`, built with `resumable_framework` (so `interactive_job = Migrated`).
- `in_palette: false`.
- Labels are `ToolRunAction::label().localized()`.
- Args are `presentation: Hidden`. `toolId`/`windowId` are text; `runId`/`generation` are integers with `min 0, step 1`. Arg labels: Tool/Werkzeug, Window/Fenster, Run/Lauf, Generation/Generation.

| Id | Icon | keys |
|---|---|---|
| `toolRunStart` | play | `mod+enter` |
| `toolRunPause` | pause | `mod+alt+enter` |
| `toolRunResume` | play | `mod+alt+enter` |
| `toolRunStep` | skip-forward | `mod+alt+arrowright` |
| `toolRunAbort` | square | `mod+.` |
| `toolRunFinalize` | check | `mod+shift+enter` |
| `toolRunDismiss` | x | none |

**Chord law** (as tested):
- Every action except dismiss has `keys == ToolRunAction::chord()`. Dismiss has no app-wide `keys`, because its `escape` chord applies with panel focus only.
- Pause and resume share one chord, and the five distinct chords are the only ones.
- None of them collides with the other framework-injected chords: `mod+z`, `mod+shift+z`, `mod+c`, `mod+x`, `mod+v`, `mod+a`, `escape`.
- No shifted punctuation, and chords are lower-case.

## 3. Tests run (foreground; logs in `T/🗑️generated/W0-B/`)

### 3.1 Passing

| Command | Result |
|---|---|
| `RUST_MIN_STACK=134217728 cargo test -p semio-framework --lib tool_run` | 4/4 pass (`manifest-test-1.txt`) |
| `bun test ./🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️tool-run-actions/🟦️.ts` | 4 pass, 37 expects (`manifest-ts-test-2.txt`). The first run failed on an ajv strictRequired error in the fixture schema; fixed |
| `bun nx run @semio-tech/framework-rs:test-tool-run-actions --skip-nx-cache` | bun 4/4, then nextest 4/4 (`nx-test-tool-run-actions.txt`) |
| `bun nx run @semio-tech/framework-rs:check` | See the red → green sequence below |
| `cargo check -p semio-framework` and `--target wasm32-wasip2` | Both Finished. A temporary `[DEBUG]` probe function made `semio-framework` emit warnings on both targets, which proves type-checking reached the crate. The probe was removed (`check-framework-probe-*.txt`) |
| `bun ./📜️script.ts test long "🎚️window-measure-controls"` (renderer react package) | 7/7 (`react-window-measure-controls.txt`) |
| `bun ./📜️script.ts test long "🛠️ShellHelpers"` | 9/9 (`react-shellhelpers.txt`) |
| `bun ./📜️script.ts typecheck` (renderer react package) | 790 errors, all pre-existing; W0-C reported 791. Neither of the two errors in touched files (`import.meta.dir` at manifest `:1228`, `Uint8Array` BlobPart at ShellHelpers `:627`) is in a changed line (`react-typecheck.txt`) |

**Schema pipeline (framework-rs check):**
1. `check` before regeneration: red, stale mirror (`framework-rs-check-red.txt`).
2. `bun nx run @semio-tech/framework-rs:generate`: ok.
3. `check` again: `exports_typescript_bindings` 1/1, "mirror is fresh" (`framework-rs-check.txt`).

`bun nx show project @semio-tech/framework-rs` confirms that the `generate` and `check` targets exist.

**Mutation check.** Two deliberate breaks were applied together: utility injection keyed on `run.is_none()`, and resume losing its keys instead of dismiss. 3 of the 4 Rust tests failed. The file was restored and verified byte-identical with `cmp` (`mutation-test.txt`).

### 3.2 Failing, not caused by this lane

**`cargo test -p semio-framework --lib`: 241 pass, 12 fail** (`framework-lib-test.txt`). The failures do not involve any type this lane touched:
- 10 `kernel::ui_turn_patch_tests` retirement/transport assertions (the retirement problem W0-C already reported);
- `action_bus::…restart_checkpoint`;
- `app_label_tests::tutorial_artifact_event_kind_round_trips_tagged_camel_case`, a DslValue object key-order mismatch.

**`cargo check -p semio-framework -p semio-framework-plugin -p semio-framework-os-mcp --tests`** (`check-native.txt`):
- The plugin lib test reached 248 warnings with no errors.
- `semio-framework-os-mcp` has peer errors: `DirectorySessionAuthorityV1.expires_at_ms`, `ArtifactStore::undo` bounds, and `crate::note_descriptor` in registry tests.
- None of the errors is in `🗂️catalog/🦀️.rs`, which holds this lane's one-line edit. Type checking did run over the crate (errors are reported from several functions), so the edit is checked only indirectly.

**`bun nx run workspace:verify -- interactivity`: exit 1** (`verify-interactivity.txt`). None of the findings mention tool definitions, `run`, actions, chords or measures:
- `.vscode/launch.json` / `launch.seed.jsonc` exceed the fixed capacity of 512 configurations, and 6 gate registrations are missing;
- 757+ all-app descriptor discovery failures;
- the puzzle fill envelope source predicates ("baseline falsely rejected", W1 scope) throw.

### 3.3 Plugin-builder injection test

**Command:** `RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib -- declaring_a_tool_run no_tools_means_no_set_active_tool declaring_tools_injects`

**Status:** **3/3 pass** (`plugin-builder-test-2.txt`), so the P injection edit is confirmed at runtime.

**Earlier attempt:** W0-D's in-flight `🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs` does not compile yet: one E0599 (`FaultCode::as_str`) and 12 E0502 borrow errors (`plugin-builder-test.txt`). Before that file existed, the build failed on its missing mount.

**What the test checks:**
- no tool-run actions are injected without `run`;
- all 7 are injected into windows with `run`;
- keybindings `mod+enter`→Start, `mod+alt+enter`→Pause, `mod+alt+arrowright`→Step, `mod+.`→Abort, `mod+shift+enter`→Finalize;
- Dismiss and Resume are not bound;
- `escape` is never bound to Dismiss.

## 4. Commands to register in launch.json

- `bun nx run @semio-tech/framework-rs:test-tool-run-actions` (new): ajv/TS fixture test, then the manifest injection and chord-law Rust tests.
- `bun nx run @semio-tech/framework-rs:generate` and `bun nx run @semio-tech/framework-rs:check` (both already exist; `check` is not yet registered in launch.json).
- The renderer react package target `test-long` with args `🎚️window-measure-controls` (vitest suite).

## 5. Deviations from the contract, with reasons

1. **Crate name** is `semio-framework`, not `semio-framework-manifest`, which does not exist.
2. **`tool_run_action_definitions` takes `&AppDefinition`**, following `interaction_action_definitions`. The contract wrote `()`, but the actions have to be conditional on `run` being declared.
3. **ActionKind `History`**, for three reasons:
   - it matches "routed like `undo`";
   - it matches P's `framework_reserved_action_kind` default, which returns History for reserved ids;
   - History actions are already excluded from the Actions panel, their `keys` are bound automatically by the builder, and the MCP catalog already treats them as framework-injected.
4. **`toolRunDismiss` has no `keys`.** As an app-wide binding, `escape` would take over `clearSelection`. The fixture records `panelChord: "escape"` instead, and the chord law pins it against `ToolRunAction::chord()`.
5. **Pause and resume both declare `mod+alt+enter`.** The builder's key dedupe binds it to `toolRunPause`; W0-D's router must toggle by state.
6. **"Legacy `UiNode` TS union":** the projection has no `UiNode` union, only the leaf `Ui*Node` rows. The missing piece was therefore added as a `UiProgressNode` row next to the other leaf nodes.
7. **Labels are `unknown`** in the projected `ToolRun*Definition` types, following the projection's existing `LocalizedLabel` convention. The precise shape stays in the JSON schema via `$ref`.

## 6. Foreign edits

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5649` (P, owned by W0-D):
  - the interaction injection loop now iterates `interaction_action_definitions(&definition).into_iter().chain(tool_run_action_definitions(&definition))`;
  - this is the auto-injection itself: window actions, plus a keybinding for each `keys` that is not already bound;
  - interaction actions come first, so `escape` stays with `clearSelection`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗂️catalog/🦀️.rs:349`: `actions.extend(manifest::tool_run_action_definitions(app));` in `framework_capabilities`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-app-builder/🦀️.rs:339`: the new builder test from §3.3.
- `🧰️framework/📦️packages/🦀️rust/{Cargo.toml,📜️script.ts,📋️project.json}`: the manifest crate's own glue.

## 7. Open items

- **W0-D:**
  - add `TOOL_RUN_ACTION_IDS` to `is_framework_reserved_action_id`, and keep `framework_reserved_action_kind` returning `History` for them;
  - route the `mod+alt+enter` toggle (the binding always targets `toolRunPause`);
  - bind `escape` → `toolRunDismiss` with panel focus only;
- **Coordinator:** triage the 12 pre-existing `semio-framework` lib failures and the peer `semio-framework-os-mcp` compile errors. `verify interactivity` is red for reasons outside this lane.
- **W1-B and later:** plugins declare `ToolDefinition { run: Some(..), ..ToolDefinition::new(..).await }`. Each plugin's own `mod+.`/`mod+enter` bindings win over the injected chords until that plugin migrates, because declared keybindings are bound first.
