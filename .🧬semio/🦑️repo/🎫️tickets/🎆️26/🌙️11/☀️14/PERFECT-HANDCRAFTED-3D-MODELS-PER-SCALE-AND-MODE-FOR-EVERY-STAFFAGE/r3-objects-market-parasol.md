# Round 3: objects/market-parasol (2026-11-20)

State: done. Root fixes are written into the catalog, the volumes are spliced, `r3-validate.ts` gives VALID (schema 0, validateAsset 0), and `volumes-check.ts --product` passes every rule at every scale (45 of 45 lines ok).

No earlier round-3 work on this asset existed: there were no `r3-market-parasol-*` scripts and no report. Backups of the state before this round are in `🗑️generated/r3-market-parasol/catalog-before-r3.json` and `source-before-r3.json`.

## Scripts (ticket folder)

- `r3-market-parasol-kit.py`: the linkage maths and the product's bone-frame convention. The convention was verified against `evaluate`: u = (cos A cos b, sin A cos b, sin b), v = (-sin A, cos A, 0), and twist gives v' = cos t·v − sin t·w.
- `r3-market-parasol-roots.py`: writes the rig, poses, clip key, summary, notes, reference, structure assumptions and all 2D, first as a candidate and with `--write` into the catalog.
- `r3-market-parasol-draw.py`: the 2D drawings of 4 scales × 3 views.
- `r3-market-parasol-texts.py`: the bilingual texts.
- `r3-market-parasol-volumes.py`: body, thermal and structure at all four scales.
- `r3-market-parasol-probe.ts`: bone frames per pose.
- `r3-market-parasol-io.py`: a loader for the kit.

## Reference table

| Item | Real value | Source |
|---|---|---|
| Canopy | Octagonal, 8 ribs at 45°, taut flat gores over straight aluminium ribs, seams over the ribs | Maanta LYBRA product data (8 ribs); general construction knowledge |
| Diameter | 2.5–3.0 m classes; this asset declares 2.59 m | asset `dimensions`; retailer data (Ø 2.5 m / 2.7 m crank-and-tilt models) |
| Headroom under the hem | 2.04 m (Ø 2.7 m), 1.98 m (Ø 3.0 m) | Maanta LYBRA (archiexpo product 4631045-2565737) |
| Overall height | 2.25–2.45 m for 2.5 m models; asset 2.46 m | retailer data (H 2.25 m, H 2.45 m) |
| Mast | Aluminium, Ø 38 mm (consumer) to Ø 48 mm (commercial), two-part with a coupling sleeve | Maanta, retailers (38 mm); commercial practice |
| Ribs | Aluminium U or rectangular profile, about 16 × 10 mm, pinned in the crown (top notch) | construction knowledge |
| Stretchers | One per rib, from the runner notch to a bracket at about 35–45 % of the rib length | construction knowledge |
| Opening | Crank winch on the mast, cord inside the mast pulls the runner | retailer data (crank and tilt) |
| Base | Concrete ballast; this asset: 0.4 × 0.3 × 0.2 m, 57.6 kg | asset, EN 1992-1-1 (density) |
| Fabric | Polyester 280 g/m², 0.5 mm | asset materials |

## Audit (before this round)

- **Rig, root defect.** The ribs were not at 45° spacing. Measured plan azimuths in `open-still` were 0, 55.5, 101.5, 148.0, 180, 235.5, 281.5 and 328.0 degrees, so the gaps were 55.5 / 46 / 46.5 / 32 degrees. The titles and summary claim 11 + 45k degrees. `rib-right` and `rib-left` lay in the paper plane at 0° and 180°.
- **Rig.** There were only 2 stretchers (in the paper plane) for 8 ribs, so 6 ribs had no prop.
- **Rig.** Rib length was 1.2374 m. The tips reached radius 1.194 m, but `widthM` is 2.59 m. The missing 0.1 m was a flat fabric "hem skirt" sticking out horizontally past the rib tips, which does not exist on a real parasol.
- **Rig.** The mast pivot was at y = 0.20 m, on the block top, inside a socket cone that reached 0.30 m with a 9–19 mm gap. Tilting 10° would collide with the cone.
- **2D and 3D canopy.** The canopy was a dome of revolution sitting up to 8 cm above straight ribs: the fabric floated over the ribs. The plan was a circle, but a real 8-rib canopy is an octagon. The scallop strokes were drawn outward from the tips.
- **3D.** Rib tips were ball ellipsoids. The crank was two capsules. The structure's mast member used material `concrete`. The base-plate region was `base-concrete` at 1:100 but `frame-alu` at 1:50.
- **Texts.** The structure assumption described "foreshortened projections ... shorter than true", which was obsolete.

## Root fixes (catalog file)

| Field | Old | New |
|---|---|---|
| `mast.y` / `lengthM` / mass `comX` | 0.20 / 2.15 / 1.075 | 0.26 / 2.09 / 1.045. This makes room for a clevis pin 60 mm above the plate; the crown stays at 2.35 m. |
| `crown.x`, `runner.x` | 2.15, 1.75744 | 2.09, 1.69744 (world positions unchanged) |
| rib `lengthM` / `comX` | 1.2374 / 0.6187 | 1.3676 / 0.6838. The tip-cap ends at azimuths 11° and 191° now give exactly 2 × 1.295 = 2.59 m. |
| rib joints | angle limits only | + `twistMinDeg -180`, `twistMaxDeg 180` |
| rib azimuths (all poses) | uneven, see audit | exactly 11 + 45k degrees at elevation −15.2803°, each with a twist that aligns the rib's v with its gore normal |
| stretchers | 2 | 8 (`stretch-b`, `-c`, `-d`, `-f`, `-g`, `-h` added). Each meets its own rib at s0 = 0.5023 m from the crown; the linkage closes exactly in every pose. |
| `furled` | ribs at varied angles, dx −0.65381 | rib elevation −83.65° solved from the linkage, stretchers solved too |
| `wind-flap` key t = 4.5 s | angles only (abduction inherited, so the azimuths drifted) | runner dx +0.04896, ribs at −11.343° solved from the linkage, full angle, abduction and twist for 8 ribs and 8 stretchers |
| summary, `provenance.notes`, `structure.assumptions[2]`, furled and wind-flap descriptions | old | rewritten in en and de |
| `provenance.references` | — | + `maanta-lybra` |
| 2D geometry and decisions | dome canopy, scallops, folds, revolve shapes | redrawn at all scales (below) |

`dimensions`, masses and `areaM2` are unchanged.

## 2D (ruling E1)

Each gore is an `extruded` (sweep v) shape on its rib, with the same definition as the 3D gore. Elevation, front and plan are therefore the true projections and follow every pose. Ribs, tip caps and stretchers are `member` shapes. Everything else (block, plate, clevis, mast, collar, crank, runner, hub) is flat on non-spread bones.

- **1:50:** block with seam and grit, plate, bolts, clevis, pin, two-part mast, collar with knob, winch, crank, runner, hub with cap and finial, 8 ribs, 8 tips, 8 stretchers, 8 gores.
- **1:100:** merged mast strip, collar, winch with crank as one outline, hub outline, ribs, stretchers, 8 gores.
- **1:200:** foot block, mast, runner, hub, 8 gores.
- **1:500:** block, mast stroke, hub, 8 gores.

Removed: the scallop, seam and fold strokes of the dome, and the runner detent. I looked at `r3-2d` at 1:50 (default pose and `furled`), 1:100 and 1:500.

## Volumes (body / thermal / structure solid counts)

- **1:50 (60 / 68 / 51).**
  - Body: concrete block (12 mm rounded edges); plate with 4 bolts; clevis flange and two cheeks; mast lug, 12 mm pin and two nuts; lower tube Ø 48 mm; coupling collar Ø 68 mm with clamp knob; upper tube Ø 40 mm; winch housing, bent crank and handle; runner sleeve with notch ring; crown hub with domed cap and finial; 8 ribs (16 × 10 mm boxes) with tip caps and stretcher brackets; 8 stretchers (11 mm, slender); 8 flat 8 mm gores on their ribs.
  - Thermal: references, plus 8 hidden 0.5 mm underside shells.
  - Structure: block member, mass and fixed support; pivot pin joint; hollow tubes 48 × 2 and 40 × 1.5 mm; coupling as a splice joint; crown and runner joints; 8 rib members (and tips); 8 rib/stretcher pin joints; 8 stretcher members; masses of mast, runner and every rib.
- **1:100 (32 / 40 / 32):** block, plate, clevis block, single 44 mm mast, collar, winch with crank as one part, runner, hub merged with cap, 8 slender ribs, 8 slender stretchers, 15 mm gores; the structure is drawn anew.
- **1:200 (13 / 37 / 22):** block, foot block, mast, runner, hub, 30 mm gores. Ribs and stretchers are slender bars in thermal and structure only.
- **1:500 (11 / 19 / 21):** block, slender mast (reaches into the block), hub, 75 mm gores; structure with box-bar ribs and stretchers.

Check (`volumes-check.ts --product`, all ok):
- Silhouette IoU elevation / front / plan: 1:50 0.998 / 0.998 / 0.996; 1:100 0.999 / 0.998 / 0.996; 1:200 and 1:500 1.0. The p95 boundary distance is 0 at every scale.
- Bounds: within 0.6 mm at 1:50.
- Areas at 1:50, measured against declared: canopy-top 5.22 against 5.3, canopy-underside 5.00 against 5.3, frame-alu 1.22 against 1.12 (+9 %), base-concrete 0.50 against 0.52.
- Monotonic counts drop at every step; nothing is copied between scales.

## What I saw in the final renders

- **1:50 body, `r3-sheet`:**
  - Front, both sides and back: a classic market parasol. A shallow octagonal pyramid with ridge lines over the near ribs, eight tip caps at the hem, the hub cap and finial on top, the runner under the hem, the coupling collar with its knob, and the crank on the left side.
  - Top: a true octagon with 8 seams.
  - Three-quarter from above, and one view from below (turn 40°, tilt −22°): ribs with tip caps, brackets at 37 % of the rib, 8 stretchers converging on the runner, the bent crank with handle, the clevis on the plate with bolts. Nothing floats and every part joins.
- **1:50 `furled`:** ribs and stretchers fold correctly along the mast, but the rigid gores hang as tangential fins (see below).
- **1:50 `wind-flap` at t = 4.5 s:** the mast tilts on the clevis, the ribs lift to −11.3° and the linkage stays closed.
- **1:50 structure:** a clean load path. Hollow mast, rib and stretcher bars, orange joints at the crown, runner, coupling and pivot, and the block.
- **1:50 thermal (cut):** regions are coloured correctly, the underside shells sit under the gores, and nothing pokes out.
- **1:100, 1:200, 1:500:** the same object, simplified, always readable. At 1:500 the gap between mast and block was fixed after the first render.

## Still imperfect (stated plainly)

- **Furled pose.** Each gore is a rigid solid on its rib (and in 2D an extruded shape on the same rib). The furled canopy therefore shows 8 tangential fins, about 1.7 m across in plan, instead of cloth bunched around the mast. No rigid assignment of a gore to rib bones can shrink a 1 m wide gore. Per-solid skinning (`weights`) is a whole-solid blend, and the r3 tools ignore it. The 2D and 3D are consistent with each other in this pose, and the default pose is exact.
- **Fabric thickness.** The gores are as thick as the scale feature (8 / 15 / 30 / 75 mm), not 0.5 mm, because there is no thin membrane role.
- **Clamp knob side.** The front 2D draws the collar knob at x = −0.061. The mirror sense of the front x axis against world z was not cross-checked with `r3-project2d`. It is a 0.3 mm detail, but it may be on the wrong side.
- **Rib profile.** The ribs are 16 × 10 mm boxes, not open U-profiles.
- **Underside area.** The canopy-underside shells measure 5.0 m² against the declared 5.3 m² (−5.7 %). This is within tolerance and was not changed.
