# Round 3 — furniture/height-adjustable-desk

Resumed on 2026-11-20 from the interrupted round-3 session (scripts `r3-height-adjustable-desk-kit|model|draw|build.py`, never spliced, no report). This session added `-apply.py`, `-ship.sh`, `-suit.ts`, fixed the defects listed under "Resume findings", and spliced.

Catalog file: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/height-adjustable-desk/🔣️.json`
Source: `volumes/furniture/height-adjustable-desk.json`

## 1. Reference

| item | real value | source | modelled |
|---|---|---|---|
| top | 1600 x 800 x 25 mm laminated particleboard, about 650 kg/m3 | asset `dimensions`, common office format | 1600 x 800 x 25, 12 mm corner radii, 20.8 kg |
| height range (top surface) | 3-stage frames 0.60 to 1.25 m; this asset's poses 0.72 / 0.935 / 1.15 m | UPLIFT V2 F600 sheet (21.6 to 47.7 in), asset poses | 0.72 / 0.935 / 1.15 m, 2 x 215 mm travel |
| T-foot | 27.6 in (0.70 m) long, about 80 mm wide, 25 to 45 mm high, two levelling glides | UPLIFT V2 F600 | 700 x 80 mm, 28 mm at the ends rising to 45 mm, glides dia 40 at +-310 mm |
| column | three nested rectangular tubes; 95 x 66 mm outer (FlexiSpot E7), 60 x 90 (LINAK DL21), 40 x 80 / 46 x 86 / 52.6 x 92.6 (Brateck) | manufacturer data, author's knowledge | 64 x 94, 56 x 86, 48 x 78 mm, 15 mm plastic slide bushings |
| tube overlap at full height | at least about 100 mm | author's knowledge | 140 mm |
| motor housing | box at the column head, about 75 x 75 x 180 mm along the side rail | FlexiSpot E7 | 80 x 100 x 150 mm |
| top frame | two side rails across the depth, crossbar between the column heads | UPLIFT, FlexiSpot | rails 60 x 25 x 660 mm, crossbar 50 x 30 mm with 8 mm end flanges |
| frame mass | about 30 to 35 kg | FlexiSpot E7 | 31.3 kg (tube walls x 7850 kg/m3 + 2.5 kg per drive) |
| monitor 27 in | panel about 614 x 368 mm, 8 kg with stand | author's knowledge | 614 x 368 x 15 mm panel, rear bulge, hinge, neck, base 250 x 200 x 18 |
| laptop 15.6 in | about 360 x 250 x 20 mm, 2 kg | author's knowledge | 360 x 250 x 20, base and lid |

The manufacturer figures are from the previous session's reference list and my own knowledge; I did not re-fetch them this session.

## 2. Audit against the reference (catalog as found)

- Rig: feet 0.5 m long at x 0.25 (real 0.70 m, column not centred on the foot width); stage 0.5 m and column 0.445 m with the column joint 0.2 m up the stage; top joint at the column, no lateral offset.
- Masses: feet 2.8 kg, columns 3.0 kg, stages none, top 20.8 kg (board only); `massKg` 32.4 for a desk whose frame alone weighs about 30 kg.
- Anchors: foot footprint 0.4 x 0.06 m (real 0.70 x 0.08).
- Loads: monitor at x 0.35, laptop at 0.82, not where they were drawn.
- Thermal: no region for the control box; `steel-desk` 2.44, `monitor-housing` 0.56.
- Solids: the old model, 35 / 22 / 17 / 14 body solids; structure without crossbar joints or equipment masses (qa-visual: B in thermal and structure at every scale).

## 3. Root fixes (old to new)

| member | old | new |
|---|---|---|
| `foot-*` x, lengthM | 0.25 (right 1.1), 0.5 | 0.21 (right 1.1 from left), 0.08 (the foot width; the 0.70 m length is lateral) |
| `stage-*` x, y, lengthM | 0, 0.05, 0.5 | 0.04, 0.05, 0.43 |
| `column-*` x, lengthM | 0.2, 0.445 | 0.08, 0.565 |
| `top` x, y | 0.445, 0 | 0.565, 0.25 |
| masses kg (foot / stage / column / top) | 2.8 / none / 3.0 / 20.8 | 4.94 / 1.76 / 4.37 / 29.93 |
| `dimensions.massKg` | 32.4 | 52.1 (20.8 board + 31.3 frame) |
| contact anchors x, footprint | 0, 0.4 x 0.06 | 0.04, 0.08 x 0.70 |
| loads x (monitor / laptop) | 0.35 / 0.82 | 0.6 / 1.07 |
| thermal region `controls` | absent | added: 0.08 m2, 6 W while lifting, in every pose |
| `steel-desk` areaM2 | 2.44 in the catalog (2.15 in `area-decisions.md`) | 1.6 |
| `monitor-housing` areaM2 | 0.56 (0.5 in `area-decisions.md`) | 0.65 |
| summary, provenance notes and references, bone titles, thermal and structure assumptions | old construction | rewritten for the construction above |
| 2D, all four scales and three views | old drawings | redrawn from the kit as true views of each scale's model (E1) |

Area hand calculation for `steel-desk` at 1:50: feet 0.32 + outer tubes 0.24 + visible stage and inner tube 0.08 + tube inside the housings 0.06 + side rails 0.22 + crossbar and flanges 0.18 + open tray (both faces) 0.55 = 1.65 m2; measured 1.607.

## 4. Resume findings fixed this session

- `rounded_rect` in the kit had its corner arcs shifted by a quarter turn, so every rounded tube and the board corners carried spikes. Fixed.
- Structure had a gap: the inner tube ended 110 mm below the head plate and the crossbar ended in the plastic housing. The tube now runs through the housing to the head plate, the crossbar reaches the tube faces and has flange joints.
- Board layer shells were two loose laminate plates (area 4.9 against 2.68). Now the laminate is the board solid and the core sits inside it.
- 1:100 and 1:200 monitor panel floated 7 mm in front of its stand. The panel block now reaches the stand.
- 1:100 was not printable (`monitor-base` 0.186 mm against 0.2 mm): the front view drew the board's pen over the monitor base. Monitor and laptop are now drawn above the board in the front view.
- 1:500 structure equalled 1:200 in counts (monotonic rule); slide and flange joints are dropped at 1:500.
- Support pads and slide joints coincided with their members (striped faces); pads stand proud of the footprint, joints are collars 2 mm proud.
- Motor housing reshaped from 80 x 120 x 120 to 80 x 100 x 150 mm; tray kept as an open sheet U at 1:100.

## 5. What is modelled

| scale | body | thermal | structure | content |
|---|---|---|---|---|
| 1:50 | 31 | 32 | 29 | glides, tapered pressed feet, three rounded tubes with bushings per column, motor housings, side rails, crossbar with flanges, sheet tray with screw flanges, control box, handset, rounded board, monitor (base, neck, hinge, panel, lofted bulge, screen), laptop base and lid; board as laminate shell and core; structure with slide collars, head plates, flange joints, contact pads, seven masses |
| 1:100 | 20 | 20 | 29 | no glides, bushings, handset, hinge, bulge, flanges; tapered foot kept, tray as plain open U |
| 1:200 | 18 | 18 | 28 | straight foot bars, housings up to the board, tray as block, monitor base, neck and panel block |
| 1:500 | 16 | 16 | 23 | sign: feet, three tubes and housing per column, board, crossbar, control box, monitor panel and stand, laptop; structure without slide and flange joints |

Final check lines: `volumes-check.ts furniture/height-adjustable-desk --product` gives `PASS (12.2 s)`; `r3-validate.ts furniture/height-adjustable-desk --volumes` gives `VALID ... schema 0, validateAsset 0, checkVolumes 0`. Print suitability is `authored` with no violations at all four scales (`r3-height-adjustable-desk-suit.ts`).

Silhouette IoU (elevation / front / plan): 1:50 0.999 / 0.991 / 0.999; 1:100 0.999 / 0.985 / 0.999; 1:200 and 1:500 1 / 1 / 1. Area deviation at 1:50: top-board 0.000, steel-desk 0.004, drives 0.128, controls 0.059, monitor-housing 0.001, laptop-lid 0.000.

## 6. What I saw in the final renders

Sheets under `🗑️generated/r3-sheet/furniture/height-adjustable-desk.s<N>.<list>.png`, all twelve opened after the last geometry change, plus the 1:50 thermal cut, the 1:50 standing-height pose, a 1:100 structure clip frame, a 1:50 view from below, and the 2D sheets of all four scales and the standing pose.

- 1:50 body. Front: tapered foot on two glides, three stepped tubes with dark bushings, housing under the rail, tray as an open U, handset just proud of the front edge, monitor edge-on with bulge and neck. Side: the full desk with crossbar entering the housings, tray and control box behind, monitor and laptop on the board. Back and left: the rear bulge and neck of the monitor. Top: rounded board corners, monitor and laptop. From below: rails, flanged crossbar, tray, housings, glides all present and attached.
- 1:50 thermal. Regions read by colour; the cut shows the core inside the laminate and the steel tube inside each housing.
- 1:50 structure. Feet, tubes, head plates, rails, crossbar and board form one closed path; orange collars and flanges at the real joints; green pads under the feet; monitor, laptop and top masses visible.
- 1:100, 1:200, 1:500. Each reads as the sit-stand desk at a glance and is simpler than the last; the monitor is attached to its stand at every scale; structure pads and joints are distinct from the members.
- Standing height. Both columns extend evenly, tubes stay overlapped, nothing opens.

## 7. Still imperfect

- No cables (motor, handset, mains) are modelled or drawn; the decisions say so.
- The foot, stage and column mass spheres (16 mm) sit inside their tubes and cannot be seen in the structure sheets.
- The crossbar hangs 65 mm below the board at the column heads. Many current frames carry the crossbeam directly under the board instead; both exist, I kept the previous session's choice.
- `steel-desk` 1.6 m2 is a factor 1.34 below the 2.15 of `area-decisions.md` (inside C16's 1.5) but 1.52 below the 2.44 that was in the catalog when I started. I could not find where 2.44 came from; the area reviewer should confirm 1.6.
- `drive-left` / `drive-right` keep 0.075 m2; the reshaped housing measures 0.065 (12.8 %, inside the limit).
- Plan depth at 1:50 reads 0.81 m because the handset stands 12 mm proud of the 0.80 m board; `dimensions.depthM` stays 0.8.

State: DONE

## Round 5 handcraft (2026-11-21)

Sheets rendered with `sh r5-sheets.sh furniture/height-adjustable-desk before "mid-height standing-height" "raise-and-lower 12"`. Opened:
- body 1:50 (six views with rig);
- a montage of body 1:100 / 1:200 / 1:500 and structure 1:50;
- a montage of the 1:50 thermal cut, structure and thermal cut at 1:100 / 1:200 / 1:500, and `raise-and-lower` at 12 s;
- a montage of `mid-height` and `standing-height` at 1:50 with the 2D of all four scales;
- the 1:50 body from below (turn 30, tilt -22).

Measurements: `r5-furniture-extent.ts furniture/height-adjustable-desk 50`.

### Reference against model (1:50, seated-height)

| Quantity | Reference | Model | Deviation |
| --- | --- | --- | --- |
| Board | 1600 x 800 x 25 mm | 1600 x 800 x 25 | 0 % |
| Top surface, seated | 720 mm (asset pose; frames 600-1250) | 720 | 0 % |
| T-foot | 700 x 80 mm, 25-45 mm high (UPLIFT V2) | 700 x 80 x 35 (taper 28-45) | 0 % |
| Outer column tube | 64 x 94 mm (95 x 66 FlexiSpot E7, 60 x 90 LINAK DL21) | 64 x 94 | 0 % |
| Stage / inner tube | 56 x 86 / 48 x 78 mm | 56 x 86 / 48 x 78 | 0 % |
| Motor housing | about 80 x 100 x 150 mm | 80 x 100 x 150 | 0 % |
| 27 in monitor panel | 614 x 368 mm, active 598 x 336 | 614 x 368, screen 598 x 336 | 0 % |
| 15.6 in laptop | 360 x 250 x 20 mm | 360 x 250 x 20 | 0 % |
| Overall height (monitor top) | 1.18 m (asset) | 1.180 | 0 % |

### Grades (before = after; no change)

| Scale | Body | Thermal | Structure |
| --- | --- | --- | --- |
| 1:50 | A | A | A |
| 1:100 | A | A | A- |
| 1:200 | A | A | A- |
| 1:500 | A- | A | A- |

What I saw:
- **1:50.** The tapered T-feet stand on glides, with three telescoping tubes and bushings, motor housings and side rails. The crossbar, open cable tray and control box sit under the rounded board. The monitor has its screen facing the user, the rear bulge, the neck and the base. Seen from below, everything is attached.
- **Poses and clip.** `mid-height`, `standing-height` and `raise-and-lower` at 12 s extend both columns evenly. The tubes stay overlapped and nothing opens.
- **Coarser scales.** These simplify cleanly down to a 1:500 sign that still reads as a sit-stand desk with a monitor.
- **Thermal and structure.** Thermal is tidy, and the cut shows the board core inside the laminate. The structure is a closed path of feet, tubes, head plates, rails, crossbar and board, with collars, pads and masses.

Checks: `r3-validate.ts furniture/height-adjustable-desk --volumes` gives `VALID ... schema 0, validateAsset 0, checkVolumes 0` (print included). `volumes-check.ts --product` gives `PASS (51.1 s)` with 41 lines ok.

Changes: none. Every measure is within 3 % and I found no defect at grade B or worse.

Open points:
- No cables are modelled.
- The laptop is shown closed.
- The crossbar is 65 mm below the board: some current frames carry it directly under the board, and both kinds exist.
- The coarse structure grades are A- only because masses and pads are hidden in the tubes.

The current 27-inch monitor has a 614 × 368 × 15 mm bezel, 598 × 336 × 2 mm cover glass, a separate backplane, a 3 mm rear panel, four M4 VESA fixings on a 100 mm square and a horizontal tilt axle. Its 97-frame tilt check preserved the spindle center and rigid screen dimensions from −5° to 20°. The authored heat allocation is 8 W at the glass and 17 W at the back. The stand transfers the 78.5 N display load into the worktop. The independent reader opened all twelve exported scenes (1,823,996 bytes).
