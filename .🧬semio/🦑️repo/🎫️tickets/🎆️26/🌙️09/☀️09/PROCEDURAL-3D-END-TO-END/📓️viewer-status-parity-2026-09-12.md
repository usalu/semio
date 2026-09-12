# Viewer Preview Status Parity — 2026-09-12

Closes the `view:*` gap in `🗑️generated/journey-7/results.json`: in the viewer role
(`http://127.0.0.1:6018/?plugin=generation3d&role=viewer`, window `procedural-view-preview`) meshes
rendered but the preview published **no status contract at all** — `data-status-json` carried no
`phase`, no `progress`, no `cancellable` — while the editor's `procedural-preview` and the
generate-mode `generation3d-generate-preview` published the full object.

Outputs: `🗑️generated/viewer-status/`. Native proof only; see §6 for the restage.

---

## 1. The defect, restated

`👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs` `render` built its `World3dScene` without ever
setting `status_json`, so the field stayed `None`. The projection that fills it
(`preview_scene_status_json` / `preview_progress_status_json` / `preview_status_json` plus each
window's own hand-rolled `debug` merge) lived on `✏️editor`, and `policyViewerPurityBreaches` forbids
a viewer file reaching through `::editor::` — so the viewer could not have called it even if someone
had wanted to. The consequence is not cosmetic: the shell learns the cancel affordance **only** from
the surface's own published status (`🌐️World3dHost/🟦️.tsx:4912` reads `computeStatus.cancelAction`
and `declareSurfaceCancelAction`s it for exactly as long as `cancellable` holds), so a viewer that
published nothing could neither show progress nor be stopped.

Three near-duplicate `debug` merge blocks also existed — one per preview window.

## 2. What changed — one projection, at the subset level

Everything below moved **verbatim** out of `✏️editor/🦀️.rs` into the surface-neutral
`🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs` (`crate::preview_eval`, mounted at the
ARTIFACT level beside `✏️editor`/`👁️viewer`), in a new `//#region 📈️Status`:

| member | note |
|---|---|
| `preview_status_json(eval_json, fixture)` | the per-widget `widgetErrors` half — surface-neutral by construction |
| `merge_status_json` (private) | unchanged |
| `preview_scene_status_json(session, widget_status)` | now takes `Option<&FlowEvalSession>` |
| `preview_progress_status_json(session)` / `…_for(session, address)` | now take `Option<&FlowEvalSession>` |
| `PreviewStatusDebug<'a> { eval_json, meshes_json, instances_json }` | **new** — the probe counters as one declared shape |
| `preview_window_status_json(session, widget_status, debug, hint)` | **new** — THE projection all three windows call |
| `PREVIEW_CANCEL_ACTION_ID` | **new** — the `cancelAction` the status names, one constant |

`✏️editor/🦀️.rs:2354` now re-exports those names so every editor call site keeps naming them
unqualified. The three window renders are one line each:

- `✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs` — 21-line inline debug merge → one call
- `✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs` — `generate_preview_status_json` is now a
  two-line binding that only forwards its `hint`
- `👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs` — publishes `status_json` for the first time,
  through the same call, including the `widgetErrors` half it never had

**`session: Option<…>` is deliberate.** A surface's marks-free `ArtifactViewer::render` entry point is
handed no retained session; the window still publishes the whole schema (idle phase, zero progress,
nothing to cancel). Publishing nothing is never an option — that was the defect.

## 3. What changed — the viewer owns its cancel

The viewer had **no** `cancelPreviewEval` command, so a published `cancelAction` would have been a
dead button (`ShellHost`'s `declaredAction` gate drops an undeclared verb before `plugin.handleAction`).
Added, mirroring the editor's shape:

| piece | file |
|---|---|
| `cancelPreviewEval` binding | `👁️viewer/🎮️commands/🛑️cancel-preview-eval/🦀️.rs` (new) |
| `flowTessellateCancelResolve` binding | `👁️viewer/🎮️commands/🧯️flow-tessellate-cancel-resolve/🦀️.rs` (new) |
| module mounts | `🧊️generation3d/🦀️.rs` viewer `commands` block |
| command enum rows | `👁️viewer/🦀️.rs` `view_commands!` — appended, never reordered (the ordinal is the wire format) |
| tool ids | `GENERATION3D_VIEW_FLOW_EVAL_TOOL_IDS` now carries both, exactly as the editor's list does |
| publication contracts | both `HostOnly` — no store lane at all |
| bounded first-step proofs | both added to `Generation3dViewFlowEvalJobFactoryProofs` |
| retained route | `Generation3dViewFlowResolveWork::step` gained the two rows; the cancel is the one row whose emit carries `extension_invocations` rather than `effects` |
| `command_from_action` | both rows; `windowKindId` defaults to `procedural-view-preview` because `World3dHost`'s `dispatch` injects only `windowId` and this viewer has exactly one preview kind |
| manifest | `cancelPreviewEval` declared in-palette with `windowId`/`windowKindId` args, `ActionKind::View`, `Migrated`; the resolve declared hidden |

It is a **View** action by construction: `ViewEmit` is what `Generation3dViewCommand::dispatch`
returns, and the cancel retires only ephemeral runtime work (the retained session's tessellation
ledger and the geometry extension's own kernel jobs).

`crate::flow_operators::retire_flow_eval_session` — the bare-session close ladder — moved out of the
editor testkit into the shared test module so the viewer law can use it without reaching through
`::editor::`; the editor testkit now re-exports it.

## 4. Tests

### 4.1 Language-agnostic fixture

`🧫️fixtures/🛑️preview-cancel.json` (the cancellation lane's own, already at subset level) gained:

- `phaseLabels.faulted` (`Geometry extension unavailable` / `Geometrie-Erweiterung nicht verfügbar`)
- three new `laws` entries: `everyPreviewWindowPublishesTheContract`,
  `theCancelVerbIsLearnedFromTheStatus`, `aSessionlessWindowStillPublishes`
- `surfaces: ["editor","viewer"]` on every existing cancel row
- a new **`statusContract`** section: `objectKeys`, `progressKeys`, `debugKeys`, `ratioLaw`,
  `evaluateFaultCode`, `addressMissCode`, a `surfaces` table naming all three preview windows and
  their shared `cancelAction`, and five `states` — `idle`, `computing`, `faulted-evaluate`,
  `faulted-unaddressable-kernel`, `cancelled` — each a replayable session `sequence` plus the exact
  status it must produce. `ratio` is declared as `{done, total}` so both implementations derive it
  from the stated law rather than pinning a float literal.

### 4.2 Rust law (viewer)

New `👁️viewer/🧪️tests/🔬️status-contract/🦀️.rs`, mounted from `👁️viewer/🦀️.rs`:

- `the_viewer_preview_status_obeys_the_shared_contract_in_every_state` — fixture-driven, replays each
  state against a real `FlowEvalSession` and asserts phase, en+de label, every progress counter, the
  ratio, `cancellable`, `cancelAction` and the fault object (code / extensionId / capability / en+de
  message), for all five states
- `a_sessionless_viewer_preview_still_publishes_the_contract`
- `the_rendered_viewer_preview_scene_carries_the_status_contract` — end-to-end: decodes the real
  rendered window body and asserts `World3dScene.status_json` carries the contract **and** the four
  `debug` keys the browser probe reads
- `the_viewer_declares_the_cancel_verb_its_status_names` — the verb is declared, `View`, `Migrated`,
  routed, and `HostOnly`
- `a_viewer_cancel_dispatches_live_and_never_mutates_the_document` — the real interactive-job
  pipeline, snapshot unchanged

The invocation **count** a cancel emits is deliberately NOT asserted here (a peer's evaluate-budget
lane made it two doors, `evaluateCancel` + `tessellateCancel`); this law states only the half the
status depends on — an addressable kernel is told, an unaddressable one is not.

### 4.3 Third-party twin

New `🧪️tests/🔬️status-contract/contract.ts` (subset level, beside the fixture's home) —
`testGeneration3dPreviewStatusContract`: an independent TypeScript model of the ledger, the phase
precedence (address miss ▸ evaluate fault ▸ gesture ▸ ledger), `cancellable` and the ratio law,
cross-checked against the SHELL's own total parser `world3dComputeStatusV1`. Runner:
`🔍️preview-status-contract.ts`.

`✏️editor/🎮️commands/🛑️cancel-preview-eval/🧪️tests/🔬️unit/contract.ts` gained the `faulted` label row
(its `phaseLabels` equality would otherwise reject the fixture's new entry).

### 4.4 Commands run, results verbatim

```
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d \
  --features component-app-assembly --lib -- viewer preview_eval --test-threads=1
```
→ **63 passed; 0 failed** (`🗑️generated/viewer-status/run-final-viewer.txt`). Includes the five new
status laws and, still green, `every_viewer_action_dispatches_live_and_never_mutates_the_document`,
`every_viewer_tool_id_is_declared_in_all_four_tables`, `no_viewer_tool_publishes_on_the_artifact_lane`,
`every_emitted_action_is_declared_on_the_preview_window_kind`.

```
… --lib -- cancel_preview_eval status_contract generate::windows::preview edit::windows::preview \
  --test-threads=1
```
→ **13 passed; 0 failed** (`run-final-cancel.txt`) — both editor preview windows' own status laws and
the editor's `the_cancel_gesture_obeys_its_fixture_end_to_end` are unaffected by the move.

```
bun ".🧬semio/…/PROCEDURAL-3D-END-TO-END/🔍️preview-status-contract.ts"
```
→ `generation3d preview-status surfaces=editor:procedural-preview generate:generation3d-generate-preview viewer:procedural-view-preview states=idle,computing,faulted-evaluate,faulted-unaddressable-kernel,cancelled cancelAction=cancelPreviewEval` / `OK`.

```
bunx tsc --noEmit --strict … 🔬️status-contract/contract.ts 🛑️cancel-preview-eval/…/contract.ts
```
→ zero errors in either file (the run's other diagnostics are pre-existing framework-package ones).

```
cargo check -p semio-s-artifact-procedural-generation3d --lib                       # default features
RUST_MIN_STACK=33554432 cargo check … --features component-app-assembly --lib --profile test
```
→ both clean; **no new warning from any file this lane touched**.

One observed status, verbatim from the `computing` state:

```
{"phase":"meshingFaces","phaseLabel":{"en":"Meshing faces","de":"Flächen werden vernetzt"},
 "progress":{"unitsDone":7,"unitsTotal":24,"facesDone":2,"facesTotal":9,"inFlight":1,
             "evalUnitsDone":0,"evalUnitsTotal":0,"ratio":0.2916666666666667},
 "cancellable":true,"cancelAction":"cancelPreviewEval"}
```

### 4.5 Full-suite result and attribution

```
RUST_MIN_STACK=33554432 cargo test … --lib -- --test-threads=1
```
→ **380 passed; 5 failed** (`run-full-lib.txt`), down from the 10 recorded in
`📓️viewer-eval-chain-2026-09-12.md` §4.4. All five are the SAME peer-lane classes documented there,
none in a status or preview path:

| failing test | where it asserts |
|---|---|
| `add_generation_records_an_undoable_generation_operation` | `🧰️framework/…/🔌️plugin/🦀️.rs:6862` "undo did not revert to the expected snapshot" |
| `undo_redo_round_trips_flow_graph_edits` | same line, same message |
| `two_instances_converge_disjoint_widget_moves` | `🧰️framework/…/🔌️plugin/🦀️.rs:6839` |
| `vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` | "P3 production envelope load did not reach terminal" |
| `generation_preview_is_one_app_transient_shared_by_two_generation_windows` | the 30 s retire-deadline class ("preview operation did not finish") |

## 5. Two things NOT mine, flagged rather than fixed

1. **`🔍️preview-cancel-contract.ts` is red on the peer's two-door change.** A concurrent
   evaluate-budget lane made `cancel_preview_eval_for` emit `evaluateCancel` **and**
   `tessellateCancel`, and updated `🧫️fixtures/🛑️preview-cancel.json`'s `rows[].invocations` to
   match — but that twin's `CancelSession.cancel()` still pushes one invocation, so it fails
   `emitted invocation count` (`1 !== 2`). Their Rust law
   (`the_cancel_gesture_obeys_its_fixture_end_to_end`) passes. Left to that lane; I only added the
   `faulted` label row their `phaseLabels` equality needed.
2. **A mid-run peer edit produced one phantom failure.** One intermediate run showed
   `a_late_contributions_install_re_arms_the_viewer_evaluation_the_empty_registry_faulted` failing
   with "retained command reducer rejected operation"; the flow host
   (`🧰️framework/…/🌊️flow/🖥️host/🦀️.rs`, mtime 10:38) changed between that compile and its run. It
   passes in isolation and in every run retained here (`run-final-viewer.txt`, `run-full-lib.txt`).

## 6. A restage is required for the browser to show this

**This lane stops at native proof.** Nothing above is visible on 6018 until the procedural plugin is
restaged and its wasm rebuilt — the guest component the shell loads is a build artifact, and the
status object is produced inside it. The `journey-7` `view:*` rows will keep reporting
`phase: null` against the currently staged wasm no matter how green the Rust laws are. I did not
restage and did not restart any dev server, per the lane's standing constraint.

## 7. Files

Modified
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🦀️.rs`
- `…/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs`
- `…/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `…/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs`
- `…/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs`
- `…/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️testkit/🦀️.rs`
- `…/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `…/🪆️subsets/✳️any/✏️editor/🎮️commands/🛑️cancel-preview-eval/🧪️tests/🔬️unit/🦀️.rs`
- `…/🪆️subsets/✳️any/✏️editor/🎮️commands/🛑️cancel-preview-eval/🧪️tests/🔬️unit/contract.ts`
- `…/🪆️subsets/✳️any/👁️viewer/🦀️.rs`
- `…/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs`
- `…/🪆️subsets/✳️any/🧫️fixtures/🛑️preview-cancel.json`
- `…/🧊️generation3d/🧪️tests/🔬️flow-operators/🦀️.rs`

Added
- `…/🪆️subsets/✳️any/👁️viewer/🎮️commands/🛑️cancel-preview-eval/🦀️.rs`
- `…/🪆️subsets/✳️any/👁️viewer/🎮️commands/🧯️flow-tessellate-cancel-resolve/🦀️.rs`
- `…/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️status-contract/🦀️.rs`
- `…/🪆️subsets/✳️any/🧪️tests/🔬️status-contract/contract.ts`
- `<ticket>/🔍️preview-status-contract.ts`
- `<ticket>/📓️viewer-status-parity-2026-09-12.md` (this file)
