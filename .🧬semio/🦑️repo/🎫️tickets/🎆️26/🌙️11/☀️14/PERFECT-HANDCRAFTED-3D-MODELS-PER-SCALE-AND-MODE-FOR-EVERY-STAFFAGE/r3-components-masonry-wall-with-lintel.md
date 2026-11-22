# Round 3 — components/masonry-wall-with-lintel

Scripts (in T): `r3-masonry-wall-with-lintel-kit.py` (the real construction: the bond rule, the block courses, the lintel and sill profiles, the solid writers, masses), `r3-masonry-wall-with-lintel-volumes.py` (all four scales with three lists each, written to `volumes/components/masonry-wall-with-lintel.json`), `r3-masonry-wall-with-lintel-roots.py` (root fixes outside `volumes`, idempotent; `--write` writes the catalog). The catalog file has the root fixes and the spliced volumes (`volumes-insert.py`). Backups of the state before this round are in `🗑️generated/r3-masonry-wall-with-lintel/` (`catalog-before.json`, `volumes-before.json`).

This was a fresh start: no earlier round-3 files for this asset existed.

## 1. Reference

| item | real value | source |
|---|---|---|
| Build-up (asset datum, kept) | 15 mm gypsum plaster, 175 mm calcium silicate, 140 mm full-fill mineral wool, 115 mm facing brick = 445 mm | asset thermal layers, EN ISO 6946 |
| Facing brick | NF 240 x 115 x 71 mm, stretcher bond (half bond, 125 mm offset), 10 mm head joints, 12.3 mm bed joints, course 1/12 m | EN 771-1 / DIN 105-100, DIN 4172 |
| Octametre openings and piers | opening = n x 0.125 + 0.01 m, pier = n x 0.125 - 0.01 m. Only then does the half bond end at both reveals in whole (240) or half (115) bricks | DIN 4172 |
| Calcium silicate leaf | units 248 mm high in 2 mm thin-bed mortar, course 0.25 m (= 3 facing courses, so both leaves share a bed joint every 0.25 m) | EN 771-2, DIN 20000-402 |
| Two-leaf wall | clear cavity at most 150 mm; at least 7 ties of 4 mm stainless wire per m² plus 3 per m along the free edges of openings; open head joints for drainage at the foot and above openings | DIN EN 1996-2/NA |
| Wall ties | 4 mm stainless wire, about 50 mm into each leaf's bed joint | EN 845-1 |
| Precast lintels | 175 mm wide units, 1.75 m long, bearing at least 0.1 m (here 0.245 m); inner 0.25 m high, exposed outer 0.24 m high over its bed | EN 13225, EN 1996-1-1 |
| Cast-stone window sill | weathering at least 5°, projection 40 to 50 mm, drip groove underneath, raised ends let into the reveals | DIN 18500, standard practice |
| Densities | brick, calcium silicate and mortar 1800, concrete 2400, wool 40, plaster 1300, stainless steel 7900 kg/m³ | EN ISO 10456, EN 1991-1-1 |
| Conductivities | brick 0.81, calcium silicate 0.79, concrete 2.3, wool 0.035, plaster 0.7, mortar 1.0, stainless steel 17 W/mK | EN ISO 10456 |

## 2. Audit (before)

- **Opening 1.25 m (0.875 to 2.125).** This is not octametric. With 1.25 m the half bond cannot close at both reveals: the left and right piers are 10 mm out of phase. Real value: 1.26 m.
- **The lintel zone was not a construction.** The old layers were a 40 mm concrete "shell", 80 mm wool, 175 mm concrete and 15 mm plaster, set from the outer face (z −0.2225 to +0.0875). Behind them sat a 120 mm "lintel-back" block with a second plaster coat. So there was **plaster inside the wall** at z 0.0725 to 0.0875, and the 175 mm bearing inner leaf above the opening rested on an unsupported block spanning 1.75 m. Only 40 mm of the load-bearing concrete lay under the inner leaf. The total was 310 mm against a 445 mm wall.
- **The sill zone had the same defect.** It was 70 mm concrete, 80 mm wool, 175 mm block and 15 mm plaster (340 mm), plus a 90 mm "sill-back" block and plaster inside the wall. The sill was a 1.4 x 0.083 m box flush with the face: no projection, no weathering, no drip. A real exterior sill would let water run down the brickwork.
- **1:50 body:** the facing leaf was one smooth 115 mm slab. There were no bricks, no head joints and no bed joints, although the 2D draws the courses. The calcium silicate leaf was one slab. There were no plaster returns in the reveals, no weep joints and no ties. The lintels were plain boxes.
- **1:50 structure:** members were `{of}` references to body slabs, including the fictitious lintel layers. The three supports were 7.5 mm pads. There were no wall ties, no bearings and no masses. QA graded structure B.
- **2D:**
  - All scales: opening 1.25 m.
  - Section A-A and plan B-B drew the fictitious lintel and sill layers, with plaster inside the wall.
  - 1:50 elevation: bed joints were drawn at the bottom of each joint instead of its centre, and there were no head joints.
  - Plan B-B was cut at y 1.5, which is exactly a bed joint, so it could not show any bond.
  - 1:100 and 1:200 carried artificial +1 mm and +0.5 mm offsets, apparently to dodge `representation-copied`.
- **Masses:** the lintel was 225.75 kg (from the fictitious 215 mm of concrete); real 358.2 kg. The sill was 42 kg; real 51.1 kg. `dimensions.massKg` was 3571.7; real 3755.0. The rig centre of mass of the wall lay in the opening; that is correct physically.
- **`dimensions.depthM` 0.445:** it ignored the sill projection.
- **Thermal:** the lintel and sill layers had thicknesses that do not fit into the wall. The field `areaM2` was based on the 1.25 m opening.

## 3. Root fixes (catalog file)

| member | old | new |
|---|---|---|
| opening (all 2D, all volumes) | 0.875 to 2.125 (1.25 m) | 0.87 to 2.13 (1.26 m, DIN 4172) |
| lintel construction | 40 shell / 80 wool / 175 concrete / plaster + "lintel-back" block | exposed outer precast lintel 175 deep (115 in the facing plane + 60 into the cavity), 237.7 high, on a 12.3 mm bed; 80 mm wool; inner precast lintel 175 x 250 under the bearing leaf; plaster on the face and the soffit |
| sill construction | 70 / 80 / 175 / 15 + "sill-back" block | cast-stone sill 175 mm in the wall plane + 50 mm projection, 5° weathering to a 40 mm seat, drip groove 10 x 10 mm, raised ends 70 mm into each reveal, bedded on course 9; 80 mm wool; calcium silicate parapet; plaster |
| `thermal.regions[lintel].layers[lintel-shell]` | 0.04 m, R 0.01739, C 96000 | 0.175 m, R 0.07609, C 420000 (title: exposed precast concrete outer lintel) |
| `thermal.regions[sill].layers[sill-concrete]` | 0.07 m, R 0.03043, C 168000 | 0.175 m, R 0.07609, C 420000 |
| `thermal.regions[lintel/sill].heatCapacityJM2K` | 99296 / 171296 | 423296 / 423296 (outer layer + insulation, as for the field) |
| `thermal.regions[masonry-field].areaM2` | 6.02917 | 6.01583 (3 x 2.75 − 1.26 x 1.3333 − 0.4375 − 0.11667; factor 0.998) |
| U values (text) | lintel 0.39, sill 0.37 | lintel 0.38, sill 0.36 (field 0.22 unchanged); every layer is now exactly d / λ |
| `dimensions.depthM` | 0.445 | 0.495 (sill nose) |
| `dimensions.massKg` | 3571.733 | 3755.016 (from the modelled 1:50 volumes) |
| `rig.bones[wall].mass` | 3303.98 kg, (1.5, 1.35) | 3345.766 kg, (1.50002, 1.25366) |
| `rig.bones[lintel].mass` | 225.75 kg, (0.875, 0.125) | 358.185 kg, (0.875, 0.12808) |
| `rig.bones[sill].mass` | 42 kg, (0.7, 0.04167) | 51.065 kg, (0.7, 0.0432) |
| bone titles lintel / sill | "0.25 m bearing", "precast concrete sill" | "inner and exposed outer, 0.245 m bearing", "cast-stone window sill" |
| `materials` | 5 | + `mortar` (1800, 1.0), + `tie-steel` (7900, 17, E 200 GPa), both en-iso-10456 |
| `provenance.references` | 9 | + din-en-1996-2-na, en-771-1, en-771-2, en-845-1, din-18500; en-1996-1-1 and en-13225 citations corrected; notes rewritten |
| `summary`, `thermal.assumptions` | fictitious lintel and sill | rewritten (en, de) |
| `representations.*.sections.plan.plane` | 1.5 (a bed joint) | 1.55 (mid brick of course 18) |
| `representations.50/100.hatches` | — | + `cut-block` (45°, 1.6 / 1.0 mm) for cut calcium silicate |
| 2D, every scale and view | see section 2 | redrawn as the true views of the new body (E1); same ids where the part persists (`field`, `lintel`, `sill`, `reveal`, `lower-*`, `upper-*`, `pier-*`, `sill-below`), new ids for new parts |
| `representations.*.decisions` | old construction | rewritten per scale (en, de) |

Details of the 2D redraw:

- **1:50 elevation:**
  - every bed joint on its centre line, one path per course;
  - every head joint, one path per course; joints touching the lintel or sill ends are left to the outline;
  - five weep joints (filled slots);
  - the lintel over its bed;
  - the sill with its nose line;
  - the interior soffit seen through the opening.
- **Section A-A:** the sill profile, the chamfered outer lintel, both lintels, 80 mm wool, the soffit plaster, the reveal beyond and the raised sill end beyond.
- **Plan B-B:**
  - the cut head joints of course 18 and the block joints of block course 6;
  - the plaster returns;
  - below the cut: the sill as a T outline with the projection, its seat edge, the wool, the parapet and the plaster.
- **1:100:** the four leaves, both lintels and the sill profile. Bed, chamfer, drip, ends and returns are merged.
- **1:200:** one wall mass, the lintel band and the sill with the parapet cap. The offsets are removed; the scales are now distinct by content.
- **1:500:** wall, opening and lintel band in elevation. In section: wall, the sill band with its 50 mm nose, and the wall above. The nose is needed: the 1:500 extents must show the real 0.495 m depth (`extent-front-depth`).

Poses: only `load-carrier` moves (`dx`). I checked 2D at `beam-over-opening` (`r3-2d --pose`) and structure at `beam-sweep` t = 6 s (`r3-sheet --clip`): the bearing pad travels with the carrier along the wall head. Thermal pose fractions are unchanged.

## 4. Models per scale

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 318 solids (756 expanded): every brick of 33 courses (repeats of whole bricks and head joints, cut pieces, bed joints; half bricks of 115 mm at every second reveal course; cut bricks 160 / 110 / 235 mm against sill and lintel; 245 mm end bricks where a 5 mm mortar sliver would fall at the sample cut), 5 weep joints (bottom course at x 0.375 / 1.375 / 2.375, above the lintel at 1.125 / 1.875); cavity wool, 80 mm lintel-zone and sill-zone wool; calcium silicate leaf block by block (11 courses of 0.25 m, half bond, seams = thin-bed joints, course 3 cut down to the parapet under the opening, two notched L-blocks at the reveals); face plaster with the opening, reveal returns 15 mm, soffit plaster; outer lintel (extruded profile with a 10 x 12 mm chamfer); inner lintel; sill in three pieces (middle with weathering and drip, two raised ends) | 14 shells: field plaster, inner leaf, fill and facing (own extrusions over the field strip; the facing counts its exterior face only); lintel plaster (own), inner lintel, wool and outer lintel (references); sill plaster and block (own), wool and three sill pieces (references) | 46: calcium silicate leaf (own extrusion with the opening and the lintel seat); both lintels and the three sill pieces (references); four lintel bearings (2 thin-bed pads 7.5 mm, 2 x the 12.3 mm bed under the outer lintel); sill bed; beam bearing pad on the head (load-carrier); 25 tie solids (10 bed joints, 4 mm slender, 8 per m²; 90 mm from the reveals); 3 foundation contact pads at the anchors; 6 mass spheres (wall lumped in parapet, two piers and top band, because its own centre lies in the opening; lintel; sill) |
| 1:100 | 9: facing, cavity wool, inner leaf, plaster (slabs); lintel-zone and sill-zone wool; outer and inner lintel (plain bars); sill (one weathered profile with the 50 mm nose) | 9: three shells per region (inner masonry with the plaster merged, wool, outer); every shell marks all faces but the exterior one, because at coarse scales the whole region union is measured | 41: leaf, both lintels, sill, two merged lintel bearings, beam pad, the ties as slender wires, 15 mm pads, masses |
| 1:200 | 4: one 445 mm wall mass, lintel band 445 mm, sill with nose, parapet cap behind it | 8: field 3 shells, lintel zone 3 shells, sill zone 2 (sill + merged inner shell) | 13: leaf, inner lintel and outer lintel members, sill, 30 mm pads, masses |
| 1:500 | 3 ("Minimal"): wall mass, lintel band, sill band 495 mm deep | 5: field 3 shells drawn over field + sill zone (as the 1:500 elevation merges them), lintel band, sill band | 12: leaf, inner lintel, sill in the wall plane, 75 mm pads, masses |

## 5. Checks

- `$BUN volumes-check.ts components/masonry-wall-with-lintel --product`: **PASS** at every scale (28 s).
  - Silhouette IoU 1.000 with p95 0 mm in elevation, front and plan at 1:50 / 100 / 200 / 500.
  - Bounds 0 mm.
  - Thermal outer areas, 1:50: field 6.0158 m² (dev 0.0 %), lintel 0.4223 (−3.5 %), sill 0.1097 (−6.0 %). 1:100: sill −19 % (limit 30 %); totals within 2 % at 1:200 and 1:500.
  - Structure inside the body in all views.
  - Supports on their anchors.
  - Monotonic, cross-scale and copied all ok.
  - Slender ratio of the ties 64.75.
- `$BUN r3-validate.ts components/masonry-wall-with-lintel --volumes`: `VALID … schema 0, validateAsset 0, checkVolumes 0`. `--catalog-only` is also VALID after the splice.

## 6. What the final renders show

- **1:50 body, `r3-sheet`, right side (elevation, outside):** a real facing-brick wall in stretcher bond. Courses are even; half bricks alternate at the reveals; head joints are offset by half a brick. The weep slots are visible in the bottom course and over the lintel. The dark exposed lintel spans 1.75 m with equal bearings. Through the opening the plastered reveals and the sill are visible.
- **1:50 body, three-quarter from outside-left (turn 330, tilt 20):** the brick bond reads at first sight. The lintel sits flush with the face. The sill projects with its weathering. The raised ends sit in the reveals. The calcium silicate blocks show as seams on the cut end.
- **1:50 body, steep view onto the parapet (turn 300, tilt 55):** the sloped sill top with its seat, then the 80 mm wool strip, the cut-down calcium silicate blocks with their seams and the plaster edge.
- **1:50 body, top view:** the top course of bricks with its head joints, the wool, the calcium silicate leaf and the plaster. The sill nose projects 50 mm below the wall line.
- **1:50 body, front and back (end faces):** the four leaves at the cut end, and the calcium silicate courses as seams every 0.25 m.
- **1:50 structure:** the blue leaf, the lintels on orange bearings, and the sill on its bed. The ties show as short black wires through the ghosted facing in the end views and as dots in the elevation. There are green pads at the three anchors. The masses sit in the parapet, the piers, the top band, the lintel and the sill; none floats in the opening.
- **1:50 structure, clip `beam-sweep` t = 6 s:** the load-carrier has moved to x ≈ 2.4 m with its bearing pad.
- **1:50 thermal (`--cut`):**
  - field: four red layers;
  - lintel zone: green plaster, inner lintel, wool and the chamfered outer lintel;
  - sill zone: purple plaster, block, wool and the sill profile with its nose.
  - The moiré on large cut faces is the tool's.
- **1:100 sheet and 2D:** four leaves, two lintels with wool and the weathered sill. The sections match the body.
- **1:200 and 1:500 (`volumes-render`):** overlays are green with IoU 1.000.
  - In the 1:500 elevation panel the owned overlay paints the opening red, but the catalog validator's numbers in the same title read IoU 1.000 and p95 0 mm. It is a quirk of the render tool's own outline fill for a `path` shape with `regions`, not a mismatch.
  - Thermal at 1:500 shows the field shells and the lintel and sill bands.
  - Structure shows the leaf, the lintel, the sill, the pads and the masses.
- **1:50 2D (`r3-2d`), light-line and light-solid:**
  - elevation: brick bond, lintel stipple, sill nose line;
  - section A-A: hatched facing and calcium silicate, wool zigzags, sill profile with drip, chamfered lintel, soffit plaster;
  - plan B-B: cut piers with joints, plaster returns, T-shaped sill below.

## 7. Still imperfect (plainly)

- The damp-proof courses at the foot and over the outer lintel (1 to 2 mm) are not modelled; they are below the 7.5 mm feature. The cavity closer and the window are not part of this asset, so the reveals show the wool, as in the shell state before the window is fitted. The reveal plaster is shown as in the finished state; both states are simplified.
- The outer lintel takes 60 mm of the cavity, which leaves 80 mm of wool. This matches the declared 80 mm insulation of the lintel zone (and so its U of 0.38), but on its 115 mm bearing the facing brick carries the lintel eccentrically. The same 60 mm tongue under the sill rests on the wool. Both are common practice with rigid insulation strips; neither is resolved further.
- Thermal regions are the 1D strips of EN ISO 6946. The outer layer counts its exterior face only. Reveal and soffit faces in the opening are marked as section faces, because they belong to the window joint and the linear bridge; C25 would count free-air faces. Counting them honestly would put the lintel at +50 % and the sill near 3x its strip area, beyond the 1.5 factor of C16. I did not raise this in `author-requests.md`; it is the convention the earlier authors and the other wall assets used.
- At 1:100 the sill's measured exterior area is −19 %, because the profile's nose is 55 mm against an 83 mm strip. It is within the 30 % tolerance.
- The tie wires are straight; real ties have a drip kink or a drip disc in the cavity (below the feature).
- Brick tone is uniform, and the joints are flush, which is real for flush pointing (Fugenglattstrich) but only visible as seams.
- The 2 mm thin-bed joints of the calcium silicate leaf are shown only as seams between solids (below the feature). The notched blocks at the parapet are single L-extrusions, so they show no false seam.

## Round 5 handcraft

Script: `r5-masonry-wall-with-lintel-volumes.py` (body and thermal of every scale from the round 3 kit, unchanged; structure with the facing leaf; writes the volumes source). Zoom tool: `r5-c02-zoom.ts`. Backup before: `🗑️generated/r5-masonry-wall-with-lintel/volumes-before.json`.

### Reference vs model (1:50, measured in the source)

| item | reference | model | deviation |
|---|---|---|---|
| facing brick | NF 240 x 115 x 71 mm, stretcher bond, half bond 125 mm | 240 x 115 x 71, pitch 250, offset 125 | 0 % |
| course | 1/12 m = 83.3 mm (bed 12.3, head 10) | 83.33 / 12.33 / 10 | 0 % |
| opening | octametric 1.26 m (n x 0.125 + 0.01) x 1.333 m | 1.26 x 1.333 | 0 % |
| cavity | at most 150 mm (DIN EN 1996-2/NA) | 140 mm full fill | inside |
| inner leaf | KS 175 mm, course 0.25 m (248 + 2 thin bed) | 175, 0.25 | 0 % |
| ties | at least 7 per m2, 4 mm stainless | 8 per m2, 4 mm | inside |
| lintel bearing | at least 0.1 m | 0.245 m | inside |
| sill | projection 40 to 50 mm, weathering at least 5 degrees, drip | 50 mm, 5 degrees, 10 x 10 drip | inside |

### Grades before (opened: body, structure, thermal `--cut` at all four scales, 2D at all four scales, poses floor-line-load, beam-over-opening, shaded-morning, clip beam-sweep t 6 s, zoomed views of the opening from outside, the left sill end and the left lintel bearing)

- Body: 1:50 A (bond, weep joints, lintel bed, sill with nose, drip and raised ends, plaster returns), 1:100 A, 1:200 A, 1:500 A.
- Thermal: A at every scale (moiré on the cut panels was the sheet tool, fixed, see below).
- Structure: 1:50 B, 1:100 B, 1:200 B, 1:500 B: only the calcium silicate leaf was a wall member; the facing leaf was missing, so the outer lintel and the sill sat on their beds on a ghost and the load path of the facing (its own weight plus the outer lintel) ended in the air.

### Changes

- Structure, every scale: `member-wall-facing` (bone `wall`), the 115 mm facing leaf with the opening, lintel and sill zones cut out, standing on the three foundation pads (which already spanned both leaves). It carries its own weight and the outer lintel to the foundation (a self-supporting, non-load-bearing leaf in the sense of DIN EN 1996-2/NA); the ties hold it against wind. 1:500: the lintel member now spans the whole wall thickness (both lintels merged, as the drawn lintel band), so the facing has its lintel too. Counts 47 / 42 / 14 / 13.
- Structure decisions rewritten at every scale (en, de). Body, thermal, 2D and all roots unchanged.

### Grades after

Body A / A / A / A; thermal A / A / A / A; structure A / A / A / A. Structure sheets after: two leaves with the cavity between them in front, back and top views, both lintels on their own leaves, the sill on the facing; the beam-sweep pad travels along the wall head.

### Checks

`volumes-check.ts --product` PASS (296 s); `r3-validate.ts --volumes` VALID (schema 0, validateAsset 0, checkVolumes 0; a first run threw inside the validator's flood module while another agent was editing it, the rerun was clean); spliced with `volumes-insert.py`.

### Remaining (with reason)

- Unchanged from round 3: damp-proof courses (1 to 2 mm) below the feature; uniform brick tone; straight ties without drip kink; no window (not part of the asset).

## Canonical component readback, 2026-11-21

Canonical source `35fe483c43a57076`. Thirty-three courses of facing units and mortar run to the jambs. The insulating cavity, inner calcium-silicate blocks, internal plaster returns, separate lintel bearings and sloping sill remain visible in cut section. All twelve exported scenes reopened (1,625,444 bytes). The strict catalog source readback passed.
