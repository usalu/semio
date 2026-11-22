# vehicles/delivery-van — rework report

Earlier work: `rework/09.md` (author 09: outriggers, knuckles, strut towers, connectivity; rig, thermal and 2D untouched). Source and catalog were identical when this pass started.

## Pass 1 (second round), 2026-11-22

### Reference (mid-size panel van, long wheelbase, low roof: Ford Transit Custom L2H1, VW Transporter T6 LWB, Renault Trafic L2H1; from memory)

| quantity | real | model | verdict |
|---|---|---|---|
| length / width / width over mirrors / height | 5.30–5.40 / 1.90–1.99 / 2.27–2.30 / 1.97–2.00 m | 5.34 / 1.97 (sides at ±0.985) / 2.27 / 1.99 m | ok (matches Transit Custom L2H1) |
| wheelbase, front / rear overhang | 3.30–3.50 m, about 0.95–1.0 / 1.0–1.05 m | 3.30 m, 0.97 / 1.07 m | ok |
| track | 1.70–1.74 m | 1.68 m | 20–30 mm narrow, kept |
| tyre | 215/65 R16 C (686 mm) or 235/65 R16 C (712 mm), 6.5J steel wheel, 5 studs | 712 mm, 235 mm wide | ok (larger option) |
| cowl / bonnet front edge height | about 1.20–1.25 / 1.0 m | 1.19–1.23 / 1.0 m | ok |
| windscreen | from the cowl above the front axle to the header about 0.8 m behind it | cowl at x 1.75, header at x 1.12 | ok |
| load sill, rear aperture | 0.53–0.57 m, about 1.40 m wide and 1.35 m high | door leaves from 0.55 m, 2 × 0.86 m wide, 1.35 m high | ok |
| sliding door | right side, 1.0–1.05 m clear, pops out about 60 mm then slides 0.9 m back on three rollers | 1.05 m, slides 0.85 m straight back | no pop-out (open) |
| rear doors | two leaves on vertical hinges at the D-pillars, closed at rest, 180° (option 270°) | were: open at rest, closed by every pose, limits 0..0 | fixed |
| tail lamps | vertical clusters on the D-pillars, about 0.1 × 0.45 m, centre about 1.4 m | 0.085 × 0.42 m at 1.4 m; missing at 1:200, 1:500 | fixed |
| headlamps | swept-back lenses at the wing corners, about 0.35 × 0.17 m, centre 0.85 m | present; ragged lower edge at 1:50; missing at 1:500 | fixed |
| front bumper | dark plastic, about 0.3 m high, whole width | present at 1:50, 1:100; missing at 1:200, 1:500 (rear bumper was there) | fixed |
| high-mount stop lamp | on the roof header above the rear doors | missing | added 1:50, 1:100 |
| roof | longitudinal pressed swages | plain | open |
| structure | unibody: two floor rails, crossmembers and outriggers to the sills, body-side cage, MacPherson struts into strut towers tied to the hinge pillars by wing rails, rigid rear axle on leaf springs | 1:50 complete; 1:100 towers not tied back; 1:500 front wheels on a beam axle without struts, no body or axle masses; 1:200 no axle masses | fixed |

### What was done

- Rig (`rework-delivery-van-rig.py`, catalog): `rear-door-r` / `rear-door-l` rest closed (`twistDeg` −90 / +90 as rest placement about the vertical hinge bone), joint twist limits 0..180 / −180..0; the closing twist is removed from nine poses and `rear-doors-open` opens the leaves by ±90. `rig-lateral-compare.ts` before / after: largest move 0.0000 mm over every pose, clip instant, view and scale; rendered closed and open.
- Body (`rework-delivery-van-body.py`): 1:50 and 1:100 headlamp lenses 60 mm deep with crisp section (clean outline from the front), high-mount stop lamp; 1:200 front bumper (six stations), grille, tail lamps; 1:500 front bumper (four stations), headlamps, tail lamps; rear brake discs 15 / 30 / 75 mm at 1:100 / 1:200 / 1:500 (six `volumes-feature` findings that the per-scale rule had opened).
- Structure (`rework-delivery-van-structure.py`): 1:100 wing rails from hinge pillar to strut tower head; 1:500 strut towers on the rails and MacPherson struts on the axle bone; axle masses at 1:200 and 1:500, body mass at 1:500.
- 2D (`rework-delivery-van-drawings.py`, catalog): 1:200 front bumper in elevation and front (front outline follows the body shell), grille in front; 1:500 front bumper and headlamps in elevation and front. Tail lamps are not drawn at 1:200 and 1:500 (0.15 mm on paper, below the print minimum).
- Decisions: one "second round" sentence pair (en, de) per scale for body, and for structure at 1:100, 1:200, 1:500.
- Thermal: not changed.

### Validation

`r3-validate.ts vehicles/delivery-van --catalog-only --volumes` → `VALID vehicles/delivery-van (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (after the last change). Published with `volumes-insert.py`; source and catalog agree.

### Rendered and read

Before the changes: body 1:50 (six views, back and front close-ups, `side-door-open`, `rear-doors-open`), body 1:100, 1:200, 1:500, structure 1:50, 1:100, 1:200, 1:500, thermal 1:50. After the last change: 1:50 front close-up, `rear-doors-open` and rear three-quarter (candidate rig), `braking-nose-down`, `stop-and-unload` t 9 s, body 1:200 and 1:500, structure 1:100 and 1:500, thermal 1:100 and 1:500, 2D 1:50 `rear-doors-open`, 2D 1:200 and 1:500. Rendered but not read after the last change: `parked` sheet, `side-door-open`, `driving-loaded`, `drive-loop` t 0.3 / 0.8, `stop-and-unload` t 1.5, body 1:100, thermal 1:200.

### Remaining, plainly

- Rig: the sliding door slides straight back in the plane of the side wall, so in `side-door-open` and `stop-and-unload` its 45 mm panel lies inside the quarter panel instead of 60 mm outside it. It needs a lateral carriage bone (rest `abductionDeg` 90, prismatic) with the slider solids, thermal solids, members and 2D shapes re-bound; a tilted slide axis fails `lateral-ambiguous` (86 mm at 1.05 m).
- Rig: front wheels cannot steer (one bone per axle carries both wheels), cab doors and bonnet have no bones.
- Body 1:50: roof without swages (raised ribs would exceed `dimensions.heightM` 1.99; the roof loft has to drop 12 mm first); grille is a small three-bar rectangle; mirrors are octagonal boxes; no cab interior door trim.
- Body 1:100: no number plates, no door handles and rub strips (all above 15 mm).
- Thermal: bumpers, lamps and mirrors belong to no region; at 1:100 and 1:500 the painted panel solids render grey in `r3-sheet` (no layer colour) while 1:50 shows wall / paint pairs — not investigated; thermal 1:200 not read.
- 2D 1:200 / 1:500 front: `slider-panel` is drawn as a 15 mm sliver outside the body shell although the door is flush; the print rule measures its width in the lower corner the bumper outline now leaves free (pre-existing fiction).
- 2D 1:50 / 1:100: stop lamp not drawn (30 mm high), mirrors drawn as ellipses against boxes in 3D.
- Track 1.68 m against 1.70–1.74 m.

### Fundamental issues (outside this asset)

- The catalog file is written by `json.dumps(indent=1)` plus the inline-array format of `volumes-insert.py` for `volumes`; there is no shared writer for the other members, so every root edit reformats the file and then needs `volumes-insert.py` to restore the `volumes` member.
- `range-end-feature` reports only the shape id and the scale, not the view or pose; finding the cause took three validation runs of three minutes each.
