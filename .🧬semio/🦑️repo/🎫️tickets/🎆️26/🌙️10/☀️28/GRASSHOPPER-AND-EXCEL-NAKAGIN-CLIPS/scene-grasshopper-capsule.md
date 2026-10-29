# Scene: grasshopper_capsule (`Beat2_Capsule`)

File: `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/💊️capsule/🐍️.py`. Storyboard: `storyboard-grasshopper-capsule.md` (the final clause list below replaces its table where they differ).

## What was built

One scene, 23 clauses, English and German authored natively. Left: the Grasshopper canvas (light sheet, real widgets from the canvas toolkit), right: a Rhino viewport frame with the Nakagin capsule in the same projection as the ground grid, so base plane, axes and dimensions line up.

- Every number comes from the model or the engine solution: slider values and domains from the `capsule` canvas (bound to `🏢️nakagin`), capsule dimensions read back from the evaluated Brep (`root_box`, `fillet_radius`, `window`), bolt points from the `bolt` output, types and the table from `capsule_types()`, planes from `slots()` (tower B, facade 0), the List Item result and the warning text from `evaluate`.
- Live values: the six sliders are `ValueTracker`s; one `Live` stage redraws the viewport (grid, base plane, lifted plane, capsule, dimension lines, bolts, envelope, mistake circle, window ring) from the engine solution; panels (centre of the box, volume, rotated plane, List Item result) are live too.
- Dimension colours follow the axes (width red = X, length green = Y, height blue = Z); radius, window diameter and window offset have their own colour that is also the colour bar on their slider.
- Three small definitions are built with `define()` in the scene (all evaluated by the engine, none typed numbers): `centred` (Center Box straight on the XY plane, the "half below the floor" step), `mistake` (Circle wired to the XY plane), `place` (Plane Params A and B, Rotate about the core axis, Orient) and `wrap` (List Item, Wrap toggle, Orient onto eight planes).
- Model addition (my topic only): a `wu` Number Slider "Window offset" (domain ±0.8 m, bound to `types.A.window_u`) wired into `Construct Point` X of the window plane, and `wu` added to the `window-circle` view. Added in `T/gh-engine/build_model.py` (capsule canvas), JSON regenerated with it; Python (57 tests) and AJV tests pass.

## Clause list

| # | key | what changes on screen |
| --- | --- | --- |
| 1 | hook | tower with one lit capsule, Panel "Capsules 140 · Types 4 · Definitions 1" (counts from the model) |
| 2 | parametric | slider → Center Box → capsule; the width slider moves 2.50 → 3.10 → 2.50 m and the capsule and its dimension follow |
| 3 | sliders | six Number Sliders with domains and colour bars, a live parameter card (values with unit) and the capsule with all six dimension lines in the viewport |
| 4 | plane | XY Plane component, Panel with the origin, red/green/blue axes in the viewport |
| 5 | box | Center Box from the plane and three sliders; box centred on the origin, half below the floor; live dimension lines |
| 6 | lift | helper components (A/B, Pt, Pl Origin); plane lifted by half length and half height, the box stands with its bottom face on the base plane, Panel "Box centre [m]" |
| 7 | live | the length slider is dragged 4.00 → 5.20 m and the keyframe shows the drag in progress: slider value, 5.20 m dimension, box and centre {0.00, 2.60, 1.25} follow (the slider returns to 4.00 m during the next clause) |
| 8 | rounded | Fillet Edge, Volume panel (24.54 m³), radius tag, edges round 0 → 0.25 m |
| 9 | wrongplane | Circle wired to XY Plane, remark bubble, red circle on the floor |
| 10 | frontplane | plane on the front face (Construct Point, XZ Plane, Pl Origin), orange ring on the front face, offset ramps 0 → type D value → 0 |
| 11 | cut | Cylinder + Solid Difference, window cut, Ø dimension |
| 12 | bolts | bolt sliders, Neg, two Merge, Construct Point, point Panel, capsule turns to the back, four orange bolt rings |
| 13 | domains | slider domains card (min – max per parameter, marked as teaching assumption) and the dashed transport envelope ("teaching assumption" / "Lehrannahme") |
| 14 | limit | width slider to its upper domain end, envelope turns orange, badge "limit reached" |
| 15 | types | type table A–D (w, l, h, r, Ø, u, α) from the model |
| 16 | four | four mini capsules, sliders and the big capsule run through types A to D |
| 17 | planes | core of tower B, base plane A (world origin) and target plane B with axes and numbers |
| 18 | orient | Orient with Capsule, A, B; dashed arrow, capsule glides to B |
| 19 | rotate | Rotate + Radians + Angle slider (in degrees) about the core axis; rotated plane in a live Panel; angle arc with its label on the core; the capsule turns with plane B |
| 20 | listitem | Types, index Panel 0–3, List Item, Orient onto four planes, result Panel Brep A–D, capsules coloured by type |
| 21 | outside | eight planes, indices 0–7, List Item turns orange with the engine message, planes 4–7 get "!" and `null` |
| 22 | wrap | Wrap toggle True: the index cycles, all eight planes get a capsule; a strip of eight numbered capsules below the core makes them countable |
| 23 | recap | legend of eight terms (en/de) with an icon each, next to the final core and the numbered capsule strip |

## Keyframe review notes

Captured with `capture.py` in both languages, every PNG looked at. Fixed during review: sliders clipped by the sheet edge (slider minimum width is about 2.9 m), white text on the light sheet (all sheet text now dark), the orange block from recolouring the dimension chip (window dimension is now drawn by the scene), Orient with the source plane not equal to the plane the capsule was built on (A is now the world origin, as in Rhino), capsules clipped by the viewport (core and scale moved), table and card overlapping the slider column, decimal comma in German coordinates (separator `;`), panels truncated.

Layout report: 46 (en) and 44 (de) strict findings in 18 of 23 frames in both languages (en and de) after the audit round (47 before; the new panels, arc, icons and strip add callouts), down from 234 on the first capture. The remaining findings are justified:

- wires drawn behind the centre cell of components they pass (Grasshopper draws wires behind components too): `A/B`, `Pl Origin`, `Cylinder`, `Orient`, `Neg`, `Vol`;
- callout chips that sit on the geometry they name (`r 0.25 m`, `Ø 1.30 m`, `u −0.45 m`, plane index tags);
- Base A / Target B parameter hexagons whose output grip touches the first wire.

German: slider names and component names stay English (the Grasshopper UI); captions, tags, panel prefixes, table headers and the legend are German; decimals use commas, coordinate lists `;`. The List Item warning text is the engine's own German message.

## Test result

- Series presentation tests `🧪️tests/🎭️presentation/🐍️.py`: 12 tests OK (all five Grasshopper scenes, native lifecycle en + de, 23 clauses within 14–24).
- Grasshopper model tests after the `wu` addition: Python 57 tests OK, AJV schema/canvas/tree/casting OK.

## Files

Scripts and helpers: `T/capsule/` (probe, measure, run scripts), outputs in `T/🗑️generated/capsule/` and `T/🗑️generated/frames/🦗️grasshopper/Beat2_Capsule/{en,de}/`.

## Audit round (audit-grasshopper-capsule.md)

Fixed: `lift` wording (bottom face, not back face, en and de); transport limit now a teaching assumption (narration, card title, envelope label, "Lehrannahme"); `live` shows the drag in progress; `sliders` viewport filled (capsule with six coloured dimension lines, live parameter card) and the Window Ø bar inside the sheet (slider column moved right); live Panel moved off Rotate; `wrap` capsules countable through the numbered strip; every coordinate panel carries an object name and unit (`Origin [m]`, `Box centre [m]`, `Volume`, `Cylinder length`, `Bolt points [m]` with indices 0 to 3, `Base plane A [m]`, `Target plane B [m]`, `Plane B rotated [m]`, `Result`); the 0.30 m cut depth is labelled `Cylinder length`; row A highlighted in the type table; angle arc with degree label in `rotate`, the slider shows degrees; legend icons, no colour reuse; front-plane axes longer with labels; `!` markers moved off the index tags; table moved inside the sheet; helper components in `lift`/`frontplane` re-laid out so the length and height wires run straight.

Not done (minor, justified): quarter-circle radius dimension (the radius tag and the live radius in the card carry the value), window-area panel (dropped from the storyboard: the Ø dimension and the Cylinder length carry the window), the Orient wire still passes behind the Target B panel.

G engine view test `test_every_engine_view_builds` (canvas capsule, view types): green after the model JSON was regenerated from `build_model.py`; the `types` view was not changed by my slider.

Final state: en and de re-captured and every changed frame reviewed (sliders, lift, live, frontplane, cut, bolts, domains, types, planes, rotate, wrap, recap). The last 0.3 m shift of the Cylinder length panel (clear of the Offset slider) was applied after the capture and is not re-captured.

## Quality round

Implemented from quality-grasshopper-capsule.md: `hook` fills the canvas (one-definition outline with slider and Center Box, counters, callout on the lit capsule, larger tower); `parametric` shows only a plain box that grows in width (rounding and window appear in `sliders`); `four` plays types A, B, D, C with a live "Same definition · type X" tag and a moving table-row highlight, the table is paired with the four mini capsules from `types` on; `lift` and `frontplane` re-laid out; `bolts` shows a core wall plate with rods and indexed bolt tags; `orient` shows the capsule mid-glide with a live translation panel; `rotate` steps 0, -90, -180, -270 degrees with arc, dashed core axis and the capsule ghosted where it is hidden by the core; `wrap` reveals capsules 4 to 7 one by one and draws hidden ones over the core; German domains wording smoothed. Not done: red-slider-out-of-domain beat in `limit`, recap pointing, wire pulse colours.

Last round: `limit` now pushes the width past its domain (request 4.00 m, slider clamps at 3.50 m, red box on the slider, red envelope, caption explains the domain as a design rule); `recap` pulses each legend row and rings the matching part of the picture (capsule, plane, strip, index 4); `lift` and `frontplane` pulse the wires in the colour of the value they carry (length green, height blue, offset teal, then the flow colour). The pulses and rings are motion and do not show in the end-of-clause keyframes.

Sweep round (de labels): slider nicknames, parameter nicknames and the viewport tab are now localized with `self.tr` (Breite, Länge, Höhe, Radius, Fenster Ø, Versatz, Bolzen x/unten/oben, Winkel, Basis A, Ziel B, Kapsel, Kernachse, Typen, Ebenen, Perspektive). Full en and de captures were reviewed after the change. The `bolts` layout was then re-spaced for the longer German slider names (bolt sliders moved to x −5.05, Negative/Merge/Construct Point shifted right); that final layout was first blocked because the shared BIM drafting `P` (`📏️geometry.py`, edited 05:43 today by someone else) now returns a tuple and breaks the Grasshopper slider widget (`factor * P((t0, 0))`). Re-capture `bolts` en+de once `P` returns an array again.
After the P() fix, bolts en+de were re-captured and checked: clean.
