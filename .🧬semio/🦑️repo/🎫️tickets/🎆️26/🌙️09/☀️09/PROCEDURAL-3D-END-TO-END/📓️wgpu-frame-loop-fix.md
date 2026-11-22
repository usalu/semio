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
