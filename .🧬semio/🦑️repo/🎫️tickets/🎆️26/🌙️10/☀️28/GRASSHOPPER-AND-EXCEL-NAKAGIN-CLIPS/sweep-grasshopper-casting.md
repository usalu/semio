# Final Sweep: Grasshopper Casting (Beat4_Casting)

Series: 🦗️grasshopper. Scene: `C:/git/semio/🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🫗️casting/🐍️.py`, class `Beat4_Casting`.
Frames: `T/🗑️generated/frames/🦗️grasshopper/Beat4_Casting/{en,de}/` (22 clauses × 2 languages = 44 PNG, all read, in NARRATION order).

## Verdict: FIX

No blockers. Four majors, all in the current scene code.

## Freshness

- Scene file last written 04:27. PNGs are dated 04:12 to 04:33.
- No PNG is older than the scene file by more than 30 minutes. The teaser to fail frames (04:12 to 04:18) predate the 04:27 edit by up to 15 minutes, so they may not show that edit.

## Fixes

1. **order, en and de (major, caption contradicts picture).** Steps 1 to 3 are filled green (GOOD) and read "Same type: passes" / "Gleicher Typ: geht durch", yet each carries a red ×. The caption says Type A fails every step.
   Fix in `_order` (scene lines 376 to 377): tone steps 1 to 3 as SERV like step 4. Reword EN "1  Same type: passes" to "1  Same type: no match", and DE "1  Gleicher Typ: geht durch" to "1  Gleicher Typ: passt nicht".

2. **ties, de (major, German number format).** The number-line labels read "2.5" and "3.5" while the caption says "2,5 gibt 2, 3,5 gibt 4". Cause: `_numberline` (line 359) calls `_num(v)` without the `_pn` localisation.
   Fix: pass the labels through `self._pn(...)` so DE shows "2,5" and "3,5".

3. **fix, de (major, decimal format).** The value-list expression reads "Type A = 2.5". Cause: the `expression` text (line 677) uses `_num` without `_pn`.
   Fix: wrap it in `self._pn([...])`, giving "Type A = 2,5" in DE.

4. **point_plane, wrong_way, line_plane, de (major, English text in German video).** The Panel row "Plane at" is generated in `_lines` (line 55) and is never translated. It is not a Grasshopper UI name.
   Fix: make the label language-aware (for example "Ebene bei" in DE). Check Rhino 8 German output first, in case the Panel shows this text itself.

5. **Model labels, de (major, English text in German video).** These parameter and item labels stay English in the German frames: "Slot origin" (point_plane, wrong_way, explicit), "Core to slot" (line_plane), "Capsule body" (geometry), "Window" (bool_int, int_bool), "Length" and "Height" (domain, fail, fix), "Type A" (fail, order, fix, matrix, recap).
   Fix: give the model a German label per language, or have the owner accept these as names. Also unify the DE captions, which say "Type A" in fail and order but "Typ A" in fix.

## Clause-by-clause check (EN and DE)

| # | Clause | EN | DE |
|---|---|---|---|
| 1 | teaser | OK | OK |
| 2 | types | OK (Width 2.50 m, Storeys 11) | OK |
| 3 | mismatch | OK | OK |
| 4 | text_number | OK (2.5 m) | OK |
| 5 | storeys | OK (10.6 to 11) | OK |
| 6 | ties | OK | Fix 2 |
| 7 | deliberate | OK (Floor gives 10, 11 ghosted) | OK |
| 8 | point_plane | OK | Fix 4, 5 |
| 9 | wrong_way | OK (90°, N/W) | Fix 4, 5 |
| 10 | line_plane | OK | Fix 4, 5 |
| 11 | explicit | OK | Fix 5 |
| 12 | domain | OK (0 To 4, 4.0 m) | Fix 5 |
| 13 | geometry | OK | Fix 5 |
| 14 | bool_int | OK (True to 1) | Fix 5 |
| 15 | int_bool | OK (2 to True) | Fix 5 |
| 16 | locale | OK | OK |
| 17 | fail | OK | Fix 5 |
| 18 | order | Fix 1 | Fix 1 |
| 19 | fix | OK | Fix 3, 5 |
| 20 | matrix | OK (empty grid) | OK |
| 21 | matrix_fill | OK (values match CELLS) | OK |
| 22 | recap | OK | Fix 5 |

- Numbers agree with the nakagin model throughout: width 2.50 m, storeys 11, plane at slot {0;1;3;3}, length 4.0 m, Floor 10.6 to 10, banker's rounding 2.5 to 2 and 3.5 to 4.
- The German captions contain no grammar errors that I found.
- No empty or static stage, and no value shown without object or unit.

## Limits

- The tower tick numerals in the ties frame were not zoomed, because PIL is not available in the system Python.
- Minor cosmetic items (low-contrast matrix corner label, badge digits on the recap cards) are not listed, per the brief.
