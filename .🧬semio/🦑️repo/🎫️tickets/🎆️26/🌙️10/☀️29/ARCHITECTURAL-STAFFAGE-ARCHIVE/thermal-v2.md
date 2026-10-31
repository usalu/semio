# Thermal v2 (thermal worker)

S = `🎓️teaching/🏛️architecture/🧍️staffage`, T = this ticket folder. Module `S/🔨️modules/🌡️thermal` (`🔬️physics`, `🧱️balance`, `🫀️physiology`, `🍃️canopy`, `⏳️transient`, `🌤️environment`, `📏️quantities`, `🕰️timeline`),
view `S/🔨️modules/🎨️depiction/🌡️thermal-view`, environment presets `S/🗂️catalog/🌤️environments/🔣️.json`. Replaces [thermal-model.md](thermal-model.md) (v1) and the plume code (now `💨️transport`, see [transport-v2.md](transport-v2.md)).
**Illustrative engineering model: lumped regions, no CFD, no conduction between regions, no condensation except dew on leaves, not a clinical or building-physics prediction.** Nothing is authored as a temperature
(except `model: "fixed"` in reference fixtures); the solver is independent of the drawing scale (a test proves bit-identical results for 1:200, 1:100, 1:50 and no selector).

## API (for website, export and core)

```ts
import { solveThermal, solveThermalAt, radiationSplit, operativeTemperatureC, areaWeighted, uValue, simulateBody, penmanMonteith,
         QUANTITY_SPECS, environmentAt, historyPlan, relax, type SolvedThermalField, type ThermalOptions, type PassengerInputs } from "S/🔨️modules/🌡️thermal/🟦️.ts";

solveThermal(state: EvaluatedState, environment?: Environment | EnvironmentTimeline, options?: { history?: (t) => {inputs, comVelocity?}; windowS?; maxSteps?; signal?; progress? }): SolvedThermalField
solveThermalAt(asset: Asset, query: EvaluationQuery, environment?, options?): SolvedThermalField   // full history from the asset's own timeline (clips, transitions)
```

- `SolvedThermalField = ThermalField & { passengers: Record<Slug, BodyField> }`; `field.body` is the asset's own body, `field.passengers[id]` every passenger (ruling 4).
- **Use `solveThermalAt` whenever the query is a clip or transition** (animated playback, export of clips): it evaluates the asset at every integration time (no geometry), so playing and scrubbing give the same temperatures
  (pure function of the absolute time). `solveThermal(state, env)` without `history` returns the steady state of the state's inputs (correct for named poses).
- Time-varying environment: pass an `EnvironmentTimeline` (also to `depict` via `options.environmentTimeline`); the response is then transient even for a pose.
- `QUANTITY_SPECS[quantity]`: title (en/de), unit, digits, `pick(regionField)`, `centre(env)`, `shared` comparison range, meaning (en/de). Region value of any quantity: `spec.pick(field.regions[id])` (`skin-temperature` is `null` for regions without skin: fall back to the surface temperature, as the view does).
- Merged small-scale forms: `areaWeighted(asset, shape, valueOf)` of `🧩️shapes` (area weights) — the thermal view uses it; `areaWeighted(regions, field, ids, quantity)` of the thermal module additionally weights by `exposedFraction`.
- Net radiation split: `radiationSplit(asset, field, regionId) → { sky, ground, surroundings }` W/m² (+ = the region loses heat); sums exactly to `netRadiativeWM2`.
- `solveThermal` reads only the asset, `state.thermal`, `state.timeS`, `state.kinematics.comVelocity` (and the environment): never the representation.

### Requests to the coordinator / other owners (cross-owner interface)

| # | Request | Why |
|---|---|---|
| R1 | `ThermalInputs.passengers?: Readonly<Record<Slug, { activityMet: number; breathing?: Slug \| null }>>` in `🔗️contracts` and filled by `🧮️evaluation` from `ThermalBinding.passengers` (blend like `activityMet`, `breathing` step) | Ruling 4. The solver already reads it through the structural type `PassengerInputs`; today the evaluator drops it, so passengers rest at 1 met and no breathing pattern |
| R2 | `ThermalField.passengers: Readonly<Record<Slug, BodyField>>` in the contract (solver already returns it as `SolvedThermalField`) | typed access for website/export |
| R3 | `RegionField` additions: `netRadiativeSkyWM2`, `netRadiativeGroundWM2`, `netRadiativeSurroundingsWM2`, `contactWM2` (optional; today `radiationSplit()` computes the split on demand) | export tables, inspector |
| R4 | `depict()`/`depictAll()`/preview CLI/check/React pipeline: call `solveThermalAt(asset, state.query, options.environmentTimeline ?? options.environment)` instead of `solveThermal(state, options.environment)` for clip and transition queries (and pass `environmentTimeline`) | transient response of animated clips; `depict` currently solves the steady state of the instant |
| R5 | `legendBlock` (🏷️annotations) prints only `convention` and `interpretation`. The thermal legend puts conditions and assumptions into `interpretation` as well, but `Legend.conditions` / `Legend.assumptions` should be drawn as their own lines when present (export/website legends) | brief: every visualisation names conditions and assumptions |
| R6 | The thermal view paints open strokes with a region (ruling 2) as a halo (≈ 2.6 × pen, ≥ 0.3 mm) of the region colour in layer `field`, *under* the ink line of layer `body`. If a strict "colour at the stroke pen" is wanted, `drawShapes` must skip the ink outline of such strokes in analytic views | ink drawn on top would hide a stroke of equal pen |
| R7 | `🏗️structure` harness (`🧰️harness`) builds `ThermalInputs` without `externalWorkW`/`breathing` (tsc error); `structuralLegend` lacks `range` (tsc error) | owner of structure |
| R8 | `💨️transport/🟦️.ts` has duplicate identifier `query` at lines 80/90 (tsc error TS2300) | transport worker |
| R9 | Package barrel `📦️packages/🟦️typescript/🟦️.ts` line 13 still exports the removed `thermalColour`, `thermalLegend` (tsc error). Replace by `export { solveThermal, solveThermalAt, radiationSplit, operativeTemperatureC, areaWeighted as areaWeightedThermal, uValue, simulateBody, penmanMonteith, QUANTITY_SPECS, environmentAt } from "../../🔨️modules/🌡️thermal/🟦️.ts";` and export the environment preset loader/type if the website needs the presets | public API |
| status | R4, R5 delegated by the coordinator to the core worker (the legend renderer already prints `conditions` and `assumptions`, so the thermal `interpretation` carries only the quantity meaning); R6 decided by the coordinator: the halo stays under the ink line, documented in the legend assumptions; R7–R9 open |
| (done by coordinator) | R1–R3 contract fields (`passengers`, region radiation split, `contactWM2`) are filled by the solver; `SolvedThermalField` alias removed | |

## Quantities (what the view shows; computed before any colour)

Fluxes per m² of the region's exposed outer area, **positive = leaving the surface**. Source of truth: `📏️quantities` (`QUANTITY_SPECS`).

| Quantity | Unit | Definition | Shared comparison range (centre) |
|---|---|---|---|
| `surface-temperature` | °C | outermost surface (clothing, fur, plumage, bark, leaf, paint) | −20 … 60 (air temperature) |
| `skin-temperature` | °C | skin beneath clothing/fur/feathers (bodies); regions without skin show the surface temperature | 10 … 40 (air temperature) |
| `emitted-radiant-flux` | W/m² | `ε σ T_s⁴`, what a thermal camera sees, independent of what is received | 230 … 700 (σ T_air⁴) |
| `net-radiative-flux` | W/m² | longwave exchange with sky, ground and surroundings `Σ_k ε σ F_k (T_s⁴ − T_k⁴)`, **+ = loss**; absorbed sunlight is *not* included; split by `radiationSplit()` | −200 … 200 (0) |
| `sensible-heat-flux` | W/m² | convection to the air `h_c (T_s − T_air)`, + = air is heated | −200 … 200 (0) |
| `latent-heat-flux` | W/m² | evaporation (diffusion + sweat through clothing, panting tissue, transpiration); + = heat leaves with vapour; negative = dew on leaves | −300 … 300 (0) |
| `moisture-flux` | g/(m² h) | `latent / λ(T)`: vapour released per area and hour (1 g/(m² h) = 6.9·10⁻⁷ kg/(m² s)) | −450 … 450 (0) |

`range: shared` = these documented ranges (identical for every asset, scale and time); `fixed` = user range; `auto` = symmetric span of the values present, **the legend says "AUTO … normalised … not comparable between figures"** (en/de). The legend names quantity, unit, numerical range, centre, sign convention, environmental conditions
(air, mean radiant, sky, ground, humidity, wind speed and direction, sun and diffuse share, pressure, time) and the model note (steady or history) and the generic assumptions. Strong deviations (> 35 % of the half range) add a colour-independent hatch (`patterns`).

## Model

### Surface balance of one region (🧱️balance)
Per outer area, exposed share `e`, contact share `c` (seat, ground, handle; conductance 20 W/(m² K), illustrative):
`q_in(T_s) + e·q_sol = e (q_rad + q_conv + q_lat) + c h_ct (T_s − T_contact)`; steady state by an Illinois regula-falsi root of this strictly monotone residual.

| Term | Equation | Class |
|---|---|---|
| Radiation | `q_rad = faces ε σ (T_s⁴ − T_env⁴)`, `T_env⁴ = F_sky T_sky⁴ + F_gnd T_gnd⁴ + F_sur T_mrt⁴`; `F_sky` = region `skyViewFactor`, ground share of the rest 1 (down-facing plates), 0 (up-facing), ½ (vertical plates, cylinders, spheres); secant coefficient `h_r = q_rad/(T_s − T_env)` (tangent `4εσT³` at equality). Full σT⁴, no linearisation in the solver | σ CODATA 2018: measured; ground shares: estimated |
| Convection | by geometry, mixed `h = (h_nat³ + h_forced³)^⅓` (Churchill 1977): natural — Churchill–Chu (vertical plate, horizontal cylinder), Churchill (sphere), McAdams/Lloyd–Moran (horizontal plates); forced — laminar/turbulent flat plate, Churchill–Bernstein (cylinder), Whitaker (sphere); air properties: Sutherland viscosity and a conductivity fit within 1 % of tables; speed = relative air speed (wind vector − figure COM velocity; bodies: `√(v_rel² + (0.0052 (M − 58))²)` after ISO 7730 `v_ar`) | literature (Incropera & DeWitt; Churchill & Chu 1975; Churchill & Bernstein 1977; Whitaker 1972) |
| Absorbed solar | `α G f_geo (s + d (1 − s))`, `s` sunlit fraction, `d` diffuse fraction; `f_geo`: sphere ¼, cylinder 1/π (projected area ratios, geometric), vertical plate ½, up plate 0.7 (mean over sun elevations), down plate 0.1 (reflected light) | geometric / estimated |
| Evaporation (wet surfaces) | `w · 16.5 h_c/1000 · (p_s(T_s) − p_a)` W/m² (Lewis relation, pressure corrected) | literature (ISO 13731, ASHRAE) |
| Conditioned element | `q_in = (T_interior − T_s)/(ΣR_layers + R_film)` | per asset |
| Operating surface | `q_in = heatW · level / area`; `level` from the pose/clip operating binding | per asset |
| Passive | `q_in = 0` | — |
| Psychrometrics | Buck (1996) Magnus form `p_s = 611.21 exp((18.678 − T/234.5) T/(257.14 + T))` (within 0.05 % of IAPWS-95 over 0 … 60 °C), `q = 0.622 e/(p − 0.378 e)`, `λ = 2.501·10⁶ − 2361 T` | literature |

### Living bodies (🫀️physiology) — Gagge-type two-node model over regions
State: one core temperature and one skin temperature per `body` region (`regions[].body.skinFraction` × `skinAreaM2` = skin area; `areaM2` outer area, `f = area/skin area`). Controls follow Gagge, Stolwijk & Nishi (1971) as implemented in pythermalcomfort `two_nodes_gagge`:
`α = 0.1`, `T_b = α T_sk + (1 − α) T_cr`; set points from `BodyPhysiology`; skin blood flow `m_bl = (6.3 + 120 WARM_cr)/(1 + 0.5 κ COLD_sk)` clamped to `[min, max]` L/(m² h); regulatory sweating `m_rsw = 170 WARM_b exp(WARM_sk/10.7)` ≤ 500 g/(m² h)
(skipped when `sweating: false`), `E_rsw = 0.68 m_rsw`, wettedness `w = 0.06 + 0.94 E_rsw/E_max` ≤ `w_max` (0.38 v^−0.29 bare, 0.59 v^−0.08 clothed); shivering `19.4 COLD_sk COLD_cr` W/m²; panting animals add ventilation `min(7 m, 3 m WARM_cr)` L/min (estimated).
Heat path: core → skin `(5.28 t_f + 1.163 m_bl)(T_cr − T_sk)` (tissue factor `t_f` per segment); skin → **outer surface through the layers** (`R` referred to the outer area: `I_cl · f_cl`, ruling 5) → environment by the surface balance above (`f` converts per outer to per skin area);
evaporation from the skin through the vapour resistance: `E_max = f e Δp / (R_v + 1/h_e)`. The **skin temperature under clothing, fur or feathers and the outer surface temperature are distinct fields** (`skinTempC`, `surfaceTempC`).
5 % of the heat production is released locally in the regions by `metabolicShare` (estimated); the rest enters the core. Respiratory exchange: `ṁ = ρ V̇`, `V̇ = rate × tidal volume` of the bound breathing pattern (the same numbers drive the breath emitters and the chest motion of 💨️transport) plus panting; sensible `ṁ c_p (T_ex − T_a)`, latent `ṁ (w_ex − w_a) λ` with the authored exhaled state (default fallback: ASHRAE `0.0014 M (34 − t_a)`, `0.0173 M (5.87 − p_a)`).
Core heat capacity `max(½, 1 − skin share) · 3490 J/(kg K) · mass`; skin node capacity = region `heatCapacityJM2K × areaM2`. Passengers (ruling 4) are solved as independent bodies (regions with `body.subject`).
Parameter classes: set points, 6.3/120/0.5/170/10.7/0.68/0.06/0.94/19.4/5.28/1.163, `w_max`, tissue heat capacity: **literature (secondary: pythermalcomfort, ASHRAE 55)**; segment factors `t_f, κ, sweat share`, local share 5 %, panting gains, `PANT_MAX`: **authored-estimate** (monotone, qualitative); `metWM2`, `skinAreaM2`, layer resistances: per asset class (see assets).

### Leaves (🍃️canopy, Penman–Monteith with stomatal control)
`λE = λ ρ g_v (q_s(T_l) − q_a)`, `g_v = g_s g_b/(g_s + g_b)`, `g_b = h/(ρ c_p) (Pr/Sc)^⅔` (Pr 0.71, Sc 0.60), solved jointly with absorbed sunlight `α G/2 (s + d (1 − s))` (spherical leaf-angle mean projection ½), two-faced longwave exchange and two-faced convection for `T_l`.
Where `q_s(T_l) < q_a` (leaf colder than the dew point) dew forms through the boundary layer only (`g_v = g_b`, negative latent and moisture flux). `areaM2` of a leaf region is its **one-sided leaf area**; `perGroundArea(leaf, flux)` converts with the leaf area index.
Stomata (Jarvis 1976 form, **illustrative parameters, not fitted to a species**): C3/C4 `g_s = g_max max(0.02, f_light f_vpd)`, `f_light = Q/(Q + 100)(1 + 100/500)` ≤ 1 (`Q` = incident shortwave W/m²), `f_vpd = (1 + 0.5/D₀)/(1 + D/D₀)` ≤ 1 (`D₀` 2 kPa C3, 3 kPa C4); CAM opens at night and closes by day `g_max (0.05 + 0.95 (1 − f_light))`.
`penmanMonteith()` exposes the combination equation `λE = (Δ(R_n − G) + ρ c_p D/r_a)/(Δ + γ(1 + r_s/r_a))` (FAO-56 eq. 3).
Transpiration is **vapour transport + latent heat**, distinct from the sensible convection of the same leaf; the transpiration emitter of 💨️transport reads `region.latentW`.

### Machines, building elements, passive objects
Operating regions release `heatW × level` into their surface (engine block, exhaust, motor, lamp, heater); conditioned regions conduct from `interiorTempC` through their layers (`uValue()` = `1/(ΣR + R_film + 1/25)`); passive regions have `q_in = 0`. Inactive machines (level 0) relax to the passive balance of their materials (tested: tyres within 20 K of the air).

### Transient (⏳️transient, 🕰️timeline)
Per non-body region `C dT/dt = q_in − q_out(T) + e q_sol` (`C` = `heatCapacityJM2K`); **exponential Euler step** with the restoring conductance `G = −∂(…)/∂T` at the current temperature: `T ← T + (r/G)(1 − exp(−G Δt/C))`, `τ = C/G` (reported per region as `timeConstantS`). Exact for constant forcing, unconditionally stable, monotone: no overshoot, no oscillation.
Body: skin nodes relax exponentially with `τ = C_skin/G`, the core takes an explicit step with its thermal mass (steps ≤ 64 s). The history starts `min(7 τ_max, 3 h)` before `t` (not before 0) in the **steady state of the inputs at the start** (at `t = 0` this is the stated assumption: the figure has been in that state forever);
steps lie on an **absolute grid** of `0.5 s · 2^k` (the smallest k with ≤ 180 steps), forcing sampled at the step midpoint, final partial step to `t`. The result is a pure function of `(asset, query, environment, t)`: scrubbing = playing (tested bit-identically). Time scales: leaves 10–60 s, thin panels 1–10 min, clothed skin 5–30 min, car body 10–20 min, tyres/brakes/engine blocks 0.5–several h, core 1–3 h.

### Environments (`S/🗂️catalog/🌤️environments/🔣️.json`)
Ten shared presets (office-neutral, indoor-heated-winter, winter-overcast, winter-clear-night, spring-mild-sun, summer-hot-sun, summer-heatwave-calm, summer-humid, summer-night, windy-cool) with en/de descriptions; clear-sky temperatures follow Swinbank (1963) `T_sky = 0.0552 T_air^1.5` (reference `swinbank-1963`); all other values `authored-estimate` / `illustrative` (no meteorological dataset claimed).

## Limitations and non-claims
- Lumped regions: no conduction between neighbouring regions, no spatial gradient inside a region, no clothing heat capacity separate from the skin node (clothing outer temperature is quasi-steady), no sweat dynamics (sweat accumulation/drying) beyond two-node wettedness, no clothing wetting/condensation in garments, no hair/feather ptiloerection, no behavioural regulation.
- Bodies in sustained high activity beyond the evaporative capacity do not reach a steady state: the solver reports the unbounded steady state (see pigeon flight, table of defects), the transient (`solveThermalAt`) shows the real time course. There is no thermal death or behaviour cut-off.
- Convection correlations are textbook mean values for isolated simple bodies; no mutual shielding, no wake, no stratification; whole-body convection coefficients agree with Gagge's 3 W/(m² K) only within ~10 % at rest and are ~25 % below Gagge's activity law at 2 met (ISO 7730 `v_ar` is used instead).
- Longwave: grey surfaces, view factors reduced to `skyViewFactor` plus a fixed ground share; no inter-reflection, no atmospheric window model (sky temperature is an input); shortwave: one absorptivity, mean geometry factors, no spectral/glazing transmission, no reflected ground light except the down-plate factor.
- Latent: dew only on leaves; wet passive surfaces (`wettedness` binding) evaporate but never condense; no rain, snow melt or water films.
- Leaf parameters (Jarvis constants, `g_max`) are illustrative, not fitted per species; the canopy is a set of leaf regions with authored sun/shade split, no canopy radiative transfer.
- Operating heat release is applied at the surface for the stated level; a steady pose with a transient heat source (brakes at level 1 is a 36 kW *peak*) gives an absurd asymptote; use clips and `solveThermalAt` for such assets.
- Not provided: PMV/PPD, SET, UTCI (only the quantities above; `operativeTemperatureC` per region); no thermal comfort claims.
- Conditions outside −30 … 60 °C, 80 … 105 kPa, and Re > 10⁶ are unchecked.

## Sources and parameter classes (only checkable references)

| Parameter / law | Class | Source |
|---|---|---|
| Stefan–Boltzmann σ = 5.670374419·10⁻⁸ W/(m² K⁴) | measured (CODATA 2018) | https://physics.nist.gov/cgi-bin/cuu/Value?sigma |
| Saturation pressure (Magnus form with Buck coefficients) | literature | Buck 1996, Buck Research Manual; checked against IAPWS-95 (CoolProp) and psychrolib (Hyland–Wexler) |
| Natural convection Churchill–Chu, Churchill, McAdams; forced plate, Churchill–Bernstein, Whitaker; blend exponent 3 | literature | Incropera & DeWitt, Fundamentals of Heat and Mass Transfer (ch. 6–9); Churchill & Chu 1975, Int. J. Heat Mass Transfer 18:1049; Churchill & Bernstein 1977, J. Heat Transfer 99:300; Whitaker 1972, AIChE J. 18:361; Churchill 1977 (mixed convection) |
| Lewis relation 16.5 K/kPa | literature | ISO 13731 terminology, ASHRAE Fundamentals ch. 9 |
| Two-node controller constants (α 0.1, 6.3, 120, 0.5, 170, 10.7, 0.68, 0.06/0.94, 19.4, 5.28, 1.163, `w_max`) | literature (secondary: reproduced by pythermalcomfort) | Gagge, Stolwijk & Nishi 1971, ASHRAE Trans. 77(1):247; pythermalcomfort `two_nodes_gagge` |
| Tissue heat capacity 3490 J/(kg K) | literature | pythermalcomfort (0.97 Wh/(kg K)) |
| Clo conversion 0.155 m² K/W; permeation efficiency 0.38 (assets) / 0.45 (Gagge fixture) | literature | ISO 9920; pythermalcomfort |
| Relative air speed `0.0052 (M − 58)` | literature | ISO 7730:2005 |
| Respiratory heat (fallback) | literature | ASHRAE Fundamentals ch. 9 (quoted); main path from ventilation × psychrometrics |
| DuBois area (assets) | literature | DuBois & DuBois 1916 |
| Penman–Monteith combination form, FAO-56 constants | literature | Allen et al. 1998, FAO-56 ch. 3 and example 18 |
| Leaf boundary-layer conductance ratio (Pr/Sc)^⅔, Pr 0.71, Sc 0.60 | literature | Monteith & Unsworth, Principles of Environmental Physics; Campbell & Norman 1998 |
| Mean leaf projection ½ (spherical leaf angles) | literature | Campbell & Norman 1998 |
| Jarvis stomatal form; `Q₅₀` 100 W/m², `Q_ref` 500 W/m², `D_ref` 0.5 kPa, `D₀` 2/3 kPa, `g_min` 0.02, CAM night opening | form: literature (Jarvis 1976, Phil. Trans. R. Soc. B 273:593); values: **illustrative** | — |
| Clear-sky temperature | literature | Swinbank 1963, Q. J. R. Meteorol. Soc. 89:339 |
| Segment tissue/κ/sweat factors, 5 % local heat, panting gains, ground share of the non-sky hemisphere, `f_geo` of plates, contact conductance 20 W/(m² K) | **authored-estimate / illustrative** | — |
| Asset parameters (layers, areas, capacities, `g_max`, heat release) | as tagged in each asset (`source`) | see assets |

## Validation (third-party oracles, `S/🧪️tests/<case>/🐍️.py` + `S/🧫️fixtures/<case>/🔣️.json` + `🟦️.ts` + `🥒️.feature`; run all with `T/🧰️scripts/thermal-oracles.sh [--write]`)
Observed maximum deviations are measured by `T/🧰️scripts/thermal/validation-report.ts`.

| Case | Oracle (test-only dependencies) | Quantities compared | Tolerance in the test | Observed |
|---|---|---|---|---|
| 🌡️blackbody-exchange | scipy.constants (CODATA), numpy | emitted εσT⁴, net exchange, sky/ground/surroundings split, secant coefficient, power | 0.2 % + 0.01 W/m² | < 10⁻⁹ relative |
| 💧️vapour-pressure | CoolProp IAPWS-95 `PropsSI`, `HAPropsSI`; psychrolib cross-check | saturation pressure, specific humidity | 0.2 %, 0.6 % (psychrolib 0.3 %) | max 0.05 % (saturation) |
| 🫀️two-node-gagge | pythermalcomfort `two_nodes_gagge`, 7 conditions, 60 min from neutral | core, mean skin, wettedness, skin blood flow, dry loss, evaporation, respiratory latent | 0.15 K, 1 K, 0.05, 0.2·x + 2 L/(m²h), 6 W/m², 3 W/m², 1 W/m² | core 0.04 K, skin 0.96 K, wettedness 0.022, blood flow 4.6 L/(m²h), dry loss 3.8 W/m², evaporation 1.1 W/m², respiratory 0.66 W/m² |
| 🍃️leaf-transpiration | scipy `brentq` + CoolProp air and water properties (second implementation of the same equations); closed-form Penman–Monteith of the linearised balance | leaf temperature, radiative, sensible, latent, moisture flux; PM latent | 0.2 K, 3 % + 2 W/m²; PM 15 % + 3 W/m² (linearisation error) | T 0.017 K, fluxes ≤ 0.2 W/m²; PM linearised up to 23 % (small fluxes) – within 15 % + 3 W/m² |
| 🍃️penman-monteith | pyet `pm_fao56`; FAO-56 example 18 published 3.88 mm/day | evapotranspiration of 3 daily cases | 4 % (pyet), 3 % (published) | pyet reproduces 3.879; ours within tolerance (see test) |
| ⏳️thermal-transient | scipy `solve_ivp` (LSODA, rtol 10⁻¹⁰) of the nonlinear lumped balance, CoolProp air | step response of a sunlit plate and an internally heated plate at 7 times to 1 h | 0.25 K | 0.040 K |

Further tests (physical/analytic oracles): 🧊️thermal-scenarios (directions and magnitudes, below), ⏳️thermal-transient (monotone response, 63 % at τ, scrub = play bit-identical, periodic clip response, slow core vs fast skin, scale invariance bit-identical),
🎨️thermal-legend (values before colour, area-weighted merged regions, shared/auto ranges, bilingual conditions and assumptions), 🌡️catalog-physics (every pose and clip instant of every asset finite, species core temperatures, passive bounds).
Known and documented deviations from the Gagge oracle: ours uses geometry-based convection (cylinder, 0.2 m) instead of the constant 3 W/(m² K) and the ISO 7730 `v_ar` instead of `5.66 (M − 0.85)^0.39`, hence larger differences at 2 met and in wind; 5 % local heat production.
The Penman–Monteith closed form assumes stomata in series with the boundary layer and linearises radiation about the air temperature: it does not apply where dew forms (case skipped, `null`).

### Scenario sensitivity (tests, walking-commuter / labrador-dog / city-car / english-oak / sliding-door)
- cold (2 °C, 3 m/s) vs mild office: skin under the coat > 1 K colder, coat surface > 10 K colder; hot sunny day: wettedness +0.05, skin blood flow > 2× cold, latent power > 3× cold.
- rest vs walking (3 met at 18 °C): metabolic heat ×2.3+, core and skin blood flow higher, coat surface warmer, sensible + latent power ×1.5; in −10 °C wind the resting body **shivers** (metabolic heat > activity value, core below set point).
- sun vs shade: absorbed power +50 W/m², coat surface > 2 K warmer; dark car roof > 5 K warmer in sun.
- wind 8 vs 0.2 m/s: coat surface > 2 K colder, convection coefficient > 2×, hands colder.
- clothing ×2 insulation: skin warmer, outer surface colder, sensible flux smaller; bare hands colder than skin under the coat; coat surface > 5 K below the skin beneath.
- dog: skin under fur > 5 K above the fur surface in the cold; hot day raises core and respiratory latent power.
- car engine load 0 → 1: exhaust system > 80 K hotter (and < 600 °C), engine bay > 15 K hotter, bonnet hotter; inactive tyres within 20 K of the air.
- oak: sun leaves transpire more than shade leaves, leaves cool below the air on a clear night (net radiation > 20 W/m²), bark has no latent flux.
- conditioned door glass lies between exterior (0 °C) and interior (20 °C) and conducts > 20 W/m².
- passengers: a toddler body in a parent fixture is solved independently (own core ≈ 37 °C) and leaves the parent's result unchanged to 9 digits.

## Asset data defects for the content workers (found by the sweep; assets not edited)

| Asset | Region / pose | Finding | Suggested fix |
|---|---|---|---|
| city-car, city-bus, city-bicycle (hub-brakes), forklift | `brakes`, poses with `braking` = 1 | `heatW` at level 1 is a **peak** braking power (city-car 36 kW on 0.22 m²): the steady state of a pose is 1000 – 1550 °C (forklift 660 °C) | named poses should bind `braking` ≤ 0.05; peak braking only in clips (the transient solver then shows tens of kelvin per second, e.g. 5 K/s for 31 kJ/(m² K)); or document `heatW` as sustained power |
| city-car | `engine-bay` (exposedFraction 0.2, 1100 W at level 1) | steady surface 85 – 174 °C, net radiation 1470 W/m²: the 80 % covered area is adiabatic, all heat leaves through 0.22 m² | exposed fraction ≥ 0.5 or `heatW` = share that leaves through the visible surface (≈ 20 – 40 % of the shaft power) |
| concrete-mixer-truck | `engine-bay`, `drum-drive` | 118 – 151 °C | as above |
| excavator | `engine-compartment` | 107 – 127 °C at rated load | as above (plausible upper end ≈ 100 °C) |
| tower-crane | `hoist-motor` | 95 – 114 °C | motor casings are typically 60 – 90 °C at rated load: check `heatW`/area |
| pigeon | poses `takeoff-liftoff`, `airborne-wings-up`, `airborne-wings-down` (`activityMet` 15) | the **steady state** of sustained flight has no airspeed in a pose query and the heat cannot be shed: core 85 – 86 °C (clip instants are fine: transient core 41.9 °C, convection from the activity speed) | bind a sustained-flight value (≈ 8 – 10 met) in airborne poses or accept that poses of transient activities are not steady states; flight clips need root advance for airspeed |
| heat-pump-outdoor-unit | `coil-guard` (`heatW` −1800 at `refrigerant-duty` 1 on 1.6 m²) | surface −75 °C in clip instants: −1125 W/m² of heat extraction through the *visible* area; a real evaporator has ~10–20× extended (fin) surface in forced fan flow | `areaM2` = effective heat-transfer (fin) area or a much smaller `heatW`; the extended surface cannot be modelled by the lumped region |
| dining-table, single-bed (fixed in the solver) | regions with `exposedFraction` 0 (stowed leaf, slide rails, covered slats) | produced NaN before the solver floor `ENCLOSED_EXPOSURE` 0.05 was introduced; no content change needed | none |
| mallard-duck, pigeon | core 41.4 – 41.9 °C | normal for birds (41 – 42 °C) | none |
| domestic-cat, labrador-dog, roe-deer, jogger, leaning-person, running-child | `pads`, `hooves`, `hand-skin`, `arm-skin`, `socks` skin 23.9 – 25 °C | below 25 °C at ≈ 20 °C air: plausible for extremities in cool/windy conditions or on cold ground, not a defect | none; check the `contact` temperature bindings (cold ground) if unintended |
| delivery-van, motor-scooter, electric-kick-scooter, tram, wheel-loader | `brakes`/`brake-disc(s)` at braking 1 (steady 420 – 1400 °C), `engine`/`engine-bay` (net radiation 1000 – 2700 W/m²), `bms-controller` 173 °C, `exhaust-stack` 2945 W/m² | same two causes as above: peak powers bound to steady poses, and small exposed fractions that concentrate released heat | as above; a battery controller at 173 °C needs a smaller `heatW` or a larger area |
| charcoal-grill, patio-heater | `bowl-shell` 105 °C, `emitter-head` 486 °C | plausible for a lit grill and a radiant heater | none |
| runner-with-blade-prosthesis, pig | `hand-skin`/`arm-skin` 21.6 – 24.4 °C, `tail` 24.5 °C | cool extremities, plausible in the reference environment | none |
| panel-radiator | `panel-front` net radiation 415 W/m² | consistent with a 75 °C panel | none |

Assets whose regions are all passive/operating/conditioned show `core` "–" in the table: no body expected (furniture, components, vegetation, machinery, objects).

## Solve of every catalog asset (all named poses at steady state in the asset's reference environment + one instant at 37 % of the first clip through `solveThermalAt`)
Script `T/🧰️scripts/thermal/sweep.ts` (table by `sweep-table.py`): **103 assets, 500 queries, 500 solved, 0 non-finite values, max 37 ms per solve**. The catalog already contains the new assets (more than the 48 converted ones). Ranges are min … max over all regions and queries of an asset;
"flags" are the automatic suspicion rules (skin < 25 °C at air ≥ 18 °C, surface > 600 °C or far from the air, negative latent flux outside leaves, |net radiation| > 400 W/m², leaf > air + 25 K, core outside 35.8 … 41.5 °C). Real defects are listed in the defect table above; the flags of extremities and of hot machine parts are discussed there.
Regenerate after content changes: `repo tools exec bun -- T/🧰️scripts/thermal/sweep.ts <out.json>` from S.

| asset | cat. | solves | surface °C | skin °C | emitted W/m² | net rad. W/m² | sensible W/m² | latent W/m² | moisture g/(m²h) | core °C | flags |
|---|---|---|---|---|---|---|---|---|---|---|---|
| apple-tree | vegetation | ✓ 5/5 | 18.1 … 29.1 | – | 388 … 450 | -44 … 18 | -113 … 88 | 0 … 182 | 0 … 266 | – | – |
| bar-stool | furniture | ✓ 5/5 | 21.4 … 29.5 | – | 43 … 428 | 0 … 46 | 1 … 33 | 0 … 0 | 0 … 0 | – | – |
| bicycle-stand | objects | ✓ 5/5 | 22.8 … 25.7 | – | 122 … 127 | 2 … 9 | 15 … 70 | 0 … 0 | 0 … 0 | – | – |
| bookshelf | furniture | ✓ 4/4 | 21.1 … 23.1 | – | 382 … 393 | 0 … 11 | 0 … 6 | 0 … 0 | 0 … 0 | – | – |
| cafe-chair | furniture | ✓ 5/5 | 24.2 … 36.9 | – | 399 … 472 | 22 … 95 | 61 … 115 | 0 … 0 | 0 … 0 | – | – |
| cantilever-balcony | components | ✓ 4/4 | 19.5 … 31.9 | – | 360 … 442 | 10 … 120 | -4 … 90 | 0 … 0 | 0 … 0 | – | – |
| cargo-bike | vehicles | ✓ 4/4 | 20.6 … 29.5 | – | 171 … 438 | 8 … 71 | 12 … 67 | 0 … 0 | 0 … 0 | – | – |
| charcoal-grill | objects | ✓ 5/5 | 19.5 … 567.6 | – | 63 … 26922 | 1 … 26535 | -6 … 5764 | 0 … 0 | 0 … 0 | – | bowl-shell net rad 2707; bowl-shell net rad 2911; bowl-shell net rad 690 (+15) |
| child-on-kick-scooter | humans | ✓ 4/4 | 21.3 … 36.1 | 27.2 … 37.3 | 128 … 494 | 11 … 138 | 34 … 319 | 0 … 219 | 0 … 325 | 37.12 … 37.80 | – |
| city-bicycle | vehicles | ✓ 5/5 | 20.4 … 993.2 | – | 42 … 14584 | 1 … 14543 | 6 … 10471 | 0 … 0 | 0 … 0 | – | hub-brakes net rad 14543; hub-brakes surface 993 C |
| city-bus | vehicles | ✓ 5/5 | 21.3 … 1546.1 | – | 286 … 403728 | 10 … 403451 | 4 … 13243 | 0 … 0 | 0 … 0 | – | brakes net rad 115942; brakes net rad 311553; brakes net rad 403451 (+7) |
| city-car | vehicles | ✓ 6/6 | 19.2 … 1435.8 | – | 285 … 314406 | 4 … 314130 | -7 … 13171 | 0 … 0 | 0 … 0 | – | brakes net rad 314130; brakes surface 1436 C; engine-bay net rad 1470 (+6) |
| concrete-mixer-truck | machinery | ✓ 5/5 | 19.9 … 188.7 | – | 129 … 2064 | 13 … 1766 | -0 … 1434 | 0 … 79 | 0 … 116 | – | drum-drive net rad 1196; drum-drive net rad 814; drum-drive surface 117.8 vs air 20 (+7) |
| courier-with-hand-truck | humans | ✓ 5/5 | 4.9 … 23.9 | 11.2 … 28.9 | 291 … 433 | 6 … 121 | -16 … 308 | 0 … 37 | 0 … 54 | 36.66 … 36.94 | – |
| curtain-wall-bay | components | ✓ 5/5 | 0.8 … 8.7 | – | 267 … 301 | 23 … 55 | 5 … 92 | 0 … 0 | 0 … 0 | – | – |
| cyclist | humans | ✓ 5/5 | 14.3 … 30.1 | 20.8 … 36.1 | 329 … 470 | 12 … 109 | 13 … 354 | 0 … 329 | 0 … 487 | 36.53 … 37.43 | – |
| dairy-cow | animals | ✓ 5/5 | 23.3 … 32.6 | 28.6 … 36.8 | 423 … 484 | 22 … 78 | 16 … 266 | 17 … 191 | 25 … 283 | 38.66 … 38.86 | – |
| date-palm | vegetation | ✓ 4/4 | 26.7 … 43.5 | – | 435 … 530 | -70 … 36 | -61 … 110 | 0 … 158 | 0 … 233 | – | – |
| delivery-van | vehicles | ✓ 6/6 | 19.2 … 1398.3 | – | 276 … 287657 | 0 … 287381 | -5 … 12647 | 0 … 0 | 0 … 0 | – | brakes net rad 287381; brakes surface 1398 C; engine-bay net rad 1072 (+7) |
| diesel-generator | machinery | ✓ 5/5 | 22.3 … 221.6 | – | 116 … 2718 | 16 … 2435 | 12 … 1640 | 0 … 0 | 0 … 0 | – | exhaust-stack net rad 1111; exhaust-stack net rad 1128; exhaust-stack net rad 2435 (+5) |
| dining-table | furniture | ✓ 5/5 | 21.0 … 30.1 | – | 106 … 432 | 0 … 50 | 0 … 35 | 0 … 0 | 0 … 0 | – | – |
| domestic-cat | animals | ✓ 6/6 | 21.6 … 38.3 | 24.0 … 38.3 | 411 … 523 | 11 … 119 | 16 … 373 | 7 … 112 | 11 … 167 | 38.20 … 38.73 | pads skin 24.0; pads skin 24.1 |
| drinking-fountain | objects | ✓ 4/4 | 16.9 … 53.2 | – | 46 … 472 | -25 … 86 | -199 … 166 | 0 … 196 | 0 … 287 | – | – |
| elderly-with-cane | humans | ✓ 6/6 | 9.5 … 21.1 | 16.0 … 27.6 | 326 … 416 | 14 … 81 | -15 … 146 | 0 … 20 | 0 … 29 | 36.00 … 36.27 | – |
| electric-kick-scooter | vehicles | ✓ 5/5 | 19.5 … 972.0 | – | 254 … 95414 | -27 … 95123 | -4 … 10868 | 0 … 0 | 0 … 0 | – | bms-controller net rad 1622; bms-controller surface 173.5 vs air 20; brake-disc net rad 95123 (+3) |
| english-oak | vegetation | ✓ 5/5 | 20.8 … 29.0 | – | 402 … 449 | -28 … 24 | -14 … 100 | 0 … 80 | 0 … 117 | – | – |
| excavator | machinery | ✓ 4/4 | 20.2 … 316.6 | – | 310 … 4803 | 5 … 4548 | 1 … 2810 | 0 … 0 | 0 … 0 | – | engine-compartment net rad 631; engine-compartment net rad 747; engine-compartment net rad 863 (+9) |
| external-venetian-blind | components | ✓ 6/6 | 31.5 … 45.0 | – | 409 … 523 | -7 … 76 | -2 … 97 | 0 … 0 | 0 … 0 | – | – |
| folding-deck-chair | furniture | ✓ 5/5 | 23.5 … 34.2 | – | 395 … 456 | 18 … 79 | 35 … 93 | 0 … 0 | 0 … 0 | – | – |
| forklift | machinery | ✓ 4/4 | 19.1 … 659.0 | – | 290 … 34249 | 1 … 33916 | -5 … 5571 | 0 … 0 | 0 … 0 | – | brakes net rad 10123; brakes net rad 33916; brakes surface 419.8 vs air 20 (+1) |
| glass-balustrade | components | ✓ 5/5 | 18.9 … 24.9 | – | 342 … 358 | 24 … 54 | -4 … 61 | 0 … 0 | 0 … 0 | – | – |
| glulam-beam | components | ✓ 4/4 | 19.6 … 28.0 | – | 354 … 420 | 10 … 98 | -5 … 112 | 0 … 0 | 0 … 0 | – | – |
| goat | animals | ✓ 4/4 | 23.1 … 31.8 | 23.9 … 37.1 | 397 … 481 | 30 … 108 | 33 … 252 | 7 … 122 | 10 … 180 | 39.06 … 39.28 | hooves skin 23.9 |
| heat-pump-outdoor-unit | machinery | ✓ 6/6 | -85.1 … 110.5 | – | 18 … 505 | -64 … 225 | -709 … 1134 | 0 … 15316 | 0 … 24290 | – | base-tray surface 97.9 vs air 7; coil-guard surface -75.1 vs air 7; coil-guard surface -85.1 vs air 7 (+2) |
| hedgehog | animals | ✓ 4/4 | 12.4 … 32.6 | 13.3 … 33.8 | 358 … 476 | 21 … 146 | 19 … 540 | 0 … 55 | 0 … 81 | 34.82 … 35.15 | core 34.82; core 35.00; core 35.08 (+1) |
| height-adjustable-desk | furniture | ✓ 4/4 | 22.0 … 76.8 | – | 387 … 765 | 0 … 378 | 0 … 323 | 0 … 0 | 0 … 0 | – | – |
| herring-gull | animals | ✓ 4/4 | 14.2 … 30.7 | 16.0 … 40.7 | 367 … 464 | 17 … 108 | 41 … 851 | 6 … 141 | 8 … 208 | 40.72 … 41.38 | – |
| hornbeam-hedge | vegetation | ✓ 5/5 | 19.1 … 23.3 | – | 389 … 425 | -32 … -10 | -70 … 56 | 0 … 123 | 0 … 180 | – | – |
| horse | animals | ✓ 4/4 | 19.7 … 33.6 | 29.3 … 38.9 | 396 … 492 | 1 … 84 | -2 … 317 | 0 … 642 | 0 … 956 | 37.84 … 40.78 | – |
| japanese-maple | vegetation | ✓ 6/6 | 17.6 … 27.4 | – | 385 … 439 | -46 … 8 | -68 … 95 | 0 … 141 | 0 … 206 | – | – |
| jogger | humans | ✓ 7/7 | 23.2 … 32.5 | 24.8 … 35.1 | 394 … 485 | 27 … 96 | 24 … 232 | 10 … 343 | 16 … 509 | 36.70 … 37.85 | arm-skin skin 24.8; hand-skin skin 24.8 |
| kneeling-gardener | humans | ✓ 4/4 | 27.0 … 35.6 | 30.4 … 35.8 | 142 … 495 | 22 … 115 | 39 … 178 | 0 … 161 | 0 … 239 | 36.98 … 37.41 | – |
| labrador-dog | animals | ✓ 4/4 | 22.9 … 37.6 | 24.3 … 37.9 | 418 … 518 | 19 … 115 | 14 … 353 | 7 … 105 | 10 … 156 | 38.15 … 38.81 | pads skin 24.3 |
| leaning-person | humans | ✓ 6/6 | 15.6 … 28.2 | 24.4 … 29.7 | 337 … 458 | 11 … 82 | -11 … 75 | 0 … 20 | 0 … 30 | 36.36 … 36.67 | arm-skin skin 24.4; arm-skin skin 24.6; hand-skin skin 24.8 (+1) |
| lombardy-poplar | vegetation | ✓ 4/4 | 20.0 … 28.1 | – | 398 … 443 | -28 … 18 | -30 … 59 | 0 … 87 | 0 … 128 | – | – |
| london-plane-pollarded | vegetation | ✓ 6/6 | 19.9 … 31.9 | – | 397 … 467 | -46 … 24 | -72 … 70 | 0 … 143 | 0 … 209 | – | – |
| mallard-duck | animals | ✓ 5/5 | 18.3 … 31.8 | 22.1 … 40.5 | 389 … 471 | 35 … 112 | 20 … 410 | 0 … 101 | 0 … 151 | 41.35 … 41.69 | core 41.59; core 41.66; core 41.69 |
| market-parasol | objects | ✓ 4/4 | 28.1 … 49.2 | – | 374 … 551 | -36 … 90 | 30 … 172 | 0 … 0 | 0 … 0 | – | – |
| masonry-wall-with-lintel | components | ✓ 5/5 | 0.4 … 8.2 | – | 286 … 320 | 23 … 57 | 5 … 64 | 0 … 0 | 0 … 0 | – | – |
| miscanthus-grass | vegetation | ✓ 5/5 | 18.7 … 25.8 | – | 377 … 421 | -34 … 4 | -142 … 61 | 0 … 202 | 0 … 296 | – | – |
| motor-scooter | vehicles | ✓ 5/5 | 20.0 … 893.4 | – | 277 … 68260 | 1 … 67984 | 1 … 8716 | 0 … 0 | 0 … 0 | – | brake-disc net rad 67984; brake-disc surface 893 C; engine net rad 1079 (+9) |
| multi-stem-birch | vegetation | ✓ 5/5 | 19.2 … 25.5 | – | 385 … 423 | -31 … 3 | -61 … 49 | 0 … 113 | 0 … 166 | – | – |
| office-swivel-chair | furniture | ✓ 4/4 | 22.0 … 22.0 | – | 387 … 387 | 0 … 0 | 0 … 0 | 0 … 0 | 0 … 0 | – | – |
| panel-radiator | components | ✓ 6/6 | 19.3 … 79.2 | – | 104 … 786 | 1 … 415 | -3 … 325 | 0 … 0 | 0 … 0 | – | panel-front net rad 415 |
| parent-carrying-toddler | humans | ✓ 4/4 | 12.9 … 28.5 | 17.9 … 32.1 | 342 … 460 | 38 … 127 | 78 … 310 | 6 … 49 | 8 … 72 | 36.43 … 37.08 | – |
| parent-with-stroller | humans | ✓ 5/5 | 8.4 … 28.0 | 14.5 … 31.3 | 71 … 457 | 4 … 133 | 8 … 209 | 0 … 30 | 0 … 45 | 36.45 … 36.97 | – |
| park-bench | furniture | ✓ 4/4 | 27.8 … 40.0 | – | 419 … 490 | 42 … 114 | 58 … 138 | 0 … 0 | 0 … 0 | – | – |
| patio-heater | objects | ✓ 5/5 | 4.7 … 668.1 | – | 68 … 35605 | 8 … 35351 | -20 … 5649 | 0 … 0 | 0 … 0 | – | emitter-head net rad 14811; emitter-head net rad 35312; emitter-head net rad 35351 (+7) |
| person-on-crutches | humans | ✓ 4/4 | 19.6 … 32.9 | 24.1 … 34.4 | 125 … 487 | 12 … 117 | 44 … 202 | 0 … 79 | 0 … 118 | 36.62 … 37.22 | hands skin 24.1; hands skin 24.3 |
| person-with-rollator | humans | ✓ 5/5 | 8.3 … 20.8 | 14.6 … 28.8 | 320 … 415 | 18 … 89 | 8 … 184 | 0 … 21 | 0 … 31 | 36.29 … 36.57 | – |
| person-with-umbrella | humans | ✓ 4/4 | 7.3 … 17.2 | 12.7 … 26.5 | 88 … 395 | 2 … 54 | -7 … 210 | 0 … 29 | 0 … 42 | 36.31 … 36.44 | – |
| person-with-white-cane | humans | ✓ 5/5 | 12.2 … 22.9 | 17.5 … 28.5 | 331 … 427 | 6 … 81 | 8 … 186 | 0 … 33 | 0 … 48 | 36.33 … 36.81 | – |
| picnic-table | furniture | ✓ 5/5 | 24.1 … 42.6 | – | 133 … 508 | 7 … 131 | 61 … 138 | 0 … 0 | 0 … 0 | – | – |
| pig | animals | ✓ 4/4 | 24.5 … 37.7 | 24.5 … 38.3 | 423 … 503 | 12 … 112 | -48 … 161 | 14 … 308 | 21 … 455 | 38.64 … 39.39 | tail skin 24.5 |
| pigeon | animals | ✓ 8/8 | 23.0 … 53.5 | 25.9 … 83.3 | 414 … 620 | 19 … 222 | 17 … 2295 | 8 … 854 | 12 … 1295 | 41.30 … 86.14 | core 41.50; core 41.64; core 41.95 (+2) |
| playground-swing | objects | ✓ 4/4 | 27.1 … 35.5 | – | 129 … 463 | -5 … 32 | 82 … 112 | 0 … 0 | 0 … 0 | – | – |
| pregnant-person | humans | ✓ 5/5 | 18.2 … 28.2 | 18.4 … 31.3 | 368 … 458 | 16 … 102 | 24 … 118 | 5 … 26 | 8 … 38 | 36.63 … 36.90 | – |
| red-fox | animals | ✓ 4/4 | 8.1 … 20.0 | 22.2 … 39.2 | 341 … 402 | 28 … 94 | 2 … 206 | 1 … 24 | 2 … 36 | 38.18 … 39.71 | – |
| red-squirrel | animals | ✓ 4/4 | 15.2 … 24.3 | 15.8 … 38.6 | 372 … 426 | 22 … 87 | 38 … 662 | 4 … 110 | 7 … 162 | 38.05 … 39.69 | – |
| retractable-bollard | objects | ✓ 5/5 | 11.0 … 118.1 | – | 104 … 1195 | -7 … 856 | -10 … 1167 | 0 … 0 | 0 … 0 | – | hydraulic-unit net rad 856; hydraulic-unit surface 118.1 vs air 12 |
| roe-deer | animals | ✓ 6/6 | 22.8 … 32.8 | 23.9 … 38.5 | 398 … 481 | 18 … 80 | 14 … 377 | 7 … 113 | 10 … 168 | 38.04 … 39.49 | hooves skin 23.9; hooves skin 24.5 |
| runner-with-blade-prosthesis | humans | ✓ 6/6 | 22.6 … 34.0 | 24.2 … 36.7 | 382 … 495 | 16 … 106 | 86 … 461 | 0 … 345 | 0 … 512 | 36.85 … 39.42 | arm-skin skin 24.4; hand-skin skin 24.2 |
| running-child | humans | ✓ 7/7 | 21.4 … 34.2 | 25.0 … 35.4 | 384 … 496 | 13 … 103 | 37 … 311 | 0 … 180 | 0 … 268 | 37.04 … 37.78 | socks skin 25.0 |
| scissor-lift | machinery | ✓ 4/4 | 22.9 … 66.3 | – | 131 … 677 | 22 … 307 | 8 … 284 | 0 … 0 | 0 … 0 | – | – |
| scots-pine | vegetation | ✓ 5/5 | 20.1 … 29.8 | – | 390 … 454 | -33 … 28 | -25 … 145 | 0 … 81 | 0 … 118 | – | – |
| seated-guitarist | humans | ✓ 5/5 | 24.7 … 32.9 | 27.5 … 33.8 | 402 … 488 | 21 … 73 | 10 … 68 | 0 … 22 | 0 … 33 | 36.64 … 36.97 | – |
| seated-reader | humans | ✓ 5/5 | 13.8 … 26.5 | 20.8 … 29.5 | 346 … 448 | 15 … 88 | -1 … 69 | 0 … 14 | 0 … 20 | 36.65 … 36.78 | – |
| sectional-garage-door | components | ✓ 5/5 | 0.0 … 4.0 | – | 95 … 284 | 8 … 36 | 0 … 48 | 0 … 0 | 0 … 0 | – | – |
| sheep | animals | ✓ 4/4 | 21.2 … 33.5 | 26.8 … 39.1 | 404 … 492 | 25 … 119 | 8 … 300 | 3 … 89 | 4 … 132 | 39.02 … 39.44 | – |
| shopping-trolley | objects | ✓ 4/4 | 23.0 … 27.8 | – | 109 … 418 | -10 … 40 | 45 … 138 | 0 … 0 | 0 … 0 | – | – |
| single-bed | furniture | ✓ 5/5 | 21.0 … 26.9 | – | 382 … 414 | 0 … 32 | 0 … 20 | 0 … 0 | 0 … 0 | – | – |
| skateboarder | humans | ✓ 5/5 | 19.9 … 39.7 | 28.7 … 36.0 | 63 … 505 | -8 … 154 | 19 … 301 | 0 … 316 | 0 … 469 | 37.06 … 38.32 | – |
| sliding-door | components | ✓ 4/4 | 7.5 … 9.8 | – | 281 … 297 | 43 … 57 | 38 … 52 | 0 … 0 | 0 … 0 | – | – |
| small-leaved-lime | vegetation | ✓ 7/7 | 17.9 … 31.1 | – | 387 … 462 | -56 … 19 | -146 … 90 | 0 … 223 | 0 … 327 | – | – |
| spiral-stair | components | ✓ 5/5 | 13.0 … 23.9 | – | 117 … 398 | 24 … 102 | -9 … 37 | 0 … 0 | 0 … 0 | – | – |
| stair-flight | components | ✓ 4/4 | 23.5 … 32.1 | – | 351 … 443 | 44 … 122 | 31 … 88 | 0 … 0 | 0 … 0 | – | – |
| standing-phone-user | humans | ✓ 4/4 | 10.5 … 23.1 | 16.2 … 27.8 | 330 … 428 | 14 … 90 | -14 … 93 | 0 … 15 | 0 … 23 | 36.65 … 36.69 | – |
| steel-column | components | ✓ 4/4 | 22.6 … 26.8 | – | 369 … 390 | 30 … 89 | 19 … 86 | 0 … 0 | 0 … 0 | – | – |
| stepladder | objects | ✓ 4/4 | 18.8 … 34.1 | – | 50 … 391 | 2 … 17 | 15 … 109 | 0 … 0 | 0 … 0 | – | – |
| street-lamp | objects | ✓ 6/6 | 15.0 … 26.7 | – | 110 … 367 | 0 … 54 | 0 … 106 | 0 … 0 | 0 … 0 | – | – |
| three-seat-sofa | furniture | ✓ 6/6 | 21.9 … 27.3 | – | 387 … 416 | 5 … 34 | 4 … 22 | 0 … 0 | 0 … 0 | – | – |
| tied-canopy | components | ✓ 4/4 | -1.6 … 3.1 | – | 254 … 292 | 25 … 37 | -20 … 44 | 0 … 0 | 0 … 0 | – | – |
| tilt-window | components | ✓ 4/4 | 0.9 … 3.3 | – | 268 … 298 | 23 … 35 | 9 … 77 | 0 … 0 | 0 … 0 | – | – |
| tour-guide | humans | ✓ 5/5 | 17.0 … 24.6 | 22.1 … 29.9 | 80 … 437 | 4 … 65 | 28 … 118 | 0 … 32 | 0 … 46 | 36.49 … 36.63 | – |
| tower-crane | machinery | ✓ 5/5 | 19.5 … 114.2 | – | 172 … 1124 | 12 … 763 | -4 … 675 | 0 … 0 | 0 … 0 | – | hoist-motor net rad 557; hoist-motor net rad 763; hoist-motor surface 114.2 vs air 20 (+1) |
| tram | vehicles | ✓ 5/5 | 19.7 … 694.0 | – | 132 … 32248 | 2 … 31971 | -2 … 5557 | 0 … 0 | 0 … 0 | – | brake-discs net rad 31971; brake-discs net rad 8092; brake-discs surface 417.1 vs air 20 (+9) |
| walking-commuter | humans | ✓ 5/5 | 6.8 … 28.6 | 13.1 … 32.0 | 313 … 461 | 10 … 141 | -11 … 268 | 0 … 38 | 0 … 56 | 36.64 … 37.01 | – |
| waste-bin | objects | ✓ 4/4 | 26.7 … 50.9 | – | 413 … 575 | 9 … 163 | 39 … 215 | 0 … 0 | 0 … 0 | – | – |
| weeping-willow | vegetation | ✓ 4/4 | 21.1 … 27.3 | – | 404 … 439 | -35 … 13 | -9 … 54 | 0 … 106 | 0 … 155 | – | – |
| wheel-loader | machinery | ✓ 5/5 | 20.5 … 297.5 | – | 350 … 4810 | 20 … 4512 | 2 … 2575 | 0 … 0 | 0 … 0 | – | engine-bay net rad 540; engine-bay net rad 541; engine-bay surface 88.9 vs air 20 (+7) |
| wheelchair-ramp | components | ✓ 5/5 | 16.0 … 31.9 | – | 119 … 442 | 22 … 170 | 24 … 103 | 0 … 0 | 0 … 0 | – | – |
| wheelchair-user | humans | ✓ 5/5 | 7.6 … 30.0 | 12.8 … 31.7 | 54 … 469 | 4 … 140 | -15 … 217 | 0 … 36 | 0 … 54 | 36.65 … 37.08 | – |
| worker-carrying-box | humans | ✓ 6/6 | 19.7 … 33.9 | 33.1 … 36.0 | 375 … 494 | -0 … 85 | -1 … 182 | 0 … 239 | 0 … 355 | 37.17 … 37.79 | – |
