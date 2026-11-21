"""📝 Round 5 (E01 appliances): appends the `## Round 5 handcraft` section to the report of each asset and one line per asset to `r5-ledger.md`; every file is re-read right before it is written and a section already present is not written twice.

Usage: python3 r5-e01-report.py
"""
import os

HERE = os.path.dirname(os.path.abspath(__file__))
COMMON = """
Process (round 5, 2026-11-21): rendered and opened with the Read tool, before and after: `r3-sheet` body, structure and `--list thermal --cut` at 1:50, 1:100, 1:200 and 1:500, every pose at 1:50 and one clip frame, the `r3-2d` drawings of every drawn scale (1:500 has no drawing: its representation is unavailable, the 3D 1:500 is a digital scale). Coarse sheets were opened as 2 x 2 contact sheets (`r5-grid.ts`, half size). Measurements with `r5-e01-measure.ts` (world extents per solid in the default pose). Root edits through `r5-e01-roots.py` (candidate validated with `r3-validate.ts --file --volumes` before writing), volumes through the asset's `r5-<slug>-model.py` (which re-uses the hand-stated round-3 lists it keeps and states every new solid by hand), splice with `volumes-insert.py`. The round-3 reference table above stays the reference; the table below measures the model against it.
"""

SECTIONS = {
    "gas-boiler": ("r3-appliances-gas-boiler.md", """
## Round 5 handcraft
""" + COMMON + """
### Model against reference (Vaillant ecoTEC plus 720 x 440 x 335 mm, 80/125 flue, 22 mm connections)

| Quantity | Reference | Model 1:50 | Deviation |
|---|---|---|---|
| Casing height | 720 mm | 720 mm (y 0.05 to 0.77) | 0 % |
| Casing width | 440 mm | 440 mm | 0 % |
| Casing depth (hood + chassis) | 335 mm | 330 mm (310 + 20) | -1.5 % |
| Over the knobs | about 350 mm | 351 mm | 0 % |
| Flue turret / spigot | 125 / 80 mm | 125 / 80 mm | 0 % |
| Overall height with flue and stubs | 0.82 m (`dimensions`) | 0.820 m | 0 % |
| Connections | 5 x 22 mm, 80 mm pitch | 5 x 22 mm, 80 mm pitch, nuts 31 mm AF | 0 % |
| 1:100 / 1:200 / 1:500 overall | 0.44 x 0.82 x 0.35 m | 0.44 x 0.82 x 0.350 / 0.340 / 0.340 m | within one feature |

### Grades (body / thermal / structure)

| Scale | Before | After |
|---|---|---|
| 1:50 | A- / A- / A- | A- / A- / A- (unchanged, re-read: hood, porthole, flame crowns, fascia, flue adapter, stubs with nuts; nested internals; chassis load path) |
| 1:100 | A- / A- / A- | A- / A- / A- (unchanged) |
| 1:200 | B+ / B (expansion vessel a square 0.31 m slab, fan a cube) / A- | A- / A- / A- |
| 1:500 | C+ (inherited digital model: asymmetric 13/35 mm chamfers on the front edges, a 9 mm hollow ring as flue collar below the 75 mm feature, one generic decision sentence) / B / B | A- / A- / A- |
| Poses off, heating, hot-water, clip heating-cycle t 2 s | A- | A- (flame crowns sink into the hood as designed; nothing torn) |
| 2D 1:50 / 1:100 / 1:200 | A- | A- |

Catalog before round 5: INVALID (`PRINT range-end-feature` of the 1:200 control band at 1:350; `connections` outer area 0.034 against 0.04 m2, 15 %).

### Changes

- 1:200 body: the control band is a 30 mm strip that stands 15 mm proud of the hood and runs 15 mm into it; the 1:200 front and plan drawings draw it 30 mm deep (root edit `representations.200.geometry.front/plan.panel`), so the band prints to the end of the range 1:350 (the 15 mm band printed 0.173 mm there).
- 1:200 structure: because the thinnest 1:200 casing drawing is now 30 mm, the chassis is generalised to a channel 30 mm deep and the rail and rest strip to 30 mm bars at their anchors, offset 0.5 mm off the chassis back face (the first render showed z-fighting marks there).
- 1:200 thermal: the expansion vessel is its real 0.32 m lens (revolve of two dished shells), the premix fan its drum.
- 1:500 authored anew (`r5-gas-boiler-model.py`): crisp hood block 0.44 x 0.72 x 0.325 m, control strip, solid 125 mm flue drum, connection bar; thermal with flush window disc, burner block, outlet patch on the flue drum, exchanger drum, fan drum, vessel lens, hydraulic block; structure chassis channel, bracket shelf, heat-exchanger drum, collector and adapter as one stepped tube, connection bar, rail, rest, mass. Three bilingual decisions.
- Root: `thermal.regions.connections.areaM2` 0.04 -> 0.035 (factor 1.14; measured 1:50 outer area of the five tubes and nuts 0.034 m2).
- Final: `r3-validate.ts appliances/gas-boiler --volumes` VALID (schema 0, validateAsset 0, checkVolumes 0); `volumes-check.ts --product` PASS.

### Remaining issues

- The porthole with flames is the asset's didactic feature (real condensing boilers have no window); kept because rig, poses and clips are built on it.
- Revolves are 12- to 16-gons at the product tessellation (knobs, bezel, collar); flat faces show the renderer's fan shading. Neither is in the geometry.
- Condensate drain and pressure-relief outlet still not modelled (the 2D draws five connections).
"""),
    "hot-water-cylinder": ("r3-appliances-hot-water-cylinder.md", """
## Round 5 handcraft
""" + COMMON + """
### Model against reference (Viessmann Vitocell 100-V 200 l class, EN 12897, DIN 28011)

| Quantity | Reference | Model 1:50 | Deviation |
|---|---|---|---|
| Jacket diameter | 581 to 637 mm (class) | 600 mm | inside the class |
| Height | 1349 to 1409 mm, class to 1530 mm | 1480 mm | inside the class |
| Over the connections | 605 mm (CVAA) to about 660 mm | 660 mm | inside the class |
| Pressure vessel | 480 mm OD, 3 mm wall, Kloepper heads (head depth 93 mm) | 480 mm, 3 mm, heads 107 / 93 mm below and above the shell | 0 % |
| Feet | 3 at 120 degrees, 50 mm pads | 3 on a 0.25 m circle, 50 mm pads | 0 % |
| Nipples | R 3/4 to R 1, union nuts | 24 mm tubes, 34 mm hex nuts (38 mm across corners) | 0 % |
| Thermometer bezel | about 100 mm | 100 mm, 23 mm proud | 0 % |
| 1:100 / 1:200 / 1:500 overall | 0.66 x 1.48 x 0.60 m | 0.66 x 1.48 x 0.61 / 0.60 / 0.60 m | within one feature |

### Grades (body / thermal / structure)

| Scale | Before | After |
|---|---|---|
| 1:50 | A- / A- / A- | A- / A- / A- (unchanged, re-read: levelling feet, plinth, quarters, domed cover, nipples with nuts, thermometer, inspection cover; steel vessel nested in each quarter; Kloepper vessel on three splayed legs) |
| 1:100 | A- / A- / A- | A- / A- / A- (unchanged) |
| 1:200 | A- / B (plume outlet a 32 mm terraced crown on the cover) / A- | A- / A- / A- |
| 1:500 | B (outlet and nipples 110 mm cubes) / C+ (plume outlet a 76 mm stack of terraces on the cover) / B+ (75 mm cube pads) | A- / A- / A- |
| Poses cold, half-charged, charged, clip charge-up t 2 s | A- | A- (needle turns, nothing else moves) |
| 2D 1:50 to 1:500 | A- | A- (unchanged) |

### Changes

- 1:200 and 1:500 thermal: the plume outlet is a 4 mm skin on the crown of the cover (D2, `texture: true` because it is a surface patch, not a mass) instead of 32 / 76 mm terraces.
- 1:500 body: hot-water outlet and side nipples as round 110 mm stubs on their real axes (were cubes); thermal references follow.
- 1:500 structure: the contact pads are 75 mm round pucks under the leg ends (were cubes).
- No root edits. Final: `r3-validate.ts --volumes` VALID, `volumes-check.ts --product` PASS.

### Remaining issues

- The four jacket quarters show seams and tone steps: the asset's didactic thermal division, also drawn in 2D.
- The top foam collar in the structure (the load path of the cover into the vessel) reads as a wide ring from outside; it is the real foam annulus between the top head and the cover.
- Coil, anode and sensor wells not modelled (no region declares them).
"""),
    "circulation-pump": ("r3-appliances-circulation-pump.md", """
## Round 5 handcraft
""" + COMMON + """
### Model against reference (Grundfos UPS 25-60 180, G 1 1/2 unions)

| Quantity | Reference | Model 1:50 | Deviation |
|---|---|---|---|
| Port to port | 180 mm | 180 mm (thread faces at x +-0.09) | 0 % |
| Overall length with nuts and tails | 220 mm (`dimensions`) | 220 mm | 0 % |
| Height (75 above, 51 below the shaft) | 126 mm | 126 mm | 0 % |
| Depth (32 behind, 102 in front of the pipe axis) | 134 mm | 134 mm | 0 % |
| Volute drum | about 96 mm | 96 mm | 0 % |
| Union nut | 52 mm across flats | 52 mm | 0 % |
| 1:100 / 1:200 / 1:500 overall | 0.22 x 0.126 x 0.134 m | identical at every scale | 0 % |

### Grades (body / thermal / structure)

| Scale | Before | After |
|---|---|---|
| 1:50 | A- / A- / A- | A- / A- / A- (unchanged, re-read: volute with necks and threads, hex unions, copper tails, square flanges with four screws, can with bezel and sight glass, terminal box with lid, gland and lamp; rotor in the open stator shell) |
| 1:100 | A- / A- / A- | A- / A- / A- |
| 1:200 | A- / A- / A- | A- / A- / A- |
| 1:500 | C+ (inherited digital model: octagonal volute and motor, hexagonal hollow pipe with a bore below the feature, near copy of 1:200) / B / B | A- / A- / A- |
| Poses off, low, high, clip run-high t 0.6 s | A- | A- (pointer turns in the glass) |
| 2D 1:50 / 1:100 / 1:200 | A- | A- |

### Changes

- 1:500 authored anew (`r5-circulation-pump-model.py`): volute with flange as one round 16-sided 96 mm drum, motor head as a 16-sided 84 mm drum, pipe run as one solid 12-sided 44 mm bar (its 26 mm bore is below the 75 mm feature, C30), terminal box block; structure with the motor as one solid drum carrying the rotor shaft. Prisms keep the triangle count below 1:200 (`volumes-monotonic`) while reading round. Three bilingual decisions.
- No root edits. Final: `r3-validate.ts --volumes` VALID, `volumes-check.ts --product` PASS.

### Remaining issues

- The sight glass with indicator is the asset's didactic simplification (a UPS has a name plate); kept, rig and clips depend on it.
- Volute spiral is an offset drum (casting simplification); 1:50 revolves are 15-gons at the product tessellation.
"""),
    "mvhr-unit": ("r3-appliances-mvhr-unit.md", """
## Round 5 handcraft
""" + COMMON + """
### Model against reference (Vent-Axia Kinetic 550 x 550 x 285; EnviroVent energiSava 260 560 x 635 x 260; DN100 / 125 spigots)

| Quantity | Reference | Model 1:50 | Deviation |
|---|---|---|---|
| Casing width x height | 550 to 560 x 550 to 635 mm | 600 x 600 mm (asset class estimate kept in round 3) | +7 % on the width of the references, inside the class height |
| Depth incl. wall rail | 260 to 285 mm + rail | 285 mm + 15 mm rail = 300 mm | inside the class |
| Overall height with spigots | 0.665 m (`dimensions`) | 0.662 m | -0.5 % |
| Spigots | DN100 (98 mm OD) to 125 mm | 98 mm, 0.7 mm wall, 114 mm seals | 0 % against DN100 |
| Supply impeller | about 190 mm (R3G190 class) | 190 mm shroud, 124 mm eye | 0 % |
| 1:100 / 1:200 / 1:500 overall | 0.60 x 0.665 x 0.30 m | 0.60 x 0.662 x 0.302 / 0.302 / 0.301 m | within one feature |

### Grades (body / thermal / structure)

| Scale | Before | After |
|---|---|---|
| 1:50 | A- / A- / A- | A- / A- / A- (unchanged, re-read: wrapper with bends, rounded cover, bezel and window, light, grip, spigots with seals, rail, hook and rest; steel and EPP shells, plate pack, extract scroll, filters; wrapper, tray, tubes, collars, impeller on its shaft) |
| 1:100 | A- / A- / A- | A- / A- / A- |
| 1:200 | A- / B+ (extract fan a 144 mm cube beside round parts) / A- | A- / A- / A- |
| 1:500 | B (inherited: one generic decision sentence, 2 mm window and 0.7 mm hollow spigots copied from 1:200) / B / B+ | A- / A- / A- |
| Poses off, normal, boost, clip boost-spin t 2 s | A- | A- |
| 2D 1:50 / 1:100 / 1:200 | A- | A- |

Catalog before round 5: INVALID (`PRINT range-end-gap` of the 1:200 spigots, 0.041 mm at 1:350).

### Changes

- Root: `representations.200.validRange.maxDenominator` 350 -> 210: the four 90 mm spigots stand 60 mm apart; with the pen the gap is 0.15 mm on paper only up to 1:214 (the spigots cannot be thinner without losing the DN100 reading).
- 1:200 thermal: the extract fan is its 144 mm scroll drum as at 1:100.
- 1:500 authored anew (`r5-mvhr-unit-model.py`): crisp casing block, flush window, impeller disc, the four spigots as solid 90 mm drums; thermal casing, window face, impeller, closed duct drums, plate pack, extract drum, filter slab, thin outlet patches on the spigot tops; structure wrapper ring, back plate, spigot tubes on collars, impeller shaft, hook, rest, mass. Three bilingual decisions.
- Final: `r3-validate.ts --volumes` VALID, `volumes-check.ts --product` PASS.

### Remaining issues

- The inspection window is the asset's didactic feature; kept.
- Casing 600 mm square is about 7 % wider than the cited 550 to 560 mm units (asset class estimate, not changed without a cited 600 mm reference).
- At 1:100 and 1:200 the thermal cut shows hatch noise on the side faces where the casing and cover references meet in one plane (render artefact of coplanar faces); the casing region is grey in merged-layer scales (tool request of curtain-wall-bay applies).
"""),
    "decentralised-hrv-fan": ("r3-appliances-decentralised-hrv-fan.md", """
## Round 5 handcraft
""" + COMMON + """
### Model against reference (Lunos e2 60, Ambientika 200, Blauberg Vento Expert A50)

| Quantity | Reference | Model 1:50 | Deviation |
|---|---|---|---|
| Sleeve | 200 mm pipe class (Ambientika ceramic 170 mm in 200 mm pipe) | 196 mm OD, 3 mm wall | inside the class |
| Ceramic core | 154 to 170 mm diameter, 150 to 243 mm long | 160 mm x 100 mm | inside the class (short end) |
| Inner cover base | about 260 mm square | 260 x 260 x 20 mm | 0 % |
| Wall depth of the sleeve | from 200 to 240 mm | 220 mm behind the tray | inside the class |
| Impeller | about 170 mm | 172 mm in a 176 mm throat | 0 % |
| 1:100 / 1:200 / 1:500 overall | 0.26 x 0.26 x 0.24 m | identical at every scale | 0 % |

### Grades (body / thermal / structure)

| Scale | Before | After |
|---|---|---|
| 1:50 | A- / A- / A- | A- / A- / A- (unchanged, re-read: tray with rim, bosses, screws and terminal box, fan housing with inlet radius, sleeve and collar, EPP jacket, ceramic with open channels, struts, twisted blades, domed can and cap; references in their regions; members, joints and the two corner screws) |
| 1:100 | A- / A- / A- | A- / A- / A- |
| 1:200 | A- / B (ceramic surface coincident with the tube bore: stripes in every cut) / A- | A- / A- / A- |
| 1:500 | C+ (inherited: tray a 4 mm sheet, blades 5 mm plates, same coincident core, one generic sentence) / B / B | A- / A- / A- |
| Poses off, extract, supply, clip reverse-spin t 1.3 s | A- | A- (blades stay rooted) |
| 2D 1:50 / 1:100 / 1:200 | A- | A- |

### Changes

- 1:200: ceramic disc and its two outlet patches seated 1 mm into the tube wall (radius 0.083 instead of 0.082, which lay exactly on the bore).
- 1:500 authored anew (`r5-decentralised-hrv-fan-model.py`): 20 mm tray with 20 mm corners and the 164 mm bore, one merged tube, seated ceramic disc, bridge bar, three broad 15 mm blades, 80 mm hub; thermal references and 5 mm outlet skins; structure references, mass, corner-screw pin and roller. Three bilingual decisions.
- No root edits. Final: `r3-validate.ts --volumes` VALID, `volumes-check.ts --product` PASS.

### Remaining issues

- Tray floor 7.5 mm at 1:50 instead of 2.5 mm (round-3 author request on C30 for a two-solid shell still open).
- Honeycomb channels are 18 mm generalisations of 3 mm cells; small revolves are visibly polygonal at the product tessellation.
- The structure is close to the body from outside because the casing parts are the members (stated in round 3).
"""),
}

LEDGER = {
    "gas-boiler": "appliances/gas-boiler | body/thermal/structure 1:50 A-/A-/A-, 1:100 A-, 1:200 B+/B/A- (square vessel slab), 1:500 C+/B/B (chamfered digital block, 9 mm hollow flue ring); catalog INVALID (1:200 print band, connections area 15 %) | all scales and lists A-; 1:200 band redrawn 30 mm (2D root), 1:200 structure 30 mm sections, round vessel, 1:500 authored anew; connections areaM2 0.035; within 1.5 % of Vaillant ecoTEC plus; VALID, check PASS | didactic porthole kept; tessellation facets and fan shading are renderer/kernel; condensate and relief outlets not drawn",
    "hot-water-cylinder": "appliances/hot-water-cylinder | 1:50/1:100 A-; 1:200 thermal B (32 mm terraced plume crown); 1:500 body B (cube nipples), thermal C+ (76 mm terraces), structure B+ (cube pads) | all scales and lists A-; plume outlet a 4 mm skin (D2), round 1:500 stubs and pads; model inside the Vitocell 100-V class, vessel per DIN 28011; VALID, check PASS | didactic quarter seams; foam collar reads wide in structure; coil/anode not modelled (no region)",
    "circulation-pump": "appliances/circulation-pump | 1:50-1:200 A-; 1:500 C+/B/B (octagon volute and motor, hollow hex pipe, near copy of 1:200) | all scales and lists A-; 1:500 authored anew (round 16/12-sided drums, solid pipe, motor drum carries rotor); Grundfos UPS 25-60 180 dimensions exact; VALID, check PASS | sight glass is the asset's didactic simplification; volute an offset drum",
    "mvhr-unit": "appliances/mvhr-unit | 1:50/1:100 A-; 1:200 thermal B+ (cube extract fan); 1:500 B/B/B+ (copied thin walls, one generic sentence); catalog INVALID (1:200 spigot gap at 1:350) | all scales and lists A-; 1:200 range to 1:210 (root), extract fan drum, 1:500 authored anew; VALID, check PASS | 600 mm casing 7 % wider than cited 550-560 mm units (class estimate kept); didactic window; coplanar hatch noise in coarse thermal cuts",
    "decentralised-hrv-fan": "appliances/decentralised-hrv-fan | 1:50/1:100 A-; 1:200 thermal B (core coincident with bore, stripes); 1:500 C+/B/B (4 mm sheet tray, 5 mm plate blades) | all scales and lists A-; 1:200 core seated, 1:500 authored anew (20 mm tray, 15 mm blades); inside the Lunos/Ambientika 200 mm class; VALID, check PASS | 1:50 tray floor 7.5 mm (C30 two-solid request open); 18 mm honeycomb generalisation",
}


def main():
    """✍️ Appends the sections and the ledger lines."""
    for slug, (name, text) in SECTIONS.items():
        path = os.path.join(HERE, name)
        have = open(path, encoding="utf-8").read()
        if "## Round 5 handcraft" in have:
            print("already", name)
            continue
        open(path, "a", encoding="utf-8").write(("" if have.endswith("\n") else "\n") + text)
        print("appended", name)
    ledger = os.path.join(HERE, "r5-ledger.md")
    have = open(ledger, encoding="utf-8").read()
    add = [line for slug, line in LEDGER.items() if not any(l.startswith(f"appliances/{slug} |") for l in have.splitlines())]
    if add:
        open(ledger, "a", encoding="utf-8").write(("" if have.endswith("\n") else "\n") + "\n".join(add) + "\n")
    print("ledger lines", len(add))


if __name__ == "__main__":
    main()
