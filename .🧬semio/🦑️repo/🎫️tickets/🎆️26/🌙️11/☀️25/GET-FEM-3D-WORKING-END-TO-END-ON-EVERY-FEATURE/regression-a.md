# Regression A: browser audit of the FEM 3D editor (builds 1 and 2)

Driver: own `page-driver.mjs` on port 6240 (PID 201396, stopped at the end), page `http://127.0.0.1:6087/`, viewport 1600x1000. Screenshots and bodies: `🗑️generated/ra/`. Build 1 notes: `🗑️generated/ra/build1-notes.md`. Build 1 was served before the coordinator's restart; build 2 after it (reload, 98 s boot). "Carried" means build 1 result, not re-run on build 2.

## Results

| # | Item | Steps | Observed | Latency | Result |
|---|---|---|---|---|---|
| 1 | Left drag orbit | drag on ground, no utility | view changes; reverse drag restores the same view hash | <2.5 s | PASS (build 1) |
| 2 | Middle drag | drag | view changes | <2.5 s | PASS (build 1) |
| 3 | Right drag | drag | view changes (pan only checked by hash, not visually) | <2.5 s | PASS (weak, build 1) |
| 4 | Wheel zoom | in/out | view changes both ways | <3 s | PASS (build 1) |
| 5 | Shift+left marquee | drag over model | red marquee mid-drag; release selects 35 items; camera unchanged | <4 s | PASS (build 1) |
| 6 | Pick thin column (x=540, +-3 px) | click at 540+dx, dx in 0,2,-2,3,-3 | hit only at dx 0 (and +-1 in build 1); miss at +-2 and +-3 | 5-8 s | FAIL (build 1 and build 2) |
| 7 | Empty-click deselect | click ground | Inspection SUMMARY | 0.05-5 s | PASS |
| 8 | Gumball move / rotate small / rotate large / scale / guide line / handle press | Transform armed, sol1 | gumball handles not locatable (canvas only, no DOM, pixel diff inconclusive, cursor identical everywhere) | n/a | NOT VERIFIED |
| 9 | Delete key | pick col_20_l, Delete | 16 -> 15 | 2.8 s | PASS (build 2) |
| 10 | Backspace | pick, Backspace | 16 -> 15 | 2.7 s | PASS (build 2, clean page) |
| 11 | Inspection Delete (element) | pick, Delete button | 16 -> 15 | 5.1 s | PASS (build 2) |
| 12 | Undo / Redo keys | Ctrl+Z, Ctrl+Shift+Z, Ctrl+Z, Ctrl+Y | each applied | 1.7-2.1 s | PASS (build 1) |
| 13 | Node delete cascade | Inspection Delete on n20_l1 | nodes 16 -> 15, elements 16 -> 12 (4 dependents), supports 8; Ctrl+Z restores 16/16/8 | 3.4 s, undo 1.5 s | PASS (build 2) |
| 14 | Delete after undo burst | repeated Delete/Backspace after Ctrl+Z/Ctrl+Shift+Z/Ctrl+Y | no change for 30-60 s with caption `Case: dead · Generating mesh`, then applied | 30-60 s | FAIL intermittent (build 1, not reproduced on build 2) |
| 15 | Create Alternative | Actions > Create Alternative, no edits | executed, no form, caption `Preparing analysis`, no alert | ~3 s | PASS (weak: console not captured on this run) |
| 16 | History Undo/Redo buttons, Commit Checkpoint | Actions pane | not run | | NOT RUN |
| 17 | Node XYZ | node n20_l1 Z 2.8 -> 2.9 -> 2.8 | value verified in Inspection and Artifact row | 4.8 s / 7.0 s | PASS (build 2) |
| 18 | Element Roll | col_20_l 0 -> 0.5 -> 0 | input verified | 6.3 s / 6.7 s | PASS (build 2) |
| 19 | Solid Height | sol1 0.5 -> 0.6 -> 0.5 | input verified | 5.9 s / 7.1 s | PASS (build 2) |
| 20 | Element kind/start/end/material/section, support DOFs, loads, combination + Add Term, material, section, Analysis row, Focus | | not run on build 2 (build 1 partial: see notes) | | NOT RUN |
| 21 | Artifact tree groups and counts | NODES 16, ELEMENTS 16, SOLIDS 1, SUPPORTS 8, LOAD CASES 2, COMBINATIONS 1, MATERIALS 2, SECTIONS 1 | all present | | PASS (build 2) |
| 22 | Artifact icons, no literal icon text | scan for ruler/timer/expand | none found in tree text | | PASS (build 2; Poisson readout not checked, Materials collapsed) |
| 23 | Actions pane open, viewport still drawn | open Actions | model drawn in 600-800 strip beside the pane, results pane intact | 2.5 s | PASS (build 2) |
| 24 | Copy/Paste element | pick col_20_l, Ctrl+C, Ctrl+V | 16 -> 17 | paste 13.6 s | PASS (build 2) |
| 25 | Undo of paste | Ctrl+Z | 17 -> 16 | 2.1 s | PASS (build 2) |
| 26 | Cut/Paste element | Ctrl+X, Ctrl+V, Ctrl+Z x2 | 16 -> 15 -> 16 -> 15 -> 16 | cut 12 s, paste 3 s | PASS (build 2) |
| 27 | Copy/Paste solid | sol1 Ctrl+C, Ctrl+V, Ctrl+Z | solids 1 -> 2 -> 1 | paste 3.5 s, undo 3.3 s | PASS (build 2) |
| 28 | Window authority stale / Model and Results unmount | orbit then 4 rapid wheel zooms | build 1: `original window authority is stale refusalKind=invalidValue`, canvases gone until reload. Build 2: one attempt, no refusal, 2 canvases | | FAIL (build 1); not reproduced (build 2, one attempt) |
| 29 | Renderer crash | first click of a build-2 body after restart | `mouse.click: Target crashed` (Playwright) | | FAIL (build 2, once; driver re-created afterwards) |

## FAIL diagnoses (exact console text)

- F-P pick tolerance (items 6): a 3 px thin column is picked only within +-1 px of centre; the padding of `WORLD3D_PICK_TOLERANCE_PX` is not in the served build. Reproduced on both builds.
- F-H stale window (item 28, build 1): `original-turn.semantic value.refusal: original window authority is stale refusalKind=invalidValue`, followed by `typed-operation progress refresh failed` and `typed-operation completion effects failed`. The Model and Results windows lost their canvases and Results/Model UI unmounted until a page reload. Not reproduced on build 2 in one attempt.
- F-L delete latency (item 14, build 1): after an undo burst, Delete/Backspace produced no change for 30-60 s while the caption read `Case: dead · Generating mesh`; no console line. Not reproduced on build 2 (items 9-11 settled in 2.7-5.1 s).
- Undo warning (seen during items 10-13, build 2): `warning input #32 undo refused: dispatch-failed (user window=fem3d-model) — interactive-job.stale`. Logged once per affected attempt; the undo itself still applied in the next attempt.
- F-R renderer crash (item 29, build 2): `mouse.click: Target crashed` on the first click of a freshly reloaded build-2 page, cause not isolated; a new driver restored the page.

## Driver-side mistake (disclosed)

While the Actions pane was open, a ground click at (420,900) landed on the pane's `Checkout Checkpoint` row (once). No refusal was captured. Checkpoints were not otherwise used.

## Not run / open

Gumball (item 8), History buttons and checkpoints (16), Inspection edits for support DOFs, loads, combination and Add Term, material, section, Analysis row and Focus (20), element kind/start/end/section/material, Poisson readout check. The build-1 partial results for these are in `build1-notes.md`.
