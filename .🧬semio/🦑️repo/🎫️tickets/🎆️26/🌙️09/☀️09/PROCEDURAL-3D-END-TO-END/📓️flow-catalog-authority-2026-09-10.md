# Flow `interactive-job.catalog-authority` — Migrating the Flow Artifact's Last 13 Actions

Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`, lane: flow artifact editor interactive-job migration.
2026-09-10. Repo MCP down all session (`invalid initialize params`); ticket bookkeeping is on disk
and no ticket was opened/closed/reopened.

Answers `📓️runtime-verification-2026-09-09.md` wgpu boot #1 (the flow plugin panics on its first step)
and the owed item `📓️catalogue-surface-2026-09-09.md` flagged as "the flow artifact's 13
`BatchOnlyPendingRewrite` classifications".

Follows `📓️extension-continuation-2026-09-09.md` §3 (generation3d's retained-route pattern),
`📓️work-capacity-2026-09-10.md` (the unified `ArtifactRetainedWorkCapacity` declaration) and
puzzle3d's `📓️interactive-job-migration-recipe.md`.

> Status: DONE.

---

## 1. TL;DR

`FlowPlayApp` could not be constructed on any host that instantiates the flow editor. The framework's
`AppActionRegistry::validate_tool_job_rows`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12035-12113`) requires the aggregated
bounded-first-step proof set to EQUAL the set of generated ids the manifest declares `Migrated`. The
flow editor satisfied neither half:

1. **13 actions were still declared `BatchOnlyPendingRewrite`** — so `expected` (generated ∩ migrated)
   excluded them and the very first proof row for `addGeneration` was rejected outright
   (`generated_migrated=false`), which is exactly boot #1's fault line.
2. **6 of the 34 declared `FlowCommand` rows had no retained route at all** — `connectMediaPorts`,
   `reorganize`, `renameFlowWidget`, `nodeGraphEdit`, `spotlightCommit`, `runExtensionAction` only
   ever reached the batch `FlowPlayApp::handle`.
3. **2 registered routes had no proof row** — `duplicateWidget` and `focusSelection` were in
   `FLOW_DIRECT_STORE_TOOL_IDS` (so they registered, and their publication contracts existed) but
   `FlowDirectStoreJobFactoryProofs` listed only 19 of the factory's 21 tools, so `seen != expected`
   would have raised `interactive-job.catalog-incomplete` even after (1).

All three are closed. Flow now declares **34 migrated actions, 34 proof rows, 4 factories that
partition the 34 tool ids exactly, 0 `BatchOnlyPendingRewrite`**, and the batch `handle` fallback is
provably unreachable.

Measured, not inferred: `interactive-job.catalog-authority` occurrences in the flow lib suite fell
from **35 → 1**, and the one that remains is `two_instances_converge_on_disjoint_edits`, which builds
its pair through the framework's **registryless** `testkit::paired_apps` (`migrated={}` in the fault
detail — no registry, therefore no declarations); that is the known framework testkit gap, not a flow
declaration.

---

## 2. Before / after — every declared action

`FLOW_*_TOOL_IDS` route ownership and the manifest classification, side by side. "batch `handle`"
means the action was dispatched through `FlowPlayApp::handle`'s `command.dispatch(...)` fallback.

| # | action | before: route | before: classification | after: route | after: classification |
|---|---|---|---|---|---|
| 1 | `addWidget` | `FlowChildGroupJobFactory` | Migrated | unchanged | Migrated |
| 2 | `removeWidget` | `FlowDirectStoreJobFactory` | Migrated | unchanged | Migrated |
| 3 | `duplicateWidget` | `FlowDirectStoreJobFactory` (**no proof row**) | **BatchOnlyPendingRewrite** | `FlowDirectStoreJobFactory` + proof | **Migrated** |
| 4 | `deleteSelection` | `FlowDirectStoreJobFactory` | Migrated | unchanged | Migrated |
| 5 | `disconnect` | `FlowDirectStoreJobFactory` | Migrated | unchanged | Migrated |
| 6 | `connectMediaPorts` | **batch `handle`** | **BatchOnlyPendingRewrite** | **`FlowGraphOperationJobFactory`** | **Migrated** |
| 7 | `moveMediaNode` | `FlowDirectStoreJobFactory` | Migrated | unchanged | Migrated |
| 8 | `reorganize` | **batch `handle`** | **BatchOnlyPendingRewrite** | **`FlowGraphOperationJobFactory`** | **Migrated** |
| 9 | `patchFlowWidgets` | `FlowDirectStoreJobFactory` | Migrated | unchanged | Migrated |
| 10 | `renameFlowWidget` | **batch `handle`** | **BatchOnlyPendingRewrite** | **`FlowGraphOperationJobFactory`** | **Migrated** |
| 11 | `nodeGraphEdit` | **batch `handle`** (`apply`, interaction-aware) | **BatchOnlyPendingRewrite** | **`FlowGraphOperationJobFactory`** | **Migrated** |
| 12 | `spotlightCommit` | **batch `handle`** (`apply`, interaction-aware) | **BatchOnlyPendingRewrite** | **`FlowGraphOperationJobFactory`** | **Migrated** |
| 13 | `runExtensionAction` | **batch `handle`** | **BatchOnlyPendingRewrite** | **`FlowGraphOperationJobFactory`** | **Migrated** |
| 14 | `evaluate` | `FlowHostEffectJobFactory` | Migrated | unchanged | Migrated |
| 15 | `focusSelection` | `FlowDirectStoreJobFactory` (**no proof row**) | **BatchOnlyPendingRewrite** | `FlowDirectStoreJobFactory` + proof | **Migrated** |
| 16 | `nodeGraphViewport` | `FlowDirectStoreJobFactory` | Migrated | unchanged | Migrated |
| 17 | `setLodMode` | `FlowDirectStoreJobFactory` | Migrated | unchanged | Migrated |
| 18 | `setProximityDistance` | `FlowDirectStoreJobFactory` | Migrated | unchanged | Migrated |
| 19 | `setGridVisible` | `FlowDirectStoreJobFactory` | Migrated | unchanged | Migrated |
| 20 | `setGridSnapEnabled` | `FlowDirectStoreJobFactory` | Migrated | unchanged | Migrated |
| 21 | `setGridFactor` | `FlowDirectStoreJobFactory` | Migrated | unchanged | Migrated |
| 22 | `contextMenuAt` | `FlowHostEffectJobFactory` | Migrated | unchanged | Migrated |
| 23 | `setPreviewOff` | `FlowDirectStoreJobFactory` | Migrated | unchanged | Migrated |
| 24 | `openSpotlight` | `FlowHostEffectJobFactory` | Migrated | unchanged | Migrated |
| 25 | `replaceImage` | `FlowHostEffectJobFactory` | Migrated | unchanged | Migrated |
| 26 | `setCatalogueSections` | `FlowDirectStoreJobFactory` | Migrated | unchanged | Migrated |
| 27 | `toggleExtension` | `FlowDirectStoreJobFactory` | Migrated | unchanged | Migrated |
| 28 | `addGeneration` | `FlowDirectStoreJobFactory` | **BatchOnlyPendingRewrite** | unchanged | **Migrated** |
| 29 | `removeGeneration` | `FlowDirectStoreJobFactory` | **BatchOnlyPendingRewrite** | unchanged | **Migrated** |
| 30 | `selectGeneration` | `FlowDirectStoreJobFactory` | **BatchOnlyPendingRewrite** | unchanged | **Migrated** |
| 31 | `renameGeneration` | `FlowDirectStoreJobFactory` | **BatchOnlyPendingRewrite** | unchanged | **Migrated** |
| 32 | `updateGenerationValues` | `FlowDirectStoreJobFactory` | **BatchOnlyPendingRewrite** | unchanged | **Migrated** |
| 33 | `flowEvalTick` | `FlowHostEffectJobFactory` | Migrated | unchanged | Migrated |
| 34 | `flowEvalResolve` | `FlowHostEffectJobFactory` | Migrated | unchanged | Migrated |

**Migrated in this lane (13):** `duplicateWidget`, `connectMediaPorts`, `reorganize`,
`renameFlowWidget`, `nodeGraphEdit`, `spotlightCommit`, `runExtensionAction`, `focusSelection`,
`addGeneration`, `removeGeneration`, `selectGeneration`, `renameGeneration`, `updateGenerationValues`.

Route totals: 21 (direct-Store) + 1 (child-group) + 6 (host-effect) + **6 (graph-operation, new)** =
**34**, and `<FlowPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len() == 34`.

---

## 3. The new retained route — `FlowGraphOperationJobFactory`

All six unrouted commands share one shape: they run a single `FlowHost` operation (or, for the
rename, a pure fixture rewrite) against the live scene and publish ONE batched artifact-lane edit.
They are therefore one route family, not six, and they follow generation3d's pattern
(`📓️extension-continuation-2026-09-09.md` §3): an app-owned `ToolJobFactory` whose `Job` is the
framework's `ArtifactRetainedCommandJob`, driven by an `ArtifactCommandWork` implementation.

### 3.1 New code, by file:line

All in `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`:

| file:line | what |
|---|---|
| `:1783` | `FLOW_GRAPH_OPERATION_TOOL_IDS` — the six ids |
| `:1784` | `FLOW_GRAPH_OPERATION_RAW_BYTES = 16_384` |
| `:1788` | `FLOW_GRAPH_OPERATION_CAPACITY` = `ArtifactRetainedWorkCapacity::for_invertible_items(FLOW_STORE_MAX_SCENE_ITEMS + 1)` — the ONE capacity declaration (`📓️work-capacity-2026-09-10.md` §4) |
| `:1793` | `flow_graph_operation_references` — the `[Option<&str>; 2]` the survey resolves: `[source, target]` for `connectMediaPorts`, `[old, trimmed new]` for `renameFlowWidget`, `[None, None]` for the whole-graph routes |
| `:1803` | `flow_graph_operation_payload_admitted` — every payload bound in one place; a payload over any of them refuses at `extent` (`None`) instead of reaching the host |
| `:1818` | `struct FlowGraphOperationWork` — `tool_id`, retained `instance_owner`, `cursor`/`replay_target`, `resolved: [bool; 2]`, `completed`, `closing` |
| `:1876` | `extent` → `FLOW_GRAPH_OPERATION_CAPACITY.rows_for_items(scene.widgets.len() + 1)`, gated on scene widgets/synapses ≤ `FLOW_STORE_MAX_SCENE_ITEMS` and on the payload bounds |
| `:1893` | `step` — the survey walk (one widget per step, `Progress`/`Replay`), then the apply step, then `Complete(emit)` |
| `:1841` | `apply` — runs the operation on the app-instance `FlowEvalSession` via `ArtifactInstanceOperationOwnerHandle::with_mut` → `FlowInstanceOperationOwner::with_session`, exactly like `FlowChildGroupWork`/`FlowHostEffectJob` |
| `:1925` | `checkpoint`/`restore` — 9 bytes (`completed` + `cursor`); a restore replays the survey from 0 with the flags cleared, so the resolution is re-derived rather than trusted |
| `:1946` | `close_step`/`terminal_is_empty` — retires the retained instance-owner handle under its exact grant |
| `:1950` | `struct FlowGraphOperationJobFactory` + `ToolJobFactory` impl (`classification() == Migrated`, wire-bound rejection in `create_job_from_wire_pages_with_payload`) |
| `:1960` | `flow_graph_operation_contract()` — the ONE contract the factory returns AND the proofs prove |
| `:2003` | `PUBLICATION_CONTRACTS` — all six `[ArtifactToolPublicationLane::Artifact]` |
| `:2019` | `FlowGraphOperationJobFactoryProofs` via `bounded_first_step_tool_proofs!` in its `contract: <expr>, tools: [ … ]` form |
| `:2248` | aggregated into `ArtifactEditor::bounded_first_step_tool_proofs()` |
| `:2256` | `registry.register(FlowGraphOperationJobFactory::new(&controller))?` |
| `:2266` / `:2278` | `build_tool_job` routes the six ids to `FlowGraphOperationWork` with `FLOW_GRAPH_OPERATION_RAW_BYTES` and `FLOW_GRAPH_OPERATION_CAPACITY.work_items()` as the payload's `maximum_work_items` |
| `:2321` | `FlowPlayApp::handle`'s legacy-dispatch guard extended with `FLOW_GRAPH_OPERATION_TOOL_IDS` — the batch path now fails closed for every one of the 34 ids |

The survey walk is real work, not ceremony: it is what resolves `connectMediaPorts`' two endpoints
and `renameFlowWidget`'s old-id/collision pair against the live scene, one widget per step, so those
two routes short-circuit to an empty `Emit` without entering the host at all when the reference is
stale. Behaviour is preserved exactly — a stale reference produced an empty operation set through
`FlowHost::connect_ports(...).is_ok()` / `renamed_fixture(...) == None` before too.

### 3.2 One body per command, shared by the batch `handle` and the retained work

`if code is repeated, it MUST be close to each other` — each command module now exposes the single
config-resolved body both paths run, so the retained route and the (now unreachable) batch `handle`
cannot diverge:

| file:line | function |
|---|---|
| `🎮️commands/🔗️connect-media-ports/🦀️.rs:23` | `pub fn connect_operations(payload, snapshot, config, session) -> Vec<FlowMutation>` |
| `🎮️commands/🗺️reorganize/🦀️.rs:20` | `pub fn reorganize_operations(snapshot, config, session)` — retargeted from `(doc, cfg, session)` |
| `🎮️commands/🏷️rename-flow-widget/🦀️.rs:64` | `pub fn rename_operations(payload, snapshot) -> Vec<FlowMutation>` |
| `🎮️commands/✏️node-graph-edit/🦀️.rs:32` | `node_graph_edit_result` made `pub` |
| `🎮️commands/✅️spotlight-commit/🦀️.rs:32` | `node_graph_edit_result` made `pub` |
| `🎮️commands/▶️run-extension-action/🦀️.rs:30` | `pub fn extension_action_result(payload, snapshot, config, session) -> Emit<…>` |

### 3.3 The direct-Store proof block, de-duplicated

`FlowDirectStoreJobFactoryProofs` carried **19 verbatim copies** of
`ToolExecutionContract::resumable(16_384, 256, 256, 16_384, 7_500, 1, 1)` — a hand-copy of what the
factory's own `execution_contract()` returns, which `validate_tool_job_rows` compares byte for byte
(`registration.contract == row.contract`). It is now one named function, `flow_direct_store_contract()`
(`🦀️.rs:1317`), returned by the factory (`:1497`) and passed to the macro's `contract:` form
(`:2033`), with the tool list extended to all **21** registered ids. Same fix pattern as
`📓️work-capacity-2026-09-10.md` §4 applied to generation3d/generation2d.

---

## 4. Laws (task: bijection / exhaustiveness / boot)

New module `✏️editor/🧪️tests/🔬️interactive-job/🦀️.rs` (7 laws), measured against a
**language-agnostic** declaration `✏️editor/🧫️fixtures/🧮️interactive-job/🔣️.json` (34 ids, 4
factories, `batchOnlyPendingRewrite: []`) — the arithmetic and the partition are data, not Rust.

| law | proves |
|---|---|
| `retained_route_dispositions_are_exact_and_exhaustive` | the four `FLOW_*_TOOL_IDS` lists equal the fixture per factory, publish one nonempty lane contract per owned tool, and PARTITION `FlowCommand::TOOL_JOB_IDS` (no id owned twice, none left over). Mirrors generation3d's law of the same name. |
| `every_declared_flow_action_is_migrated` | walks the BUILT `create_flow_app()` manifest (serialized, so it reads app actions, window-kind actions, app commands and mode commands — exactly the four places `AppActionRegistry::migrated_tool_ids` reads) and asserts `interactiveJob == "migrated"` for all 34, with zero `batchOnlyPendingRewrite`. |
| `bounded_first_step_proofs_are_a_bijection_onto_the_migrated_ids` | the aggregated proof set is a bijection onto the migrated ids — the exact `seen != expected` comparison behind `interactive-job.catalog-incomplete`. |
| `graph_operation_route_declares_one_work_capacity` | the factory's contract IS `flow_graph_operation_contract()`, classification is `Migrated`, and `work_items() == rows(FLOW_STORE_MAX_SCENE_ITEMS + 1)`, `rows_for_items(1) == rows(1)`, `admits(rows(n))` true at n and false at n+1. |
| **`flow_play_app_boots_through_the_real_registry_without_a_catalog_authority_fault`** | **the boot law the task names** — constructs `VcsArtifactApp<EditorApp<FlowPlayApp>, SemioMembers>::with_registry(…)` from the REAL `AppActionRegistry::from_definition(&create_flow_app())`, which is the exact construction that ran `validate_tool_job_rows` and aborted the wgpu playground, then runs a maintenance turn and drains the app's own close ladder to its terminal-empty witness. |
| `every_graph_operation_route_is_admitted_by_its_own_retained_factory` | all six new routes dispatch to a retained admission receipt (`{operationId, generation}`), never `flow.retained.legacy-dispatch`, never `interactive-job.catalog-authority`. |
| `the_batch_handle_fallback_is_closed_for_every_declared_route` | every `FlowCommand::TOOL_JOB_IDS` row is owned by one of the four factories, so `FlowPlayApp::handle`'s `_ => command.dispatch(...)` arm is provably unreachable. |

Third-party-validated twin (CLAUDE.md's "same output with at least one third-party library"): the
`🧪️tests/🔬️source-contract/🟦️.ts` oracle re-derives the classifications and the four tool-id lists
**from the Rust source text**, independently of the Rust law, and must land on the same fixture;
two hostile mutations of the fixture must not compare equal under `fast-json-stable-stringify`.

```
[DEBUG] Flow interactive-job oracle: 34 tool ids over 4 factories, batchOnly=0 runtimeClaims=0
```

New testkit scaffolding, `✏️editor/🧪️tests/🔬️testkit/🦀️.rs:15` — `FlowAppFixture`, a self-closing
app (Deref/DerefMut to `FlowApp`, `Drop` drains `PluginApp::close_step` to the terminal-empty witness)
and `flow_app_closing()`. It deliberately skips `register_content_child`; see §7.

---

## 5. Runs

`CARGO_TARGET_DIR=$S/target-flowcat` (seeded `cp -Rc target/debug`), `RUSTC_WRAPPER=""`,
`RUST_MIN_STACK=134217728`, `--keep-going`. The shared `target/` was never touched (a restage lane
owns it). Raw logs in `🗑️generated/flowcat-*.txt`.

| gate | command | result |
|---|---|---|
| artifact crate + tests | `cargo check -p semio-s-artifact-flow-flow --tests --keep-going` | **0 errors**, 12 warnings (all pre-existing dead-code/unused-import) |
| flow lib suite | `cargo test -p semio-s-artifact-flow-flow --lib -- --test-threads=2` | **130 passed / 89 failed** of 219 (baseline **122 / 90** of 212) — **0 new failures**, 1 fixed, 7 new laws all green |
| language-agnostic oracle | `bun ✏️editor/🧪️tests/🔬️source-contract/🟦️.ts` | **pass**, 34 ids over 4 factories |
| plugin native | `cargo check -p semio-s-plugin-flow --keep-going` | **0 errors**, `Finished dev in 17.21s` |
| plugin wasm | `cargo check -p semio-s-plugin-flow --target wasm32-wasip2 --profile wasm-dev` (`CARGO_PROFILE_WASM_DEV_DEBUG=false`) | **0 errors**, `Finished wasm-dev in 1m 34s` |
| descriptor | `bun nx run @semio-tech/flow-plugin:describe` | **regenerated**, `Successfully ran target describe`, 5m 24s |

Suite tails:

```
test result: FAILED. 122 passed; 90 failed; 0 ignored; …   # baseline, before this lane
test result: FAILED. 130 passed; 89 failed; 0 ignored; …   # after
```

```
    Finished `dev` profile [unoptimized] target(s) in 17.21s          # plugin native
    Finished `wasm-dev` profile [unoptimized] target(s) in 1m 34s     # plugin wasm32-wasip2
```

### 5.1 Descriptor

The target name is `@semio-tech/flow-plugin:describe` (verified in
`✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📋️project.json`). Note: the describe script REFUSES a
`CARGO_TARGET_DIR` outside the repo —

```
describe semio-s-plugin-flow failed: artifact root /private/tmp/.../scratchpad/target-flowcat resolves outside /Users/ueli/Documents/semio
```

— so it was run against an in-repo, gitignored private target dir (`target-flowcat-describe/`,
matched by `.gitignore:16 target*`), never the shared `target/`. Result:

```
described flow (plugin semio:flow@0.1.0) -> ✏️s/🔌️plugins/🌊️flow
  (wasm=9010db93dbc44074fa147787d33d355305c2e1fdd6d2f0d3a463fc458926d88f
   core=3af04662c8832dbb8c03eb2cd6410750e1e48c674e125b6783180e036af464b4
   descriptor=5807094ae7b1794ac6a7e800604e0dfc6a33d3503cc924836e20ed6abb74e846)
```

`✏️s/🔌️plugins/🌊️flow/🛂️.descriptor.semio` and `🔣️.json` are rewritten, and the regenerated
manifest carries **250 `"interactiveJob": "migrated"` rows and 0 `batchOnlyPendingRewrite`** (250
covers the editor's 34 plus the viewer's and the framework-injected rows across every window kind and
mode).

The 89 remaining failures are **the same set as the baseline's 90** minus the one this lane fixed —
`comm -13 baseline after` is empty. Their causes are §7's pre-existing retirement/close debt, not
this lane.

---

## 6. Restage

**restage required: yes — flow-plugin component in both react (`activate-generation3d-react-dev`) and
wgpu (`buildPlugins`) closures.** The change is in the flow plugin's Rust component
(`semio-s-plugin-flow` → `semio-s-artifact-flow-flow`), so both playgrounds carry a stale
`.wasm`/descriptor until the flow component is rebuilt and restaged into each closure. The react
playground survives a stale flow component only because it never instantiates the flow editor; the
wgpu shell does, and that is the boot this lane unblocks.

---

## 7. Found in passing — NOT this lane, and why

Clearing the catalog fault let 34 tests construct a live `FlowPlayApp` for the first time, which
UNMASKED a pre-existing, separate defect class: **flow drops framework-owned values that must be
explicitly retired.** `OrderedMap<V>::drop` asserts `root.is_none()` and `ArtifactStore`'s
`Drop` asserts a terminal-empty shallow-shell witness, so a bare drop of a non-empty
`FlowFixture`/`FlowHost`/app aborts. Both `FlowFixture::retire_cold` and `FlowHost::retire_cold`
exist for exactly this and were simply not being called.

Fixed here, because the six new retained routes run through them and a panic inside
`ArtifactInstanceOperationOwnerHandle::with_mut` poisons the whole app instance
(`interactive-job.instance-owner-poisoned`, reproduced natively before the fix):

| file:line | leak |
|---|---|
| `✏️editor/🦀️.rs:2500` (`host_from_snapshot`) | the `to_fixture()` temporary handed to `flow_host_with_session` |
| `✏️editor/🦀️.rs:2508-2517` (`host_operations`) | the diff-base `to_fixture()` temporary AND the `FlowHost` itself, on both the "nothing changed" and the normal path |
| `✏️editor/🦀️.rs:243` | `flow_context_menu_items`' `to_fixture()` probe |
| `✏️editor/🦀️.rs:757-767` | `evaluate_generation_preview`'s two fixtures |
| `✏️editor/🦀️.rs:790` | `generation_window_transient`'s form-spec fixture |
| `✏️editor/🦀️.rs:2381` (`interaction_topology`) | the topology projection fixture |
| `🎮️commands/🏷️rename-flow-widget/🦀️.rs:26` | `renamed_fixture`'s early-return (blank / unchanged / taken id) path |
| `✏️editor/🧪️tests/🔬️testkit/🦀️.rs` (`register_content_child`) | the shared test fixture builder's `to_fixture()` |

Still open, and deliberately left to a dedicated lane (each needs the host retired on every `?`
early-return path, which is a restructure of the render functions, and one of them is in the
retained-artifact store path a peer session owns):

1. **`FlowHost` dropped in the render paths** — `📌️panels/🛍️catalogue/🦀️.rs:58`,
   `🎭️modes/✏️edit/🪟️windows/🌊️main/🦀️.rs:85`, `🎭️modes/✏️edit/🪟️windows/🗣️compiled/🦀️.rs:40`,
   `🎮️commands/⏱️flow-eval-tick/🦀️.rs:27,50`, `🎮️commands/🏁️flow-eval-resolve/🦀️.rs:27`,
   `🎮️commands/🧮️evaluate/🦀️.rs:27`, `👁️viewer/🎭️modes/👁️view/🪟️windows/🌊️main/🦀️.rs:86`
   (the viewer is outside this lane's ownership). Confirmed live:
   `render → drop_glue::<FlowHost> → drop_glue::<FlowFixture> → OrderedMap<WidgetLayout>::drop`.
2. **`editor::flow::component::retained::artifact::recipe::Recipe::advance` leaks an `OrderedMap`
   root** — reached the moment a real artifact-lane mutation is published, so
   `settle_registered_typed_operation` on any graph-operation route aborts. This is why
   `every_graph_operation_route_is_admitted_by_its_own_retained_factory` proves admission and stops
   short of publication; the retained work itself does complete (the settle got as far as
   `advance_apply_batch` before the leak fired).
3. **The registered content child never releases** — `close_step` returns
   `Blocked { reason: "child snapshot disposer is waiting on external ownership" }` forever once
   `register_content_child` has run, so no flow test that registers a child can reach its close
   witness. `flow_app_closing()` skips the registration for that reason.
4. **`testkit::paired_apps` is registryless** — `two_instances_converge_on_disjoint_edits` still
   faults `interactive-job.catalog-authority` with `migrated={}`, the known framework testkit gap
   (any app publishing `bounded_first_step_tool_proofs!` cannot use it).
5. **`setPreviewOff`'s extent can exceed its own ceiling** — `✏️editor/🦀️.rs:876` answers
   `ids.len() * FLOW_STORE_MAX_SCENE_ITEMS`, which is 512 for two ids against a
   `maximum_work_items` of 256, so preflight refuses any multi-id `setPreviewOff` with
   `retained command exceeds semantic work capacity`. Not touched here; it wants the same
   `ArtifactRetainedWorkCapacity` treatment the graph-operation route got.

---

## 8. Files

Created:

- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️interactive-job/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🧮️interactive-job/🔣️.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/📓️flow-catalog-authority-2026-09-10.md`

Updated:

- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️connect-media-ports/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️reorganize/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏷️rename-flow-widget/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️spotlight-commit/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/▶️run-extension-action/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️testkit/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️source-contract/🟦️.ts`

Updated (regenerated):

- `✏️s/🔌️plugins/🌊️flow/🛂️.descriptor.semio`
- `✏️s/🔌️plugins/🌊️flow/🔣️.json`

Raw logs: `🗑️generated/flowcat-*.txt` (0 baseline check, 1 baseline suite, 2-7 iterations, 8 plugin native, 9 plugin wasm, 10-11 describe, plus the two failure-set diffs).
