# `furniture/height-adjustable-desk` — running report

Earlier work: `rework/14.md`, section 7 (glides as members at 1:50, head joints at 1:200).

## Pass 1 (second round) — 2026-11-22

State found: source `volumes/furniture/height-adjustable-desk.json` and catalog `asset.volumes` were identical; `VALID` with `--volumes`.

### Reference table (electric two-column sit-stand desk, three-stage columns, with 27-inch monitor and 15.6-inch laptop)

| Part | Real (typical product / standard) | Model 1:50 | Verdict |
|---|---|---|---|
| Top | 1600 x 800 x 25 mm, EN 527-1 type A; seated 720, standing up to 1150 – 1250 | 1600 x 800 x 25, R12 corners, 720 / 1150 | right |
| Stroke | 3-stage: 2 moving stages, 430 – 650 mm total | 2 x 215 = 430 mm, prismatic, both columns | right (short end of the range) |
| Column tubes | rectangular 80 x 50 to 95 x 65 outer, steps of 6 – 8 mm per stage, plastic slide bushings 15 mm | 94 x 64 / 86 x 56 / 78 x 48, collars | right |
| Column spacing | 1100 – 1200 mm for a 1600 top | 1100 mm | right |
| Foot | 680 – 750 x 70 – 80 mm, 25 – 45 high, pressed taper, 2 levelling glides Ø 40 | 700 x 80, 28 → 45, glides Ø 40 | right |
| Motor housing | at column head, about 80 x 100 x 150 mm | 80 x 100 x 150 | right |
| Side rails | 580 – 680 x 50 – 60 x 25 mm under the top | 660 x 60 x 25 | right |
| Crossbar | 50 x 30 telescopic tube between the heads | 50 x 30, 1052 long, 8 mm flanges | right |
| Cable tray | folded 1.5 mm sheet, about 700 x 140 x 110 | 700 x 140 x 110, open top | right |
| Control box / handset | 220 x 85 x 35 / 130 x 80 x 24 at the front edge | same | right |
| Monitor 27" | housing 614 x 368, 45 – 60 deep with rear bulge, VESA 100, base 250 x 200, stand 60 x 25, top edge 430 – 450 above the desk | 614 x 368, 53 deep, VESA 100 with 4 screws, top edge 435 | right |
| Laptop 15.6" closed | 360 x 250 x 20 | 360 x 250 x 20 (base + lid) | right |

### Defects found (rendered and read: 1:50 body six views + underside, standing-height, raise-and-lower t 7.5, structure, thermal cut; 1:100 / 1:500 body before and after)

- 1:50 body, rig, structure, thermal: form, proportions, joints and load path agree with the table from all six directions and in both poses; nothing floats or pokes through. No rebuild needed. Omissions stated below.
- **1:100, 1:200, 1:500 monitor was the 1:50 parts list** (bezel frame with hole, 9 mm backplane, 3 mm rear panel, five-section bulge, 2.5 mm VESA plate, two 9 mm hinge ears, Ø 12 hinge pin, four Ø 7.6 screws) — at 1:500 twelve solids of 2 – 17 mm against a 75 mm feature, and the decisions texts described something else ("60 mm panel block", "monitor base omitted" while the base was in the list).

### Rebuilt (`rework-14-height-adjustable-desk-coarse.py`, hand-chosen numbers)

- **1:100 body**: 14 mm panel slab with the glass set 1 mm into its face, rear bulge in three sections, one 116 x 116 x 16 mm mount block (VESA plate + hinge) that meets the stand. 30 → 22 solids.
- **1:100 thermal**: the housing is the hollow shell it is (8 mm bezel ring + 3 mm rear wall around the display cavity as own thermal solids), bulge and mount block on the rear; area of `monitor-housing` back inside the 30 % band (a solid slab counts its face behind the glass: 0.647 m² against 0.449).
- **1:200 body/thermal**: 29 mm panel slab, glass, two-section rear housing that tapers to the stand. 29 → 20 solids.
- **1:500 body/thermal**: one 59 mm panel block from glass to stand, glass sheet for the `display-front` region. 28 → 18 solids.
- Decisions of 1:100, 1:200, 1:500 corrected in English and German (first and third paragraph) to say what is modelled.
- Structure lists unchanged (monitor is a load; its mass marker stays on `screen-tilt`).
- 2D: not changed. The coarse monitor differs from the drawings by less than a pen width (`volumes-silhouette` and `views-disagree` pass); `r3-2d` 1:500 read.

### Validation

`VALID furniture/height-adjustable-desk (catalog asset + volumes source): schema 0, validateAsset 0, checkVolumes 0`; published with `volumes-insert.py`; `r3-validate.ts furniture/height-adjustable-desk --catalog-only --volumes` (with print checks): `VALID ... (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.

### Remaining, stated plainly

- 1:50 body: no cables (mains, motor and handset leads are Ø 5 – 7 mm, under the 7.5 mm feature), no plastic end caps on the feet, no fixing screws of rails and top; the foot is a solid pressed profile, not an open shell from below.
- 1:100 / 1:200 2D: the drawings still carry the `monitor-hinge` stroke (100 x 12 mm) of the 1:50 drawing where the 3D now has a mount block / tapered housing; front and plan still draw the bulge with the 1:50 outline at 1:500 where the 3D is one block. Sub-pen, but not redrawn.
- 1:200 and 1:500 frame: tubes, crossbar, control box, stand and laptop keep true sections below the feature (decision C13 of the asset); not typified.
- Rendered after the last change but not read: 1:200 thermal / structure / standing, 1:100 structure / standing, 1:500 thermal, raise-and-lower t 22, load-on-surface t 2 (1:50 lists are unchanged since they were read).
- Thermal area at 1:200 / 1:500 for `monitor-housing` is reported-only by the rule (deviation about 0.5, as before this pass); the hollow shell was only modelled at 1:100.

### Fundamental issues (outside this asset)

- `volumes-thermal-coverage`: the face of a housing behind a glass of another region counts as outer area of the housing, and the start cap of a loft standing inside a closed cavity counts too (found by bisecting: +0.20 m² and +0.15 m²). Authors have to model the cavity hollow and keep caps out of it; README section "Volume" does not say this for glazing in front of a slab.
- `volumes-check.ts` without `--product` prints a different engine's numbers with `FAIL` for a source the product accepts (`drive-left` 0.56 at 1:50); misleading while iterating.
- `volumes-feature` accepted Ø 7.6 x 11 mm screws and 9 mm ears at 1:500 (copied 1:50 monitor passed `VALID`); the rule does not catch compact sub-feature parts that touch a larger solid.
