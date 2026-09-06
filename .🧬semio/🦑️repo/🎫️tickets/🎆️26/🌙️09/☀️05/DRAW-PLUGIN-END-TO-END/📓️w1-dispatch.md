# W1 — draw dispatch (PUBLICATION_CONTRACTS + 20-action migration + example ids + TS oracle)

Lane W1 of ticket `26/09/05/DRAW-PLUGIN-END-TO-END`, started 2026-09-06.
Spec: `📓️explore-editor-dispatch.md` §1-§3; recipe `S-END-TO-END/📓️explore-action-migration-recipe.md`;
precedents `BLOCK-PLUGIN-END-TO-END/📓️w1-block2d-factory.md` (block2d), `🏭️process/…/🧊️process3d` (two
factories in one app), `🌀️procedural/…/🌀️generation2d` (compact bounded factory + config preparation),
`🌀️procedural/…/🧊️generation3d` (generic diff/inverse preparation).

Status: **implementation in progress** (design settled, edits being applied).

## 1. Framework facts established before editing (all read from current source)

| fact | anchor |
|---|---|
| `ArtifactToolFactoryRegistry::register` rejects an empty/partial `PUBLICATION_CONTRACTS` with `interactive-job.publication-contract`; `HostOnly` must be the sole lane of its tool | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12906-12912` |
| An `Artifact`/`Config` lane with **no** `build_{artifact,config}_store_one_item_preparation_factory` is recorded in `unsupported_publication_contracts` at construction and every dispatch of that tool then faults `interactive-job.publication-authority-missing` | `🦀️.rs:19560-19578`, `:19403`, `:19419` |
| Only `emit.artifact_mutations` / `config_mutations` / `draft_mutations` / presence / transient / `child_emits` are lane-checked at commit — `emit.effects` are **not**, so an effects-only handler is honestly `HostOnly` | `🦀️.rs:22942-22949` |
| `expected = TOOL_JOB_IDS ∩ Migrated` and `seen == expected` exactly: a proof row for a non-migrated id faults `catalog-authority`, a migrated id with no row faults `catalog-incomplete` | `🦀️.rs:12244-12296` |
| The framework auto-injects `setActiveUtility` **already classified `Migrated`** (`ActionDefinition::resumable_framework_catalog` sets it) whenever `.utility(..)` rows exist, and registers **no** factory for it | `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:1023-1027,1247-1250`; `🔌️plugin/🦀️.rs:5442`, `:14664-14687` |
| ⇒ draw's `setActiveUtility` is a generated + migrated id **today** with no proof row, so the app already fails `interactive-job.catalog-incomplete` at construction even before the other 19 | derived from the two rows above |
| Two owned factories per app are supported and already precedented — `bounded_first_step_tool_proofs!` is invoked once per factory inside a helper `struct`+`impl`, and the trait method concatenates them | `🏭️process/…/🧊️process3d/…/✏️editor/🦀️.rs:1249-1310,1354-1356`; `🌊️flow/…/✏️editor/🦀️.rs:1837-1840` |

## 2. Lane table — read off each handler's real `Emit` construction

Gesture factory (`DrawingGestureOperationJobFactory`, 6 ids). Route: `DrawingInstanceOperationOwner::dispatch`
(`✏️editor/🦀️.rs:222-374`).

| tool id | evidence | lanes |
|---|---|---|
| `canvasPointerDown` | `canvas_pointer_down::handle` → `commit_with_utility_reset` (`Emit::commit`, artifact) at `🖱️canvas-pointer-down/🦀️.rs:213`; `advance_trace_pointer` pushes `config_mutations` (`:1063,1067`) | `Artifact, Config` |
| `canvasPointerMove` | idle → point-query, `Emit::default()` + effect only; otherwise `canvas_pointer_move::handle` → `step_gesture` → `commit_with_utility_reset` | `Artifact` |
| `canvasPointerUp` | `step_gesture_retained(PointerUp)` → `commit_with_utility_reset` | `Artifact` |
| `canvasDoubleClick` | `step_gesture_retained(CommitDraft)` → `DrawingDraftQuery::advance` → `commit_with_utility_reset` (`:867-870`) | `Artifact` |
| `canvasCommitDraft` | same draft-query commit path | `Artifact` |
| `canvasEscape` | `canvas_escape::handle` → `step_gesture` (artifact commit possible) **plus** `emit.config_mutations.push(SetTracePointerProgress)` (`🚪️canvas-escape/🦀️.rs:21`) | `Artifact, Config` |

Bounded factory (`DrawingBoundedCommandJobFactory`, 20 ids). Route: `command.dispatch` through
`drawing_bounded_reduce`.

| tool id | evidence (`✏️editor/🎮️commands/<dir>/🦀️.rs`) | lanes |
|---|---|---|
| `setSnapshot` | `Emit { effects: vec![drawing_reset_document_effect(..)] }` only | `HostOnly` |
| `commitDocument` | effects only | `HostOnly` |
| `setFixtureJson` | effects only / `Emit::default()` | `HostOnly` |
| `setActiveExample` | effects only / `Emit::default()` | `HostOnly` |
| `setSelectedOpacity` | `Emit::amend(operations, "opacity")` | `Artifact` |
| `engagementSubmit` | `Emit::mutations(vec![rename_layer(..)])` | `Artifact` |
| `addLayer` | `Emit { artifact_mutations: vec![create_layer(..)] }` | `Artifact` |
| `dropLayerKind` | `Emit { artifact_mutations: vec![create_layer(..)] }` | `Artifact` |
| `moveLayer` | `Emit::mutations(vec![reorder_layer(..)])` | `Artifact` |
| `deleteLayer` | `Emit { artifact_mutations: vec![delete_layer(..)] }` | `Artifact` |
| `duplicateLayer` | `Emit::mutations(vec![duplicate_layer(..)])` | `Artifact` |
| `toggleLayerVisible` | `Emit::mutations(vec![set_layer_visible(..)])` | `Artifact` |
| `combineBoolean` | `Emit { artifact_mutations: vec![create_layer(..)] }` | `Artifact` |
| `patchLayer` | `Emit::mutations(vec![operation])` | `Artifact` |
| `patchLayers` | `Emit::mutations(operations)` | `Artifact` |
| `setActiveUtility` | `Emit::config(vec![SetActiveUtility { .. }])` | `Config` |
| `setCamera` | `Emit::config(vec![SetCamera { .. }])` | `Config` |
| `setCameraZoom` | `Emit::config(vec![SetCamera { .. }])` | `Config` |
| `engagementInput` | `Emit::config(vec![SetEngagementInput { .. }])` | `Config` |
| `setLocale` | `Emit::config(vec![SetLocale { .. }])` | `Config` |

## 3. Pre-existing compile drift found (NOT W1's, reported for W2)

`impl ArtifactEditor for DrawingPlayApp` declares `async fn` for methods the trait declares **sync**:
`build_tool_job`, `app_schema`, `initial_snapshot`, `io`, `export_media`, `command_id`, `handle`,
`render`, `render_with_instance_operation_owner` (`✏️editor/🦀️.rs:1021,1047,1051,1055,1062,1082,1086,1102,1106`
vs `🔌️plugin/🦀️.rs:26647-26810` — `EditorApp<E>` calls e.g. `E::initial_snapshot()` with no `.await`
at `:27355`). W1 writes its new trait items with the **current sync** signatures and leaves the
existing ones untouched.

## Log
- reading done; framework contract, precedents and lane evidence settled (sections 1-2).
- Rust edits applied (gesture contracts, bounded factory, both store preparations, two proof catalogs,
  20 classifications, example-id resolution, law tests).
- TS publication-authority oracle written and **green**:
  `cd ✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript && bun ./📜️script.ts test` →
  `2 pass 0 fail` (example twins) + `validated Draw publication authority; apps=DrawingPlayApp:26; schema=Ajv; oracle=owned; hostile=5`.
- next: `cargo check -p semio-s-plugin-draw --lib` in the seeded private target dir.
