# machinery/oil-pumpjack — running author report

## Pass 1 (second round)

Scripts (hand-chosen numbers, idempotent): `rework-oil-pumpjack-pass1.py` (source `volumes/machinery/oil-pumpjack.json`), `rework-oil-pumpjack-pass1-root.py` (catalog document: 2D shapes, `structure.supports`; candidate `🗑️generated/r3-oil-pumpjack/🔣️.json` by default, `--write` for the catalog file). Earlier work: `rework/10.md` section 7 (`rework-10-oil-pumpjack-wellhead.py`).

### State found

- Source and catalog differed: the source carried the unpublished edits of batch 10 (wellhead walls per scale, `saddle-shaft` and `crank-shaft` at 1:200 and 1:500, newer decisions). The source is the better state (with the shafts the beam group touches the post and the cranks touch the reducer at 1:200 and 1:500); it is the base of this pass and is now published, catalog and source are identical.
- `r3-validate --volumes` on that state: INVALID, 5 findings, all `volumes-connected` on the structure lists (10 / 9 / 3 / 1 open groups at 1:50 / 100 / 200 / 500, no support solid, `structure.supports` empty).
- Read before the edit: 1:50 body (six views), 1:50 structure, 1:200 body, 1:500 body. The coarse lists are modelled per scale (rolled beam section at 1:200, box beam at 1:500, merged wellhead, cylinder motor), not thinned copies.

### Reference table (conventional crank-balanced beam unit, API C-320-256-74 class)

| quantity | real machine | model (measured) | verdict |
|---|---|---|---|
| polished rod stroke | up to 74 in = 1.88 m | crank 0.85 m, beam arms 2.70 / 3.00 m; carrier travel between the authored poses 2 x 0.965 = 1.93 m | ok |
| saddle bearing height above the base | 4.2 to 4.6 m | 4.40 m | ok |
| walking beam | W24 to W27 rolled section, 5.5 to 6.0 m | W24 (611 x 325 mm, flanges 19, web 12.7), 5.07 m plus horsehead to R 3.00 m | ok |
| horsehead | arc of radius = front arm, box 0.5 to 0.7 m wide, arc longer than the stroke | R 3.00 m, 0.56 m wide, arc 2.03 m | ok |
| pitman length | 2.6 to 3.0 m | 2.65 m | ok |
| samson post | four legs splayed both ways, braced, cap with saddle bearing, access ladder to the saddle bearing | four 170 mm legs, brace frame, cap, saddle blocks and shaft; no ladder | ladder missing |
| reducer | double reduction, crank shaft about 1.75 m above the base, input shaft with sheave on one side and brake drum on the other | 2.42 x 1.65 x 0.84 m housing, crank shaft 1.75 m, sheave under the belt guard; no brake | brake missing |
| brake | drum 350 to 400 mm, band or shoes, hand lever about 1 m at the motor end, pull rod | none | missing |
| cranks and counterweights | two arms about 1.4 m, weights about 0.16 m thick | arms 1.42 m, weights 0.16 m, swept radius 1.30 m | ok |
| prime mover | 30 kW motor, frame 225, about 0.45 to 0.55 m over the fins, on a base plate | 0.54 m, feet and plate | ok |
| wellhead | casing head, pumping tee, stuffing box, polished rod 32 to 38 mm, flowline 60 to 114 mm | casing 170 mm, tee, stuffing box, rod 50 mm, flowline 120 mm | rod and flowline a size too thick (open) |
| bridle | two wirelines 19 to 25 mm about 0.46 m apart, carrier bar | 15 mm, 0.46 m apart, carrier 0.56 m | wireline a size thin (open) |
| footprint, height | about 8 x 2.4 m, 6.5 to 7 m at the top of the stroke | slab 7.8 x 2.4 m, 6.87 m | ok |
| mass | 17 to 19 t | 18 130 kg | ok |

### Defects found

1. Structure, all four scales: no closed load path. The lists named the big members only; the parts that pass the load on were missing (skid deck under reducer and motor, motor feet, main bearing caps and crank shaft, saddle blocks, tail hanger plates, pitman eyes, wellhead stack under flowline and stuffing box, rod clamp on the carrier, horsehead face plate under the wirelines), there was no support solid and `structure.supports` was empty.
2. Body: no access ladder on the samson post and no brake.
3. Rig, poses, clips: unchanged. The four-bar linkage (crank, pitman, beam, post) and the bridle chain close in the default pose (read on the sheet). Crank, saddle and equalizer axes run across the asset; the bones lie in the drawing plane and turn about z, which is the true axis, so no lateral rest placement (`rig-lateral-rest.md`) is needed.

### Rebuilt

- Body, 1:50: ladder of two 60 x 10 mm flat stringers 440 mm apart, parallel to the well-side legs 110 mm in front of them, from the slab (y 0.30) to the underside of the cap plate, thirteen 20 mm rungs at 280 mm (`slender`). Brake: input shaft extended on the side away from the sheave (100 mm, to z 0.81), drum 380 x 110 mm outboard of the crank arm (z 0.80 to 0.91; placed there because a drum between housing and crank would lie inside the 1.30 m counterweight sweep), band block on the drum, lever stand on the outer skid beam at the motor end, 1 m lever of 32 mm, 16 mm pull rod to the band block.
- Body, coarser: 1:100 stringers 60 x 15 mm, everything else kept; 1:200 stringers 60 x 30 mm, rungs as true rods, drum, shaft, stand, lever, no band block and no pull rod; 1:500 two 75 mm stringers without rungs, drum on its shaft only.
- Thermal: every new solid has its thermal solid at every scale (ladder, stand, lever, rod, band in region `steel`; shaft and drum in region `gearbox`); `volumes-thermal-coverage` passes at all scales.
- Structure: member solids for the missing load path parts listed above at every scale where the part exists, the ladder as secondary members, four support solids 1.0 x 1.2 m under the slab at the contact anchors, `structure.supports` = the four contact anchors. `volumes-connected` and `volumes-structure-coverage` pass at all four scales.
- 2D, all four scales: elevation ladder stringer (polygon on the post), brake stand, lever and rod; front and plan stringers, rungs, lever, rod as `member` shapes, brake shaft, drum, band block and stand as rectangles; at 1:200 without band and rod, at 1:500 stringers, shaft and drum only. `volumes-silhouette` and the print rules pass.
- Decisions: one sentence pair (en, de) per scale appended to `volumes[scale].decisions`.

### Validation

`r3-validate.ts machinery/oil-pumpjack --catalog-only --volumes` after publishing: `VALID machinery/oil-pumpjack (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.

### Not proven by eye (open for the next pass)

After the first version of the edit one 1:50 three-quarter view (turn 130, tilt 20) was read: ladder, drum and lever stand where intended, the ladder reads as a ladder, nothing pokes through. After that the lever moved from z 0.86 to z 0.97, the brake shaft was lengthened and the 2D shapes were added; from then on the Read tool of this session failed (hook timeout), so none of the final renders was read. They are rendered and wait under `🗑️generated/r3-sheet/machinery/oil-pumpjack.*` and `🗑️generated/r3-2d/machinery/oil-pumpjack.s*.png`: body, thermal, structure at 1:50, 1:100, 1:200, 1:500; poses `stopped`, `bottom-of-stroke`; `pump-cycle` at 0.75 s and 2.2 s; `start-up` at 2 s and 6 s; the four 2D sheets.

Open list:

- All scales, all lists, all poses and clips: read the final renders (see above). Check in particular: the pull rod (skew from z 0.97 to 0.88) against motor terminal box and belt guard; the lever stand on the outer flange of `skid-beam-l`; ladder top against the leg tops under the cap (20 mm clearance by calculation); the brake drum against pitman eye and wrist pin over the whole crank turn (clear by calculation: drum axis 1.366 m from the crank axis, drum reaches in to 1.176 m, pitman eye reaches out to 0.99 m).
- 1:50 body: no fence or guard around the cranks, no wireline hanger at the top of the horsehead, no stand-off brackets from the ladder to the brace frame (the ladder is held at the slab and the cap only), no handle grip on the lever.
- 1:50 body: `counterweight-l/r` outer corner sweeps 1.30 m, `input-cap` (radius 0.16 m at 1.366 m, z 0.42 to 0.47) and the counterweight (z 0.43 to 0.59) overlap by about 40 mm in z over a 94 mm deep sliver of the sweep: check by rendering the crank at about -24 degrees and trim the cap or the weight corner.
- All scales: polished rod 50 mm (real 32 to 38 mm), flowline 120 mm (real up to 114 mm), wirelines 15 mm (real 19 to 25 mm); the 2D member shapes and the thermal region `polished-rod` carry the same diameters and have to follow.
- Thermal: `areaM2` of the regions `steel` and `gearbox` in `asset.thermal` was not re-derived for the added ladder and brake surfaces (the validator accepts the lists).
- Structure 1:50 to 1:200: rungs are listed as members so that the post's member drawing shapes are covered; whether the product's force and moment views read well with them was not looked at.
- Mass: ladder (about 60 kg) and brake (about 45 kg) are not added to the bone masses.
