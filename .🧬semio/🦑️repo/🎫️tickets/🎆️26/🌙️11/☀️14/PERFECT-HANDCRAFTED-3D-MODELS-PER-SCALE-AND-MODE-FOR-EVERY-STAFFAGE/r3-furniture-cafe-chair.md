# Round 3 — furniture/cafe-chair

State: finished and spliced (2026-11-20). There was no earlier round-3 work on this asset: no `r3-cafe-chair-*` scripts and no report. At the start, the catalog `volumes` equalled `volumes/furniture/cafe-chair.json`, `r3-validate` said VALID and `volumes-check --product` said PASS. The asset was rebuilt from the roots. Everything below describes the catalog file as it is now.

## Scripts

All scripts are in the ticket folder:

| Script | What it holds |
| --- | --- |
| `r3-cafe-chair-kit.py` | The measures, bend geometry, world-to-bone transforms, section quaternions, plan outline of the seat, masses and the corrected rig |
| `r3-cafe-chair-body.py` | Every body solid of every scale, stated by hand |
| `r3-cafe-chair-lists.py` | Thermal and structure lists, volume decisions |
| `r3-cafe-chair-roots.py` | Summary, provenance, dimensions, materials, anchors, tilt pivot, back load, thermal regions and assumptions, supports, structure assumptions |
| `r3-cafe-chair-draw.py` | 2D shapes and representation decisions of all scales (ruling E1) |
| `r3-cafe-chair-build.py` | Builds the candidate document and its adhoc volumes source |
| `r3-cafe-chair-apply.py` | Writes the root members into the catalog in its own formatting and copies the volumes source |
| `r3-cafe-chair-probe.ts` | Diagnosis of the slender stroke rule (section 4) |

Running build and apply a second time changes nothing (the output is "members: none"). The scratch candidate under `🗑️generated/r3-cafe-chair/` and `volumes/adhoc/r3-cafe-chair.json` were deleted. The final renders stay in the tools' own folders.

## 1. Reference

The subject is a contract café chair of the bistro type:

- **Frame:** powder-coated round steel tube, Ø22 x 2 mm (the asset's `tube-wall` layer). On each side the rear leg rises from its glide, bends at the seat and continues as the reclined back post: one bent tube, as in the Thonet 214 bentwood logic.
- **Legs and stretchers:** two straight front legs and two Ø16 side stretchers.
- **Seat:** a solid beech board on a beech apron frame (seat rails), raked back.
- **Back:** a ladder back made of a curved beech backrest board and a lower rail, both screwed to the fronts of the posts.
- **Fittings:** nylon glides under the feet and domed plastic caps on the posts.

| Quantity | Reference | Source | Model |
| --- | --- | --- | --- |
| Overall height | 82-88 cm | Thonet 214 84 cm; Fermob Bistro 82 cm; Café VII 85.5 cm; Parisian bistro chairs 87 cm | 0.88 m (kept; cap top 0.882) |
| Seat height | 45-47 cm | Thonet 214 46 cm; Fermob 46 cm; Neufert 45-47 cm | 0.467 m at the front edge, 0.452 m at the rear edge (2 degree rake) |
| Overall depth (front-back, `widthM`) | 50-52 cm | Thonet 214 52 cm; Fermob 50 cm | 0.487 m (rear glide edge to the seat front) |
| Overall width (lateral, `depthM`) | 42-45 cm | Thonet 214 43 cm; Fermob 42 cm | 0.44 m declared (the 1:50 body is 0.445 m including the glides) |
| Seat | 40-43 x 40-43 cm, wider at the front, notched round the posts | Café VII 395 x 425 mm; Manutan 43 x 43 cm; chair-making practice | 0.43 m wide at the front, 0.398 m at the rear, 0.43 m deep, 20 mm solid beech, front corners R 30 mm |
| Apron | 18-22 x 40-60 mm rails | practice | 18 x 45 mm beech, four rails fixed with welded tabs |
| Back | backrest at 95-105 degrees to the seat; about 40 cm above the seat; board 15-18 mm | Manutan back height 42 cm; asset layer 18 mm | posts reclined 8 degrees (106 degrees to the seat); board 170 mm high and rail 50 mm high, both 0.41 m wide and 18 mm thick; board curved to a 26 mm sagitta (concave to the sitter) |
| Rear legs | raked back 6-10 degrees, feet under the back top | Thonet 214; tipping stability | 7.7 degree rake; the foot is at x 0, directly under the post top (x -0.0003) |
| Tube | Ø16-25, 1.5-2 mm wall, bend R 2-4 D | Café VII Ø16 x 2.0; Manutan Ø18; asset layer | Ø22 x 2, bend R 80 mm; stretchers Ø16 x 2 |
| Glides | Ø22-25 nylon, 8-10 mm high | Thonet 214 plastic glides | Ø25 x 10 mm polyamide |
| Densities | beech 720, steel 7850, PA 1150 kg/m3 | ISO 10456, EN 1993-1-1 | masses from the modelled volumes |
| Mass | 2.7 kg (all bentwood) up to 6-8 kg for steel frames with a solid beech seat and aprons | Thonet 214 | 7.8 kg |

Sources:

- Thonet 214: https://www.thonet.de/en/all-products/detail/214
- Fermob Bistro: https://archipro.com.au/product/bistro-natural-chair-fermob-au and https://www.lumens.com/bistro-natural-folding-chair---set-of-2-by-fermob-FRM2525319.html
- Café VII technical description: https://cdn.abicart.com/shop/32642/art96/156231896-c82da7-CAFE_VII.pdf
- Manutan beech chair on a steel base: https://manutan.co.uk/en/key/laminate-wooden-bistro-chair

The new reference ids are `thonet-214`, `fermob-bistro` and `cafe-vii-datasheet`.

## 2. Audit of the state before round 3

**Construction.**
- The "back posts" were beech capsules that started on top of the seat pad, at z ±0.19, separate from the steel legs (rear leg tops at z ±0.18, y 0.42). Nothing anchored them: the back was not connected to the frame.
- The rear legs leaned forward 2 degrees (angle 88). The rear feet (x 0.05) lay 2 cm in front of the seat rear edge and of the back top (x 0.03). That is an unstable, unreal chair, since a real chair rakes its rear legs backward.

**Dimensions.**
- Front-back depth `widthM` was 0.44 against 0.50-0.52 in the references.
- The lateral width was 0.50 (feet at ±0.245) against 0.42-0.45.
- The front legs splayed 4.5 cm per side.

**Seat and back.**
- The seat was a flat 28 mm "pad" slab with the apron centred under it.
- The back was a 28 mm flat board (its layer says 18 mm) and a flat rail, with ball finials.

**Layer against masses.** The seat layer said 28 mm, while the rig mass of 2.4 kg corresponds to a 20 mm board.

**Masses.** The rig masses (4.5 kg in total) were not computed from the parts. qa-visual found the mass solids sitting on the leg ends instead of at the centres of mass.

**Feet and supports.** `structure.supports` was empty and had no support solids. The foot revolves fell into the kernel trap "revolve-implied-closure" (kernel-traps.md).

**Floor reference.** The `ground` bone had a length of 0.1 m, which made it an eligible member, and the 1:100 cap collars were "ground members".

**Coarse scales.** At 1:200 and 1:500 the legs were 20 mm slender capsules, i.e. the 1:100 positions.

**Thermal.**
- `steel-tube` 0.154 m2 counted only the legs and stretchers.
- `back-beech` 0.242 m2 counted beech posts and finials that do not exist in the real construction.

**2D.** The drawings showed the same wrong construction (floating posts, forward-leaning rear legs, splayed feet).

## 3. Root fixes (catalog file, outside `volumes`)

**Summary and provenance.**

| Member | Old | New |
| --- | --- | --- |
| `summary` | bentwood-style chair on steel legs | bistro café chair: steel tube frame whose rear legs bend into the reclined posts; beech seat on an apron; ladder back (en, de) |
| `provenance.references` | 6 | + `thonet-214`, `fermob-bistro`, `cafe-vii-datasheet` |
| `provenance.notes` | — | construction and tipping pivot (en, de) |

**Dimensions.**

| Member | Old | New |
| --- | --- | --- |
| `dimensions.heightM` | 0.88 | 0.88 (unchanged) |
| `dimensions.widthM` | 0.44 | 0.487 |
| `dimensions.depthM` | 0.50 | 0.44 |
| `dimensions.massKg` | 4.5 | 7.8 (sum of the modelled parts) |

**Materials.**

| Member | Old | New |
| --- | --- | --- |
| `materials` | 2 entries | + `polyamide-glide` (1150 kg/m3, 0.25 W/mK, 1600 J/kgK, ISO 10456), for the glides and caps |

**Rig.** All bones are children of `ground`.

| Bone | Old | New |
| --- | --- | --- |
| `ground` | length 0.1 | length 0 (a reference, not a member) |
| `leg-rear` | (0.05, 0), 88°, 0.46, 0.7 kg | (0, 0), 82.321°, 0.449, 0.9 kg at comX 0.219 |
| `leg-front` | (0.42, 0), 92°, 0.46, 0.7 kg | (0.455, 0), 92.5°, 0.4481, 0.9 kg at comX 0.224 |
| `stretcher` | (0.0542, 0.12), 0.3616, 0.2 kg | (0.0202, 0.15), 0.4282, 0.59 kg |
| `seat` | (0.03, 0.46), 0°, 0.42, 2.4 kg | (0.045, 0.452), 2°, 0.43, 3.36 kg at (0.215, -0.018) |
| `back` | (0.066, 0.46), 95°, 0.42, 0.5 kg | (0.06, 0.445), 98°, 0.433, 2.04 kg at (0.257, -0.003); now the steel post with its board and rail |

**Anchors, poses and supports.**

| Member | Old | New |
| --- | --- | --- |
| `anchors` (four contacts) | z ±0.23 / ±0.245, footprint 0.03 | z ±0.20 (rear) / ±0.21 (front), footprint 0.025 |
| `poses[tilted-on-two-legs].root` | (0.001093, -0.010396, 12°), about the old foot centre | (-0.000273, 0.002599, 12°), about the rear bottom edge of the rear glides; description (en, de) |
| `poses[*].loads[occupant-back]` | x 0.28, y -0.014 | x 0.33, y -0.003 (front face of the backrest board, mid height) |
| `structure.supports` | [] | the four contact anchors |
| `structure.assumptions` | — | rewritten (en, de): one bent tube per side, contacts at the glides |

**Thermal regions.**

| Member | Old | New |
| --- | --- | --- |
| `seat-beech` title | "seat pad and apron" | "seat board and apron frame" |
| `seat-beech` layer | 28 mm, R 0.155556, 27322 J/m2K | 20 mm, R 0.111111, 23040 J/m2K (region heat capacity too) |
| `seat-beech` areaM2 | 0.524 | 0.51 |
| `back-beech` title | "back post, panel and rail" | "backrest board and rail" |
| `back-beech` areaM2 | 0.242 | 0.215 |
| `steel-tube` title | "Steel tubes" | "Steel tube frame" |
| `steel-tube` areaM2 | 0.154 | 0.22 |
| `thermal.assumptions` 2 and 4 | — | 20 mm board, 23 kJ/m2K; 0.22 m2 steel with the posts |

Notes on the area changes:

- `seat-beech` 0.51: hand calculation. The board is 0.168 m2 per face, both faces 0.336, plus 0.032 of edges, plus about 0.16 of rails, minus 0.024 of rail tops under the board. The model measures 0.513.
- `back-beech` 0.215: the posts are steel now. Hand calculation: board 0.139, rail 0.041, edges 0.038, giving 0.218. The model measures 0.2097.
- `steel-tube` 0.22: the posts are now steel. Hand calculation: 2.64 m of Ø22 tube plus 0.86 m of Ø16 gives 0.226. The model measures 0.2236.

All three changes are within the factor 1.5 of C16. They are plain envelope areas (C29, no exchange basis).

**Representations.** All 2D shapes and the representation decisions of 1:50, 1:100, 1:200 and 1:500 were redrawn (E1, section 4).

**Unchanged.** The other poses and clips (they move only the root, so they stay valid), contact fractions, title, symbols, exports and presentation. No other asset references the chair.

**Catalog index.** The entry `🗂️catalog/🔣️.json` still says `widthM` 0.44. I did not change it: the index differs from the assets for 24 of 152 entries and is not maintained by the authors.

## 4. What was modelled (solid counts = listed = expanded; no repeats)

### 1:50 — body 21, thermal 15, structure 44

**Body.**

- **Rear tubes.** Per side, two lofts with explicitly turned circular sections, so the two meet seamlessly in the middle of the bend:
  - the leg part (bone `leg-rear`) runs from inside the glide over the 80 mm bend;
  - the post part (bone `back`) runs on to the top.
  - The tube splays laterally from z 0.185 at the bend to 0.200 at the foot.
- **Front legs and stretchers.** Two straight front-leg lofts, 2 mm into the seat board (z 0.19 at the top, 0.21 at the foot). Two Ø16 stretcher capsules end inside the leg axes, as welded.
- **Glides and caps.** Four polyamide glides (revolves standing on the floor, Ø25 x 10) and two domed post caps (8 mm).
- **Seat.** The board is a plan extrusion of the trapezoid, with front corners in four arc steps and notches round the posts (2 mm clearance at the bend apex). Under it sit the four apron rails: two side rails between the tubes and front and back rails between the side rails.
- **Back.** The curved backrest board and the lower rail are lofts of 11 superelliptic sections along w, with their back faces on the post fronts.

**Thermal.** Every beech and steel solid is referenced into its region with its layer. The sealed tubes are modelled solid: the glides and caps close them, so the bore is not outer surface (C31). The glides and caps are nylon and belong to no region.

**Structure.**
- Members are references to the tubes, the board, the rails, the back board and the rail.
- Joints:
  - the two bends (the frame corner between `leg-rear` and `back`);
  - four stretcher welds;
  - four seat tab fixings at the rail ends;
  - four back screw fixings at the post fronts.
- One glide-shaped support per contact anchor.
- Masses: one at the middle of every tube and post (each pair's common centre lies in the air between the two tubes, so every tube carries its own), one at the combined centre of seat board and aprons (inside the board) and one each at the centres of the back board and the rail.

### 1:100 — body 12, thermal 12, structure 37

**Body.**
- Glides and caps are merged; the tubes stand on the floor with a horizontal end.
- The tubes are slender true-size Ø22 lofts (the bend in two steps), because 1:100 draws them as 0.35 mm strokes (C24).
- Stretchers are Ø16 slender.
- The board has two-step corners and keeps its notches. The apron is one 18 mm ring.
- Back board and rail are curved, with five sections.

**Thermal.** References without layers.

**Structure.**
- Members: tubes, board, ring, back board and rail.
- Joints: bends, welds and seat fixings (the back screws are below the 15 mm feature).
- Supports: 30 mm pads.
- Masses: as at 1:50.

### 1:200 — body 12, thermal 12, structure 33

**Body.**
- The same slender tubes, with a coarser bend.
- A 30 mm trapezoidal board without notches or radii, on a ring with 30 mm walls.
- Flat 30 mm back board and rail in front of the posts (the 26 mm curve is below the feature).

**Structure.**
- Idealised true-size bars per tube member: rear leg from foot to corner, post from corner to top, front leg and stretcher, per side.
- The referenced board, ring, back board and rail.
- Joints: two bends and four seat fixings.
- Supports: 40 mm pads.
- Masses: per tube and at the board centres.

### 1:500 — body 10, thermal 10, structure 26

**Body.**
- The chair sign: a 75 mm seat slab (board and apron merged) and a 75 mm back slab (board and posts merged; the rail is dropped). The slab is shifted so that it does not pass behind the rear glides.
- Slender true-size tubes. The rear tube is straight from the foot to the corner and from the corner to the top.
- Slender stretchers. They are kept because the structure needs a stretcher member inside the body silhouette; they are drawn as a stroke.

**Structure.**
- True-size bars per member.
- The two slabs.
- Joints: four 75 mm seat corners (the bends lie in the seat slab).
- Supports: 75 mm pads.
- Masses: per leg tube, at the seat centre and at the back centre.

`decisions`: two bilingual sentences per scale.

### 2D (ruling E1)

**1:50** (13 / 15 / 18 shapes in elevation / front / plan). Every shape is drawn from the measures of the solids.

- **Elevation:**
  - the raked board over the side rail, with grain;
  - the bent rear tube and post as filled outlines (pen 0.25);
  - glides and caps as details;
  - the front leg under the board;
  - the stretcher;
  - back board and rail edge-on, with the line of the board's end face.
- **Front:**
  - board band and apron;
  - front legs with glides;
  - rear tubes and posts behind them, with caps;
  - backrest board (with grain) and rail across the posts.
- **Plan:**
  - the notched trapezoidal seat with grain;
  - the curved board and the rail band behind it;
  - posts with caps;
  - legs, glides and stretchers where they show past the seat.

**Coarser scales** (8 / 10 / 11 at 1:100, 8 / 10 / 10 at 1:200, 6 / 8 / 8 at 1:500). The same views in the scale's own generalisation, with the tubes as pen strokes along the true centre lines.

## 5. Checks (final, on the catalog asset)

**`volumes-check.ts furniture/cafe-chair --product`:** `PASS (18.4 s)`.

Silhouette IoU (elevation / front / plan) and 95th-percentile boundary distance:

| Scale | IoU | p95 |
| --- | --- | --- |
| 1:50 | 0.994 / 0.964 / 0.995 | 0 / 5.6 / 1.9 mm |
| 1:100 | 0.994 / 0.967 / 0.993 | ≤ 3.8 mm |
| 1:200 | 0.997 / 1.000 / 1.000 | 0 |
| 1:500 | 1.000 / 1.000 / 0.985 | ≤ 19 mm |

Bounds at 1:50: width +0.9 mm, height +0.3 mm, depth +5 mm (the glides against the declared 0.44).

Thermal area against `areaM2`:

| Scale | seat-beech | back-beech | steel-tube |
| --- | --- | --- | --- |
| 1:50 | +0.6 % | -2.5 % | +1.6 % |
| 1:100 | +0.4 % | -4.7 % | +2.5 % |

The totals are +2.2 % at 1:200 and +0.9 % at 1:500. Monotonic, cross-scale, feature, slender and copied all pass.

**`r3-validate.ts furniture/cafe-chair --catalog-only --volumes`:** `VALID ... schema 0, validateAsset 0, checkVolumes 0`. The normal form is VALID as well, and the catalog `volumes` equal `volumes/furniture/cafe-chair.json`.

## 6. What the final renders show (opened with the Read tool)

**Seen:**
- `r3-sheet` body at 1:50 (six views with the rig) and at 1:500;
- 1:50 from below rear (turn 210, tilt -25);
- 1:50 structure (six views);
- 1:50 thermal cut;
- 1:100 structure;
- 1:50 pose `tilted-on-two-legs` (right side);
- `r3-2d` at 1:50 (upright and tilted), 1:100 and 1:500;
- `volumes-render` 1:200 overlay (during the diagnosis).

Most of these were seen on the candidate document, which is identical to the spliced catalog.

**Rendered but not opened:** catalog sheets of body at 1:100 and 1:200, thermal at 1:100, and 2D at 1:200.

**1:50 body, direction by direction:**
- **Front:** a café chair whose two steel posts carry a wide, gently curved backrest board and a lower rail. Under it, the seat board overhangs the apron, and the legs stand slightly splayed on dark glides.
- **Right side:**
  - the rear tube rakes back, bends visibly at the seat and continues reclined to the cap;
  - the backrest board is edge-on in front of the post;
  - the seat rises 2 degrees to the front, over its apron;
  - the front leg is nearly upright, with the stretcher low between the legs.
- **Back:** the posts, the board and rail seen from behind, and the dark rear edge of the seat between the posts.
- **Top:** a trapezoidal seat with rounded front corners, the curved board and rail behind it, and the post caps.
- **Three-quarter:** reads at once as a bistro chair.
- **From below:** the apron frame under the board, the tubes passing the board through its notches, the stretchers and the glides. Nothing floats or pierces anything else.

**Other views:**
- **Thermal cut:** red seat and apron, green board and rail, violet tubes.
- **Structure:** blue members, orange tab fixings, bends and welds, magenta masses at every tube middle and at the board centres, green pads.
- **Tilted 12 degrees:** the chair stands on the rear glides, the front feet are lifted and nothing gapes. In 2D the same pose rotates cleanly.
- **1:500:** a recognisable chair sign of a seat slab and a back slab on thin tubes.

## 7. Still imperfect

- **Seat edges.** The seat board is a straight extrusion. The real 3 mm round-over of the top edges and a slight seat dish are not modelled; both are below the 7.5 mm feature, but a carpenter would see the hard edge in a close render.
- **Back fixings.** Back board and rail touch the post fronts tangentially. The screws and the welded seat tabs are represented only as structural joints.
- **Masses inside the boards.** The back-board and back-rail mass boxes lie in the curved board and show as small magenta patches on its rear face in the structure view.
- **Closed frame in the solver.** The rig is a tree. Each stretcher is welded at both ends but hangs from one parent bone, and the seat ties both leg pairs. The structural solver cannot represent a closed frame (see the bicycle-stand request in `tool-requests.md`), so stretcher and seat-frame forces are only as good as a tree allows.
- **Slender centre line.** The product's slender centre line found a straight chord for one bent tube (IoU 0.78 at 1:200). I split the rear tube at the bend into a leg loft and a post loft at 1:100 to 1:500 and logged the issue in `tool-requests.md` (2026-11-20).
- **Mass.** The model weighs 7.8 kg, against 4.5 kg before. That is real for a solid-beech steel chair, but heavier than a bentwood 214.
- **Catalog index.** The index entry still lists `widthM` 0.44 (see section 3).
- **Not run.** The repo test suites were not run, only the checks of the brief.
