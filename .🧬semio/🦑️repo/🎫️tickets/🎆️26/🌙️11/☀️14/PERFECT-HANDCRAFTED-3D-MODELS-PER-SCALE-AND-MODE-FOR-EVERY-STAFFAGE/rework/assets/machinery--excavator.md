# machinery/excavator — author report

Earlier work on this asset: `T/rework/10.md` section 1 (reference table, kinematic proof of the cylinder pins, per-scale undercarriage and bucket, 1:200 elevation of the track frame). This file continues from there.

## Pass 1 (second round)

### State found

- Source `T/volumes/machinery/excavator.json` (01:25) was newer than the catalog (00:58): it added `monitor-arm`, `mirror-arm-left` (1:50), `idler-pin`, `link-pin` (1:100), `boom-pin` (1:200), `boom-cyl-pin` (1:500) with their thermal twins and moved consoles, boom pipes and grousers so that the body is one connected object. Rendered both; the source is the better state (no floating monitor and mirror) and was kept.
- `r3-validate.ts machinery/excavator --volumes` on that state: `INVALID … checkVolumes 4`, all four `volumes-connected` on the structure lists (open load path at 1:50, 1:100, 1:200, 1:500).

### Reference table (22 t class crawler excavator: Cat 320, Komatsu PC210, Liebherr R 922; mono boom 5.7 m, stick 2.9 m)

| quantity | real machine | model (rig, solids, sheet titles) | verdict |
|---|---|---|---|
| operating mass | 21.5 to 23 t | 22 515 kg (sum of bone masses) | ok |
| transport length x width x height | 9.5 to 9.6 x 2.98 to 3.0 x 2.96 to 3.2 m | 9.63 x 2.98 x 3.25 m in `stowed` | ok |
| track length / sprocket to idler / gauge / shoe | 4.45 / 3.65 / 2.38 / 0.60 m | 4.40 / 3.65 / 2.38 / 0.60 m | ok |
| track height, rollers / carrier rollers per side | 0.85 to 0.95 m, 7 to 8 / 2 | 0.84 m, 8 / 2 | ok |
| slew ring height / diameter | 0.95 to 1.1 m / 1.2 to 1.4 m | 0.94 to 1.06 m / 1.32 m | ok |
| tail swing radius, counterweight clearance | 2.75 to 2.95 m, 1.05 m | 2.83 m, 1.07 m | ok |
| cab: height over ground / width / length, on the left | 2.96 / 0.95 to 1.0 / 1.8 m | 2.96 / 0.83 to 0.95 / 1.82 m, left (plan top with +x right) | ok |
| boom foot pin ahead of slew axis / height | 0.1 to 0.5 m / 1.8 to 2.0 m | 0.15 m / 1.92 m | ok |
| boom / stick pin to pin, bucket tip radius, bucket width | 5.70 / 2.90 m, 1.45 to 1.5 m, 1.2 to 1.35 m | 5.70 / 2.90 m, 1.47 m, 1.20 m with five teeth | ok |
| cylinders: boom 2 x 120/85, stick 135/95, bucket 120/85 mm | barrels 140 to 170 mm outside | 145 / 168 / 145 mm barrels, 85 / 100 / 85 mm rods | ok |
| upper frame | two main girders that rise into the boom foot brackets, carry the slew ring, the engine bearers and the counterweight; cab on a platform beside the left girder | girders stood 0.12 to 0.16 m beside the boom feet, cab frame stood on nothing | defect |
| ROPS (ISO 12117-2) | closed frame: four posts, two cant rails, roof bows | two separate side frames, left one floating | defect |
| hydraulic cylinder | rod carries a piston that bears on the barrel; cap end has the pin eye | rod 17 to 20 mm clear inside the tube, cap eye missing in the structure | defect |
| slew bearing | bolted to the carbody ring below and the frame ring above | 20 mm clear of the carbody ring at 1:50 | defect |

Rig (16 bones): undercarriage, slew-ring (vertical axis), upper, counterweight, engine, boom, stick, bucket, three barrel/rod pairs with prismatic rods, idler links and H link closing the four-bar bucket linkage. Pivots and limits agree with the table; no change needed. Rendered `stowed`, `digging`, `raised-boom` at 1:50 from six directions: every cylinder spans its pins, the bucket linkage stays closed, nothing opens or pokes through.

### Stage 1 — 1:50 body and rig

Judged from six directions and a close three-quarter view: proportions, boom banana, cylinders, undercarriage, cab with seat, consoles, joysticks and glazing, counterweight arc, grilles, stack, steps, grab rails and lights are right against the table. Kept as it is. What a modeller would still add is listed under "Remains imperfect".

### Stage 2 — structure, true load path, all four scales (`rework-excavator-structure.py`, diagnostic `rework-excavator-connected.ts`)

Found with `rework-excavator-connected.ts` (prints every group of touching solids with its nearest neighbour, optional `--masses off` so that mass markers cannot bridge a gap, `--pose`): 17 / 10 / 10 / 6 groups at 1:50 / 1:100 / 1:200 / 1:500.

- **Main girders** `frame-rail-1/2`: moved under the boom foot brackets (z of `boom-foot-right/left`), as on the real machine where the brackets are the girders' own webs. Before, the whole front equipment hung beside the frame.
- **Counterweight carriers**: are the rear ends of the girders now (same z, overlapping the girder ends); before, carrier 2 ended 20 to 30 mm short of the rear cross member.
- **Engine bearers**: 1:50 two transverse bearers across both girders under the transverse engine; coarser scales the one bed lowered 5 mm onto the girders.
- **Cab platform and ROPS**: `frame-cab-rear`, `frame-cab-mid` (cross members from the left girder to the cab side), `frame-cab-sill-right` (and `-left` where there is no `frame-side-2`), posts taken down onto them, `rops-bow-front/rear` close the two side frames into one roll-over frame.
- **Slew bearing** 1:50: `joint-slew` now reaches from the carbody ring into the frame ring.
- **Cylinders** 1:50 and 1:100 (hollow tubes): `piston-*` on every rod bone (travels inside the barrel through the whole stroke: 0.10 to 1.40 / 1.60 / 1.20 m of 1.62 / 1.95 / 1.47 m); 1:50 cap eyes as references to the body eyes.
- **Cylinder lugs and rod boss**: `stick-cyl-lug*`, `bucket-cyl-lug*`, `boom-rod-boss` are members now; the pins no longer hang beside the boom and the stick.
- **Bucket** 1:200 and 1:500: shell, cheeks, lip and ears are members; before, lip and teeth floated.
- **Undercarriage** 1:200: the rollers span the two frame flanges (0.23 m with their end collars; body solid, thermal follows); the idler hub is a member. 1:50 and 1:100: the lower grouser run is a member, so the path really ends on the eight support patches.

Result: one group at every scale with and without the mass markers, in `stowed`, `digging` and `raised-boom`.

`r3-validate.ts machinery/excavator --volumes`: `VALID machinery/excavator (catalog asset + volumes source): schema 0, validateAsset 0, checkVolumes 0`. Published with `volumes-insert.py`; `--catalog-only`: `VALID … schema 0, validateAsset 0`.

### Stage 3 — 1:100, 1:200, 1:500 (`rework-excavator-lugs.py`)

Rendered each scale (three-quarter close-up, six views, `digging` at 1:200 and 1:500) and read them: each list is modelled for its feature (1:100 without seat, joysticks, slats and bosses; 1:200 one shoe band on one chain, plain discs, closed cab shell; 1:500 track blocks, one house prism, closed side profiles), same proportions and bones as 1:50.

- Defect seen at 1:200 and 1:500: the cap end of the bucket cylinder hung free above the stick (no lug), at 1:500 also the cap end of the stick cylinder above the boom and the rod ends of the boom cylinders beside the boom. Added `bucket-cyl-lug` (1:200, 1:500), `stick-cyl-lug` and `boom-rod-boss` (1:500) at their 1:100 size (0.2 m wide, 0.14 to 0.27 m high, boss 0.14 m: all above 75 mm), each in body, thermal (`attachment`) and structure.
- 1:200 rollers: see stage 2.
- Thermal: read the 1:50 thermal sheet with `--cut`: the same object, engine compartment as block inside the house with the grille patches, stack emitter, cab glazing, hydraulics as the cylinders, tracks and attachment; nothing to rebuild. Thermal lists at the coarser scales follow the body through their references.

### Stage 4 — 2D (`rework-excavator-2d.py`)

The new lug solids are drawn: `bucket-cyl-foot` on the stick at 1:200; `stick-cyl-foot`, `boom-rod-boss` and `bucket-cyl-foot` at 1:500, in elevation, front and plan, directly above the body shape of their bone, with the profile and lateral span of the solid (the ids and form of the 1:100 / 1:200 drawings). Candidate checked with `r3-validate.ts --file` (`VALID`) before the catalog file was written.

### Stage 5 — decisions

Structure decision of every scale and body decision of 1:200 and 1:500 extended (en, de) with what was modelled and why; the 1:500 text no longer says the lug is left out.

### Validation (after the last change)

- `r3-validate.ts machinery/excavator --volumes` (source over catalog): `VALID … schema 0, validateAsset 0, checkVolumes 0`.
- Published with `volumes-insert.py`, 2D written to the catalog file, then `r3-validate.ts machinery/excavator --catalog-only --volumes`: `VALID machinery/excavator (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- `rework-excavator-connected.ts`: body one group at every scale (`stowed`, 1:50 also `digging`); structure one group at every scale with and without mass markers in all three poses.
- `rework-10-pins.ts`, 3 poses and 36 instants of each of the 3 clips: boom rod eye 0.1 mm, stick rod eye 0.3 mm, bucket rod eye 0.1 mm at worst.

### Not looked at (stated plainly)

From about 02:50 the Read tool stopped answering (hook timeout on every image), so these renders were made after the last change but **not read**:

- the six clip frames at 1:50 (`dig-cycle` t 3 and 6.5, `dump-and-stow` t 2.5 and 7, `start-and-idle` t 2 and 6); the 1:50 body is unchanged in this pass and the pins close numerically, but nobody looked at the frames;
- the 2D drawings of 1:200 and 1:500 with the new feet (read before the change, not after);
- structure sheets of 1:100, 1:200, 1:500 and thermal sheets of 1:100, 1:200, 1:500 (only 1:50 was read for both; the rest is proven by the connectivity and coverage rules only);
- `raised-boom` and `stowed` after the structure change at 1:50 (the `digging` structure sheet was read).

### Remains imperfect

- 1:50 body: the house is rounded boxes (engine hood, tank, tool box) without door seams, hinges, latches, handrail along the walkway, or the hose bundle from the boom foot to the valve block; no travel motors visible behind the sprockets; no wiper, no door handle.
- 1:100 keeps most body solids of 1:50 that are above 15 mm unchanged (see `rework/10.md`).
- Slew ring: limits 0/0, no slewing pose or clip; the rig cannot show the machine turned over the side.
- Structure: the engine mass marker sits 0.1 m above its bearers (no mount blocks); the cab platform is a plain grid, not the real viscous-mount riser.
- The rear cross member `frame-cross-1` and the carriers at 1:50 overlap the girders as plain boxes (no gussets).

### Fundamental issues (outside this asset)

- `volumes-connected` counts `mass` markers as touching parts: at 1:50 `mass-upper` joined the right ROPS frame, the boom feet and the whole front equipment into one group although none of them touched the frame; a mass marker should neither bridge nor be bridged.
- `rework-10-connected.ts` prints one line per finding (the rule's `data` has no groups), so a loose member cannot be located; `rework-excavator-connected.ts` in `T` prints every group with its nearest neighbour and could replace it for all authors.
- `validator.md` has no section "Rule changes authors must know" and `rig-lateral-rest.md` does not exist (both named in the task).
- The Read tool's PreToolUse hook timed out on every call for the last part of this pass.
