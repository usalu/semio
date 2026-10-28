# Concrete Optimization Study: Model Report

Simplified annual teaching model of one 8 m × 6 m × 3 m seminar room (48 m²), one glazed façade of 24 m², Zurich-like climate. Model: `🧮️model/🐍️.py` (stdlib only); inputs: `🧮️model/🔣️.json`; schema: `🧮️model/🧬️schema/🔣️.schema.json`; expectations: `🧪️cases/🔣️.json`. Generated from the model by `build_report.py` in this folder.

## Baseline design (orientation 180°, WWR 45 %, overhang 0.6 m, insulation 16 cm)

| Quantity | Value |
| --- | --- |
| window_area | 10.8 m² |
| wall_area | 13.2 m² |
| u_wall | 0.193 W/(m²·K) |
| heating_demand | 203.9 kWh/a |
| cooling_demand | 735.6 kWh/a |
| heating | 58.3 kWh/a |
| cooling | 245.2 kWh/a |
| lighting | 218.6 kWh/a |
| electricity | 522.1 kWh/a |
| intensity | 10.9 kWh/(m²·a) |
| solar_winter | 1,836.0 kWh/a |
| solar_summer | 429.4 kWh/a |
| shading_winter | 0.000 1 |
| shading_summer | 0.887 1 |
| sun_winter | 1.000 1 |
| sun_summer | 0.575 1 |
| daylight | 2.700 % |
| autonomy | 0.663 1 |
| cost | 20,359.2 CHF |
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

Sun: noon altitude winter 19.16°, summer 66.04°, low summer sun 30°; azimuth winter [130.2, 229.8], summer [58, 302].

## 2. Surface and samples (south, 16 cm insulation; WWR 10–80 %, overhang 0–1.2 m)

Dense 41 × 41 minimum: WWR 66.0 %, overhang 0.99 m, **494.8 kWh/a** (interior; valley of best overhang per WWR runs 0.27–1.20 m). Corners: 10 % / 0 m 808.0, 80 % / 0 m 775.5, 10 % / 1.2 m 846.4, 80 % / 1.2 m 499.2.

| Plan | Points (WWR %, d m → kWh/a) | Best |
| --- | --- | --- |
| grid | 10 %, 0.00 m → 808; 10 %, 0.60 m → 809; 10 %, 1.20 m → 846; 45 %, 0.00 m → 637; 45 %, 0.60 m → 522; 45 %, 1.20 m → 547; 80 %, 0.00 m → 775; 80 %, 0.60 m → 616; 80 %, 1.20 m → 499 | 499.2 |
| random | 14 %, 0.95 m → 770; 23 %, 0.30 m → 625; 33 %, 0.72 m → 568; 35 %, 0.78 m → 562; 47 %, 0.12 m → 634; 58 %, 0.15 m → 657; 70 %, 0.62 m → 568; 77 %, 0.30 m → 686; 18 %, 0.08 m → 707 | 561.8 |
| lhs | 14 %, 0.73 m → 757; 22 %, 0.20 m → 655; 29 %, 1.13 m → 625; 37 %, 0.87 m → 557; 45 %, 0.33 m → 572; 53 %, 0.47 m → 554; 61 %, 0.07 m → 682; 68 %, 1.00 m → 496; 76 %, 0.60 m → 599 | 495.9 |

Random set: largest empty box WWR 35–70 % × overhang 0.15–1.20 m (44 % of the domain); closest pair #2/#3 (ΔWWR 2.0 pp, Δd 0.06 m; normalized distance 0.057). LHS strata index of the points: [[0, 5], [1, 1], [2, 8], [3, 6], [4, 2], [5, 3], [6, 0], [7, 7], [8, 4]].

## 3. Factorial experiment (response: cooling electricity, kWh/a; south, 16 cm)

| Run | WWR | Overhang | Cooling | Solar summer | Shading summer |
| --- | --- | --- | --- | --- | --- |
| r1 [-1, -1] | 30 % | 0.0 m | 312.8 | 612 | 0.00 |
| r2 [-1, 1] | 30 % | 1.0 m | 190.5 | 245 | 1.00 |
| r3 [1, -1] | 70 % | 0.0 m | 566.7 | 1,428 | 0.00 |
| r4 [1, 1] | 70 % | 1.0 m | 287.8 | 591 | 0.98 |

Effects: A (WWR) 175.6, B (overhang) -200.6, AB -78.3 kWh/a. Coefficients b0 339.4, b1 87.8, b2 -100.3, b12 -39.2. Overhang saves 122.3 kWh/a at 30 % and 278.9 kWh/a at 70 %. Centre (50 %, 0.5 m): predicted 339.4, actual 298.3 (-12.1 %). At 50 % the 0.5 m overhang delivers 69 % of the 1.0 m saving.

Half fraction C = AB (C = insulation 10 / 30 cm): runs ['316.5', '186.3', '564.8', '289.4']; confounded contrast -72.6; true AB -78.3, true C 5.7, sum -72.6 kWh/a.

## 4. Variants, criteria and index (south façade)

| Variant | WWR | d | t | kWh/(m²·a) | CHF | Daylight % | kg CO₂e |
| --- | --- | --- | --- | --- | --- | --- | --- |
| punched — Punched windows / Lochfassade | 25 % | 0.0 m | 16 cm | 13.7 | 19,428 | 2.00 | 2,520 |
| ribbon — Ribbon window / Fensterband | 50 % | 0.6 m | 16 cm | 10.9 | 20,544 | 3.00 | 3,840 |
| glass_shaded — Shaded glass wall / Verschattete Glaswand | 70 % | 1.2 m | 16 cm | 10.4 | 21,475 | 3.36 | 5,136 |
| punched_insulated — Super-insulated punched windows / Hochgedämmte Lochfassade | 25 % | 0.0 m | 30 cm | 13.4 | 20,940 | 2.00 | 3,150 |
| glass_bare — Bare glass wall / Unverschattete Glaswand | 70 % | 0.0 m | 16 cm | 15.1 | 21,091 | 5.60 | 2,736 |
| glass_premium (extra) — Premium glass wall / Premium-Glaswand | 70 % | 1.2 m | 30 cm | 10.4 | 22,080 | 3.36 | 5,388 |

Weights balanced (electricity 40 %, cost 30 %, daylight 20 %, carbon 10 %); normalization 0 best … 1 worst, score lower is better: 1. ribbon 0.400; 2. punched 0.482; 3. glass_shaded 0.524; 4. glass_bare 0.652; 5. punched_insulated 0.700.

Weights cost_heavy (electricity 23 %, cost 60 %, daylight 11 %, carbon 6 %); normalization 0 best … 1 worst, score lower is better: 1. punched 0.276; 2. ribbon 0.462; 3. punched_insulated 0.717; 4. glass_bare 0.721; 5. glass_shaded 0.728.

With the sixth variant (balanced): 1. ribbon 0.361; 2. glass_shaded 0.450; 3. punched 0.483; 4. glass_premium 0.524; 5. glass_bare 0.596; 6. punched_insulated 0.648.

## 5. Pareto (south; WWR {30, 40, 50, 60, 70, 80} % × insulation {12, 20, 30} cm × overhang {0.0, 0.6} m)

Front (8 of 36): w30-t12-d0, w30-t12-d60, w40-t12-d60, w50-t12-d60, w40-t20-d60, w50-t20-d60, w40-t30-d60, w50-t30-d60.

| Front member | Cost CHF | Electricity kWh/a | Daylight % |
| --- | --- | --- | --- |
| w30-t12-d0 | 19,210 | 648.9 | 2.40 |
| w30-t12-d60 | 19,402 | 583.9 | 1.80 |
| w40-t12-d60 | 19,829 | 533.1 | 2.40 |
| w50-t12-d60 | 20,256 | 527.5 | 3.00 |
| w40-t20-d60 | 20,520 | 524.4 | 2.40 |
| w50-t20-d60 | 20,832 | 522.4 | 3.00 |
| w40-t30-d60 | 21,384 | 520.1 | 2.40 |
| w50-t30-d60 | 21,552 | 519.8 | 3.00 |

| Step | + CHF | − kWh/a | CHF per kWh/a | Payback 0.25 CHF/kWh (a) | Payback 0.40 CHF/kWh (a) |
| --- | --- | --- | --- | --- | --- |
| w30-t12-d0 → w30-t12-d60 | 192 | 64.9 | 3.0 | 11.8 | 7.4 |
| w30-t12-d60 → w40-t12-d60 | 427 | 50.9 | 8.4 | 33.6 | 21.0 |
| w40-t12-d60 → w50-t12-d60 | 427 | 5.5 | 77.2 | 308.7 | 193.0 |
| w50-t12-d60 → w40-t20-d60 | 264 | 3.2 | 83.7 | 334.6 | 209.1 |
| w40-t20-d60 → w50-t20-d60 | 312 | 2.0 | 156.1 | 624.4 | 390.3 |
| w50-t20-d60 → w40-t30-d60 | 552 | 2.3 | 240.9 | 963.5 | 602.2 |
| w40-t30-d60 → w50-t30-d60 | 168 | 0.3 | 669.9 | 2,679.6 | 1,674.8 |

Service life 30 a. Knee: tariff w30-t12-d60, tariff_hi w40-t12-d60. Dominated pair: w30-t12-d0 dominates w70-t30-d0 (19,210 CHF / 648.9 vs 21,696 CHF / 723.7). Step example: w30-t12-d0 → w30-t12-d60: +192 CHF → -64.9 kWh/a.

Refinement: the dense 41 × 41 surface optimum (south, 16 cm), an off-grid design, WWR 66 %, d 0.99 m, t 16 cm (21,260 CHF, 494.8 kWh/a) dominates ['w40-t30-d60', 'w50-t30-d60']. Third criterion: w80-t12-d0 is dominated in cost × electricity by w30-t12-d0 but nondominated with daylight; front3 has 30 members. Weighted picks (weight on electricity → front member): 0 → w30-t12-d0; 0.25 → w30-t12-d60; 0.5 → w40-t12-d60; 0.75 → w40-t12-d60; 1 → w50-t30-d60.

## 6. Algorithms (south surface; start WWR 75 %, overhang 0.1 m = 745 kWh/a)

Gradient descent (10 iterates, 5 runs each, 50 runs): (75 %, 0.10 m, 745) → (55 %, 0.26 m, 613) → (44 %, 0.56 m, 528) → (45 %, 0.73 m, 515) → (57 %, 0.65 m, 524) → (50 %, 0.85 m, 509) → (60 %, 0.76 m, 511) → (52 %, 0.97 m, 513) → (61 %, 0.89 m, 497) → (58 %, 1.00 m, 504). First slope 5.02 kWh/a per %-point, -136 kWh/a per m; the descent overshoots: electricity rises at iterate(s) [5, 7, 8, 10] before it recovers.

Pattern search (13 iterates, 49 runs, 3 halvings): (75 %, 0.10 m, 745 halve) → (75 %, 0.40 m, 649) → (75 %, 0.70 m, 566) → (75 %, 1.00 m, 507) → (75 %, 1.20 m, 499) → (80 %, 1.20 m, 499) → (80 %, 1.20 m, 499 halve) → (80 %, 1.20 m, 499 halve) → (80 %, 1.20 m, 499 halve) → (78 %, 1.20 m, 499) → (78 %, 1.16 m, 498) → (75 %, 1.16 m, 498) → (75 %, 1.12 m, 497).

Genetic algorithm (population 8, 2 elites, 6 children, tournament of 2, Park-Miller LCG seed 313, 7 generations, 50 runs): best per generation 586.6, 548.3, 524.1, 522.7, 513.7, 506.3, 498.1, 498.1.

Convergence (budget 50, map minimum 494.8 kWh/a over 1681 runs): final gradient 496.4 (50 runs), pattern 496.7 (49 runs), genetic 498.1 (50 runs); best after 15 runs: gradient 526.0, pattern 507.5, genetic 548.3.

## Coefficients (all in `🔣️.json`, each with a bilingual `note`)

| Section.name | Value | Unit | Meaning |
| --- | --- | --- | --- |
| room.width | 8 | m | Room and façade width. |
| room.depth | 6 | m | Room depth behind the façade. |
| room.height | 3 | m | Room and façade height. |
| room.head | 2.7 | m | Window head height above the floor; the overhang slab sits on top of the wall, so 0.3 m of wall stay above the window. |
| climate.heating_degree_hours | 70000 | Kh/a | Indoor-outdoor temperature difference summed over the heated, occupied hours. |
| climate.cooling_degree_hours | 8000 | Kh/a | Same for the hours when heat can be rejected through the envelope in summer. |
| climate.winter_irradiance_mean | 177.5 | kWh/(m²·a) | Heating season, occupied hours: mean vertical solar irradiation over all bearings. |
| climate.winter_irradiance_cos1 | 135 | kWh/(m²·a) | Winter: extra on the south façade, less on the north façade (low sun). |
| climate.winter_irradiance_cos2 | 27.5 | kWh/(m²·a) | Winter: south and north receive a bit more than east and west. |
| climate.summer_irradiance_mean | 222.5 | kWh/(m²·a) | Cooling season, occupied hours: mean vertical solar irradiation over all bearings. |
| climate.summer_irradiance_cos1 | 20 | kWh/(m²·a) | Summer: south still gets more than north. |
| climate.summer_irradiance_cos2 | -72.5 | kWh/(m²·a) | Summer: east and west get the most (low morning and evening sun). |
| climate.direct_share_winter | 0.5 | 1 | Share of the winter irradiation that arrives as direct beam; only this share can be shaded, the diffuse rest always enters. |
| climate.direct_share_summer | 0.6 | 1 | Same for summer: the clear summer sky is more direct. |
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
| prices.wall | 300 | CHF/m² | Opaque wall without insulation. |
| prices.window | 550 | CHF/m² | Triple-glazed window with frame. |
| prices.insulation_per_cm | 6 | CHF/(m²·cm) | Insulation per square metre of opaque wall and centimetre of thickness. |
| prices.overhang | 40 | CHF/m² | Projecting concrete slab per square metre. |
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

## Geometric shading

The overhang slab sits on the wall top (3.0 m) and the window head at 2.7 m (`room.head`), so 0.3 m of wall stay above the window: the same numbers as `🎨️visuals/🏠️room/🐍️.py` (`HEAD = 2.7`, `HEIGHT = 3.0`, window height = min(area / 8 m, HEAD − 0.05), overhang tip at depth d from the glass plane). They agree.

Shaded share of the window height at one sun position = clamp((d · tan(profile angle) − 0.3 m) / window height, 0, 1) with tan(profile) = tan(altitude) / cos(angle across the façade normal); sun behind the façade (across ≥ 89°) or below 3° altitude gives no direct sun. `shading_winter` / `shading_summer` are the beam-weighted averages (weight cos(altitude) · cos(across)) over the hourly solstice sun positions noon ± 4 h that shine on the façade. Solar gain = window · g · irradiance · (1 − direct_share · shading).

South, summer solstice: -4 h alt 37.1° across 85.1° profile 83.5° weight 0.07; -3 h alt 47.0° across 72.2° profile 74.1° weight 0.21; -2 h alt 56.2° across 55.5° profile 69.2° weight 0.32; -1 h alt 63.2° across 31.8° profile 66.8° weight 0.38; +0 h alt 66.0° across 0.0° profile 66.0° weight 0.41; +1 h alt 63.2° across 31.8° profile 66.8° weight 0.38; +2 h alt 56.2° across 55.5° profile 69.2° weight 0.32; +3 h alt 47.0° across 72.2° profile 74.1° weight 0.21; +4 h alt 37.1° across 85.1° profile 83.5° weight 0.07.

Baseline (d = 0.6 m, WWR 45 %): shading_winter 0.00 (the 19° winter sun casts only 0.21 m of shadow, less than the 0.3 m gap), shading_summer 0.89.

The toolkit's `sun_state` takes the single sun-path point nearest to the façade bearing (noon for south); the model offers the same function as `sun_state(orientation, season)` but averages the hourly positions, so south summer shading is a little higher than the noon ray alone (the toolkit already bends the drawn ray so that round(7 · shading) rays are blocked).

## Test results

```
python 🧪️tests/🐍️.py (all 63 tests, SEMIO_TEACHING_OUTPUT_ROOT set): Ran 63 tests in 1492 s, OK
  mathematics (StudyModel, OrientationExtrema, SurfaceAndSampling, FactorialExperiments, IndexOfVariants, ParetoFront, AlgorithmPaths, SchemaFirstStudy): 58 tests, OK, about 8 s
  caption/narration/scene-discovery/curriculum contract (OptimizationClipContract): 5 tests, OK against the scene files as they were at run time
node 🧪️tests/🟨️.cjs: Optimization schema: authored model accepted; 12 invalid models rejected.
Independent checks: vectorized NumPy re-implementation of evaluate (including the hourly sun geometry and beam-weighted shading) on 300 random designs (rtol 1e-9); geometric shading re-derived per hour with NumPy; scipy minimize_scalar(bounded) and brentq for the orientation extrema; scipy Nelder-Mead from five starts vs the 41 x 41 surface minimum; scipy.stats.qmc.discrepancy and floor-stratum checks for the Latin hypercube; scipy pdist and brute-force rectangle search for the sampling diagnostics; numpy.linalg.lstsq for the 2^2 and 2^3 factorial coefficients; NumPy dominance matrices for the front and the three-criteria front; NumPy min-max normalization for the index; Park-Miller LCG re-run for the initial population; AJV (strict, 2020-12) with 12 invalid mutations.
```

