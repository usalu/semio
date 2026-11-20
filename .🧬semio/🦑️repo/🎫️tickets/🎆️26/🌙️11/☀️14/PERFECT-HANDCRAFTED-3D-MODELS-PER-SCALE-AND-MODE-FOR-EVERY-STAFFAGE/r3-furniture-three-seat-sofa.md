# Round 3: furniture/three-seat-sofa

**State:** finished and spliced on 2026-11-20. There was no earlier round-3 work on this asset (no `r3-three-seat-sofa-*` scripts and no report), so the asset was rebuilt from the roots. Everything below describes the catalog file as it is now.

**Scripts** (all in the ticket folder; the other files are named `r3-three-seat-sofa-<part>.py`):

| Script | What it holds |
| --- | --- |
| `r3-three-seat-sofa-kit.py` | The measures, the bone-frame helpers and every station table of the y-lofts. Solids, outlines and sections are all built from these tables. |
| `-body.py` | The body list of every scale. |
| `-lists.py` | Thermal, structure and the volume decisions. |
| `-roots.py` | Rig, anchors, the new thermal region, poses, references, notes, assumptions, summary and symbol. |
| `-draw.py` | The 2D drawings and representation decisions of every scale (ruling E1). |
| `-build.py` | Writes the candidate document and its volumes source. |
| `-apply.py` | Writes the root members into the catalog file in the file's own formatting. `volumes` then goes in through `volumes-insert.py`. |

Re-running build and apply changes nothing (apply reported "members: none"). The scratch candidate was deleted. The renders stay in `🗑️generated/r3-sheet/` and `🗑️generated/r3-2d/`.

## 1. Reference

The subject is a modern upholstered three-seat sofa with box (track) arms, loose seat and back cushions and turned wooden feet.

| Quantity | Reference | Source | Model |
| --- | --- | --- | --- |
| Overall width | 71-86 in (1.80-2.18 m) for a three-seater | Wayfair sofa-dimension guide | 2.00 m (kept) |
| Overall height | 32-36 in (0.81-0.91 m) | Wayfair | 0.85 m (kept) |
| Overall depth | 0.85-0.95 m typical; a comparable listed three-seater is 34 in (0.86 m) | Wayfair, Lowe's listing, my knowledge | 0.90 m (kept) |
| Seat height | 17-19 in (0.43-0.48 m) | Livingetc (designer quote) | 0.43 m at the edge, 0.44 m at the crown |
| Seat depth | 20-25 in (0.51-0.64 m) | Wayfair, Livingetc | 0.57 m from the cushion front to the back cushion |
| Arm height | 25-30 in (0.64-0.78 m) on market sofas; low track arms reach about 0.60-0.65 m | Livingetc, Lowe's / Lulu and Georgia listings | 0.62 m (kept; low modern track arm) |
| Arm width | 0.15-0.25 m for a track arm | my knowledge | 0.20 m |
| Seat cushion | HR foam 0.15-0.20 m in a boxed cover with piped seams | asset layer `foam-seat` 0.20 m, ISO 3386-1 | 0.533 x 0.57 x 0.20 m, 10 mm crown, 8 mm piping |
| Back cushion | 0.12-0.20 m, loose, leaning against the inner back | my knowledge, asset layer `foam-back` 0.15 m | 0.15 m at the base, 0.17 m at the lumbar belly, top at 0.80 m |
| Back frame | timber frame with ply, padded; inner back raked 5-15 degrees | upholstery practice | rear vertical at z 0.45, inner face raked 8 degrees, top rolled at radius 40 mm to 0.85 m |
| Base | hardwood or ply rails, 25 mm, with a deck of springs or webbing and a dust cover below | upholstery practice | plinth 0.10-0.23 m, 18 mm ply over beech rails, 20 mm round front edge |
| Feet | turned hardwood, 0.08-0.15 m | my knowledge | 100 mm; diameter 45 mm at the floor, 60 mm at the top; bead at 70 mm |
| Mass | 45-70 kg for a three-seater | my knowledge | 51.8 kg (kept) |

Sources: https://www.wayfair.ca/sca/ideas-and-advice/guides/sofa-dimensions-how-to-choose-the-right-size-sofa-for-your-home-T5610 , https://livingetc.com/advice/sofa-dimensions , https://www.lowes.com/pd/Naturefalls-84-Inch-Rectangular-3-Seater-Sofa-with-Linen-Beige-Upholstery-and-Two-Pillows/9738250 . The first two are now in `provenance.references` as `wayfair-sofa-dimensions` and `livingetc-sofa-dimensions`.

## 2. Audit of the state before round 3

**Construction.** The old sofa had no back frame. The three back cushions (0.26 m deep, from 0.20 to 0.85 m) were the whole back, so nothing carried them. The back was one flat slab from the rear. QA graded the 1:50 body B.

**Rig.**
- `ground` carried 2.0 m, which made the floor reference a structural member.
- `frame` lay on the floor (y 0), so its structural bar was drawn along the floor.
- The arm bones ran horizontally across the bottom of each arm, 0.2 m long at y 0.1. As a member that is meaningless; the arm is a vertical post and panel.
- The seat and back bones started at x 0.19 with 0.5333 m length, but the solids were 0.5433 m wide. The outer cushions therefore cut 10 mm into the arms.
- The seat bones sat at y 0.22, 20 mm above the cushion underside. That underside lay at 0.20, buried 40 mm deep in a skirt that ended at 0.24.
- The load anchors were off-centre (x 0.27165 on seats 1 and 3) and at z 0, the back edge of the sitting area.

**Anchors.** The foot footprints were 70 x 70 mm, larger than any turned foot.

**2D.**
- The feet were drawn 0.12 m high and overlapped the frame by 20 mm.
- The 1:500 feet were 0.14 m blocks.
- The plan drew the skirt over the seat cushions (z 7 over z 5), which hid the cushions.
- The 1:200 right foot sat at a different x than the 3D foot.
- There was no back frame in any view.

**Solids.**
- 1:50 and 1:100 were the same 11 boxes and extrusions (QA: "100 is a copy of 50").
- The cushions were rounded boxes and the back cushions were one profile extrusion.
- Structure was filled blocks: arm boxes, cushion foam blocks as members, joints floating under the cushions, legs absent (QA grade C at 1:50 to 1:200).

**Thermal.**
- `seat-cushions` 3.6 m2 had been set for three 0.62 x 0.23 m boxes; the honest cushions are 0.57 x 0.20 m.
- `back-cushions` 3.0 m2 had been set for 0.6 x 0.55 x 0.15 m slabs.
- There was no region for the outer back.

## 3. Root fixes (catalog file, outside `volumes`)

| Member | Old | New |
| --- | --- | --- |
| `rig` `ground.lengthM` | 2.0 | 0 (a reference, not a member) |
| `rig` `frame` | y 0, comY 0.3, "Timber frame, skirt and feet" | y 0.165 (mid height of the plinth: the base rails), comY 0.165 (world 0.33), title "Timber frame: plinth, back frame and feet" |
| `rig` `arm-left`, `arm-right` | (0, 0.1) / (1.8, 0.1), 0 degrees, 0.2 m, com (0.1, 0.26) | (0.1, -0.065) / (1.9, -0.065) in the frame, 90 degrees, 0.52 m: upright through the arm from 0.10 to 0.62 m; com (0.25, 0) |
| `rig` `seat-1..3` | x 0.19 / 0.7333 / 1.2667, y 0.22, com (0.2667, 0.1) | x 0.2 / 0.7333 / 1.2667, y 0.065 (world 0.23, the deck), length 0.5333, com (0.2667, 0.105) |
| `rig` `back-1..3` | x 0.19 / ..., y 0.2, com (0.2667, 0.27) | x 0.2 / ..., y 0.065 (deck), com (0.2667, 0.28) |
| Masses | 26 / 3.6 / 3.6 / 2.6 kg | unchanged (authored estimates inside the real range, total 51.8 kg) |
| `anchors` foot contacts | y 0, footprint 0.07 x 0.07 | y -0.165 (same floor point in the moved frame), footprint 0.045 x 0.045 (foot at the floor) |
| `anchors` `seat-load-1..3` | x 0.27165 / 0.2667, y 0.21, z 0 | x 0.2667 (centre), y 0.21 (crown 0.44 m), z -0.2 (sitting point 0.23 m behind the cushion front) |
| `thermal.regions` | 5 regions | + `back-frame` "Upholstered back frame" (layers ply-back 18 mm, batting-back 20 mm, fabric 1 mm; 2.3 m2, the hand sum of the 1.6 x 0.62 m frame: front 0.99, back 0.99, top and ends 0.3, measured 2.43); `base-rail` retitled "Upholstered plinth" |
| `areaM2` `seat-cushions` | 3.6 | 2.8. Hand sum: 3 x 1.05 m2 minus the four touching side faces, 0.46 m2. Measured 2.82. The change is within the factor 1.5. |
| `areaM2` `back-cushions` | 3.0 | 2.5. Hand sum: 3 x 0.97 m2 minus the shared sides, 0.37 m2. Measured 2.50. |
| `poses` (all 7) | 5 regions | + `back-frame`: exposedFraction 1, sunlitFraction 0 |
| `structure.assumptions[0]` | rigid frame, cushions sink behind the skirt rail | the same, now naming base rails through the arm frames, ply deck and back frame bolted to the arm back posts (en, de) |
| `provenance` | 9 references, notes | + 2 references; notes rewritten to the construction (en, de) |
| `summary` | "...padded arms..." | box arms, rolled back frame, plinth (en, de) |
| `symbols[0]` | old boxes, back cushions as the back | plinth, box arms, back frame, cushions, tapered feet; description updated |
| `representations[*].geometry` | 23/12/17, 16/8/12, 9/6/9, 4/4/2 shapes | 17/7/10, 12/7/10, 12/7/10, 6/5/4 (elevation/front/plan at 1:50, 1:100, 1:200, 1:500), ruling E1 |
| `representations[*].decisions` | old texts | rewritten per scale (en, de) |

These were left unchanged: dimensions, poses (except the new region entry), clips, loads, materials, the arm-pads, base-rail and feet areas, and the catalog index entry. The sofa is referenced only by section fixtures and the multiview test. The section stays Section A-A at x = 1.0 m, and the thermal regions stay drawn at every scale.

## 4. What was modelled (solids listed in body / thermal / structure)

All curved parts are y-lofts: superelliptic plan sections stacked bottom to top, with every station stated in the kit.

### 1:50 (15 / 23 / 47)

**Body:**
- Four turned beech feet (revolve, 45 mm to 60 mm diameter, bead at 70 to 78 mm).
- The plinth: an extruded section 0.10 to 0.23 m, 10 mm proud of the cushions, with a 20 mm round front edge and 6 mm eased edges elsewhere.
- Two box arms, 0.20 x 0.90 m. They are lofts with plan exponent 10, 15 mm bottom edges and a 60 mm top roll in four steps.
- The back frame between the arms, with a vertical rear, an inner face raked 8 degrees and a rolled top.
- Three seat cushions on the deck, with rounded edges (elliptic 55 x 40 mm at the top), a 10 mm crown and 8 mm piping rings round the top and bottom boxing seams (`texture`).
- Three pillow back cushions standing on the deck against the frame face. Their front profile is drawn by hand in 13 stations: 0.15 m at the base, a lumbar belly, and a pillow top at 0.80 m.

**Thermal.** Fabric is the body surface. Inside it:
- the foam cores of the cushions, 1 mm in;
- the arm batting;
- batting (1 mm in) and ply (21 mm in) in the back frame;
- the ply core of the plinth;
- the feet in beech.

**Structure:**
- The feet as posts (`of`).
- Front and back beech base rails, 25 x 90 mm, through the arms; the 18 mm ply deck.
- In the back frame: the bottom rail, four uprights and the top rail.
- Per arm: two posts, top and bottom rail, and the inner ply panel.
- Per cushion: a 30 x 150 mm foam bearing band along the bone. The seat bands lie under the sitting line; the back bands lie at lumbar height on the frame face.
- Joints: corner blocks of the base rails in the arm frames, the back-frame bolts into the arm back posts, and the contact pads of the cushions.
- Supports: four felt glides.
- Masses: nine, one per bone.

### 1:100 (12 / 12 / 43)

The same masses in fewer stations:
- arm roll and cushion edges in two to three steps, back cushions in seven stations;
- tapered feet without the bead;
- no piping.

Thermal is references only, with the layers merged. Structure: feet, rails, deck, back rail, two end uprights, top rail, arm posts and rails, bearing bands, and joints at 15 mm.

### 1:200 (12 / 12 / 28)

- Square plinth with a 30 mm round front edge.
- Straight round feet (50 to 60 mm).
- Arm rolls, cushion tops and the back-frame top as one chamfer step; back cushions in four stations.

Structure: feet, rails, a 30 mm deck, one back panel, one 30 mm panel per arm, bearing bands, arm corners, glides and masses.

### 1:500 (6 / 8 / 25)

Body:
- one seat block (plinth and seat cushions, 0.10 to 0.43 m);
- one back block (raked face, 0.85 m);
- two arm blocks;
- four 75 mm feet.

Thermal splits the blocks back into plinth, seat cushions, back cushions and back frame with its own solids.

Structure: feet, two 75 mm base rails, a 75 mm back panel, a 75 mm panel per arm, 75 mm bearing bands, glides and masses.

### Decisions

Two bilingual sentences per scale (body; thermal and structure).

## 5. Checks (final, on the catalog asset)

**`volumes-check.ts furniture/three-seat-sofa --product`:** `PASS (12.7 s)`.

Silhouette IoU (elevation / front / plan) and p95 boundary distance:

| Scale | Elevation | Front | Plan | p95 |
| --- | --- | --- | --- | --- |
| 1:50 | 0.9995 | 1.0000 | 0.9996 | 0 to 1.9 mm |
| 1:100 | 1.0000 | 1.0000 | 0.9996 | 0 |
| 1:200 | 1.0000 | 1.0000 | 0.9997 | 0 |
| 1:500 | 1.000 | 1.000 | 1.000 | 0 |

Thermal area deviation at 1:50:

| Region | Deviation |
| --- | --- |
| seat-cushions | 0.009 |
| back-cushions | 0.001 |
| arm-pads | 0.078 |
| back-frame | 0.054 |
| base-rail | 0.050 |
| feet | 0.073 |

At 1:100 every region is under 0.09. The total deviation is 0.042 at 1:200 and 0.013 at 1:500. Cross-scale, monotonic, copied, feature, bounds, bones and structure coverage are all ok.

**`r3-validate.ts furniture/three-seat-sofa --catalog-only --volumes`:** `VALID ... schema 0, validateAsset 0, checkVolumes 0`. The catalog `volumes` equal `volumes/furniture/three-seat-sofa.json`.

**Tool request filed:** `tool-requests.md` "C31 (outer = reachable from outside) is not in the product's outer area". A hollow plinth measured +82 % because its closed cavity was counted, so the plinth body is modelled solid.

## 6. What the final renders show (opened with the Read tool)

**Seen:**
- `r3-sheet` body at 1:50 (six views with the rig, and an enlarged three-quarter view at turn 35, tilt 20), 1:100, 1:200 and 1:500;
- body at 1:50 in the pose `seat-proof-load`;
- structure at 1:50 (with and without the rig), 1:100 and 1:500;
- thermal cut at 1:50;
- `r3-2d` at 1:50, 1:200 and 1:500;
- `r3-2d` at 1:50 in `seat-proof-load` (light-solid).

**Rendered but not opened:** the 1:100 2D sheet, 1:200 structure, and thermal at 1:100 to 1:500.

What the views show:
- **Front (from +x, the right end):** box arm with its 60 mm roll. Behind it rise the pillow top of the back cushion and the raked back frame with its rolled top. The feet are tapered.
- **Right side (product elevation, from the front):** two arms with soft tops, the plinth with its round front edge, and three crowned seat cushions with piping lines at the top and bottom seams. Above them are three pillow back cushions with V seams between them, under the 50 mm band of the back-frame roll. Feet stand under the arms.
- **Back:** a flat upholstered outer back above the plinth between the arms. This is true to such sofas.
- **Top:** arms, the back roll, the back cushions and three seat cushions with rounded corners.
- **Three-quarter:** reads at once as a real modern track-arm sofa.
- **`seat-proof-load`:** the middle seat drops 90 mm behind the plinth face and its back cushion 30 mm, with no visible interpenetration. In 2D the plinth is drawn over the sunk cushion in elevation and section, as the 3D shows it.
- **Thermal cut:** red foam cores in pink fabric, green back cores, violet arm batting, the three back-frame layers in olive and yellow, the cyan plinth core, magenta feet.
- **Structure:** the frame is legible as rails through arm frames (posts, rails, inner panel), with the deck, the back-frame uprights and top rail, the cushion bearing bands with their pads, glides under the feet, and masses.
- **1:100 to 1:500:** the same sofa simplified step by step. 1:500 is seat block, back block, arms and feet, recognisable at 4 mm.
- **2D:** clean elevations, Section A-A and plans at every scale. The plan no longer hides the cushions.

## 7. Still imperfect

- **Plinth cavity.** The plinth is a closed solid in the body, because the product counts the faces of a closed cavity as outer area (C31 not implemented; tool request filed). The real cavity, deck and dust cover exist only in `structure` as rails and deck.
- **Masses.** The bone masses are the asset's authored estimates; only their centres were moved to the modelled parts. A full mass take-off of a ply-and-timber frame would likely give a frame mass nearer 30 kg.
- **Rigid cushions.** Cushions are rigid bodies that sink vertically, as the rig declares. Sinking parts move behind the plinth face and into the deck, and the foam does not deform. A sinking back cushion opens a gap of about 2 to 4 mm to the raked frame face.
- **Simplifications.** The arm front is plain: there is no separate front panel or welt. Back cushions have no piping. Zips, staples and spring wires are omitted.
- **1:200 seat tops.** The single chamfer step shades slightly faceted in the renderer.
- **Not run.** The repo test suites were not run, only the two checks of the brief.
