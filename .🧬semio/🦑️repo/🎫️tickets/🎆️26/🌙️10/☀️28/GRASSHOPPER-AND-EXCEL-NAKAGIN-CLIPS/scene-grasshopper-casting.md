# Scene report: grasshopper_casting (`Beat4_Casting`, topic 🫗️casting)

File: `A/🦗️grasshopper/🎬️clip/🫗️casting/🐍️.py` (A = `🎓️teaching/🏛️architecture`). Storyboard: `T/storyboard-grasshopper-casting.md`. Facts: `T/research-casting.md`.

## What was built

One scene, 21 clauses, both languages authored natively (caption check: every clause fits two lines, `T/gh-casting/captions.py`). Left half: a Grasshopper canvas sheet that shows the engine view of the `casting` canvas for the current clause (real component names, real wires, states and messages from `evaluate`). Right half: a Rhino viewport with the Nakagin drawing (tower, capsule type A, or a plan of the Tower B west facade).

Every number and value comes from the engine or the Nakagin model: panel contents from `Solution.tree`, rounding from `cast(...)`, ties by overriding the slider (`sl` 10.6, 2.5, 3.5), planes from `plc`, `lpl`, `cpl`, the line from `ln`, domain cuts from Divide Domain, the failure text from `Solution.msgs("cbx")`, matrix colours from `rules()` (`ok`, `basis`). The slot is `(0,1,3,3,0)` of Tower B (west facade, storey 3).

Evidence handling (from `research-casting.md`): verified and community rows are shown as fact (Number to Integer ties to even with Rutten 2010, Boolean to Integer, Point to Plane, Line to Plane, failure message, cast order). Rows with `basis: "unverified"` in the engine (Text "2.5" to Number, Number to Domain, Circle to Curve, Box to Brep, Number to Boolean) show the on-screen marker "teaching assumption — check in Rhino 8" / "Lehrannahme — in Rhino 8 prüfen"; the marker is decided by the engine rule, not by the scene. In the matrix those cells have dashed outlines and a legend entry.

## Clauses

hook (types of Width, Storeys, Plane; "teaching model" said once), mismatch (Panel text into Width), text_number (2.5 m wide capsule, marker), order (four-step cast order, McNeel source), storeys (10.6 becomes 11, eleven capsule storeys of Tower B lit), ties (slider 10.6, 2.5, 3.5 with number line, 2 and 4), deliberate (Floor before Integer, 10 storeys), point_plane (world plane at the slot origin, plan view), wrong_way (capsule on the world plane turned by the engine-derived angle, window arrows), line_plane (line becomes Z axis), explicit (Construct Plane from origin and axes, capsule correct), domain (4.0 to 0 To 4, Divide Domain cuts drawn on the capsule length, marker), circle_curve (window outline, marker), box_brep (six faces tinted one after another, marker), bool_int (toggle flips, Integer 1 and 0, window follows), int_bool (slider 1, 0, 2, window hides and shows, marker), fail (red Center Box, balloon "Data conversion failed from Text to Number", ghost capsule), fix (Value List, grey component, capsule 2.50 m), matrix (empty grid), matrix_fill (ten cells in the order shown), recap (Panels before and after the cast, term legend en/de).

Live values: the slider tracker drives the Integer cast, the Panels, the tower lit storeys and the number line; the toggle and slider in bool_int and int_bool drive the Panels and the capsule window.

## Keyframe review notes

Captured en and de with `T/capture.py` and looked at every PNG. Fixed during review: three-line captions (shortened), clipped slider and Box widgets (layout by columns, per view overrides), dark text on the light sheet, live Panels not following the slider, toggle and Panel placed wrongly, pale plan colours in the light viewport, tower overlapping value chips, the failure balloon covering the Height parameter, split matrix cell text too small, Panel decimals now follow the language (de `10,6`; raw text panels keep `2.5` and `Type A`).

Remaining strict findings are justified: labels, axes arrows and chips inside the Rhino viewport sit on its ground grid (strokes behind text on a light background, same as in Rhino); one wire passes under a slider domain label. No overlaps with the caption band, no text under 11 px.

Plan views use the scene's own 2D drawing (X east, Y north) because the library plan view is rotated; capsule placement uses the Nakagin `Plane` objects of the slots.

## Not done / hand-over

- The presentation cases file `🧪️tests/🎭️presentation/🔣️.cases.json` has `chapters: []` shared by all scene agents; the scene is not pinned there to avoid concurrent edits (add `{folder: "🫗️casting", scene: "Beat4_Casting", topic: "casting", keys: [...]}` when the registrar finalises it).
- Standalone entry `🫗️casting/🎬️clip/🐍️.py` and registry rows belong to the registration agent.
- No change to `🧱️model/🔣️.json`: the existing `casting` canvas views were sufficient.

## Test result

Series presentation tests (`G/🧪️tests/🎭️presentation/🐍️.py`): the toolkit tests pass (widget zoo, every engine view, states and panels, tree notation, bezier). The full `test_native_lifecycle` renders every scene of the series in both languages and exceeded the 15 minute limit of the full run, so the casting scene was run alone with `T/gh-casting/lifecycle.py` (same `lifecycle` method of the contract): en OK (84 s), de OK (80 s), together with the narration, dispatch and step key, import, provenance and explicit-language tests of the contract: 5 tests OK.
Keyframes: `T/🗑️generated/frames/🦗️grasshopper/Beat4_Casting/{en,de}/` (strict findings 26 en, 27 de, all of the justified kinds above). Helpers kept in `T/gh-casting/` (`captions.py`, `probe.py`, `widths.py`, `slider.py`, `lifecycle.py`).

## Audit round (T/audit-grasshopper-casting.md), coordinator decisions applied

- Rounding: the 10.6 to 11 and 2.5 to 2 / 3.5 to 4 examples stay; storeys, ties, recap carry the marker "teaching assumption — check in Rhino 8" (forced for `Number>Integer`, although the engine rule says verified) and storeys and ties add the source line "D. Rutten, McNeel forum 2010". The ties caption reads "Grasshopper rounds; at exactly .5 it rounds to the even number ... Check this in Rhino 8." The recap says "Successful casts give no message."
- Marker rule in `_mark`: every rule whose basis is not `verified` (community included) shows it; the markers sit on text_number, storeys, ties, point_plane, wrong_way, line_plane, domain, circle_curve, box_brep, int_bool (now `Number>Boolean`), fix (Text to Number success) and recap. In the matrix those cells are dashed (Number to Integer, Point to Plane, Line to Plane included); only Boolean to Integer is solid; legend "works (verified)".
- Orient `A`: the engine canvas now wires a world-XY `Plane` parameter into Orient `Source` (nickname A) in `point-plane` and `explicit` (`gh-engine/build_model.py` casting section, JSON regenerated; Python 57 tests, schema and tree/cast oracle pass). The `explicit` view was relaid; its German title is "Ebene ausdrücklich konstruieren".
- int_bool: the view is now "Number into Boolean" with an integer-rounded slider shown without decimals (0 to 3); the marker follows the engine rule `Number>Boolean`.
- Fix: the Value List shows its expression "Type A = 2.5".
- German: error text stays English with the German engine explanation in a remark balloon beside it; grammar list applied (Capsule Tower, mit den Weltachsen, nur geraten, Scheitern beide, bleibt der Platz leer, richtet ... aus, Umwandlung as the single term, quoted message), domain caption "0 To 4" matches the Panel.
- Contrast: gold dimension and highlight ink on the light viewport darkened to `#7A5C00` (context manager around the chapter, restored afterwards), marker `#9A5B00`, chips, axis labels, face chips and the failure label darkened; wrong_way fades the correct capsules to 30 %; the failure label moved below the capsule; circle_curve drops the stray diagonal.
- Not changed: Floor wire route and unverified Floor output pins (check in Rhino 8 before filming), plan row "2.3" (coordinator file).

Re-captured en and de (strict findings 23 and 24, all of the justified kinds above plus labels on the viewport grid); lifecycle en 89 s and de 82 s OK with the contract tests (6 tests OK).

Update: the scene-side override for `Number>Integer` is removed. The engine rule now has basis `conflicting`, and the marker and the dashed matrix cell follow `rules()[id]["basis"]` (anything but `verified` shows them). Re-captured `storeys` and `matrix_fill` (en) to confirm; marker and source line unchanged.

## Final verification round (T/verify-grasshopper-casting.md)

Storeys and recap captions no longer state 10.6 to 11 as fact ("rounds, here to 11; check this in Rhino 8"; recap "10.6 shows 11 (check in Rhino 8)"), en and de. The number line draws both ties as arrows (2.5 to 2 and 3.5 to 4). Darker inks for hook chips, marker, dimension labels, axis labels (now on a light backing plate); wrong_way draws the world-plane capsule translucent so the slot capsules show through; the German fail frame labels the English balloon "Grasshopper-Meldung (englisch)" and keeps a shortened German explanation inside the canvas. The English recap keeps the bilingual glossary on purpose. Re-captured all 21 keys en and de (strict findings 19 en, 20 de, grid-line kinds).

## Quality round (T/quality-grasshopper-casting.md)

The scene was rebuilt to 22 clauses: teaser, types, mismatch, text_number, storeys, ties, deliberate, point_plane, wrong_way, line_plane, explicit, domain, geometry, bool_int, int_bool, locale, fail, order, fix, matrix, matrix_fill, recap.

- Opens with the wrong-way teaser: two 3D capsules on the west facade of Tower B (green slot plane, translucent red world plane), window arrows, a 90° arc, W/N labels and a legend. wrong_way and explicit reuse the stage; in explicit the red capsule rotates into the green one (live tracker) and the red overlay, arc and arrow disappear at the end. line_plane and point_plane use the same 3D stage with plane axes and the core-to-slot line. The viewport label is "Perspective"/"Perspektive".
- Casts are shown happening: a typed token rides the wire (text_number: Text "2.5" becomes Number 2.5; storeys: Number 10.6 becomes Integer 11 and ".6 dropped" appears). Tower B gets numbered storey ticks that follow the count (live), a red crossed ghost "11" appears in deliberate, and both ties are drawn as arrows on the number line.
- Markers are lighter: one dashed outline around the sink widget plus one footnote ("check in Rhino 8", with the Rutten source line on storeys and ties) instead of a banner. A rule gets it when `rules()[id]["basis"]` is not `verified` (no scene override). The unit/locale mistake is a new clause `locale` using the engine view `locale` (Panel "2,5" to a Number parameter) and the engine rule `Text>Number!comma` (basis unverified, added by GH-ENGINE); the Number parameter turns orange with the real warning text.
- Circle to Curve and Box to Brep are merged into one clause (`geometry`); the order ladder follows the failure as its explanation and runs a "Type A" token down to step 4; the failed Panel reads "null".
- The matrix is grouped by families (numbers, truth, text, geometry) with grey hatching where no cast exists, faint green where the engine knows a cast, "=" on the diagonal, the shown casts highlighted (dashed when not verified) and the one-line family rule. The recap shows three habit cards (check with a Panel, build planes explicitly, labels are no numbers) beside the correct capsule on its slot; the English recap has no German glossary, the German recap keeps Panel and Value List in English.
- Engine side (my casting section only): view `locale` with objects `tde`, `tdn`, `tdo`; the G test that lists the non-ok demos now expects `tdn: warning`. Python 57 tests, schema and oracle tests pass.

Re-captured and reviewed en and de (22 keys each). The last full capture had 48 (en) and 45 (de) strict findings, all of the viewport grid or backing-plate kinds; only matrix and matrix_fill were re-captured after the final grid fix (0 findings). Lifecycle en 85 s, de 98 s OK with the contract tests (6 tests OK).

## Final sweep (T/sweep-grasshopper-casting.md)

Order ladder steps 1 to 3 are red with "no match" / "passt nicht". German numbers go through `_pn` everywhere (number-line labels "2,5" and "3,5", "Typ A = 2,5", Panel rows, the plane Panel label "Ebene bei"). User-chosen labels are localized in German by the scene (`LABELS_DE`: Slot-Ursprung, Kern zu Slot, Kapselkörper, Kapsel, Fenster, Länge, Höhe, Breite, Etagen, Typ A, X-Achse, Y-Achse, Ebene bei); built-in Grasshopper component and type names stay English. The German narration says Typ A and Etagen consistently. Re-captured en and de (22 keys each), changed frames reviewed; lifecycle and contract tests OK. During the round a foreign refactor of the BIM drafting `P` helper briefly broke the Nakagin drawings; captures were redone after its owner fixed it.

Consistency: German uses "Geschosse" (narration and slider/param label, matching tower and anatomy). The storeys, ties and deliberate views were re-laid out (Panels below slider and parameter) so the longer label fits; re-captured en and de, reviewed. Contract tests OK; the full lifecycle was last run before this layout change.
