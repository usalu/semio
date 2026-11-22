# appliances/tower-server — rework report

## Pass 1 (second round), 2026-11-22

State found: source `volumes/appliances/tower-server.json` and catalog `volumes` were identical (no half-written edit). Batch 16 had only added `th-psu-case` and `th-heat-sink` at 1:200 and 1:500. No per-asset report existed.

### Reference table (single-socket pedestal server, e.g. HPE ML110 / Dell T-series class; own knowledge)

| Feature | Reality | Model before | Model now |
|---|---|---|---|
| Case H x D x W | 430–450 x 400–480 x 175–220 mm | 450 x 450 (incl. 25 mm bezel, 12 mm rear panel) x 200 | unchanged |
| Feet | four rubber feet, 10–15 mm | 12 mm, 40 x 30 mm | unchanged |
| Bezel | ABS, 20–30 mm deep, mesh field, 5.25" bay 148 x 42 mm | 25 mm, mesh 160 x 300, bay 148 x 42 (1:500: 75 mm slab, no opening) | 1:500: intake opening with dark core behind; still 75 mm deep |
| Side cover | removable steel panel with release latch, tilts out on its bottom edge and lifts off | fixed solid of bone `case`, no bone | bone `cover`, hinge on the bottom edge, 0–90 degrees |
| PSU | ATX 150 x 86 x 140 mm, top rear, internals on the shell floor | shell + core floating 8 mm inside the shell | core stands on the shell floor |
| CPU cooler | tower 120 mm, heat pipes through the base plate | pipes 6 mm beside a 40 mm base (floating fin stack) | 68 mm base plate, pipes run through it |
| Mainboard | ATX 305 x 244 x 1.6 mm on 6.4 mm standoffs | floating 6.5 mm off the wall | nine standoffs (6 mm, 6.5 mm long) |
| Drive cage | 3.5" cage on the floor behind the front fan, 2 disks 26 x 147 x 102 mm | only in the structure list | body 1:50: cage + two disks on rails; 1:100: one block |
| Memory | 4 DIMM 133 x 30 x 4 mm beside the socket | missing | 4 modules at 1:50 |
| Rear | PSU opening 150 x 86, 120 mm fan, I/O field, 7 slots | present 1:50/1:100, 1:200 holes, 1:500 plain box | 1:500: rear 75 mm with PSU and fan openings; 1:200: fan hub in the opening |

### Defects found

- Rig: one bone only; the one real moving part of the subject (side cover) could not move. No service pose.
- 1:50 body: three floating groups (cooler fins/fan, mainboard, PSU core) — real modelling defects reported by `volumes-connected`; no drive cage, no memory.
- 1:100 body: fan hub floating in the fan opening; interior behind the cover only cooler, PSU and optical drive.
- 1:200 body: bay an empty hole, no latch recess, fan opening empty.
- 1:500 body: three plain boxes (plinth, case, bezel).
- 1:500 structure: frame a centred 0.3 m box that did not match the chassis (0.2 m ahead to 0.213 m behind the middle); PSU mass 75 mm cube.
- 1:200 / 1:500 thermal: no skin between bezel face and side skins (27.5 mm / 72.5 mm open band all round).
- 2D: cover not a part of its own, so no pose could show it.

### Rebuilt

- **Rig**: bone `cover` (parent `case`, origin at the rear end of the cover's bottom edge, `angleDeg -90` so u runs ahead along the hinge and v up, 0.413 m, revolute with `twistMinDeg 0`, `twistMaxDeg 90`). No lateral rest needed: the hinge runs ahead, the opening is the twist about u. New pose `service-open` (copy of `off`, `cover.twistDeg 35`).
- **Body**: 1:50 `cover`, `cpu-grille`, `latch-well`, `latch-lever` re-authored in the cover frame; new `drive-cage`, `disk-drives` (2), `memory-modules` (4), `board-standoffs-low/mid/high` (9); cooler base 68 mm, PSU core on the shell floor. 1:100: cover parts on the bone, hub reaches its guard, `drive-cage` block. 1:200: cover 30 mm plate on the bone with latch recess (chassis, core and frame moved to its inner face), `optical-drive` face, `rear-fan-hub`. 1:500 re-modelled: `plinth`, `case` block, `rear-panel` 75 mm with PSU and fan openings, `front-bezel` 75 mm with intake opening, dark `core`, 0.8 mm `top-skin` with `psu-top-grille` field, 0.8 mm hinged `cover` with `cpu-grille` field.
- **Thermal**: `th-cover-skin` and `th-cpu-grille` ride on the cover bone at all four scales; `th-bezel-skirt` (2.5 mm tube) closes the skin at 1:200 and 1:500.
- **Structure**: `member-cover-panel` (member `cover`, shear panel against the posts) at all four scales; 1:200 frame moved to the 30 mm cover; 1:500 frame over the true chassis length, PSU mass 86 x 140 x 150 mm in place.
- **2D** (1:50, 1:100, 1:200): elevation `cover` (and at 1:50/1:100 `cpu-grille`, `latch`) as `solid` extruded shapes on bone `cover`; front and plan carry the same `cover` solid. `z0`/`z1` of an extruded solid are lateral positions in the asset frame (measured: bone-local values lift the part by 0.09 m x sin 35). Default-pose drawings unchanged in silhouette; `service-open` shows the cover foreshortened in the elevation, tilted in the front view and 0.245 m out in the plan.
- **Decisions**: body, thermal and structure texts extended en/de at all scales (1:500 body rewritten); 2D `retained` extended.
- Scripts: `rework-ts-tower-server-1-rig.py`, `-2-drawings.py`, `-3-thermal.py`, `-4-structure500.py`, `rework-ts-tower-server-publish.py` (splices named root members into the catalog file without reformatting the rest).

### Validation

`r3-validate.ts appliances/tower-server --catalog-only --volumes`: `VALID appliances/tower-server (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (after the last publish).

### Rendered and read after the last change

1:500 body (default), 1:500 structure (`service-open`), 1:500 thermal (default), 1:200 body (default), 1:200 thermal cut, 1:50 body clip `load-surge` t 3 s, 2D 1:100 default. Read before the last change (which touched only 1:500 structure and one 1:500 tone): 1:50 body `service-open` (sheet and three-quarter close-up), 1:50 structure `service-open`, 1:100 body `service-open`, 1:200 and 1:500 body `service-open`, 2D 1:50 `service-open`.

### Remaining, stated plainly

- Rendered but not read: 1:100 thermal, 1:100 structure, 1:100 body default, 1:200 structure, clip `power-up`; not rendered: 1:50 thermal and structure in the default pose after the change, 1:200 thermal/structure `service-open`, 2D 1:200 `service-open` (rendered, not read), poses `rated`, `standby`, `off` (no bone offsets, geometry identical to `typical`).
- 1:500 body: the bezel is a 75 mm slab (real 25 mm) because no thinner slab passes as a part; in `service-open` the cover is a 0.8 mm skin swinging off a solid block.
- 1:200 `service-open`: the interior is the solid `core` block; a 15 mm strip beside it is see-through under the top opening and behind the fan opening.
- 1:100: no mainboard (a 1.6 mm plate in `bezel-abs` is refused as `sheet` because the material's thermal layer is 2.5 mm).
- 2D 1:100: the drawn cover reaches the top edge (0.45 m) instead of 0.435 m; drawn 15 mm lower, the remaining strip of `case-side` trips `PRINT range-end-feature` at 1:150.
- Thermal `front-bezel` area at 1:500 is larger than `areaM2` suggests (75 mm skirt instead of 22.5 mm); the validator accepts it, I did not measure the number.
- No mass on bone `cover` (the 11 kg stay on `case`); no clip that opens the cover; cover thumbscrews, status LEDs, cables, expansion cards not modelled.
- 2D representation 1:500 stays `unavailable` (asset 0.9 mm on paper).

### Fundamental issues (outside this asset)

- `volumes-feature`: a declared `sheet` whose material happens to have a thermal layer on that bone is forced to that layer's thickness (mainboard 1.6 mm vs bezel layer 2.5 mm) — the gauge should permit, not forbid, another declared thickness.
- README "Solid placements" does not say in which frame `z0`/`z1` of an `extruded` solid are measured (asset lateral, not bone-local) — matters for every bone with `z` not 0.
- `r3-tools.md`/README: nothing says that `solid` shapes must be repeated in front and plan to appear there (the `r3-tools` text "the product generates `solid` shapes from the elevation profile" reads as automatic).
