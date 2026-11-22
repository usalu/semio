# appliances/decentralised-hrv-fan — rework report

Earlier batch report `rework/17.md`: does not mention this asset (not started there).

## Pass 1 (2026-11-22)

### Reference table (written before any change)

Subject: decentralised push-pull (pendulum) ventilation unit of the wall-pipe class with a ceramic regenerator, shown with inner cover and filter taken off (LUNOS e2, Blauberg Vento Expert A50, Ambientika, Helios EcoVent Verso).

| Part | Real unit | Model before pass 1 (1:50) | Verdict |
|---|---|---|---|
| Wall sleeve, outer diameter | 160 mm (LUNOS, Blauberg) to 200 mm (Ambientika) pipe | 196 mm, wall 3 mm | right (200 class) |
| Wall crossing | 240 to 500 mm, shortened to the wall | 220 mm behind a 20 mm tray, 240 mm overall | right (minimum wall) |
| Inner base / cover | 180 to 260 mm square, 20 to 50 mm deep, rounded corners | 260 mm square, 20 mm tray, corner radius 20 mm, rim 2.5 mm | right |
| Fixing | four corner screws 4 to 5 mm in 12 to 14 mm bosses | four 14 mm bosses with screw heads at 108 mm from the axis | right |
| Ceramic store | diameter 150 to 170 mm, length 100 to 150 mm, hexagonal cells of about 3 mm | 160 mm by 100 mm, 37 cells of 18 mm (generalised) | right for a 7.5 mm feature |
| Insulating jacket | EPP foam 6 to 10 mm round the ceramic, funnel to the fan | 176 mm throat, 45 degree funnel to 160 mm, 7 mm lip | right |
| Fan | reversible axial impeller, 5 to 7 blades, symmetric section, tip clearance 1 to 3 mm, hub 50 to 60 mm | five radial lens-section blades, chord 34 to 56 mm, pitch 36 to 24 degrees, tip 172 mm in a 176 mm throat, can 56 mm | right |
| Motor mount | spider of 3 or 4 struts behind the impeller carrying stator and bearing | three struts 8 by 12 mm, 40 mm stator boss, 8 mm shaft, bearing | right |
| Mass | 2.5 to 3.5 kg, ceramic about 1.5 kg | 2.65 kg: tray 0.25, sleeve and housing 0.7, core 1.6, rotor 0.1 | right |
| Operation | 70 s extract, reversal, 70 s supply, 5 to 60 m3/h | poses off / extract / supply, clip of three double cycles, rotor revolute about the axis | right |

### Defects found (current catalog state judged from sheets of all four scales)

1. 1:50 body, rig joints, thermal and structure: carefully modelled already; all six views, the three poses and the clip read as the real unit. No rebuild needed there.
2. 1:100, 1:200 and 1:500 body lists were the 1:50 list with fewer polygon points and a few entries deleted: 8 mm struts, 8 mm shaft, bearing, 37 cells of 18 mm with 3.5 mm walls and a 2.5 mm tray rim down to 1:500 (feature 75 mm); the tray corner radius became one 45 degree chamfer at 1:500 (an octagonal plate). Thermal and structure lists followed these by reference.
3. 2D drawings at 1:100 and 1:200 were copies of the 1:50 shapes (8 mm struts, 8 mm shaft section) while their own `decisions` promised 15 mm struts, 19 cells, one 30 mm bridge bar: text and drawing disagreed. The validator rule `volumes-feature` accepted the thin 3D solids only because the thinnest drawing of the bone was just as thin.
4. Volume `decisions` at 1:100 to 1:500 were two generic sentences that claimed "five pitched blades and the physical transitions" without saying what is modelled.
5. Rig: the three fixed bones (`plate`, `ring`, `core`) had their diameter as `lengthM` and stuck 130 mm out of the unit beside it; the rotor bone was the tip radius.

### Rebuilt in pass 1

Edit scripts (hand-chosen numbers): `rework-hrv-decentralised-hrv-fan-coarse.py` (volumes), `rework-hrv-decentralised-hrv-fan-2d.py` (2D shapes, 2D decisions, rig lengths).

- **1:100 (15 mm), 13 body / 13 thermal / 18 structure solids.** Tray floor and rim with the 60 mm terminal box (no bosses, no screws); housing with one chord for the inlet radius; sleeve, 208 mm outer collar and EPP jacket as separate parts; ceramic with 19 hexagonal cells of 25 mm at 31 mm pitch (walls 6 mm); three struts 15 by 15 mm; 40 mm stator boss; 15 mm shaft; can without cap; five blades as one loft of five stations repeated five times (exact five-fold symmetry). Thermal by reference, stator boss and can filled, 3 mm outlet patches. Structure: all members including the jacket as seat of the core (was missing), three joints (tray bore, core seat, bearing as own solid), four masses, two wall screws.
- **1:200 (30 mm), 8 / 9 / 11.** Tray as one 20 mm slab with three chords per corner; housing and sleeve one turned tube; black EPP liner flush to the outer end (the dark end ring of the drawing); ceramic as a plain disc; one bridge bar 30 by 30 mm with the stator boss in it; plain can; five blades of four stations with rounded tip, tip chord reduced to 50 mm so that no corner touches the coarser tessellated throat.
- **1:500 (75 mm), 7 / 9 / 10.** Sharp square slab, one thick tube 196 / 160 mm, ceramic plug, 60 mm can with conical nose, three broad blades (chord 34 to 60 mm, ending 12 mm inside the bore), one 40 mm bar and a 60 mm boss as load path; supports drawn 40 mm thick and touching the slab.
- **2D drawings.** 1:100: struts redrawn 15 mm wide, the cell path redrawn as the 19 cells of the model, shaft section 15 mm, outer collar 15 mm long as in the model. 1:200: the three struts replaced by one 30 mm bridge bar, tray floor and rim merged into one slab rectangle, the 8 mm shaft section replaced by the side view of the can; the `retained` / `merged` / `silhouette` sentences corrected (en, de). With the honest drawings `volumes-feature` then demanded real 15 / 30 mm solids, which the lists above now are.
- **Rig.** `lengthM` of `plate`, `ring`, `core` set to their radii (0.13, 0.098, 0.08) so every bone ends at the edge of its part like the rotor bone.
- **Decisions.** New en/de texts for 1:100, 1:200 and 1:500 saying what is modelled and why.

### Validation

- `r3-validate.ts appliances/decentralised-hrv-fan --volumes`: `VALID appliances/decentralised-hrv-fan (catalog asset + volumes source): schema 0, validateAsset 0, checkVolumes 0` (after the last change).
- Published with `volumes-insert.py`; `r3-validate.ts appliances/decentralised-hrv-fan --catalog-only`: `VALID ... (catalog file as it is): schema 0, validateAsset 0`.
- Rendered and read after the changes: 1:100 body, structure, thermal cut (clip t = 100 s); 1:200 body, thermal cut, structure (pose supply); 1:500 body (pose supply, clip t = 30 s, close-up elevation), structure, thermal cut; 1:50 body (clip t = 20 s close-up, t = 35 s six views), structure, thermal cut; 2D sheet 1:200 light-line and forces. Rendered after the last change but not read one by one: 1:50 pose off, 1:50 structure supply, 1:50 thermal clip, 1:100 / 1:500 pose off and structure supply, 1:200 / 1:500 thermal clip, 1:200 structure cut.

### Open list (what remains imperfect)

- 2D, 1:100 and 1:200, elevation / front / plan: the redrawn shapes (15 mm struts, 19-cell path, 15 mm shaft section, bridge bar, slab, can section) are valid and their numbers are those of the model, but they were judged only on the product sheet, where the whole unit is 1.3 to 2.6 mm of paper; an enlarged reading of these panels is still to be done (a zoom helper with sharp at 2400 dpi was too slow on the shared host and was removed).
- 2D, 1:200, front and plan: the can side view still carries the id `shaft-section`; rename to `hub-section` in one go with any reference.
- 2D, 1:50: not touched (model unchanged); the front and plan show only tray, sleeve and collar, which agrees with the model seen from outside.
- 1:50 body: no outer weather hood and no inner cover or filter (the asset is defined as the opened unit); a closed-cover pose would need a new bone and is not part of this pass.
- Forces view, all scales: the free-body diagram reports a residual `ΣMx 2.57 N·m` (26 N weight on the axis, 0.1 m in front of the two wall screws). The pin and roller of the asset cannot balance this overturning moment in the product's solver; either the roller must be modelled as a normal contact on the wall face with a lever arm to the pin, or the sleeve needs a bedding support in the wall. Not solved in this pass (see Fundamental issues).
- Poses and clip: all read correctly at the instants looked at; the six sheets listed above as "not read one by one" should be read by the next pass.

### Fundamental issues (outside this asset)

- `volumes-feature` takes the smaller of the scale feature and "the thinnest drawing of the bone": a 2D shape copied from 1:50 therefore silently licenses 3D solids far below the feature of a coarse scale (here 8 mm struts at 1:200 passed until the drawing was corrected). The rule should compare against the scale feature and name the drawing as a second finding.
- Forces analysis: a support pair whose reactions leave a moment residual of 2.57 N·m is reported only as small print in the free-body diagram; `validateAsset` has no finding for an unbalanced free body.
- `r3-2d.ts`: each panel is fitted to its frame including title and scale bar, so an asset of 0.26 m is about 20 px wide; a `--zoom figure` option that fits the drawn figure alone is needed to judge small appliances at 1:100 and coarser.
- The catalog file is written in two formats (`json.dumps` indent 2 for everything, the compact rows of `volumes-insert.py` for `volumes`); a root edit of another member has to be followed by `volumes-insert.py` to restore the volume rows. One writer for the whole document would remove this trap.
