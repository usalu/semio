# Round 3 — objects/charcoal-grill

State: finished and spliced (2026-11-20). One author, started from the committed state (no earlier round-3 work existed for this asset).

Scripts in the ticket folder:
- `r3-charcoal-grill-kit.py`: the real geometry (sphere zones of bowl and lid, leg line, sockets, wheel, pan, damper, dial) and the solid writers. It includes a twist-free `flush_loft` that cuts rod and sleeve ends in the tangent plane of the shell or the ground.
- `-s50.py`, `-s100.py`, `-s200.py`, `-s500.py`: every solid of every scale and list, stated by hand.
- `-roots.py`: references, notes, dimensions, rig, anchors, poses, clips, supports and areas.
- `-draw.py`: the 2D drawings of every scale and view, drawn from the same geometry, plus the representation decisions.
- `-areas.json`: the two corrected `areaM2` values.
- `-build.py`: builds the candidate, writes the source with `--source`, and writes the catalog members with `--apply` (plain layout check, then a member-by-member check that nothing else changed).
- `-sheet.ts`: a copy of `r3-sheet.ts` with `--file`, used to look at the candidate before the rig was in the catalog.

## 1. Reference

The subject is a 57 cm charcoal kettle of the Weber Original Kettle class: the classic tripod kettle with wooden handles, a top vent dial and a bottom damper.

| Quantity | Reference | Source | Model |
|---|---|---|---|
| Height, lid closed | 100.3 cm (Original Kettle 57 cm). The E-5710, E-5730 and Premium are 106–107 cm because their legs are 5 cm longer | weber.com product pages (AE, PH, IE, GB), retrieved 2026-11-19 | 1.00 m (top of the vent fin) |
| Bowl diameter | 57.2 cm (22.5 in), cooking grate 54.6 cm (21.5 in) | weber.com; class data | Rim 0.585 m over the bead; grate ring 0.546 m |
| Depth with lid handle | 63.5 cm | weber.com | 0.64 m in x (the asset's `widthM` axis) |
| Width over the handles | No single value: the classic wooden-handle kettle has two side bowl handles | Authored | 0.67 m in z (the asset's `depthM` axis) |
| Mass | 14.5–15.6 kg (standard 57 cm). E-5710: 16.3 kg | weber.com | Grill 14.5 kg, plus 1.5 kg charcoal = 16.0 kg |
| Legs | Three aluminium legs: one on a foot, two carrying the wheel axle. Triangle leg support. Leg about 19.5 in (0.50 m) | weber.com parts 65130 and 80695 | Tripod of 25 mm tubes, 0.52 m long, splayed 10.8 degrees. Feet at r 0.30 m (front, az 0°) and wheel legs at az ±120° |
| Wheels | 6 in (152 mm), two, with hub caps | Weber part 65930 | 152 × 38 mm with hub caps, on a 10 mm axle |
| Bowl / lid form | Bowl about a hemisphere, wall nearly vertical at the rim. Lid a slightly flatter dome. Rolled rims | Class knowledge (photos) | Bowl: sphere r 0.2904 m, 257 mm deep. Lid: sphere r 0.2945 m, 236 mm high. 8.4 mm beads |
| Lid vent | Aluminium damper of about 80 mm on top of the lid | weber.com ("aluminium dampers"); area-review row (80 × 35 mm dial) | 80 mm dial plate, 3 × 12 mm holes, grip fin, dished seat. Top 18 mm above the dome |
| Bottom damper | One-Touch style: plate under the bowl with a lever | weber.com | Dished 120 mm plate hugging the bowl, 8 × 13 mm lever with knob |
| Fuel grate, charcoal | Charcoal grate about 7 cm above the bottom | Class knowledge | Fuel grate ring 0.417 m at y 0.575 on three rests. Mound 0.39 × 0.04 m with briquettes |
| Wall | Enamelled steel about 0.8 mm (the asset's layer) | Asset `thermal.layers` | 0.8 mm shells at every scale (C30) |

Sources:
- [Weber Original Kettle 57 cm (AE)](https://weber.com/AE/en/grills/charcoal-grills/original-kettle-series/1341504.html)
- [Weber Original Kettle 57 cm (PH)](https://www.weber.com/PH/en/charcoal/original-kettle/original-kettle-charcoal-grill-57cm-with-thermometer/1341308.html)
- [Weber E-5710 (IE)](https://www.weber.com/IE/en/barbecues/charcoal-barbecues/original-kettle-series-/14101004.html)
- [Weber Original Kettle Premium (GB)](https://www.weber.com/GB/en/charcoal-barbecues/original-kettle/1502210.html)
- [Weber wheel 65930](https://www.weber.com/US/en/parts--et--support/replacement-parts/charcoal-parts/wheels/65930.html)
- [Weber triangle leg support 80695](https://www.weber.com/US/en/parts/charcoal-parts/leg-parts/80695.html)
- [Weber leg parts 65130](https://www.weber.com/US/en/parts/charcoal-parts/leg-parts/65130.html)

The product data are added to the asset as the reference `weber-original-kettle-57`.

## 2. Audit of the committed state

**Rig and supports**
- The summary says three legs, but the model had **four legs and four ground contacts** (two rear feet at x −0.275 and two wheels at x +0.27, z ±0.14). This is a square table, not a tripod.
- Wheels were 100 mm (real: 152 mm) and sat under the lid handle side. On the real kettle the wheels are at the back, so lifting the front tilts the grill onto them.
- `lid-vent` and `ash-damper` were posed with `angleDeg`. In `embers-vents-low` the dial tilted 20 degrees off the lid and the damper rod swung 70 degrees out of the bowl. Both really turn about the vertical.
- `grate` and `charcoal` bones started at the bowl axis and ran 0.39 m forward, so the rig line stuck out of the bowl.
- Masses: the bowl was 8 kg at comY 0.42, which sits under the bowl in the air. The grill totalled 12.5 kg without charcoal; the reference is 14.5 kg.

**Dimensions**
- Height was 0.97 m; the reference is 1.003 m.
- The lid dome was only 200 mm high (205 mm to the dial) and the bowl 240 mm deep. The closed kettle measured 0.445 m high on a 0.585 m diameter (ratio 0.76); the real kettle is about 0.85, so it read as a flattened ball.

**2D**
- Four legs in the front view and two foot dots in the plan.
- The wheel was in front and only 100 mm.
- The elevation drew the vent holes, which are on the top face and cannot be seen from the side.
- The 1:500 legs were filled 75 mm bars. QA reported this as a red halo.

**Solids (QA grades B/B/B/C)**
- Bowl and lid showed gore facets.
- The handles were boxes.
- Structure joints were missing at the leg brackets.
- The ring mass sat outside the bowl.
- The 1:500 legs and supports were wrong.
- Thermal and structure lists reused the four-leg layout.

## 3. Root fixes (catalog, outside `volumes`)

| Member | Old | New |
|---|---|---|
| `dimensions` | h 0.97, w 0.67, d 0.64, 14.0 kg | h 1.00, w 0.64 (x, front to back, lid handle), d 0.67 (z, over the side handles), 16.0 kg. The validator measures `widthM` along x: the old values were swapped |
| `rig.bowl` | 8 kg, comX 0, comY 0.42 | 10.0 kg, comX −0.017, comY 0.485, reference weber-original-kettle-57 |
| `rig.grate` | x 0, length 0.39 | x −0.273, length 0.546 (spans the grate), comX 0.273 |
| `rig.charcoal` | x 0, length 0.39 | x −0.195, length 0.39 (spans the mound), comX 0.195, comY 0.018 |
| `rig.lid` | comX 0.2925, comY 0.07 | comX 0.331, comY 0.11 (dome plus handle) |
| `rig.lid-vent` | y 0.20, length 0.07, revolute 0–90° tilt | y 0.242, length 0.018, revolute with tilt locked (0/0), twist 0–90° |
| `rig.ash-damper` | y 0.495, angle 0, length 0.1, revolute 0–90° tilt | y 0.4800 (bowl pole), angle −90 (pointing down), length 0.0045, twist 0–90° |
| `anchors` | leg-rear-left/right (x −0.275, z ±0.145), wheel-left/right (x 0.27, z ±0.14) | leg-front (x 0.30), wheel-left/right (x −0.1428, z ±0.282, 30 × 30 mm patches), grate-point x 0.273, vent-top x 0.018 |
| `poses` | four contacts; `embers-vents-low` used `angleDeg` 70 on the vent and the damper | three contacts; `twistDeg` 70 |
| `clips` | four contact tracks | three contact tracks |
| `structure` | four supports, assumptions for four contacts | three supports, new assumptions (bilingual) |
| `thermal.regions` | lid-shell 0.40 m², ash-catcher 0.14 m², legs-steel title | lid-shell 0.447 m² (taller real lid, measured total outer area), ash-catcher 0.197 m² (open 300 mm pan, inside and outside, C29), legs-steel title now lists sockets, triangle, axle and wheels |
| `provenance` | — | Reference weber-original-kettle-57 added; notes rewritten (en, de) |
| `representations` | old drawings | All four scales and three views redrawn (see section 4); decisions rewritten (en, de) |

Both area corrections are within the factor 1.5 of C16, so `area-review.md` gets no new row. The 0.14 m² of `area-decisions.md` counted only the underside and rim of a closed tray. The other regions stay within tolerance and are unchanged.

## 4. Per scale and list

### 1:50 — body 97, thermal 67, structure 72

**Body**
- Bowl: 0.8 mm sphere-zone shell. Rolled bead as a torus. Three grate rests and three fuel-grate rests riveted inside, with their faces on the inner wall.
- Side handles: two wooden lofts (superelliptic, barrelled), each on two Ø10 rod brackets cut flush in the shell's tangent plane.
- Ash pan: 2 mm aluminium pan with a rolled lip, hung 65 mm below the bowl on three Ø8 hangers.
- Legs: three Ø25 legs in sockets. Each socket's top is cut in the bowl's tangent plane, so nothing enters the shell. The front leg stands on a foot with a level sole.
- Wheels: two 152 mm wheels with dished faces and hub caps, on a Ø10 axle. Triangle of Ø8 wire at 240 mm.
- Bottom damper: dished plate following the bowl, 8 × 13 mm lever with knob.
- Charcoal: mound with 14 briquettes (texture). Fuel grate: ring and nine bars.
- Cooking grate: Ø8 ring, 27 bars of Ø6 at 19.5 mm pitch, two cross bars.
- Lid: shell with bead, wooden grip on two flush Ø10 rods.
- Vent: dial plate with three Ø12 holes, a dished seat that closes it onto the dome, grip fin.

**Thermal**
- Bowl and lid as outer envelopes, with the rim faces as section faces (the pattern accepted in `area-decisions.md`).
- Every other region references its body parts.
- Cooking grate (hidden): its own rod copies (B4). Charcoal bed (hidden): its own lens.
- Emitters: a Ø50 flue patch on the dial and a Ø400 mouth disc.

**Structure**
- Members: bowl shell, three legs, triangle, axle, the cooking-grate rods and the fuel-grate rods.
- Joints: three socket joints, wheel bearings, grate and fuel rests, lid hinge pin at the rear rim, vent and damper pivots.
- Supports: three pads at the anchors.
- Masses: four, at the centres of mass.

### 1:100 — body 54, thermal 59, structure 52

**Body**
- Shells meet at the 742 mm rim; the beads are merged into them.
- Grips have three sections; brackets are thickened to 15 mm.
- Vent dial is one dished cap (holes and fin dropped).
- Legs run flush into the bowl, with foot and socket merged.
- Wheels have no hub caps.
- Cooking grate: 13 bars at 39 mm pitch. Fuel grate: ring and five bars.
- The damper is not drawn and not in the body.

**Thermal**
- The hidden grate keeps all 27 real rods.
- The damper exists only as a thermal stand-in, 15.5 mm thick at the feature, so the vents region keeps its area.

**Structure:** as at 1:50, with ellipsoid leg joints.

### 1:200 — body 26, thermal 28, structure 32

**Body**
- Shells, 30 mm wooden blocks reaching the shells, dial cap.
- Pan raised against the bowl: the 65 mm gap would print as a sliver below the band's minimum gap.
- Legs are slender Ø25 flush tubes, drawn as strokes. Wheels 152 × 38 mm, axle and triangle slender.
- Grate: ring and seven bars. Fuel ring. Charcoal mound.

**Thermal:** damper as a stand-in; only the area total is compared (+9.6 %).

**Structure:** members, joints, 30 mm supports and masses.

### 1:500 — body 11, thermal 16, structure 14

**Body**
- Shells and a 75 mm lid handle block.
- Pan merged with its gap into one 75 mm mass.
- Slender legs, drawn as strokes.
- Wheels 152 mm in diameter, widened outward to 75 mm so that the drawing keeps the 0.67 m width once the 30 mm side handles drop out.
- Charcoal and fuel grate as one mass lying in the bowl; grate ring.

**Thermal**
- Side handles, vent and damper are thermal-only 75 mm solids.
- The 2D lid shape carries the regions lid-shell and vents-alu.
- Area total +1 %.

**Structure:** bowl shell, legs and axle, 75 mm supports, three masses.

Decisions are written per scale in English and German.

### Final check line

`volumes-check.ts objects/charcoal-grill --product`: **PASS (79.6 s)**

Silhouette IoU, elevation / front / plan:

| Scale | Elevation | Front | Plan |
|---|---|---|---|
| 1:50 | 0.995 | 0.995 | 0.995 |
| 1:100 | 0.994 | 0.990 | 0.996 |
| 1:200 | 0.995 | 0.994 | 0.996 |
| 1:500 | 0.997 | 0.996 | 0.913 |

Area deviation at 1:50:

| Region | Deviation |
|---|---|
| bowl-shell | 0.008 |
| lid-shell | 0.001 |
| legs-steel | 0.033 |
| ash-catcher | 0.001 |
| vents-alu | 0.095 |
| handle-wood | 0.113 |
| cooking-grate | 0.033 |
| charcoal-bed | 0.029 |

`r3-validate.ts objects/charcoal-grill --catalog-only --volumes`: `VALID … schema 0, validateAsset 0, checkVolumes 0`. The catalog `volumes` are spliced with `volumes-insert.py` and equal the source.

## 5. What the final renders show

I opened every image below with the Read tool:
- `r3-sheet` body sheets at 1:50, 1:100, 1:200 and 1:500 (six views each).
- 1:50 three-quarter views: from above, from below (turn 20, tilt −15), lid open from two sides, `embers-vents-low` from below, lid-open top view.
- Thermal cut at 1:50 and 1:100; structure at 1:50 and 1:100.
- `volumes-render` overlay sheets at all scales.
- `r3-2d` at 1:50, 1:100 and 1:500, and at 1:50 in `embers-vents-low`.

**Body by view**
- **Front (from +x):** lid handle centred on the dome above the rim, front leg on its foot in front of the pan, rear legs behind, both wheels outside the rear legs on the axle, triangle, side handles on their rods.
- **Right side:** near-spherical kettle (0.50 m on 0.585 m), lid handle at the front, wheel at the rear under the bowl, front leg to the foot, pan hanging free under the damper.
- **Back and left:** mirror images, with nothing floating or poking through.
- **Top:** round bowl and lid with the dial, fin and three holes, handles with visible rod brackets, wheels peeking out at the rear quarter, foot ahead of the rim.
- **Lid open (lit-lid-open):** clean lid interior with no dial poking through, 27-bar grate over the briquette mound, fuel grate with its rests, rim bead.
- **From below:** dished damper and lever, sockets sitting flush on the shell, pan on three hangers, triangle on the legs.
- **`embers-vents-low`:** the dial fin and the damper lever turn 70 degrees about the vertical (3D and 2D plan both show it); nothing tilts.

**Thermal:** nested reading of the bowl and lid envelopes, the hidden grate (green hatched rods) and the charcoal (blue) inside, pan and vents in place.

**Structure:**
- The bowl and lid shells are the blue members; they really carry the load.
- Legs, triangle and axle are members, joints are orange at the sockets and bearings, supports are green pads at the foot and wheels.
- Masses sit inside the bowl, the lid and the charcoal.

**Coarser scales:** 1:100 and 1:200 still read as the same kettle at a glance; 1:500 reads as a kettle on three legs with two wheels.

## 6. Still imperfect

- **Faceting in the renders.** The wheels (about 14 facets), the bead tori and the bowl rim seen from the top are visibly polygonal. Tessellation segments come from the renderer's or kernel's quality setting, and the schema gives a solid no segment count, so I cannot fix it in the asset. It is a request for the tool owners and was noted before in `qa-visual.md`.
- **Wooden handles and the rear hinge.** The asset keeps wooden handles (classic kettles; current Weber uses glass-filled nylon) and the rear-rim hinge of the rig (the real lid lifts off a hook). Both are kept deliberately, as before.
- **1:500 generalisations.** The pan is a 75 mm mass, the wheels are 75 mm wide, and the support pads and masses are 75 mm cubes and spheres (the feature rule). The side handles and dial drop out, so the drawn height is 0.98 m (the bounds and extent rules pass).
- **1:100 and 1:200 stand-ins.** The bottom damper exists only as a thermal stand-in, thickened to the feature (15.5 and 30.5 mm); the 1:200 pan is raised against the bowl and its handles are closed blocks. These follow the print gap rule of each band.
- **Structure shells.** The bowl and lid shells are referenced as members (D3 allows it because they are the real load-bearing shells), so the structure view is dominated by two blue shells.
- **Not modelled:** the One-Touch blades inside the bowl and the lid hook.

## Round 5 handcraft (2026-11-21)

Script: `T/r5-charcoal-grill-build.py` (loads the round-3 builder and the per-scale scripts, states the round-5 change by hand, writes the volumes source with `--source`); renders with `T/r5-objects-render.sh`, montages with `T/r5-objects-montage.ts` under `🗑️generated/r5-charcoal-grill/`.

### Reference (modelled to): the table of section 1 stands (Weber Original Kettle 57 cm: H 100.3 cm, bowl Ø 57.2 cm, grate Ø 54.6 cm, depth 63.5 cm, 14.5–15.6 kg, three legs ≈ 0.50 m, wheels 6 in = 152 mm, aluminium lid vent ≈ 80 mm).

### Model vs reference (1:50, measured on the sheets and solids)

| quantity | model | reference | dev. |
|---|---|---|---|
| height, lid closed | 1.000 m | 1.003 m | −0.3 % |
| bowl rim Ø (over the bead) | 0.585 m | 0.572 m bowl + bead | +2.3 % (bead) |
| cooking grate Ø | 0.546 m | 0.546 m | 0 % |
| depth incl. lid handle | 0.640 m | 0.635 m | +0.8 % |
| wheel Ø | 0.152 m | 0.152 m | 0 % |
| leg length | 0.52 m | ≈ 0.50 m (19.5 in) | +4 % (leg into socket, visible length 0.50) |
| rim height (sheet, front view) | 0.74 m | ≈ 0.73–0.74 m (class) | ≈ 0 % |
| mass | 14.5 kg + 1.5 kg charcoal | 14.5–15.6 kg | 0 % |

### Grades (sheets opened: body, structure, thermal --cut at 1:50/100/200/500, 2D at all four scales, poses cold-closed, grilling-closed, lit-lid-open, embers-vents-low, clips light-and-close t 720 s, lid-lift-cycle t 8 s, 1:500 right side alone)

| scale | body | thermal | structure | 2D |
|---|---|---|---|---|
| 1:50 | A → A | A → A | A → A | A → A |
| 1:100 | A → A | A → A | A → A | A → A |
| 1:200 | A− → A− | A → A | A → A | A → A |
| 1:500 | A− (INVALID) → A− | A → A | A → A | A → A |

Defects seen and fixed:
- Catalog INVALID before: `volumes-slender` 1:500 `leg-left`, `leg-right` 19.7 × (< 20): the wheel legs stopped at the axle centre (0.076 m). They now run to their real lower end 14 mm below the axle (0.062 m, as at 1:50), 20.2 ×.
- Poses and clip frames: lid opens on the rear hinge without passing through the bowl, vent fin and damper lever turn 70° in `embers-vents-low`, everything closed.

Checks: `r3-validate.ts objects/charcoal-grill --volumes` VALID (schema 0, validateAsset 0, checkVolumes 0); `volumes-check.ts objects/charcoal-grill --product` PASS (599.7 s); spliced with `volumes-insert.py`.

Remaining (A−, with reason): at 1:200 and 1:500 the 30 mm ash pan and its 65 mm hanging gap are one mass hugging the bowl (a drum under the bowl). The gap cannot be drawn: with the body pen it prints below the 0.15 mm gap of the band at its range end 1:350 / 1:1000, and the 3D must agree with the drawing. Lid and bowl silhouettes at 1:500 show the coarse tessellation of the renderer at that scale (profile chords 0.7 mm, not a model facet).

Across the four scales, the ash pan lies 385–418 mm above ground and the bulk coal bed lies at 578–618 mm. Ash passes through three floor openings. The three hood flues coincide with the damper skirt and dial. All 144 shutter rays had the expected open or blocked path. The current GLB passed all twelve scenes in 4,131,984 bytes. All 984 print checks and all four volume checks passed.
