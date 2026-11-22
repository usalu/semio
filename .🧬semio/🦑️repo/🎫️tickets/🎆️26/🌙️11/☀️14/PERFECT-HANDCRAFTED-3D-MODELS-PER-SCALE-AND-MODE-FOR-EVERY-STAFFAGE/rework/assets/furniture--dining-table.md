# `furniture/dining-table` — running report

## Pass 1 (second round), 2026-11-22

State found: source `volumes/furniture/dining-table.json` and catalog `volumes` were identical; batch report `rework/14.md` section 6 had judged the asset "no defect found, not changed" and named two imperfections (30 mm daylight under the top, 21 mm centre bar).

### Reference table (pull-apart extension table, solid oak, DIN 68885 / EN 12521 class of furniture)

| Feature | Real subject | Model before | Model now |
| --- | --- | --- | --- |
| Height / width | 740 – 760 / 900 – 1000 mm | 750 / 900 | unchanged |
| Length closed / extended, leaf | 1600 / 2200, leaf 500 – 600 | 1600 / 2200, 600 | unchanged |
| Top thickness | 25 – 40 mm solid, often under-bevelled | 30 with 15 mm bevel | unchanged |
| Legs | 60 – 90 mm square, tapered to 40 – 50 mm | 70 to 45 mm, 720 long | unchanged |
| Aprons | 20 – 30 x 80 – 110 mm, knee clearance at least 600 mm | 25 x 80, clear 610 | unchanged |
| Overhang of the top | 30 – 60 mm at the sides, 80 – 150 mm at the ends | 40 / 100 | unchanged |
| Extension slide (each half stands on two legs only, the slide carries its tipping moment) | telescopic steel or hardwood slide 45 – 70 mm deep per section | flat 21 x 8 mm in a 25 x 12 mm tube: second moment about 6 200 mm4 per side, a wire | flat 63.5 x 8 mm on edge in a 67.5 x 12 mm slotted C-rail: about 171 000 mm4 per side (x 27) |
| Mass | 60 – 80 kg | 64.65 kg | 73.24 kg (frame 12.757, halves 24.312 each, leaf 11.859) |

### Defects found (read from the sheets at full size)

1. 1:50 body, structure, thermal: the slide was far too shallow for its job (see table); in `extended` the 0.6 m between the apron ends was bridged by a 21 mm strip.
2. 1:100 / 1:200 / 1:500: the slide was 20 / 30 / 21 mm deep (same defect); the lift mechanism was the 1:50 list unchanged (crossheads with seven lightening slots at 1:200, two bores at 1:500, 50 mm lifting nuts at 1:500).
3. Texts (notes, drawing decisions, volume decisions) described the 21 x 8 mm bar.

### Rebuilt (scripts `rework-dining-table-slide.py`, `rework-dining-table-coarse.py`, `rework-dining-table-texts.py`)

- 1:50 body: `centre-bar` 1000 x 63.5 x 8 mm (world height 0.6245 – 0.688), `runner-*` as slotted C-rails 67.5 x 12 mm of 2 mm sheet around it, slot kept where the lift brackets pass. Structure joint `shoe-*` at the full bar depth. Thermal follows by `of`.
- 1:100: `slide` 18 x 65 mm, links alike; 1:200: 30 x 70 mm, crossheads without slots; 1:500: 18 x 66 mm (no `slender` needed any more), crossheads plain, lifting nuts removed from all three lists.
- Rig masses recomputed from the changed steel volumes (frame 12.75746 kg comY -0.02351; halves 24.312 kg, comX 0.36532 / 0.43468, comY -0.06976), `dimensions.massKg` 73.23617, mass markers of the structure lists moved to the new centres.
- Thermal `slide-rails.areaM2` 0.405 -> 0.652 (measured outer surface of the rebuilt region).
- 2D: elevation shape `rail` / `slide-bar` redrawn to the new depth at the four scales (visible between the apron ends in the extended poses).
- Decisions (en, de) at four scales, asset notes and the 1:50 drawing decision rewritten for the new slide.
- Rig topology unchanged and judged right: fixed slide frame, two prismatic halves (0 – 0.30 m), prismatic leaf (0 – 0.03 m). No lateral pivot exists in this subject.

### Rendered and read after the last change

1:50 body `closed` (before the change), `extended` (three-quarter from above, underside turn 130 tilt -20, right side), structure `extended` right side, clip `extend-table` t = 3 s, 2D sheet `extended`; 1:100 thermal `extended` cut; 1:500 body `extended`. Rendered without error but NOT read: 1:50 `set-for-dinner`, `edge-test-load`, `pulled-apart-key`, clips `extend-table` 1.2 s, `retract-table` 1 s / 3 s, `lay-the-table` 2 s / 5 s, 1:50 thermal `extended`, 1:100 and 1:200 body `extended`, 1:200 structure `extended`, the closed sheets of 1:100 / 1:200 (all lists) after the change.

### Validation

`VALID furniture/dining-table (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (`r3-validate.ts furniture/dining-table --catalog-only --volumes`), published with `volumes-insert.py`.

### Remains imperfect (plainly)

- The concept "top on 30 mm blocks above the aprons, 0.9 m leaf stored flat in the gap" is kept; it shows 30 mm daylight between apron and top from the side. A butterfly leaf folding between the slides would be the common construction; that is a rebuild of leaf bone, poses, clips and all drawings.
- 1:50 body has no corner braces at the leg–apron joints and no table locks under the joint; legs meet the top only by contact (no fixing shown).
- 2D elevation in the extended poses does not draw the four screw posts and nuts that the 3D model shows between slide and leaf; front and plan were not rechecked with `r3-project2d`.
- Pose `thermal.regions.slide-rails.exposedFraction` (0.73 closed) was not re-derived for the deeper rail.
- 1:100 lift mechanism still has the 1:50 crosshead slots and nut profile nearly unchanged.
- Symbol geometry (`symbols[0]`, bone `ground`, line `rail` at 0.6775) not redrawn for the deeper slide.
