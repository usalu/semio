# machinery/scissor-lift — rework report

## Pass 1 (second round), 2026-11-22

State found: no report of this asset yet; `rework/10.md` section 5 (first round: coarse scales modelled anew, `volumes-connected` left open) and an unreported, cut-off second-round attempt (`rework-scissor-lift-join.py`, 13:55: bearers, hinges, brackets, axles joining every part). The source `volumes/machinery/scissor-lift.json` was ahead of the catalog (catalog 1:200 / 1:500 still held the 103 / 97 solid lists copied from 1:50). Rendered both: the source is the better state and was kept; catalog and source now agree (published).

### Reference (slab scissor lift of the 19 ft class: Genie GS-1930, JLG 1930ES, Skyjack SJ3219)

| quantity | real machine | model (rig, solids, sheets) | verdict |
|---|---|---|---|
| stowed length x width x height, rails up | 1.83 x 0.76–0.81 x 2.11 m | 1.83 x 0.81 x 2.11 m | ok |
| platform height stowed / raised | 0.97–1.0 m / 5.79 m | 1.00 m / 5.8 m (arms at 60°) | ok |
| platform 1.63 x 0.74 m, slide-out deck 0.91 m | yes | 1.63 x 0.74 m, travel 0.91 m | ok |
| scissor stack | 4 stages, rectangular tube about 90 x 50 x 6 mm, arm centres about 1.45 m | 4 stages, 90 x 50 mm, 1.46 m, a-arms outside, b-arms nested inside | ok |
| lift cylinder | one, lower inner arm to cross tube of stage 2, bore about 90–100 mm | one, barrel 100 mm, rod 50 mm, own bones (revolute + prismatic) | ok |
| wheels | 305 x 100–115 mm solid non-marking, wheelbase 1.32 m | 305 x 110 mm, wheelbase 1.32 m, track 0.70 m | ok |
| ground clearance / pothole guards | 60–90 mm / 20 mm deployed | 60 mm, guards as retracted plates | ok |
| guard rail | 1.10 m top, mid rail, 150 mm toe board, 30–40 mm tube | 1.11 m, 0.65 m, 150 mm, 40 mm square | ok |
| chassis | tub about 0.48 m wide with swing-out trays (batteries 4 x 6 V, power unit), entry ladder at the rear | tub 0.48 m, trays to 0.77 m, doors, handles, ground controls, ladder with 3 rungs | ok |
| front axle | steered front wheels on kingpin knuckles, steer cylinder under the tub | wheels on stub axles straight out of the tub: knuckles missing | defect, see below |
| mass | 1 500 kg | 1 503 kg | ok |

### Judged on the sheets (every scale, three lists, poses stowed / mid-raise / raised-working / lowering, clip raise-cycle)

- 1:50 body: reads as the machine from all six sides and in every pose; the scissor chain stays closed (pins meet in every pose), platform stays level, deck extension slides inside the main rails. Defect: no steering knuckles.
- 1:100: gate frame 5 mm off the rear posts (structure load path open, validator finding).
- 1:200: wheels were plain discs; arms were solid bars, so the thermal list showed 20.6 m² against the declared 31.6 m² total (validator finding, 35 %): the declared area counts the inside of the tubes, which a solid bar cannot have.
- 1:500: body sound for a 75 mm feature; two of its three `decisions` texts described things that do not exist at this scale (bored cross sections, welded sleeves).
- Thermal (cut sheets 1:50, 1:500) and structure (1:50, 1:200 raised) read as the same object: seven regions, frame rails / stack rail / axle tubes on four tyre-contact supports, arms as members with pin joints, deck joists, masses for batteries, power unit and platform.

### Rebuilt in this pass (`rework-scissor-lift-p1.py`, hand-chosen numbers; the join script of the cut-off attempt is kept as it ran)

- 1:50, 1:100, 1:200 body + thermal: `steer-yoke` (two knuckle blocks 70 x 170 x 50 mm between tub and front wheels around the stub axles), at 1:50 and 1:100 with `kingpin-cap`.
- 1:100: `gate-hinge` (two, 20 mm) and `gate-latch` join the gate to the rear posts, body, thermal and structure members.
- 1:200: the four tyres are revolves with a 30 mm dished wheel face (rim radius 105 mm); the eight arms are the true 90 x 50 mm tubes with 6 mm wall and open ends (`extrude` in `yz` with a hole), which closes the thermal area finding without touching `areaM2`.
- 2D: front view of 1:50, 1:100 and 1:200 gets `steer-yoke-l` / `steer-yoke-r` (extruded shapes behind the front tyres). Elevation and plan are unchanged (the knuckles are hidden by the tyres and the platform).
- `decisions`: one new entry per scale 1:50–1:200 (en, de); 1:500 rewritten completely (body, thermal and structure as they are modelled).

### Validation

`r3-validate.ts machinery/scissor-lift --catalog-only --volumes` → `VALID machinery/scissor-lift (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (print checks included). Before this pass: 2 findings (thermal coverage 1:200, open load path 1:100); before the join script: 13.

### Remains imperfect (open, for the next pass)

- Rig: the front wheels roll on one common bone `wheel-f` and do not steer. Missing: two steering bones with vertical axes at the kingpins (lateral rest placement), front wheels split left / right, the 2D wheel shapes rebound, a steered pose. The knuckle solids are in place for it.
- 1:50 body, parts not modelled: document box on the guard rail (tried at z 0.28–0.33 on the left rail with shapes in all three views; `volumes-silhouette` front p95 31.9 mm against 30 mm although the SVG shows the shape exactly on the solid; removed again, cause not found), steer cylinder and tie rod under the tub, fork pockets and tie-down lugs, hydraulic hoses and platform cable along the stack, maintenance prop, beacon.
- 1:200: ladder is two stiles without rungs (a 30 mm rung would be admissible); 1:500 has no ladder.
- 2D: not redrawn beyond the knuckles; the 1:500 representation stays `unavailable` (symbol only), as before.
- Poses and clip were read at 1:50 (mid-raise, raised-working via clip t = 22 s) and at 1:100 / 1:200 / 1:500 in one raised pose each; `lowering` only rendered at 1:100, not every pose at every scale.
- Thermal and structure lists at 1:100 were validated but not read as sheets after the change.

### Fundamental issues (outside this asset)

- `volumes-thermal-coverage` compares solids with an `areaM2` whose `areaBasis` is `total` (inside and outside of hollow sections): a coarse scale that merges a tube into a bar necessarily fails; the rule should compare against the outer share at scales where the wall is below the feature, or say so in its message.
- `volumes-silhouette` gives no location: the finding names p95 and the largest distance but not where; a point (view coordinates) or the nearest solid / shape id would have settled the document-box case in one step.
- The catalog asset file mixes two JSON layouts (indent 1 for most members, indent 2 inside `representations`), so a `json.dump` round trip rewrites 170 000 lines; edits to 2D shapes have to be text splices.
