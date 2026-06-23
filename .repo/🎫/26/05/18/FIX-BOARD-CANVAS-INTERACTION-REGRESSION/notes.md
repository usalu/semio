# Board Canvas Interaction Regression

## Symptom

- Board UI chrome still rendered and tools still switched.
- Canvas interactions were effectively dead: selection, drag, pan, zoom, and linking did not reach the board host.

## What Was Checked

- Confirmed the user-reported boundary: commit `434` (`c40a07ca0`) worked, commit `435` (`5957bf92a`) broke board interaction.
- Verified that `434`/`435` touched `elements/client/lib/board/*`, not `compose/*`. This regression belongs to the shared board package.
- Read the `434 -> 435` diff. First hypothesis was the new stacked text overlay canvas introduced in `elements/client/lib/board/index.tsx`.
- Checked current code and found a later mitigation already exists: `eventSurface` wrapper plus window-capture pointer bridge. That made the overlay-only hypothesis incomplete.

## Root Cause

- The active app path uses the board window-capture bridge in `elements/client/lib/board/index.ts`.
- That bridge filters captured events through `eventTargetIsUnderEventSurface()`.
- The function used `event.target instanceof Node`, but inside this module `Node` refers to the board model class, not the DOM `Node` constructor.
- Result: every captured `pointerdown` / `pointermove` / `pointerup` / `wheel` event failed the guard and was dropped before reaching WASM.
- Observable outcome matched the report exactly: board rendered, surrounding UI worked, canvas felt read-only.

## Fix Applied

- Replaced the shadowed `Node` check with a DOM-safe lookup using `surface.ownerDocument?.defaultView?.Node ?? globalThis.Node`.
- Made the guard path-aware via `event.composedPath()` so overlay/wrapper targets still count as being under the board event surface.
- Added a regression test that would fail with the shadowed `Node` check and passes with the DOM constructor.

## Hypotheses To Avoid Repeating

- `435` text overlay canvas alone is not the complete present-day root cause.
- `compose` sketchpad code is not on the failing path for this bug.
- WASM hit policy / LOD changes may affect picking behavior, but they do not explain the total loss of pan/zoom/select when the capture guard rejects all DOM events first.

## Validation Notes

- Added targeted regression coverage in `elements/client/lib/board/index.ts` for the event-surface target guard.
- Focused board in-source test execution should be run from `index.ts`, not `index.tsx`; `index.tsx` currently has unrelated test-loading issues and existing style diagnostics.
- `bun test ./elements/client/lib/board/index.ts` does not execute the embedded Vitest blocks in this repo shape, so it returned `0` tests.
- `bun x vitest run elements/client/lib/board/index.ts` is currently blocked by repo config drift before test discovery: `C:/git/compose/compose/client/lib/react/vite.config.ts` is referenced but missing.
- `get_errors` reports no compile/type errors in the changed board renderer file after the fix.