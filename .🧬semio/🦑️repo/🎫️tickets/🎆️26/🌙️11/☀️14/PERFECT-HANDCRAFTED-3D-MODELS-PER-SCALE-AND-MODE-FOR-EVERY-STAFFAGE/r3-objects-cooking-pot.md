# Round 3 — objects/cooking-pot

Asset: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/cooking-pot/🔣️.json`. No earlier round-3 work existed; started at step 1 on 2026-11-20. The catalog `volumes` (with the bulk pass's digital 1:500) were taken as the start and are fully replaced.

Scripts (all in the ticket folder): `r3-cooking-pot-kit.py` (real dimensions and solid writers), `r3-cooking-pot-volumes.py` (every list of every scale, part by part), `r3-cooking-pot-roots.py` (rig, poses, clips, anchors, provenance, 2D drawings), `r3-cooking-pot-build.py` (candidate document, volumes source, catalog members), `r3-cooking-pot-ship.sh` (build, validate candidate, print suitability, write catalog, splice, validate, check), `r3-cooking-pot-zoom2d.py` (enlarges the product's 2D SVG panels around the 5 mm drawing).

## 1. Reference

| quantity | value | source |
|---|---|---|
| type | stainless stew pot, nominal 16 cm, two side handles, no lid | asset `provenance.notes` (0.16 m inner width, 0.10 m high, 0.8 mm wall, 5 mm sandwich base, handles to 0.25 m) |
| inner diameter / height | 160 mm / 100 mm outside, 95 mm inside | asset; 16 cm stew pots of the trade are 9 to 11 cm high and hold about 1.9 to 2.1 L (own knowledge of catalogue data of the common makes) |
| brim volume | pi x 0.08^2 x 0.095 = 1.91 L | hand calculation; 1 L fills 49.7 mm, the asset's 50 mm |
| wall | 0.8 mm deep-drawn 18/10 sheet, outside 161.6 mm | asset; 0.6 to 1.0 mm is the usual range (own knowledge) |
| pouring rim | flared about 45 degrees, 5 mm, outside 172 mm | `dimensions.depthM` 0.172; a pouring rim projects 4 to 6 mm (own knowledge) |
| base | encapsulated sandwich: 0.8 mm floor + aluminium core + capsule = 5 mm, standing on about 150 mm, corner radius 5 to 10 mm | asset (5 mm, contact 0.15 m); construction from own knowledge |
| handles | loop of 6 mm round rod, 50 mm wide, opening 20 x 32 mm, 44 mm out from the wall to 250 mm overall, grip 14 mm under the rim, ends forged flat to 12 x 18 x 2.5 mm pads, one 8 mm rivet each | overall width from `dimensions.widthM` 0.25; construction of riveted rod handles from own knowledge |
| mass | pot 0.54 kg, handles 0.10 kg, water 1.0 kg | asset. Hand check of the modelled sheet: wall 0.31 kg + floor 0.13 kg + capsule (3.4 mm Al + 0.8 mm steel) 0.25 kg = 0.69 kg; handles 2 x 37 g. Declared values kept (no cited reference to change them against) |

WebSearch was not used; nothing here rests on a manufacturer's page. Thermodynamic references of the asset (Cengel, Incropera, ASHRAE) are untouched.

## 2. Audit against the reference (state before)

- Rig: `pot` lay on the hob (angle 0, 0.1 m along x from the centre: neither axis nor diameter); `handles` started on the pot axis at 0.07 m and ran 0.25 m to the right, 0.125 m past the right grip; `water` and `surface` were given lengths (0.05, 0.02 m), which made the water a structural member of the forces view (B1) although it is a load; mass centres were stated in a frame that only worked while the bone lay flat.
- Dimensions: correct (0.10 / 0.25 / 0.172 m), kept. The note called 0.172 m the outside of the wall while the wall is 0.8 mm: 0.172 m is the pouring rim, the wall is 0.1616 m.
- 2D: the wall was drawn 6 mm thick at 1:50 and 1:100 and 8 mm at 1:200 with a straight top, no rim, an 8 / 12 mm base without corner; the handles were 41 x 14 mm blocks at 0.064 to 0.078 m (1:50, 1:100) and at 1:200 one 0.25 x 0.04 m bar through the pot, 6 mm below the pot's foot line up to the rim; the water disc in plan was 0.072 / 0.07 / 0.055 m radius instead of 0.08; the rivets in plan lay where the rim covers them. 1:500 text named a "4 mm pot wall".
- Solids: 16-sided revolves, a 7 mm torus bead as rim, torpedo lofts as handles, a ball rivet on top of each, a lens under five bubbles; 1:100 and 1:200 thickened cups, 1:200 handle blocks standing above the rim; 1:500 a cone-topped cup. Thermal: steam and plume as discs floating 15 mm above the water (D2), base open at coarser scales. Structure: the whole body as one opaque block, masses hidden inside, the support disc inside the base (D3).

## 3. Root fixes (old -> new)

| member | old | new |
|---|---|---|
| `rig.bones.pot` | x 0, y 0, angle 0, length 0.1; com (0, 0.025) | angle 90 (the vessel axis from base to rim), length 0.1; com (0.024, 0) along the axis |
| `rig.bones.handles` | x 0, y 0.07, angle 0, length 0.25; com (0, 0) | x 0.086, y 0.125, angle -90: from the left grip end to the right grip end at grip height 0.086 m, length 0.25; com (0.125, -0.003) |
| `rig.bones.water` | y 0.005, length 0.05; com (0, 0.025) | x 0.005 on the floor, length 0 (a frame for the fill, no member); com (0.025, 0) |
| `rig.bones.surface` | y 0.05, length 0.02, prismatic axis 90 | x 0.05 along the axis, length 0, prismatic axis 0 (the axis of its parent), travel unchanged +-4 mm |
| anchor `steam-top` | (0, 0.015) | (0.015, 0): the same point, 15 mm above the surface, in the upright frame |
| pose `boiling`, clip `boil` keys | `surface.dy` 0.002 / 0.004 / -0.002 / 0.003 | the same rises as `surface.dx` |
| `provenance.notes` | wall "0.172 m outer" | wall 0.1616 m, pouring rim 0.172 m, base corner, sandwich base, riveted rod handles |
| `representations.50/100/200.geometry` | see audit | redrawn, see 4 |
| `representations.*.decisions` | old texts | silhouette, retained, merged, omitted, thinElements (and overlays at 1:100, 1:200) rewritten for the new drawings |
| `representations.500` | limiting feature "4 mm pot wall", 0.004 | "0.8 mm pot wall", 0.0008 |
| `dimensions`, `materials`, `thermal` (every `areaM2`), `structure`, masses | | unchanged |

Default pose, pose `cold` and clip `boil` at t = 0.4 s were rendered: the surface layer and its bubbles rise and fall inside the wall, nothing opens.

## 4. 2D drawings (ruling E1)

Each scale is redrawn as the true view of its own body, in the bone frames of the new rig (elevation shapes of `pot`, `water`, `surface` have x up the axis; `handles` shapes run from the left grip).

- 1:50: section A-A: `wall-left`, `wall-right` as the real 0.8 mm cut with base corner and 45 degree rim, `base` as the 5 mm plate with bevelled edge, `water` with its bevelled foot, `surface` line, `wall-far`, `handle-left/right` as the far arm rising from the pad to the grip, new `rivet-left/right`. Front: `wall-end`, `base-end`, `rim-line` at the start of the flare, `handle-end` as the arch of grip rod over two pads, `handle-rivet` and new `handle-rivet-near`. Plan: `pot-top` 0.086, `water-top` 0.08 (was 0.072), `rim-inner`, nine bubbles (`bubble-a..i`), the two open loops; `rivet-left/right` removed from the plan (the pouring rim covers pads and rivets).
- 1:100: `pot` is now the two cut walls (one path, region wall) and `base` / `base-end` are own shapes (region base) instead of one merged U; handles as bare loops; front handle as grip bar plus two arm stubs (`handle-arm-far`, `handle-arm-near`).
- 1:200: walls with a 5 mm rim band, own base plate, `handles` as one path of two lug bars at 0.081 to 0.089 m (was one bar through the pot), lugs with clipped corners in plan.
- Plan handles lie under the rim disc (z 3.4), as in reality.
- Print suitability (`office-laser`, feature 0.2 mm): the true thin parts needed the 0.18 mm pen instead of thickened geometry: the two section rivets at 1:50, the base plate and the lugs in the elevations at 1:200. All three scales resolve as `authored`.

## 5. Model per scale

Round parts are prisms of 72 (1:50, 1:100), 64 (1:200) and 48 (1:500) sides instead of revolves, because the kernel gives a 0.08 m revolve only about 20 sides at this asset size; the wall skin and the mass markers are stated ten times larger and scaled back for the same reason.

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 30: base plate, corner ring, wall, pouring rim; water foot, water, boiling surface layer, 9 bubbles; 2 bows (16 sections each), 4 pads, 4 rivet domes, 4 set heads | 17: base, wall skin, water foot, water, surface layer, 2 bows, 4 pads, 4 rivets, steam ring, plume core | 17: base plate, corner, wall, rim, 2 bows (members); 4 pads, 4 rivets (joints); pot mass, water mass; contact pad |
| 1:100 | 6: chamfered base, wall, rim, water, 2 bare loops | 7 | 8 |
| 1:200 | 6: base, wall, rim band, water, 2 lugs with clipped corners | 7 | 7 |
| 1:500 | 5: base, straight wall, water, 2 square lugs | 7 | 6 |

- Thermal: the wall is an open skin on the outside of the wall (its `areaM2` 0.054 is one face, used by the solver for the room and, half wetted, for the water; a closed 0.8 mm shell would count 0.099 m2). The water is the real body; its faces against floor and wall are `sectionFaces`, so the free surface remains (0.0204 against 0.0201). Pad faces riveted onto the wall are `sectionFaces`; set heads inside are left out of the handle region. Emitters are outlets flush on the water (D2).
- Structure: the vessel is a monocoque, so the members are the sheet parts by reference; what makes it a load path and not the body is that water, surface layer, bubbles and set heads are absent, the water is the magenta mass at its centre of mass inside the open pot, the pot mass is the flat disc round it, the support is the 150 mm contact pad under the base.

Final check (2026-11-20, catalog spliced): `volumes-check.ts objects/cooking-pot --product` PASS, all rules ok at 1:50, 1:100, 1:200, 1:500; `r3-validate.ts objects/cooking-pot --volumes` VALID: schema 0, validateAsset 0, checkVolumes 0.

| measure | 1:50 | 1:100 | 1:200 | 1:500 |
|---|---|---|---|---|
| silhouette IoU elevation / front / plan | 0.996 / 0.998 / 0.994 | 0.999 / 1.000 / 0.992 | 1.000 / 1.000 / 0.998 | no drawing (digital scale) |
| thermal area deviation wall / handle / water | 0.082 / 0.103 / 0.016 | 0.084 / 0.094 / 0.003 | total 0.08 | total 0.11 |

No `areaM2` was changed. The handle value 0.00496 m2 stands; the modelled loops with pads and rivet domes give 0.0055 m2.

## 6. What I saw

Every sheet below was rendered after the last change of the volumes and opened: `r3-sheet.ts` body, thermal (`--cut`) and structure at 1:50, 1:100, 1:200, 1:500 (twelve sheets), the clip `boil` at 0.4 s, the pose `cold`, and the product's 2D panels of 1:50, 1:100, 1:200 enlarged.

- 1:50 body. Front and back: straight wall, bevelled base edge and corner ring as two quiet bands at the foot, the rim flaring out at the top, the handle as a rod across two riveted pads. Sides: the arms rise from the pads to the level grip, the rivet domes stand on the pads. Top: round rim, water with nine bubbles, the set heads on the inside wall, two open loops with rounded corners. Three-quarter: reads at first sight as a small stainless stew pot. Grade A.
- 1:50 thermal, cut: red wall skin from base corner to rim lip, yellow water body with its surface layer and the plume core outlined on it, green 5 mm base, violet handles; nothing pokes through the skin. The dark patches on the cut water are the renderer's shading of the hollow mesh seen from inside. Grade A.
- 1:50 structure: blue shell with rim, orange pads and rivets, blue bows, the green contact pad under the base; from above the magenta water ball inside the flat pot-mass disc. From the side the masses are hidden by the wall, which is what an open vessel allows. Grade A.
- 1:100: the same pot with chamfered base and bare loops that meet the wall; thermal and structure as at 1:50 without pads. Two specks of handle colour show where the rod ends meet the skin. Grade A.
- 1:200: wall, 5 mm rim band, flat lugs with clipped corners under the rim; clean thermal cut; structure with the water mass and the pad. Grade A for the scale.
- 1:500: plain open cylinder with water and two square lugs. It is recognisable as a pot with two handles and nothing more; for a digital scale at which the object is a third of a millimetre I grade it A-, see below.
- Clip and pose: at `boil` 0.4 s the surface layer stands 4 mm higher inside the wall, closed all round; `cold` three-quarter at full size shows pads, rivets, bow and rim cleanly joined.
- 2D: the 1:50 section reads as a cut pot (thin wall, base, water with its wavy line, the far handle arms with the rivet lumps); the front shows the handle arch; the plan the rim and the two loops. 1:100 and 1:200 read the same with fewer parts.

## 7. Still imperfect

- The bubbles are body solids on the surface bone and therefore also stand on the water in the poses `cold` and `heating`, as the 2D plan draws them in every pose. A pose cannot hide a solid; it was so before this round.
- 1:500 is a plain cylinder with two box lugs (grade A- by my eye): the pouring rim is dropped and the lugs are square. Finer would be the 1:200 model again.
- Flat water at 1:100, 1:200 and 1:500 shows a light and a dark wedge in the top panels of `r3-sheet.ts`. This is the tool's corner-normal averaging on the cap of a many-sided prism, not geometry (reported in `tool-requests.md`).
- Pads, lugs and the 1:100 rod ends stand 0.2 to 0.4 mm off the many-sided wall in places, so that nothing pokes through the zero-thickness wall skin of the thermal list. Not visible in any sheet.
- The base corner is one conical ring of 7.5 mm (a part of the pot bone cannot be lower than the 5 mm base drawing allows), not a true 5 mm radius; the capsule and the floor are one plate.
- The structure members are the body's own sheet parts by reference. For a monocoque vessel there is no other load path; water, surface layer, bubbles and set heads are left out.
- The modelled steel adds up to 0.69 kg against the declared 0.54 kg for the pot. Not changed: no cited reference.
- The first paragraph of the reference table is from my knowledge, not from a manufacturer's sheet.

State: DONE, with the points of section 7 stated as they are.

## Round 5 handcraft (2026-11-21)

Scripts: `r5-cooking-pot-model.py` (round-3 volumes plus the round-5 changes and root fixes, `--apply`), shared `r5_o02_lib.py`, sheets `r5-o02-sheets.sh` / `r5-o02-montage.sh`, measures `r5-o02-measure.ts`. Renders under `🗑️generated/r5-cooking-pot/{before,after}/` (every scale body, thermal cut, structure, 2D; poses cold, heating, boiling; clip boil 0.4 s), all opened.

### Reference and model against reference (1:50, default pose)

| quantity | reference (section 1, 16 cm stew pot class) | model | deviation |
|---|---|---|---|
| wall outside diameter | 161.6 mm (160 + 2 x 0.8) | 161.6 mm | 0 % |
| pouring rim diameter | 172 mm (`depthM`) | 172.0 mm | 0 % |
| overall height | 100 mm | 100.0 mm | 0 % |
| base on the hob | 150 to 157 mm, 5 mm thick | 157 mm, 5.0 mm | 0 % |
| handle span | 250 mm (`widthM`) | 250.0 mm | 0 % |
| grip loop width / rod | 50 mm / 6 mm | 50.0 / 6 mm | 0 % |
| grip height | 86 mm | 77 to 89 mm (rod centre 86) | 0 % |
| water fill | 1 L to 50 mm above the floor | 45.8 mm body + 8 mm surface layer, top 55 mm (boiling) | as round 3 |

### Grades (body / thermal / structure)

| scale | before | after |
|---|---|---|
| 1:50 | A- / A- / A- (bubbles stood on the cold and heating water) | A / A- / A- |
| 1:100 | A- / A- / A- | unchanged A- |
| 1:200 | A- / A- / A- | unchanged A-; 2D prints over its whole band |
| 1:500 | B / A- / A- (handles as square 50 x 10 mm planks) | A- / A- / A- (loops with a round grip end) |

Catalog before: INVALID (4 `range-end-feature` findings of the 1:200 band at 1:350: base, base-end, wall-far, water 0.18 to 0.19 mm). After: VALID, `checkVolumes` 0, `volumes-check --product` PASS.

### Changes

- Rig (root): new bone `bubbles` (child of `surface`, prismatic along the vessel axis, -12 to 0 mm). Pose `cold` sinks it 12 mm (the bubbles are inside the water, the surface is still), `heating` 3 mm (only the tops break through), `boiling` and the clips unchanged; `heat-to-boil` interpolates through the poses. The nine 1:50 bubbles (body solids and plan shapes) are bound to it.
- 1:500 body: the two lugs keep 50 x 10 mm but end in a 25 mm half round, so in plan they read as the loop handles they stand for (`LUG_500`); decision text says so.
- 1:200 2D: `base`, `base-end`, `wall-far`, `water` drawn with the 0.2 mm pen, so the band 1:150 to 1:350 prints to its end; the range is unchanged (bands 20-75-150-350 stay gapless).

### Remaining

- The 1:50 plan still draws the bubbles in every pose (a 2D shape cannot sink below the water surface in plan); the 3D shows them only when the water boils.
- Everything else of section 7 stands (monocoque structure by reference, modelled steel 0.69 kg against the declared 0.54 kg without a cited source).
