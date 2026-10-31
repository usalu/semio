# Morph done: excel references (`Beat2_References`)

File: `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📍️references/🐍️.py` (imports `🔀️morph`). Storyboard: `morph-excel-references.md`. Clauses and narration unchanged (24, bilingual); no counts in narration were touched.

## Implemented beats (11 morph animations in 10 clauses)

| clause | direction | morph |
| --- | --- | --- |
| hook | C->A | width dimension token on a lit type A capsule of the tower -> cell chip on B2 (ghost); then the width runs 2.50 -> 3.00 m |
| pointer | C->A | four hatched faces (top, bottom, front, back) -> area chips on D3, D4, D7, D9 (`scatter`) |
| pointer | A->C | B2 (blue) and B3 (red) chips -> width and length labels on the capsule dimensions |
| shift | A->A | `=D3*E3` retexts into `=D4*E4` (row copy flies to the F4 slot) |
| back | A->C | factor chip `b = 0.50` from F9 -> the back face |
| total | C->A | seven U·A tokens on their faces gather into the SUM chip on F10 |
| repair | A->A | `=F4*D11` retexts into `=F4*$D$10` |
| repair | A->C | seven loss chips G3:G9 -> labels on their faces (stay until mixed2) |
| mixed | C->A | the lit capsules of tower A / type A gather into the COUNTIFS count in B11 |
| sheet | C->A | plan-view floor face -> the net floor area chip on Program C4 |
| threed / span | A->A | four B13 values gather into the sum (span: only the three tabs inside the bracket) |
| guard | A->A | `=F3*$D$10` retexts into `=F3*#REF!` |
| names | A->A | `=F3*(Climate!$B$2-Climate!$B$3)` retexts into `=F3*(theta_i-theta_e)` |

Four concrete -> abstract (hook, pointer, total, mixed, sheet) and three abstract -> concrete (pointer, back, repair) beats; chains: width (hook -> pointer), surface (total -> repair labels).

Not done from the storyboard: slide/stuck/pertype/deleted shards, f4 padlock chain, mixed2 bars, fill2d split back into capsules, recap travel (the toolkit work for them needs per-glyph shards; they keep their previous fades).

## Technique notes

- Sources are added with `self.add` or a short pre-play, targets are tokens built at the final place; `step` receives the swarm/morph as one animation next to normal fades (nesting a Swarm in the group works, checked in `excel-references-morph/nest.py`).
- Chips left over are removed by `_drop` (fade 0.4 s) after the clause.
- Axon view has no `back`/`left` faces: `_face` searches both capsule views.
- Retext rows are taken from `_formula_card` by index and absorbed back (`_absorb`) so card fades still remove them.

## Frames checked

en: all clauses captured (index in `🗑️generated/morph/📊️excel/Beat2_References/en`); mid frames read for hook, pointer, shift, back, total, repair, mixed, sheet, span, guard. de: pointer, shift, back, total, repair, mixed, sheet, threed, span, guard, names captured, pointer mid read. In-flight objects read as coloured chips or outlines; colours carry (blue/red references, yellow lit capsules).

## Open items

- `capture.py` layout keyframes (en/de) and strict layout run not repeated after the morphs; the retained chips on faces (loss labels in repair) add small text on the capsule views.
- Part-wise capsule -> token morphs (`pairs=`) not used; sources are silhouettes.
- Tests: `X/🧪️tests/🐍️.py` 84 OK. Presentation contract not re-run.

## Round 2: remaining beats (done)

| clause | direction | morph |
| --- | --- | --- |
| f4 | A->A | `D10` row retexts into `$D$10`, `D$10`, `$D10` (LaggedStart); padlocks drop onto the column part (left) and row part (right) of the reference token |
| slide | A->C | wrong G chips crack into shards (`_shatter`) that fall onto the face each wrong value describes; the faces turn red at the moment they land |
| repair | A->C | corrected loss chips fly onto their faces; the labels now fly back into the G cells as chips before mixed2 (no leftover labels) |
| mixed2 | A->A / A->C | D19:D21 values gather into a `W/m²` token; the token hands over to the bar chart, whose bars grow from zero (ValueTracker rows) |
| fill2d | A->C | the C11 count chip splits into the lit capsules of tower/type C11 before the fill runs |
| stuck | A->C | wrong C5:C7 chips shatter and fall onto the Type A cells B10:B12 they wrongly read |
| pertype | C->A | four type-coloured tab chips (`Type A..D`) fly from the sheet tabs to their formula rows |
| deleted | A->C | the `#REF!` chip shatters onto the sum cell; the new sum chip flies from the card into Energy B16 |
| recap | C->A | six term chips start on the cells/tabs where each rule lived (F3, D10, D3, Climate tab, Type A tab, Name Manager) and fly into the legend labels |

`pairs=` is used for face -> chip morphs (pointer faces, sheet floor); `gather`/`split` have no pairs option, so mixed (capsules -> count) and fill2d (count -> capsules) use plain silhouettes.

Frames read: en f4, slide (shards fall onto the capsule), stuck, mixed2, deleted, recap, pertype, fill2d; de f4, repair. Both runs finished without errors. X tests 84 OK. Not re-run: layout keyframes (`capture.py`) and the presentation contract.

Incident: a scripted write went to a wrong `/tmp` file and overwrote the scene file once; restored from the intended content (all edits verified present, 84 tests OK).

## Round 3 (audit FIX list)

Changes in the scene file:
- lights: the seven capsule surfaces (silhouettes, `pairs=`) fly to their U·A cells F3:F9 as chips, one per row, in step with the fill (surface chain face -> value).
- absolute: the D10 chip (20.0 K) splits into the two climate values it stands for, theta_i on the capsule and theta_e outside it (`split`); chips fade after the clause.
- mixed2: "Lock the input" card is faded in the same play that sends the loss labels back to the G cells; the three D19:D21 value chips now morph into the real bars of the chart (bars removed from the chart group, targets of `scatter`, then absorbed); no more fade-swap chart or W/m² token.
- pertype -> threed: the four type chips from the formula rows fly into the four small type capsules (`scatter`), the capsules send silhouettes into the B13 value texts, the values gather into the sum; span repeats the capsule -> value step before gathering the three counted values (capsules replace FadeIn). Type chain: tab chip -> formula-row chip -> capsule -> B13 value -> sum, and the capsules/values of the span step are faded with the deleted clause.
- deleted: sum chip moved to the card corner (no longer over the formula text); recap: windows and cards fade first (0.7 s), then the term chips fly into the legend.
- pertype tab chips are smaller (0.55).

Evidence: ONE full capture each, en and de, no key filter, 68 files each in `🗑️generated/morph/📊️excel/Beat2_References/{en,de}` (both logs without errors; captured after the toolkit's automatic part matching landed at 16:59). X tests were not re-run after round 3.

Frames read (en): lights-03, absolute-03, mixed2-04, mixed2-05, threed-03, threed-04, deleted-03, pointer-03, pertype-03, recap-03 (earlier rounds: f4, slide, stuck, back, total, repair, mixed, sheet, span, guard). Not read this round: the remaining en mid frames and the de set (present on disk for the audit).

Seen in frames: lights chips land on F3:F6 while surfaces hold the capsule views; absolute splits the ΔT chip onto the capsule; mixed2 chips become the bars; threed capsules/values travel; the deleted sum chip sits in the card corner.

Open items:
- pertype: the tab chips start on the tab bar and cross the Type C/D tabs and the Total row while flying (transient, inherent to starting at the tabs).
- deleted: the `#REF!` shatter and shards are not visible in the sampled mid frame (the shards fall for 1.4 s; the mid sample caught the sum-chip hand-over).
- f4: the live reference token in the formula bar does not retext (only the card rows); absolute has no seven tethers (they would cross the F column text); slide/stuck shards have no ghost at the origin cell.
- hook M2/pointer M3 splits and the mixed2 face -> row-label morph are not built.

## Round 4 (second audit + calibrated review)

Changed in the scene file (precise):
- pointer: after the B2/B3 chips land on the capsule as blue/red dimension tokens they now morph (`scatter`) into the two reference tokens of a live `=B2*B3` formula text under the capsules (the other glyphs fade in); the pieces fade at `relative`. This is the width-chain split.
- sheet: the window swap runs first (0.7 s), then the floor silhouette -> C4 chip morph on a clean stage.
- absolute: the F4 card fades before the step (no longer over the capsules); the seven G3:G9 chips fly onto their faces as loss labels (`scatter`, one per row), the D10 chip still splits into theta_i / theta_e; labels fade after the clause.
- slide: one dashed frame is born on D10; D11 and D12 are born from it by `morph(keep_source=True)` (the reference visibly slides down and leaves the D10 frame), the shatter onto the faces stays.
- span: the Type B tab chip (type colour) is dragged from the bracket to the end of the tab strip (`morph`, arc) before the three remaining values gather; the capsules of span are born by `scatter` from the threed capsules (no FadeIn), same for pertype -> threed chips.
- deleted: the Type C tab chip shatters onto the `#REF!` cell together with the error chip, then the new sum chip flies into B16.
- guard: the `#REF!` chip of the deleted clause morphs into the G3 error chip of the guard window.
- mixed2: U-value faces (roof, left wall, window) morph into the row labels A19:A21; the cell chips morph into the real chart bars.
- recap: real formula text copies (`=D3*E3`, `=F3*$D$10`, `=$B19*C$18`, `='Type A'!B9`, the 3D sum, `=F3*(theta_i-theta_e)`) start at the place they were taught and fly into their legend rows; the old anonymous kind chips are gone; the kind labels fade in with the table.

Capture: one full en and one full de run (no key filter, 71 files each, no tracebacks) after these edits.
Mid frames read (en): slide-04, span-03, deleted-05, recap-03, absolute-03 (plus the earlier full-capture frames of round 3). Seen: recap formulas fly from their places and land in their rows; absolute loss chips fly onto the capsule faces. slide-04, span-03 and deleted-05 are transition samples (window swap / card), so the sliding frame, the tab drag and the tab shatter are in the code but not caught by a sampled mid frame. de frames are on disk, not read.

Not done from the review (still open): formula-bar token retext (shift, f4, repair, guard, names; needs a `win` formula-bar handle), hook/pointer capsule widening with tethers, seven tethers into one D10 frame, dimming a type C capsule on the tower in deleted, fill2d per-cell gather/split loop (a loop cannot sit inside one `step` animation without a custom animation; still the tracker sweep), pertype formula-bar retext chain, 1280x720 key-frame check.

## Round 5 (formula bar, fill2d loop)

- Formula-bar retext: new `_bar_retext(win, old, new)` hides the window's bar text (`win.edit("")`), puts a copy of the old formula at the bar's own position (`win.field_left`, centre of `win.formula_bar[0].base[0]`) and retexts it into the new formula next to the card retext; afterwards the real bar is restored with `win.edit(...)`. Used in shift (`=D3*E3` -> `=D4*E4`), repair (`=F4*D11` -> `=F4*$D$10`), guard (`=F3*$D$10` -> `=F3*#REF!`), names (`Climate!$B$2-Climate!$B$3` -> `theta_i-theta_e`). f4 is not done in the bar: its window selects D10 (no formula with the reference) and the four forms live only in the key-cycle card.
- fill2d: the tracker sweep is gone. New `_clause(key, plays)` runs a clause as several successive plays before the narration hold (chained from the one `step` play). For each cell B11..E12 (except the first, which `mixed` already did) the tower relights, then odd cells split the count chip into the lit capsules and even cells gather the lit capsules into the chip (0.9 s each), 7 cells in about 9 s.
- Captures (changed keys, en and de, no tracebacks): shift, repair, guard, names, then fill2d again after the sequential-play fix (the first fill2d version used a `Succession`, which added all later sources at the start; replaced).
- Frames read: en shift-03 (bar token mid-retext `=D3*E3` -> `=D4*E4`), en fill2d-10 (capsules streaming to the E11 chip), de fill2d-16 (row 12 cells filling in order), de guard-03 (bar retext of `$D$10` into `#REF!`), de guard-04, de names-04, de repair-04 (loss chips flying to faces). X tests: 84 of 85 OK; the one failure is `NakaginCrossCheckTests.test_capsule_row_positions_follow_primary_phase_datums` (Nakagin model positions, edited concurrently by another agent, not this scene).
- Not done: dimming a type C capsule on the tower in deleted (no room on that stage: two windows and the card fill it, the tower would be an unreadable thumbnail); f4 bar retext (see above).
