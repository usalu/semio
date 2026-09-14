# 📓️ Wave W3-2 — Procedural Preview Eval as a Tool Run

Lane **W3-2 procedural preview eval** (`📋️wave-3-lane-brief.md`). Abbreviations:

- `G` = `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any`
- `P` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`
- `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`

## 1. What Changed

### Preview eval is a read-only `previewEval` tool run (both surfaces)

- **Source of record:** `G/🧵️preview-eval/🔣️.json` (new) holds the tool label and icon plus the `ToolRunDefinition`:
  - `mutating: false`;
  - stages `evaluate`, `tessellate`;
  - counters `evaluated`, `failed`, `blocked`, `tessellated`, `hops`;
  - 12 reasons in EN and DE;
  - trace kind `entity`;
  - `runJob` `generation3d.previewEval.run`.
- **Run module:** `G/🧵️preview-eval/⏯️tool-run/🦀️.rs` (new, split out of `G/🧵️preview-eval/🦀️.rs:~118`, which mounts it as `mod tool_run; pub use tool_run::*`).
  - `preview_eval_tool_definition` (:126).
  - `observe_preview_eval` (:178) maps retained node statuses to trace verdicts: testing → success, warning, danger.
  - `PreviewEvalRunLink` (:249) with its one-request latch.
  - `PreviewEvalRunOwner` (:317).
  - `preview_eval_run_effects` (:329) is what `pending_effects` owes: a start, a finalize, or a wake.
  - `PreviewEvalRunJob` (:397) and its `InteractiveJob` impl (:500). One hop per `cx.consume_fuel(1)` (:538), bounded steps.
  - The job waits through the port while hops are outstanding.
  - Its close quiesces the session and dispatches the kernel release.
- **Trace:** subjects are `Entity` records keyed by FNV-1a 64 of the flow node id, so the node graph can highlight the node under test.
- **Transport:** `G/🧵️preview-eval/🦀️.rs`.
  - `FlowEvalRelease` (:112) and the release command `🔓️flow-eval-release` for the editor and the viewer (new). It emits `evaluateCancel` and `tessellateCancel`.
  - `resolve_eval` and `resolve_tessellate` only settle latches and wake the job; they no longer re-arm the next hop.
  - The status JSON publishes `cancellable` plus `cancelAction: "toolRunAbort"` plus `cancelArgs {runId, generation}` while the run is starting, running or paused.
- **Editor wiring:** `G/✏️editor/🦀️.rs`.
  - `owe_attached_previews_carrying` (:170): a gesture puts its own `toolRunStart` on its emit.
  - `pending_effects` → `preview_eval_run_effects` (:2136).
  - `build_tool_run_job` (:2143).
  - `.tool(preview_eval_tool_definition())` (:2292).
- **Viewer wiring:** `G/👁️viewer/🦀️.rs` (:147, :1212, :1218, :1573). The same wiring, plus the view command route owes previews only when the example or the tolerance changed.
- **Contributions install:** `set-contributions` in both surfaces exposes `pub fn install(payload, session) -> Result<bool, Fault>` (:39). `servable` is computed after the install.
- **Deleted:**
  - `🛑️cancel-preview-eval` in the editor and the viewer, with its command, view action, keybinding and window refs;
  - the plugin tick-latch rearm plumbing: the `set-active-example` rearm fn and `🔒️tick-latch.json`.
- **Aborting:** `toolRunAbort` is framework-reserved and dispatched by the host. `🎛️generate-mode-interactions.json` now carries the row `{command: toolRunAbort, triggers: [statusChrome, keybinding], keys: mod+.}`.
- **Crate `Cargo.toml`:** added `semio-framework-tool-run`, plus dev-dependency `fnv = "1.0.7"` (the third-party oracle).

### Host-driven abort — the Shell forwards `cancelArgs` (foreign edits)

- **Fixture:** `E/🧱️elements/🐚️Shell/🧫️fixtures/🛑️surface-controls/🔣️.json`. Every `cancelContract` row now expects `cancelArgs`, and there are 2 new rows (11 total):
  - `a-tool-run-abort-names-the-run-it-stops`;
  - `hostile-cancel-args-keep-only-scalar-values`.
- **TS contract:** `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts:356,381,424`. `World3dComputeStatusV1.cancelArgs` keeps string values and finite number values only.
- **React:** `E/🧱️elements/🌐️World3dHost/🟦️.tsx:6544`. `onCancel` now dispatches `cancelAction` with `cancelArgs`.
- **wgpu:** `E/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`.
  - `cancel_args` added on `World3dCancelAffordance` and `World3dComputeStatus` (:9912, :9938).
  - Parsing at :10103.
  - The dispatch at :7430 merges `cancelArgs` with `surfaceId`.
- **Tests:**
  - `E/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs` asserts `cancel_args` and `rows == 11`.
  - `E/🧪️tests/🔬️engine-contract/🟦️.ts` asserts the schema requires `cancelArgs`, and asserts `toEqual`.

### Framework port (foreign edits, `P`)

- `P/⏯️tool-run/🦀️.rs`: `ToolRunJobPort` (:124), `ToolRunLedger::port` (:615) and `drain_tool_run_port` (:1129).
  - A waiting job is never stepped, and keeps `has_pending_work` false.
  - Port effects drain into the typed effect outbox.
  - Admit wakes the port.
  - `ToolRunJobRequest` gained `instance_owner` and `port`.
- `P/🦀️.rs`:
  - re-exports at :6490 and :37406;
  - an `ArtifactViewer::build_tool_run_job` default plus the `ViewerApp` forward;
  - `declared_actions_bridge` skips reserved tool-run ids (:6984).
- `P/🧪️tests/🔬️tool-run/🦀️.rs:922` (`ToyWaitJob` law) plus a `port` section in `P/🧫️fixtures/⏯️tool-run/🔣️.json`.

### W1-D predicate row (foreign edit)

- Root `📜️script.ts:9739`: the generation3d `importDocument` row is replaced by `previewEval`.
  - Scope: `🧵️preview-eval/⏯️tool-run`.
  - Verbs: `cancelPreviewEval`, `cancel_preview_eval`, `CancelPreviewEval`, `PREVIEW_CANCEL_ACTION_ID`, `rearm_attached_previews`.

## 2. Public API

- **`preview_eval`:**
  - `preview_eval_tool_definition() -> ToolDefinition`
  - `observe_preview_eval(status_json, eval_json, preview_widget_ids, mesh_state, settled) -> PreviewEvalObservation`
  - `preview_eval_run_effects(session, link, windows, run: Option<&ToolRunView>, servable) -> Vec<Effect>`
  - `PreviewEvalRunLink`
  - `trait PreviewEvalRunOwner`
  - `PreviewEvalRunJob<O>`
  - `FlowEvalRelease`
  - `owe_attached_previews{,_for_mutations,_carrying}`, where the peer added the latter two
- **Commands:** `flowEvalRelease` in both surfaces. `cancelPreviewEval` is removed.
- **Framework:**
  - `ToolRunJobPort {dispatch, wait, wake, is_waiting, has_effects}`
  - `ToolRunLedger::port()`
  - `ToolRunJobRequest.{instance_owner, port}`
  - `ArtifactViewer::build_tool_run_job`
- **Shell contract:** `World3dComputeStatusV1.cancelArgs`.

## 3. Tests Run (Foreground, Logs in `🗑️generated/W3-2/`)

| Command | Result |
|---|---|
| `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib` | 453 pass, 3 fail (pre-existing, §6) — `test-lib-full-3.txt` |
| `… --lib preview_eval` | 18/18 — `test-preview-eval.txt` |
| `cargo check -p semio-s-artifact-procedural-generation3d --features component-app-assembly --tests` | clean — `check-split.txt` |
| `cargo check -p semio-s-artifact-procedural-generation3d --features component-app-assembly --target wasm32-wasip2` | clean — `check-wasm.txt` |
| `cargo test -p semio-framework-plugin --lib tool_run` | 24 pass, 1 fail — `plugin-tool-run-tests.txt`. The failure is W0-I's in-flight chord law; the port law passes. |
| `cargo test -p semio-framework-os-renderer-wgpu --lib shell_chrome_parity` | 24/24 — `wgpu-chrome-parity.txt` |
| `vitest run engine-contract -t "cancel affordance\|declared schema\|control ids the wgpu shell"` (react package config) | 4/4 — `engine-contract-cancel.txt` |
| `bun T/🐍️w3-2-preview-eval-contracts.ts` (TS twins of `⏯️preview-eval-run.json`, `⏱️evaluate-budget.json`, `🚧️contribution-gated-arming.json`) | OK — `ts-twins.txt` |
| `bun T/🐍️w1d-policy-probe.ts` | local-lifecycle 0, legacy-trace 0, reserved-action 0. previewEval is not listed; only the unassigned `reorganize` declarations remain — `w1d-probe.txt` |

The laws cover:

- a language-agnostic fixture: `G/🧫️fixtures/⏯️preview-eval-run.json`, with vocabulary, entityKeys, observations, scheduling, release, status and runEffects;
- the `fnv` crate as the entity-key oracle;
- abort, finalize and supersession through the `runEffects`, `status` and `jobSupersession` rows;
- the drive-step budget (`⏱️evaluate-budget.json` `owesHop`, hex boot `turns <= round_trips + 1`).

## 4. `launch.json`

No new entries; this lane does not edit `launch.json`. Existing entries are `🧪️generation3d-preview-window-transient` and `…-native`. The commands in §3 are the run instructions until the coordinator registers them.

## 5. Deviations

- **Transport through a port instead of in-step effects.** A run job cannot emit extension invocations or window-transient publications from `step`. Instead, the job hands `flowEvalTick` and `flowEvalRelease` hops to the host through `ToolRunJobPort` and waits. Resolves wake it. The hop commands stay as transport; they are not run verbs.
- **`importDocument` was not converted.** It is a host-driven chunk transfer followed by one bounded parse that already publishes one staged edit, so it has no multi-step algorithm to trace. The W1-D row was corrected accordingly.
- **Predicate scope.** The previewEval row covers `🧵️preview-eval/⏯️tool-run`. The kernel transport keeps `evaluateCancel` and `tessellateCancel`, which are extension verbs, not run verbs.
- **A peer reshaped `runEffects`** (PROCEDURAL-3D-END-TO-END): a complete run is always finalized, and expectations are arrays. I aligned the TS twin with it.

## 6. Foreign Edits and Peers

- **Edited outside owned paths:**
  - `P/⏯️tool-run/🦀️.rs`, `P/🦀️.rs`, `P/🧪️tests/🔬️tool-run/🦀️.rs`, `P/🧫️fixtures/⏯️tool-run/🔣️.json`
  - root `📜️script.ts`
  - `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts`
  - `E/🧱️elements/🌐️World3dHost/🟦️.tsx`
  - `E/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
  - `E/🧱️elements/🐚️Shell/🧫️fixtures/🛑️surface-controls/🔣️.json`
  - `E/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs`
  - `E/🧪️tests/🔬️engine-contract/🟦️.ts`
- **Peer work kept intact:**
  - W0-I: generic job slot, `trace_keys`, `entity_marks`, `settings`, chord actions, read-only auto-finalize, float `runId`.
  - PROCEDURAL-3D-END-TO-END: `owe_attached_previews_for_mutations` and `_carrying` in `G/🧵️preview-eval/🦀️.rs`, the `gestureRearm` and `jobSupersession` rows, `🩹️gesture-rearm`, `🛑️preview-cancel.json`, `⌨️keyboard-reachability.json`.
  - To let that peer code compile after the split, the `PreviewEvalRunLink` fields, `PreviewEvalRunRequest`, `owned_by` and `run_action_effect` are now `pub(crate)`.
- **Pre-existing reds, not caused by this lane:**
  - `generation_preview_is_one_app_transient_shared_by_two_generation_windows` (`transient=0`);
  - `two_instances_converge_disjoint_widget_moves` (VCS merge fail-closed);
  - `vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` (publication contended).
- **Blocked meanwhile:** the puzzle-3d crate (W2-C) failed to compile for a while, which held up the wgpu test until it recovered.

## 7. Open Items

- **generation2d:** its eval loop is not a tool run yet. It has no cancel verb, so the W1-D rows are clean, but its preview still evaluates outside the ledger.
- **`reorganize`** in generation3d and generation2d is still undeclared; it belongs to the W3-F shared layout lane.
- **Trace consumers:** mounting the host run panel and highlighting nodes from entity trace records in the node graph are for the UI lanes.
- **The `mod+.` chord** depends on W0-I's argument-free chord resolution, whose law is still red in-flight.
- **wgpu Shell** was checked natively only. Its `cancelArgs` path uses only `serde_json` and `semio_framework`.
