# 🩹️ Five Small Red Spots of the Staffage Suites

Fixer report of 2026-11-23. Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `U` = this ticket, `T22` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22`. Every number below is from a run of this session that ended; the suites ran one after the other with `U/older/suite.sh`.

## Final suite lines

| Suite | Before | After |
|---|---|---|
| `🧬️schema-conformance` | 42 of 43 | **43 passed (43)**, exit 0 |
| `🌡️catalog-physics` | 838 of 840 | **840 passed (840)**, exit 0 |
| `🧍️plan-projection` | 12 of 13 | **13 passed (13)**, exit 0 |
| `🔭️multiview-coverage` | 21 of 23 | **23 passed (23)**, exit 0 |
| `🏗️structure-projection` (new reproducing case) | 53 of 53 | **54 passed (54)**, exit 0 (the new case was red before the fix: `-11205.98` against `11222.03`) |
| `🦴️lateral-rest` | | 29 passed (29) |
| `🎨️seven-modes`, `🎨️view-synchrony`, `🎨️structure-legend`, `🥇️force-first`, `📑️multiview-sheet` (after point 4) | | 59 passed (59), 5 files |
| `🧍️front-projection`, `🧭️representation-resolution`, `🧊️depth-order`, `🪞️representation-copied`, `✂️section-views`, `📑️multiview-sheet`, `🌐️thermal-multiview`, `🎨️view-synchrony`, `🎨️seven-modes`, `📐️dimension-clearance`, `🔤️caption-fit`, `🧊️lateral-spread`, `🧊️axial-twist` (after point 3) | | 192 passed (192), 13 files |
| three-views sweep `walking-commuter` (`🧰️three-views/sweep-2.ts`, catalog file) | sweep 8 | **sweep 0, consistency 0** (26 instants, four scales) |

Not run: `🧊️three-views` and `🧊️catalog-silhouette` as vitest suites over the whole catalog (hours), the site build.

## 1. `🧬️schema-conformance`: provenance notes of two wave people

- Cause: `HW-06/knit_texts.py` and `HW-06/scoot_texts.py` wrote `provenance.notes` as a list of Texts; the schema (`Provenance.notes`) holds one Text `{en, de}`.
- Fix: `HW-06/hw06lib.py` gets `one_text(texts)` (joins the Texts with a space per language), both generators wrap their list in it; the two catalog files had only the span of `notes` rewritten to the same joined Text (`🧰️small/notes-one-text.py`, keeps every other byte, refuses when the file changed on disk meanwhile). Generator and catalog agree by construction (same join); the generators were not re-run (their kept output `🗑️generated/REP-A/base` no longer exists and the repair ops would have to be re-applied by the wave's chain).
- Proof: suite 43 of 43. Check CLI exit 0 for both; validator schema 0 for both.
- Open, not caused here (the notes are text only): the validator ends `INVALID` for both on print findings of their drawings, `mobility-scooter-user` `validateAsset 5` (`PRINT range-end-feature` of `seat-base` 0.192 mm at 1:350, of `cap-cuff` 0.190 mm at 1:75 and three more), `elderly-person-knitting` `validateAsset 2` (`range-end-feature` of `work` 0.194 mm and `hand-r` 0.200 mm at 1:1000). These are the wave people another agent is editing now.

## 2. `🌡️catalog-physics`

### runner-with-blade-prosthesis: core 39.31 °C → 37.80 °C

- Cause, two physically wrong values of the default (running) state:
  1. `activityMet` 11.5 was the ACSM MET count (3.5 ml O2 per kg and minute, a quantity per body mass) used as the solver's met (58.15 W per m² of skin). The asset's own derivation gives 40 ml/(kg min) × 68.5 kg = 2.74 L/min; with the energy equivalent of oxygen of ISO 8996 (5.81 Wh/L at RQ 0.95) that is 955 W, not 11.5 × 58.15 × 1.776 = 1188 W. In the solver's unit: 955 / 103.3 = **9.25 met**.
  2. `environment.windMS` 1.5 for a figure that runs at 3.1 m/s in place (treadmill frame): the air passes the skin at least at the running speed. **3.1 m/s** (calm air), as the cyclist states its riding speed as wind.
- Measured: 9.25 met alone 38.2 °C, wind alone 38.9 °C, both 37.80 °C (standing-ready 36.80, hands-on-hips 37.25, flight 37.80).
- Fix (`🧰️small/fix-runner-thermal.py`, span edits): 25 poses and the two keys of the `activityMet` track of `run-cycle` 11.5 → 9.25, `windMS` 1.5 → 3.1, title of the breathing pattern `running`, the assumption rewritten with the derivation, a new assumption for the wind, reference `iso-8996-2021` added. The asset has no generator that can be run again; the catalog file is its source.
- Proof: check CLI exit 0; validator `VALID … schema 0, validateAsset 0`.
- **Manifest (not edited): `powerW` of `runner-with-blade-prosthesis` is 1187.7 in the manifest; the asset now gives 955.3 W in its default pose.**
- Seen, not touched: `jogger` has the same two conventions (8 met as MET count, wind 0.2 m/s at 2.6 m/s) and passes at 37.90 °C.

### used-playground-roundabout: deck 84.87 °C → 73.80 °C

- Cause: (1) solar absorptance 0.89 of weathered galvanised steel against the table value 0.80 of the cited source (Duffie and Beckman: clean and new 0.65 / 0.13, oxidised and weathered 0.80 / 0.28; 0.89 was the outlier among sixteen galvanised materials of today's generators). Alone: 80.35 °C. (2) The 5 mm tread plate was modelled with an adiabatic underside; it is isothermal through its thickness (Biot about 0.001) and loses heat below: radiation to the pit tray (two zinc faces of emittance 0.28: 1.2 W/(m² K)), still air 0.14 W/(m² K), hub and spindle 0.5 W/K into the footing, together 1.3 W/(m² K) on the deck area.
- Fix in the generator `T22/USED-PARK-OBJECTS-STAFFAGE-COLLECTION/assets/used-playground-roundabout/author-roundabout.py`: absorptance 0.80, citation and assumption text corrected, the underside path entered as the contact share of the deck (0.07 towards 24 °C in service; 0.12 at 27.8 °C with the children's contact; 0.19 towards the air at 32 °C when lifted out) with a new assumption that states the derivation. The generator reproduced the published document byte for byte before the change; regenerated, published with the ticket's `publish.ts` under its lock: candidate = catalog (cmp), manifest row unchanged (diff of the manifest empty, `powerW` 0).
- Stated plainly: the solver has no radiation between two regions, so the underside path is a surrogate through `contact` (20 W/(m² K) per contact share); the numbers are derived, the mechanism is named in the asset. Other poses: in-use 66.0 °C, maintenance 65.3 °C.
- Proof: check CLI exit 0; validator `VALID … schema 0, validateAsset 0`.

## 3. `walking-commuter`

- Cause: every front and plan limb was a flat outline of the standing figure, carried by its bone as a card (per bone the plan was up to 150 mm short of the side view in the walk; the guards saw only the hand, 87 mm).
- Redraw (`🧰️small/wc_redraw.py`, applied by `🧰️small/fix-walking-commuter.py`, 186 shapes, span edits; rest frames in `🧰️small/wc-rest.json`), all four scales, plan and front:
  - sleeve, forearm, cuff, wrist, both thighs and shanks: bodies of revolution about their bone; the half profile is the half width of the side outline, the axis keeps the lateral middle of the old outline (this keeps the front depth extent, where the earlier try lost 17 mm).
  - hand at 1:200 and 1:500: the side outline swept over the lateral range of the old outline (86 mm at 1:500: no thinning, print minimum 0.202 mm as before). Hand at 1:50 and 1:100, where the 3D model has fingers: the palm up to the knuckles as a prism and one body of revolution per finger and thumb bone (15 shapes with the ids of the model's capsules), **in all three views**: the side elevation drew the fingers straight while the model curls them inwards. This is what the three earlier tries missed (their boxes covered ground the curled hand leaves free: plan Hausdorff 0.037 to 0.044).
  - shoes: the side outline swept over the lateral range of the old outline (plan and front); the toe box in plan is a capsule from the ball to the tip with `followTip`.
- Measured on the catalog file: sweep 0; walk sums of `🔭️multiview-coverage` at 0 / 0.3 / 0.6 / 0.9 s: 0.0004 / 0.0081 / 0.0000 / 0.0000 m (limit 0.072; before 0.0802 at 0.6 s); print minimum 0.284 / 0.294 / 0.225 / 0.202 mm (before 0.346 / 0.294 / 0.225 / 0.202), all suitable; validator `VALID … schema 0, validateAsset 0` (extents and print included) and, with the measured volume rules, `VALID … schema 0, validateAsset 0, checkVolumes 0` (run with `--volumes --no-print`); check CLI exit 0; silhouette (`silhouette-2.ts`): the same four failing pairs as before and nothing new (1:100 plan 0.0128 → 0.0139, front 0.0168 → 0.0187; 1:200 front 0.0141 → 0.0422 against 0.06; 1:500 unchanged within 0.007).
- Pins and oracle of `🧍️plan-projection`, and why:
  1. `🧫️fixtures/🧍️plan-projection/🔣️.json` `catalog.stride.shape`: `shoe-heel-l` → `shoe-toe-l`. The pin named a shape of the drawing before 2026-11-22 (a heel capsule that no longer exists); the rule (a plan capsule of the commuter's foot travels as the kinematics predicts, more than 0.2 m between the two poses) is unchanged. Measured travel 0.3477 m.
  2. `🧪️tests/🧍️plan-projection/🟦️.ts` (and one line of its feature): the reference chain was planar (`mat2d`, the sum of `angleDeg`); the commuter's rig places its pelvis halves laterally since 2026-11-22, so that chain turns the whole leg by 4.33° against the rig and predicts the toe 9 mm off (0.4659 against 0.4750 m, limit 5 mm) whatever is drawn. The chain is now the documented 3D chain in gl-matrix `mat4` (`Rw(angle) · Rv(abduction) · Ru(twist)`, `z`), still independent of the product code; for a planar rig it is the same numbers (the other twelve cases pass unchanged), with it the capsule matches to 0.1 mm. No tolerance or assertion was changed. This is an edit of a shared test beyond a pin; it is stated here for the coordinator's review.
- Sheet opened: 1:50 `check-watch`, old and new side by side (`wc-look.sh`). New plan: the raised sleeve and forearm run ahead from the shoulder to the hand over the chest; old plan: three loose blobs beside the head. Front and side read as before; the front shoes are plain blocks now.
- Stays imperfect: shoes are rectangles in plan (rear) and front; the plan toe capsule reaches 29 mm behind the ball under the rear shoe; the right arm (never moves against the torso in any pose) and the trunk were not redrawn; the people derived from the commuter (`person-with-arm-sling` among them) were not touched.

## 4. Moments read one magnitude per member

- Cause: `drawDiagram` gave each diagram path the signed in-plane peak of its view as `value`; with lateral girdle bones the moment vector is no longer normal to the side view, so each view read another number (`pelvis` −1.2154 side, −0.0012 front).
- Decision: a member's bending moment (and shear force) is the invariant magnitude of its 3D vector, the largest over its stations; every valued path of the member (each lobe and the line-of-sight symbol) carries it, as force arrows already carry the total. The ordinate and the colour stay the in-plane part; where the view does not see the whole (difference above 1 ‰), the label reads `total (⟷ in-plane part)`. Axial force keeps its signed value (it has no line-of-sight part). Legend text extended in both languages.
- Fix: `S/🔨️modules/🎨️depiction/🏗️structural-view/🟦️.ts` (`memberMagnitude`, `DiagramPolygon.magnitude`, `DiagramOutOfPlane.total`, `drawDiagram`). Reproducing test in `S/🧪️tests/🏗️structure-projection/🟦️.ts`: the balcony with a lateral load reads the invariant peak of the slab for moment and shear in every view that draws it.
- Not changed: diagram lobes are still drawn from 1e-9 (the second point of the request); such slivers now carry the member's magnitude.

## Files changed (repo-relative)

- `S/🗂️catalog/👤️humans/mobility-scooter-user/🔣️.json`, `…/elderly-person-knitting/🔣️.json`, `…/runner-with-blade-prosthesis/🔣️.json`, `…/walking-commuter/🔣️.json`, `S/🗂️catalog/🪑️objects/used-playground-roundabout/🔣️.json`
- `S/🔨️modules/🎨️depiction/🏗️structural-view/🟦️.ts`
- `S/🧪️tests/🏗️structure-projection/🟦️.ts`, `S/🧪️tests/🧍️plan-projection/🟦️.ts`, `S/🧪️tests/🧍️plan-projection/🥒️.feature`, `S/🧫️fixtures/🧍️plan-projection/🔣️.json`
- `T22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/HW-06/hw06lib.py`, `knit_texts.py`, `scoot_texts.py`
- `T22/USED-PARK-OBJECTS-STAFFAGE-COLLECTION/assets/used-playground-roundabout/author-roundabout.py`, `🔣️.json`
- `U/small-reds.md`, `U/requests.md` (resolution lines), new under `U/🧰️small/`: `notes-one-text.py`, `thermal-probe.ts`, `fix-runner-thermal.py`, `wc_redraw.py`, `wc-rest.json`, `fix-walking-commuter.py`, `wc-cand.py`, `wc-probe.ts`, `wc-measure.sh`, `wc-diff.ts`, `wc-sil.ts`, `wc-look.sh`
- Removed: `U/🗑️generated/small/` and the `small-*.txt` outputs under `U/🗑️generated/older/`.
