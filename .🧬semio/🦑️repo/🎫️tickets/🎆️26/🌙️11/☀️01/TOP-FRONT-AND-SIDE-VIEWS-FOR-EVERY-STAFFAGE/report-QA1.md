# Report QA1: Visual QA of A1 Animals and V2 Vegetation (15 Assets)

## Rounds 1–6
- **Status:**
  - The check is suitable for all 15 assets.
  - The guard (including spread and twist) passes for 14.
  - herring-gull fails `spread-elevation-y-front-y` in touchdown, landing@1.92 s and flare-key at 1:200. The end-on wing bones do not lift in front, which TS is fixing.
- **Fixes:**
  - Cat 1:100: front and plan are no longer near copies of 1:50.
  - Gull 1:200: no longer a near copy of 1:100.
  - Hedgehog 1:100 height corrected.
  - Gull: plan tail and hand reach corrected, 1:50 feet are extruded, stale decisions text rewritten.
  - Horse: tail swish via `abductionDeg` in all 128 keys; ear twist in standing-rest; plan muzzle and body capsules corrected.
- **Not done:**
  - Hedgehog coat closure: decided to use near and far coat bones.
  - Sheets: only a sample was inspected.
  - The three-views vitest was not run.

## QA1 round 2 (visual QA and fixes)
Assets: dairy-cow, domestic-cat, goat, hedgehog, herring-gull, horse, labrador-dog, japanese-maple, lombardy-poplar, london-plane-pollarded, miscanthus-grass, multi-stem-birch, scots-pine, small-leaved-lime, weeping-willow.
Driver: `T/🧰️QA1/all.ts` (check, guard, `--views` sheets; `--render a:50/100,b` selects what to render), helper scripts `montage.py`, `shapes.ts`, `feat.ts`, `dbg.ts`, fix scripts `fix*.py`, `hh*.py`, `goat*.py`, `gull*.py`, `pen*.py`.

### Results
- Check and three-views guard (round 15, all 15 assets, all authored scales): no problems, 56 of 56 guard lines ok.
- Three-views vitest (ticket config, filtered to the 15 assets): 30 of 30 pass.
- Detail levels: shape and vertex counts fall with the scale in every view (no inverted `scale >= 100` condition). Two near copies found and redrawn: cat 1:100 vs 1:50 and gull 1:200 vs 1:100.

### Changes
- Hedgehog: spine skirt split into four bones coat-rn, coat-rf, coat-fn, coat-ff (`Bone.z` = -0.04/+0.04, mass halved); rolled (+-40) and rolling-half (+-20) close them by mirrored spread; elevation (near skirt only), front and plan redrawn at 1:50 and 1:100; front head is an extruded profile; 1:100 front neck dropped (always hidden, leftover sliver).
- Gull: wings are TS solids; far leg added to the 1:100 and 1:200 elevation (same polyline as the near leg); 1:200 head drawn 12 % slimmer so the default-pose height is inside the tolerance; plan tail shifted, plan hands tapered to the same lateral reach as the front; 1:50 front feet extruded.
- Goat: hindquarters, chest and neck are revolved solids about spine axes in front and plan (two zero-mass fixed axis bones axis-hind and axis-chest, the neck bone itself; revolved axisX of the neck is its absolute rest x 0.7397). A rearing goat is now one body with neck and legs.
- Horse: tail swish track (abduction 0.9 x angle on tail-1..4 in graze-and-swish), ear twist in standing-rest, plan trunk capsules no longer overshoot, tail one piece per bone at 1:200/1:500, plan muzzle/forelock/eyes removed at 1:50/1:100.
- Cow 1:500: udder kept in all three views (the thermal region must be drawn). Cat 1:50/1:100: tail-tip z raised so no measured gap sliver.
- Print: body pen 0.2 mm at cow 1:500, cat 1:200, goat 1:500 and 1:200, hedgehog 1:100, gull 1:200 and 1:100, horse 1:500 and 1:200, dog 1:500. The corrected feature-width measure sees limbs foreshortened to hairlines in one view of a pose; the stroke weight keeps them at the minimum feature. Per-shape `penMm: 0.2` on cat 1:200 tail pieces, dog 1:200/1:500 hind thighs can be reverted once FW skips fully hidden shapes.

### Sheets read
Cow 1:50 all pages, 1:100 and 1:200 page 2, 1:500 (card before the udder fix); goat 1:50 (twice, before and after the revolved solids), 1:100, 1:200, 1:500 first pages; horse 1:50 first page; dog 1:50 pages 1 and 2; maple 1:50 and 1:100 page 1; miscanthus 1:50, birch 1:50, willow 1:50, lime 1:100 page 1; earlier single sheets of pine, hedgehog, gull. Not read: the rest of horse, dog, cow, goat pages and of the vegetation montages (plane, poplar, pine, lime 1:50/200/500, willow 1:100-1:500, birch 1:100-1:500, maple 1:200/1:500).
Evidence kept in `T/🗑️generated/QA1/`: `mont/goat-50-1.png` (rearing goat), `hh-rolled.png`, `hh-rolling-half.png`, `herring-gull--touchdown--views-dark-line-1-50.png`, `horse--graze-and-swish@0.64--views-dark-line-1-100.png`, `vitest3.txt`, `round15.txt`.

### Doubts
- Vegetation sway is along the elevation x axis only, so front and plan crowns look nearly static across clip instants (they shift in x or height only).
- Miscanthus front plumes are thin hollow loops above the thick culms.
- Weeping willow front crown dome is sparse compared to the elevation.
- Dog and cat sitting trunks are still rigid ellipses in front (seated trunk reads connected but not revolved).
- 1:200 and 1:500 animal sheets are 2-5 mm tall; montages only show that nothing floats.
