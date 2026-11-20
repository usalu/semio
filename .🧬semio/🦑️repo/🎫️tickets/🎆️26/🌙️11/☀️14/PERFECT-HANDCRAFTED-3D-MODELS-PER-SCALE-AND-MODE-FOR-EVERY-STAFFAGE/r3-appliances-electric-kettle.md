# Round 3: appliances/electric-kettle

State on 2026-11-20: finished and spliced. No earlier round-3 work existed for this asset (only the round-1 script `author-11-electric-kettle.py`), so it was started from scratch. The catalog file now holds the corrected roots, the 2D drawings of every scale and view, and the new volumes. 1:500 stays unavailable, as its representation already was.

Scripts in the ticket folder:
- `r3-electric-kettle-kit.py`: measures, profiles, the spout and handle centre lines, and the writers for the solids.
- `-model.py`: states every solid of every scale and list by hand and writes `volumes/appliances/electric-kettle.json`.
- `-draw.py`: draws the 2D of every scale and view from the same measures (ruling E1).
- `-roots.py`: builds the candidate document `🗑️generated/r3-electric-kettle/🔣️.json`. With `--apply` it writes only the changed members into the catalog file, in the catalog's style.
- `-texts.json`: provenance notes, volume decisions and drawing decisions, each in en and de.
- `-probe.ts`: checks the thermal behaviour that the catalog tests pin (`🫕️phase-change`, `🚰️hydronic-coupling`) before an area change is made.
- `-zoom.ts`: crops the `r3-2d --svg` panels to the drawn object so each stroke can be judged.

## 1. Reference

| Quantity | Real subject | Source | Model now |
|---|---|---|---|
| Overall H x W x D | 205 to 305 mm high, 220 to 230 mm wide, 160 to 220 mm deep (1.7 L, 2 to 2.2 kW stainless jug kettles) | Currys and Bosch product data, read 2026-11-20 (Russell Hobbs Classics, Ariete Vintage 2869 and 2877, Bosch TWK7203); new provenance reference `retail-jug-kettles-2026` | 0.24 x 0.245 x 0.18 m (`dimensions` unchanged, within the class) |
| Power base | about 180 mm across, 25 to 30 mm high, rounded top edge, recess for the jug foot, central 360-degree connector | product knowledge | 0.18 m across x 30 mm, 4 mm round-overs, 4 mm recess of r 76 mm, connector r 16 mm |
| Jug | thin stainless wall (0.5 to 0.8 mm), slightly tapered, rolled rim, black plastic foot skirt of 15 to 20 mm carrying the connector, element bonded under the floor | product knowledge; `shell-skin` layer 0.7 mm | cone of 0.15 m tapering to 0.14 m over 0.175 m, 0.7 mm wall, 2 mm floor, rolled rim with seat collar, 16 mm skirt |
| Fill | 1.0 kg of water (asset data) | rig `water.mass`; Cengel 2015 | 1 L, 59.5 mm deep in the tapered jug (was drawn 125 mm, which is about 2.1 L) |
| Spout | covered pouring spout with limescale filter, starting at about two thirds of the height, root about 55 to 60 mm wide, tip about 25 to 30 mm, 35 to 50 mm proud of the wall | product knowledge | six sections square to the centre line, from the wall at v 0.135 to 0.19, 57 mm wide at the root, 27 mm at the tip, 46 mm proud |
| Handle | C handle, grip about 25 to 30 mm deep and 30 mm wide, finger clearance 30 to 40 mm, rocker switch at the foot or back of the grip | product knowledge (anthropometric hand thickness 25 to 30 mm) | grip 23 to 24 mm deep and 32 mm wide, clearance 31 to 34 mm, 10 x 18 x 16 mm rocker on the back of the grip |
| Lid | loose push-in lid on a seat collar, skirt into the opening, knob | asset concept (prismatic rattle joint) | flange r 66 mm, dome 14 mm, skirt r 63 mm and 10 mm deep (overlaps the collar by 0.5 mm), mushroom knob of 25 mm |

## 2. Audit of the state before round 3

- **Rig**
  - The `handle` joint sat at body x 0.098, the middle of the grip, not where the handle is fixed. `lengthM` 0.1 reached 0.072 m past the grip.
  - The water's `comY` was 0.04, placed for a fill of 125 mm.
  - The `spout-tip` anchor (-0.112, 0.192) was not at the mouth.
- **Dimensions**: in agreement with the class. Unchanged.
- **2D**
  - Water was drawn 0.125 m deep at 1:50 and 1:100, which is 2.1 L against the asset's 1.0 kg.
  - The lid was a flat rectangle (0.132 x 0.016) with a block for a button.
  - The spout was a stuck-on triangle.
  - The handle was a stiff polygon with a 16 mm grip and a 30 mm finger gap.
  - The front `body-shoulder` line at mid height matched no real feature.
  - No skirt and no switch were drawn.
  - The decisions repeated whole sentences at 1:50, 1:100 and 1:200 (silhouette and omitted texts).
- **Solids**: `qa-visual.md` graded body B, thermal C and structure B.
  - The body revolves were open profiles that the kernel filled through the axis (`kernel-traps.md`: base and lid at 1:50, 1:100 and 1:200; lid button at 1:50).
  - The shell was a 48-gon extrusion and visibly faceted.
  - The spout was a cone stuck onto the rim.
  - The gauge read as a decal.
  - The water was a cone of 125 mm.
  - The lid had no skirt, so the 3 mm boiling lift opened a visible gap.
  - Thermal had no nested water, and the steam was two floating rings in the air.
  - Structure was `{ of }` copies of the whole body, plus a floor disc and a ring.
- **Thermal areas**: the declared `shell` 0.15 was computed from a jug of 0.16 x 0.2 m that also counted the handle. The declared `lid` 0.025 was computed from a lid of 0.14 m.

## 3. Root fixes (catalog file, outside `volumes`)

| Member | Old | New |
|---|---|---|
| `rig.handle` | x 0.098, lengthM 0.1, comX 0 | x 0.072 (on the wall, between the two fixings), lengthM 0.057 (wall to grip), comX 0.035 |
| `rig.water.mass.comY` | 0.04 | 0.03 (centre of the 59.5 mm fill) |
| `anchors.spout-tip` | (-0.112, 0.192) | (-0.116, 0.19), the centre of the mouth |
| `thermal.regions.shell.areaM2` | 0.15 | 0.104: the honest outer area of the 1:50 model, made up of the cone (0.069), rim (0.003), skirt (0.009), bottom (0.018) and spout (0.005). Factor 1.44, within the 1.5 allowed by C16 |
| `thermal.regions.lid.areaM2` | 0.025 | 0.019: dome and flange 0.0155, knob 0.0018, skirt band exposed when the lid rattles 0.002. Factor 1.32 |
| `thermal.assumptions` | "the boil still takes about 184 seconds" | "the boil takes about 179 seconds" (en and de). The probe gives the boil at 179 s with the smaller shell loss |
| `clips.boil` | heating key 171, boiling key and phase boundary 184, plume track keys 141 and 184 | 166, 179, and 136 and 179, so the clip follows the energy balance again |
| `provenance` | 3 references, old notes | + `retail-jug-kettles-2026`; notes rewritten in en and de (construction, 60 mm fill, 0.83 g/s) |
| `representations[50,100,200].geometry` | old drawings | redrawn from the model at every scale and view (section 4); new 1:50 shapes `skirt-line` (elevation and front) and `switch` (elevation and front). The front `body-shoulder` line became `skirt-line` |
| `representations[*].decisions` | duplicated sentences | rewritten per scale in en and de |

The `🫕️phase-change` and `🚰️hydronic-coupling` expectations were checked against the candidate with `r3-electric-kettle-probe.ts`; all of them still hold:

| Check | Limit | Probe result |
|---|---|---|
| water at 120 s | below 80 | 74.8 |
| water at 170 s | below 97 | 96.3 |
| water from 185 s | 100 | 100 |
| phase-change power at 184 s | above 1500 W | 1866 W at 195 s |
| evaporation at 195 s | 0.75 to 0.85 g/s | 0.831 g/s |
| evaporation at 100 s | below 0.05 g/s | 0.0009 g/s |
| wall gap | 0 to 6 K | 2.74 K |

Summary, pose descriptions, emitters, supports, materials and the `dimensions` are unchanged.

## 4. What was modelled

### 1:50 (feature 7.5 mm): body 10, thermal 12, structure 16

- **Body**
  - Base turned from its real section, with a recess that takes the skirt.
  - Black skirt ring overlapping the shell wall and resting in the recess.
  - Shell as a closed-section revolve: 0.7 mm wall, 2 mm floor, rolled rim and seat collar.
  - Spout loft: six rotation-fixed superelliptic sections square to its centre line, entering 4.7 mm through the wall.
  - Window: a rounded glazing loft, 1.2 mm proud.
  - Water: 1 L revolve against the inner wall.
  - Handle: one planar loft of 17 sections along a C centre line with radius 17.5 mm corners, so there are no seams or twist.
  - Switch box.
  - Lid with skirt, and knob.
  - Every revolve profile ends on the axis or is a closed loop (no implied closure).
- **Thermal**
  - `plate`: the base by reference, plus the element bonded under the floor with its faces marked.
  - `shell`: the closed outer envelope (`th-shell`, rim disc marked `+v`) plus the spout.
  - `water`: the fill and the window.
  - `lid`: its own skin with the skirt band (`-v` marked), plus the knob.
  - `handle`: the handle and the switch.
  - Emitters as outlet patches (D2): `th-steam` caps the spout mouth; `th-steam-plume` is a sleeve over the last 30 mm of the spout.
  - The cutaway shows the water nested in the shell, with air under the lid.
- **Structure**
  - Base wall ring, deck annulus and four radial ribs (repeat).
  - Connector boss as the body joint.
  - Skirt (reference, it is the member).
  - Stainless cup (floor and wall) up to the seat.
  - Lid seat ring as the lid joint, and the lid plate.
  - Handle core loft at 55 % section, with top and bottom screw bosses as joints.
  - Water footprint on the floor.
  - Masses of base, jug and water at their centres.
  - Rim support ring at the `base-contact` anchor.

### 1:100 (feature 15 mm): body 6, thermal 8, structure 11

- **Body**
  - Plain turned base.
  - Hollow cone with a 10 mm foot (skirt and floor merged) and a flat seat collar.
  - Spout from three sections, handle from seven sections.
  - Lid and knob as one 30 mm turned mass, as the 1:100 drawing gives it.
  - Window, switch, rolled rim and skirt line are below the scale and merged.
- **Thermal**: envelope, lid skin, references, steam patch and plume sleeve.
- **Structure**
  - 15 mm housing.
  - Connector.
  - Cup with a 15 mm foot.
  - Handle core at 70 % section with one boss.
  - Lid plate on a 15 mm seat ring.
  - 15 mm water footprint.
  - Masses of base and water.
  - 15 mm support ring.

### 1:200 (feature 30 mm): body 5, thermal 8, structure 7

- **Body**
  - 40 mm base disc.
  - The jug as one solid tapered mass with a rounded rim.
  - Spout from three sections, handle 30 mm thick throughout, and the 40 mm stepped lid and knob.
- **Thermal**: references, plus the water as its own turned volume inside the solid jug, plus the two emitters.
- **Structure**
  - Housing on a 30 mm ring.
  - 5 mm cup on a 30 mm foot.
  - Handle and lid as members.
  - Water footprint and water mass.

### 2D (ruling E1)

- Every shape keeps its id, bone, role, z, tone, region and material. Its geometry is the true orthographic view of the solids of that scale, computed from the same profiles and centre lines:
  - the body outline with skirt and rim;
  - the spout clipped at the wall;
  - the handle as the two offset edges of its centre line;
  - the lid and knob profiles;
  - the base round-over;
  - the 1:200 plan as one outline of the jug circle, spout wedge and handle bar.
- The coarser scales keep their own merges (body and spout in one outline at 1:100 and 1:200, the merged front and plan at 1:200).
- Checked as zoomed SVG panels with `r3-electric-kettle-zoom.ts`: the line work is clean, with no slivers.
- The pose `boiling` (default) and the clip `rattle` at t 0.3 s (lid up 5 mm) were rendered. The lid skirt closes the gap in both.

## 5. Final check

`$BUN volumes-check.ts appliances/electric-kettle --product`: PASS, every rule at every scale.
- **Silhouette IoU** (elevation / front / plan):
  - 1:50: 0.983 / 0.983 / 0.992;
  - 1:100: 0.997 / 0.998 / 0.994;
  - 1:200: 0.973 / 0.973 / 0.992.
- **Thermal area deviation** (plate / shell / lid / handle):
  - 1:50: 4 / 0.2 / 1.2 / 13.6 %;
  - 1:100: 0.9 / 2.1 / 0.5 / 16.2 %;
  - 1:200: total 24 %, below the 30 % limit.
- **Other rules**: bounds, feature, monotonic (solid counts fall at every step in every list), cross-scale, copied, structure coverage and views-disagree all pass.

Validation results:
- `$BUN r3-validate.ts appliances/electric-kettle`: VALID (schema 0, validateAsset 0).
- `--catalog-only --volumes`: VALID, checkVolumes 0.

## 6. What the final renders show

**1:50 body**
- Front (from +x): the handle stands in front of the jug, with the rocker on the back of the grip; tapered stainless jug, black skirt band, rounded base. The window edge is just visible at the left flank.
- Right side (elevation): the spout rises from two thirds of the height to above the rim. The C handle is round in section, with even corners and a bellied grip, and the switch is a small step on its back. The window has rounded ends. The domed lid sits on the rolled rim, with the skirt visible in the 3 mm boiling gap.
- Back: the spout's root is wide and narrows to a 27 mm mouth; the rim and lid are symmetric.
- Left side: mirror of the right side, without the window.
- Top: lid with knob, the seat ring as a dark shadow line, the spout wedge, the handle bar and the base rim.
- Three-quarter: reads at once as a classic stainless jug kettle on a 360-degree base.

**1:50 rattle clip, t 0.3**: lid up 5 mm, skirt still in the collar, no gap and no floating.

**1:50 thermal (cut)**
- Front and back show the water (60 mm) nested in the green shell and air above it.
- The red element band under the floor, the red base and the yellow lid with knob are visible.
- The cut also shows the root cap of the spout and the handle roots as discs on the inner wall.
- Steam patch at the mouth.

**1:50 structure (cut)**
- Blue cup wall and floor; orange connector between the base deck and the floor, and an orange seat ring under the lid plate.
- Handle core with two orange bosses at the wall.
- Base wall, deck and ribs; green rim support; magenta masses in base, jug and water.
- Uncut, the cup reads like the body: it is a monocoque, so the wall is the member.

**1:100**
- Body: the same kettle with plainer base and rim and a stepped lid-and-knob mass.
- Thermal cut: the water nested inside the shell.
- Structure cut: cup, joints, masses and support all readable.

**1:200**: chunky but true. A 40 mm base, the solid jug, a thick spout and handle, and the stepped lid. Recognisable at a glance.

**Overlays**: all green, except a 3 mm blue line under the lid (the skirt band in the boiling lift) and a sub-feature red corner at the 1:200 plan handle end (the drawing's square corners against the round section).

## 7. Still imperfect

- **C31 stand-in.** The product's `outerArea` has no cavity flood, so the thermal shell is an envelope solid and the lid and element faces are marked (`sectionFaces`) instead. These marks can go once C31 is implemented (`tool-requests.md`, 2026-11-20, second case).
- **Structure from outside.** The uncut structure view still looks like a blue kettle, because the jug is a monocoque. Only the `--cut` sheets show the load path.
- **`handle` area.** It reads 0.018 against the declared 0.021 (-14 % at 1:50, -16 % at 1:100). This is within tolerance and was left unchanged.
- **Not modelled.**
  - The limescale filter and cord, which the 2D also omits.
  - The rubber feet: about 3 mm, below the feature.
  - The rim roll's 0.9 mm sheet. It is modelled, but too thin to be seen in the renders.
- **Facets.** Lid flange and base edges show tessellation facets in the product renderer. This is renderer quality, not geometry.
- **Lid and pose.** The 3 mm boiling lift appears in every overlay as a thin 3D-only strip under the lid flange. It is below the feature at every scale.
