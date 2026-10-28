# Concrete Optimization Study: Model Report

Simplified annual teaching model of one 8 m × 6 m × 3 m seminar room (48 m²), one glazed façade of 24 m², Zurich-like climate. Model: `🧮️model/🐍️.py` (stdlib only); inputs: `🧮️model/🔣️.json`; schema: `🧮️model/🧬️schema/🔣️.schema.json`; expectations: `🧪️cases/🔣️.json`. Generated from the model by `build_report.py` in this folder.

## Baseline design (orientation 180°, WWR 45 %, overhang 0.6 m, insulation 16 cm)

| Quantity | Value |
| --- | --- |
| window_area | 10.8 m² |
| wall_area | 13.2 m² |
| u_wall | 0.193 W/(m²·K) |
| heating_demand | 557.8 kWh/a |
| cooling_demand | 741.0 kWh/a |
| heating | 159.4 kWh/a |
| cooling | 247.0 kWh/a |
| lighting | 218.6 kWh/a |
| electricity | 625.0 kWh/a |
| intensity | 13.0 kWh/(m²·a) |
| solar_winter | 972.0 kWh/a |
| solar_summer | 434.8 kWh/a |
| shading_winter | 0.471 1 |
| shading_summer | 0.526 1 |
| sun_winter | 1.000 1 |
| sun_summer | 0.575 1 |
| daylight | 2.700 % |
| autonomy | 0.663 1 |
| cost | 21,775.2 CHF |
| carbon | 3,816.0 kg CO₂e |

## 1. Orientation extrema (WWR 60 %, overhang 0 m, insulation 16 cm; 1° grid; electricity kWh/a)

| Bearing | Electricity |
| --- | --- |
| East 90° = 450° (reference) | 1,131.1 |
| Local maximum 282° | 1,145.9 |
| Local maximum 438° | 1,145.9 |
| Local minimum 180° (global, south) | 681.3 |
| Local minimum 360° (north, feasible) | 882.0 |

North is 29.5 % worse than south. Constraint [270, 450]: feasible minimum 360°. Tariff 0.25 CHF/kWh: saving at south vs east 449.8 kWh/a = 112.5 CHF/a; at north 249.1 kWh/a = 62.3 CHF/a.

Greedy ±5° descent from 315°: 315° (1,042) → 320° (1,016) → 325° (990) → 330° (965) → 335° (942) → 340° (921) → 345° (905) → 350° (892) → 355° (885) → 360° (882).

Sun: noon altitude winter 19°, summer 66°, low summer sun 30°; azimuth winter [128, 232], summer [53, 307].

## 2. Surface and samples (south, 16 cm insulation; WWR 10–80 %, overhang 0–1.2 m)

Dense 41 × 41 minimum: WWR 53.8 %, overhang 0.45 m, **608.0 kWh/a** (interior; valley of best overhang per WWR runs 0.00–1.20 m). Corners: 10 % / 0 m 808.0, 80 % / 0 m 775.5, 10 % / 1.2 m 898.1, 80 % / 1.2 m 619.2.

| Plan | Points (WWR %, d m → kWh/a) | Best |
| --- | --- | --- |
| grid | 10 %, 0.00 m → 808; 10 %, 0.60 m → 878; 10 %, 1.20 m → 898; 45 %, 0.00 m → 637; 45 %, 0.60 m → 625; 45 %, 1.20 m → 662; 80 %, 0.00 m → 775; 80 %, 0.60 m → 642; 80 %, 1.20 m → 619 | 619.2 |
| random | 14 %, 0.95 m → 850; 23 %, 0.30 m → 709; 33 %, 0.72 m → 684; 35 %, 0.78 m → 679; 47 %, 0.12 m → 618; 58 %, 0.15 m → 635; 70 %, 0.62 m → 618; 77 %, 0.30 m → 671; 18 %, 0.08 m → 721 | 618.0 |
| lhs | 14 %, 0.73 m → 841; 22 %, 0.20 m → 706; 29 %, 1.13 m → 735; 37 %, 0.87 m → 674; 45 %, 0.33 m → 612; 53 %, 0.47 m → 608; 61 %, 0.07 m → 662; 68 %, 1.00 m → 613; 76 %, 0.60 m → 631 | 608.3 |

Random set: largest empty box WWR 35–70 % × overhang 0.15–1.20 m (44 % of the domain); closest pair #2/#3 (ΔWWR 2.0 pp, Δd 0.06 m; normalized distance 0.057). LHS strata index of the points: [[0, 5], [1, 1], [2, 8], [3, 6], [4, 2], [5, 3], [6, 0], [7, 7], [8, 4]].

## 3. Factorial experiment (response: cooling electricity, kWh/a; south, 16 cm)

| Run | WWR | Overhang | Cooling | Solar summer | Shading summer |
| --- | --- | --- | --- | --- | --- |
| r1 [-1, -1] | 30 % | 0.0 m | 312.8 | 612 | 0.00 |
| r2 [-1, 1] | 30 % | 1.0 m | 163.0 | 162 | 0.74 |
| r3 [1, -1] | 70 % | 0.0 m | 566.7 | 1,428 | 0.00 |
| r4 [1, 1] | 70 % | 1.0 m | 308.0 | 652 | 0.54 |

Effects: A (WWR) 199.5, B (overhang) -204.2, AB -54.4 kWh/a. Coefficients b0 337.6, b1 99.7, b2 -102.1, b12 -27.2. Overhang saves 149.8 kWh/a at 30 % and 258.6 kWh/a at 70 %. Centre (50 %, 0.5 m): predicted 337.6, actual 285.2 (-15.5 %). At 50 % the 0.5 m overhang delivers 73 % of the 1.0 m saving.

Half fraction C = AB (C = insulation 10 / 30 cm): runs ['316.5', '158.8', '564.8', '309.6']; confounded contrast -48.7; true AB -54.4, true C 5.7, sum -48.7 kWh/a.

## 4. Variants, criteria and index (south façade)

| Variant | WWR | d | t | kWh/(m²·a) | CHF | Daylight % | kg CO₂e |
| --- | --- | --- | --- | --- | --- | --- | --- |
| punched — Punched windows / Lochfassade | 20 % | 0.0 m | 16 cm | 14.4 | 21,163 | 1.60 | 2,496 |
| ribbon — Ribbon window / Fensterband | 50 % | 0.6 m | 16 cm | 12.8 | 21,840 | 3.00 | 3,840 |
| glass_shaded — Shaded glass wall / Verschattete Glaswand | 75 % | 1.2 m | 16 cm | 12.8 | 22,452 | 4.00 | 5,160 |
| punched_insulated — Super-insulated punched windows / Hochgedämmte Lochfassade | 20 % | 0.0 m | 25 cm | 14.1 | 22,200 | 1.60 | 2,928 |
| glass_bare — Bare glass wall / Unverschattete Glaswand | 75 % | 0.0 m | 16 cm | 15.6 | 21,876 | 6.00 | 2,760 |
| glass_premium (extra) — Premium glass wall / Premium-Glaswand | 75 % | 1.2 m | 30 cm | 12.7 | 22,956 | 4.00 | 5,370 |

Weights balanced (electricity 40 %, cost 30 %, daylight 20 %, carbon 10 %); normalization 0 best … 1 worst, score lower is better: 1. ribbon 0.344; 2. punched 0.427; 3. glass_shaded 0.495; 4. glass_bare 0.576; 5. punched_insulated 0.640.

Weights cost_heavy (electricity 23 %, cost 60 %, daylight 11 %, carbon 6 %); normalization 0 best … 1 worst, score lower is better: 1. punched 0.244; 2. ribbon 0.422; 3. glass_bare 0.566; 4. punched_insulated 0.711; 5. glass_shaded 0.712.

With the sixth variant (balanced): 1. ribbon 0.309; 2. glass_shaded 0.416; 3. punched 0.432; 4. glass_premium 0.491; 5. glass_bare 0.528; 6. punched_insulated 0.578.

## 5. Pareto (south; WWR {10,25,40,55,70} % × insulation {12,20,30} cm × overhang {0,0.6} m)

Front (6 of 30): w10-t12-d0, w25-t12-d0, w40-t12-d0, w55-t12-d60, w55-t20-d60, w55-t30-d60.

| Front member | Cost CHF | Electricity kWh/a | Daylight % |
| --- | --- | --- | --- |
| w10-t12-d0 | 20,515 | 826.8 | 0.80 |
| w25-t12-d0 | 20,796 | 670.1 | 2.00 |
| w40-t12-d0 | 21,077 | 636.3 | 3.20 |
| w55-t12-d60 | 21,646 | 616.1 | 3.30 |
| w55-t20-d60 | 22,164 | 605.1 | 3.30 |
| w55-t30-d60 | 22,812 | 599.3 | 3.30 |

| Step | + CHF | − kWh/a | CHF per kWh/a | Payback 0.25 CHF/kWh (a) | Payback 0.40 CHF/kWh (a) |
| --- | --- | --- | --- | --- | --- |
| w10-t12-d0 → w25-t12-d0 | 281 | 156.7 | 1.8 | 7.2 | 4.5 |
| w25-t12-d0 → w40-t12-d0 | 281 | 33.9 | 8.3 | 33.1 | 20.7 |
| w40-t12-d0 → w55-t12-d60 | 569 | 20.2 | 28.1 | 112.6 | 70.4 |
| w55-t12-d60 → w55-t20-d60 | 518 | 10.9 | 47.4 | 189.6 | 118.5 |
| w55-t20-d60 → w55-t30-d60 | 648 | 5.8 | 111.8 | 447.2 | 279.5 |

Service life 30 a. Knee: tariff w25-t12-d0, tariff_hi w40-t12-d0. Dominated pair: w25-t12-d0 dominates w70-t30-d0 (20,796 CHF / 670.1 vs 22,416 CHF / 723.7). Step example: w10-t12-d0 → w25-t12-d0: +281 CHF → -156.7 kWh/a.

Refinement: off-grid design WWR 50 %, d 0.30 m, t 12 cm (21,408 CHF, 615.5 kWh/a) dominates ['w55-t12-d60']. Third criterion: w70-t12-d0 is dominated in cost × electricity by w25-t12-d0 but nondominated with daylight; front3 has 15 members. Weighted picks (weight on electricity → front member): 0 → w10-t12-d0; 0.25 → w25-t12-d0; 0.5 → w40-t12-d0; 0.75 → w55-t12-d60; 1 → w55-t30-d60.

## 6. Algorithms (south surface; start WWR 75 %, overhang 0.1 m = 712 kWh/a)

Gradient descent (10 iterates, 5 runs each, 50 runs): (75 %, 0.10 m, 712) → (48 %, 0.65 m, 622) → (61 %, 0.56 m, 609) → (57 %, 0.58 m, 608) → (58 %, 0.57 m, 608) → (57 %, 0.56 m, 608) → (57 %, 0.56 m, 608) → (57 %, 0.55 m, 608) → (57 %, 0.55 m, 608) → (57 %, 0.54 m, 608). First slope 4.65 kWh/a per %-point, -320 kWh/a per m; the second iterate overshoots the valley floor (WWR 48 % < 54 %) and recovers.

Pattern search (13 iterates, 49 runs, 6 halvings): (75 %, 0.10 m, 712 halve) → (55 %, 0.10 m, 637) → (55 %, 0.40 m, 609) → (55 %, 0.40 m, 609 halve) → (55 %, 0.55 m, 608) → (55 %, 0.55 m, 608 halve) → (55 %, 0.48 m, 608) → (55 %, 0.48 m, 608 halve) → (55 %, 0.48 m, 608 halve) → (55 %, 0.48 m, 608 halve) → (55 %, 0.48 m, 608) → (55 %, 0.48 m, 608 halve) → (55 %, 0.48 m, 608).

Genetic algorithm (population 8, 2 elites, 6 children, tournament of 2, Park-Miller LCG seed 313, 7 generations, 50 runs): best per generation 651.6, 639.4, 626.4, 625.2, 613.5, 608.6, 608.6, 608.6.

Convergence (budget 50, map minimum 608.0 kWh/a over 1681 runs): final gradient 608.1 (50 runs), pattern 608.0 (49 runs), genetic 608.6 (50 runs); best after 15 runs: gradient 608.9, pattern 608.8, genetic 639.4.

## Coefficients (all in `🔣️.json`, each with a bilingual `note`)

| Section.name | Value | Unit | Meaning |
| --- | --- | --- | --- |
| room.width | 8 | m | Room and façade width. |
| room.depth | 6 | m | Room depth behind the façade. |
| room.height | 3 | m | Room and façade height. |
| climate.heating_degree_hours | 70000 | Kh/a | Indoor-outdoor temperature difference summed over the heated, occupied hours. |
| climate.cooling_degree_hours | 8000 | Kh/a | Same for the hours when heat can be rejected through the envelope in summer. |
| climate.winter_irradiance_mean | 177.5 | kWh/(m²·a) | Heating season, occupied hours: mean vertical solar irradiation over all bearings. |
| climate.winter_irradiance_cos1 | 135 | kWh/(m²·a) | Winter: extra on the south façade, less on the north façade (low sun). |
| climate.winter_irradiance_cos2 | 27.5 | kWh/(m²·a) | Winter: south and north receive a bit more than east and west. |
| climate.summer_irradiance_mean | 222.5 | kWh/(m²·a) | Cooling season, occupied hours: mean vertical solar irradiation over all bearings. |
| climate.summer_irradiance_cos1 | 20 | kWh/(m²·a) | Summer: south still gets more than north. |
| climate.summer_irradiance_cos2 | -72.5 | kWh/(m²·a) | Summer: east and west get the most (low morning and evening sun). |
| climate.internal_gains_winter | 300 | kWh/a | People, laptops and lamps in the heating season. |
| climate.internal_gains_summer | 500 | kWh/a | People, laptops and lamps in the cooling season. |
| climate.utilization_exponent_heating | 4 | 1 | Thermal inertia: how well free heat is used (EN ISO 13790 style, tau about 45 h). |
| climate.utilization_exponent_cooling | 4 | 1 | Thermal inertia when losses are used to remove summer heat. |
| envelope.r_si | 0.13 | m²·K/W | Inner surface resistance. |
| envelope.r_se | 0.04 | m²·K/W | Outer surface resistance. |
| envelope.r_masonry | 0.45 | m²·K/W | Load-bearing wall without insulation. |
| envelope.conductivity_insulation | 0.035 | W/(m·K) | Mineral wool insulation. |
| envelope.u_window | 0.9 | W/(m²·K) | Triple glazing including frame. |
| envelope.h_other | 12 | W/K | Roof, floor and ventilation losses with heat recovery, fixed. |
| envelope.g_value | 0.5 | 1 | Share of solar energy that passes the glazing (frame and dirt included). |
| shading.orientation_floor | 0.2 | 1 | Overhang effectiveness on the shadow-side bearings (sun not frontal). |
| shading.orientation_exponent | 2 | 1 | How quickly the overhang loses effect away from south. |
| shading.winter_height_ratio | 0.5 | 1 | Overhang depth, in window heights, that shades half the glass at the low winter sun. |
| shading.summer_height_ratio | 0.4 | 1 | Same for the high summer sun: a short overhang already shades a lot. |
| daylight.transmittance | 0.6 | 1 | Visible light transmittance including frame and dirt. |
| daylight.sky_angle | 75 | ° | Visible sky angle seen from the window without overhang. |
| daylight.sky_angle_loss | 0.5 | 1 | Largest share of the sky angle a very deep overhang can hide. |
| daylight.overhang_half_depth | 0.6 | m | Overhang depth that hides half of that largest share. |
| daylight.surface_area | 180 | m² | All interior surfaces of the room. |
| daylight.reflectance | 0.5 | 1 | Mean interior reflectance. |
| daylight.reference_factor | 2 | % | Daylight factor at which about two thirds of the maximum autonomy is reached. |
| daylight.max_autonomy | 0.85 | 1 | Largest share of lighting hours that daylight can cover. |
| daylight.orientation_gain | 0.12 | 1 | Sun-facing bearings give slightly more usable daylight. |
| systems.cop | 3.5 | 1 | Seasonal coefficient of performance of the heat pump. |
| systems.eer | 3 | 1 | Seasonal energy efficiency ratio of the chiller. |
| lighting.power_density | 9 | W/m² | Installed lighting power per floor area. |
| lighting.hours | 1500 | h/a | Hours per year in which the room needs light. |
| prices.fixed | 9000 | CHF | Fixed façade items independent of the design: structure, scaffolding, finishes. |
| prices.wall | 400 | CHF/m² | Opaque wall without insulation. |
| prices.window | 550 | CHF/m² | Triple-glazed window with frame. |
| prices.insulation_per_cm | 6 | CHF/(m²·cm) | Insulation per square metre of opaque wall and centimetre of thickness. |
| prices.overhang | 60 | CHF/m² | Projecting concrete slab per square metre. |
| economics.tariff | 0.25 | CHF/kWh | Electricity price today. |
| economics.tariff_hi | 0.4 | CHF/kWh | Electricity price in a high-price scenario. |
| economics.service_life | 30 | a | Years a façade is expected to serve before it is replaced. |
| carbon.wall | 60 | kg CO₂e/m² | Opaque wall without insulation. |
| carbon.window | 120 | kg CO₂e/m² | Triple-glazed window with frame. |
| carbon.insulation_per_cm | 2.5 | kg CO₂e/(m²·cm) | Insulation per square metre and centimetre. |
| carbon.overhang | 250 | kg CO₂e/m² | Projecting concrete slab per square metre. |

## Notes for scene agents

- `index`: normalized 0 = best … 1 = worst per criterion (daylight reversed), score = weighted mean (lower is better), rank 1 = lowest score. `tournaments` rows are `[candidate, candidate, winner]`.
- `gradient` is per unit (WWR ratio 1.0, metre); `slope` is per percentage point and per metre.
- Pattern `step` is the step used for the polls of that round (the start iterate carries the initial step); a stalled round (`moved` false) halves the step of the next round.
- Genetic `evaluations` of a generation is cumulative including that generation's population (8, 14, … 50); elites keep their ids across generations.
- Sample/surface/pareto designs all use the south façade with 16 cm insulation unless stated.

## Test results

```
python 🧪️tests/🐍️.py (all 61 tests, SEMIO_TEACHING_OUTPUT_ROOT set): Ran 61 tests in 658 s, OK
  mathematics (StudyModel, OrientationExtrema, SurfaceAndSampling, FactorialExperiments, IndexOfVariants, ParetoFront, AlgorithmPaths, SchemaFirstStudy): 56 tests, OK, about 4 s
  caption/narration/scene-discovery/curriculum contract (OptimizationClipContract): 5 tests, OK against the scene files as they were at run time
node 🧪️tests/🟨️.cjs: Optimization schema: authored model accepted; 12 invalid models rejected.
Independent checks: vectorized NumPy re-implementation of evaluate on 300 random designs (rtol 1e-9); scipy minimize_scalar(bounded) and brentq for the orientation extrema; scipy Nelder-Mead from five starts vs the 41 x 41 surface minimum; scipy.stats.qmc.discrepancy and floor-stratum checks for the Latin hypercube; scipy pdist and brute-force rectangle search for the sampling diagnostics; numpy.linalg.lstsq for the 2^2 and 2^3 factorial coefficients; NumPy dominance matrices for the front and the three-criteria front; NumPy min-max normalization for the index; Park-Miller LCG re-run for the initial population; AJV (strict, 2020-12) with 12 invalid mutations.
```

