# vehicles/motor-scooter — running report

## Pass 1 (second round), 2026-11-22

Status: **partial**. The asset was already a careful hand model (not the sloppy state of the brief); this pass corrected what the renders showed and what fitted into one pass, and lists plainly what a second pass has to rebuild.

### State found

- No earlier report for this asset; `rework/09.md` lists it but never reached it.
- Source `volumes/vehicles/motor-scooter.json` against the catalog: body, structure and decisions identical at all four scales, thermal 1:50 identical. Thermal 1:100 / 1:200 / 1:500 differed in one thing only: the source states `layer: "paint"` on the references of the painted panels and painted steering parts (31 / 17 / 8 entries), the catalog had them without a layer. The source is the later edit, consistent with the nested paint/wall modelling of 1:50, and `VALID` with `--volumes`; it was kept and published. The 1:100 thermal decision ("layers are merged") contradicted it and was rewritten.
- `r3-validate.ts vehicles/motor-scooter --volumes` before any change: `VALID … schema 0, validateAsset 0, checkVolumes 0`.

### Reference table (Vespa Primavera 125 class, steel monocoque; from memory of manufacturer data)

| quantity | real | model (measured from rig, solids, sheets) | deviation |
|---|---|---|---|
| length | 1.852–1.870 m | tail lamp −0.967 to front tyre +0.900 = 1.867 m | in class |
| width over grips / mirrors | 0.68 / 0.735 m | grips ±0.340 = 0.68 m, mirrors ±0.370 = 0.74 m | 0 |
| height over mirrors | 1.15 m | 1.15 m | 0 |
| wheelbase | 1.334–1.340 m | axles at x +0.6701 and −0.6700 = 1.340 m | 0 |
| seat height | 0.79 m | seat top 0.775–0.80 m | in class |
| handlebar height | about 1.00 m | steering head joint at y 1.00 m | 0 |
| steering rake | 25–27° | steer bone −65° from horizontal = 25° | 0 |
| tyres | 110/70-12 front, 120/70-12 rear, outer diameter about 0.46 m | sections 110 and 120 mm, radius 0.23 m | 0 |
| rim | 12 in = 0.305 m bead seat, flange about 0.33 m | rim flange radius 0.166 m | 0 |
| front disc / rear drum | 200 mm / 140 mm | disc radius 0.100 m, drum hub radius 0.083 m | drum +19 % (hub with fins, accepted) |
| leg shield width | about 0.44 m | wings to ±0.19–0.22 m | in class |
| floor width / height | about 0.40 m / 0.30 m | ±0.185 m, top 0.30 m | 0 |
| rear body width over cowls | about 0.45 m | cheeks to ±0.20–0.22 m | 0 |
| front mudguard width | about 0.16–0.17 m, rounded nose and tail in plan | 0.196 m, square-cut ends in plan | +15 %, plan corners not rounded (open) |
| front suspension | single-sided trailing link with coil-over shock on the right | fork leg bent right, link, stub axle, shock: present | link is no bone (open) |
| engine unit | swinging unit, CVT case left, silencer right, one rear shock left | present, pivot at (−0.27, 0.205), ±8° | 0 |
| kerb mass | 126–132 kg | 50 + 13 + 8.5 + 41 + 9.5 + 4.5 + 3.5 = 130 kg | 0 |

### Looked at (rendered and read)

1:50 body (six views, two extra three-quarter close-ups front and rear), 1:50 thermal, 1:50 structure, poses `idling-seat-open`, `turning-left`, `braking-nose-down` (after the change), 1:100 body, 1:200 body and structure, 1:500 body (before and after, with a side close-up). Rendered but **not read**: pose `parked-on-stand` alone (the stand is the same as in `idling-seat-open`, which was read), clips `ride-loop` t 0.2 and `stop-and-shut-down` t 4.5, thermal and structure sheets of 1:100 and 1:500, thermal of 1:200, every `--cut` thermal sheet. `r3-2d.ts` for 1:50 did not finish within five minutes under host load, so **no 2D drawing was looked at in this pass**.

### Defects found

1. Seat lifted (`idling-seat-open`): the rear body was a closed smooth lump under the seat, no helmet bin, no fuel filler. Fixed at 1:50 (see below).
2. 1:500 body: the rear body had no raked front bulkhead (sections jumped from x −0.10 to −0.30), so the body top sagged to about 0.68 m under the seat nose and the seat hovered. Fixed.
3. 1:100 thermal decision contradicted the list. Fixed.
4. Leg shield wings at 1:50: seven-section lofts with a visibly polygonal outline (angular top corner and kinks at y 0.80 and 0.34) and a step against the centre body. **Open.**
5. Front mudguard: revolve with square-cut ends in plan at every scale, 0.196 m wide. **Open** (a loft cannot carry the C-section with its valances; it needs a revolve plus end caps, the nested thermal wall at four scales and the plan drawings).
6. Rig: the trailing link of the front suspension is not a bone; `braking-nose-down` pitches the whole scooter about the front contact and swings the engine unit, the front wheel does not travel in its link. **Open.**
7. Structure: the decisions call the frame a monocoque, the members are an idealised tube set (head tube, box spine, tunnel, two shell rails, cross members, bulkhead struts). It is connected and supported and reads as a frame, but it is not the stressed skin of a pressed-steel monocoque. **Open** (judgement call for the next pass: shell members by reference to floor, shield core and rear body, or keep the idealisation and say so more plainly).
8. `structure.supports` names only `contact-f` although `cruising` stands on both tyres; only one contact pad solid exists in the structure lists. Not changed (solver convention not verified). **Open.**
9. Seat edge at 1:50 shows a few dark facets at the stepped part of the loft (rear three-quarter). **Open**, cosmetic.

### Rebuilt in this pass

Edit script: `rework-09-motor-scooter-model.py` (hand-stated numbers, re-runnable).

- **1:50 body**: `bin-rim` (loft, rubber, six sections, 0.365 m long, 0.25 m wide, raised 4–16 mm on the rear body top between x −0.20 and −0.565) and `fuel-cap` (revolve, radius 34 mm, at x −0.635) under the seat. Hidden by the closed seat (top 0.729 m inside the seat base at 0.725 m), visible with the seat lifted. Honest limit: it is a raised collar on a closed body, not a modelled cavity.
- **1:50 thermal**: `th-bin-rim`, `th-fuel-cap` by reference in `body-panels` / `paint`, as the floor rails.
- **1:500 body**: one section added to `body` at x −0.17 (centre 0.525, half depth 0.195, half width 0.172, exponent 3), the raked front bulkhead up to the seat hinge; the seat now lies on the body (side close-up read).
- **Decisions**: 1:50 body (bin collar and cap), 1:500 body (bulkhead), 1:100 thermal (rewritten), en and de.
- Rig, structure lists, 2D drawings, 1:100 and 1:200 lists: unchanged.

### Validation

- `r3-validate.ts vehicles/motor-scooter --volumes` after the changes: `VALID vehicles/motor-scooter (catalog asset + volumes source): schema 0, validateAsset 0, checkVolumes 0`.
- Published with `volumes-insert.py` (`volumes spliced: 1:50 authored, 1:100 authored, 1:200 authored, 1:500 authored`); catalog `volumes` equal to the source.
- `r3-validate.ts vehicles/motor-scooter --catalog-only --volumes`: see the last line of this section.

`VALID vehicles/motor-scooter (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`

### Remaining for the next pass (precise)

| scale | list | part | defect |
|---|---|---|---|
| 50, 100 | body + thermal wall | `shield-wing-l/-r` | polygonal outline, step against `shield-core`; re-loft with more sections and a rolled edge, then the nested `-wall` solids and the front drawing |
| 50, 100, 200, 500 | body + thermal wall + 2D plan | `mudguard` | square-cut ends in plan, 0.196 m wide against about 0.17 m; rounded nose and tail |
| all | rig | front trailing link | no `link` bone; add it (pivot at the fork leg foot, wheel-f as its child, shock telescoping as at the rear), rebind link, stub axle, shock foot, caliper, `contact-f`, the 2D shapes and `braking-nose-down` |
| all | structure | frame members | tube idealisation of a monocoque; decide and model the stressed skin or state the idealisation in `structure.assumptions` |
| all | structure | supports | only `contact-f`; check whether `contact-r` / `stand-foot` need pad solids and entries |
| 50 | body | `bin-rim` | collar on a closed body, no cavity |
| 50 | body | `seat` | facets at the step |
| 200 | body | `seat` over `body` | thin gap under the seat nose still visible in the side view, not measured |
| all | 2D | elevation, front, plan | not rendered or read in this pass; 1:500 elevation of the rear body changed by up to 35 mm at the front bulkhead (under half a feature, `volumes-silhouette` passes) |
| 50 | poses / clips | `parked-on-stand`, both clips | rendered, not read |
| 100, 200, 500 | thermal, structure | sheets | rendered, not read (1:200 structure read) |

### Fundamental issues (outside this asset)

- `r3-2d.ts vehicles/motor-scooter --scale 50` did not return within 300 s while other authors' tools were running (the documented time is 5 s); no output file was written. Not investigated.
- `r3-sheet.ts` keeps one output name per option set, so a sheet read right after an edit can be an older file if the render is still running or failed silently; the 1:500 sheet read directly after the edit showed the old form, the close-up rendered next showed the new one.
