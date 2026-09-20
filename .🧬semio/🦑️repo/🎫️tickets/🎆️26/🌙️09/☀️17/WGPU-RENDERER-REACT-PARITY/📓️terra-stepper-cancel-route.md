# Stepper Cancellation Route Audit

Read-only route map for the current first-pass cancellation implementation. No tests, builds, activation, or production files were changed.

## Current typed route

The smallest correct carrier is a buttonless cancellation with pointer identity only. A cancellation must not be represented as `PointerUp`: the retained router commits a primary `NumberStepper` on `PointerDown` and uses `PointerUp` to complete other captured controls.

The present implementation covers the intended browser route:

- Browser capture and lossless admission are in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts:198-206` and `🚚️browser-frame-transport/🟦️.ts:113-127,137-222`. `pointercancel` projects to lossless `pointer-cancel`; it carries no `button`.
- The Worker decodes and projects it at `🎮️input-wire/🦀️.rs:29-36,210-220`. Its preflight `_` arm at `🌐️browser-worker/🦀️.rs:621-681` admits it as a discrete event, so no new Worker branch is needed.
- The normalized model is `DispatchEvent::PointerCancel { pointer }` at `🧰️framework/🔨️modules/🖱️ui/🖌️render/🖱️dispatch/🦀️.rs:412-435`. Its general dispatcher already removes pointer-keyed capture, origin, and thumb state at `🖌️render/🖱️dispatch/🦀️.rs:1347-1365`.
- The mounted WGPU host routes cancellation directly, not through `handle_pointer_button(..., false, ...)`, at `🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:343-358`. Its native touch mapper also separates `TouchPhase::Cancelled` at `🪟️winit-app/🦀️.rs:735-743`.
- Renderer cancellation clears the outer owner and calls the Shell cancellation entrypoint at `🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:15502-15510`. Shell routes `UiEvent::PointerCancel` to the captured retained window and clears its own drag/release state at `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12293-12311`.
- Retained cancellation releases capture, removes `ACTIVE`, clears drag/origin/thumb state, emits only `DropCancelled` for an existing drag, and emits no value or action command at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs:1821-1839`. The Escape path now uses the same event at `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15151-15160`.

Confidence: high. The source chain is typed end-to-end and its cancellation branch cannot enter the boolean release/action path in `route_retained_pointer_press` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12930-12991`).

## React Stepper oracle

The React Stepper applies one increment/decrement during mouse-down, then starts a timer. `applyDelta` sends `onDelta` or `onChange` once at `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪜️Stepper/🟦️.tsx:86-95`; `handleMouseDown` invokes it before scheduling repeat at `🪜️Stepper/🟦️.tsx:146-160`. The 500 ms / 100 ms repeat changes only local state (`🪜️Stepper/🟦️.tsx:97-112`). Mouse leave stops repeat, clears active interaction/editing, and calls `onPointerCancel` at `🪜️Stepper/🟦️.tsx:163-179`.

There is no global `disabled` or `readOnly` Stepper prop. Its two segment buttons are disabled only at their min/max bounds (`🪜️Stepper/🟦️.tsx:181-205`); the numeric input remains editable. The retained WGPU route matches the one authored increment on primary press at `⚡️events/🦀️.rs:1916-1918`, then intentionally skips an extra NumberStepper release commit at `⚡️events/🦀️.rs:1968-1973`.

Confidence: high.

## Remaining concrete work

1. **P1 — add browser-wire oracle coverage.** The implementation admits `pointer-cancel`, but the shared fixture has no row and its two consumers still assert the old vocabulary.
   - Add a `pointercancel` DOM / `pointer-cancel` wire / identity-only dispatch row to `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🎮️wgpu-browser-input-wire/🔣️.json`.
   - Extend the exact TypeScript kind set at `🧪️tests/🎮️wgpu-browser-input-wire/🟦️.ts:82-88`.
   - Extend Rust `dispatch_json` at `🧪️tests/🎮️wgpu-browser-input-wire/🦀️.rs:63-81`; it otherwise falls through to the debug-string `other` arm and cannot match the fixture dispatch.
   - There is no separate JSON schema for this wire fixture; the fixture plus those Rust/TypeScript twins are its neutral contract.

2. **P1 — exercise the already-supported Stepper cancel terminal.** `stepper-pointer-commit` already permits `terminal: "cancel"` in `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🪜️stepper-pointer-commit/🔣️.json`, and both consumers already branch on it: React at `🧪️tests/⚙️puzzle3d-settings-document/🟦️tsx:23-50` and WGPU at `🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs:273-304`. Its fixture contains only release cases at `🧫️fixtures/🪜️stepper-pointer-commit/🔣️.json`. Add one `plus-cancel` case with `pressActions: 1`, `terminalActions: 0`, and `deltaSteps: 1`; no schema or consumer logic change is needed.

3. **P1 — graph cancellation has no ownership primitive.** `AppInteractionState::handle_pointer_cancel` only calls Shell cancellation and World3d relocate cancellation (`🧊️renderer/🦀️.rs:15502-15510`). Shell cancellation does not invoke a node-graph function (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12295-12311`). Node graph exposes only `node_graph_pointer_down_into`, `_move_into`, and `_up_into` (`🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:3351-3438`). Its Flow and Dag hosts map both `DagPointerPhase::Up` and `Leave` to `pointer_up_screen` (`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:3600-3613`), so either existing terminal would manufacture the very release semantics cancellation must avoid. A graph press followed by `pointercancel` receives no graph-side terminal event today. Do not reuse `_up_into`; this needs an owned graph cancel operation or an explicit noncommitting Dag phase.

4. **P1 — board has a release primitive, but cancel does not reach it.** `release_board_pointer_claim(surface_id, input)` clears the retained board claim at `🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4833-4844`; the frame transaction uses it only when the board authority itself returns `Cancelled` (`🧊️renderer/🦀️.rs:12940-12953`). `ShellState::handle_pointer_cancel` clears `input.drag` but neither iterates board surfaces nor calls that function (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12295-12311`). A cancellation during a retained board pointer sequence therefore has no direct claim-release route. Reuse `release_board_pointer_claim` per active board surface; do not send `puzzle_board_pointer_up_into`, which plans `BoardPointerPhase::Up` at `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5023-5031`.

5. **No Ink/Canvas gap found.** `scenes::cancel_canvas_interactions` is the existing cancellation owner (`🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1736-1739`). Its pointer cancel path emits the canvas release wire with the cancellation flag and retires its active state at `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1335-1367`; Shell already calls it at `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12299`.
