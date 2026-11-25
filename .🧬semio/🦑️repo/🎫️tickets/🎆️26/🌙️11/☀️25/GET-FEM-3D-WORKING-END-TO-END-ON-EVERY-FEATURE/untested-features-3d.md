# Untested Features 3D (browser audit of the FEM 3D editor)

Run: headless Chromium driven by `page-driver.mjs` (own port 6230, driver PID 23812 stopped at the end) against `http://127.0.0.1:6087/`, viewport 1600x1000. Boot to `Case: dead` took 56-89 s. Each row is one feature from the "Not tested" list of `feature-matrix-3d-v2.md`. Several FAILs poison the interaction domain for the rest of the page session, so the page was reloaded between groups. Console lines are quoted from the driver log. Screenshots and bodies are in `🗑️generated/hx/`. Noise on every action (not counted as a failure): `error [DEBUG] receiving quote lane=WindowConfig ...`.

## Results

| # | Feature | Steps | Expected | Observed | Latency | Result |
|---|---|---|---|---|---|---|
| 1 | Member load edit | addMemberUdl on `col_20_l`, wz -2500, case `live`; then Inspection wz = -3000 | row updates, re-solve | row `wz −3000 N/m @ col_20_l Load`, caption `Preparing analysis` | add 3.5 s, edit 3.2 s | PASS |
| 2 | Combination term edit | select `ULS`, Live Load 1.5 -> 1.6 | row updates | row `ULS · 1.35 dead + 1.6 live`, re-solve | 3.1 s | PASS |
| 3 | Element Section combobox | Add Section `HEA 300`, then `col_20_l` Section combobox | element uses HEA 300 | options `HEA 200`, `HEA 300`; Section shows HEA 300, re-solve | section 1.7 s, combo ~3 s | PASS |
| 4 | Select All | Actions > Select All (clean page; then with one pick) | all elements selected | no refusal, but no highlight; viewport hash identical to empty selection; Inspection stays SUMMARY | 2.5-3 s | FAIL (F-A) |
| 5 | Clear Selection | pick `col_20_u`, then Clear Selection | highlight removed | viewport returns to empty-selection hash | 2.5 s | PASS |
| 6 | Set Selection Mode | domain (required) + mode Multiple, Execute | mode set | Execute disabled until a domain is typed; with `fem3d` (declared id) still refused, see F-B | 2.5 s | FAIL (F-B) |
| 7 | Set Granularity | domain `fem3d`, granularity `element`, Execute | granularity set | no refusal, no visible effect; follow-up check unreliable | 2.5 s | NOT VERIFIED |
| 8 | Translate Selection | pick `col_20_u`, dx 0.5 | element moves, re-solve | no refusal, caption re-solve, element moved in Model view | 1 s | PASS (weak) |
| 9 | Rotate Selection | pick `col_20_u`, axis Z, angle 3 | rotation | no refusal, geometry visibly changed | 1 s | PASS |
| 10 | Scale Selection | pick `col_20_u`, sx 1.3 | scale, re-solve | no refusal, caption `Preparing analysis` | 1 s | PASS (weak) |
| 11 | Set Result Display | sourceId `Live Load`, mode Modal, index 1, Execute | caption and display change | no refusal; caption stays `Case: dead`; Display panel stays Dead Load / Static | 4 s | FAIL (F-C) |
| 12 | Set Active Example | Concrete Forest, Execute | example switches, geometry changes | combobox and navbar show Concrete Forest, no refusal, no re-solve; geometry looked identical to Demo | 6 s | NOT VERIFIED (F-D) |
| 13 | Cut | col_20_u selected, Cut | element removed to clipboard | refused, see F-E | 1 s | FAIL (F-E) |
| 14 | Export Document | click Export on a clean page | file downloads | downloaded `fem3d-20261125T171025Z.semio-archive`, 1272 bytes, header `SEM ... fem.fem3d.pack v1`; copy in `hx/export-roundtrip.semio-archive`. On a poisoned page it fails, see F-E | 0.2 s | PASS (clean page) |
| 15 | Import Document | chooser, probe JSON, then the exported archive | probe rejected, archive accepted | probe rejected with a clear message; archive accepted silently; afterwards edits refused, see F-F | n/a | FAIL (F-F) |
| 16 | Concrete Forest example | = row 12 | | | | NOT VERIFIED |
| 17 | Theme/Appearance | Settings > Driver > Appearance: System -> Dark -> System | UI dark | whole UI turned dark (`hx/appearance-dark.png`) and restored; `body` computed background unchanged (beige), so the change is visual only | 2 s | PASS |
| 18 | Window reopen via Display | Close Results (aria-label Close), reopen from Display > Windows | Results window returns | Close works (2.1 s). Click on the `Results` entry does nothing. Dragging the handle (x 293, y 804) reopens it (Close count 1 -> 2). Dragging the entry text only selects text | close 2.1 s, reopen 7 s | PASS (drag handle only) |
| 19 | Playback readouts | Results window: Play, then Step +, then Pause | readouts change, Pause stops | Play: caption `Case: dead · Phase 0.04 · ▶ 0.5 Hz`. Step +: caption and Phase `1` unchanged. Pause: panic, playback frozen, see F-G | 2.5 s | FAIL (F-G) |

## FAIL and NOT VERIFIED diagnoses

**F-A Select All (silent no-op).** On a clean page the action produced no refusal and no visible selection (viewport hash `f58261f6` equals the empty-selection hash; Inspection stays SUMMARY). After a pick, the highlight disappeared while Inspection still showed one element. Once Set Selection Mode had poisoned the session, the same action was refused: `warning input #44 selectAll refused: dispatch-failed (user window=fem3d-model) — original-turn.native-return-refused: semantic: plugin.internal: setSelectionMode: undeclared interaction domain viewport`.

**F-B Set Selection Mode (wrong domain dispatched).** The `domainId` argument is required (Execute stays disabled, reason text `Domain`). Typing `viewport` enables Execute and gets refused. Typing `fem3d`, the declared interaction domain (`FEM3D_INTERACTION_DOMAIN` in `🎬️scene/🦀️.rs`), is also refused, with the same dispatch target:
`[actor] original-turn.semantic plugin.internal: setSelectionMode: undeclared interaction domain viewport`
`warning input #46 setSelectionMode refused: dispatch-failed (user window=fem3d-model) — original-turn.native-return-refused: semantic: plugin.internal: setSelectionMode: undeclared interaction domain viewport`
The dispatched domain ignores the typed argument. After this refusal, picks, Clear Selection and Select All are all refused until reload.

**F-C Set Result Display (silent no-op).** Execute runs with no refusal and no console line. The Results caption stays `Case: dead`, the geometry does not change, and the Display panel keeps `Dead Load` / `Static`. Same as matrix F10.

**F-D Set Active Example, Concrete Forest (not verified).** The combobox, the navbar and the execute all took the new example, with no refusal. The viewport did not visibly differ from Demo, and no re-solve started. A full analysis check was not done.

**F-E Cut, and Export after a refusal.** Cut on a selected element is refused:
`[actor] original-turn.semantic UnsupportedOwner: natural-file import requires its owned declared media and no wire`
`warning input #18 cut refused: dispatch-failed (user window=fem3d-model) — original-turn.native-return-refused: semantic: UnsupportedOwner: natural-file import requires its owned declared media and no wire`
This refusal poisons the session: `warning input #22 interactionSelect refused: dispatch-failed (gesture window=fem3d-model) — ... UnsupportedOwner ...`. Export Document on that poisoned page fails: `[os-shell] shell.documentTransfer.export-failed fem#1 Error: original-turn.native-return-refused: semantic: UnsupportedOwner: natural-file import requires its owned declared media and no wire`. On a clean page Export works (row 14). Matrix F11 (Paste) has the same error text.

**F-F Import Document (round trip breaks the editor).** The file chooser opens. A probe JSON is rejected cleanly: `[os-shell] shell.documentTransfer.import-unreadable import-probe.json Error: document archive: unsupported or missing version`. The exported archive is accepted with no message, and the counts are unchanged (the content is identical, so this is not proof of a restore). After that, an edit is refused: `warning input #4 addNode refused: instance-sealed (user window=fem3d-model) — fem#1`, followed by `noteShellCommand refused: instance-sealed`. An "Unsaved changes" modal then appears: `"s.fem.fem3d@1/*#editor" has changes that were not saved. Close anyway and lose them? The save was not acknowledged.` with `Keep editing` / `Close and discard` (`hx/after-import-check.png`). The modal appeared after my next actions, so its trigger could not be pinned down.

**F-G Playback Pause panics; Step + does nothing.** Pause triggers:
`error [semio-plugin panic] panicked at semio-framework-value-allocation-return-0.1.0\..\..\shared\original\🦀️.rs:152:62:`
`error shared original alias must retire or transfer`
`error scheduled dispatch of "resultAnimationTick" failed Error: wasm trap: unreachable`
`warning input #31 setResultAnimation refused: dispatch-failed (user window=fem3d-results) — wasm trap: unreachable`
Afterwards the caption stays frozen at `Case: dead · Phase 0.04 · ▶ 0.5 Hz`. This matches matrix F9. Step + took 1.6 s with no visible change to the Phase slider (still 1) or the caption.

**Set Granularity (not verified).** Execute runs with no refusal. A follow-up pick check did not produce a readable Inspection, so the effect is unknown.

## Files

- Screenshots, driver bodies (`e*.js`), helpers (`pre.js`, `h2.js`, `run.sh`), `reload.js`, `boot.js`, the probe `import-probe.json` and the export `export-roundtrip.semio-archive`: `🗑️generated/hx/` in this ticket.
- Driver process stopped (PID 23812).
