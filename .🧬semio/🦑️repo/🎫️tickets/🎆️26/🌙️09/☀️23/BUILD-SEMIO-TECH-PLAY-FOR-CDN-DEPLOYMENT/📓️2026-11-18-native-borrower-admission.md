# Native borrower admission waits instead of refusing

Failure: `Native borrower operation admission closed or full` after ~2h; ~70 components with several post-link stages nested under one borrower exceeded the 32-entry pending table.

Change (`💾️storage/📥️borrower/🟦️.ts`):
- `Scope.top` bounds outermost operations separately; the existing arrival-order queue/pump now keys on `top`.
- `Frame.queue` + `pumpFrame`: a nested call into a full, live, accepting frame queues in that frame and is admitted FIFO when a sibling settles. It waits only on siblings, never on ancestors. A frame drains its queue before it dies.
- Nested admission still refuses when the frame is dead or not accepting; after stop/close, outermost births are refused by the unchanged closed-borrower law, live nested work finishes.

Data law (`🧹️cleanup/📋️state/🟦️.ts`, operation schema Case, corpus): `pending` counts only the entered table; new `waits = (topOpen||nestedLive) && pending>=32`; `accepts` unchanged. SQLite oracle test and corpus updated.

Tests (bun, run directly, `SEMIO_TEST_ARTIFACT_DIR` set; no script envelope needed): borrower tests 8/8 (new: 100 nested in one frame, 70 nested outliving frame body, 100 outermost start in arrival order, stop refuses new outermost), operation law 11/11, cleanup state 26/26, storage 6/6.
