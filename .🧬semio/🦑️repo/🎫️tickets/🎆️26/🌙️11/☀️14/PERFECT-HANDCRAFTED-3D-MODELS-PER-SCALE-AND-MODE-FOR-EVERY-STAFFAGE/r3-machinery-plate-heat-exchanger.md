# Round 3: machinery/plate-heat-exchanger

State: finished and spliced (2026-11-20). No earlier round-3 work existed for this asset, so it was rebuilt from the roots. Everything below describes the catalog file as it is now.

## Final state

- `volumes-check.ts machinery/plate-heat-exchanger --product`: **PASS** (4.8 s) at 1:50, 1:100 and 1:200.
- `r3-validate.ts machinery/plate-heat-exchanger --catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`.
- Print suitability on the office laser profile: suitable at all three scales.

## Scripts

All scripts are in the ticket folder.

| Script | What it holds |
|---|---|
| `r3-plate-heat-exchanger-geo.py` | Measures, bone frames, outlines (diamond, cradle, top rail, side rails, flap profile) |
| `r3-plate-heat-exchanger-volumes.py` | Every solid of every scale and list, stated by hand |
| `r3-plate-heat-exchanger-roots.py` | Rig, anchors, emitter titles, texts, areas, and the 2D of 1:50, 1:100 and 1:200 |
| `r3-plate-heat-exchanger-build.py` | Writes the candidate `🗑️generated/r3-plate-heat-exchanger/🔣️.json` and the source `volumes/machinery/plate-heat-exchanger.json` |
| `r3-plate-heat-exchanger-apply.py` | Writes the changed root members into the catalog file, keeping its formatting |
| `r3-plate-heat-exchanger-suit.ts` | Print-suitability probe |

To resume:
1. Run the build script.
2. Run `r3-validate.ts --file <candidate>`.
3. Run the apply script.
4. Run `volumes-insert.py`.

## 1. Reference

The subject is the heat-recovery section of a compact air handler of the 1000 m3/h class: a cross-flow plate core in a sheet-steel casing with four duct spigots and a bypass flap.

| Quantity | Real value / practice | Source | Model |
|---|---|---|---|
| Core type | Cross-flow plate pack of square aluminium plates. In compact units the core stands on one corner (diamond), so its four flow faces face four chambers. Both streams cross diagonally: outdoor upper left to supply lower right, extract upper right to exhaust lower left. | Product knowledge (Zehnder, Helios, Recair type layouts); EN 13141-7 unit layouts; VDI 3803-1 | Diamond core, square plates of 0.318 m (half-diagonal 0.225 m), centre 0.315 m above the floor |
| Plates | 0.3 mm embossed aluminium at 2.5 mm pitch; stack length = inner casing depth | Asset provenance; EN 308 class | 0.485 m stack, about 192 plates; plate mass 192 × 0.101 m2 × 0.3 mm × 2700 kg/m3 ≈ 15.7 kg against 12 kg declared (kept; the core is slightly generous) |
| Face velocity | 1.5 to 3 m/s design | ASHRAE Fundamentals | 0.278 m3/s over 0.318 × 0.485 m = 1.8 m/s |
| Core holding | Extruded corner profiles at the four corners, end plates; corners seated and sealed in rails that separate the chambers | Product knowledge | 30 mm corner profiles, 7.5 mm end plates, V cradle, top rail, two side seal rails |
| Spigots | Circular DN 160 to 250 for this class; circular ducts per EN 1506; the emitters already declared 0.2 m diameter | EN 1506; asset `thermal.emitters[].diameterM` | DN 200, 1 mm galvanised wall, 60 mm proud, 15 mm flange |
| Casing | Galvanised sheet of 0.8 to 1.25 mm; removable inspection cover; condensate drain from the cold (exhaust) side; base rails or feet | VDI 3803-1; EN 13141-7 (condensate) | 1 mm sheet (7.5/15/30 mm at the scale floors), front panel = cover with four quarter-turn fasteners, Ø 22 mm drain under the exhaust chamber, two 40 × 30 mm U-channel rails |
| Bypass | A flap that lets air pass the core in summer | Product knowledge | Kept as the asset's 0.4 m flap on a 20 mm hinge barrel over a 0.36 m opening in the lid |
| Envelope | 0.5 × 0.6 × 0.5 m, 18 kg | Asset `dimensions` | Unchanged: x ±0.31 with spigots, height 0.62 with the hinge, z ±0.25 |

## 2. Audit of the state before round 3

**Rig**
- `frame` started at the base centre and ran 0.6 m along +x, so 0.35 m of it lay outside the casing.
- `ports` ran 0.62 m from x = 0 at a height of 0.3 m, so half of it stuck out to the right.
- `plates` was a horizontal 0.54 m bone on the floor at y = 0.03.
- The damper hinge axis sat at 0.62 m, above the flap it carries.
- The masses were at `comY` 0.27 and 0.3 (frame), not at the core and spigot centres.

**Construction**
- The core was drawn and modelled as horizontal plates stacked vertically (six repeated boxes), 0.44 × 0.4 m.
- Those plates cannot be cross-flow with four spigots on two end walls.
- The anchors made the supply leave upper right and the exhaust leave lower left. Both outlets lie on the same core diagonal, which is impossible in a 4-port exchanger.

**Spigots**
- They were rectangular 0.30 × 0.15 m frames.
- The emitters declare a diameter of 0.2 m.
- qa-visual-2 asked for round stubs with a flange.

**Casing and 2D**
- The casing was posts plus 30 mm end walls with open sides, and the plate pack was visible.
- The 2D elevation drew a rectangle "pack" over the frame.
- The front drew the pack hatched under spigots standing in front of it, which is not one coherent view.
- The plan drew the pack 0.4 × 0.4 m under the flap.

**Lists**
- `structure` at every scale was references to the whole body (the D3 defect; qa grade B-).

**Thermal**
- `frame` would have measured inner faces.
- `ports` bores counted, which area-decisions line 955 flagged.

## 3. Root fixes (catalog file outside `volumes`)

| Member | Old | New |
|---|---|---|
| `rig.frame` | (0, 0), length 0.6, com (0, 0.3) | (−0.25, 0) base edge, length 0.5, com (0.25, 0.3) |
| `rig.plates` | (0, 0.03), length 0.54, com (0, 0.27) | (0.025, 0.315) = left core corner, length 0.45 (horizontal core diagonal), com (0.225, 0) |
| `rig.ports` | (0, 0.3), length 0.62, com (0, 0) | (−0.06, 0.315) = left spigot mouth, length 0.62 (mouth to mouth), com (0.31, 0) |
| `rig.damper` | (−0.2, 0.62) in a frame at the origin | Hinge axis at world (−0.2, 0.61), i.e. (0.05, 0.61) in the new frame; com (0.2, −0.006) |
| Bone titles | Frame / pack / collars | Say where the bone runs |
| `anchors.base-contact` | (0, 0) | (0.25, 0), the same world point |
| `anchors.supply-collar` | Upper right (0.28, 0.175) | Lower right, at the mouth (0.62, −0.155) = world (0.31, 0.16); title "lower-right spigot" |
| `anchors.exhaust-collar` | (−0.28, −0.165) | At the mouth (0, −0.155) = world (−0.31, 0.16) |
| `thermal.emitters[].title` | Upper-right / lower-left collar | Lower-right / lower-left spigot |
| `thermal.regions.frame.areaM2` | 2.128 | **1.54** (factor 1.38, within C16). Outer area of the closed casing: panels 1.38 net of spigot holes and opening, plus base rails 0.19, minus rail contact 0.04 = 1.53 by hand; 1.542 measured |
| `thermal.regions.ports.areaM2` | 0.22 | **0.25** (factor 1.14). Per spigot: tube outside 0.033, flange both faces 0.020, rim 0.005, tube inside the wall hole 0.005, about 0.063; × 4 = 0.25; 0.254 measured |
| `summary`, `provenance.notes` (en, de) | "cut open", end plates 0.5 × 0.6 | Describe the diamond core, chambers, DN 200 spigots, cover, drain, rails and the two sections |
| `provenance.references` | 4 | Added EN 1506:2007 and EN 13141-7:2021 |
| `representations.500.reason` / `limitingFeature` | "14 mm damper" | "1 mm sheet of spigots and flap" |

### 2D at 1:50, 1:100 and 1:200 (ruling E1)

The 2D was redrawn by hand from the same measures as the 3D. Kept shape ids: `frame`, `plate-pack`, the four `collar-*`, `collar-left/right`, `damper`, `damper-hinge`.

**Sections**
- New `sections`: the elevation is section A-A at z = 0 and the front is section B-B at x = 0.
- The plan is a plain view.

**Elevation**
- The back panel behind the chambers, with the diamond core over it.
- The cradle, top rail and (at 1:50) side seal rails.
- The spigots, cut through their axes, with flanges at 1:50.
- The base rails, the drain (1:50), and the flap profile with its hinge circle (1:50).

**Front**
- The casing, with the rear base rail and drain beyond.
- The plate pack cut across its plates, with the hatch changed to vertical lines (plate edges): 0.5 mm paper at 1:50, 0.4 mm at 1:100, none at 1:200.
- The cradle and top rail, cut.
- The flap as an extruded `solid`, and the hinge (1:50).

**Plan**
- The lid, plus a new dark `bypass-opening` that shows when the flap lifts.
- T-shaped spigots with flanges (1:50), the flap `solid` and the hinge.

**Coarser scales**
- 1:100 merges the side seals into the core.
- 1:200 merges every seal into one rhombus from floor to lid and draws the flap 20 mm thick.

**Decisions**
- All eight decision texts per scale were rewritten (en, de).
- Silhouette IoU against the 3D: 0.993, 1.000 and 0.999 at 1:50, 1.000 elsewhere, with p95 = 0 everywhere.

## 4. Model per scale (body / thermal / structure solid counts)

### 1:50 (25 / 27 / 22)

**Body**
- Six casing panels at 7.5 mm:
  - lid with the 0.36 m opening;
  - end walls with two Ø 200 holes each;
  - back;
  - front as the inspection cover.
- Two inverted U-channel rails of 2 mm steel, 40 × 30 mm.
- V cradle, top rail and two side seal rails, whose V notches cup the core corners.
- The core:
  - plate pack prism;
  - two end plates (one solid with a repeat);
  - four corner profiles (one solid repeated by a 90° rotation).
- Four revolved DN 200 spigots, 1 mm wall, with a 15 mm flange, passing through the wall holes.
- The flap blade (7.5 mm) and its slender hinge barrel (Ø 20 mm).
- A hinge leaf on the lid.
- The condensate drain (Ø 22 mm, revolve).
- Four quarter-turn fasteners on the cover.

**Thermal**
- References to the panels, with the inner face marked (see section 6), and to the rails, leaf, drain and fasteners.
- References to the core parts (exchange area) and to the flap and barrel.
- Own filled spigot tubes with mouth and inner end marked `±u`, plus four flange rings.
- Two outlet discs in the supply and exhaust mouths (D2).
- The seals inside the airway are left out.

**Structure**
- Members:
  - rails and bottom panel (references);
  - four 20 mm corner posts and a 20 mm top-edge ring (own solids);
  - cradle and top rail (references);
  - core end plates and corner profiles (references);
  - four spigots (references);
  - flap (reference).
- Joints: core seat, two pairs of spigot flange rings, hinge capsule.
- Masses: core and casing.
- Support: one support pad pair under the rails, anchored to `base-contact`.

### 1:100 (16 / 16 / 21)

- Body:
  - 15 mm panels and solid rails;
  - the core as one prism that reaches the end walls (side seals merged), with cradle and top rail;
  - spigots as plain 1 mm tubes;
  - the flap as one profile of blade and barrel.
- Leaf, drain and fasteners are omitted.
- Structure: 30 mm posts and ring, and four 20 mm core corner bars on their seat. These are not body solids at this scale; they are the core's real frame.

### 1:200 (14 / 16 / 19)

- Body: 30 mm panels, 40 × 30 mm rail bars, the core rhombus filling the inside, plain spigots, and the 20 mm flap slab.
- Structure: 45 mm posts and ring, and four 35 mm core corner bars.

### Every scale

`decisions` give 2 to 4 bilingual sentences per scale. Monotonic, copied, slender and cross-scale checks all pass.

## 5. What I saw in the final renders

These are the r3-sheet six-view sheets, including `--cut`, the clip frame `bypass t=2`, the posed 2D `partial`, and the volumes-render overlays.

**1:50 body**
- Front (+x): the end wall with the two round spigots and flanges. Through the upper bore you see the light core face; the lower bore is darker, with the seal rail edge.
- Right side (−z): the cover with four fasteners, the four spigots, the rails as end-on U channels and the drain between them.
- Back: mirror of the front.
- Top: the lid with the flap, hinge barrel and leaf.
- Three-quarter: reads as a real compact heat-recovery box.

**1:50 cut**
- The diamond core stands on its corner in the cradle, with the top rail and side rails at the corners, and four empty chambers.
- The three-quarter cut shows the core prism, the end plate, the corner profiles and a spigot bore into a chamber.

**1:50 thermal cut**
- Red casing shell, green core, purple filled spigots, yellow flap.
- Nothing pokes out.

**1:50 structure**
- A blue edge frame: posts, top ring, bottom panel and rails.
- The diamond end plates, spigot tubes with orange flange joints, the flap with an orange hinge, the core seat joint along z, the magenta core mass and green pads.
- It is clearly a load path, not the body.

**1:100 and 1:200 cuts**
- The same reading with merged seals, and at 1:200 the rhombus core touching the lid and floor.

**Clip `bypass` at t = 2 s (flap 60°)**
- The flap swings about its barrel. Through the opening you see the top rail and both upper chambers.
- In the posed 2D, the flap rises in A-A and B-B, and in plan it shortens and uncovers the dark bypass opening.

**2D**
- A-A shows the diamond core with its rails and four flanged spigots.
- B-B shows the vertical plate hatch between cradle and top rail.
- The plan shows the flap over the opening, with A-A and B-B cut markers.

## 6. Still imperfect

- **Base rails in section A-A.** They are drawn filled, although the 3D U-channel is open along z: you look through it in a true section. The 2 mm legs are 0.04 mm on paper, below every print profile, so the drawing generalises the channel to its envelope. The overlay shows this as a 36 × 28 mm "drawn only" patch at each rail; IoU is 0.993 and p95 is 0.
- **Sheet thickness.** The casing sheet is modelled at the scale floor (7.5 / 15 / 30 mm), not the real 1 mm. Only the spigots, rails and core parts keep their real wall, where C30 allows it as a hollow profile.
- **Bypass routing.** The bypass flap on the lid is kept as the asset defines it, a flap over a lid opening above the core apex. In a real unit the bypass channel routes outdoor air from the outdoor chamber to the supply chamber past the core. That duct is outside this asset and is not modelled.
- **C31 workaround.** The product's `outerArea` has no flood, so the inner faces of the closed casing are marked `sectionFaces` on the thermal references. The four seal parts inside the airway are left out of `thermal`. Logged in `tool-requests.md` (2026-11-20, third C31 case).
- **Core mass.** It is about 15.7 kg by plate count against 12 kg declared. `massKg` was not changed (no cited reference for this exact core).
- **Area corrections.** `areaM2` corrections were applied within C16: frame 2.128 → 1.54 and ports 0.22 → 0.25. They are listed here for the area reviewers.
