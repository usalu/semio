# 🧤️ Silhouette Batch R2-HA: 7 Humans, Second Round

Fixer report of 2026-11-23/24 for `🧰️silhouette/R2-HA.tsv` (brief `📋️silhouette-brief.md`): the seven people batch SH-2 left red. Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `U` = this ticket folder, `K` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE`, `W` = `U/🧰️silhouette/R2-HA`. The ops of batch SH-2 stay in force and run before the ops of this batch. No test, rule, threshold, engine, schema, fixture, README or manifest file was edited.

## Result

- All seven pass the real `🧊️catalog-silhouette` test: final run with the seven in the name filter `exit 0 … Tests 7 passed | 378 skipped (385)` (before: 7 failed). Failing checks (bounding-box axis, IoU, largest boundary distance per scale and view) measured at the start of this round: 26; after: 0.
- Other suites, final run with the seven in the name filter, each to its end: `🧊️three-views` `Tests 14 passed | 761 skipped (775)`; `🌡️catalog-physics` `Tests 21 passed | 819 skipped (840)`; `🫁️breathing-catalog` `Tests 21 passed | 269 skipped (290)`; `✅️asset-validation` `Tests 9 passed | 74 skipped (83)`; `⚡️asset-power` `Tests 7 passed | 17 skipped (24)`.
- `validateAsset`: 0 issues for each of the seven (0 before). Three-views sweep (`sweep` and `consistency` of `🧪️tests/🧊️three-views/🧰️.ts`): 0 findings for each (0 before). Product check CLI: `✓` for each, run on the document that is byte-for-value identical to the catalog file (compared as JSON).
- Margins are thin in places, stated under "What is close".

## Method

- Measured anew with `measure.ts` (the test's own `measureAsset`), located with `clusters.ts` and overlay pictures (`png.ts`), named with `near.ts`, `kinds.py`, `show.py`.
- Every repair is derived from the volume and written as idempotent ops (`ops.ts` format) in `W/ops/<slug>.json`; hand-written ops are recorded in `W/hand/<slug>.json` (`W/make/person-with-rollator.py` writes the rollator's). The work was tried on a copy built from the state before this round (`try.sh`, `try2.sh`: all silhouette checks, `validateAsset`, sweep, check CLI) and only then landed (`land.sh`; `REBUILD=1` rebuilds from the state before, so ops taken back leave no trace).
- New tools, all reading the volume:
  - `solidfit.ts`: refits drawn shapes that are solids. A mirrored body of revolution gets its axis through the middle of its solids (`axisX`, `axisZ`) and, per view, the half width the view sees; `parts` splits it into lengths with their own axes for a limb whose middle line leans against its bone. An extrusion gets the exact outline of its solids seen along its sweep, as a full prism, a slab (`z=thin`, `slab`), a stack of slabs (`layers`) or cut into pieces (`pieces`); it can add a shape that does not exist yet (`like`).
  - `outline.ts` (from batch R2-O) extended: `solid=keep` and `solid=half` write the exact outline as the half profile of a body of revolution without mirror; `shift` and `clip` move a detail stroke with its body and cut it back to the outline; the union retries on a coarser grid when the clipping library fails.
  - `plug.ts`: a pad of flesh where two parts touch in life and the solids leave a slit narrower than the smallest feature (a capsule of 7.6 mm, with its thermal entry).
- Lessons that decided the choices: a flat trace on an arm that turns out of its plane is refused (`lateral-ambiguous`, sweep findings), so limbs stay solids; a half profile without mirror is exact in the default pose but failed the sweep on the selfie arm (20 findings) and is used only where the sweep stays at 0; a single thin slab of a hand differs between views in other poses (2 sweep findings on the cane hand at 7.5 mm); stacks of slabs in small pieces fail the print profile at 1:100 and 1:200 (`suitability-gap`, `suitability-feature`) and were replaced there by one slab.

Notation: per view IoU / largest boundary distance; limits IoU 0.92 and 15, 30, 60, 150 mm at 1:50, 1:100, 1:200, 1:500.

## Per person

### `person-taking-selfie`

- Before: 1:50 elevation 17.0 mm, front 16.0 mm, plan 16.8 mm (3 checks).
- After: 1:50 elevation 0.993/10.1, front 0.988/12.6, plan 0.965/10.6. Real test `Tests 1 passed`; validate 0; sweep 0; check `✓`.
- Changed (37 ops, 1:50 drawing only): both sleeves as bodies of revolution about the axis of their solid (3 lengths in the side elevation, 5 in front and plan; the sleeve's middle line leans up to 17 mm against the bone); both forearms refitted; the hoodie of the front and the daypack redrawn from their solids; the pocket seam of the daypack, drawn as a 12 mm bump the body does not have, is drawn inside the pack outline. The ops are also merged into the chain's repair file `K/🧰️scripts/HW-11/repair/person-taking-selfie.json` (applying the merged file to the catalog file leaves it unchanged).

### `person-with-guide-dog`

- Before: 1:50 elevation 20.2 mm, front 21.2 mm, plan 18.8 mm (3 checks).
- After: 1:50 elevation 0.994/8.0, front 0.992/13.1, plan 0.991/11.9. Real test `Tests 1 passed`; validate 0; sweep 0; check `✓`.
- Changed (8 ops): both sleeves of the front as exact half profiles; the left hand of the side elevation drawn from its solids (hand and fingers around the grip); the harness grip of the side elevation drawn as the full cylinder again (batch SH-2 had thinned its middle under the old mitten hand). Volume repaired at 1:50: two pads where the fingers close on the grip and on each other (`hand-l-grip-pad`, `hand-l-web-2`, 9 and 7.6 mm, with thermal entries); the slits they close were 2 to 6 mm wide, below the smallest feature. Merged into `K/🧰️scripts/HW-15/repair/person-with-guide-dog.json` (same check).

### `person-with-rollator`

- Before: 1:50 elevation 17.8 mm, front 26.8 mm, plan 39.4 mm; 1:100 plan 45.0 mm (4 checks).
- After: 1:50 0.994/7.7, 0.991/11.4, 0.994/8.4; 1:100 0.988/11.3, 0.984/22.5, 0.987/13.5. Real test `Tests 1 passed`; validate 0; sweep 0; check `✓`.
- Changed (67 ops): sleeves and forearms as exact half profiles in side and front, refitted bodies of revolution in the plan; cuffs and wrists refitted; hands drawn from hand and finger solids. 1:50: the coat shoulders (not drawn in the front), the seat and the handle posts at their true width, the post knobs (not drawn at all) and the basket of the plan from their solids. 1:100 front: the moulded wheel discs are drawn by their outer ring (the disc with spoke openings swept sideways opened slots across the wheel), coat shoulders drawn. Volume repaired at 1:50: one pad per hand where the little finger closes on the grip (hairline slit).

### `person-with-umbrella`

- Before: 1:50 elevation 30.1 mm, front 31.4 mm; 1:100 front 33.5 mm (3 checks).
- After: 1:50 0.990/12.6, 0.987/14.6, 0.999/8.0; 1:100 0.987/11.9, 0.979/28.6, 1.000/3.8. Real test `Tests 1 passed`; validate 0; sweep 0; check `✓`.
- Changed (124 ops): the ribs, rib tips and stretchers under the canopy, which the body has and the side and front drawings did not, are drawn as prisms of their true section (ribs in two pieces, see requests.md); the shanks of the 1:50 side elevation reach the shoes (a 6 mm slit). Volume at 1:50: `rib-front` and `rib-back` carried `texture: true` (left out of the comparison) unlike the other six ribs; they are body like the others (`texture: false`) and are drawn.

### `person-with-white-cane`

- Before: 1:50 elevation 41.9 mm, front 23.5 mm, plan 60.0 mm; 1:100 front 78.8 mm, plan 58.2 mm (5 checks).
- After: 1:50 0.990/9.4, 0.996/9.4, 0.979/14.6; 1:100 0.990/8.4, 0.995/11.3, 0.976/15.9. Real test `Tests 1 passed`; validate 0; sweep 0; check `✓`.
- Changed (53 ops, drawing only): sleeves, forearms with cuffs and wrists as exact half profiles in side and front and as fitted bodies of revolution in the plan; both hands drawn from hand and finger solids (the cane hand of the plan as a slab of 48 mm at 1:50 and 60 mm at 1:100, the thinnest that keeps the sweep at 0); the wrist loop of the cane, which no view drew, is drawn at 1:50; 1:100 front: hips as a slab across the view (batch SH-2's prism along the bone was a box), the bag with its flap from its solids.

### `pregnant-person`

- Before: 1:50 elevation 33.2 mm (1 check).
- After: 1:50 0.993/6.8, 0.988/13.3, 0.974/13.7. Real test `Tests 1 passed`; validate 0; sweep 0; check `✓`.
- Changed (19 ops): side elevation of 1:50: the left hand on the belly as a stack of six slabs of its solids (the hand points at the viewer), the right hand as a slab, right sleeve and forearm in three lengths, the collar 4 mm higher under the hair (a 2 mm notch between hair and collar that the body does not have). Volume repaired at 1:50: two pads between the spread fingers of the left hand (`hand-l-web-ring`, `hand-l-web-middle`, with thermal entries; slits of 3 to 6 mm).

### `runner-with-blade-prosthesis`

- Before: 1:50 widthM 0.8738/0.864, heightM 1.7828/1.773, elevation 26.1 mm, front 95.8 mm, plan 38.0 mm; 1:100 front 112.4 mm; 1:200 plan IoU 0.9185 (7 checks).
- After: 1:50 0.990/12.6, 0.994/12.0, 0.971/11.9; 1:100 0.935/27.0, 0.995/18.8, 0.968/17.1; 1:200 0.931/28.7, 0.939/27.9, 0.977/10.4; 1:500 unchanged 0.927/34.9, 0.943/27.9, 0.925/28.1. Real test `Tests 1 passed`; validate 0; sweep 0; check `✓`.
- Changed (191 ops): declared width 0.874 m and height 1.783 m (the body of 1:50). 1:50 side elevation: every flat part retraced from its solids; the face details (brow, mouth, nostril, jaw line), drawn for a head 13 mm further forward than the body's, moved with the head and cut to its outline; hem line cut to the body. Front of 1:50 and 1:100: limbs as exact half profiles, torso, shorts, neck, head, hair, shoes and hands as slabs of their solids. Plans of 1:50, 1:100, 1:200: limbs refitted, torso, shorts, neck, head and hair from their solids, hands and shoes from their solids (the right hand of the 1:50 plan as 16 slabs). The separate finger shapes of the left hand are gone (the hand outline holds the fingers). Volume at 1:50: one pad where the left forearm lies against the singlet (the tip of a slit of 2 to 4 mm).

## Declared dimensions (`W/manifest-changes.json`)

`runner-with-blade-prosthesis`: `widthM` 0.874, `heightM` 1.783 (the asset file carries them). No power changed. The manifest cases of `✅️asset-validation` passed in the final run.

## Sheets

Rendered with `look.sh` (the product's own drawings: side, front, plan; line and solid) and opened: 1:50 after of all seven; 1:50 before of `person-taking-selfie`, `person-with-umbrella`, `runner-with-blade-prosthesis`; 1:100 after of `runner-with-blade-prosthesis` and `person-with-white-cane`. Rendered and not opened: 1:100 of `person-with-rollator` and `person-with-umbrella`, 1:200 of the runner. Not rendered: the 1:500 sheets (no op touches 1:500; those representations are unchanged) and the before-sheets of the other four.

What the sheets show: the line views are unchanged to the eye. In the solid views the first version of the runner showed bands across limbs and torso (lengths and slabs each draw their outline); that was reworked (half profiles and single slabs) and the final sheet is clean except for the right hand of the 1:50 plan, which reads as a dark striped shape of about 2 mm on paper. The sleeves of `person-taking-selfie` show two or three faint joints in the solid view. Nothing else looked worse or wrong.

## What is close

- `person-with-white-cane` 1:50: plan 14.6 mm of 15; the cane hand slab is the compromise between that and the sweep (`cane-parked`: 50.8 mm of 51 mm tolerance between side and plan).
- `person-with-umbrella`: 1:50 front 14.6 mm of 15 (hair against head, not worked on); 1:100 front 28.6 mm of 30 (width the guard gives the diagonal ribs, requests.md).
- `pregnant-person` 1:100 front 30.0 mm of 30 (untouched, as before).
- `runner-with-blade-prosthesis`: side elevations of 1:100 to 1:500 keep IoU 0.927 to 0.935 (limit 0.92; not worked on, they passed before).

## What stays red

Nothing of this batch in `🧊️catalog-silhouette`.

Stated plainly, where this round did less than it could:

- Scales that passed before were left alone even where the drawing has the same faults as the repaired scale (sleeves of 1:100 and coarser, hands).
- `pregnant-person` keeps three lengths for the right sleeve and forearm in the side elevation: the exact half profile measured 17.8 mm there for a reason not found, so it was taken back.
- Slabs and stacks of slabs for hands are exact in the default pose of their view only; in other poses the views of one hand are different approximations (the sweep tolerance holds them).
- The two people of the generator chains (`person-taking-selfie`, `person-with-guide-dog`) have the ops in both places: `W/ops/<slug>.json` and merged into the chain's repair file. The other five have no generator that still runs: after a rebuild from the state before, `W/land.sh <slug>` has to run after batch SH-2's ops.

## Files

Paths relative to the repository root; `C` = `S/🗂️catalog/👤️humans`.

Updated:

- `C/<slug>/🔣️.json` for the seven: `person-taking-selfie`, `person-with-guide-dog` (rewritten in their own indentation), `person-with-rollator`, `person-with-umbrella`, `person-with-white-cane`, `pregnant-person`, `runner-with-blade-prosthesis` (only the changed text spans).
- `K/🧰️scripts/HW-11/repair/person-taking-selfie.json`, `K/🧰️scripts/HW-15/repair/person-with-guide-dog.json` (this round's ops merged in).
- `U/requests.md`: one section `## silhouette R2-HA — …`.

Created:

- `U/silhouette-R2-HA.md` (this report).
- `U/🧰️silhouette/R2-HA/`: `env.sh`, `people.ts`, `ops.ts`, `apply.ts`, `apply.py`, `sh2_doc.py`, `addop.ts`, `dropop.ts`, `measure.ts`, `one.ts`, `clusters.ts`, `near.ts`, `png.ts`, `rings.ts`, `solids.ts`, `strokes.ts`, `tops.ts`, `posetops.ts`, `validate.ts`, `sweep.ts`, `outline.ts`, `solidfit.ts`, `plug.ts`, `kinds.py`, `show.py`, `t.sh`, `gate.sh`, `cyc.sh`, `sf.sh`, `half.sh`, `pg.sh`, `try.sh`, `try2.sh`, `land.sh`, `look.sh`, `vt.sh`, `manifest-changes.json`, `ops/<slug>.json` (7), `hand/<slug>.json` (7), `make/person-with-rollator.py`.

Removed:

- `U/🗑️generated/silhouette/R2-HA` (backups, work copies, logs, overlays, sheets).
