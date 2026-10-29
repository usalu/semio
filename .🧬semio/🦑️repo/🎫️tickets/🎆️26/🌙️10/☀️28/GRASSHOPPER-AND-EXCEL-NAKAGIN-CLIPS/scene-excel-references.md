# Scene report: excel_references (`Beat2_References`)

File: `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📍️references/🐍️.py`. Storyboard: `storyboard-excel-references.md` (the built scene follows the clause arc, with the cells bound to the real workbook as described below).

## What was built

24 clauses, `TITLE` en/de, `NARRATION` en/de (each clause at most two caption lines), `construct` -> `present("references")` -> `present_references`. Left: the Excel window (real engine workbook, German tabs/functions/`;`/decimal comma through the engine). Right: the Nakagin capsule type A seen from above and from below (`capsule_axon`, live width, hatched face, per-surface tint) or the tower (`tower_axon`, lit capsules by tower and type), or formula cards (`xl_formula_text`, reference colours equal the frames).

All values come from the engine: cell texts via `shown`/`display`, edits via `apply` events (`set-cell`, `copy`, `move-sheet`, `delete-sheet`), formula text via `localize` (sheet names `'Typ A'!B9` in German), `cycle_reference` for the F4 forms, error labels via `localize_error`. No building number is typed in the scene. `X/🧱️model/🔣️.json` was not given new cells; the topic's scratch cells (`C10`, `D10` design temperature difference, column `G` loss in W on sheet Type A) are created in the scene with `apply` events. The only JSON edit: column widths E 14 -> 17 and F 10 -> 13 on the four type sheets, so U-values and U·A show instead of `###` (re-read, small line edits).

| # | key | on screen |
| --- | --- | --- |
| 1 | hook | Type A sheet (A:D), capsule with live width dimension; teaching model said once |
| 2 | pointer | D3 `=B2*B3` in edit mode with two reference frames; width 2.50 -> 3.00 m moves B2, D3, D4, D7, D9 live; top face hatched |
| 3 | relative | C:G window, F4:F8 emptied, fill handle F3 -> F8 |
| 4 | shift | card `F3 =D3*E3` / `F4 =D4*E4` with row shift note, frames on D3 and E3 |
| 5 | lights | F4:F8 fill in one by one, a frame follows the row, the matching capsule face lights (window row lights the glass front) |
| 6 | fixed | input D10 (design temperature difference from Climate B2, B3) framed |
| 7 | f4 | four cards from `cycle_reference`: D10, `$D$10`, `D$10`, `$D10` with their lock names |
| 8 | absolute | G3 `=F3*$D$10` filled down, losses in W appear row by row, faces tinted by loss |
| 9 | mixed | Program tower-by-type table, B11 COUNTIFS with `$A11` and `B$10`, tower lit |
| 10 | fill2d | table fills cell by cell, tower lights the capsules of each cell's tower and type, totals follow |
| 11 | sheet | Program C4 `='Type A'!B9` plus a thumbnail of Type A with B9 framed, plan view hatched |
| 12 | stuck | C4 filled down gives `'Type A'!B10`, `B11`: only the cell part moves, wrong values visible |
| 13 | threed | Energy B16 `=SUM('Type A:Type D'!B13)`, four capsules with B13 values and the sum |
| 14 | span | Type B tab moved out of the span, B greyed, sum recomputed (2,747 -> 2,066 W) |
| 15 | names | G3 `=F3*(theta_i-theta_e)` against the Climate cell version, same values |
| 16 | slide | `=F3*D10` filled down: D10, D11, D12 dashed red frames, wrong values, red faces |
| 17 | repair | F4 -> `$D$10`, fill again, values and faces correct |
| 18 | deleted | delete Type C: `#REF!` / `#BEZUG!` on Program, 3D sum shrinks silently (2,747 -> 2,042 W), type C capsules lit |
| 19 | guard | `$D$10` stays valid, `='Type C'!B9` is the error |
| 20 | recap | six-row legend en/de: relative, absolute, mixed, sheet, 3D, name with coloured examples |

Storyboard deviation: the relative fill-down uses the U·A column (`=D*E`, one uniform formula) instead of area = width × height, because the workbook's area formulas differ per surface; the area cells are shown and the pointer clause uses D3 = B2*B3. The fixed input is the design temperature difference on the Type A sheet (D10), not a cell on Climate. The deleted-sheet clause states the true Excel/engine behaviour: direct references show `#REF!`, the 3D sum shrinks without an error.

## Audit round (audit-excel-references.md) and toolkit update

Fixed in the scene: `lights` now accumulates all filled faces and F4:F9 are revealed row by row (back wall included, so no pre-filled row); `slide` shows the wrong value beside the correct one per row (`#VALUE! ≠ 120 W`); `repair` tags G4 before/after; `guard` tag gets width by text length; F4 tags read "1st F4" … "4th F4" (de "1. F4"); card captions capitalised; view labels in ink colour; `fill2d` shows "Tower B · Type D: 22" for the visited cell; German wording (comma in shift, "Nach rechts und unten in einem Zug", "Das Blatt der Art C löschen", "Das Dollarzeichen vergessen", apostrophes for sheet names with a space, recap "Namen machen es lesbar"); `fixed` says the design temperature difference is kept in one cell; the sheet clause says the same in en and de.

Dropped: the scene `__deepcopy__` override (toolkit windows hold a weak scene). Remaining pickle error came from `self.live(self._faces, …)` with bound methods, now lambdas. Workbook widths: Program columns widened (A13 B13 C17 D19 E21) so taught values and `#BEZUG!` fit; E and F of type sheets widened earlier.

Not done (justified): thousands separators on Program totals are engine formats (not this topic); the stuck clause does not show the corrected per-type formulas (the clause states the rule; the repair is shown in the recap card); the Name Manager step stays out (formula text swap only); mixed uses the real 140-row COUNTIFS (a 2×2 intro would need new cells); no span bracket (all seven tabs fit and the moved tab is visible).

Final capture after the toolkit update: en 81 strict findings in 15 of 20 frames, de 117 in 17 of 20, all window gridlines, edit overlays and token adjacency by design. Every frame of both languages was reviewed again; no `###` on a taught value, no truncated error code, tab names equal the formula sheet names in both languages.

Tests: `X/🧪️tests/🐍️.py` 71 tests OK. Presentation tests: scene contract for `Beat2_References` passes (the earlier run's only failures were other agents' scenes).

## Left-over items now done (22 clauses)

- `mixed2` (new): 2×2 table on the Program sheet (scratch cells A18:C20 from engine values: outer/inner width down, outer/inner length across), `=$A19*B$18` filled right and down, cells appear one by one; a card names each cell (roof area 10.0 m², net floor area 8.7 m² on the diagonal). `mixed` then repeats the pattern with the 140-row COUNTIFS.
- `names`: Name Manager panel (en "Name Manager", de "Namens-Manager") with Name, Refers to (localized `Klima!$B$2`) and Value for `theta_i`, `theta_e`, `theta_base`, read from the workbook names and the engine.
- `threed` / `span`: yellow bracket over the sheet tabs from Type A to Type D ("3D span" / "3D-Bereich"): four tabs before, three tabs plus the moved Type B outside after.
- `pertype` (new): the corrected formulas `='Type A'!B9` … `='Type D'!B9` with their values after the wrong fill-down of `stuck`.

Final capture: en 69 strict findings in 16 of 22 frames, de 105 in 18 of 22 (by design as above); all 22 frames per language reviewed for the changed clauses (mixed2, mixed, stuck, pertype, threed, span, names, slide) in at least one language and the unchanged ones in the earlier round.

## Final verification fixes

- Program widths now A 9, B 8, C 8, D 12, E 12, F 12 (shared with Beat3_Program, agreed with its agent); my Program windows use columns A:E and rows 10–13 / 3–8, so the stray row 9 total no longer shows; no `###` on any shown value in either language.
- `deleted`: the error tag sits on D6 (formula bar `=B6*C6`), off the count in B6; the 3D sum card reads `2,747.0 − 704.5 = 2,042.5 W` (de `2.747,0 − 704,5 = 2.042,5 W`), asserted consistent in code.
- Final capture: en 62 strict findings in 16 of 22 frames, de 93 in 18 of 22 (by design). Open minor: the Type A thumbnail title is overlapped by the search pill in `sheet`.

## Quality round (quality-excel-references.md)

New order (23 clauses): hook, pointer, relative, shift, lights, total, fixed, f4, absolute, slide, repair, mixed2, mixed, fill2d, sheet, stuck, pertype, threed, span, deleted, guard, names, recap.
- `hook`: the tower with type A capsules lit, width 2.50 -> 3.00 m, dashed frames on the four dependent cells (D3, D4, D7, D9). `pointer`: top, bottom, front and back hatched, width and length dimensions.
- `slide` and `repair` follow `absolute` directly. `lights` tints faces by U·A; new `total` clause: `=SUM(F3:F9)` in F10 equals H_T in B11 (30.92 W/K).
- `mixed2` replaced: U-values (roof, wall, window from the Energy sheet) down by temperature differences (half, full and 1.5 times the design difference, from Climate) across, `=$B19*C$18`, plus a bar chart of the heat flux at the design difference. `mixed` and `fill2d` unchanged.
- `stuck`: red dashed frames on C5:C7, the Type A thumbnail with frames stepping down B10 to B12, "expected" text; `pertype` shows each type's own formula.
- `deleted`: the Energy sheet in a second window with the unframed sum cell (2,042 W), the card `2,747.0 − 704.5 = 2,042.5 W`; `guard`: deleting row 10 turns even `=F3*$D$10` into `=F3*#REF!`; `names` is one short clause (two names) before the recap.
- German "Typ" everywhere in clauses and cards, English "single quotes", recap rule texts at one size.
- Program windows were re-fitted to the widened Program columns (A 26, B/C/F 17, D/E 12): stuck, sheet and pertype show A:C, deleted uses a 9.0-unit window with A:D, mixed and fill2d A:E rows 10–13. No `###`, no clipped value.

Final capture: en 53 strict findings in 15 of 23 frames; de run reviewed frame by frame (hook, total, f4, mixed2, deleted, stuck, guard, recap, names, threed, pertype, slide read this round; the others unchanged from the earlier round). X engine tests 84 OK; narration/title/step contract OK; the long native-lifecycle presentation test was not re-run.

## Sweep round (sweep-excel-references.md)

- Blocker: the fill-down now stops at the window row; the new clause `back` shows F9 `=D9*E9*attached_factor` with the card "b = 0.50"; `total` then sums all seven rows (30.92 W/K = H_T). 24 clauses.
- `hook` selects D3, so the formula bar (`=B2*B3`) matches the visible 3.00 m / 12.0 m²; `stuck` crops the Total row (rows 3–7).
- German labels: scene-local relabel on Type A (Wand links, Wand rechts, Rückwand) and Program C3 (Nettofläche) so nothing clips; shared widths unchanged. "Transmissionsverlust H_T" in the workbook JSON (4 type sheets); narration says the same.
- `mixed2` units sit in the header cells (q, W/m²; U, W/m²K; K); `deleted` has no tooltip over Type D, the Energy label is shortened to Heizlast/Heating load in the small window; Name Manager moved below the formula card.
- Open (toolkit): the file name under the search pill in narrow windows (stuck thumbnail, deleted Energy window); German "Transmissionsverlust H_T" is cut at the window edge in `hook` row 11.
- Capture needed the BIM `P()` fix (it briefly returned a tuple and broke every window); en 49 strict findings in 16 of 24 frames. Changed frames read in both languages (hook, back, total, stuck, deleted, mixed2, names). X engine tests 84 OK.

## Title-bar re-capture

After the toolkit title-bar fix the whole scene was re-captured (en 42 strict findings in 16 of 24 frames). `hook` now shows rows 1–10, so the German "Transmissionsverlust H_T" is no longer clipped. hook, stuck and deleted were read in en and de: the file name is no longer covered by the search pill and no value is clipped.

## Technique notes

- No `__deepcopy__` override any more; `Live` builders are lambdas, never bound methods (`deepcopy` of a bound method copies the scene).
- Workbook states live in a module dict, not on the scene, for the same reason.
- Values that move use `ValueTracker` + `live`/`bind`; cell reveal during fill is bound to a row tracker, so no cell is typed.

## Keyframe review

Captured with `capture.py 📊️excel 📍️references Beat2_References en|de` (20 frames each); every frame was read at least once in `de` and the key frames also in `en`. Fixed during review: ### cells (window widened, 11 rows for 16.2 px text, type-sheet column widths), card/capsule overlaps, dim text of formula cards on the dark stage (ink), recap header overlap, tower overflow of the four chips, thumbnail squeeze, German captions over two lines (shortened), unlocalized `#REF!` tag in the guard card.

Strict layout report: 87 findings in 15 of 20 frames (en) and 119 in 17 of 20 (de); all remaining ones are by design per `xl-toolkit.md`: window gridlines through cell text, reference frames and the edit-mode cell editor lying over cells (pointer, shift, names, slide), two-pixel token adjacency in `xl_formula_text`. No clipped or off-frame text, no text over the caption band.

## Known limits

- None open from the toolkit side: German type labels and the `-` glyph are fixed by the engine and toolkit updates.

## Tests

`python X/🧪️tests/🎭️presentation/🐍️.py`: 11 tests, 2 failures, both in other scenes under construction (`Beat3_Program`, `Beat4_Energy`: "narration without step"); every contract test for `Beat2_References` (narration, title, 14–24 clauses, step keys, literal imports, provenance, native Cairo lifecycle en and de) passes.
