"""🧾 Round 5 appliances (E03): appends the `## Round 5 handcraft` section to each asset report (r3-appliances-<slug>.md) and one line per asset to r5-ledger.md; every file is re-read right before it is written, a section already present is not appended twice.

Usage: python3 r5-appliances-report.py
@see ./r5-brief.md
"""
import pathlib

HERE = pathlib.Path(__file__).parent
COMMON = """
Process (all assets of E03; final `r3-validate.ts <asset> --catalog-only --volumes` VALID for all five after the splice): `r5-appliances-render.sh` rendered every list at every scale, every pose and both clips at 1:50 (t 0.5 s) and the 2D of every drawn scale into `🗑️generated/r5-<slug>/before` and `/after`; `r5-seated-montage.ts` laid them out for reading; every sheet named below was opened with the Read tool. `r5-appliances-measure.ts` measured the body extents per scale in the default pose (numbers below). Grades: body / thermal / structure.
"""

SECTIONS = {
    "server-rack": """
## Round 5 handcraft (2026-11-21)
""" + COMMON + """
### Reference and model (1:50, default pose `full-load`)

| quantity | reference | source | model 1:50 | deviation |
|---|---|---|---|---|
| width x height x depth | 600 x 2000 x 1000 mm | Rittal TS IT DK 5509.161 (800 x 2000 x 1000), DK 5529.110 (600 x 2000 x 600); asset dimensions | 600 x 2000 x 1015 mm (handle 15 mm proud of the door) | 0 / 0 / +1.5 % (handle only; carcass 1000) |
| 19-inch front | 482.6 mm | IEC 60297-3-100 | 482.6 mm | 0 % |
| 12U server block | 12 x 44.45 = 533.4 mm | IEC 60297-3-100 | 532.6 mm | -0.15 % |
| caster wheel | 50 mm | StarTech RKCASTER2 | 50 mm | 0 % |
| door field | 537 x 1864 mm in a 600 x 2000 leaf | Rittal TS IT door (product knowledge) | 537 x 1864 mm | - |
| coarse scales | same envelope | - | 1:100 0.600 x 2.000 x 1.015, 1:200 and 1:500 0.600 x 2.000 x 1.000 | 0 % |

### Grades

| scale | before | after |
|---|---|---|
| 1:50 | A / A- / A- | A / A- / A- (unchanged) |
| 1:100 | A- / A- / A- | unchanged |
| 1:200 | A- / A- / A- | unchanged |
| 1:500 | B+ / A- / B+ | unchanged |
| poses `idle`, `off`, clips `load-ramp`, `power-up` (t 0.5) | A (rigid frame, nothing moves) | unchanged |

Seen: body 1:50 (six views, a close three-quarter from the door side turn 300 and from the side turn 70), 1:100, 1:200, 1:500; structure and thermal cut at all four scales; the three poses and both clip frames at 1:50; 2D 1:50, 1:100, 1:200, 1:500. The 1:50 rack reads as a Rittal-class cabinet at first sight: perforated door with handle, lock and three hinges, divided side panels with louvres, split vented rear door, roof with two guarded fans and a brush slot, swivel casters with fork and wheel; the structure is the welded frame with corner joints, 19-inch angles, the unit masses on the angles and the wheel contact ring. No change was needed for this round.

### Open points

- 1:500 door face with two windows (top group and server block 10 mm behind the door plane) follows its own 2D; at 2 x 4 mm on paper it reads as a vented cabinet, but a single perforated field would read more like a rack. Left as drawn (2D and 3D agree, the change would be cosmetic at the 75 mm feature).
- Cabinet mass sphere sits at the empty centre of the frame (physically right, touches nothing); fine 6 x 6 mm sight lines through both perforated doors in the open units (stated in round 3).
- `r3-sheet` shades the flat caps of extrudes with holes with streaks (renderer, filed in round 3).
""",
    "tower-server": """
## Round 5 handcraft (2026-11-21)
""" + COMMON + """
### Reference and model (default pose `typical`)

| quantity | reference | source | model 1:50 | deviation |
|---|---|---|---|---|
| height x width x depth | 440 x 195 x 480.5 mm / 360 x 175 x 453.75 mm | HPE ProLiant ML110 Gen10 / Dell PowerEdge T150 | 450 x 200 x 450 mm (asset dimensions, class between the two) | ML110 +2.3 / +2.6 / -6.3 %; T150 depth -0.8 % |
| power supply | 150 x 86 x 140 mm | ATX12V Design Guide 2.2 | 150 x 86 x 139 mm | 0 / 0 / -0.6 % |
| rear fan | 120 x 120 x 25 mm | product practice | 120 x 120 x 25 mm | 0 % |
| bezel depth | 25 mm | product knowledge | 25 mm (1:50, 1:100), 30 mm (1:200), 75 mm (1:500, one feature) | - |
| feet | 40 x 12 mm, footprint 0.4 x 0.17 m | asset anchor | 40 x 12 mm, 0.40 x 0.18 m | - |
| coarse scales | same envelope | - | 0.450 x 0.450 x 0.200 m at 1:100, 1:200, 1:500 | 0 % |

### Grades

| scale | before | after |
|---|---|---|
| 1:50 | A- / A- / A- | unchanged |
| 1:100 | A- / A- / A- | unchanged |
| 1:200 | B+ / A- / A- | unchanged |
| 1:500 | C+ / B+ / B+ | A- / B+ / B+ |
| poses `rated`, `standby`, `off`, clips `power-up`, `load-surge` (t 0.5) | A (one fixed bone) | unchanged |

Seen before: body at all four scales, structure and thermal cut at all four scales, the four poses and both clip frames at 1:50, 2D 1:50, 1:100, 1:200 (1:500 has no drawing). After: 1:500 body, thermal cut and structure.

### Change

- 1:500 body (`r5-tower-server-500.py`): the round-3 block had a bezel extrude with a 160 x 300 mm window 75 mm deep and a rear plate with power-supply and fan pockets 75 mm deep; at the 75 mm feature they read as holes into a hollow box (three-quarter view) and split the side views into three bands. Now three solids: rubber plinth, light steel case block with the rear panel merged (375 mm deep), dark ABS bezel block of one feature. Decisions rewritten (en, de). Thermal skins (side, grille, bezel, power supply, rear, feet, outlets) unchanged, they lie on the faces of the new block. `volumes-check --product` PASS (30.6 s); cross-scale, monotonic and copied ok.

### Open points

- 1:200 windows onto a dark core and the 12-gon fan circle are the tessellation of the coarse scale (renderer quality at the 30 mm feature), kept.
- Panels as deep as their folded edges, square sheet edges and the 8 mm honeycomb stay as stated in round 3.
""",
    "desktop-monitor": """
## Round 5 handcraft (2026-11-21)
""" + COMMON + """
### Reference and model (default pose `on`)

| quantity | reference (Dell SE2422H) | model | deviation |
|---|---|---|---|
| height with stand | 420.3 mm | 410.9 mm (1:50, 1:100, 1:200), 413.9 mm (1:500) | -2.2 % (1:500 -1.5 %) |
| width | 552.6 mm | 540 mm | -2.3 % |
| depth with stand | 178.8 mm | 210 mm (asset `dimensions`, base plate 250 x 210 mm) | +17 % (defect, see open points) |
| panel head height | 331.6 mm | 322.6 mm | -2.7 % |
| active area 23.8 in | 527.0 x 296.5 mm | 527.0 x 296.1 mm | 0 / -0.1 % |
| panel depth | 49.7 mm | 46.4 mm (housing) + glass, 37.6 mm slab | within the class |
| stand base | 240.4 x 178.8 mm | 250 x 210 mm | +4 % / +17 % |

### Grades

| scale | before | after |
|---|---|---|
| 1:50 | A- / A- / B+; catalog INVALID (print check: 2 mm pads 0.190 mm at 1:20) | A- / A- / B+; VALID |
| 1:100 | A- / A- / B+ | unchanged |
| 1:200 | A- / A- / B+ | unchanged |
| 1:500 | B / B- / C+ (bulk copy: thermal lofts and structure copied from 1:200, lopsided base outline, one-sentence decision) | A- / A- / B+ |
| poses `standby`, `off`, clips `power-on-warm-up`, `standby-cool-down` (t 0.5) | A (tilt hinge at the head of the riser) | unchanged |

Seen before: body at all scales, structure and thermal cut at all scales, the three poses and both clip frames, 2D 1:50, 1:100, 1:200. After: 1:50 body and 2D, 1:500 body, thermal cut and structure.

### Changes

- Root, 1:50 (`r5-desktop-monitor-pads.py`, candidate validated before writing): the four 2 mm rubber pads were below the 7.5 mm feature and below 0.2 mm at the 1:20 end of the band, which refused the catalog asset (four `PRINT range-end-feature` lines). They now merge into the base plate in 2D (foot shapes of elevation and front removed, plate drawn down to the desk) and in 3D (pad solids and their thermal references removed, plate loft to the desk); the steel core, its mass and the contact pad are now 7.5 mm (the pad drawing no longer lowers the feature). The 1:50 representation decisions (silhouette, retained, thinElements, styles) and the volume decisions say so in en and de.
- 1:500 (`r5-desktop-monitor-500.py`): authored anew. Body: panel prism of the true head outline 540 x 323 mm with the edge thickening from 14 to 35 mm, tapered riser, base plate 250 x 210 mm with even chamfers (was a trapezoid chamfered at the back only). Thermal: 2 mm front skin, the housing as the rest of the prism, plume outlet flush on the back, stand by reference (the 1:200 lofts copied into 1:500 are gone). Structure: plate and riser as members, the LCD frame as one plate across the VESA field, the tilt hinge as a joint, panel mass, contact pad. Decisions written for body, thermal and structure.
- `volumes-check --product` PASS (9.7 s).

### Open points

- Depth 0.21 m against 0.1788 m of the SE2422H (+17 %) and base 250 x 210 against 240 x 179 mm: the asset declares a class estimate (round 3 kept it); correcting it changes `dimensions`, the base bone, the anchor, the 2D of three scales and the 3D of four. Not done in this round; it is the one measure outside 3 %.
- Structure 1:50 to 1:200: the LCD module frame as a ring with rail and stile is an idealisation of the steel back tray (B+).
""",
    "laptop": """
## Round 5 handcraft (2026-11-21)
""" + COMMON + """
### Reference and model (default pose `open`, 105 degrees)

| quantity | reference | source | model | deviation |
|---|---|---|---|---|
| width | 313.6 / 312.6 mm | ASUS Zenbook 14 UX3402 / MacBook Pro 14 | 313.0 mm (all scales) | -0.2 % |
| depth (chassis) | 220.6 / 221.2 mm | same | 218.0 mm | -1.2 % |
| closed thickness | 16.9 / 15.5 mm | same | 17 mm + 2 mm feet | +0.6 % |
| panel 14 in 16:10 | 301.6 x 188.5 mm | diagonal | 301.6 x 188.5 mm | 0 % |
| trackpad | 130 x 78-80 mm | product practice | 130 x 78 mm | 0 % |
| keyboard well | about 280 x 105 mm | product practice | 279 x 104 mm | -0.4 / -1 % |
| hinge barrel | 8-10 mm | product practice | 8.9 mm | - |
| open envelope | 0.221 x 0.273 m (asset dimensions) | - | 0.2211 x 0.2725 m (1:50), 0.2212 x 0.2731 m (1:500) | 0 / +0.1 % |

### Grades

| scale | before | after |
|---|---|---|
| 1:50 | A / A- / A-; catalog INVALID (print: hidden plan barrel 0.187 mm at 1:20) | A / A- / A-; VALID |
| 1:100 | A- / A- / A- | unchanged |
| 1:200 | A- / A- / A-; catalog INVALID (print: 6.5 mm lid 0.199 mm at 1:350) | unchanged; band ends at 1:320 |
| 1:500 | A- / A- / B+ | unchanged |
| poses `open-idle`, `closed`, clips `open-lid`, `work-and-sleep` (t 0.5) | A (lid closes flush onto the deck, half-open lid clear of the chassis) | unchanged |

Seen: body at all scales, structure and thermal cut at all scales, three poses and both clip frames at 1:50, 2D 1:50, 1:100, 1:200. The clamshell reads right at first sight: tub with bilge and feet, 78 keys with inverted-T arrows, glass trackpad, speaker grilles, hinge barrel at the real axis, display in its bezel; closed, it is one 19 mm slab. The model needed no change; the catalog asset was refused by the print check.

### Change (root, `r5-laptop-print.py`, candidate validated before writing)

- 1:50 plan: the hinge barrel shape lay under the lid shell, only a 1 mm sliver of it stayed visible (0.187 mm at the 1:20 end of the band); it is removed from the plan, the silhouette decision says the lid band hides the barrel (en, de). The 3D barrel stays (it is hidden in plan in 3D as well).
- 1:200 `validRange.maxDenominator` 350 -> 320: the 6.5 mm lid with the 0.18 mm body pen reaches 0.200 mm exactly at 1:320 (6.5 / 320 + 0.18); the 1:500 drawing is unavailable, so no band starts at 1:350. 2D 1:50 and 1:200 re-rendered and opened after the change.

### Open points

- 1:500 keeps a dark screen inset in the lid at the 75 mm feature (digital only), structure there is two plates on one hinge axle (B+).
""",
    "electric-kettle": """
## Round 5 handcraft (2026-11-21)
""" + COMMON + """
### Reference and model (default pose `boiling`)

| quantity | reference | source | model | deviation |
|---|---|---|---|---|
| height | 205-305 mm (1.7 L class), asset 240 mm | Russell Hobbs Classics, Ariete 2869/2877, Bosch TWK7203 | 240.0 mm (1:50), 238 (1:100), 248 (1:200), 248 -> 240 mm (1:500 after) | 1:200 +3.3 % |
| width spout to handle | 220-230 mm, asset 245 mm | same | 248.3 mm (1:50), 246.5, 250.4, 251.7 mm | +1.3 % to +2.7 % |
| depth (jug diameter) | 160-220 mm, asset 180 mm | same | 180 mm (1:50, 1:100, 1:500), 190 mm (1:200 base) | 0 % / +5.6 % at 1:200 |
| power base | about 180 mm, 25-30 mm high | product knowledge | 180 x 30 mm | 0 % |
| jug | r 75 to 70 mm cone, 175 mm | round 3 | r 74.8 mm at 1:50; 1:500 was a barrel r 88 mm, now r 79 to 71 mm | 1:500 before +18 % |
| fill 1.0 L | 59.5 mm deep | asset mass | 59.5 mm (1:50), 60 mm (1:500; was 75 mm and 4 mm outside the jug wall) | - |

### Grades

| scale | before | after |
|---|---|---|
| 1:50 | A- / A- / B+ | unchanged |
| 1:100 | A- / A- / B+ | unchanged |
| 1:200 | B+ / A- / B+ | unchanged |
| 1:500 | C+ / C / C+ (barrel-bellied jug wider than the real one, flat fin spout, octagon plate handle, water through the wall, lid and four structure solids copied from 1:200, one-sentence decision) | A- / A- / B+ |
| poses `off`, `heating`, clips `boil`, `rattle` (t 0.5) | A- (lid rattle 3 mm, skirt keeps it closed) | unchanged |

Seen before: body at all scales, structure and thermal cut at all scales, three poses and both clip frames, 2D 1:50, 1:100, 1:200. After: 1:500 body, thermal cut and structure (twice, before and after lowering the lid).

### Change

- 1:500 (`r5-electric-kettle-500.py`): authored anew. Body: power base, the jug as the true cone r 79 to 71 mm with the rolled rim, the spout as a two-section taper from its 60 mm root to the 18 mm mouth, the C handle as a five-section round bow through its two fixings, lid with dome and knob in one turn ending at the 0.24 m height (was 8 mm over). Thermal: references, the 1 L fill 60 mm deep inside the jug, steam outlet in the mouth, plume sleeve 2 mm over the new spout. Structure: base rim wall with top deck, jug cup 6 mm on an 8 mm floor, handle, lid plate, water slab and mass, contact ring. Decisions for body, thermal and structure (en, de). `volumes-check --product` PASS (7.8 s), `volumes-copied` 0 against every finer scale.

### Open points

- 1:200 keeps its 40 mm stepped lid and 40 mm base (height 0.248 m, +3.3 %), which its own 2D draws; not changed in this round.
- Teapot-like conical spout at 1:50 to 1:200 matches the covered spout of the class but is a separate cone on the shoulder with a visible seam; lid flange facets are tessellation.
- Structure from outside still looks like the kettle (monocoque jug), as stated in round 3.
""",
}

LEDGER = {
    "server-rack": "appliances/server-rack | 1:50 A/A-/A-, 1:100 A-, 1:200 A-, 1:500 B+/A-/B+ | unchanged (no defect needing a change found); VALID with volumes | 1:500 door with two equipment windows (as drawn); mass sphere in the empty frame",
    "tower-server": "appliances/tower-server | 1:50/1:100 A-, 1:200 B+/A-/A-, 1:500 C+/B+/B+ (75 mm deep window pockets read as holes) | 1:500 body A- (plinth, case block, bezel block); others unchanged; VALID with volumes, check PASS | dimensions are a class between ML110 and T150 (depth -6.3 % to ML110, -0.8 % to T150)",
    "desktop-monitor": "appliances/desktop-monitor | 1:50-1:200 A-/A-/B+, 1:500 B/B-/C+ (bulk copy); catalog INVALID (print: 2 mm pads at 1:20) | 1:50 pads merged at the root (2D and 3D), VALID; 1:500 authored anew A-/A-/B+; check PASS | depth 0.21 vs SE2422H 0.179 m (+17 %, class estimate kept); LCD frame idealised",
    "laptop": "appliances/laptop | 1:50 A/A-/A-, 1:100-1:500 A-, 1:500 structure B+; catalog INVALID (print: hidden plan barrel at 1:20, lid at 1:350) | model unchanged; plan barrel removed at 1:50, 1:200 band ends at 1:320; VALID with volumes | 1:500 two plates on one axle; gap 1:320-1:350 between bands",
    "electric-kettle": "appliances/electric-kettle | 1:50/1:100 A-/A-/B+, 1:200 B+, 1:500 C+/C/C+ (barrel jug, fin spout, plate handle, water through the wall, 1:200 copies) | 1:500 authored anew A-/A-/B+; check PASS, VALID with volumes | 1:200 lid +8 mm (as drawn); monocoque structure reads as the body from outside",
}

for slug, section in SECTIONS.items():
    path = HERE / f"r3-appliances-{slug}.md"
    text = path.read_text()
    if "## Round 5 handcraft" not in text:
        path.write_text(text.rstrip("\n") + "\n" + section)
        print("[r5] report", path.name)
ledger = HERE / "r5-ledger.md"
text = ledger.read_text()
lines = [line for line in LEDGER.values() if line.split(" | ")[0] not in text]
if lines:
    ledger.write_text(text.rstrip("\n") + "\n" + "\n".join(lines) + "\n")
    print("[r5] ledger", len(lines))
