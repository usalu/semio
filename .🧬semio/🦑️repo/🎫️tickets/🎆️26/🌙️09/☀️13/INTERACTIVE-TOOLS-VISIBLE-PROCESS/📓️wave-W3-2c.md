# 📓️ Wave W3-2c — Generation2d App Close Defect

Lane **W3-2c generation2d app close defect** (follow-up to `📓️wave-W3-2b.md` §6/§7). Abbreviation:

- `A` = `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any`

## 1. Root Cause

The retained reader that never retired was the generation2d **document snapshot retirement** itself, not a session or lease.

- Every document owner (widgets, synapses, layout, text) is retired through `semio_framework_artifact_flow_flow::retained::FlowRetirement`.
  - That frontier uses reserve-then-close: its bare `ErasedSnapshotRetirement::close_step` answers `Blocked`, never an error, for as long as `next_allocation_bytes` still names a page.
  - `FlowRetirement::close_page` pays that reservation first.
- `A/🧬️schema/🧬️mutations/💾️binary/🦀️.rs` still called the bare `self.flow.close_step(..)` / `self.domain.close_step(..)`, and gated on `is_empty()` instead of `terminal_is_empty()`. This affected both `Generation2dRetainedSnapshotRetirement` and `Generation2dReplayRetirement`.
- The framework close ladder drives with one item and one 4 KiB page and has no demand channel. So:
  - `ArtifactDocumentStoreDisposer` reported `Blocked` ("document store close awaits a retained reader or owner");
  - `close_registered_fixture_app` spun until its 30 s deadline;
  - the store-fixture tests failed their terminal witness in the same way.
- generation3d got this fix in ticket 26/09/09/PROCEDURAL-3D-END-TO-END (`generation3d_close_flow_frontier`). generation2d never received it. That is the whole 2d/3d diff on the close path.
- **The framework is not at fault** (`close_page` exists and is documented), so no framework edit was needed.

## 2. What Changed

- **Close fix:** `A/🧬️schema/🧬️mutations/💾️binary/🦀️.rs`
  - New `fn generation2d_close_flow_frontier(flow: &mut FlowRetirement, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String>` (:536). It returns `Blocked` only on a zero grant; otherwise it calls `flow.close_page`.
  - `Generation2dReplayRetirement::close_step` (:550) and `Generation2dRetainedSnapshotRetirement::close_step` (:712) route through it and gate on `terminal_is_empty()`. Their `terminal_is_empty` witnesses do the same.
- **Regression laws (failed first, then passed):** `A/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️retained-authority-laws/🦀️.rs` :348–:397, region `FlowFrontierOwnership`.
  - `drive_under_the_frameworks_fixed_page_grant` (exact 1-item/4 KiB driver; `Blocked` panics).
  - `every_document_retirement_pays_its_own_flow_frontier_under_the_fixed_page_grant`. Oracle: the framework's generic `Arc` retirement route (`store::SnapshotRetirementFactory`) must release the same bytes as the owned route.
  - `every_displaced_replay_owner_pays_its_own_flow_frontier_under_the_fixed_page_grant`.
- **Unmasked red #1, fixed:** `declared_actions_bridge_to_commands` ("nodeGraphViewport requires viewport").
  - `A/✏️editor/🦀️.rs` gains `fn parse_flow_viewport(args: &dsl::DslValue) -> Result<Viewport2d, Fault>` (:1304).
    - An absent `viewport` decodes to the identity camera; a malformed one still faults.
    - This mirrors generation3d. `"nodeGraphViewport"` now uses it (:1547).
  - New law `node_graph_viewport_decodes_an_absent_viewport_as_identity_and_refuses_a_malformed_one` (`A/✏️editor/🧪️tests/🔬️unit/🦀️.rs` :790). It failed first, then passed.
- **Unmasked red #2, the law was out of date after W3-2b:** `A/✏️editor/🧪️tests/🪟️generation2d-window-camera-ownership/🦀️.rs`.
  - The law still asserted that `AddGeneration` publishes a `Transient` lane. W3-2b's contract says the command publishes none and carries `toolRunStart`, and the `previewEval` run publishes the transient.
  - New helper `populate_generate_preview` (:136):
    - requires no transient lane on the command;
    - drives the real run through `context::drive_preview_run`;
    - requires `transient_generation` to advance.
  - It is preceded by `removeWidget rect` (:205), because the bundled `rect` needs the uncontributed `math.add` and no run is servable otherwise. This is the same setup as W3-2b's law.
  - The lane stack went from 2 MiB to 4 MiB, with a docstring explaining why. The unoptimized reducer path plus the run hops overflow 2 MiB, and an overflow SIGABRTs the whole test binary. Measured: 2 MiB overflows, 4 MiB and 8 MiB do not.
- **Test context:** `context::drive_preview_run(app, action_meta: &ActionMeta, initial: &[Effect]) -> PreviewRunReceipt` (`🔬️unit/🦀️.rs` :116).
  - It now takes the caller's `ActionMeta` (shell view + bound instance) instead of a `ViewModel` with a hardcoded instance 1.
  - Its inner awaits are `Box::pin`ned.
  - The four W3-2b callers were updated.

## 3. `🔣️.json` Regeneration

- Generator: `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📜️script.ts describe`. It is nx target `@semio-tech/procedural-plugin:describe` and uses `describePluginComponent`: it builds the wasm component, then emits `🛂️.descriptor.semio` and `🔣️.json`.
- Run: `bun ./📜️script.ts describe` from `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust`, foreground, exit 0, about 18 min under the shared build lock (`describe-1.txt`).
  - Output: `described procedural (plugin semio:procedural@0.1.0)`, descriptor sha `c05a54e6…`.
  - It rewrote `🛂️.descriptor.semio` and `🔣️.json`.
- Before: `"id": "flowEvalTick"` appeared 6 times, 5 of them as window-kind **actions**.
- After: 3 times, all under `manifest/apps[*]/commands` only. No window kind lists `flowEvalTick` as an action.
  - generation2d editor
  - generation3d editor
  - generation3d viewer
- The remaining entries are the declared runtime command that the `previewEval` run's hop redispatches (W3-2b §1, "`flowEvalTick` is declared as a runtime command"; W3-2 for generation3d). Removing them would break the hop route, so they stay by contract.

## 4. Tests Run (Foreground, Logs in `🗑️generated/W3-2c/`)

| Command | Result |
|---|---|
| `cargo test -p semio-s-artifact-procedural-generation2d --features component-app-assembly --lib -- flow_frontier` | failed first: 2 fail, `Blocked` (`test-red.txt`); after the fix: 2 pass (`test-green.txt`) |
| `… --lib -- node_graph_viewport_decodes declared_actions_bridge` | failed first: 2 fail (`test-viewport-red.txt`); after the fix: 2 pass (`test-viewport-green.txt`) |
| `… --lib` (full generation2d) | before this lane: 213 pass / 35 fail in 120 s (W3-2b). After the close fix: 246 / 4 in 7.6 s (`test-lib-full-1.txt`). Final: **248 pass / 3 fail** in 2.4 s (`test-lib-full-2.txt`). |
| `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib` | 452 pass / 4 fail (`test-3d-lib-full.txt`). Zero generation3d files were edited, so none are new (§6). |
| `cargo check -p semio-s-artifact-procedural-generation2d --features component-app-assembly --tests` | exit 0, 35 warnings in the crate, none in lane-edited lines (`check-native-tests.txt`) |
| `… --target wasm32-wasip2` | first attempt failed on a peer's in-flight `🔌️plugin/⏯️tool-run/🦀️.rs` edit (`ToolRunLedger` fields, `check-wasm.txt`); the retry was clean, 12 warnings in the crate (`check-wasm-2.txt`) |

W3-2b's app laws now pass for real in the full run:

- `a_generation_command_carries_the_run_start_and_the_run_publishes_the_shared_generate_preview`
- `the_preview_eval_run_finalizes_nothing_and_an_abort_leaves_the_document_byte_identical`
- `closing_an_unsettled_run_job_quiesces_its_window`

## 5. Remaining Generation2d Reds (3, Each With a Distinct Cause, None in the Lane)

1. **`two_instances_converge_disjoint_widget_moves`**
   - `attach b` faults with `module.vcs`: "remote snapshot merge is fail-closed until the app-owned streaming envelope decoder and persistent candidate transaction are terminal-authorized".
   - This is framework VCS merge policy. The same test fails identically in generation3d.
2. **`vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed`**
   - "production envelope load did not reach terminal": the decode `WorkerJobSession` stays `Pending` forever.
   - generation3d fails identically, and its own driver docstring documents the stall (PROCEDURAL-3D-END-TO-END `remaining-suite-reds-2026-09-13.md` §3.3).
3. **`generation2d_window_camera_ownership_runtime_isolates_routes_renders_and_reopens`**: fails with "edit-preview renderer crossed exact instances". Root cause is in the framework, found with temporary `[DEBUG]` probes that were then removed:
   - `WindowConfigOwnerRegistry::load` (`🧰️framework/…/🔌️plugin/🪟️window/🎚️config/🦀️.rs`) drives `WindowConfigPackLoadGrant::one_page()` (4 096 B).
   - In `📥️retained/🦀️.rs` `reserve_next`, the source page reservation asks for **4 104 B** (the page plus the machine-width header), and `requested > maximum_bytes` refuses it on every turn.
   - The load stays in `PackIngress` for the whole 1 048 576-turn bound. It is then cancelled and **`load` returns `Ok(())` with nothing installed**, so every loaded window config reads `Default`.
   - Trial fix: paying the reservation out of `remaining` credits instead of the payload grant. The load then advances, but is rejected with `Pack` and its retirement panics in `Drop`.
   - The framework's own law `window_config_retained_pack_load_current_registry_identity_and_reopen_baseline` is also red at HEAD, with a different cause: `window-config.pack-envelope` (`test-fw-window-load-red.txt`).
   - This is a multi-layer framework window-config subsystem defect, outside this lane, so the trial edit was **fully reverted**.
   - Open item for a framework lane: fix the reservation grant, make bound exhaustion an `Err`, then the `Pack` rejection and retirement.

## 6. Generation3d Reds (Not Caused by This Lane: No 3d Edits)

- `two_instances_converge_disjoint_widget_moves`, and `vcs_artifact_app_non_empty_retained_maintenance_swap_…`: the same framework causes as §5.1 and §5.2.
- `generation_preview_is_one_app_transient_shared_by_two_generation_windows` (`🔬️unit/🦀️.rs:770`) and `viewer … only_a_viewer_preview_addressed_tick_passes_the_retained_preflight` (`👁️viewer/🧪️tests/🔬️eval-chain/🦀️.rs:113`): these belong to generation3d preview-eval work in flight by peers.

## 7. `launch.json`

To register, beside the generation2d preview entries:

- `cargo test -p semio-s-artifact-procedural-generation2d --features component-app-assembly --lib -- flow_frontier`
- `cargo test -p semio-s-artifact-procedural-generation2d --features component-app-assembly --lib -- node_graph_viewport_decodes`
- `nx run @semio-tech/procedural-plugin:describe` (the manifest regeneration), if it is not registered already.

## 8. Foreign Edits, Deviations, Open Items

- **Foreign edits:** none remain.
  - The temporary `[DEBUG]` probes in `🧰️framework/…/🪟️window/🎚️config/🦀️.rs` and the trial fix in `…/📥️retained/🦀️.rs` were removed. `git diff` shows no W3-2c lines, and `📥️retained/🦀️.rs` is back to HEAD.
- **Deviation:** the window-camera law's pinned stack went from 2 MiB to 4 MiB (§2).
- **Manifest:** regenerated (§3). `flowEvalTick` survives only as the runtime command, as the contract requires.
- **Open items:**
  - The framework window-config retained load defect (§5.3).
  - The framework remote merge and envelope-decode stalls (§5.1, §5.2).
