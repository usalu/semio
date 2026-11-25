# Browser E2E v9: verification on the current hub (boot fixed)

## Result: NOT green. Boot, example switches and timing work; three problems found.

Hub: current tree (progress changes of v8 plus the brep agent's in-flight cuts and its `[DEBUG]` receiver prints, which flood the console and slow the harness). Scenario files at the ticket root: `process-interactions.json`, `process-load-trace.json`, `process-cancel.json`; runner `run-e2e.sh`; hub scripts `hub-build.sh`, `publish-hub.ts`, `rebuild-and-boot.sh`.

## 1. Full scenario (`process-interactions.json`): exit 1
Run twice. Steps through Demo, No example, Concrete Forest and Viewer pass with no console error; the step "switch back to Editor" and every later step (Actions, Increase, Undo, final state) time out (90 s each) and the final screenshot (`e2e-process-v9.png`) shows an empty shell, "No plugins loaded". On the first run the page had also been reloaded mid-run by another agent republishing the hub. So: switching Viewer then back to Editor on the Concrete Forest document freezes the page (it passed on Demo in v6/v7). Not diagnosed further: the guest is busy with the brep load and the page answers nothing for minutes. A scenario prelude now waits for the initial document (canvas and no `Loading document` text) so steps no longer race the Demo load.

## 2. Banner trace and counters
Observed during a Concrete Forest load (500 ms banner sampling): `Loading document: 129 of 135` for the whole ~90 s the banner was visible, then gone. Total 135 = fold 127 + commit 1 + 8 stages + 2 x members (0: this document has no archive members). `completed` stood at 129, so the stage counter moved to stage 1 only; the member counter cannot move (no members). The long phase is therefore not in the member passes; the stage counter does not advance during it (the ladder updates it only while polling the `AwaitingMembers` and `AwaitReplacement` phases). A real signal for this phase needs the counters inside the initializer / candidate-view drivers (items done and total) which also belong to the brep agent's receiver work. Previously the banner read N of N at once, so the total now includes the post-fold stages, but it still shows no movement in the long phase.

## 3. Cancel replay (`process-cancel.json`)
Cancel replay is clicked while the banner shows 129 of 135: the banner disappears about 90 s later and the example is applied (`Concrete Forest`, one canvas, 60 s later still the same). So the button does not cancel the Concrete Forest load: the load runs to completion. "Cancel replay" is the time-travel replay cancel (label in `🎠️kernel`/`⏪️time-travel` panel), not the document-archive-load cancel; the guest command `CancelDocumentArchiveLoad` exists but the status bar offers no control for it. The product gap: the load needs its own Cancel control wired to `CancelDocumentArchiveLoad`, and the example pick must restore the previous document on cancel. Not implemented in this round. The app stayed usable during the load until the Viewer/Editor steps (item 1).
Also seen: a console error from another agent's edit, `[vite] TypeError: error loading dynamically imported module ...🎨️.css` (CSS hot reload), counted by the harness as a console error.

## 4. Timing (machine load average 10-17)
| Quantity | Now | Before (v8) |
|---|---|---|
| First canvas after page load | 50 s, 70 s, 75 s (three runs) | 35-65 s |
| Concrete Forest pick to banner | about 48 s | about 60 s |
| Banner visible to applied | about 90 s | 100+ s |
| Concrete Forest pick to applied | about 135-140 s | about 180 s and up |
The brep agent is still cutting; re-time after it lands (`STEPS="$(cat process-cancel.json)"` prints the same trace).

## Next
1. Diagnose the freeze after Viewer then Editor on Concrete Forest (guest turn starvation or main-thread work).
2. Add a document-load Cancel control (UI + `CancelDocumentArchiveLoad`) and a restore-previous-document law; then `process-cancel.json` passes.
3. Move the stage/member counters to where the long phase runs (initializer and candidate views) once the brep receiver work lands.
