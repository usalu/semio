# Round 3: appliances/vacuum-cleaner

State on 2026-11-20: finished and spliced. No earlier round-3 work existed for this asset. It was rebuilt from the roots: rig, poses, anchors, dimensions, three thermal areas, provenance, all 2D drawings and all volumes. Everything below describes the catalog file as it is now.

Scripts in the ticket folder:
- `r3-vacuum-cleaner-kit.py`: the measures, the canister's plan-section table, outlines and solid builders.
- `-poses.py`: solves the hose route of each pose as a hanging hose, with clearances.
- `-model.py`: states every solid of every scale and list by hand and writes `volumes/appliances/vacuum-cleaner.json`.
- `-draw.py`: draws the 2D of every scale and view from the same measures and holds the representation decisions.
- `-roots.py`: builds the candidate document `🗑️generated/r3-vacuum-cleaner/🔣️.json`.
- `-apply.py`: writes only the changed root members into the catalog file, in its own formatting, plus the height and length in the catalog index.
- `-build.sh`: model, roots, `--file` validation, then apply, `volumes-insert.py`, catalog validation and print suitability.
- `-probe.ts`: prints the bone frames of a pose or clip.
- `-suit.ts`: print-suitability probe adapted from `r3-city-bicycle-suit.ts`.

## 1. Reference

| Quantity | Real subject | Source | Model now |
| --- | --- | --- | --- |
| Canister L x W x H | 0.496 x 0.284 x 0.227 m; 0.469 x 0.315 x 0.255 m; 0.367 x 0.290 x 0.237 m | Miele Complete C3, retailer data (thebrick.com); Blue Vac Power data sheet (d.otto.de); Bosch spec sheet (BSH), read 2026-11-19 | 0.50 x 0.25 m shell (0.30 m over the wheels); lid top 0.245 m, carry handle 0.282 m |
| Hose | 1.5 to 1.8 m (Blue Vac 1.70 m), OD about 36 to 40 mm, corrugated, rigid cuff at both ends | otto data sheet; product knowledge | 10 x 0.15 m = 1.5 m, OD 36 mm, corrugation ring every 20 mm, cuff in the socket, end fitting on the handle swivel |
| Telescopic wand | 0.6 to 0.95 m (Blue Vac 0.90 m), tubes 35 mm (standard), inner tube about 32 mm, locking collar | otto data sheet; Miele 35 mm system | outer tube 35 mm to 0.30 m, collar with lock, inner tube 32 mm to 0.74 m |
| Handle | about 0.2 to 0.3 m, air slide, hose swivel | product knowledge | 0.20 m sleeve, 8 mm air slide, swivel 40 mm |
| Floor nozzle | 285 mm wide (Miele SBD 285-3), about 30 mm head, 70 to 85 mm deep, elbow about 0.12 to 0.15 m, pivot behind mid-depth, bristle strips, rubber end caps | Miele SBD 285-3 retailer listings (coolblue, galaxus, manomano); product knowledge | 0.285 x 0.083 x 0.034 m incl. strips, elbow 0.14 m, ball pivot 47 mm above the floor in a yoke, two 6 x 8 mm strips |
| Hand height in use | about 0.9 to 1.0 m | ergonomics, own knowledge | handle swivel at 0.965 m, top 0.987 m |
| Rear wheels / caster | 0.12 to 0.15 m wheels at the rear corners, partly within the body sides; swivel twin caster at the front | product knowledge | 0.12 m tyre rings on dished rims at x -0.17, z +-0.1375; twin 50 mm caster |
| Exhaust | filter grille on the rear face | product knowledge | 94 x 156 mm grille with five slots |
| Mass | 4 to 6 kg class | EN 60312-1 (asset reference) | 5.1 kg unchanged; centre of mass of the canister moved to the motor at the rear |

## 2. Audit of the state before round 3 (catalog of 2026-11-19)

- **Rig**
  - Hose: 10 x 0.11 m = 1.1 m, a third shorter than real.
  - Handle and wand: the wand started at the handle's own origin (`wand.x` 0), so handle, wand and nozzle elbow were squeezed into 0.78 m. The handle sat at 0.74 m, about 0.25 m lower than in real use, and the nozzle joint was the wand tip at 0.047 m with no elbow.
  - Nozzle bone 0.28 m long.
  - Rear wheels at x -0.12, mid-body instead of at the rear corners.
- **Poses**
  - Running: hose-7 crossed the wand (13 mm between the axes, 36.5 mm needed). The hose arrived at the handle folded back along the wand: a 195 degree turn at the grip joint.
  - Parked: the hose ran straight through the canister at a height of 0.11 to 0.2 m.
- **Nozzle (2D and 3D)**: a 0.28 m deep x 0.28 m wide x 0.095 m block, a square in plan. The real head is 0.083 m deep.
- **Canister**: 0.305 m high to the handle (class 0.23 to 0.26 m). The handle was 80 mm wide (real 30 to 35 mm). The bumper box sat across the hose socket. The wheels cut 10 mm into the shell.
- **Solids**: 41/25/16/16 body solids.
  - The 1:50 shell showed banding (13 sections) and the grip loft twisted (sections without a fixed rotation).
  - Hose rings were open revolves filled through the axis (`kernel-traps.md`, th-tyre).
  - Thermal had no honest emitter: a duct solid floated inside the shell.
  - Structure was a 0.4 x 0.1 x 0.2 m block as the canister "chassis" (qa-visual: thermal C, structure C at 1:50).
- **Thermal areas**: `hose` 0.12 (1.1 m tube), `lid` 0.15 (the dome with handle measures 0.17), `brush` 0.01 (a fringe), `grip` 0.025.
- **Dimensions**: `heightM` 0.76 and `widthM` 1.50, both following the too-short wand and the 0.28 m nozzle.

## 3. Root fixes (catalog file, outside `volumes`)

| Member | Old | New |
| --- | --- | --- |
| `dimensions` | height 0.76, width 1.50, depth 0.30 | 0.99, 1.35, 0.30 (mass 5.1 unchanged); catalog index heightM / widthM updated |
| `rig.wheel.x` | -0.12 | -0.17 |
| `rig.hose-1..10` | lengthM 0.11, x 0.11, comX 0.055 | 0.15, 0.15, 0.075 |
| `rig.grip` | x 0.11, lengthM 0.14, comX 0.07, title "Grip sleeve with air slide" | 0.15, 0.20, 0.10, "Handle with air slide on the hose swivel" |
| `rig.wand` | x 0, lengthM 0.78, comX 0.39 | 0.18, 0.86, 0.40, title "Telescopic wand with nozzle elbow" |
| `rig.nozzle` | x 0.78, lengthM 0.28, comX 0.05, comY -0.01 | 0.86, 0.05, 0.005, -0.022 |
| `rig.body.mass` | comX 0, comY 0.14 | -0.05, 0.10 (motor and cord reel at the rear bottom) |
| `anchors` | nozzle-contact x 0.05, y -0.045, footprint 0.2 x 0.28; exhaust-port -0.262 / 0.14; footprint widths wheel 0.05, caster 0.02 | 0.0045, -0.047, 0.083 x 0.285; -0.258 / 0.121 (centre of the grille); 0.05, 0.024 |
| `poses` | hose angles crossing wand and canister | solved by `r3-vacuum-cleaner-poses.py` (bending plus weight, floor contact, 45 mm from wand and handle, 10 mm from the canister). The grip is held where the wand needs it, so nozzle positions, wand angles and both clips are unchanged. Hose leaves the handle 40 degrees off the wand axis (pulled stroke 70; parked on the canister side). Max joint turn 40 / 56 / 33 degrees (running / pulled / parked) |
| `thermal.regions` areaM2 | lid 0.15, hose 0.12, grip 0.025, brush 0.01 | 0.17, 0.18, 0.03, 0.014 (each within factor 1.5; hand: hose 1.5 m x pi x 0.036 = 0.170 + cuff and end fitting 0.009; lid dome 0.155 + handle 0.015; handle sleeve 0.026 + swivel; two strips 0.25 x (2 x 0.008 + 2 x 0.006) = 0.014) |
| `provenance` | 4 references, old notes | + `miele-complete-c3`, `otto-blue-vac-power`, `miele-sbd-285-3`; notes rewritten (en, de) |
| `representations[*].geometry` and `decisions` | old drawings | redrawn at every scale and view from the same measures (ruling E1); decisions rewritten per scale (en, de) |

Summary, pose and clip descriptions remain true and are unchanged; structure supports and thermal operating data are unchanged.

## 4. What was modelled

### 1:50: body 55, thermal 43, structure 41
- **Body**
  - Canister: shell and lid lofted through 15 + 12 horizontal superelliptic plan sections (exponent 3) from a key table of rear x, front x and half width over height. The front slopes into the lid; the rear face is vertical for the grille.
  - Fittings: grey bumper band round the parting line; carry handle 32 mm wide with a 17 mm finger opening; two foot buttons; socket ring with the hose cuff locked in it; exhaust grille with five through slots.
  - Wheels: rear tyre rings on dished rims with hub caps; twin swivel caster in its fork.
  - Hose: ten 36 mm tubes with corrugation rings (texture, 7 per segment) and the end fitting.
  - Handle: shaped sleeve whose loft sections are now rotation-fixed, on a 40 mm swivel, with the air slide.
  - Wand: outer tube, locking collar with lock button, inner tube.
  - Nozzle: elbow neck on the wand bone, ball pivot in a two-cheek yoke, head extruded from its real side profile, rubber end caps following that profile, two bristle strips.
- **Thermal**
  - Body-surface references for shell, bumper, socket, tyres, rims, casters, hose parts, wand parts, handle parts, nozzle parts and strips.
  - The lid is its own open-based dome, so its seat on the shell does not count.
  - The grille is a plate whose back face against the shell is a C25 b face (`sectionFaces +u`).
  - Exhaust-air emitter: a 90 mm outlet patch 2 mm proud of the grille (D2).
  - There are no hidden regions: the asset's thermal data put the motor and bag heat into shell, lid, grille and air.
- **Structure**
  - Base tub moulding plus its 3 mm side wall (hollow) as the canister's load path.
  - 10 mm steel axle (slender), hub bearings, caster stem and pin.
  - Ten 0.6 mm hose walls with joint at each hose joint.
  - Handle tube with swivel joint; outer and inner wand tubes and the elbow.
  - Hollow 2.5 mm nozzle housing with its pivot pin.
  - Supports: contact strip under both tyres, caster tread, both bristle strips.
  - Masses: canister (rear), wand, nozzle, handle.

### 1:100: body 25, thermal 26, structure 26
- Shell and lid with 8 + 6 sections; handle as a solid bridge; socket and cuff as one collar; 15 mm grille plate.
- Each tyre with rim is one revolve; the twin caster is one roller.
- Hose tubes; simplified handle loft; one wand tube; simplified elbow.
- Nozzle profile across the full width, with the two strips.
- Thermal as at 1:50 on these solids.
- Structure: tub with wall, axle, hose and tube walls, hollow nozzle; joints only at the wheels, handle and pivot.

### 1:200: body 19, thermal 24, structure 24
- One canister loft with the handle block and a 30 mm socket.
- 30 mm wheel discs; no caster (it is not drawn at this scale).
- Hose capsules; handle and wand as plain tubes at true size.
- Nozzle block with its ball.
- Thermal-only shell and lid skins under the merged body, grille plate, both strips and the outlet.
- Structure: base plate plus hollow wall, hollow tube members, solid nozzle block.

### 1:500: body 15, thermal 21, structure 22
- One canister loft without the handle.
- Rear wheel pair as one 0.30 m roller (the 35 mm gap under the canister is below 75 mm).
- 60 mm hose, handle and wand tubes, the same in 2D for print; 75 mm nozzle block.
- Thermal keeps true-size skins: hose 36 mm, wand, handle, tyres, nozzle head, strips, grille, outlet.
- Structure: hollow tub wall, axle, solid 60 mm members, nozzle block, supports and masses.

Decisions: three bilingual entries at 1:50 and two at each coarser scale.

## 5. Checks (final, on the catalog file)

- `r3-validate.ts appliances/vacuum-cleaner --catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`. Catalog `volumes` equal the source.
- `volumes-check.ts appliances/vacuum-cleaner --product`: `PASS (7.7 s)`.
  - Silhouette IoU (elevation / front / plan): 1:50 0.998 / 0.987 / 0.994; 1:100 0.997 / 0.985 / 0.993; 1:200 0.998 / 0.991 / 0.995; 1:500 0.995 / 0.983 / 0.986.
  - Bounds within 6 mm at every scale.
  - Area deviation at 1:50:

    | Region | Deviation |
    | --- | --- |
    | body-shell | 0.038 |
    | lid | 0.026 |
    | exhaust | 0.003 |
    | wheels | 0.040 |
    | hose | 0.002 |
    | wand | 0.093 |
    | grip | 0.021 |
    | nozzle | 0.037 |
    | brush | 0.014 |

  - Totals at 1:200 and 1:500 are within 30 %.
- Print suitability (`r3-vacuum-cleaner-suit.ts`): all four scales `authored`, no violations. Two violations found during the work were fixed:
  - The twin caster gap at 1:50: wheels moved to z +-0.018.
  - The rear wheel gap at 1:500: the wheels became one roller.

## 6. What the renders show (all opened with the Read tool)

Seen:
- `r3-sheet` body at 1:50 (with rig and without), 1:100, 1:200 and 1:500, six views each.
- 1:50 three-quarter close-up (turn 30, tilt 20).
- 1:50 `parked`, six views.
- Clip `floor-stroke` at t = 1 s, right side.
- Thermal with cut at 1:50 (six views) and 1:100 three-quarter.
- Structure at 1:50 (six views) and 1:500 three-quarter.
- `r3-2d` at 1:50 (line and solid) and 1:500 (solid).

What they show:
- **Front**: the canister section with its domed lid, handle and bumper between the rear wheels, the socket ring with the cuff, and in front of it the hose, wand and the full 0.285 m nozzle.
- **Right side**:
  - The canister reads as a real canister: low front, sloping lid, handle, bumper at the joint, rear wheel with a dished rim, caster, socket and cuff.
  - The hose lies on the floor and rises in a loop 45 mm clear of the wand into the handle top. The handle sits at hand height above a 35 / 32 mm wand with its collar.
  - The elbow and the ball pivot stand on a flat 83 mm nozzle head.
- **Back**: the slotted exhaust grille in the rear face, the wheels and the wand behind.
- **Top**: the rounded canister with handle, two foot buttons and the bumper rim; wheel strips; the hose, handle and wand on the centre line; the nozzle across.
- **Three-quarter**: reads at first sight as a canister vacuum cleaner. Bristle strips and end caps are visible on the nozzle; the corrugation is visible.
- **Parked**: the wand stands behind the canister on its nozzle; the hose arches over the canister into the handle. No crossing.
- **Clip frame**: hose clear of floor and wand.
- **Thermal**: red shell, green lid, purple grille, yellow wheels, cyan hose, magenta wand, green handle, blue nozzle.
- **Structure**: an open tub with its wall (hollow), the axle, the hose and tube chain, the nozzle housing, supports and masses. The 1:500 version is coarse but a load path, not a block.
- **2D**: clean outlines at every view; 1:500 is minimal but legible.

## 7. Still imperfect

- **Hose kinks**: the hose is ten rigid 0.15 m segments in a planar rig, so the bends show as knuckles: 38 to 40 degrees at the floor in running, 56 degrees in the pulled stroke. A real hose would also swing out of the symmetry plane, which this planar rig cannot do. More hose bones would smooth it; that was not done, to keep the rig's ten-segment hose.
- **Hose route**: in the running pose the hose rises close to the wand (45 mm clear) in a forward loop. With 1.5 m of hose in one plane between the canister and the wand there is no slack-free alternative.
- **Faceted wheels**: wheels and the caster appear 12-sided in the 1:50 renders. That is the shared tessellation quality, which the asset cannot set (no segment count in the solid schema).
- **No motor or dust bag**: they are not regions in the thermal data, which deliberately distribute the motor heat to the shell, lid, grille and exhaust air. Adding a hidden motor region would change the thermal balance, so they are not in the thermal cutaway. The motor is represented as the canister mass at the rear in structure.
- **1:500 hose below the floor**: the 60 mm hose touches 18 mm below the floor where the real 36 mm hose rests on it. The 1:500 drawn height reads 1.00 m against 0.99 m.
- **1:500 supports**: the support pads are 75 mm boxes, as wide as the scale needs; they look heavy in the 1:500 structure render.
- **Parked hose side**: in the parked pose the hose leaves the handle on the canister side (the handle turned about the wand). The handle solid is nearly symmetric, so this reads, but the air slide then faces the canister.
- **Exposure deviations**: reported by the check as information for the area reviewers (C29 item 2): for example hose 0.78, because the hose lies on the floor and passes close to the wand.
- **Not run**: the repo test suites; only the checks of the brief.
- **Index**: the catalog index `distinction` still says 520 W of warm air (the asset says 543 W). It was untouched before this round and is not mine to rewrite; noted only.

## Round 5 handcraft

Scripts: `r5-vacuum-cleaner-model.py` (keeps the hand-stated lists of `r3-vacuum-cleaner-model.py` and rebuilds the hose of every scale), `r5-vacuum-cleaner-roots.py` (2D and range roots), tool fix in `r3-lib.ts` (`weightsOf`, see `tool-requests.md`). Renders in `🗑️generated/r5-vacuum-cleaner/{before,after}/` (opened: body, structure and thermal cut at every scale, the 1:50 side view at full size, poses `parked`, `running-pulled`, clips `floor-stroke` t 1 and `run-and-park` t 1200; after the tool fix all re-rendered and re-opened).

### Reference and measurement (1:50)

| Quantity | Reference | Source | Model | Deviation |
|---|---|---|---|---|
| Canister L x W x H | 0.37-0.50 x 0.28-0.32 x 0.23-0.26 m | Miele Complete C3, Bosch, Blue Vac data | 0.50 x 0.25 (0.30 over wheels) x 0.245 | in class |
| Hose | 1.5-1.8 m, OD 36-40 mm | otto data sheet | 1.5 m, OD 36 | in range |
| Wand | 0.6-0.95 m, 35 / 32 mm | Miele 35 mm system | 0.74 m extended, 35 / 32 | in range |
| Floor nozzle | 285 mm wide | Miele SBD 285-3 | 285 mm | 0 % |
| Hand height | 0.9-1.0 m | ergonomics | 0.965 m swivel | in range |
| Rear wheels | 0.12-0.15 m | product | 0.12 m | in range |

### Grades (body / thermal / structure)

| Scale | Before | After |
|---|---|---|
| 1:50 | B (hose a chain of ten straight segments with 38-56 degree knuckles) / A- / A- | A- / A- / A- |
| 1:100 | B (same knuckles) / A- / A- | A- / A- / A- |
| 1:200 | B / B+ / A- | A- / A- / A- |
| 1:500 | B (60 mm hose knuckles) / B+ / B+ | A- / A- / B+ (hose members still per bone) |
| Poses/clips | B (knuckles in every pose) | A- (the hose bends through every joint in three steps) |
| Catalog | INVALID (print slivers of hose-4/5/6 at 1:20, gap 1:800-1:1000) | VALID |

### Changes

- Hose at every scale: each 0.15 m segment keeps a rigid middle on its bone and every joint gets two 25 mm bend pieces blended over both bones (weights 2/3 : 1/3 and 1/3 : 2/3), so each joint turns in three steps; at 1:50 the corrugation ribs follow at 25 mm pitch on the middles and the bend pieces. Thermal hose follows (references at 1:50 to 1:200, own true-size skins at 1:500). The rig is unchanged (ten hose bones, poses, clips).
- `r3-lib.ts` now blends weighted solids like the product (it showed them rigid, sticking out of the joints).
- 2D 1:50: the hose is drawn 34 mm in the front view and plan, inside the 35 mm wand it lies along (its true 0.5 mm step printed as an unprintable sliver at 1:20); the side view keeps 36 mm. The 1:50 band starts again at 1:20 and the 1:500 band reaches 1:1000 (rule `valid-range-gap`).
- `r3-validate.ts` VALID; `volumes-check.ts --product` all ok except `volumes-thermal-coverage` on `body-shell` (0.293 measured against 0.42 declared at 1:50, red before round 5 too): the reviewers restored 0.42 as the whole canister, which the model shows as shell 0.293 plus lid 0.178; listed in `area-review.md` (round 5 entry), not applied. Spliced.

### Remaining

- `body-shell` area dispute (area reviewers).
- The bend pieces leave slight bulges on the outside of the sharpest bends (56 degrees in the pulled stroke); a finer hose rig would smooth them further.
- The hose stays in the symmetry plane (planar rig).
- Structure keeps one hose wall member per bone (the load path bends at the bone joints).

All scales retain a 36 mm flexible hose, 35 mm steel wand with a 33 mm bore, 285 mm floor head and separate swivel. The fine telescoping inner tube has a 32 mm outside diameter and 30 mm bore. The nozzle has two 250 × 8 × 6 mm brush strips. The canister wall exposes 0.293 m² and the separately modeled lid about 0.178 m². All four measured geometry checks and twelve independent export scenes passed (2,596,572 bytes).

## Rolling contacts, 21 November

At 1:500 the 300 mm rear drum filled the 250 mm gap between the inboard faces of the rear wheels. The front twin castor was one roller at 1:100 and disappeared from the body at 1:200 and 1:500. The structural view placed a rectangular support across the rear axle at every scale.

Each scale now has two 120 mm diameter rear wheels, with 25 mm tyres and dished rims, on 275 mm centres. The 50 mm diameter twin castor uses two 12 mm wheels on 36 mm centres. Four anchors carry the rolling reactions alongside the separate floor-tool contact. The body, thermal and structural meshes have a clear gap between the left and right treads. Rims connect the rear treads to the axle, and the front wheels seat against their yoke.

The 1:500 print side view marks the front castor. Front and plan use a grouped sign for the rear pair at the coarse end of the print band. The maintained Three.js regression measures floor-intersecting triangles, checks both tread widths and rejects a surface bridging an axle. It also probes each structural rear rim from the axle centre to the 44 mm tyre seat.

The revised canonical asset matched the fully validated candidate and the authored volume source. The canonical strict run returned zero schema, asset or measured-volume findings. The GLB reopened in 12 scenes at 2,365,104 bytes with zero findings. The four-scale render shows a solid rim between each structural rear tyre and its axle. Reviewed SHA-256 `28dd9d675ed27f426510ae9c3643732ee48a7c045e5fe946c0cda75f5a13e955`.
