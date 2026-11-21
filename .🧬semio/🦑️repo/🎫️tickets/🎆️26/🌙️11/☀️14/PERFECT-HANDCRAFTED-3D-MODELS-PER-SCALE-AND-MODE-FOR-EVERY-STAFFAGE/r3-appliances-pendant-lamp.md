# Round 3: appliances/pendant-lamp

State on 2026-11-20: finished and spliced. No earlier round-3 work existed for this asset; only the round-1 script `author-12-pendant-lamp.py` did, so I started from scratch. The catalog file now holds:
- the corrected roots: the rig, the masses, the emitter direction, the cord area, the texts and a new reference;
- the redrawn 2D of every scale and view;
- the new volumes.

1:500 stays unavailable, as its representation already was.

Scripts in the ticket folder:
- `r3-pendant-lamp-kit.py`: the measures, every turned profile (rose, cord grip, collar, lampholder, the dome with its rolled bead, the cap, the A60 glass), the plume outlet patch, and the solid writers.
- `-model.py`: states every solid of every scale and list by hand. With `--write` it writes `volumes/appliances/pendant-lamp.json`.
- `-roots.py`: builds the candidate document `🗑️generated/r3-pendant-lamp/🔣️.json` from the catalog. That covers the rig, masses, emitter, areas, texts, and the 2D drawn from the same profiles as the solids (ruling E1). With `--apply` it writes only the changed members, in the file's own formatting.
- `-texts.json`: summary, provenance notes, the `on` pose description, the representation decisions and the volume decisions, each in en and de.
- `-zoom.ts`: zooms the `r3-2d --svg` panels onto the shade (or the rose, with `top`).
- `-crops.ts`: crops one region out of several `r3-sheet` images and lays them side by side.

## 1. Reference

| Quantity | Real subject | Source | Model now |
|---|---|---|---|
| Overall drop | 0.8 m from ceiling to bulb tip (common drop; cord length is free) | asset `dimensions`, product class | 0.8 m (unchanged) |
| Ceiling rose | 90 to 110 mm across, 20 to 30 mm deep; flat back on the ceiling, rounded lower edge, cord grip at the centre | product knowledge (Hornbach/Bauhaus canopy and enamel pendant sets) | 100 x 24 mm, 8 mm lower round, 14 x 10 mm cord grip |
| Cord | 3 x 0.75 mm² textile-sheathed, 6.5 to 7.5 mm | asset materials | 6.6 mm, 0.6 m |
| Cord-grip collar | brass sleeve 25 to 35 mm across, 25 to 35 mm high, on an M10x1 lamp nipple | product knowledge | 31 x 30 mm, domed top, 11 mm bushing |
| Shade | enamelled steel 0.5 to 0.8 mm, dome 260 to 300 mm, rolled rim bead; clamped at the apex between collar and lampholder | asset (0.266 x 0.133 m, layer 0.6 mm) | 0.6 mm sheet, Bezier dome of the asset's drawing, 6 mm rolled bead |
| E27 lampholder | brass, 35 to 40 mm across, 55 to 70 mm long; the bulb screws into it | IEC 60238 (new reference `iec-60238`) | 40 x 65 mm, rounded top, 28.4 mm bore |
| Bulb | A60: 60 mm across, 105 mm long; E27 cap 26.5 to 27 mm across | IEC 60064, IEC 60061-1 (asset references) | A60 envelope of the asset's profile, neck into a 27 x 27 mm cap with contact and insulator shoulder |
| Bulb position | carried by the lampholder; in a 133 mm deep dome the bulb sits inside, and only the tip shows | geometry of the parts above | tip 7.5 mm below the rim |

## 2. Audit of the state before round 3

- **Rig**
  - Every bone had `angleDeg` 0, so all of them pointed horizontally ahead. The cord bone ran 0.52 m sideways from the rose (`r3-sheet` showed the magenta bones off to the side).
  - The bulb was a child of the shade at 0.225 m below the apex. Its cap top (world 0.105) was 0.12 m below the collar, and **nothing held the bulb**: there was no lampholder in the body, only a 16 mm "stem" in the structure list.
  - The bulb hung 87 mm below the rim. With a real lampholder in the apex this is impossible for a 133 mm deep dome.
- **2D**
  - The rose was drawn upside down: the full width was at the bottom and the rounded edges were against the ceiling.
  - The bulb was drawn hanging below the shade, consistent with the wrong rig.
  - The plan's `plan-shade-rim` (r 0.1235) and `plan-rose-face` (r 0.032) rings matched no real edge that can be seen from above.
  - The 1:100 dome ended at r 0.13 against a plan of r 0.133 (the red ring found by QA).
- **Solids** (`qa-visual.md`: B / B / B at every scale; `kernel-traps.md`: 1:100 and 1:200 shades refused for self-intersection)
  - The rose and shade revolves were open profiles that the kernel filled through the axis.
  - The thermal lofts had overlapping sections and missing caps.
  - The 1:100 body was a near copy of 1:50.
  - The plume was a disc floating on the apex.
  - The 1:200 support capsule stuck out of the rose.
  - The mass beads were wider than the cord.
- **Thermal areas**: the check already failed before round 3 (shade +44 %, rose +59 %, lampholder +75 %, 1:200 total +54 %).

## 3. Root fixes (catalog file, outside `volumes`)

| Member | Old | New |
|---|---|---|
| `rig.rose` | angleDeg 0, comY -0.012 | angleDeg -90 (every bone now hangs down, u = down), comX 0.012 |
| `rig.cord` | x 0, y -0.024, lengthM 0.521, comY -0.26 | x 0.024, y 0, lengthM 0.6, comX 0.3 |
| `rig.holder` | "Brass collar", y -0.521, lengthM 0.03, comY -0.015 | "Brass collar and E27 lampholder" / "Messinghülse und E27-Fassung", x 0.6, lengthM 0.0956 (collar top to lampholder mouth), comX 0.049 |
| `rig.shade` | y -0.03, comY -0.06 | x 0.03 (apex), comX 0.068 (centroid of the shell) |
| `rig.bulb` | parent shade, y -0.225, comY 0.05 (joint at the tip) | parent **holder**, x 0.071 (joint at the cap contact, inside the lampholder), comX 0.053 |
| `thermal.emitters.shade-plume.directionDeg` | 90 (bone-relative, now horizontal) | 180 (up along -u; a plume rises anyway) |
| `thermal.regions.cord.areaM2` | 0.0108 | 0.0124 (pi x 0.0066 x 0.6 m, factor 1.15) |
| `summary`, `provenance.notes` | bulb "hanging below the rim", cord 0.52 m, cap and collar "6 W at 0.005 m2" | real construction, cord 0.6 m, bulb on the lampholder with its tip 7.5 mm below the rim, split 34 / 20.5 / 2.5 W as in `thermal` (en, de) |
| `provenance.references` | 6 | + `iec-60238` |
| `poses.on.description` | "a plume rises through the shade" | "rises over the shade"; cap and lampholder warm |
| `representations[50,100,200].geometry` | old drawings in the upright frames | redrawn from the kit's profiles in the hanging frames (section 4) |
| `representations[*].decisions` | old texts | rewritten per scale (en, de) |

Unchanged: `dimensions`, materials, anchors (both at bone origin (0, 0)), poses' thermal data, clips, the structure supports, and every region's heatW.

The shade (0.223 measured against 0.11), rose (0.022 against 0.012) and lampholder (0.012 against 0.0042) are beyond C16's factor 1.5. Their areas are **not applied**; they are listed with hand calculations in `area-review.md`.

## 4. What was modelled

All parts are revolves about the hanging axis (axis u). Every profile is closed on the axis or is a loop, so there is no implied closure, and there are no lofts.

### 1:50 (feature 7.5 mm): body 8, thermal 9, structure 16

- **Body**
  - Rose: flat back, rounded lower edge.
  - Rose cord grip.
  - Cord: 6.6 mm, `slender`.
  - Collar: domed top and bushing.
  - E27 lampholder: rounded top, chamfered mouth, bore around the cap.
  - Shade: a 0.6 mm sheet from the nipple hole along the drawing's Bezier (40 steps) into a 6 mm outward rolled bead.
  - E27 cap: contact and insulator shoulder.
  - A60 glass: the neck enters the cap.
- **Thermal**: every region is a reference to its body surface, with its layer:
  - rose (2 solids);
  - cord;
  - lampholder: collar, lampholder and cap;
  - glass;
  - shade sheet: both faces exchange heat.

  The plume outlet `th-shade-plume` (tag `shade-plume`) is a crown patch from the collar to r 0.05 that lies flush on the dome: 0.5 mm into it and 1.5 mm proud (D2).
- **Structure**
  - Ceiling-hook pin (support, `anchor` ceiling-hook).
  - Rose hanger.
  - Cord grip joint.
  - Cord member (slender).
  - Collar cord-grip joint.
  - M10 nipple member.
  - Lampholder member (`of` socket).
  - Lock-nut joint clamping the shade at the apex.
  - Shade member (`of` shade: the shell is its own structure).
  - E27 thread joint.
  - Cap member (`of` bulb-cap).
  - Masses of rose, cord (no wider than the cord), holder, shade and bulb at their centres.

### 1:100 (feature 15 mm): body 6, thermal 7, structure 9

- **Body**
  - Rose with a chamfer; its cord grip is merged.
  - Cord, slender at its true size.
  - Plain collar.
  - Plain 40 mm lampholder with bore.
  - Ten-step dome of the same 0.6 mm sheet (C30), with a short flare to r 0.133 instead of the bead. This fixes the old red plan ring.
  - Cap and glass as one turned bulb.
- **Thermal**: references, plus a 15 mm outlet patch (to r 0.066).
- **Structure**
  - 15 mm pin.
  - Cord.
  - Nipple and lampholder as one turned member.
  - 15 mm lock-nut joint.
  - Dome.
  - Cap rod.
  - Masses of holder, shade and bulb.

### 1:200 (feature 30 mm): body 4, thermal 6, structure 6

- **Body**
  - 30 mm rose disc.
  - Slender cord.
  - Bell: the dome with the collar as its 31 mm neck and a 3 mm wall.
  - One bulb body: lampholder, cap and a six-point pear, so the bulb still hangs from the apex.
- **Thermal**
  - References for rose, cord and shade.
  - Its own glass solid below the lampholder mouth.
  - Its own lampholder solid: collar sleeve 1 mm proud of the neck, plus the lampholder.
  - A 30 mm outlet patch to r 0.088.
- **Structure**
  - 30 mm pin.
  - Cord.
  - 31 mm holder rod down to the cap contact.
  - Bell.
  - 31 mm cap rod.
  - Shade mass.

Monotonic in every list: 8 / 6 / 4, 9 / 7 / 6 and 16 / 9 / 6 solids.

### 2D (ruling E1)

Every shape keeps its id, bone, role, z, tone, region and material. The geometry is the outer profile of that scale's solids, point for point:
- the rose with its round and its grip;
- the collar;
- the dome (40 / 10 / 6 steps) with the bead or flare;
- the cap;
- the glass, using the drawing's own Beziers re-expressed in the hanging frame;
- at 1:200, the bell with its neck and the bulb with the lampholder.

The front view is the same profile in world coordinates. In the plan, the shade is a disc of r 0.133 and the rose a disc of r 0.05. The two ring details were removed, because neither matches a visible edge and the bead lies 3 mm inside the outline.

The highlight was redrawn 12 mm inside the dome curve. The filament and cap-thread details moved with the bulb frame and now lie behind the shade, as the real ones do.

## 5. Final check

`$BUN volumes-check.ts appliances/pendant-lamp --product`: `FAIL: 7 measurement(s) over 3 rule line(s)`. All seven are `volumes-thermal-coverage` and come only from the three declared areas that wait for review:

| Scale | shade | rose | lampholder |
|---|---|---|---|
| 1:50 | +103 % | +84 % | +193 % |
| 1:100 | +101 % | +78 % | +244 % |

At 1:200 the total is +85 %. With the proposals of `area-review.md` (0.22 / 0.022 / 0.013) every region would be within 3 to 6 % at 1:50.

Every other rule passes:
- availability;
- silhouette: IoU elevation / front / plan, with p95 of 0 to 4 mm:
  - 1:50: 0.995 / 1.000 / 0.991;
  - 1:100: 0.996 / 0.999 / 0.992;
  - 1:200: 0.999 / 1.000 / 0.994;
- views-disagree, bounds, feature, monotonic;
- cross-scale: IoU 1.0 in all views;
- structure coverage: outside the body 0 / 0 / 1.7 mm;
- bones, slender and copied.

`$BUN r3-validate.ts appliances/pendant-lamp`: VALID (schema 0, validateAsset 0). With `--catalog-only --volumes` after the splice it is INVALID, but only through the same 7 `volumes-thermal-coverage` area lines (checkVolumes 7). The catalog before round 3 already failed the same rule.

## 6. What the final renders show

**1:50 body sheet (`r3-sheet`, six panels)**
- The rig now hangs as one vertical line: joints at the ceiling, under the rose, at the collar top, at the apex and at the cap contact.
- Front, right, back and left: a flat ceiling rose with a rounded lower edge and its small grip; the thin cord; the brass collar seated on a deep enamelled dome; the rolled rim bead as a fine lip; the glass tip just below the rim.
- Top: the shade disc under the rose.
- Three-quarter: reads at once as a classic enamel pendant.

**Seen from below (turn 20, tilt -60)**: the inside of the dome, the lampholder cylinder and the A60 bulb in the middle, and the bead around the rim.

**Cut (right side, near half removed)**: the collar on the apex, the lampholder under the sheet, the cap in the bore with 2.4 mm showing below the mouth, and the bulb filling the dome down to the rim. Nothing floats and nothing interpenetrates visibly. The cut 1:100 and 1:200 bodies show the same arrangement, simplified.

**Thermal cut**
- Purple shade sheet; green collar, lampholder and cap; red glass.
- From outside, the outlet patch is a flush ring on the crown at every scale.
- At 1:200 the lampholder sleeve and the bell's neck no longer z-fight.

**Structure cut**
- Blue cord, nipple and lampholder, dome and cap.
- Orange cord grips and lock nut.
- Green pin in the rose.
- Magenta masses on the axis: the shade's sits in the air at its true centroid, inside the dome.
- At 1:200 the holder rod and cap rod meet without overlapping.

**Other states**: the clip `switch-on-warm-up` at t 100 s and the pose `dimmed` render identically to `on`, because no bone moves in any pose.

**2D zooms (1:50 / 1:100 / 1:200)**: clean dome outlines, the bead as a small lip, the rose with its grip and round, and a highlight inside the dome. In the solid views the glass tip shows under the rim.

## 7. Still imperfect

- **Areas.** Shade, rose and lampholder await the area reviewers (`area-review.md`), so `volumes-thermal-coverage` stays red at 1:50, 1:100 and 1:200 (total) until then.
- **Ground line.** The product draws a ground line at the origin (the bulb tip). Because the bulb now sits inside the dome, that line covers most of the 7.5 mm glass tip in the line views. The lamp no longer "shows its bulb" in elevation. That is true to the real object, but it is a change of character from the old drawing.
- **Cord at coarse scales.** The 1:200 drawing keeps its 14 mm cord (and the 1:100 drawing its 9 mm cord) for print, while the 3D cord stays at the true 6.6 mm (C24). The overlay shows a red halo along the cord, which the rule does not count.
- **Facets.** The render tessellation still shows facets on the rose and the bead; the profiles themselves are smooth.
- **Not modelled.**
  - The M10 nipple and lock nut in the body: they are hidden inside the collar and lampholder; the structure list has them.
  - The pull-switch, the filament and its stem as solids, and the E27 thread and the textile weave: all are below the feature or hidden.
- **Faces in contact.** The faces of the collar and lampholder that press on the dome are counted in the lampholder area as faces covered by another region (C29). I did not mark them as `sectionFaces`.

## Round 5 handcraft

Scripts: `r5-pendant-lamp-model.py` (keeps the hand-stated 1:50 to 1:200 lists of `r3-pendant-lamp-model.py`, states 1:500 anew), `r5-pendant-lamp-texts.json` (1:500 decisions), `r5-appliances-crop.ts` (close-up crops of tall sheets). Renders in `🗑️generated/r5-pendant-lamp/{before,after}/` (opened: body, structure and thermal cut at every scale, close-ups of shade, collar and rose at 1:50 and of the 1:500 body and structure, poses `dimmed`, `off`, clip `switch-on-warm-up` t 150, the 2D of 1:50, 1:100, 1:200 zoomed).

### Reference and measurement (1:50)

| Quantity | Reference | Source | Model | Deviation |
|---|---|---|---|---|
| Drop ceiling to bulb tip | 0.8 m | asset dimensions | 0.800 m | 0 % |
| Ceiling rose | 90 to 110 x 20 to 30 mm | canopy sets (Hornbach/Bauhaus) | 100 x 24 mm | in range |
| Cord | 6.5 to 7.5 mm | 3 x 0.75 mm² textile cord | 6.6 mm | in range |
| Shade | 260 to 300 mm dome, 0.5 to 0.8 mm enamelled steel | product class, asset | 266 x 133 mm, 0.6 mm, 6 mm bead | 0 % to dims |
| E27 lampholder | 35 to 40 x 55 to 70 mm | IEC 60238 | 40 x 65 mm | in range |
| A60 bulb | 60 x 105 mm, E27 cap 27 mm | IEC 60064 / 60061-1 | 60 x 105 mm, cap 27 x 27 | 0 % |

### Grades (body / thermal / structure)

| Scale | Before | After |
|---|---|---|
| 1:50 | A- / A- / A- | unchanged A- / A- / A- (re-opened) |
| 1:100 | A- / A- / A- | unchanged |
| 1:200 | A- / A- / B+ | unchanged |
| 1:500 | C / C / C (the shade a straight cone, a witch hat, thermal shell a stepped different outline) | A- / A- / A- |
| Poses/clips | A- | A- (light states, geometry closed) |

### Changes

- 1:500 authored anew: 100 mm rose with a rounded edge, true slender cord, the shade as the 1:50 Bezier dome at six stations as a 4 mm shell with its neck (a dome, not a cone), the A60 bulb inside on a 40 mm lampholder stub; thermal references the shade and has its own lampholder, bulb and plume patch; structure hook, cord, holder, shade shell, bulb cap, shade mass.
- `volumes-check.ts --product` PASS, `r3-validate.ts --volumes` VALID; spliced.

### Remaining

- 1:50 collar is a plain turned sleeve (no knurl, no grub screw: below 7.5 mm).
- Coarse shade shells show slight facets from the tessellation of their few profile stations.
