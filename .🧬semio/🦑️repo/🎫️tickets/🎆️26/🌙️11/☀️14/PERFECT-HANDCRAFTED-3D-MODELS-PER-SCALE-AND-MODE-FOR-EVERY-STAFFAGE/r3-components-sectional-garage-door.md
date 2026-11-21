# Round 3 — components/sectional-garage-door

State: DONE (with the imperfections listed at the end, stated plainly)

Scripts (all in the ticket folder): `r3-sectional-garage-door-kit.py` (construction numbers, section profiles, travel path, pose solver), `r3-sectional-garage-door-volumes.py` (every solid of every scale and list), `r3-sectional-garage-door-roots.py` (rig, anchors, poses, dimensions, materials, references, thermal areas, all 2D drawings; `--apply` writes the members into the catalog), `r3-sectional-garage-door-crop.ts` (close-ups of renders and SVG). No earlier round-3 work existed for this asset; the catalog `volumes` were identical to `volumes/components/sectional-garage-door.json` at the start (no bulk-pass change to this asset).

## 1. Reference

| item | real value | source |
|---|---|---|
| leaf | 2.42 m wide between the tracks (2.5 m over the brackets), 2.0 m high, four sections of 0.5 m | asset summary, `door-catalogues` |
| section | 40 to 42 mm PU sandwich, steel skins 0.5 mm, embossed centre rib, finger-protection joint (convex nose on the lower section, socket behind an outer lip on the upper one, the joint closed in every angle), end stiles as folded steel caps, bottom rail with EPDM bulb seal | `door-catalogues`, EN 12604 (new reference `en-12604`), my knowledge of insulated sectional doors |
| hinge | 2.5 mm steel leaves, pin behind the inner face; side hinges carry the roller | hardware catalogues (new reference `door-hardware`) |
| roller | nylon wheel 46 mm on an 11 mm stem of about 110 mm, running in a 2 inch track | `door-hardware` |
| track | 2 mm galvanised channel about 52 x 30 mm with curled lips; vertical run, curve, horizontal run with a stiffening angle | `door-hardware` |
| curve radius | 0.5 m (20 inch radius tracks exist; 12 and 15 inch are the residential standard) | asset summary and `door-catalogues` declare 0.5 m: kept |
| fixing | jamb brackets and a flag bracket per side, punched angle 30 x 30 x 2.5 mm hangers with sway braces, rear spreader | `door-hardware` |
| balance | torsion shaft 25.4 mm in flanged bearings on end bearing plates, two cable drums, one torsion spring of 122 turns of 8 mm wire on a 75 mm coil (0.976 m) between a stationary cone on an anchor bracket and a winding cone | `door-hardware`; spring data from the asset's thermal layer `coil-wire` and `area-decisions.md` |
| drum | flange 0.18 m, cable radius 0.08 m; consistent with the declared spring (5.5 turns over the 2.785 m travel, torque about 44 Nm for 56 kg, wire stress about 875 MPa) | hand calculation, kept |
| bottom roller | about 80 mm above the floor in a bottom carrier with the cable stud | `door-hardware` |
| top roller | about 60 mm below the top edge in a top carrier | `door-hardware` |
| cable | runs between the leaf edge and the wall side of the roller stems | `door-hardware` |

## 2. Audit against the reference (before)

- Rig: the bone line of every panel was the panel centre line and at the same time the roller path: the leaf stood inside its own track (panel -20..+20 mm, track band -20..+20 mm). A real leaf lies wholly in front of the roller line.
- Hinge knuckles of the old solids sat at v = -33.5 mm, not on the bone joints: every posed joint tore the hinges apart.
- Bottom roller on joint 0 at floor level (a 22 mm disc half in the floor); top roller on the top edge; rollers 22 mm in a 40 x 40 mm "C" track (real 46 mm in 52 x 30 mm).
- Spring shaft at x = 0.12 m: the cable tangent (0.04 m) lay on the garage side of the roller stems, where the stems cross it as soon as the door enters the curve.
- `drum-cable` anchor on the flange radius (0.09 m) instead of the cable radius; `cable-eye` and `jamb-fix` on the rig origin; wind anchors and panel masses on the hinge line instead of the panel.
- 2D: front view drawn as "seen from outside" with rib lines although the product's front is seen from ahead (inside); hinges as bands across the whole width; plan with "hinge blocks" on the track bone; no brackets, no stiffening angle, no brace, no carriers, no stiles.
- 3D (qa-visual B/B/B, 1:500 structure C): flat panel boxes with a 5 mm chamfer, hinges as 8 mm boxes, coil centred without cones or anchor bracket, bearing "flags" hanging in the air, structure = the body by reference, supports floating.
- Thermal: `track-steel` 3.18 m2 was an estimate for bare tracks and hangers.

## 3. Root fixes (catalog members `provenance`, `dimensions`, `materials`, `rig`, `anchors`, `representations`, `poses`, `thermal`)

| what | old | new |
|---|---|---|
| panel bone line | panel centre line | hinge line 20 mm behind the inner face (panel solids and shapes at v = 20..60 mm); joints unchanged, so the joints are now the hinge pins |
| panel mass `comY` (4 bones) | 0 | 0.04 |
| `spring-shaft` bone x, y | 0.12, 2.75 | 0.07, 2.75 (cable tangent 10 mm on the wall side of the stems) |
| `carriage-x` joint `minM` | 0 | -0.05 (joint 0 leaves the path by up to 31 mm while the bottom roller rounds the curve) |
| anchor `drum-cable` x | -0.09 | -0.08 |
| anchor `cable-eye` | (0, 0) | (0.06, 0.01), the cable stud |
| anchor `jamb-fix` | (0, 0) | (-0.075, 0.03), the foot bracket on the wall plane |
| anchors `wind-1..4` y | 0 | 0.06 (outer face) |
| poses (11) | joints 0 to 4 on the path | bottom roller (u = 0.08 of panel 1), hinges 1 to 3 and top roller (u = 0.44 of panel 4) on the path, re-solved from the old position of hinge 1; closed and open are unchanged, in the travel poses joint 0 moves by up to about 0.05 m; shaft angle = travel of the bottom roller / 0.08 m (open 1994.9 degrees as before) |
| `dimensions` | 2.861 / 2.62 / 2.5 | 2.85 / 2.675 / 2.5 (ceiling plane; wall plane -0.075 to track end 2.6) |
| `materials` | 3 | + `epdm` (EN ISO 10456: 1150 kg/m3, 0.25 W/mK, 1000 J/kgK) |
| `provenance.references` | 8 | + `en-12604`, `door-hardware`; `notes` rewritten for the new kinematics |
| `thermal` `track-steel` `areaM2` | 3.18 | 4.2 (measured 4.17 on the 1:50 model: tracks 2.46, stiffening angles 0.67, spreader 0.58, brackets, plates, hangers, feet, braces 0.46; factor 1.32, inside C16) |
| `thermal` `spring-assembly` | no `areaBasis` | `areaBasis: "exchange"` (C19 names a coil: 0.67 of the 1.03 m2 is wire surface that no scale coarser than 1:50 can show; `areaM2` unchanged, the 1:50 model measures 1.02) |
| 2D, all four scales, all three views | see audit | redrawn from the same numbers as the solids: section A-A with the true panel sections, hinge leaves, 46 mm rollers in the 52 mm track band, brackets, stiffening angle, hangers, feet, brace, cut spreader, bearing plate, drum, cone, coil, shaft; front and plan as bone-frame profiles with lateral spans (`solid: extruded`), so they are projections in every pose; front is the inside view; `decisions` rewritten |

Print profile: every scale resolves `authored` (min feature 0.29 / 0.22 / 0.20 / 0.20 mm, min stroke 0.1 mm).

## 4. Model per scale

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 78 | 86 | 79 |
| 1:100 | 42 | 42 | 46 |
| 1:200 | 24 | 24 | 21 |
| 1:500 | 10 | 13 | 17 |

- 1:50 body: four panels as extrusions of their true section (rib, nose as an arc of 52 mm about the pin, socket of 53 mm behind the lip, 8 mm shadow groove), 8 folded end stiles, bottom rail, hollow EPDM bulb, top cap; per joint four hinges (cranked upper leaf with eye, two lower leaves, pins; the side hinges' pin is the roller stem), 10 rollers (stem, hub, wheel with rounded tread), bottom and top carriers, cable studs; channel tracks with lips (vertical, quarter circle in three revolves, horizontal), jamb brackets, flag brackets, stiffening angles, hangers, ceiling feet, braces with feet, spreader channel; bearing plates, flanged bearings, shaft, grooved drums, 122 coil turns, stationary cone on the anchor bracket, winding cone with sockets.
- 1:50 thermal: per panel inner skin 0.5 mm, core, outer skin 0.5 mm as nested profiles; stiles, rail, cap, seal on the panel region with the glued and interior faces marked (C25); hardware, track frame, spring line by reference. Measured outer areas 5.61 / 0.376 / 4.17 m2 against 5 / 0.43 / 4.2.
- 1:50 structure: each panel a hollow tube of its two skins plus stiles, hinge leaves and carriers as members, pins, stems and studs as joints, track frame as members, bearings as the shaft joints, panel masses and the spring mass, supports as the foot anchors in the jamb and the ceiling anchors.
- 1:100: simplified panel section (chord nose, plain socket), seal block, one leaf pair per hinge, wheels only, plain channels, spring as 59 double turns between collars; structure with skin tubes, bearing rings, bolts.
- 1:200: slabs, solid track bars, 30 mm hanger and brace bars, slender spreader, shaft and hinge-line rods, folded plates kept; supports as 30 mm blocks.
- 1:500: slabs, track bars of true 52 x 30 mm, slender spreader and shaft, drums; hinge lines only in thermal and structure; supports as 75 mm blocks (foot fixing, hanger posts from track to ceiling).

Final check: `volumes-check.ts components/sectional-garage-door --product` PASS (91 s); silhouette IoU elevation / front / plan 0.999 / 0.998 / 0.983 (1:50), 0.999 / 0.996 / 0.973 (1:100), 0.991 / 0.998 / 0.979 (1:200), 0.995 / 1.000 / 1.000 (1:500); area totals 1:200 0.16, 1:500 0.23. `r3-validate.ts components/sectional-garage-door --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`. Spliced; catalog `volumes` equal the source.

## 5. What I saw in the final renders

- 1:50 body, six-view sheet and three-quarter from inside (turn 120): reads at once as a sectional door: inner face with four hinges per joint, roller carriers top and bottom, stiles, bulb seal, channel tracks through the curve, stiffening angles, hangers with feet and braces, spreader, shaft with grooved drums, spring between cones, anchor bracket and bearing plates on the wall plane. Left and right side: curve in about nine chords (kernel tessellation), brackets on the wall plane. Back: rib and shadow grooves on the outer face.
- 1:50 poses: `half-open` and clip `open-close` at 9 s: panels round the curve outside the track, joints open on the outside only, rollers stay in the band; `open`: leaf lies above the horizontal tracks between the angles, clear of hangers, braces and spreader.
- 1:50 thermal cut: skin line, core, hardware green, frame violet, spring yellow. 1:50 structure: skin tubes, leaves, orange pins and stems, magenta panel masses and spring, green anchors.
- 1:100, 1:200, 1:500 (`volumes-render.ts` sheets of the final state, `r3-sheet.ts` body 1:100): each a recognisable door with fewer parts; overlays green with single red cells at bracket corners.
- 2D 1:50, 1:100, 1:200, 1:500 (`r3-2d.ts`), close-up of the joint at 1:50: nose, groove, tongue, hinge eye with both leaves and the roller in the band are drawn.

## 6. Still imperfect

- The lift cables are not modelled at any scale: their length changes with the pose and a solid follows one bone (`author-requests.md`, 2026-11-20). The drums therefore carry no cable; the pull is the load pair of the poses.
- The curve radius 0.5 m and the horizontal track 0.5 m above the door top are the asset's declared design (a high-headroom installation), not the residential standard (0.3 to 0.38 m radius, about 0.2 m headroom); changing them means a new travel path for all poses and was not done.
- Hinge pin and roller stem are coaxial at the side hinges (real carriers set the stem 10 to 20 mm off the pin); needed so that the solved poses keep every roller exactly on the path.
- The track curve shows about nine chords in shaded views (kernel segments per revolve; split into three revolves at 1:50, two at 1:100).
- In the plan the hinges are drawn over the panels in every pose (painter order is fixed per shape), which is right for the closed door and wrong under the open leaf.
- 1:200 structure leaves out brackets and bearing plates (they stay in the body): the validator's body mask misses folded sheet seen edge on (`tool-requests.md`, 2026-11-20, with two more findings: slender centre line of a bent bar, `r3-validate.ts` green on unsuitable representations).
- No track-frame mass solid: its centre of mass lies in the void between the tracks.
- Screws and bolts below 7.5 mm, handle and lock are not modelled.

At 1:200 the three hinge axes are drawn with a 0.20 mm pen, and the hangers, braces and tracks use at least 0.15 mm. At 1:500 the visible panels and track edges use at least 0.18 mm. The whole asset passed its paper endpoints through 1:1000. All four physical panels and their real gauges remain in the model. Both affected scales passed silhouette and cross-scale checks.
