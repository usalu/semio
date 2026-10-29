# Verify: grasshopper_casting, Beat4_Casting (en and de)

Scope: fresh keyframes re-captured for both languages (21 per language, 1280x720) with `T/capture.py`; all 42 PNG read; prior audit `T/audit-grasshopper-casting.md` checked item by item. Model authority: `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧱️model/🔣️.json` (rule `Number>Integer` basis now `conflicting`, source `rutten-2010`).

Capture logs: `T/🗑️generated/verify-cap-en.log`, `T/🗑️generated/verify-cap-de.log`. Frames: `T/🗑️generated/frames/🦗️grasshopper/Beat4_Casting/{en,de}/`. The first English run crashed on a cp1252 console write after the frames were saved; the rerun with `PYTHONUTF8=1` completed and is the one used here.

## Verdict

NOT READY. The rounding claim is now marked and the model basis is honest (`conflicting`), and most audit majors are fixed. Still open: the storeys and recap captions state 10.6 becomes 11 as fact, the contrast of chips and labels, the untranslated error quote in German, a bubble that leaves the canvas in the German fail frame, and the world-axis overlay on wrong_way.

## Status of audit blockers and majors

| audit item | status | evidence (en / de frame) |
| --- | --- | --- |
| B1 storeys: "10.6 becomes 11" unmarked, model basis `verified` | PARTLY | Model basis now `conflicting`. Marker "teaching assumption — check in Rhino 8" on the frame. Caption still says "Grasshopper rounds it to 11" as fact (de: "rundet auf 11"). `storeys.png` |
| B2 ties: "2.5 becomes 2, 3.5 becomes 4" unmarked | PARTLY | Marker present and caption hedged ("Check this in Rhino 8" / "In Rhino 8 prüfen"). Number line still shows only 3.5 to 4; 2.5 to 2 is not drawn. `ties.png` |
| B3 matrix cell "10.6 → 11" solid green | FIXED | Cell now dashed in both languages (zoomed crop checked). `matrix_fill.png` |
| M recap "10.6 → 11" panel unmarked; "A cast changes data silently" | PARTLY | Panel marked; contradiction fixed ("Successful casts give no message"). Caption "10.6 became 11" still stated as fact (see B1). `recap.png` |
| M matrix_fill "world XY" and "Z = line" solid | FIXED | Both dashed (de "Welt-XY", "Z = Linie"). |
| M point_plane world-axis claim unmarked | FIXED | Marker present. Model basis for `Point>Plane` still `community` (see N3). |
| M point_plane Orient input `A` unwired | FIXED | `A` wired from the world Plane node in both languages. |
| M wrong_way: unmarked; orange overlay hides capsules and "Y" | PARTLY | Marker present. Orange overlay still covers the slot capsules; the wrong capsule is not isolated and the "Y" label sits under the overlay edge. |
| M line_plane "X and Y still guessed" unmarked | FIXED | Marker present. |
| M fix: Value List expression not shown; success path unmarked | FIXED | "Type A = 2.5" shown under the dropdown; marker on frame. `fix.png` |
| M explicit: "Y" label green on green capsule | NOT FIXED | Label still near-zero contrast. `explicit.png` |
| M int_bool: Number slider vs "Integer" heading; marker keyed to Integer | FIXED | Slider is now an integer slider ("Window 2", range 0 to 3); heading "Number into Boolean" / "Zahl in Boolean". `int_bool.png` |
| M fail (de): English error text quoted in German narration, unmarked | NOT FIXED | Quote still in caption and bubble with no "check" marker on the frame. `fail.png` |
| M fail: red "no width, no capsule" crosses the axis and the ghost outline | FIXED | Label moved below the capsule in both languages. |
| M text_number: German parse of "2,5" unverified | PARTLY | Marker present on the frame. The German Panel still shows "2 . 5"; the parse is not tested. `text_number.png` |
| M hook: yellow chips ("Storeys 11", "Geschosse 11") low contrast | NOT FIXED | Still gold on near-white. `hook.png` |
| M yellow dimension labels and orange marker on light grey | NOT FIXED | "2.50 m", "4.0 m", "6 / 6" and the orange marker still about 2:1. `box_brep.png`, `domain.png` |
| Minor: German grammar (point_plane, line_plane, order, explicit heading, matrix caption, "Capsule Tower") | FIXED | All applied. |
| Minor: wrong_way de "dreht ... nach Norden" | FIXED | "richtet sie nach Norden aus". |
| Minor: domain Panel "0 To 4" vs caption "4.0" | NOT FIXED | Panel still "0 To 4" (en and de). `domain.png` |
| Minor: circle_curve Panel "Curve" shows type word | NOT FIXED | Still there; the stray diagonal line is gone. `circle_curve.png` |
| Minor: deliberate, Floor wire runs under the Floor body | NOT FIXED | `deliberate.png` |
| Minor: plan row "2.3" in `T/plan.md` | NOT RECHECKED | Not in this verification's scope. |

## New findings (this round)

| frame | lang | severity | defect | fix suggestion |
| --- | --- | --- | --- | --- |
| fail | de | major | The speech bubble "Der Text lässt sich nicht als Zahl lesen ..." starts above the canvas top edge (bubble top about y 103, canvas top about y 135) and covers the top inputs of Center Box. | Move the bubble inside the canvas and below the heading, or shorten it to two lines. |
| recap | en+de | major (confirm intent) | The glossary "Casting · Typumwandlung", "Parameter type · Parametertyp", "Wire · Verbindung", "Value List · Werteliste" is shown in the English frame too. The scene's `TERMS` tuple is not language-conditional (`🫗️casting/🐍️.py` line 43). An English viewer sees an unexplained German column. | Show the glossary only in the German version, or state in the English caption that it is a German glossary. |
| hook, box_brep, domain, explicit | en+de | minor | Gridline and leader-line hits in `layout.md` (23 en, 24 de strict findings) are grid lines through labels or the capsule, not overlapping content; no new text collisions beyond the bubble above. | None beyond the contrast fixes. |

## Fact check against the model and research

- Number to Integer: model basis is now `conflicting` (honest). The narration still states the 10.6 result as fact (see B1). Research section 3 leaves non-tie rounding unresolved.
- Point to Plane and Line to Plane: model basis is `community`, while `research-casting.md` row 10 says the exact axis orientation is UNVERIFIED. The frames carry the marker, so the drawing is covered; the model basis should be aligned (N3, minor).
- Nakagin values (width 2.50 m, Tower B 11 capsule storeys, window Ø 1.30 m, slot `{0;1;3;3}`) match `🏢️nakagin/🔣️.json`.

## Must change before READY

1. Storeys and recap captions (en and de): replace "rounds it to 11" and "10.6 became 11" with "check it in a Panel", or film the test first and update the model basis and research section 3.
2. Ties: draw 2.5 to 2 on the number line, or drop the example from the caption.
3. Contrast: hook chips, dimension labels, orange marker, explicit "Y" label (use a darker ink or a filled chip).
4. German fail frame: mark the English error wording as unverified, and fix the bubble so it stays inside the canvas.
5. wrong_way: draw only the wrong capsule at full strength and move the "Y" label off the overlay edge.
6. Recap: decide whether the German glossary belongs in the English video (see table).
7. Align `Point>Plane` and `Line>Plane` model basis with research row 10.
