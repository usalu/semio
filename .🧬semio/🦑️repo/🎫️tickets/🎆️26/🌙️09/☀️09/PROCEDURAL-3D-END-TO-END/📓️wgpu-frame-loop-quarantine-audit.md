# WGPU frame-loop row: quarantine audit (2026-11-22)

Read-only audit. No repo edits, no git writes, no builds, no servers.

Paths below are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/` unless shown otherwise. `T` is this ticket.

## 1. Verdict

- **The battery does not quarantine the row.** `T/🐍️wgpu-battery.mjs` has no skip list, no quarantine flag and no `--only` default that excludes `frame-loop` (PROBES entry at line 243). The word "quarantin" appears only in `FAULT_RE` (line 43), which scans for the runtime fault.
- **"Quarantine" in the audit means the runtime quarantine**: the renderer's `frame-credits` fault closes the surface for good (`🚚️browser-frame-transport/🟦️.ts` `quarantine()`). That is a product defect, not a battery decision.
- **The quarantine cause has a recorded fix.** `T/📓️wgpu-frame-loop-after-selection-2026-09-13.md` (section 4) records it. The audit row R12 (`battery-readiness-audit.md` line 89, and item 7 at line 122) says "no note records a fix". That is wrong.
- **The fix is in source** for the main path (verified by grep, not by running it): `INTERACTION_CHECKOUT_CREDITS` is gone; admission is owner-based.
- **The row has never produced a verdict in this ticket.** `T/🗑️generated/wgpu-verify/` does not exist, so there is no `scoreboard.json` and no `frame-loop/results.json`.
- **The probe grades on two traces that current source never emits**, so the row would be red even if the loop works (see section 3).

## 2. What the probe asserts (`T/🐍️wgpu-frame-loop-probe.mjs`, battery verdict at battery lines 244-268)

Battery steps:
1. at least 20 gestures run (step names `g\d+_`);
2. zero wire quarantines (`frame.quarantined === true` across all samples);
3. the wire is at most one batch ahead of frames, with no fault, on the final sample;
4. the loop keeps publishing actions (`actions > 0`, counted from the `frame input action` log line).

Not asserted anywhere: idle frames stop, frame pacing, redraw on demand. The probe records `requestFrameFalse` and `wakes` in `wireVerdict()` but never grades them. The 2026-09-13 note (section 7.1) states the idle frame-build spin (about 20 builds per second on an idle shell) is still present.

## 3. Trace dependencies that do not exist in source

| Probe reads | Where | Source status |
|---|---|---|
| `frame build admitted` (`admits`) | probe line 139 | Only in a doc comment (`winit-app/🎞️frame-generation-and-os-host/🦀️.rs:223`). No `log_debug` emits it. |
| `frame input action` (`actions`, battery step 4) | probe line 139 area, battery line 262 | Only in doc comments (`app-runtime/🦀️.rs:499-500`) and in fault strings `frame input action batch ...` (`frame-actions/🦀️.rs:259-461`). No `[TRACE] frame input action ...` emitter. |
| `wgpu-shell render leave surface=` | probe `renderSweeps()` | Emitted (`🧱️elements/🐚️Shell/.../window-actions/.../🦀️.rs:167,222`). Surface id matching `framework.panel.history` still unverified. |
| `"selected":true` | probe `appliedSelections()` | No emitter found in non-test source. Count is informational only. |
| `frame fault recorded`, `boot_shell leave`, `wgpu-shell dock plan`, `world3d surface=` | probe, battery | Emitted (`mailbox-impl/⏳️pending-work-deadlines-and-spawns/🦀️.rs:117`, `browser-worker/🦀️.rs:840`, `chrome-frame-and-main-window/...:241`). |

So step 4 ("the loop keeps publishing actions") reads 0 by construction. The 2026-09-13 note (section 6.1) reports `frame builds admitted` and `frame input action … action=interactionSelect`, so those traces existed in a build that is no longer the source.

## 4. What is fixed (verified in source)

- `📮️runtime-mailbox-core/🦀️.rs`: `interaction_owner_outstanding()` (line 104) counts an in-flight reservation, a returning checkout, or a ready completion with `restores_interaction`. `admit()` (line 207) returns `Deferred` while an owner exists, `Abandoned` otherwise. Credits constant deleted.
- `🧊️renderer/🧠️app-runtime/🦀️.rs:48`: `checkout.admit(requires_interaction, available, owner_outstanding)`.
- `🧊️renderer/📮️mailbox-impl/🦀️.rs:98,144`: owner predicate passed to admit.
- `🚚️browser-frame-transport/📋️contract/🟦️.ts:61` and `🟦️.ts:686`: `frame-credits` maps to `worker-frame-failed`, not `worker-step-overrun`.
- `🚀️browser-boot/🟦️.ts` and `🌐️browser-host/🟦️.ts:153`: `renderFault` writes `console.error("wgpu renderer fault: ...")`.
- Law fixture and tests exist (`🧪️tests/🎮️wgpu-runtime-mailbox-admission/`, `🧪️tests/🔬️wgpu-runtime-mailbox-core-unit/`). Not run here.
- Sustained-overrun does not quarantine: `🎞️frame-worker/🎞️frame-and-introspection/🟦️.ts:38` only drops media slots.

## 5. What is still open (can still quarantine the surface)

1. **Time-based fatal lane**: `📮️mailbox-impl/⏳️pending-work-deadlines-and-spawns/🦀️.rs:315-317` sets `frame_fault = "frame maintenance bounded field exceeded its deadline"` when a deadline passes. Any frame fault becomes a quarantine. This is the same class of defect the fix removed, and the probe would catch it under load.
2. **Capacity refusals are fatal**: `🛎️frame-actions/🔁️runtime-apply/🦀️.rs:227,259` set `frame_fault` when `reserve_interaction_future()` fails (`runtime dispatch completion credits exhausted`, `frame deferred completion credits exhausted`). `reserve_interaction` (`📮️runtime-mailbox-core/🦀️.rs:74`) returns false when `len() == CAPACITY`. This is a real capacity bound, not age, but a long settle with input can hit it.
3. **Renderer asset decode fault**: `📮️mailbox-impl/📥️renderer-asset-steps/🦀️.rs:186` calls `record_frame_fault`.
4. **Frame-build spin** (2026-09-13 note 7.1): unfixed. Drives wasted work and idle non-stop.
5. **Enqueue ordering** (2026-09-13 note 10.4): `enqueue_runtime_completion` publishes before invalidating. Has a failing test (`🧪️tests/🔬️wgpu-renderer-async-boundary/`), no owner.

## 6. Fix plan for the execution agent

Ordering matters: make the probe honest first, then run it, then fix whatever it finds.

**Step A. Make the probe read real traces (no behaviour change).**
- Files: `🧰️framework/…/🧊️renderer/🧾️frame-build/🧮️frame-transaction/🦀️.rs` (frame admission site, find the function that begins a build and logs nothing today) and `…/🧊️renderer/🛎️frame-actions/🦀️.rs` (where a dispatchable input action is published to the frame).
- Change: emit `log_debug("[TRACE] frame build admitted generation={g}")` and `log_debug("[TRACE] frame input action action={id} …")`. Use the `[TRACE]` prefix already used by neighbours. Do not add comments inside definitions.
- Test: a Rust unit test that drives one admission and asserts the trace is captured through the existing log hook (look at how `frame-action-ledger` tests observe logs). Plus a language-agnostic fixture row in `🧑‍🎨engine/🧫️fixtures/` for "a publishable action emits exactly one `frame input action` line".

**Step B. Grade the missing behaviour in the probe and battery.**
- File: `T/🐍️wgpu-frame-loop-probe.mjs`. After the gesture ladder add an idle window (for example 8 s, no pointer nudges). Record `wire.batches` and `admits` at the start and end of the window.
- File: `T/🐍️wgpu-battery.mjs` frame-loop `verdict` (lines 248-268). Add steps:
  - "idle window sends no batches" (batch delta over the idle window is 0, allowing one trailing batch);
  - "idle window builds no frames" (admit delta is 0 or at most 1, which tests §5.4);
  - "every sample requests no frame after idle" (last `requestFrame` is false).
- Keep the existing four steps unchanged.

**Step C. Fix the idle spin (only if Step B is red).**
- Files: the frame-build scheduling in `🧾️frame-build/` and the `requestFrame` computation in the frame worker (`🎞️frame-worker/🎞️frame-and-introspection/🟦️.ts` line 36-41 and the `result.requestFrame` source in `🌐️browser-worker/🦀️.rs`). Find why `requestFrame` stays true with nothing pending, and make it false when no input, no deadline and no maintenance is pending.
- Test: TS transport test in `🚚️browser-frame-transport/🧪️tests/` (existing transport suite, `nx run @semio-tech/framework-renderer-wgpu:test-browser-worker`) that a `requestFrame:false` reply with `nextDeadlineDelayMs` arms exactly one timer and no rAF.

**Step D. Remove the time-based fatal lane (§5.1).**
- File: `📮️mailbox-impl/⏳️pending-work-deadlines-and-spawns/🦀️.rs` around 315-317. Replace the deadline fault with the same owner rule as the admission fix: a deadline that passes while maintenance still has an owner defers; with no owner it is a defect on the first occurrence. Update `frame_maintenance_terminal_fault` tests.

**Step E. Run the row, then correct the audit.**
- Run: `cd <ticket> && bun 🐍️wgpu-battery.mjs --only=frame-loop` against a served 6118 build (the execution agent starts the server; this audit did not).
- Correct `📓️battery-readiness-audit.md` R12 and item 7 to cite `📓️wgpu-frame-loop-after-selection-2026-09-13.md` §4 and this file.
- Delete `🗑️generated` outputs per ticket rules after reading them.

## 7. Not verified by this audit

- No test, build, or browser run was executed (read-only mandate).
- Whether 6118 currently serves a build that matches the source; the 2026-09-13 note cites its own 22:31 build.
- The `framework.panel.history` surface-id match in `renderSweeps()`.
