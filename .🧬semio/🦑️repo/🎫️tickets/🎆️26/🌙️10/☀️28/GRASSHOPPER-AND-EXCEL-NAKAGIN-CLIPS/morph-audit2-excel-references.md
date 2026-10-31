# Morph audit 2: excel references (`Beat2_References`)

Scene: `C:/git/semio/🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📍️references/🐍️.py` (last edit 2026-10-29 16:53:29). Plan: `morph-excel-references.md` (M1-M14). Previous audit: `morph-audit-excel-references.md`. Built report: `morph-excel-references-done.md` (17:28).
Frames: `🗑️generated/morph/📊️excel/Beat2_References/{en,de}/` (index.md + `<key>-<n>-mid.png`, `<key>-end.png`). Contact sheets used for reading: `🗑️generated/audit2/*.png` (temporary, delete at ticket close). Full-size reads: `en/pointer-03-mid.png`, `en/mixed-03-mid.png`, `en/slide-03-mid.png`, `en/threed-03-mid.png`, `en/absolute-03-mid.png`, `en/lights-03-mid.png`, `en/guard-03-mid.png`, `en/hook-02-mid.png`, `de/mixed2-04-mid.png`.

Stale check: cutoff = 16:53:29 minus 60 min = 15:53:29. The earliest en frame is 16:54 (hook-02-mid), en ends 17:10, de frames 17:11 to 17:26. No frame is stale; all were judged.

Coverage: every planned beat has frames in both languages, except the recap end: `recap-03-mid.png` exists in en and de, but `recap-end.png` and `recap-04-mid.png` (listed in index.md) are not on disk. Frames for `back` (not a planned beat) exist and are noted in section 2 only.

## 1. Planned beats: status against the previous audit

Status: BUILT = recognisable object in transformation, identity and colour readable, origin visible. WEAK = blob, crossfade, unreadable text, collision. MISSING = no frame (none found in this pass).
FIXED = was WEAK or MISSING, now BUILT. STILL WEAK = frames exist, still WEAK.

| beat | clause | dir | previous | now | frame evidence (en / de) |
| --- | --- | --- | --- | --- | --- |
| M1 width dim -> B2 chip | hook | C->A | MISSING | STILL WEAK | en/hook-02-mid: a blue pill at the right edge of the sheet over the D column (`10.0 m2` values), text unreadable, no leader, no ghost of the capsule dimension. de/hook-02-mid same. |
| M2 B2 ticks, capsules widen, D3/D4/D7/D9 flash | hook | A->C | MISSING | STILL WEAK | en/hook-04-mid: B2 reads `2.99 m` in a blue frame, so the tick is visible. Capsule widening and the four dependant flashes are not visible; no tether. de/hook-04-mid same. |
| M3 B2/B3 -> width/length dims, `=B2*B3` split | pointer | A->C | MISSING | FIXED (partial) | en/pointer-03-mid (full size): blue `2.50 m` and red `4.00 m` dimension labels sit on the capsule, colour carried from the frames. en/pointer-05-mid: the labels read `2.99 m` / `4.00 m` on the capsule views. de/pointer-05-mid same. The `=B2*B3` split into two labels is not on any frame (done file also says it is not built). |
| M4 four hatched faces -> D3/D4/D7/D9 area chips | pointer | C->A | MISSING | STILL WEAK | en/pointer-03-mid (full size): grey rounded pills cross the C column labels (`Floor`, `Left side wall`); the hatch is not readable; D3 is empty mid-flight. de/pointer-03-mid: pills over `Wand links` / `Fenster`. |
| M5 `=D3*E3` retexts to `=D4*E4` | shift | A->A | MISSING | STILL WEAK | en/shift-03-mid and shift-end: the retext happens in the side card (F3 and F4 rows). The live formula in F3 stays `=D3*E3` in the formula bar on shift-end. de/shift-03-mid same. |
| M6 F3..F9 values -> lit face tints | lights | A->C | MISSING | STILL WEAK | en/lights-03-mid (full size): the capsule faces fill with tint in place, and F3:F6 get yellow outlines. No number token travels from a cell to a face. de/lights-03-mid same. |
| M7 seven U·A tokens gather into F10 | total | A->C | MISSING | STILL WEAK | en/total-03-mid: a yellow-green blob sits near row 10; the seven tokens are not separate readable objects. en/total-end: `=SUM(F3:F9)` in F10 and `30.92 W/K` in B11; the tether F10 -> B11 is not visible. de/total-03-mid same. |
| M8 `D10` -> `$D$10`, `D$10`, `$D10` with padlocks | f4 | A->A | WEAK | STILL WEAK | en/f4-03-mid: the four key-cycle rows in the card show `$` marks; the live token in the formula bar does not change. de/f4-03-mid same. |
| M9 seven tethers G3..G9 -> D10, loss numbers onto faces | absolute | A->C | MISSING | STILL WEAK | en/absolute-03-mid (full size): the `20.0 K` chip from D10 lands on the capsule (split started, identity readable); the F4 key-cycle card overlaps the capsule views. en/absolute-05-mid and absolute-end: loss values are red pills in column G, not on the faces; no seven tethers. de/absolute-03-mid same. |
| M10a wrong values become red faces | slide | A->C | WEAK | STILL WEAK | en/slide-03-mid (full size): red shards sit on the G column values (G5:G9); the capsule floor face is red-pink. en/slide-end: faces red. de/slide-03-mid: shards over `5 W`, `6 W`. No ghost of the origin G4 cell. |
| M10b corrected loss chips onto faces, retext to `$D$10` | repair | A->C | WEAK | STILL WEAK | en/repair-03-mid: yellow outlined pills cluster on the capsule and over G column values, text unreadable at mid. en/repair-04-mid: the faces show yellow tint; the G values are back in place at repair-end. de/repair-03-mid same. |
| M11 U-values down, dT across, one mixed formula, bars | mixed2 | C->A, A->C | WEAK | STILL WEAK | en/mixed2-03-mid: table dimmed by fade, no card (the "Lock the input" card from the round-2 state is gone; FIXED as clutter). en/mixed2-04-mid: chart fades in, grey. de/mixed2-04-mid (full size): value chips `10.0` and `12.0` sit on the blue bars of the chart; the face-to-row-label morph is not on any frame. en/mixed2-05-mid: fade into the count table. |
| M12a lit capsules gather into COUNTIFS count | mixed | C->A | MISSING | FIXED | en/mixed-03-mid (full size): grey hexagonal capsule silhouettes with yellow lit ends stream from the tower to B11; origin (tower) visible. en/mixed-05-mid: B11 reads `18`. de/mixed-03-mid same. Caveat: the capsules are grey/yellow, not their type colour as planned, and the stream crosses the `Type B`..`Type D` header cells. |
| M12b count splits back into capsules for the next cell | fill2d | A->C | WEAK | STILL WEAK | en/fill2d-03-mid: B11 already shows `18` and the lit capsules are on the tower; no split chip visible in flight. en/fill2d-end: result shown. de/fill2d-03-mid same. |
| M13a plan floor face -> area chip, travel to Program C4 | sheet | C->A | MISSING | STILL WEAK | en/sheet-03-mid: crossfade of the sheet window with a pale capsule view; no floor chip is visible on the way to C4. en/sheet-05-mid: red pills in column C of the Type B..D rows. de/sheet-03-mid same crossfade. index.md lists PartMorph for sheet-03, but the frame shows a window fade. |
| M13b stuck: wrong chips shatter onto B10..B12 | stuck | C->A | WEAK | STILL WEAK | en/stuck-03-mid: small red shards at C5:C7 on the sheet, too small to read. en/stuck-04-mid: card only. de/stuck-04-mid: coloured tab pills (`Typ A`..`Typ D`) sit on the tab bar, which is the collision noted before. |
| M13c four type chips fly from tabs to formula rows | pertype | C->A | WEAK | STILL WEAK | en/pertype-03-mid: teal and pink pills over the `Type C`/`Type D` tabs and the `Total` row; labels unreadable. en/pertype-04-mid and pertype-05-mid: the four capsules (teal, orange, green, red) appear beside the Energy sheet; index.md says FadeIn x3 for this clause, so the capsules are not morphed. de/pertype-03-mid and pertype-04-mid same. |
| M14a four capsules -> tab chips -> B13 values -> sum | threed | C->A, A->C | MISSING | STILL WEAK | en/threed-03-mid (full size): capsule colours carry (teal, orange, green, red); a yellow `681 W` chip and grey-green silhouettes sit below the labels, not on a path to B13. en/threed-04-mid: capsule B dims, `2,042 W` in B16. index.md still lists FadeIn x7 for threed-04, so the values are faded in. de/threed-03-mid same. |
| M14b span: Type B chip leaves bracket, sum shrinks | span | A->A | MISSING | STILL WEAK | en/span-03-mid: the Type B capsule is desaturated (identity kept). en/span-04-mid: the sum is in the card (`2,723.3 - 680.8 = 2,042.5 W`); the Type C row shows `#REF!`. The value flight is not on the frames. index.md lists FadeIn x5 for span-04, so the values are faded in. de/span-03-mid same. |
| M14c Type C chip + value shatter into `#REF!`, sum shrinks | deleted | A->C | WEAK | STILL WEAK | en/deleted-03-mid: a teal pill sits in the card, not on the formula text. en/deleted-05-mid: crossfade to the guard card, no shards. en/deleted-end: `#REFI` in G3 and the `before/after` card. de/deleted-03-mid same. |
| M14d `$D$10` -> `#REF!`, padlock breaks | guard | A->A | MISSING | STILL WEAK | en/guard-03-mid (full size): the live formula bar changes from `=F3*$D$10` to `=F3*#REF!` (retext visible); the card `after` line is garbled mid-retext. No padlock break on any frame. guard-04: card retext only. de/guard-03-mid same. |
| M14e `Climate!$B$2` -> `theta_i` pills | names | A->A | MISSING | STILL WEAK | en/names-03-mid: formula text `Climate!$B$2` and `theta_i` overlap in the formula bar. en/names-04-mid: crossfade into the legend table with yellow ellipses; the Name Manager rows do not receive the pills. de/names-03-mid same. |
| M14f six formulas fly into the legend rows | recap | C->A | WEAK | STILL WEAK | en/recap-03-mid: yellow ellipses fly across the `On copy` column and over the `only the open part shifts` text; `Mixed` is partly covered. de/recap-03-mid: the `Gemischt` row is covered. No recap-end frame on disk. |

Count: 23 rows (the 14 planned beats, with M10, M12, M13 and M14 split into their sub-beats a-f). BUILT 0 (strict). FIXED 2 (M3 partial, M12a). STILL WEAK 21. MISSING 0.

Previous audit: 9 WEAK rows (f4, slide, repair, mixed2, fill2d, stuck, pertype, deleted, recap) and 14 MISSING rows (hook M1 and M2, pointer M3 and M4, shift, lights, total, absolute, mixed, sheet, threed, span, guard, names). Now: the 9 WEAK rows are all STILL WEAK; of the 14 MISSING rows, 2 are FIXED (M3, M12a) and 12 are STILL WEAK, because their frames now exist but still do not show a readable morph. Frames for M1-M14 exist now; the previous "missing frames" finding is resolved, the morph findings are not.

## 2. Fade-swaps and cuts between a concrete thing and its abstract stand-in (index.md)

- `pertype-04` (en and de): FadeOut x2 and FadeIn x3 (en also Swarm). The four type capsules arrive by FadeIn beside the Energy sheet. This is the concrete end of the type chain. The done file says this clause became scatter; index.md does not show it.
- `threed-04` (en and de): FadeOut x8 and FadeIn x7. The four B13 values fade in before the sum. Identity is lost at the capsule-to-value handover.
- `span-04` (en and de): FadeOut x8 and FadeIn x5. The Type B value fades out and the sum fades in. Done file says "capsules replace FadeIn"; index.md still shows FadeIn.
- `mixed2-03` (en and de): FadeOut x9 then chart FadeIn. The faces do not morph into row labels.
- `mixed2-05` (en and de): FadeOut x2 and FadeIn x2; bars are not a morph of the value chip.
- `stuck-04` (en and de): FadeOut x10 and FadeIn x2. Wrong values fade out instead of morphing onto B10:B12.
- `deleted-05` (en and de): FadeOut x5 and FadeIn x2. The `#REF!` chip fades into the guard card.
- `absolute-05` (en and de): FadeOut x3 and FadeIn x5. The loss values appear by fade, not on the faces.
- `lights-03` (en and de): FadeOut x2, FadeIn, Swarm. The values to tint step has no visible number token.
- `total-03` (en and de): FadeOut x3, FadeIn x2, Swarm. The seven tokens are not visible as separate objects.
- `recap-03` (en and de): Swarm only; windows and cards are not faded out before the chips fly (see fix 17).
- Window swaps (relative, fixed, sheet): acceptable as window changes per the plan.
- Not planned: `back-03` has FadeIn plus Morph (en) and PartMorph (de). The factor chip morph from F9 to the back face is on the frames; it is not in the plan's beat list, so it is not scored.

## 3. Directions and identity chains

- Concrete -> abstract (C->A): BUILT in M12a (lit capsules gather into the COUNTIFS count, en and de). WEAK in M4 (faces to area chips), M13a (floor to C4), M13b (stuck shards), M13c (type chips), M14f (recap terms), M14c (shatter).
- Abstract -> concrete (A->C): BUILT in M3 (blue and red dimension labels on the capsule). WEAK in M2, M6 (faces lit by values), M9 (tethers and loss faces), M10a/b (red and yellow faces), M11 (bars), M14a (capsule to value).
- Both directions on one clause: pointer (M3 A->C and M4 C->A) is the only clause where both directions are on frames, and only the A->C half is built. Hook has C->A (M1) on frames but it is unreadable.
- Width chain (W: dimension -> B2 -> `=B2*B3` -> D3 -> roof -> bar): M3 gives the blue/red dimension labels on the capsule. The `=B2*B3` split is missing. The chain is broken at the split.
- Surface chain (S: face -> area chip -> F value -> tint -> loss): broken at M4 (no hatch identity), M6 (no value-to-face token) and M9 (no tethers).
- Type chain (T: capsule -> tab chip -> formula chip -> B13 value -> sum -> shatter): colours carry from pertype through threed/span to recap, but the morph between capsule and value is faded (pertype-04, threed-04, span-04). Broken at pertype and threed.

## 4. Collisions and clutter

- `pointer-03` (en and de): grey face pills cover the C column labels (`Floor`, `Left side wall`, `Wand links`).
- `hook-02` (en and de): the blue pill sits over the D column values (`10.0 m2`).
- `mixed-03` (en and de): the capsule stream crosses the `Type B`..`Type D` header row and the Type A cell border; the stream reaches B11 over the header.
- `absolute-03` (en and de): the F4 key-cycle card overlaps the capsule views and the `20.0 K` chip.
- `slide-03` (en and de): red shards sit on the G column values; they hide the `5 W` and `6 W` text.
- `repair-03` (en and de): yellow pills cover the G column values and the capsule top view.
- `pertype-03` (en and de): pills cover the `Type C`/`Type D` tabs, the `Total` row and `Klima`.
- `stuck-04` (de): coloured tab pills sit on the tab bar.
- `names-03` (en and de): `Climate!$B$2` and `theta_i` overlap in the formula bar.
- `recap-03` (en and de): term ellipses cross the `On copy` text and the `Mixed` / `Gemischt` row labels.
- Fixed since the previous audit: the `Lock the input` card no longer stays on the right in `mixed2-03` (en and de); the `deleted` sum chip is in the card corner, not over the formula text.

## 5. Verdict

FIX. No beat is BUILT strictly; 2 of 23 rows are FIXED (M3 partial, M12a). 21 rows remain STILL WEAK and none are MISSING. Frames are current for every beat.

Fix list (exact, short; line numbers refer to the functions in the scene file, since the line numbers in the previous list are stale):

1. `present_references` (hook): give M1 a leader and a 14px label that clears the D column; add the M2 tether from B2 to the four dashed frames D3/D4/D7/D9 with a flash.
2. `present_references` (pointer): keep the hatch on the flying face silhouettes and route them above the C column (`path_arc`); write the area text in the D cell on arrival; add the `=B2*B3` split into two dimension labels (M3).
3. `present_references` (shift): retext the live F3 formula token in the formula bar, not only the card.
4. `present_references` (lights): morph each F value token (F3..F9) onto its face with `path_arc` before the tint.
5. `present_references` (total): make the seven tokens separate visible objects in flight; tether F10 to B11.
6. `present_references` (f4): retext the live reference in the formula bar, not only the card rows; show the padlock on the column and row parts.
7. `present_references` (absolute): draw the seven tethers G3..G9 to D10 in the margin (not over F text); morph loss numbers onto the faces; move the F4 card off the capsule views.
8. `present_references` (slide, repair): shards keep a ghost at the origin G cell and fall into the left margin, off the G values; repair pills carry their value text.
9. `_stage_two_way` (mixed2): morph U-value faces into row labels; grow bars from the value chips, not by fade.
10. `_stage_two_way` (fill2d): keep the split chip visible in flight from B11 to the capsules; confirm with a mid frame.
11. `_stage_sheets` (sheet): the floor chip must be visible in flight from the plan view to C4, with a ghost at B9; no window crossfade.
12. `_stage_sheets` (stuck): drop the coloured chips on the cells B10:B12, not on the tab bar.
13. `_stage_sheets` (pertype): replace the FadeIn of the four capsules with a morph from the tab chips; route chips below the Total row.
14. `_stage_sheets` (threed, span): replace the FadeIn of the B13 values with a morph from the capsules; show the Type B value flying out of the bracket to B16; update index.md if the FadeIn is gone.
15. `_stage_failures` (deleted): make the `#REF!` shatter visible in a mid frame (sample or extend the shatter); keep the teal pill off the formula text.
16. `_stage_failures` (guard): show the padlock breaking; retext the card after it settles so the `after` line is not garbled.
17. `_stage_names_and_recap` (names): avoid the overlap of `Climate!$B$2` and `theta_i` in the formula bar; morph the pills onto the Name Manager rows.
18. `_stage_names_and_recap` (recap): fade the windows and cards before the term chips fly; keep chips off the table text; capture the missing `recap-end` and `recap-04` frames (en and de).
19. Capture: rerun for en and de; the done file's claim that pertype, threed and span are scatter/morph is not what index.md shows (FadeIn still listed), so re-check the done file after the fixes.
