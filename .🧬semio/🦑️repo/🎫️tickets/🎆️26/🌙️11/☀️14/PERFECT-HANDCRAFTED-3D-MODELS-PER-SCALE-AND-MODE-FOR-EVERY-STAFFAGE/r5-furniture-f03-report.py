"""📝 Round 5, F03 furniture: appends the `## Round 5 handcraft` section to the four round 3 reports (single-bed, three-seat-sofa, bookshelf, operating-table) and one line per asset to r5-ledger.md, each file re-read right before writing; a section that is already there is not written twice.

Usage: python3 r5-furniture-f03-report.py
"""
import os

HERE = os.path.dirname(os.path.abspath(__file__))

COMMON = """Sheets opened for grading (Read tool, 1000 to 1400 px copies of the 2400 px sheets under `🗑️generated/r5-{slug}/before|after/`): `r3-sheet` body, structure and thermal (`--cut`) at 1:50, 1:100, 1:200 and 1:500, every pose at 1:50, one clip frame, and `r3-2d` at every scale (the 2D drawings were not changed in round 5, so the before set is the after set)."""

SECTIONS = {
    "single-bed": """
## Round 5 handcraft

Scripts: `r5-single-bed-build.py` (round 3 kit, body and lists loaded, the mattress functions replaced by hand-stated round 5 forms; writes `🗑️generated/r5-single-bed/🔣️.json` and `volumes/furniture/single-bed.json`), `r5-furniture-f03-ship.sh` (build, print band, splice, validate), `r5-furniture-f03-report.py` (this section). {common}

### Reference (manufacturer data and standards)

| Quantity | Reference | Source |
| --- | --- | --- |
| Mattress | 900 x 2000 mm nominal; inner bed size 10 to 20 mm larger | EN 1334:1996 |
| Bed for 90 x 200 | 2090 x 980 mm overall, headboard 920 mm (IKEA TARVA); post beds 0.85 to 1.05 m at the head | IKEA TARVA 302.612.70 data sheet; manufacturer ranges |
| Slatted base 90 x 200, head adjustable | 28 slats 35 to 38 x 8 mm in rubber caps, frame 45 mm, about 80 mm over the camber | retail data sheets (r3 section 1) |
| Cold-foam mattress | 180 to 220 mm, top about 0.45 to 0.50 m above the floor | ISO 3386-1 asset reference, product practice |
| Lifting head section | hinge in the frame rails, 0 to 65 degrees | product practice |

### Model against reference (1:50)

| Quantity | Model | Reference | Deviation |
| --- | --- | --- | --- |
| Mattress | 0.900 x 2.000 x 0.200 m (head part 0.894 wide) | 0.90 x 2.00 x 0.18-0.22 | 0 % (head part -0.7 % width) |
| Clear inner size | 0.910 x 2.000 m | 0.910-0.920 x 2.010-2.020 | width 0 %, length -0.5 % |
| Overall | 2.120 x 1.010 x 0.950 m | 2.090 x 0.980 x 0.920 (TARVA) | +1.4 %, +3.1 % (60 mm posts stand 20 mm proud of the rails), +3.3 % (inside the 0.85-1.05 m post-bed range; asset dimension kept) |
| Mattress top | 0.485 m | 0.45-0.50 m | in range |
| Slats | 28 x 38 x 8 mm, crown 0.285 m, frame 45 mm | 28 x 35-38 x 8 mm, 45 mm | 0 % |
| Hinge | bolt at x 0.78 m, y 0.23 m, 0 to 65 degrees | in the frame rails, 0-65 degrees | as reference |

### Grades (before -> after)

| Scale | Body | Thermal | Structure |
| --- | --- | --- | --- |
| 1:50 | B+ -> A- | A- -> A- | A- -> A- |
| 1:100 | A- -> A- | A- -> A- | A- -> A- |
| 1:200 | A- -> A- | A- -> A- | A- -> A- |
| 1:500 | B+ -> A- | B+ -> B+ | B+ -> B+ |

Poses at 1:50: head-raised, reading-in-bed, half-raised-key and clip sit-up-to-read t 4.5 s B -> A-; asleep B+ -> A-; flat A- -> A-.

### Defects seen before, and what was changed

- Raised head section (head-raised, reading-in-bed, half-raised-key, clip): the head mattress part turned about the bolt into the middle part with coincident side faces, so its end showed as a crossing "X" of seams on the side and the piping crossed the fold. Now the head part is 3 mm narrower each side (half width 0.447 m; its piping at 0.444 m) and the end that turns in lies inside the middle part: the fold reads as the head part rising out of the middle part, no crossing seams.
- Loaded poses (asleep, reading-in-bed): the three rigid parts sink by different amounts (8 to 18 mm) and showed sharp ledges. Every joint end now closes into a quilted crease (1:50: half width -4 mm and half height -6 mm at the joint face, back to full section over 12 mm; 1:100: -6 / -8 mm over 15 mm), so a step is a rounded shoulder of a zoned mattress. Piping stops at the creases.
- 1:500: the three mattress blocks were two-station lofts with 20 to 30 mm soft edges (below the 75 mm feature) that shaded as long facets in the side views. They are crisp boxes now (the head block 3 mm narrower): one clean mattress mass on the tray.
- Root, print: the catalog asset was refused by the print check (`PRINT range-end-feature`, the 30 mm side rail of the 1:200 plan is 0.191 mm at 1:350). The 1:200 band now ends at 1:330 and the 1:500 band starts there (`representations.200/500.validRange`).

### Checks

`volumes-check.ts furniture/single-bed --product`: PASS (170.7 s); silhouette IoU elevation / front / plan 1:50 0.9996 / 1.000 / 0.998, 1:100 0.999 / 1.000 / 1.000, 1:200 1.000 / 1.000 / 0.997, 1:500 1.000 / 1.000 / 1.000; mattress outer area 4.61 m2 at 1:50 (declared 4.5). A first 1:500 variant with eight-station lofts failed `volumes-monotonic` (+404 triangles against 1:200) and was replaced by the boxes. After the splice `r3-validate.ts furniture/single-bed --catalog-only --volumes`: VALID, schema 0, validateAsset 0, checkVolumes 0.

### Remaining issues

- The mattress is still three rigid parts on three bones; per-solid `weights` blend a whole solid, so a true bend of one mattress needs per-vertex skinning (not in the schema). The creases are honest for a zoned mattress but have no counterpart in a one-piece cover.
- 1:500 thermal: the tray box makes the frame region larger than declared (only the total counts there); `r3-sheet --list thermal` paints the mattress grey at 1:100 and coarser (legend knows only the 1:50 layers): a sheet artefact, the data are right.
- 1:500 structure: slat plates and bearing strips at the 75 mm floor read as planks (B+).
""",
    "three-seat-sofa": """
## Round 5 handcraft

Scripts: `r5-three-seat-sofa-build.py` (round 3 scripts loaded, the 1:500 structure and its decision extended; writes `🗑️generated/r5-three-seat-sofa/🔣️.json` and `volumes/furniture/three-seat-sofa.json`), `r5-furniture-f03-ship.sh`, `r5-furniture-f03-report.py`. {common}

### Reference (manufacturer data)

| Quantity | Reference | Source |
| --- | --- | --- |
| Overall width / height / depth | 1.80-2.18 m / 0.81-0.91 m / 0.85-0.95 m | Wayfair sofa-dimension guide; Lowe's listing (34 in depth) |
| Seat height | 0.43-0.48 m | Livingetc (designer data) |
| Seat depth | 0.51-0.64 m | Wayfair, Livingetc |
| Arm height | 0.60-0.65 m for low track arms (market 0.64-0.78) | Livingetc, Lowe's / Lulu and Georgia listings |
| Feet | turned hardwood 0.08-0.15 m | product practice |
| Mass | 45-70 kg | product practice |

### Model against reference (1:50)

| Quantity | Model | Reference | Deviation |
| --- | --- | --- | --- |
| Width / height / depth | 2.00 / 0.85 / 0.90 m | 1.80-2.18 / 0.81-0.91 / 0.85-0.95 | in range |
| Seat height | 0.43 m edge, 0.44 m crown | 0.43-0.48 | in range |
| Seat depth | 0.57 m | 0.51-0.64 | in range |
| Arm height / width | 0.62 m / 0.20 m | 0.60-0.65 / 0.15-0.25 | in range |
| Feet | 0.10 m, 45 to 60 mm | 0.08-0.15 | in range |
| Mass | 51.8 kg | 45-70 | in range |

### Grades (before -> after)

| Scale | Body | Thermal | Structure |
| --- | --- | --- | --- |
| 1:50 | A- -> A- | A- -> A- | A- -> A- |
| 1:100 | A- -> A- | A- -> A- | A- -> A- |
| 1:200 | B+ -> B+ | A- -> A- | A- -> A- |
| 1:500 | A- -> A- | B+ -> B+ | B -> A- |

Poses at 1:50 (one-seated, two-seated, lounging, seat-proof-load, impact-key, recovering-key) and clip lie-down-on-sofa t 2.5 s: A- (closed; sunk cushions pass behind the plinth face without visible interpenetration).

### Defects seen before, and what was changed

- 1:500 structure: the seat bearing bands stood on nothing between the two base rails (no deck), so the load path of the cushions was broken. A 75 mm ply deck member now spans rail to rail under the seat bands (0.155 to 0.23 m, between the arms); the 1:500 structure decision says so (en, de).

### Checks

`volumes-check.ts furniture/three-seat-sofa --product`: PASS (66.5 s); silhouette IoU 1:50 0.9995 / 1.000 / 0.9996, 1:500 1.000 / 1.000 / 1.000. After the splice `r3-validate.ts <asset> --catalog-only --volumes`: VALID, schema 0, validateAsset 0, checkVolumes 0.

### Remaining issues

- 1:200 body: arm rolls, cushion tops and the back-frame top are one chamfer step that shades faceted (B+); a two-step round would add triangles against the 1:100 list, which `volumes-monotonic` refuses at equal solid counts.
- Rigid cushions sink as blocks; foam does not deform (rig limit). Arm front without welt, back cushions without piping.
- `r3-sheet --list thermal` at 1:100 to 1:500 paints regions grey and stripes coincident region solids (sheet legend and z order); the data are right.
""",
    "bookshelf": """
## Round 5 handcraft

Scripts: `r5-bookshelf-build.py` (round 3 model loaded, the support pads restated; writes `🗑️generated/r5-bookshelf/🔣️.json` and `volumes/furniture/bookshelf.json`), `r5-furniture-f03-ship.sh`, `r5-furniture-f03-report.py`. {common}

### Reference (manufacturer data and standards)

| Quantity | Reference | Source |
| --- | --- | --- |
| Open bookcase type | 800 x 280 x 2020 / 1060 mm, shelf 760 x 260 mm (IKEA BILLY) | IKEA BILLY data |
| Board | 18 mm melamine chipboard P5, 650 kg/m3 | EN 312 |
| Shelf supports | 5 mm pins in 32 mm rows, first row 37 mm behind the front edge | System 32 (asset reference) |
| Cam fittings | 15 mm drum, 34 mm drilling distance | Häfele Minifix 15 |
| Back | 3 mm hardboard in 4 x 8 mm grooves | carcass practice |

### Model against reference (1:50)

| Quantity | Model | Reference | Deviation |
| --- | --- | --- | --- |
| Overall | 0.80 x 0.30 x 1.80 m | asset dimensions (BILLY 0.80 x 0.28 x 2.02 as type) | width 0 %; depth and height are the asset's declared unit |
| Board | 18 mm | 18 mm | 0 % |
| Shelf | 0.762 x 0.280 x 0.018 m, 1 mm play | inner width 0.764 - 2 mm | 0 % |
| Shelf pitch | 288 mm = 9 x 32 | multiple of 32 mm | 0 % |
| Pins | 5 mm, rows 37 mm and 261 mm behind the front | System 32 | 0 % |

### Grades (before -> after)

| Scale | Body | Thermal | Structure |
| --- | --- | --- | --- |
| 1:50 | A- -> A- | A- -> A- | A- -> A- |
| 1:100 | A- -> A- | A- -> A- | A- -> A- |
| 1:200 | A- -> A- | A- -> A- | B+ -> A- |
| 1:500 | B+ -> B+ | B+ -> B+ | B -> B+ |

Poses at 1:50 (loaded, shelf-adjusted, top-two-loaded-key) and clip re-pin-shelf t 2 s: A- (shelves sag closed about their pins, the re-pinned shelf moves with its pins).

### Defects seen before, and what was changed

- Supports: each floor-edge support was the 18 x 300 mm edge grown by 6 / 12 / 20 / 30 mm all round, so at 1:200 and 1:500 green tabs stood 20 to 30 mm outside the carcass in plan and elevation. The pads keep their width (18 mm plus twice the growth) but lie from the outer face of the side panel inward over exactly the 300 mm depth, under the carcass at every scale.

### Checks

`volumes-check.ts furniture/bookshelf --product`: PASS (20.2 s); silhouette IoU 1:50 1 / 1 / 1, 1:500 1 / 0.988 / 1; carcass outer area 3.36 m2 at 1:50 (declared 3.26). After the splice `r3-validate.ts <asset> --catalog-only --volumes`: VALID, schema 0, validateAsset 0, checkVolumes 0.

### Remaining issues

- 1:500 (and 1:200 less so): 75 / 30 mm boards are a heavy caricature of an 18 mm carcass, forced by the printable feature (0.15 mm on paper); the 75 mm cleats under the shelf ends read as brackets in the structure. B+.
- 1:50 structure: the carcass boards are the members themselves (allowed by D3 for carcass furniture), so it looks like the body in blue.
""",
    "operating-table": """
## Round 5 handcraft

Scripts: `r5-operating-table-build.py` (round 3 model loaded, the mass solids restated; writes `🗑️generated/r5-operating-table/🔣️.json` and `volumes/furniture/operating-table.json`), `r5-furniture-f03-ship.sh`, `r5-furniture-f03-report.py`. {common}

### Reference (manufacturer data and standards)

| Quantity | Reference | Source |
| --- | --- | --- |
| Table top | 500-540 mm wide, 560-590 mm over the rails, 2030-2130 mm long | ALVO Etiuda, Schmitz OPX mobilis, IEC 60601-2-46 |
| Height range | 690-1050 mm | Schmitz catalogue 60, medifa 6000 |
| Base | 940 x 540 mm (medifa 6000), 1120 x 580 mm (Schmitz) | brochures |
| Castors | twin 100-125 mm | product data |
| Mass | about 200 kg | medifa 6000 |
| Light stand | stand 1.88 m, arm 0.805 m, head 0.9-2.08 m | Dr. Mach LED 150 FP |
| Light head | 440-520 mm, up to 150 mm thick | Constella C, CL-2MB, FY-500BS |

### Model against reference (1:50)

| Quantity | Model | Reference | Deviation |
| --- | --- | --- | --- |
| Top | 2.00 x 0.50 m, 0.56 m over the rails | 2.03-2.13 x 0.50-0.54, 0.56-0.59 | length -1.5 % to the shortest, width and rails in range |
| Top height | 0.90 m | 0.69-1.05 | in range |
| Base | 0.94 x 0.54 m | 0.94 x 0.54 (medifa) | 0 % |
| Castors | 100 mm twin | 100-125 | in range |
| Table mass | 210 kg | about 200 | +5 % (asset value, inside the class) |
| Stand / arm | 1.96 m / 0.80 m | 1.88 / 0.805 | +4.3 % (asset height kept) / -0.6 % |
| Light head | 0.50 x 0.105 m | 0.44-0.52 x <= 0.15 | in range |

### Grades (before -> after)

| Scale | Body | Thermal | Structure |
| --- | --- | --- | --- |
| 1:50 | A- -> A- | A- -> A- | B+ -> A- |
| 1:100 | A- -> A- | A- -> A- | B+ -> A- |
| 1:200 | A- -> A- | A- -> A- | B+ -> A- |
| 1:500 | B+ -> B+ | B+ -> B+ | B+ -> B+ |

Poses at 1:50 (light-on, light-off, head-down) and clip tilt-table t 10 s: A- (the tilt clears the column and the stand).

### Defects seen before, and what was changed

- Structure, all scales: every bone declares a point mass, but the masses of chassis, table top, stand base and lamp head were drawn as flat discs up to 0.60 x 0.30 m that read as plates in the load path (a magenta slab across the table top). Each mass is now a compact spheroid at the same centre: half width 0.011 m x kg^(1/3) (base 120 kg 54 mm, column 42 kg 38 mm, top 48 kg 40 mm, stand base 18 kg 29 mm, lamp head 9 kg 23 mm), never wider than before, the vertical half axis kept at the value that fits in its part (the half feature at 1:200 and 1:500).

### Checks

`volumes-check.ts furniture/operating-table --product`: PASS (123.7 s); silhouette IoU 1:50 0.998 / 0.997 / 0.994, 1:500 0.999 / 0.995 / 1.000. After the splice `r3-validate.ts <asset> --catalog-only --volumes`: VALID, schema 0, validateAsset 0, checkVolumes 0.

### Remaining issues

- The telescopic column reads as three stepped shrouds (real telescopic covers step in, but less); the lamp head stands over the leg plates because a 0.8 m arm from a stand beyond the foot end reaches no further (asset layout kept).
- 1:500: slabs for base and stand base, the stand tube at the slender floor; thermal coincident region solids stripe in `r3-sheet` (sheet artefact). B+.
- Lamp internals (LED boards, driver), actuator, hand control and cables are not modelled (no regions declared for them).
""",
}

LEDGER = {
    "single-bed": "furniture/single-bed | body 1:50 B+ (fold crossing seams in raised poses, sharp ledges between rigid mattress parts in loaded poses), 1:100/1:200 A-, 1:500 B+ (faceted soft blocks); thermal A-/B+; structure A-/B+; catalog INVALID (PRINT 1:200 rail at 1:350) | body A- at every scale, raised/loaded poses A- (head part 3 mm narrower turns inside the middle part, quilted creases at the joints), 1:500 crisp blocks; 1:200/1:500 band boundary 1:330; check PASS, VALID | mattress still three rigid parts (no per-vertex skinning); 1:500 structure planks and thermal tray B+",
    "three-seat-sofa": "furniture/three-seat-sofa | body A-/A-/B+/A-; structure 1:500 B (seat bands floating, no deck); thermal A-/B+ | 1:500 structure A- (75 mm deck rail to rail under the seat bands); body unchanged A-/A-/B+/A-; check PASS, VALID | 1:200 chamfer-faceted cushions (monotonic triangle limit); rigid cushions; thermal sheet stripes at 1:500 (tool)",
    "bookshelf": "furniture/bookshelf | body A-/A-/A-/B+; structure 1:200 B+ / 1:500 B (support pads 20-30 mm outside the carcass) | supports laid under the side panels at every scale: structure A-/A-/A-/B+; body unchanged; check PASS, VALID | 1:500 75 mm boards and cleats (print floor) B+",
    "operating-table": "furniture/operating-table | body A-/A-/A-/B+; structure B+ at every scale (point masses drawn as 0.6 x 0.3 m discs) | structure A-/A-/A-/B+ with compact mass spheroids (0.011 m kg^1/3); body unchanged; check PASS, VALID | stepped column shrouds; lamp over the leg plates (asset layout); 1:500 slabs B+",
}


def main():
    for slug, text in SECTIONS.items():
        path = os.path.join(HERE, f"r3-furniture-{slug}.md")
        current = open(path, encoding="utf-8").read()
        if "## Round 5 handcraft" in current:
            continue
        open(path, "a", encoding="utf-8").write(text.format(common=COMMON.format(slug=slug)))
    path = os.path.join(HERE, "r5-ledger.md")
    current = open(path, encoding="utf-8").read()
    lines = [line for slug, line in LEDGER.items() if not any(l.startswith(f"furniture/{slug} |") for l in current.splitlines())]
    if lines:
        open(path, "a", encoding="utf-8").write(("" if current.endswith("\n") else "\n") + "\n".join(lines) + "\n")
    print("written", len(SECTIONS), "sections,", len(lines), "ledger lines")


if __name__ == "__main__":
    main()
