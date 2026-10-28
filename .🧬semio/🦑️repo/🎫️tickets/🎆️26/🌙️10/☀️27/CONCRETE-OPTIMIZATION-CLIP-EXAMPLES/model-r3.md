# Model Round 3: Continuous Shading And Energy Balance Terms

Files changed (model owner only): `C/🧮️model/🐍️.py`, `C/🧮️model/🔣️.json`, `C/🧮️model/🧬️schema/🔣️.schema.json`, `C/🧪️cases/🔣️.json`, `C/🧪️tests/🐍️.py`. `C/🧪️tests/🟨️.cjs` needed no edit: it reads `invalidModels` from the cases file, which gained two entries. Probes, dumps and diffs: `T/model-r3-*.py`, outputs in `T/🗑️generated/model-r3/` (`before.json`, `after.json`, `changed.json`, `continuity-before.json`, `continuity-after.json`, `tests-full.log`).

## 1. The shading rule

Before: shading = (sum of beam weight × shaded share) / (sum of beam weight), taken over the hourly solstice sun positions that shine on the façade (angle across the normal below 89°). The ratio hides how much beam arrives: one grazing hour of weight 0.015 was a "fully shaded façade" (shading 1.0), and one degree further (354° to 355°) the sample list is empty and shading is 0. The harmonic irradiance term is the same in both cases, so cooling jumped 222 to 410 kWh/a (the same happened for any bearing with only weak hours: 325°-354° and 6°-35° in summer, 50°-70° and 290°-310° in winter, where the sample appears one hour at a time).

Now, per season:

```
w_i      = cos(altitude_i) · cos(across_i)             beam weight of hour i, across_i < 90°  (0 at 90°)
B        = Σ w_i                                        beam delivered to the façade   (key beam_<season>)
N        = Σ w_i · s_i                                  shaded beam, s_i = clamp((d · tan(profile_i) − gap) / window height, 0, 1)
shading  = N / max(B, beam_reference)                   (key shading_<season>)
reach    = direct_share · min(1, B / beam_reference)    direct share that really arrives   (key direct_<season>)
solar    = window · g · irradiance(θ) · (1 − direct_share · shading)
```

`beam_reference = 0.5` (new coefficient `parameters.climate.beam_reference`, unit 1, bilingual note). Reading: of the climate irradiance the direct part is `reach · irradiance`, the diffuse rest `irradiance − reach · irradiance`; only the direct part can be shaded; `direct_share · shading = reach · (N / B)` is the shaded share of the direct part (in the well lit case `reach = direct_share` and `N / B` is the old beam-weighted mean). With no sun hour on the façade `B = 0`, `reach = 0`, shading 0: nothing direct to shade, the total gain equals the bare-window gain (asserted: `solar_summer(360°, d = 1.2) == solar_summer(360°, d = 0)`). Total irradiance, hence every d = 0 value, is untouched.

Two consequences of "weight by the delivered beam":

- Hours with `across` between 89° and 90° (weight at most 0.009) now count in the shading sum so that an hour does not appear with a finite weight; the public list `sun_samples()` / `sun_state()` keeps the 89° rule (toolkit rays, "direct sun N h" are identical: compared for all integer bearings 90-450, both seasons).
- Where the façade collects at least `beam_reference` of beam (every bearing with a real sun path on it: south summer B = 2.36, winter B = 5.99; east/west summer 2.14, winter 1.35) the formula is the old beam-weighted mean, bit for bit.

## 2. Continuity (proof and numbers)

Analytic: `w_i` is continuous in the bearing and vanishes when hour i leaves the façade (cos 90° = 0), so hours enter and leave with weight 0; the profile angle is continuous (capped at 85°) hence `s_i` is continuous in bearing, depth and window share; `max(B, 0.5) ≥ 0.5 > 0` so the quotient is continuous; `reach` is continuous; the irradiance harmonic is continuous; heating and cooling are continuous functions (`max(0, ·)`, utilization factor are continuous); lighting does not depend on shading. All terms are products and sums of continuous factors, none multiplies a jumping factor. The altitude test (≥ 3°) depends on the hour only, not on the bearing.

Numeric: θ = 90…450° in 0.25° steps (1,441 bearings), d ∈ {0, 0.3, 0.6, 1.0, 1.2} m, WWR ∈ {30, 60, 80} %, t = 16 cm (15 curves × 4 quantities), also at 0.0625°. Bound (stated and tested, `cases.continuity.step_bound`): a step between neighbouring 0.25° points is at most **2 kWh/a heating, 4 kWh/a cooling, 0.15 kWh/a lighting, 4 kWh/a total** (≤ 16 kWh/(a·°); for scale the unshaded d = 0 curve already has steps up to 2.2 kWh/a). Found maxima after the fix: **heating 1.53, cooling 3.76, lighting 0.107, total 3.48 kWh/a** (cooling and total at WWR 80 %, d 0.6 m, 328°). Before the fix: heating 106.7, cooling 252.2, total 252.3 kWh/a (354°; the heating jump at 309.75°).

Scaling check: for a Lipschitz curve the largest step at 0.25° is four times the largest step at 0.0625°; a jump keeps its size. Ratio of the two largest steps over all 60 curves: min 3.96, max 4.00 (before the fix: 0.81…4.00, ratio 1.0 at 354° and 309.75°); tested with a limit of 3.3.

| WWR | d (m) | heating | cooling | lighting | total | total before fix | smallest step ratio |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 30 % | 0.0 | 0.52 | 0.83 | 0.105 | 1.15 | 1.1 | 4.00 |
| 30 % | 0.3 | 0.61 | 1.59 | 0.107 | 1.43 | 94.6 | 3.97 |
| 30 % | 0.6 | 0.70 | 1.18 | 0.106 | 1.34 | 94.6 | 3.98 |
| 30 % | 1.0 | 0.83 | 1.18 | 0.103 | 1.32 | 94.6 | 3.97 |
| 30 % | 1.2 | 0.63 | 1.18 | 0.101 | 1.32 | 94.6 | 3.97 |
| 60 % | 0.0 | 0.78 | 1.67 | 0.065 | 1.79 | 1.8 | 4.00 |
| 60 % | 0.3 | 0.90 | 2.51 | 0.074 | 2.46 | 189.2 | 3.98 |
| 60 % | 0.6 | 1.04 | 2.35 | 0.088 | 2.07 | 189.2 | 3.97 |
| 60 % | 1.0 | 1.28 | 2.15 | 0.098 | 2.33 | 189.2 | 3.98 |
| 60 % | 1.2 | 1.41 | 2.37 | 0.100 | 2.59 | 189.2 | 3.98 |
| 80 % | 0.0 | 0.91 | 2.22 | 0.040 | 2.19 | 2.2 | 4.00 |
| 80 % | 0.3 | 1.04 | 3.00 | 0.049 | 3.12 | 252.3 | 3.99 |
| 80 % | 0.6 | 1.20 | 3.76 | 0.065 | 3.48 | 252.3 | 3.96 |
| 80 % | 1.0 | 1.40 | 2.95 | 0.079 | 2.78 | 252.3 | 3.98 |
| 80 % | 1.2 | 1.53 | 2.78 | 0.083 | 2.98 | 252.3 | 3.98 |

(maximum |step| in kWh/a per 0.25°; "before fix" column is the old model's total.) Tests: `BearingContinuity` (NumPy re-derivation of the new rule on the same grid, steps and scaling; the public `evaluate` against it at 17 grazing bearings; `evaluate` on the same grid for two curves) and `StudyModel.test_direct_gain_fades_with_the_beam_and_the_diffuse_rest_never_jumps`.

At the reported case (WWR 45 %, d 0.6 m, t 16 cm):

| Bearing | shading summer before → after | cooling kWh/a before → after | total kWh/a before → after |
| --- | --- | --- | --- |
| 340° | 1.000 → 0.467 | 208.8 → 294.0 | 764.3 → 849.5 |
| 345° | 1.000 → 0.279 | 203.2 → 312.4 | 760.6 → 869.8 |
| 350° | 1.000 → 0.141 | 199.1 → 323.9 | 757.8 → 882.5 |
| 354° | 1.000 → 0.030 | 197.0 → 334.8 | 756.2 → 894.1 |
| 354.5° | 0.000 → 0.016 | 338.6 → 336.3 | 897.9 → 895.6 |
| 355° | 0.000 → 0.002 | 338.1 → 337.8 | 897.5 → 897.2 |
| 360° | 0.000 → 0.000 | 336.0 → 336.0 | 895.6 → 895.6 |

The 354° → 355° step is now 3.0 kWh/a cooling (was 141 here, 222 → 410 with the extrema scene's other window and insulation values).

## 3. Changed values

Method: `evaluate` dumped for 4,568 designs before and after (`T/model-r3-dump.py`): everything the public functions use (baseline, orientation study, 41 × 41 surface, grid / random / LHS samples, factorial, fractional, variants, Pareto grid, gradient / pattern / genetic paths, convergence = 2,963 designs), all south WWR × d combinations of the check scripts (`check-*`, `verify-check-*`), the extrema sweeps (WWR 30 / 60 / 80 % at every integer bearing 90-450°, d = 0 as in `orientation.design`), plus a grid of 5 WWR × 5 d × 73 bearings (step 5°) and the bearings 354 / 354.5 / 355 / 365 / 366°. Also `sun_samples` for every integer bearing and both seasons, and `sun_state`.

- **Unchanged:** all 2,963 designs the scenes use; every design with θ = 180°; every design with d = 0 (all extrema curves, extrema table values, 43 / 503 / 135 kWh/a at the example, savings, minima 180° / 360°); `sun_samples`, `sun_state`; the outputs of `evaluate()`, `orientation_study`, `surface`, `samples`, `factorial`, `fractional`, `variants`, `index`, `pareto`, `gradient_path`, `pattern_path`, `genetic`, `convergence`, `sampling_diagnostics` (compared with the old keys only, `T/model-r3-compare.py`: identical).
- **Changed: 513 of 4,568, all of them with d > 0 and θ ≠ 180°, i.e. façades the sun only grazes.** Largest effects (maximum over WWR and d of the grid, kWh/a):

| Bearing | Season | max \|Δ cooling\| | max \|Δ heating\| | max \|Δ shading\| |
| --- | --- | --- | --- | --- |
| 6°-35° and 325°-354° (north-ish) | summer | 137.8 at 6° / 354°, 221.9 at 10° / 350°, 194.2 at 15° / 345°, 151.4 at 20° / 340°, 83.7 at 25° / 335°, 9.8 at 30° / 330°, 2.2 at 35° / 325° | 0 | 0.97 |
| 354.5°, 355°, 5° | summer | 2.3, 0.55, 0.55 | 0 | 0.016, 0.002 |
| 50°-70° and 290°-310° (winter low sun at the side) | winter | 0 | 3.4 at 50° / 310°, 52.9 at 55° / 305°, 20.5 at 60° / 300°, 11.5 at 65° / 295°, 0.5 at 70° / 290° | 0.79 |
| 105°, 255° | winter | 0 | 0.43 | 0.004 (an hour at 89°-90° now carries its tiny weight) |

Full table (all changed bearings): `T/model-r3-summary.py` output; list of changed designs with before / after values per key: `T/🗑️generated/model-r3/changed.json`.

Printed values per clip that change: **none** (scenes print only θ = 180° values with d > 0, and bearing sweeps with d = 0). The extrema clip can now go back to scope tiles with an overhang at any bearing, the former 354° / 355° jump (verifier M4) is gone.

Coefficients' documentation: new `parameters.climate.beam_reference` (value 0.5, unit 1, bilingual note, positive in the schema); `sun.note` (en, de) names the weights and the denominator. Schema: `beam_reference` required in `climate`, value `exclusiveMinimum: 0`; `build_schema.py` in `T/model-tools` is stale (it lacks the `brief` section) and was not re-run, the schema was edited in place.

## 4. New `evaluate` keys (additive, existing keys unchanged)

| Key | Unit | Meaning |
| --- | --- | --- |
| `conductance` | W/K | wall + window + other transmission and ventilation conductance |
| `heating_losses` | kWh/a | `conductance × heating_degree_hours / 1000` |
| `heating_gains` | kWh/a | `solar_winter + internal_winter` |
| `heating_eta` | 1 | utilization factor η of the gains (EN ISO 13790 monthly method) |
| `heating_used` | kWh/a | `heating_eta × heating_gains` |
| `heating_demand` (existing) | kWh/a | net demand `max(0, heating_losses − heating_used)` |
| `cop` | 1 | heat pump COP (`heating = heating_demand / cop`) |
| `cooling_gains` | kWh/a | `solar_summer + internal_summer` |
| `cooling_removable` | kWh/a | `conductance × cooling_degree_hours / 1000` |
| `cooling_eta` | 1 | utilization factor of the removable heat |
| `cooling_removed` | kWh/a | `cooling_eta × cooling_removable` |
| `cooling_demand` (existing) | kWh/a | net cooling load `max(0, cooling_gains − cooling_removed)` |
| `eer` | 1 | chiller EER (`cooling = cooling_demand / eer`) |
| `daylight` (existing) | % | daylight factor |
| `autonomy` (existing) | 1 | daylight autonomy |
| `lamps_on` | 1 | `1 − autonomy` |
| `lighting_power` | kW | installed power `power_density × floor / 1000` (0.432 kW) |
| `lighting_hours` | h/a | hours the room needs light (1,500) |
| `internal_winter`, `internal_summer` | kWh/a | internal gains of the two seasons |
| `beam_winter`, `beam_summer` | 1 | delivered beam weight B |
| `direct_winter`, `direct_summer` | 1 | share of the irradiance that arrives direct (`reach`) |

Baseline (180°, 45 %, 0.6 m, 16 cm): conductance 24.26 W/K; heating 1,698.4 losses, 2,136.0 gains (1,836.0 solar + 300), η 0.700, 1,494.5 used, demand 203.9, COP 3.5, 58.3 electricity; cooling 929.4 gains (429.4 + 500), 194.1 removable, η 0.998, 193.8 removed, load 735.6, EER 3, 245.2 electricity; lighting 0.432 kW × 1,500 h/a × 33.7 % lamps on = 218.6 (daylight factor 2.7 %, autonomy 66.3 %).

Tests: `StudyModel.test_energy_balance_terms_close_each_demand_and_each_conversion` (60 random designs), the existing NumPy comparison (300 random designs, rtol 1e-9) now also compares every new numeric term; baseline expectations in `cases.baseline`.

## 5. Test results

- `python 🧪️tests/🐍️.py` mathematics classes (65 tests): OK in 6 s. Full run (`SEMIO_TEACHING_OUTPUT_ROOT=T/🗑️generated/output`, contract tests included, scenes as the other agents had them at run time): `Ran 70 tests in 932.620s`, OK.
- `node 🧪️tests/🟨️.cjs`: "Optimization schema: authored model accepted; 14 invalid models rejected."
