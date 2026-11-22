# animals/pigeon

- 2026-11-22 12:2x, rig verification (not the volume author): `poses` and `clips` re-solved with `rig-reoffset.ts animals/pigeon --write` because the rig now turns a pose offset about the bone's own rest axes instead of the parent's joint axis. The 40 primary and secondary feather bones had moved by up to 11.5 mm in wings-half-open, the take-off and airborne poses and the take-off clip. Every bone keeps the posed frame it had before the rig change (largest difference of a bone tip after the re-solve under 0.01 mm). Only `asset.poses` and `asset.clips` were written; rig, shapes, volumes untouched. Do not run the tool on this asset again (`rig-reoffset.done.json`); new poses are authored about the bone's own axes (`rig-lateral-rest.md` 1.1).

## Pass 1 (second round) — 2026-11-22

State in one line: wing poses, remiges and the pelvic girdle were rebuilt and published; the shoulder girdle as a rig bone is NOT done (the validator throws on it), the 2D drawings of the opened wing are NOT redrawn, and the body, head, neck, legs and thermal lists were looked at but not re-modelled. Status: partial.

Scripts (ticket folder): `rework_pigeon_fk.py` (forward kinematics with the full rest placement, checked against `rig-rest.ts` on four bones), `rework-pigeon-rig-wing.py` (girdle, wing poses, vanes, coarse thickness; ran once with `GIRDLE=thigh --write`, it refuses a second run on the coracoid check only, so do not run it again on the published rig), `rework-pigeon-decisions.py`, `rework-pigeon-shaft.py`, `rework-pigeon-skull500.py`, `rework-pigeon-tail500.py`.

### Reference (feral rock pigeon, Columba livia, my knowledge) against the model

| measure | real | model | verdict |
|---|---|---|---|
| length bill to tail tip | 0.31-0.34 m | 0.315 m | ok |
| standing height alert | 0.20-0.23 m | 0.225 m | ok |
| body breadth with folded wings | 0.09-0.10 m | 0.097 m | ok |
| wing span | 0.63-0.70 m | 0.60 m in `takeoff-push` (elbow 125, wrist 155 degrees) | short by about 5 %, not changed |
| humerus / ulna / hand (carpometacarpus + digit) | 45-48 / 53-55 / 55-58 mm | 45 / 54 / 58 | ok |
| longest primary (P9) / secondaries | 150 / 75-100 mm | 150 / 75-100 | ok |
| femur / tibiotarsus / tarsometatarsus / middle toe | 39-41 / 57-60 / 30-32 / 30-35 mm | 39 / 57 / 31 / 36 | ok |
| joint order hind limb | hip - knee - intertarsal joint - toes (three forward, hallux back) | the same; all four toes on one `toes` bone | topology ok, toes do not flex singly |
| skull with bill / neck | 53-55 mm / 70-80 mm in 12 vertebrae | 54 / 75 mm in three links | ok for the scale |
| coracoid | 35 mm, sternum to shoulder joint, out and up | solid 37 mm on the trunk bone | solid ok, no rig bone (see below) |
| breadth across shoulder joints | 40-44 mm | 44 mm | ok |
| breadth across hip joints | 28-32 mm | 40 mm | 8-10 mm wide, not changed (front and plan leg shapes would have to move) |
| head breadth | 24-27 mm with feathers | about 34 mm | too wide, not changed |
| visible tarsus thickness | about 5 mm | 6 mm at 1:50 and 1:100, 2.8 mm wire at 1:200 and 1:500 | too thin at the two coarse scales, not changed |

Source and catalog volumes were identical at the start. Validation at the start: INVALID, 172 `volumes-feature` findings (feathers and rachis bars 7.6 mm thick at 1:100 and 1:200, shoulder lobe, scapulars and hand below the feature).

### Defects found by reading the sheets

- **Opened wings were turned inside out.** The poses reached the open wing by turning the elbow on through the straight position (forearm offset +135 to +204 on a rest angle of -200), so in `wings-half-open` the secondaries pointed forward over the breast and the trailing edge led.
- Remiges were 7.6 mm thick lumps at every scale, all in one plane.
- No girdle bones: humerus and femur hung directly from trunk and pelvis beside the plane.
- A 197 mm "primary shaft" member from the old hand bone still stuck out of the opened wing in the structure list.

### Rebuilt and published

- **Rig**: `ilium-n` / `ilium-f` on the lateral rest placement, from the synsacrum [-0.004, 0.014, ±0.005] down and out to the hip joint [0, 0, ±0.020], 20.9 mm; the thighs are re-parented without moving (largest move of an existing bone origin 0.003 mm). Feather bones rest turned ±10 degrees about their shaft (shingling). Joint limits of humerus (ball joint, twist allowed), forearm (-165..15) and hand (-15..165) follow the true folding sense.
- **Poses** (`wings-half-open`, the four take-off poses, the three airborne poses; clip `take-off` follows): each wing solved from humerus direction, dorsal normal, elbow and wrist angle and tract spread (table `WING` in the script), far side solved, near side mirrored exactly. Leading edge leads, secondaries trail, primaries fan from 72 degrees (P1) to 4 degrees (P10) off the hand.
- **Body**: remiges are vanes of 0.9 mm, 19 to 27 mm wide, `sheet: true`, new material `remex` (a copy of the plumage material: with `plumage` the validator demands the 8 mm thickness of the plumage thermal layer). 1:100 the same twenty per wing; 1:200 five vanes of 32 mm per tract; 1:500 two vanes of 76 mm per tract. Shoulder lobe, hand and scapulars thickened to the feature at 1:100 and 1:200; scapulars merged into the trunk at 1:200.
- **Structure**: hip bone member per side at 1:50, synsacrum `members: [ilium-n, ilium-f]` at the coarser scales; rachis members are slender rods of 1.6 to 1 mm; the old primary shaft is removed; at 1:500 the skull spans the mandible and the pygostyle lump is left out.
- **Thermal**: entries of removed feathers and scapulars removed; nothing else touched.
- **Decisions**: one "Second round" entry per scale, English and German.

Validation: `VALID animals/pigeon (catalog asset + volumes source): schema 0, validateAsset 0, checkVolumes 0` and after publishing `--catalog-only --volumes`: `VALID animals/pigeon (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.

Read after the last pose and feather change: 1:50 body `standing-alert`, `wings-half-open` (candidate), `takeoff-push` (candidate), `airborne-wings-down`, clip `take-off` at 0.29 s; 1:50 structure `takeoff-push`; 1:500 body `takeoff-push`; 2D 1:50 `takeoff-push`. Rendered but NOT read: `takeoff-crouch`, 1:200 `takeoff-push`, 1:50 thermal `wings-half-open`. Not rendered after the change: `takeoff-wings-up`, `takeoff-liftoff`, `airborne-wings-up`, `airborne-climb`, the pecking and walk poses and clips, 1:100, the thermal list at any scale, structure at 1:100 to 1:500. The last three edits (shaft removed, 1:500 skull and tail) were validated but not rendered.

### Remaining, plainly

- Rig: shoulder girdle on lateral rest placement (coracoid bone from [0.0634, -0.0071, ±0.009] to the shoulder joint, humerus re-parented): the script does it (`GIRDLE=humerus,thigh`), but `validateAsset` throws (see fundamental). Coracoid, scapula and furcula stay solids on the trunk bone.
- Rig: hip breadth 40 mm instead of about 30; one bone for all four toes; trunk and synsacrum pivot at the hip joint (root `pelvis` is the hip axis, not the synsacrum).
- 2D, all scales, all three views: the wing is still drawn as hand, forearm and humerus blades; in every opened pose the drawings show sticks (1:50 `takeoff-push` read: plan and front show two bars). Needs per-tract shapes on the feather bones, or `solid` shapes.
- Body 1:50: head about 7 mm too wide; no tertials or humeral coverts, so a gap stays between the innermost secondary and the trunk in the spread wing; no patagium; vanes are symmetric ellipses, not asymmetric.
- Body 1:200 and 1:500: legs are 2.8 mm wires; typified vanes at 1:500 read as four paddles per wing.
- Span 0.60 m instead of 0.63 to 0.70 m.
- Thermal lists: not re-read at any scale after the change; `areaM2` of `plumage-wings` not re-derived for the thinner vanes (validator passes).
- Clip `take-off`: offsets are interpolated as angle triples; between `airborne-wings-down` (humerus twist +27) and `airborne-climb` (-172) the wing passes through intermediate twists that were not rendered.

### Fundamental issues (outside this asset)

- `validateAsset` throws instead of reporting when the humerus hangs from a lateral coracoid bone: `ASSET validator-threw $: the union of 6 polygon(s) with area cannot be built: neither the edge-splitting nor the slab union closed a boundary of the expected area` (also as `PRINT validator-threw`). Isolated on candidates: the rig change alone triggers it (old poses, old volumes, no feather twist, old joint limits); the same re-parenting of the thighs onto the lateral hip bones validates. The humerus, forearm and hand carry flat elevation, front and plan shapes; the cause inside the union was not investigated. Reproduce: add `coracoid-f` / `coracoid-n` under `trunk` from [0.0634, -0.0071, ±0.009] to [0.0428, 0.0211, ±0.022] with `rig-rest.ts --solve`, re-parent `humerus-f` / `humerus-n` with `--solve humerus-f --parent coracoid-f`, run `r3-validate.ts --file`.
- `volumes-feature`, sheet declaration: a solid with `sheet: true` whose material equals the material of a thermal layer of its region must have that layer's thickness (8 mm plumage), and the message then says "a lump or a strip is no sheet" although the solid is 0.9 x 23 mm. A flight feather needed its own material to be declared as a sheet; the message should name the layer thickness as the reason.
- `r3-sheet.ts` colours solids by material, so the vanes of the new material render violet beside the grey plumage; only a rendering matter.
- No image tool on the host (no PIL, ImageMagick or sips montage), so several sheets cannot be read in one image; every sheet costs one read.
