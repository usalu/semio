# WGPU frame loop fix: traces, redraw on demand, non-fatal shortages (2026-11-22)

Steps A-D of section 6 of `📓️wgpu-frame-loop-quarantine-audit.md`. Paths relative to `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/` (E). Status: edited, NOT compiled or run (see section 6).

## 1. Step A: traces (schema-first)

- Oracle: `E/🧪️tests/🧫️fixtures/🚦️frame-demand-and-traces/🔣️.json` (7 demand rows, 2 action rows, laws).
- `[TRACE] frame build admitted generation=N reason=<flags joined by +>` built by `frame_build_admitted_line` in the new `E/🎯️targets/🧊️wgpu/👷️frame-job/🚦️frame-demand/🦀️.rs`; emitted by `build_and_publish_snapshot_on_original_caller` (`🪟️winit-app/🎞️frame-generation-and-os-host/🦀️.rs`) once, when a build goes from no live session to a live or produced one. The `reason` names why the tick built, so a residual spin names its own cause.
- `[TRACE] frame input action controller=C action=A` built by `frame_input_action_line` and emitted in `transfer_frame_input_action` (`🧊️renderer/🛎️frame-actions/🦀️.rs`) once per materialized action, before publication. It matches the node-gestures probe key `action=interactionSelect`.

## 2. Step C: idle spin, fixed at the root

Cause: `OsHost::redraw_offscreen_worker` (browser tick) ignored the scheduler verdict (`let _ = should_render(..)`) and always ran `prepare_redraw` (which advances the frame generation when no build is live) and admitted a new build. Any tick, including a deadline timer or a wake, therefore built a frame.

Fix: a tick builds only when `FrameDemand` is non-empty: scheduler invalidation (input, due deadline, animation, resource wake), first frame, queued input, frame ready, live build, pending presentation, surface resize work, retained snapshot, component close, cursor wake, or runtime pending work (icon preview, text, world3d, raster selection, settle pump, ready applies). Otherwise `prepare_redraw` and the build are skipped and the tick only republishes the last snapshot.
Residual risk: if one of the runtime-pending predicates stays true on an idle shell, builds continue; the `reason=` field of the admitted trace names it.

## 3. Step D and capacity refusals (non-fatal)

- Removed the post-step deadline fault `frame maintenance bounded field exceeded its deadline` (`🧊️renderer/📮️mailbox-impl/⏳️pending-work-deadlines-and-spawns/🦀️.rs`).
- Removed `runtime dispatch completion credits exhausted` and `frame deferred completion credits exhausted` frame faults (`🧊️renderer/🛎️frame-actions/🔁️runtime-apply/🦀️.rs`). The checkout is returned and the apply retries next frame; the surface is not closed.
- Retired the pre-step classifier lane. `frame_maintenance_terminal_fault` is now `(cancelled, stale)` and no longer returns `frame maintenance deadline expired`. In `try_spawn_frame_maintenance_reserved`, a deadline counts as passed only with `deadline_ms.is_some_and(|deadline| worker_pool.now_ms() > deadline)`. Cancel and stale still `begin_close` and set `interaction.frame_fault`. A passed deadline while that owner is live only sets `cursor.shell_maintenance = false` (defer), then the existing release and `execution.finish()` return the owner. Law pins: `chrome-and-maintenance-checks` requires the two-argument call, the defer arm, and rejects `deadline_exceeded` / `frame maintenance deadline expired`. The terminal slice now ends at `pub const WORLD3D_DEADLINE_CAPACITY` after the function, because `enum RuntimeApply` can appear earlier in the concatenated renderer text. The mounted self-test mutation `maintenance-terminal-keeps-deferred-work` and the cancel mutation use the current source text. The Rust source pin in `dispatch-cursors-frame-deferred-and-maintenance` asserts the expired string is absent from both the mailbox file and the classifier file. Not compiled (`repo cargo` withheld for the blit.wgsl landing).
- Not touched: renderer asset decode fault (audit 5.3) and enqueue ordering (5.5).

## 4. Tests

- `E/🧪️tests/🚦️frame-demand-and-traces/🦀️.rs` (mounted in `🚦️frame-job` crate, package `semio-renderer-wgpu-frame-job`): fixture rows, each flag alone demands a build, idle demands none, source pins of the gate and both emitters.
- `E/🧪️tests/🚦️frame-demand-and-traces/🟦️.ts`: independent TypeScript derivation of the same oracle.
- `E/🧪️tests/🔬️wgpu-renderer-async-boundary/🚦️dispatch-cursors-frame-deferred-and-maintenance/🦀️.rs` (package `semio-renderer-wgpu-runtime-mailbox-tests`): `exhausted_interaction_credits_defer_the_deferred_apply_without_a_frame_fault` (behavioural) and a source pin that the three fatal strings are gone.
- Third-party cross-check (CLAUDE.md) not added: no external library models these traces.

## 5. Step B: probe and battery

- `T/🐍️wgpu-frame-loop-probe.mjs`: after the gesture ladder, drain (`SEMIO_PROBE_IDLE_DRAIN_MS`, 4000), record `idle_start`, no input for `SEMIO_PROBE_IDLE_MS` (8000), record `idle_end`.
- `T/🐍️wgpu-battery.mjs` frame-loop verdict: new steps "idle window sends no batches" (delta <= 1), "idle window builds no frames" (admit delta <= 1), "the last frame after idle requests no further frame". Existing four steps unchanged. Both files pass `node --check`.

## 6. What was not done

- Compilation and Rust tests: every `mcp__repo__cargo` call for `semio-renderer-wgpu-frame-job` failed in the dependency `wgpu 29.0.4` with `couldn't read .../src/util/blit.wgsl` although the file exists on disk (environment of the cargo gate, not this change). A first attempt on `semio-renderer-wgpu-host-winit` failed earlier on its source boundary (262144 bytes); the demand code was moved into the frame-job crate for that reason, so the host-winit crate is now only slightly larger than before and must be re-checked.
- Step E (earlier): server on 6118 down and the battery was not run. R12 and item 7 are corrected in `battery-readiness-audit.md` to cite `wgpu-frame-loop-after-selection-2026-09-13.md` §4, `wgpu-frame-loop-quarantine-audit.md`, and this note. The row is not marked green.

## 7. Evening pass (2026-11-22): deadline lane and law tests

- `host-winit` was not edited. Its boundary stays the previously measured production closure (6 files, 134708 bytes, 2317 active lines), under 262144 / 32 / 4000.
- TypeScript: `repo tools exec --light bun -- test` on `deadline-maintenance-law.test.ts` — 1 pass, 0 fail, 12 expects, exit 0. It reads the frame-actions and mailbox sources through `policyReadRustPolicySource` and checks the two-argument classifier, the defer arm, and the absence of `frame maintenance deadline expired`.
- The mounted self-test `interactivity-mounted-frame-transaction` still throws before the mutation loop: `P5a duplicate Rust block scope did not emit one named stale-boundary failure`. The shell policy read (partition crates are not followed) does not contain a line-starting `fn render_navbar_step`; production is `pub fn render_navbar_step` in the `shell-view-chrome` crate. That preflight is independent of the deadline pins. Glue for that self-test now also includes `frame-actions` and `mailbox-impl` so the deadline text is in the source the audit scans once the preflight is fixed.

### P5a recheck (2026-11-22, later the same evening)

The shell policy read of the shell audit file, with partition crates enabled, already includes the shell-view-chrome crate root and the shell-view-chrome-overlays crate. Production contains exactly one `pub fn render_navbar_step` (one hit in 3059143 bytes of shell evidence). The missing/duplicate block-scope preflight runs and does not throw.

`repo tools exec --light bun -- test` on `interactivity-mounted-frame-transaction`: 0 pass, 1 fail, 14.3s. The failure is after that preflight: 43 mutations did not alter source, and the clean baseline is still rejected across transaction, maintenance, chrome, paint, atlas, and host checks. Those needles are absent from the glued production text. The navbar anchor does not make the mutation battery green.

## 2026-11-23 p5a: mounted frame transaction law re-anchored, nine real violations remain

Command: `repo tools exec --light bun -- test <abs path of interactivity-mounted-frame-transaction/🟦️.ts>` (a relative operand is refused by the wrapper; use the absolute path). The previous failure at `🟦️.ts:147` was not one defect but two layers.

### Root cause 1: test drift (law correct, sources stale)

The Rust renderer was carved into partition crates and sub-modules after the law was written, so the law read almost none of the production text it audits:

- `policyReadRustPolicySource` only follows `#[path]` children of the split roots. The glue crates (`frame-build`, `frame-transaction`, `app-runtime`, `frame-actions`, `runtime-apply`, `mailbox-impl`, `presentation-raster`, `frame-presentation`), `frame-job` and `winit-app` children, the Shell view crates, `retained-paint-walk-and-hits`, `prepared/📤️staging`, `draw/🔧️pipeline`, the EngineCanvas and World3D crates were invisible; `🧊️gpu/🦀️.rs` no longer exists. 43 mutations "did not alter source", and the baseline was rejected by about 60 absent needles.
- The law-name needles live in separate test crates (`runtime-mailbox-tests`, `shell-tests-chrome-*`, `🧪️tests/**`) that the evidence reader never mounts.
- Boundaries were cut with `indexOf(a)..indexOf(b)` across the joined text, which is order dependent and silently yields empty or 2 MB slices once code moves (transaction, frame cursors, maintenance owners, atlas, UI frame, scene host, find, maintenance).
- Needles named API that was renamed or generalised with the same meaning: `FrameMaintenanceExecutionRegistry` -> `FrameExecutionRegistry<T>`, `FrameMaintenanceOwnerCell` -> `FrameExecutionOwnerCell<T>`, `try_take_abandoned` -> `try_take_returned`, `pop_front` now returns `FrameActionEnvelope`, `next_abandoned_close_demands(4096)` -> `(grant.maximum_copy_bytes)`, `retire_step()` -> `retire_granted(grant)`, `push_page(.., context.retained_grant())`, `ui_wgpu::wgpu::*` -> `semio_ui_prepared` / `semio_ui_wgpu_paint` / `semio_ui_draw_contract`, atlas slots and pages via `new_uninit` (still after the process permit), `OriginalFixedSlotPages` world slots, `StagedEngineScenes`, `try_admit_owned` -> `original_admission.advance`, `take_rejected` removed, presentation moved to `frame-presentation` (`AppPresentStep::Complete{..theme_dark..}`, `publish_on_original_caller`), `wgpu::Origin3d` -> `gpu_device::Origin3d`, `pub(crate)` -> `pub` after the crate split.
- The native preference law forbade `OS_SHELL_CONFIG_STORAGE_KEY`, `.lock()` and `loop {` in the whole native page text, while the same law already required the bounded host-storage document read. That contradiction encodes the `terra-native-preference-cap-contract-audit.md` design (64 KiB config document through the host-storage document service, 4 KiB single-page field route). The law now splits the config branch from the field-page route: field pages keep every old prohibition, the config document must keep `OS_SHELL_CONFIG_MAX_BYTES` admission, staged `write_step`, atomic `publish`, `cancel`, and no `serde_json`, `vec!`, `std::fs`.

### Fix (no Rust touched)

- `🧪️tests/🔬️interactivity-mounted-frame-transaction/🟦️.ts`: new `interactivityMountedFrameSources` reads each of the 17 owner sources as module tree plus crates (`policyModuleTreeFiles`, files joined behind `INTERACTIVITY_AUDIT_FILE_MARKER`), directory families for the UI crates (`prepared`, `draw`, `⚙️engine`, `🖌️paint`, `📨️scene_slots`), the Shell overlay crate, and a test corpus per domain behind the evidence boundary. The mutation battery now measures only failures that are new against the baseline, rejects a mutation whose needle first occurs in test evidence (except `-law` mutations), and replaces every occurrence. Seventeen stale mutations were re-derived against current code (renamed `missing-take-rejected` to `reintroduced-take-rejected`, `missing-max-identity` to `missing-max-identity-law`; `postallocation-credit` now targets the World3D slot, `whole-chrome` the glue call site, `missing-paint-byte-cursor` the real byte authority).
- `⚖️laws/🔬️interactivity-audit/🖥️mounted/🎞️frame-transaction/**`: boundaries re-anchored with `rustItem`, `rustItems`, `fileText`, `splitRustBranches`; renamed needles updated; forbidden-token scans of the transaction and deferred boundary now ignore comments and strings (doc comments containing "while " were false hits); `pub(crate)` needles accept `pub`; native close law is "begin_close is not test-only"; config-document law added; the weak `byte: usize` needle (satisfied by `measure_byte: usize`) is now `pub(super) byte: usize`.
- `🗂️files/🟦️.ts` and `📦️packages/🟦️typescript/⚖️laws/🟦️.ts`: new `INTERACTIVITY_AUDIT_FILE_MARKER` export.

### Result

Mutation battery: all 84 source mutations (and 3 legacy chrome mutations) now alter production source and are detected as new failures. The preflight (navbar, tutorial, tooltip scopes) passes. The test is still red, deliberately, with exactly nine baseline findings that are real implementation deviations from the P5a law, not drift. They need Rust fixes by the crate owners (not done here: Rust crates are being compiled by other agents):

1. `🧾️frame-build/🧮️frame-transaction/🦀️.rs` ~209 and ~322: `app.interaction.as_mut().expect("runtime interaction state is worker-owned")` in the mounted transaction (panic; use `let Some(..) else { Fault }`).
2. Same file ~243: `self.scene_camera_cursor.as_mut().unwrap()` (camera original admission).
3. Same file ~279: `self.brush_mesh_cursor.saturating_add(1)` (BrushMesh phase; law requires `checked_add` fail-closed).
4. `🛎️frame-actions/🔁️runtime-apply/🦀️.rs` ~307: `.completions.lock().expect("runtime completion mailbox lock")` in the deferred refusal arm (panic on poison).
5. Shell chrome `FrameSetup` phase (`view-panels/🖼️chrome-frame-and-main-window`): four whole-collection `.clear()` calls (`retained_body_rects_staging`, `retained_hit_windows_staging`, `retained_scene_hits_staging`, `with_chrome_control_names`) in one grant.
6. `retained_chrome_group_item_width` and `render_retained_chrome_group_item_step_with_trailing`: whole-label `atlas.measure_text(label, ..)` up to `RETAINED_NODE_TEXT_MAX_BYTES` (4 MiB) in one grant.
7. `render_footer_step` phase 2: `Vec::new()` plus `while let .. footer_tab_row_rect(..)` over all footer tabs in one grant (shared-border plan).
8. `interactive-state-sync` select/external-slot/picker phases: `item.value.clone()`, `slot.body_key.clone()`, `params_json.clone()`, `input_kind.clone()` of unbounded strings in one grant.
9. Same file: sibling-walk `while let Some(candidate) = child` scans in `SelectScan`/`ExtRowScan`/`PickerScan` (O(children) per grant).

Neighbours in the same `🧪️tests` directory (`interactivity-live-reconcile`, `interactivity-mounted-engine-surface-lifetime`) register no bun test; run as self-test functions they fail with the same kind of drift (`per-surface-credit-cap made no source mutation`, `wrapping-cpu-generation did not bind live source`). They read none of the files edited here and were not changed.

## 2026-11-23 p5a Rust fixes

The nine P5a findings are fixed in Rust without touching the law. `bun test` of `🧪️tests/🔬️interactivity-mounted-frame-transaction/🟦️.ts` (via `repo tools exec --light bun -- test <abs path>`) is green: 1 pass, 0 fail.

1. `🧾️frame-build/🧮️frame-transaction/🦀️.rs`: both `.expect("runtime interaction state is worker-owned")` became a `match app.interaction.as_mut()` that answers `Err("runtime interaction state lost its worker owner")`, which the existing `Err(fault)` arms turn into `record_frame_fault` plus `Terminal`/`Fault`.
2. Same file: `scene_camera_cursor.as_mut().unwrap()` became `let Some(cursor) = .. else { record_frame_fault; Terminal; Fault }`.
3. Same file: `brush_mesh_cursor.saturating_add(1)` became `checked_add(1)` with a typed fault ("frame brush mesh cursor exceeded index authority").
4. `🛎️frame-actions/🔁️runtime-apply/🦀️.rs`: the deferred refusal arm no longer panics on a poisoned completion mailbox; `lock().unwrap_or_else(PoisonError::into_inner)` still cancels the interaction reservation.
5. Shell chrome `FrameSetup` (`🪀️view-panels/🖼️chrome-frame-and-main-window`): the four whole-collection `.clear()` calls are removed from setup step 0 and drained one entry per grant by new setup steps 20..23 (`Vec::pop`, `extract_if(..).next()`, `BTreeMap::pop_first`); the `MainWindow` hand-off moved to step 24.
6. `🍱️view-chrome/🧱️chrome-paint-and-navbar`: the label span after the glyph walk goes through `note_chrome_label_span` instead of a local `measure_text`; new `RETAINED_CHROME_GROUP_LABEL_MAX_BYTES = 4096` in `🎛️prefs/🧱️chrome-paint-and-navbar` bounds both `retained_chrome_group_item_width` (returns `None`) and the group step (returns `Fault`), so every measurement in a grant is bounded by 4 KiB; the law's 4 MiB `RETAINED_NODE_TEXT_MAX_BYTES` needle stays.
7. `🪀️view-panels/🧭️navbar-footer-and-overlays` `render_footer_step` phase 2: the `Vec::new()` plus `while let` sweep is now one footer tab per grant into new `ShellChromeChildCursor` fields `shared_border_frames`, `shared_border_sweep`, `shared_border_planned` (declared in `🍱️view-chrome/🖼️chrome-frame-and-main-window`); the border plan is built once the sweep ends and the fields reset when phase 2 ends.
8. `🔄️interactive-state-sync`: `item.value.clone()` is a `RetainedSyncKey` bounded copy (over 256 bytes the popup row is skipped via `retained_sync_select_skip`); the external-slot and picker scans compute their counts by borrowing, no `body_key`, `params_json` or `input_kind` copy.
9. Same file: the icon-cell, external-row and picker sibling-walk loops are new `IconCellFind`, `ExtRowFind`, `PickerFind` phases that compare exactly one sibling per grant through `cursor.child_scan`; a doc comment containing the word "while" was reworded because the law scans raw text.

Verification: `cargo check` of the `🖌️paint` package (contains `🔄️interactive-state-sync`) finished clean via `repo cargo --store=shared --configuration=prototype`. The frame-transaction, runtime-mailbox (hosts `runtime-apply`) and Shell crates could not be checked: the frame-transaction check stopped on two errors in code other agents are editing (a `[DEBUG]` line in `🌱️value/🌳️value/🔀️variant/🦀️.rs` using an unimported `NativeRecordKey`, and `wasmtime-wasi-io` failing to find its WIT directory). Those edits parse with `rustfmt` and follow existing in-file patterns, but their type check is still open; rerun `p5a-rust-check.sh` (this folder) once the value crate compiles. Logs: `🗑️generated/p5a-check-🧾️frame-transaction.txt`.
