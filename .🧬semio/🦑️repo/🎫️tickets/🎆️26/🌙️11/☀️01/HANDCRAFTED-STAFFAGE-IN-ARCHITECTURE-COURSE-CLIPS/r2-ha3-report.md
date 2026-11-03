# r2 ha3: Authoring audit of 18 handcrafted assets

Scope: machinery (expansion-valve, finned-heat-exchanger, oil-pumpjack, scroll-compressor), humans (person-cooking, seated-office-worker, sleeping-person, standing-adult, walking-commuter), objects (chalkboard, cooking-pot, storage-crate-stack, tealight), furniture (operating-table), vehicles (vintage-car-1929, city-car), components (panel-radiator, tilt-window). Rules: `ARCHITECTURAL-STAFFAGE-ARCHIVE/authoring-v2.md` and `content-worker-brief.md`.

Method: a mechanical audit script (`🧰️ha3/audit.py`) for structure, pens, validRange, region drawing per scale, pose bindings, clip keys, metabolic and skin fractions, respiration sources and references, and operating keys. Then a digest of dimensions, masses, thermal and physiology values per asset, and a read of the notes. No previews were rendered or viewed, so the visual items below are open.

## Fixed

| Asset | File | Change | Reason |
|---|---|---|---|
| standing-adult | `👤️humans/standing-adult/🔣️.json` | Height text 1.67 m to 1.69 m (summary, 1:500 silhouette decision, en and de) | `dimensions.heightM` and the thermal assumptions are 1.69 m; the drawn figure reproduces 1.69 m |
| walking-commuter | `👤️humans/walking-commuter/🔣️.json` | `activityMet` 2.8 to 2.6 in all six walking poses and in the `walk-cycle` track | Rule: walking 2.0 to 2.6 met |
| tilt-window | `🧱️components/tilt-window/🔣️.json` | Sash mass 72.8 kg to 33.0 kg (`dimensions.massKg` and the rig bone). Source changed from literature/EN 1991-1-1 to authored-estimate/EN ISO 10456. Notes and assumptions (en and de) rewritten | The mass counted the 24 mm drawn glass at glass density, which includes the 16 mm gas gap. Two 4 mm panes give 21.6 kg, timber 9 kg, hardware 2 kg |
| vintage-car-1929 | `🚜️vehicles/vintage-car-1929/🔣️.json` | 114 filled shapes given a `region` (all representations). Wheel shapes to `tyres`, radiator parts to `radiator`, headlamp bowls and plan lamps to `fenders`, lens to `glazing`, bumpers, frame, tail lamp, brackets and spare wheel to `body-panels` | Rule: every filled shape showing a surface carries a region. Audit: no body-role shape without region remains |
| city-car | `🚜️vehicles/city-car/🔣️.json` | 14 shapes plus 6 wheel-well entries given a region: wheel shapes to `tyres`, wells and mirrors to `body-panels`, headlamps to `glazing`, tail lamps to `tailgate`. Duplicate-key scan clean | Same rule |

Backing scripts: `🧰️ha3/bind_regions.py` (dry-run and apply), `🧰️ha3/dupkeys.py`, `🧰️ha3/audit.py`, `🧰️ha3/digest` helpers.

## Mechanical audit results (all 18)

- Thermal: `metabolicShare` and `skinFraction` sum to 1 for all humans. Every pose binds every region and every non-breath emitter. Every pose binds the `operating` key of its machine. Respiration patterns cite resolvable references. Body regions drawn at every authored scale.
- Pens: body is never thinner than detail or texture, and ground is never thinner than body. Pens grow or stay equal from 1:500 to 1:50.
- validRange: the "overlaps" flagged by the script are inclusive shared endpoints (75, 150, 350), which are acceptable. Each range contains its own denominator.
- Clips: no `inhale` or `exhale` events remain. Phases named inhale or exhale are kept (judgement).
- Titles, summaries, decisions and references are present in en and de. `completion` is authored throughout.

## Judgement items (not changed)

1. **Interior shapes with no region at 1:50**: vintage-car has 7 (seats, steering column, two handles, number plate, fender lamp). city-car has 9 (seat backs and headrests, door handles, rear bumper lamp, well liners on axles). These need an interior passive region, not a relabel.
2. **cooking-pot thermal budget is inconsistent**: the water region has `heatCapacityJM2K` 3000. Three kilograms over 0.034 m² (90 mm depth) needs about 3.8e5 J/(m²K), so it is about 125 times too small. Total operating heat is 126 W (base 60, wall 28, water 38), against about 1.0 MJ needed to heat 3 kg by 80 K in the 240 s clip (roughly 3 to 4 kW). The steam emitter at 0.004 m³/s and 0.3 kg/kg implies about 1.4 g/s vapour, about 3 kW latent. The notes admit the water figure is "illustrative". Fixing it needs one coordinated redesign of heat powers, clip timeline and steam flow, so I did not change it.
3. **panel-radiator massKg 255.68 kg**: the wall bone (229.5 kg masonry 1 m strip) is part of the asset bounding box, so dims and mass include it on purpose. The radiator alone is about 26.2 kg, which is realistic for a type 22, 1000 by 600 mm. The coordinator should choose between keeping the wall in the asset or moving it out.
4. **Breathing motion**: most humans use `angleDeg` 0.5 and `dy` 0.003 (sleeping 0.8 and 0.006). The rule example is 1 to 2 degrees and 3 to 6 mm at 0.5 L. This is the catalog-wide norm, so I left it.
5. **Sleeping metabolic rate**: `activityMet` 0.8 is documented in the notes. ISO 8996 and ASHRAE list sleeping at 0.7 met. Kept, flagged.
6. **1:500 part counts**: the rule asks for one composed silhouette of 1 to 4 closed paths. Elevation shape counts at 1:500 are oil-pumpjack 18, seated-office-worker 18, person-cooking 14, standing-adult 13, sleeping-person 12. Some of these are merged regions. Needs a visual check.
7. **Pen values** are off the ISO 128-20 steps (0.13, 0.18, 0.25, 0.35, 0.5, 0.7) in places: 0.10, 0.15, 0.20 and 0.22 (oil-pumpjack 50 body). Pens are not part of the printed-profile check, so kept.
8. **Dimension and text mismatches**: operating-table summary says 0.5 m wide, `depthM` is 0.6 (check whether castors are included). tilt-window `heightM` 1.25 against a 1.2 m sash (sill included?). tealight `heightM` 40.5 mm includes the flame (summary says 16 mm cup height).
9. **Machinery values**: oil-pumpjack `heightM` 5.5 m is low for a conventional unit (6 to 8 m). Its motor heat of 700 W is low for a 30 to 50 kW motor. expansion-valve mass 0.83 kg is high for a small TXV (0.3 to 0.6 kg typical).
10. **Hob (person-cooking)**: the hob is `passive`, but the pot is `operating` with key `hob`, so the heat source is the pot. Plausible, flagged.
11. **Physiology sanity**: metWM2 58.15 everywhere. DuBois areas match stated heights and masses (checked for four humans). Basal metabolism from Mifflin matches (checked). Region areas exceed skin area by 9 to 22 percent because of the clothing area factor, which is by design.

## Not done

- Visual check of previews (1:500 silhouettes, proportions, pen weights) for these 18 assets.
- Catalog check on the 13 assets I did not touch (the expansion-valve check passed earlier in this session).
- Interior regions for the vehicles (judgement item 1).
- Cooking-pot thermal redesign (item 2).

## Catalog check

Run after the edits, all with exit 0 and a check mark:

- standing-adult: check mark. 1:500 to 1:50 suitable under office-laser (min stroke 0.10 to 0.13 mm).
- walking-commuter: check mark. 1:500 to 1:50 suitable under office-laser.
- tilt-window: check mark. 1:500 unavailable (24 mm glass, 0.05 mm at 1:500, as already declared). 1:200 to 1:50 suitable.
- vintage-car-1929: check mark. 1:500 to 1:50 suitable.
- city-car: check mark. 1:500 to 1:50 suitable.

Not run: the 13 untouched assets (expansion-valve passed earlier in this session).
