# Round 3: components/tied-canopy

Author: round-3 author (claude-code), 2026-11-20. Status: IN PROGRESS (updated step by step).

Frame: one bone `canopy` (joint at the wall face, beam axis at 2.33 m, +x along the beam, +y up, z lateral). The system line of the strut runs from the beam axis at x 4.0 (anchor `tie-rod`, `normalDeg` 144.246) to the wall face at y 2.88 (5.21 m above ground), slope tan 35.754 deg = 0.7200.

## 1. Reference

| item | real value | source |
|---|---|---|
| IPE 160 | h 160, b 82, t_w 5.0, t_f 7.4, r 9 mm; A 20.1 cm2; 15.8 kg/m; I_y 869 cm4 | EN 10365:2017 (asset reference `en-10365`) |
| beam mass | 15.8 kg/m x 5.5 m = 86.9 kg (rig 86.78 kg) | EN 10365 |
| strut | CHS 60.3 x 3.2 mm, A 5.74 cm2, 4.51 kg/m | EN 10219-2 table of CHS sizes (knowledge) |
| strut ends | tube strut systems (e.g. Pfeifer, Halfen, Macalloy tube struts): conical end piece welded into the tube, fork head (clevis) with a pin of about 0.4 x tube diameter (here 24 mm) on a single gusset (tongue) plate 12-20 mm thick | manufacturer catalogues of tension/compression rod systems (knowledge) |
| node rule | system lines of beam and strut meet in one point; the pin sits on the strut line above the glass, the gusset welded on the top flange carries the force down to the beam axis | EN 1993-1-8 practice (knowledge) |
| beam end | end plate welded to the beam, bolted with 4 M16 studs of a load-bearing thermal break element (e.g. Schoeck Isokorb type S, stainless studs through 80-180 mm insulation); edge distance e >= 1.2 d0 = 22 mm for M16 (d0 18) | EN 1993-1-8 table 3.3; Schoeck Isokorb S product data (knowledge) |
| nut M16 | across flats 24 mm (corner 27.7), height 13 mm | ISO 4032 |
| glass | laminated safety glass 2 x 8 mm float + 0.76 mm PVB = 16.8 mm (asset material `glass` "2 x 8 mm") | DIN 18008-2 overhead glazing (laminated glass required) |
| glass bearing | linear bearing on EPDM setting strip 5-10 mm, mechanical clamps against uplift at the pane joints and ends; joints 10-20 mm, sealed | DIN 18008-2, glazing practice (knowledge) |
| EPDM | lambda 0.25 W/(m K), rho 1150 kg/m3, c 1000 J/(kg K) | EN ISO 10456 table 3 |
| wall | concrete 300 + mineral wool 180 + render 20 mm (U about 0.17 W/(m2 K)) | asset `thermal.regions.wall-face`, EN ISO 6946 |
| glass fall | overhead glazing is usually laid with >= 2-3 deg fall for drainage and self-cleaning | glazing practice (knowledge); see section 6 |

## 2. Audit (before this round, product check: 3 failing measurements)

Rig and dimensions: the single bone, the anchors and `dimensions` (6.0 x 5.35 x 1.0 m, 86.78 kg) agree with the reference; nothing to correct there.

Findings in the 1:50 model and its 2D (numbers in bone-local metres):

1. Strut node at the beam is constructively wrong: two cheek plates (z 0.032-0.040, 8 mm) lie inside the flange outstand (flange tip z 0.041) and run from y -0.06 up to 0.134 straight through the top flange (y 0.0726-0.08); a pin of 24 mm passes through the web at the beam axis (4.0, 0) and a second pin (`pin-beam-2`) at (3.813, 0.134) pins the tube end into the same plates. Two pins in one rigid pair of plates is not a hinge, and the plates cannot pass through the flange.
2. Strut node at the wall: the bare tube end (60.3 mm) is pinned straight through its own wall between two cheeks 64 mm apart; no fork head, no end piece; the tube runs 0.05 m past the pin to the wall.
3. Glass lies directly on the steel top flange (glass bottom = flange top, 0.08), no bearing strip and no clamp against uplift; glass 20 mm thick instead of 16.8 mm; pane joints 60 mm wide (real 10-20); the glass stops 0.10 m short of the wall and 0.10 m short of the beam end, so the open beam end projects past the roof edge.
4. The beam end plate is 200 x 160 mm but the 2D elevation draws it 240 mm tall (`plate-beam`, `plate-strut` rects h 0.24): the 2D contradicts the 3D and the front view (0.20). The bolts sit at y +-0.085 in a 0.20 plate: edge distance 15 mm < 22 mm (EN 1993-1-8).
5. The beam end at the tip is an open cut I section (no end plate).
6. Thermal: `wall-plate` outer area 0.157 m2 at 1:50 against the decided 0.093 (the plate faces bonded to the separation pad are not marked as C25 b faces, finding of `model-findings.md`); 0.544 m2 at 1:100 against 0.093 (the merged anchor blocks expose their faces buried in the insulation). Product check FAIL at 1:50 and 1:100.
7. Structure: no `mass` solid at any scale; at 1:200 and 1:500 the strut has no joint at either end (QA `qa-visual.md`: "rod top end unsupported"); the support at the tie node is a pin through the web.
8. Coarser scales: 1:200 glass slab has an elliptical hole for a strut that does not pass there any more once the node is right; cross-scale 1:100 -> 1:200 plan p95 0.285 > 0.2175 (FAIL).
9. Asset data: `structure.assumptions[0]` says the rod direction is 144.04 deg while the anchor says 144.246 deg; clip `wind-gust` carries the same event "strut switches to compression" five times at t 2 s.

## 3. Root fixes

Scripts: `r3-tied-canopy-kit.py` (the hand-stated construction shared by 2D and 3D), `r3-tied-canopy-roots.py` (candidate `🗑️generated/r3-tied-canopy/🔣️.json`, validated with `r3-validate.ts --file`, then `--apply` writes only the changed members in the file's own formatting), `r3-tied-canopy-model.py` (volumes source), spliced with `volumes-insert.py`.

Construction (old -> new, bone-local m):

| part | old | new |
|---|---|---|
| beam pin of the strut | pin through the web at (4.000, 0) plus a second pin at (3.813, 0.134) in cheeks through the flange | one 24 mm pin at (3.7639, 0.1700) on the strut system line, 90 mm above the top flange, on a 15 mm gusset welded on the flange (base 3.735-3.995); the force line still meets the beam axis at x 4.0 (anchor `tie-rod` unchanged) |
| wall pin | tube pinned through its wall at (0.0812, 2.8216) between cheeks | 24 mm pin at (0.0900, 2.8152) on a 15 mm gusset welded on the anchor plate |
| strut ends | bare tube | fork head: two 10 mm cheeks (60 wide, R 30 round the pin, 17 mm slot), bridge, 36 mm shank, end cone to the 60.3 tube; tube length between pins 4.527 m, between cones 4.207 m |
| glass | 4 panes 1.28 m, 20 mm thick, x 0.10-5.40, joints 60 mm, on the bare flange | 4 panes 1.3525 m, 16.8 mm (2 x 8 + PVB), x 0.03-5.50, on an 8 mm EPDM strip; 20 mm EPDM joint gaskets, 30 mm EPDM wall joint, five clamp plates 60 x 80 x 8; pane 2 notched 25 mm wide round the gusset and sealed |
| beam end plates | end plate 200 x 160 x 20, bolts at y +-0.085 (edge 15 mm) | same plate, bolts at y +-0.073, z +-0.058 (edges 27 / 22 mm, nut corners clear of the flange tips); tip of the beam closed by an 8 mm plate |

Root edits in the catalog asset:

- `representations` 50/100/200/500: elevation, front and plan redrawn as the true views of the construction above (same ids where the part survives: `plate-*`, `glass-*`, `strut-tube`/`strut`, `strut-axis`, `clevis-*`, `pin-*`, `glass-roof`, `roof`, `beam`); new shapes `gusset-beam`, `gusset-wall`, `nut-*`, `beam-end-plate`, `bearing-wall`, `bearing-tip`, `joint-0..2`, `joint-wall`, `notch-seal`, `clamp-0..4` (1:50), gussets, joints and seal (1:100), `node-beam`, `node-wall` (1:200); removed `pin-beam-2` (the second pin does not exist). 1:50 elevation `plate-beam`/`plate-strut` h 0.24 -> 0.20 (now agrees with 3D and front). 1:100 glass strokes pen 0.13 -> 0.25 mm (true 25 mm). 1:200 glass pen 0.10 -> 0.125 mm, glass from x 0.10 -> 0.0 (wall joint merged). 1:500 beam bar h 0.18 -> 0.185, strut line now ends at the roof where its node stands. 2D `decisions` texts rewritten (en, de).
- `materials`: new `epdm` (EN ISO 10456: 0.25 W/(m K), 1150 kg/m3, 1000 J/(kg K)).
- `thermal.regions[].areaM2` from the honest 1:50 model (C16, factor within 1.5): `rod-steel` 1.12 -> 0.95 (measured 0.947: tube 0.80 between the cones + forks, gussets, pins; the old value assumed a 4.68 m tube to the wall face), `canopy-glass` 5.46 -> 5.73 (measured 5.730: glass now 0.03-5.50 m, 16.8 mm). `wall-plate` stays 0.093 (now met: 0.0928, the plate faces bonded to the pad marked `-u`, C25 b). `canopy-steel` 3.43 stays (3.506 measured, +2.2 %, end plate and clamps).
- `structure.assumptions[0]`: strut direction "144.04 deg" -> "144.25 deg" (the anchor's 144.246).
- `clips[wind-gust].events`: five identical events at t 2 s -> one.

Not changed (checked against the reference): rig (one bone, y 2.33 = underside 2.25 + 80), anchors, `dimensions` (6.0 x 5.35 x 1.0 m, 86.78 kg = 15.8 kg/m x 5.5 m), loads.

## 4. Model per scale

(in progress)

## 5. What I saw in the final renders

(in progress)

## 6. Still imperfect (stated plainly)

(in progress)
