# Folder Binding Lost and Never "Saved" in the Draw Journey (2026-11-24)

## Verdict
The journey was right. Two product bugs made the `persisted` step time out in `waitSaved`, and one journey bug sits right behind it. The event log did grow (1098 B after attach, about 6.5 KB after redo): the worker's binding was never lost, the shell UI was.

## Cause 1: the shell forgot the attached folder (UI "detached")
`useBrowserActorViewAndContextMenu` (`ShellHost/focus-and-input/registers-and-context-menu`) re-ran its `SET_SYNC_BACKBONE_URI` effect whenever the `session` OBJECT changed. A republished view state (the rectangle utility, a gesture) creates a new `session` object with the same identity. With no pending opening publication the effect computed `runtimeKey = null`, so it dispatched `SET_SYNC_BACKBONE_URI null` and `SET_SYNC_CARD_KIND null`. The card then showed the empty attach form without Detach and without the `actor://` uri.

Evidence (temporary `[DEBUG]` log in the effect, with `performance.now()` against the journey snapshots): effect fired at 1.5 s and 2.4 s (boot), then once at 55.0 s, between the `attach` snapshot (35.5 s) and `rectangle` (55.7 s), always with `runtimeKey null`, `pending false`. Right after attach the card correctly showed Detach (probe `probe-folder-binding.mjs`).

Fix: new pure `shellDialogSessionKeyV1(session)` in `ShellHost/dialog-origin` (equal exactly while `shellDialogSessionIsCurrentV1` holds); the effect now depends on that key instead of the object.

## Cause 2: an acknowledged folder write still read "unsaved"
After cause 1 the card showed `actor://...` but `offline · unsaved` with 0 pending forever. The worker emitted `persisted: true, pendingMutations: 0` (probe log), yet the shell state held `writeFailure: null`. `setStatus` writes `writeFailure: undefined` after an acknowledged write; the worker pack wire (`encodePackValue`) turns `undefined` into `null`; `deriveSavedStateV1` requires `writeFailure === undefined`, so `null` counted as a failure. (Visible in the `[DEBUG]` memo log: `{"persisted":true,"pendingMutations":0,...,"writeFailure":null}` and the card text "unsaved".)

Fix in `backbone/artifact-actor`: `wireArtifactEvent` strips an undefined `writeFailure` from a `status` event; `parseArtifactEvent` reads a `null` one as absent (a twin may serialize absence as null). `deriveSavedStateV1` is unchanged and stays strict.

## Cause 3 (journey bug, provable): second `exportPdf` toggled the form shut
`exportPdf` always clicked `action.exportDocument`. After the first export the action's argument form stays expanded, so the second click collapsed it and the format combobox never appeared (probe: after the click only the `treeitem` existed; the first call listed the whole form). Fix in the journey: click only when the combobox is not visible. No assertion weakened (event-log growth and "saved" checks untouched).

## Files changed
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🟦️.ts` (add `shellDialogSessionKeyV1`)
- `.../🏛️ShellHost/🎮️focus-and-input/⌨️registers-and-context-menu/🟦️.tsx` (effect keyed by session identity)
- `🧰️framework/🔨️modules/🛰️backbone/🎭️artifact-actor/🟦️.ts` (status wire strip and null read)
- `✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs` (idempotent export click)
- New tests: `🧰️framework/🛍️products/💻️os/🧪️tests/🪪️document-backbone-worker-wire/🔹️acknowledged-status/🟦️.ts` (registered in the suite's `🟦️.ts`), `.../📺️renderer/🧑‍🎨engine/🧪️tests/🔑️dialog-session-key/🟦️.ts`
- Probes in this ticket: `probe-folder-binding.mjs`, `probe-journey-binding.mjs` (journey copy with snapshots), `probe-pdf-compare.mjs`

## Tests
- `repo test-file <os>/🧪️tests/🪪️document-backbone-worker-wire/🟦️.ts`: 8 pass, 3 fail. The 3 failures (`admits inference opening only...` 30 s timeout, `projects real browser intent publications...`, `settles only the exact live browser intent mailbox...`) are not in files I touched and are not related to status events; my two new tests pass (absent `writeFailure` stays absent across encode/decode; a packed `null` reads absent; a real `folder-write-failed` survives).
- `repo test-file <renderer>/🧪️tests/🔑️dialog-session-key/🟦️.ts`: 2 pass, 0 fail.
- The pre-existing `🗣️dialog-origin` test file cannot be run by `repo test-file` (needs a DOM: `Element is not defined`), so the new key test lives in its own DOM-free file.
- Browser journey (`draw-journeys.sh`): `persisted` passes in en (r35i) and de (r35j) with `eventLogBytes - beforeEventBytes` about 5.4 KB. The folder card showed `offline · saved` in the probe run.

## Not verified / still open
- Both locales now fail one step later at `export`: `PDF pixels must change after the rectangle gesture`. before.pdf and drawing.pdf differ in bytes (3447 B vs 4264 B, the second has the extra layer form), but rendered pixels are identical: the page is the 200 x 200 artboard (`MediaBox [0 0 200 200]`) and the rectangle is at about (208, 338), size 506 x 288, i.e. outside it, so it is clipped. The gesture fractions in `🔣️.json` assume a canvas the artboard fills; the screenshot shows the artboard small at the top left (see `probe-artboard-fit.mjs`, `footer-overlap`). Not touched; either fit the view/artboard or move the gesture/fixture into the artboard.
- About 70 percent of runs (r35a, c, e, g, h) additionally hit the known guest faults `original-job.cancel-refused` then `original-job.start-stalled`; the guest then stops answering, `2 pending` never drains and `waitSaved` times out. Coordinator area (retained-turn / handle-turn-drive), not touched. `persisted` passed in the runs without that fault.
- `reload` step not reached. The Rust twin of the status event was not checked for how it serializes an absent `writeFailure` (the parser now accepts both).
- Deleted only my own generated output (probe runs and `journey-r35*`); if the coordinator had its own `r35*` run directories they would be gone too.
