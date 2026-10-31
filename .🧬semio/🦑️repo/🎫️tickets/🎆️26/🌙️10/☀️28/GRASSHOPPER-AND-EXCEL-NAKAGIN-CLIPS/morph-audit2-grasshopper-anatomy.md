# Morph Audit 2: Grasshopper / Anatomy (`Beat1_Anatomy`), read-only

Inputs: first audit `morph-audit-grasshopper-anatomy.md`, scene `C:/git/semio/🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🦴️anatomy/🐍️.py` (mtime 2026-10-29 16:29:12), frames `T/🗑️generated/morph/🦗️grasshopper/Beat1_Anatomy/{en,de}/` (`index.md` read in both).

Freshness: cutoff = 16:29:12 minus 60 min = 15:29:12. The folder was recreated at 16:29:13, and every PNG is from 16:30:37 to 16:37:11, so all frames are newer than the cutoff and all are judged. None are stale.

Coverage: the current run holds frames for 5 clauses only (box, count, address, legend_tool, legend_data), each with `-03`/`-04` mid frames and an end frame where captured. The index lists the same 5 clauses in en and de. The done file's "full run of 19 clauses" does not match the folder. The frames for script, slider, flow, sliders, preview, panel, params, toggle, valuelist, plugin, item, series, tree and mistake are no longer present, so those beats cannot be judged in this round. The brief forbids capturing, so they were not re-captured. `en/media/v/MorphCapture.mp4` exists and may hold the other clauses; extracting from it is a separate decision.

## 1. Earlier WEAK / MISSING beats

| # | Beat | Round 1 | Round 2 | Evidence |
| --- | --- | --- | --- | --- |
| 1 | script (capsule > .gh chip) | WEAK | UNVERIFIED (no frame in current run) | none |
| 2 | slider (width value > knob) | MISSING | UNVERIFIED (no frame) | none; done file's `slider-03-mid` is absent |
| 3 | box (dimension lines > X, Y, Z wires) | WEAK | STILL WEAK | `en/box-03-mid.png`, `de/box-03-mid.png`: value pills 2.5 / 4.0 / 2.5 (yellow, teal, green) sit on the X, Y, Z grips and read well, so the tokens are fixed. The viewport dimension lines (2.50 m, 4.00 m, 2.50 m) are still at full strength and do not detach. `index.md` still has FadeIn x3 beside PartMorph x3 and Morph x3. The "2.5" pill covers the "Center Box" label. |
| 3b | box output grip > viewport box (`flow-05`) | WEAK | UNVERIFIED (no flow frame) | none |
| 4 | flow (pulses > box edges) | WEAK | UNVERIFIED (no frame) | none |
| 5 | sliders (knobs > edges) | WEAK | UNVERIFIED (no frame) | none |
| 6 | preview (box > 25 m³ row) | WEAK | UNVERIFIED (no frame) | none |
| 7 | panel | BUILT | not in this run | none |
| 8 | toggle (True knob > window ring) | WEAK | UNVERIFIED (no frame) | none |
| 9 | valuelist | BUILT | not in this run | none |
| 10 | plugin (ring > Window Brep) | WEAK | UNVERIFIED (no frame) | none |
| 11 | item > series | WEAK | UNVERIFIED (no frame) | none |
| 12 | count (new rows > capsules) | WEAK | STILL WEAK | `en/count-03-mid.png` and `en/count-04-mid.png` are the same frame, and `de/count-04-mid.png` matches. The list already has 11 rows (0 to 10), the Count slider already reads 11, and the tower already has 11 lit levels. No row or token is in flight, so the morph is invisible. `index.md`: count-03 UpdateFromAlphaFunc, count-04 PartMorph x2, count-05 FadeOut x2. |
| 13 | tree (capsules > leaves, path chip) | WEAK | UNVERIFIED (no frame) | none |
| 14 | mistake ([1] > one per branch) | WEAK | UNVERIFIED (no frame) | none |
| 15 | address (dots > one tag) | WEAK | STILL WEAK | `en/address-03-mid.png`: the green chip "{0;0;0;3}, index 1: one capsule" is already on the canvas, and the red "[1] exists in 35 of 41 branches" strip runs behind it, so the two texts collide. `en/address-04-mid.png` (and `de`): a green-orange smear over the lower tower and the canvas, with a red strip that runs into the viewport. The smear has no identity, and the dots do not read as dots. The smear has moved off the tree panel, but it is still a blob. |
| 16 | legend_tool (markers > rows) | WEAK | STILL WEAK | `en/legend_tool-03-mid.png` and `de/...-03`: discs 1 to 8 sit fully opaque on the canvas (chip, Box, Width, wires, output, slider, LD) while the rows are already visible. No disc is in flight. `en/legend_tool-04-mid.png` and `de/...-04`: grey ghost discs are scattered over the canvas, and the black numbered discs overlap on the rows ("12", "23", "34", "43", "56", "7"), so the numbers collide. |
| 17 | legend_data (brackets > rows) | WEAK | STILL WEAK | `en/legend_data-03-mid.png` and `de/...-03`: discs 2 to 6 are static on the Panel and the rows are not yet filled, so no flight. `en/legend_data-04-mid.png` and `de/...-04`: the same overlapping disc pairs ("12", "23", "34", "43", "56", "7") land on the rows, with grey ghost discs around the canvas. |

Round 2 counts: FIXED 0. STILL WEAK 5 (box, count, address, legend_tool, legend_data). UNVERIFIED 12 (no frame in the current run). BUILT earlier (panel, valuelist) not in this run.

## 2. Remaining fade-swaps and cuts (from `en/index.md`, same in `de`)

- box-03: FadeIn x3 still sit beside PartMorph x3 and Morph x3 (wires and value tokens are not one morph).
- box-04: FadeOut x3 with no box output morph.
- count-05: FadeOut x2 after the morph; the new rows are not seen leaving the list.
- address-03: FadeOut x3, Create, FadeIn x2 for the tag beside the chip; the gather is a Swarm in address-04, not a morph.
- legend_tool-03: FadeOut x9 and FadeIn x9 for the canvas markers and rows beside one LaggedStart. The rows should be the morph's landing, not fades.
- legend_data-03: FadeOut x11 and FadeIn x2 beside LaggedStart; the Panel markers fade instead of flying.

## 3. Directions and chain

- Concrete > abstract visible: legend_tool-04 and legend_data-04 (markers become disc/row labels, grey scatter visible but overlapping).
- Abstract > concrete visible: address-04 (dots converge on the tower, but as a smear).
- Concrete > abstract missing: box (grips do get tokens, but the dimension lines do not detach).
- Abstract > concrete missing: count (no token in flight).
- Both directions occur in the sampled frames, but no single object is traced across clauses in this run. The chain item > tree > `[1]` > address > legend cannot be checked.

## 4. Collisions and clutter

- box-03 (en, de): "2.5" pill over the "Center Box" label; faint "Name Value" / "Name Wert" header ghost above the sliders.
- count-03/04 (en, de): none in motion; the tower and list are already in end state.
- address-03 (en, de): red "[1] exists" strip behind the green chip text, both legible in en, collided in de.
- address-04 (en, de): green-orange smear over the lower tower and canvas, red strip into the viewport.
- legend_tool-03/04 (en, de): ghost grey discs at canvas edges and overlapping number discs on the rows.
- legend_data-04 (en, de): overlapping number discs on the rows, grey ghosts around the Panel.

## Verdict: FIX

Fix list:

1. Restore frames for script, slider, flow, sliders, preview, panel, params, toggle, valuelist, plugin, item, series, tree and mistake (en and de). The current run captured only 5 clauses, so 12 beats cannot be judged; the done file's claims for them are unverified.
2. box: fade the viewport dimension lines out only after their tokens leave; replace the 3 FadeIn with the morph; move the "2.5" pill off the "Center Box" label.
3. count: the two new rows must visibly leave the list and land on the tower; count-03 and count-04 are the same frame, so the flight must be inside the 1.6 s window, not after the end state.
4. address: replace the green-orange smear with a small chip that docks on the capsule; keep the red "[1] exists" strip out from behind the chip text (fix in de most of all).
5. legend_tool and legend_data: fly the discs during the morph, not as static discs or grey ghosts; stop the number discs from overlapping on the rows (offset each row's disc, or land one disc at a time with lag).
6. Replace the remaining FadeIn/FadeOut pairs listed in section 2 with morphs where the object keeps its identity.
7. Re-run the capture with all 19 clauses so the done file and the frames agree; then re-audit.

## Summary

1. 17 earlier beats: FIXED 0, STILL WEAK 5 (box, count, address, legend_tool, legend_data), UNVERIFIED 12 (no frame in the current run).
2. Box value tokens (2.5 / 4.0 / 2.5) read well at the grips; dimension lines still do not detach.
3. Count is identical at -03 and -04; the flight is not visible. Address still has a smear and the strip collides with the chip.
4. Legend discs are static at -03 and overlap on the rows at -04, in en and de; German labels fit the boxes.
5. Verdict FIX; fix list above. Scene mtime 16:29:12, all 26 frames newer than the cutoff, none stale.
