## Reviewer 3 (humans, animals, vegetation, appliances, furniture; ruling C29, C27)

Scripts: `areareview3-measure.py` (product measurement, `volumes-check.ts --scale 50 --rules volumes-thermal-coverage --json`, 94 assets, output `🗑️generated/areareview3/measure.json`), `areareview3-table.py`, `areareview3-dump.py` / `areareview3-brief.sh` (closed analytic surface per solid, reads the 1:50 volumes from `volumes/<category>/<slug>.json` when the asset file has none), `areareview3-decisions.py` (one reason per row), `areareview3-apply.py` (surgical: number token of `areaM2` and `skinFraction`, JSON-compared, atomic, re-read before writing), `areareview3-c27.py` / `areareview3-c27apply.py` (C27), `areareview3-report.py` (this section and the findings). The rows of reviewer 1 for these five categories stay as written above; where this section decides the same row, this section is the later decision.

### Method and rule

Rule of reviewer 2 (R-A passive, R-B operating, R-C conditioned, enclosed casings and inner faces of cavities do not count, both faces of a free sheet count, covered faces count) with one extension for stacked slices: the faces between slices of one construction (the four zones of a hot-water cylinder, the plank and its joint) are interfaces of that construction, not outer faces. Every decision is a hand calculation of the real object from `dimensions`, the drawing and standard sections, compared with the measured outer area of the 1:50 model (validator `info:outerAreaM2`, which for plain solids agrees with the closed analytic surface of the solids within 3 %). Where model and hand figure differ by more than 15 %, the hand figure is kept where it is the honest object, and the model is named in `model-findings.md`; where the two bracket a value (hand range), the value inside both is set. `heatW` of operating regions is the datum and stays; the flux density follows the honest outer area.

Measurement over all 94 assets of the five categories: 620 regions, 360 outside 0.75-1.3 of the declared value, of which about 300 are exchange regions (foliage, fur, feather, hair: not compared, C19), body-model overlays (below) or regions already decided.

### Totals

{TOTALS}

### Decisions (committed = HEAD; value = in the file after this review)

{DECISIONS}

### Re-check of reviewer 1's accepted rows (C29 against measurement)

Of the 95 ACCEPT rows of reviewer 1 in these categories, 44 were re-calculated by hand against the measured outer area (all rows whose measured value is outside 0.75-1.3 of the accepted value, plus the named ones). Corrected: stroller-frame stands, but crutch-cuffs (one face counted, 0.06 -> 0.10), crutch-tips (0.009 -> 0.014), rollator tyres (0.116 -> 0.21: the sidewall was a narrow ring), sunglasses (outer lens faces only, 0.0046 -> 0.0098), castor-rubber of the office worker (five wheels at 0.007 are 0.035, not 0.025), chair-steel and laptop of the office worker (0.127 -> 0.2, 0.233 -> 0.39), kitchen-extractor-hood filter (reviewer 1's own hand figure of one face 0.45 was kept at 0.2; both faces count: 0.84) and chimney (0.55 -> 0.78), circulation-pump casing and motor, electric-fan-heater casing and control, gas-meter index and pipes, hot-water-cylinder connections, mvhr-unit casing (0.9 -> 1.16: box 0.6 x 0.665 x 0.3) and ducts, toaster control, vacuum-cleaner wand (the retracted tube), desktop-monitor stand, bookshelf plinth stands (model defect), dining-table apron, operating-table column and light housing (0.35 -> 0.43), single-bed mattress (2.5 -> 4.5: the bottom on the slats counts), slats (1.5 -> 3.0) and bed-frame (3.37 -> 4.7), picnic-table a-frame (0.998 -> 1.2), park-bench back-timber (1.7 -> 1.3: three slats of the drawn 0.08 x 0.05 section). 32 rows were wrong by more than 15 % of the real object and are changed; 12 stand (accepted again with the measured figure as support). Slips of the kind reviewer 2 found: the extractor filter (own hand figure 0.45 against decided 0.2), the office-worker castors (arithmetic), the mattress and slats (one face), the hot-water-cylinder zones were right but their measure counts interfaces.

### C27 (segment shares of living subjects), measured

Solver reading (`🫀️physiology bodySetup`, `🛋️comfort`): the physiological skin area of a segment is `skinFraction / sum(skinFraction) x skinAreaM2`; `f = areaM2 / that` is the clothing-area factor (bare skin and fur regions sit at f = 1.0, garments at 1.1-1.5, shoes up to 1.5); `metabolicShare` is independent of the area. A redistribution therefore keeps `f >= 1` for garments and, for a part whose `skinFraction / areaM2` equals that of the donor (horse: both 0.1719), leaves every `f` unchanged when `skinFraction` moves with the area; `metabolicShare` never moves (heat production is physiology, not geometry).

Procedure (`areareview3-c27apply.py`): redistribute among the clothing regions of a human only (bare skin keeps f = 1, a hat keeps its one-sided value, hair and fur are exchange) when (1) every clothing region is measured, (2) the sum of the declared values over the sum of the measured ones lies in 0.80-1.20, (3) no region measures more than 3x or less than 0.4x its declared value (otherwise the model is buried, overlapping or crude), (4) a region deviates beyond 30 % after the rescale, (5) every f stays within 1.0-1.6. New value = measured x (sum declared / sum measured), clamped at f = 1.0, last digit absorbed so the sum is exact. Result:

{BODY}

Totals before and after: standing-adult 1.5740 -> 1.5740, seated-guitarist 1.9546 -> 1.9546, kneeling-gardener 1.7067 -> 1.7067 (clothing sets); `skinFraction` and `metabolicShare` unchanged; f after: standing-adult hoodie 1.00, jeans 1.22, sneakers 1.42; seated-guitarist shirt-torso 1.00, sleeves 1.21, jeans 1.33, boots 1.55; kneeling-gardener shirt-torso 1.00, trousers 1.32, clogs 1.39 (the hat, f 3.3, is not touched).

Horse (animal, the one sound case): the four hooves are drawn 0.049 m2 each (loft, 4 x 0.0489 analytic, 0.211 measured); the real hoof (o0.13, wall 0.09 high: 0.41 x 0.09 = 0.037 + sole 0.010) is 0.047, so the model is true and the declared 0.12 (0.03 per hoof) was low. Hooves 0.12 -> 0.195, coat-trunk 4.05 -> 3.975 (fur, exchange: shares move, the coat is 98 % of the subject), total of the body regions 5.818 -> 5.818; `skinFraction` hooves 0.02063 -> 0.03353 and coat-trunk 0.69611 -> 0.68321 (equal `skinFraction/areaM2` of 0.1719, so every f stays 0.96; sum 1.0); `metabolicShare` unchanged (hooves 0.001, trunk 0.7235).

No redistribution, with the defect named (measured 1:50 outer area against the declared value; none of these models is sound for the part):

| subject | segments | declared / measured | verdict |
|---|---|---|---|
| dairy-cow | lower-legs, hooves, udder | 0.30 / 0.427, 0.05 / 0.306 (6.1x; real hoof 4 x 0.044 = 0.18), 0.45 / 0.355 | hooves drawn 1.7x the real part, udder undersized; the rescale would give the hooves 27 % of the skin parts (real 14 %) |
| goat | horns, hooves | 0.012 / 0.058 (4.9x), 0.012 / 0.081 (6.7x) | drawn 2-4x the real horn and hoof |
| sheep | ears, hooves | 0.022 / 0.045 (2.0x), 0.016 / 0.125 (7.8x) | hoof drawn 6x (real 0.02), ear 2x |
| pig | skin-legs, skin-head, hooves | 0.2 / 0.66 (3.3x), 0.1 / 0.24 (2.4x), 0.018 / 0.134 (7.5x) | legs and hooves oversized (real legs 0.26) |
| roe-deer | hooves, nose | 0.013 / 0.071 (5.5x), 0.004 / 0.0016 (0.41x) | hooves drawn 4x, nose undersized |
| pigeon | legs, beak, cere, eye | 0.002 / 0.0137, 0.0003 / 0.0024, 0.0001 / 0.0012, 0.00015 / 0.0006 (3.8-12x) | parts at the 7.5 mm floor, real 3-4 mm |
| herring-gull, mallard-duck | bill, legs, web-feet | 3.3-5.4x | toes and bill drawn at the feature floor and thicker than the real part |
| red-squirrel | hands, feet | 0.0008 / 0.0044, 0.0016 / 0.0092 (5.5-5.7x) | feature floor |
| domestic-cat, hedgehog | nose, eyes, pads, paws | 1.6-2.3x | feature floor, all below 0.4 % of the subject |
| labrador-dog | nose, tongue, pads | 0.92x-1.21x | none beyond 30 %: nothing to do |
| humans, bare skin | face, head, arms, legs | 1.2-2.2x | the face solid covers the scalp under the hair (hair is not measured), the skin solid runs under the garments and counts whole (C29): the region partition of the model is not that of the declared values; f = 1 must stay |
| humans, clothing | child-on-kick-scooter, jogger, person-cooking, person-with-umbrella, pregnant-person, worker-carrying-box | sum declared / measured 0.67-0.79 | overlaid or buried solids (factor below 0.8) |
| humans, clothing | courier, cyclist, parent-carrying-toddler (toddler suit 0.58 / 0.37, mittens 2.1x), parent-with-stroller, person-on-crutches (shorts 0.197 / 0.567, 2.9x: drawn loose), person-with-rollator, tour-guide, walking-commuter, standing-phone-user | f would reach 1.6-2.9 | a helmet, cap, glove or shoe drawn 1.9-2.6x; the rescale would push a garment's f beyond the physical range |
| humans, clothing | elderly-with-cane, leaning-person, runner-with-blade-prosthesis, running-child, seated-office-worker, seated-reader, skateboarder, sleeping-person, wheelchair-user (hoodie -25 %) | factor 0.89-1.03 | no clothing segment beyond 30 % after the rescale: nothing to do |
| kneeling-gardener hat | 0.226 / 0.398 | kept | one-sided sky surface 0.213 (hand); the real hat has 0.38 with the brim underside, which the pose and the head carry; C18 stands, the validator number is a finding |

### Note for the audit and the revert scripts

The audit's Table A (forbidden changes) lists horse/coat-trunk 4.05 -> 3.975: that is the C27 redistribution above (hooves 0.12 -> 0.195 with the total fixed), not an author correction; `area-revert.py` and `area-revert-named.py` must not restore it.

### Pose fractions and open points

Pose fractions were not edited in this pass (the totals above are the real outer areas; `exposedFraction` and `contact.fraction` of the default poses that were set against exposed-only values are listed in the `info:exposureDeviation` of the validator). The dining-table `leaf-top` region (stored leaf, pose exposure 0 against the measured 0.47) and the single-bed `slats` (target 0) carry the stowing in the pose, as intended.

### Findings for the model authors

See `model-findings.md`, section "Area review 3 (reviewer 3)".

### Tests and fixtures affected by the confirmed changes (nothing regenerated)

Found by searching `S/🧪️tests` and `S/🧫️fixtures` for the 41 slugs with a changed `areaM2` (or `skinFraction`) of this section (search result `🗑️generated/areareview3/`).

**Tests that solve thermal results or hydronic/phase coupling of changed assets (re-run; the checks are inequalities, expected to hold, the coupled draws move):**
- `🚰️hydronic-coupling` and `🫕️phase-change` (electric-kettle: shell 0.095 -> 0.15 and water 0.0145 -> 0.0155, both now enter the coupled draw `areaM2_dep x inflow / areaM2_source` and the converted mass `kg = joules x area / latent`; any pinned kg or draw changes by about 0.0155/0.0145 and by the shell ratio).
- `🧊️thermal-scenarios` (english-oak trunk-bark 16 -> 25: area weights of the Watt totals only, temperature unchanged), `💨️transport-assets` (english-oak, electric-kettle plume), `🌡️catalog-physics` (horse hooves and coat-trunk areas, horse `skinFraction`: whole-catalog bounds and finiteness, not expected to change, the horse f stays 0.96), `🎨️view-synchrony` (english-oak colour merge weights).
- `🎭️cast-export` (standing-adult hoodie/jeans/sneakers and toaster/electric-kettle: area-weighted merge colours of the cast), `💨️transport-field` (standing-adult: clothing-area weights of the surface flux).

**Geometry-only tests and fixtures that mention the assets (they depend on the volumes, not on `areaM2`; the area change alters `areaM2` inside the embedded catalog fixtures only):** `🏗️structure-projection` (bar-stool, dining-table, kneeling-gardener, leaning-person, operating-table, seated-office-worker, standing-adult), `🔭️multiview-coverage` (json and ts: bar-stool, english-oak, kneeling-gardener, person-with-white-cane, single-bed, electric-kettle), `🧊️model-silhouette`, `🧊️model-frame`, `🧊️model-catalog` (kneeling-gardener, japanese-maple, london-plane-pollarded, scots-pine), `🧊️chain-state` (person-on-crutches, person-with-umbrella), `🧊️volumes-rules` (office-swivel-chair), `🧊️model-glyph-edges`, `🧊️model-glyph-size` (electric-fan-heater), `🧊️axonometric-exports` (duct-fan), `📐️dimension-leader` (standing-adult, thermostatic-radiator-valve), `📑️multiview-sheet`, `📑️sheet-model...` (standing-adult), `🔺️polygon-union` and `🧍️plan-projection` (seated-office-worker, vacuum-cleaner), `🎭️cast-export` fixture json (standing-adult).

### Passes

{PASSES}
