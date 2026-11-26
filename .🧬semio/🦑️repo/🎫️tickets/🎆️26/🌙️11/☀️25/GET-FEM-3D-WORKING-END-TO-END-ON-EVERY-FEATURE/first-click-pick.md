# F-K first click does not select (item 37)

## Verdict
The pick itself is not stale. Padded pick resolves the right member on the first click every time (instrumented in the browser, Demo model, left viewport, 80 ms hold after a 1.5 s hover). The selection becomes visible in the Inspection tree only about 1.1 to 1.5 s after pointer-up, so a reader that samples after about 1 s sees the previous click's result: for the sequence beam@A, beam@A, col@B, col@B, beam@A it reads [], A, A, B, B, which is exactly the reported F-K list shifted by one click.

## Measured chain (per click, before the fix)
- pointer-up: padded pick (`resolveClickInstanceId`) resolves instantly and dispatches `interactionSelect`.
- The r3f mesh `onClick` (`instance-node`, `onInstancePointerDown`) dispatched the SAME `interactionSelect` again 1 ms later. Two round trips queued: settled +435 ms and +746 ms.
- Inspector shows the new id 1.2 to 1.6 s after pointer-up (5 samples, 1083 to 1575 ms).

## Root cause fixed (TS)
A real raycast hit selected twice per click, once by the padded pick on pointer-up and once by the mesh click, because `paddedPickReleaseRef` only suppressed the `onPointerMissed` path. The duplicate queues a second guest turn behind the first and delays the settle.
Fix: the padded pick now remembers `{ id, timeStamp }` (`paddedPickRef`); the mesh click goes through `handleInstanceClick`, which skips only a click that repeats the padded pick of the same instance inside `WORLD3D_CLICK_AFTER_RELEASE_MS` (`world3dClickRepeatsPaddedPick`, `🔗️instance-selection`). A click whose true raycast hit differs from the padded pick still dispatches (the later, real hit wins).

Browser after the fix: exactly one `interactionSelect` per click, settle +329 to +476 ms (was +435 and +746). Inspector latency is still about 1.1 to 1.4 s (1083, 1208, 1212, 1364, 1420 ms): the remaining ~0.8 s is after the select turn settled (inspection publication), outside the pick code, not changed here (Rust/shell publication, needs wasm rebuild and profiling). Until then drivers must wait for the Inspection tree to change instead of sampling at a fixed delay.

## Pick tolerance (viewport-robustness item 1) still holds
Same `resolveClickInstanceIdFromProjected` and 4 px tolerance, untouched; law green. Browser: B+3 px x selects fb1_0, A+3 px y selects fb2_3, a click 40 px off the beam selects the member actually there (fb2_0), never a stale one.

## Laws
`vr-vitest.config.mjs` (picking-and-previews + pick-bounds): 31 tests pass, 0 fail. New: `dispatches one selection for a click whose padded pick on release already selected that very instance` in `🧪️tests/🌍️world3d-picking-and-previews/🔶️raycasts-z-0-ground`. It failed first (`world3dClickRepeatsPaddedPick is not a function`), then passed.

## Files
`🌐️World3dHost/🔗️instance-selection/🟦️.ts` (new `world3dClickRepeatsPaddedPick`), `🌐️World3dHost/👆️pointer-and-drop/🔸️handle-pointer-up/🟦️.ts` (`paddedPickRef`, `handleInstanceClick`), `👆️pointer-and-drop/🟦️.ts` (pass-through), `🌐️World3dHost/🟦️.tsx` (uses `handleInstanceClick`), test file above. Temporary `[DEBUG]` logs were removed.

## Item 3 (`retrySupersededHistoryAction`)
Not present in the tree any more: no match for it or for `history-supersede` anywhere under `🧰️framework` (`🚀️program-dispatch/🟦️.ts` has no retry), so there is nothing to remove. The stale-root cause (`interactive-job.stale` for undo/redo in `typed-operation-unit`) still needs a Rust rebase onto the live root, not done here (no wasm rebuild).
