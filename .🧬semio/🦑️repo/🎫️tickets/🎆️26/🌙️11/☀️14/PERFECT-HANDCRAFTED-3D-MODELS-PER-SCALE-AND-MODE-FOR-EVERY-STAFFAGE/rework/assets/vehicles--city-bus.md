# vehicles/city-bus — rework report

## Pass 1 (second round), 2026-11-22

No earlier report existed for this asset and no batch report `T/rework/*.md` mentions the slug. Source `T/volumes/vehicles/city-bus.json` and catalog `asset.volumes` were byte-equal in content at the start (all four scales, all three lists), so there was no half-written state to reconcile.

### Reference table (12 m low-floor battery-electric solo city bus, class Citaro / Urbino 12 / Lion's City 12 E)

| quantity | real | model (rig, solids, sheet) | verdict |
|---|---|---|---|
| length over body | 12.00 … 12.20 m | 12.00 m (`body` bone 12 m, nose profile to x = 12.000) | ok |
| width over body / over mirrors | 2.55 m / about 2.95 m | 2.55 m (walls at z = ±1.275) / 2.95 m | ok |
| height roof skin / over roof units | 2.95 … 3.05 m / 3.3 … 3.4 m | 2.98 m / 3.30 m | ok |
| wheelbase | 5.90 m | 9.35 − 3.45 = 5.90 m | ok |
| overhang front / rear | 2.70 … 2.80 m / 3.30 … 3.45 m | 2.65 m / 3.45 m | ok (front 5 cm short of the usual band) |
| tyre 275/70 R 22.5 | diameter 0.96 m, section 0.276 m, rear twins at 0.31 m pitch | radius 0.48 m, front single with outer face at 1.19 m, rear twins separate at 1:50…1:200 | ok |
| floor height at doors / kneeling | 0.32 … 0.34 m / 60 … 80 mm | 0.34 m / 70 mm (`axle-f` dy) | ok |
| door clear width / height | 1.20 … 1.25 m / 1.95 … 2.05 m (two leaves) | 2 × 0.625 m / 2.06 m | ok |
| belt line (lower window edge) | 0.95 … 1.10 m | 1.10 m | ok, upper end |
| tumblehome above the belt | 30 … 50 mm | 35 mm | ok |
| windscreen rake | 5 … 10° | 7° | ok |
| skirt clearance | 0.27 … 0.32 m | 0.30 m | ok |
| kerb mass | 12.5 … 13.5 t | 12 580 kg | ok |
| roof battery / heat pump | 6 … 7 m × 2.0 m × 0.3 m / 2.5 m × 2.0 m × 0.25 m | 6.3 m / 2.5 m long, 0.32 m high | ok |

### What I looked at (rendered and read at full size)

`r3-sheet` 1:50 body (six views, three-quarter, front and back close-ups, pose `kneeling-at-stop`), 1:50 thermal (plain and `--cut`), 1:50 structure, 1:100 body, 1:200 body, 1:500 body, 1:500 thermal, 1:500 structure.

### Judgement of the state found

- **Rig**: `body` (fixed) with `battery`, `hvac`, four prismatic door leaves (0.60 m centre, 0.55 m front travel), two prismatic axle bones (±0.12 m air spring travel), two revolute wheel bones. Wheelbase, door places and joint limits match the table. The rig is planar: one wheel bone per axle carries both sides, there is no steering pivot, and kneeling is a pitch of the root (−0.68°) with `axle-f` dy 0.07 m instead of a roll towards the kerb.
- **Body 1:50** (213 solids): a real shell (side panels with arches and door notches, leaning window frame, arched roof, curved nose and tail on the plan outline, header with lip, raked curved windscreen, panes with transoms, wipers, lamps, plates, mirrors on arms, interior, 275/70 R 22.5 wheels with hand holes and nuts, dropped front beam, portal drive axle, air springs, roof units). Proportions agree with the table from all six directions; nothing floats or pokes through in the default and the kneeling pose; the leaves slide outside the wall.
- **Thermal 1:50** (166 solids): nested skin and lining plates, double panes with gap, door region, roof units, drive axle, tyres, brakes; the sheet reads as the same bus and the cut shows the nesting.
- **Structure 1:50** (189 solids): welded tube frame (longerons, sills, ring frames at every post, waist and cant rails, door portals, A-pillars), roof unit rails, axle beams with air springs and king pins, wheels with hubs, masses at the centres of mass.
- **1:100 / 1:200 / 1:500**: each is modelled for its feature size (wall thickness 60 / 80 / 100 mm, glazing per bay → bands, twins merged at 1:500, ring frames 8 / 5 / 3), same bones and ids.

### Defects found (measured, `r3-validate.ts vehicles/city-bus --volumes` at the start: INVALID, 15 findings, all `volumes-connected`)

The form was right, but many parts hung in the air (full list from `rework-10-connected.ts`):

- **Body 1:50**: wheel discs 25 mm clear of the hubs (bore radius 105 mm, hub 80 mm), brake discs loose around nothing, rear twin wheels 5 mm clear of the spindle, console, driver's seat and steering wheel rim floating, kerb-side mirror arm ending 10 mm before the wall, all four door leaves hanging 3 mm outside the wall with nothing carrying them.
- **Body 1:100**: same hubs; both axle assemblies ended under the wheelhouses without spring seats. **Body 1:200**: front beam 74 mm short of the wheels, no springs at all between axles and body, leaves loose. **Body 1:500**: beams short of the wheel discs, no connection axle to body, both mirror ears floating in front of the window frame.
- **Structure, every scale**: roof unit rails 8 … 450 mm above or beside the roof bows, door leaf frames with no header to hang from, air springs ending in the air (no member between spring head and frame), axle beams short of springs and hubs; at 1:50 also bumper beams, screen sill and header beams, rear corner posts not meeting the frame.
- **Rig (not changed)**: the front leaf of the front door slides 0.55 m forward and so runs past the nose in `kneeling-at-stop` (seen in the right-side render); a real front door of this class swings in or slides only backwards. One wheel bone per axle, no steering pivot; kneeling is a pitch, not a roll to the kerb.
- **Tool**: `r3-sheet --list thermal` draws solids without `layer` of a region that has layers at finer scales in grey (1:500 thermal sheet all grey except roof units and tyres), so the coarse thermal sheets cannot be read by region.

### What I rebuilt

Scripts (hand-chosen numbers, idempotent): `T/rework-city-bus-body-joints.py`, `T/rework-city-bus-structure-path.py`.

- **Body 1:50** (213 → 219 solids): hubs remodelled as real hubs (pilot through the disc bore at r 105 mm, bolting flange r 200 mm behind the disc, barrel r 110 mm on the stub axle / spindle carrying the brake disc; rear hub passes both twin discs); guide rail over each doorway (`door-track-m` x 5.05 … 8.125 m, `door-track-f` x 9.65 … 11.47 m, y 2.385 … 2.445 m) from which the leaves hang over their whole travel; `dashboard-plinth`, `driver-seat-pedestal`, dished `steering-wheel-hub`, `mirror-base-r`.
- **Body 1:100** (123 → 131): the same hubs for 15 mm features, six spring seats, guide rails.
- **Body 1:200** (79 → 97): stub axles, one air spring cylinder per side at the front, carrier with two cylinders per side at the rear, six spring seats, guide rails.
- **Body 1:500** (40 → 46): axle beams lengthened into the wheel discs, one suspension strut per side and axle up into the wheelhouse arch, a short arm for each mirror ear.
- **Structure 1:100** (134 → 154), **1:200** (100 → 106), **1:500** (73 → 85): roof unit rails lowered onto the roof bows; `door-header-m` / `door-header-f` spanning from post to post and carrying the leaf carriages; top and bottom rails of the leaf frames at 1:100; spring carriers as members at 1:100; inclined `spring-strut-*` from the longerons to every spring head; front air spring joint lengthened onto the dropped beam at 1:200; a fourth ring frame at x = 8.25 m at 1:500 (under the heat pump rails and at the end of the centre door header).
- **Decisions** (en, de) extended at all four scales for body and at 1:100, 1:200, 1:500 for structure.
- Thermal lists, rig, poses, clips and 2D drawings: not changed in this pass.

### Proof

- After the body work: `rework-10-connected.ts` reports no body finding at any scale; the full `r3-validate.ts vehicles/city-bus --volumes` went from 15 findings to 3, and after the 1:100 structure block `--scale 100 --rules connected` is `VALID` (partial) and `rework-10-connected.ts --scale 100` prints `CONNECTED`. 1:200 and 1:500 passed every rule in the full run.
- Last scoped run `--volumes --scale 50 --rules silhouette,connected`: **INVALID, 2 findings** (below). A full unscoped run after the last two edits (1:100 structure block, mirror base) was not made.
- Published with `volumes-insert.py` (`volumes spliced: 1:50 … 1:500 authored`); `r3-validate.ts vehicles/city-bus --catalog-only`: `VALID … schema 0, validateAsset 0, checkVolumes not run`. Published although not `VALID` with `--volumes`, because the catalog held the state with 15 findings and source and catalog must agree.
- Rendered and read after the changes: 1:50 body from below (turn 300, tilt −20), 1:50 `kneeling-at-stop` right side, 1:500 structure sheet, 1:200 structure three-quarter. Not re-read after the changes: the other poses, the clips, 1:100 and 1:200 body, all thermal sheets, the 2D drawings.

### What remains imperfect (for the next pass)

1. **1:50 body, front drawing**: `volumes-silhouette` p95 31.9 mm (limit 30.0), largest distance 39.4 mm. New in this pass; not caused by the guide rail thickness or the mirror arm (both varied, number unchanged). Suspects: hub flange / barrel seen under the floor between the twin tyres, guide rail height band y 2.39 … 2.445 m outside the leaning wall. Either redraw the front elevation (rail, hubs) or find the part with `r3-project2d --view front --check`.
2. **1:50 structure**: `volumes-connected`, 17 loose groups: `bumper-beam-front`, `screen-header-beam`, `screen-sill-beam`, `bumper-beam-rear`, `rear-header-beam`, `rear-corner-post-l/r` (≥ 45 mm from the frame), `axle-f-beam` with king pins 34 mm from `axle-f-spring-l/r`, `axle-r-beam` and carriers from their springs, `hvac-rail-*` and `battery-rail-*` with their mounts (17 … 29 mm above the bows), all four leaf frames (3 mm from the door headers), wheel hubs. Same recipe as at 1:100 (rails onto bows, struts to spring heads, carriages into the headers) plus joining the end beams to the A-pillars and corner posts.
3. **Rig**: front door front leaf leaves the body when open (travel 0.55 m forward of x = 11.45 m); needs an inward-swinging or rearward-sliding front door, with poses, clips and 2D leaf shapes rebound. No steering pivot, one wheel bone per axle, kneeling as pitch instead of roll.
4. **2D drawings**: guide rails, mirror arms at 1:500 and suspension struts are not drawn; 1:100 … 1:500 pass the silhouette rule without them.
5. **Thermal**: new exposed body parts (guide rails, 1:500 mirror arms) have no thermal solid; coverage passed at 1:200 and 1:500 in the full run, 1:50 and 1:100 passed too, but the panels region does not include them.
6. Poses `braking`, `arriving`, `pull-away-8`, `pull-away-18`, `at-stop-level` and the three clips were not rendered and read in this pass.

### Fundamental issues (outside this asset)

- `r3-sheet.ts --list thermal`: layerless solids of a layered region are drawn grey and have no legend entry (coarse scales unreadable).
- `volumes-silhouette` names the view and p95 but not the place or the solids of the largest boundary distance; with one number it is not possible to tell which part to redraw.
- Host load (load average about 29): a full `r3-validate --volumes` of this asset took over ten minutes, a single sheet 30 … 60 s.
