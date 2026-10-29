# Thermal model of the staffage archive

Module: `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts` (`solveThermal`, `thermalLegend`, `plumeLegend`, `thermalColour`).
Deterministic in (state, environment). Steady-state surface energy balances per region plus Lagrangian jet/puff plumes per emitter.
This is an illustrative engineering estimate. It is **not CFD, not validated against measurements, and not physiologically accurate**.

## Conventions

- Temperatures in the API are °C, computed in K. Fluxes in W/m² of region area, positive = leaving the surface; powers in W use `areaM2 × exposedFraction`.
- `windDirectionDeg`: direction the air moves **towards**, CCW from +x in the elevation plane (0 = blows to the right). `relativeHumidity` is a fraction 0…1.
- Relative air velocity = ambient wind vector − figure centre-of-mass velocity (`state.kinematics.comVelocity`); it drives convection and the plume frame.
- `netFluxWM2 = q_rad + q_conv + q_lat − q_solar`. For `core-resistance` regions it equals the conduction flux `(T_core − T_s)/R`; for `leaf`/`passive` it is 0.

## Region equations

| Term | Equation | Source |
|---|---|---|
| Radiation | `q_r = ε σ (T_s⁴ − T_mrt⁴)`; reported `h_r = q_r/(T_s − T_mrt)` (secant, `4εσT³` at equality) | Stefan–Boltzmann, CODATA σ |
| Convection, clothed body (skin, clothing, hair) | `h_c = max(2.38 ΔT^0.25, 12.1 √v)` | ISO 7730:2005 annex D (Fanger) |
| Convection, plate (foliage, surface, engine, glazing, wall) | natural: `1.42 (ΔT/L)^¼` for Ra < 10⁹ else `1.31 ΔT^⅓`; forced: `Nu = 0.664 Re^½ Pr^⅓` (Re < 5·10⁵) or `(0.037 Re^0.8 − 871) Pr^⅓`; `h = max(natural, forced)` | Holman; Incropera & DeWitt ch. 7, 9; ASHRAE Fundamentals ch. 4 |
| Convection, cylinder (fur, feather, bark, tyre, exhaust) | same natural; forced Churchill–Bernstein cross-flow | Incropera & DeWitt ch. 7 |
| Evaporation (skin etc.) | `E = w h_e (p_sk − p_a)`, `h_e = 16.5 h_c` W/(m²·kPa) (scaled by 101325/p), Magnus `p_sk`; `w` = region wettedness (default 0.06 for kind `skin`, 0 otherwise); no condensation | ASHRAE Fundamentals ch. 9 (Lewis relation) |
| Leaf | `λE = λ ρ g_s g_b/(g_s+g_b) (q_sat(T_leaf) − q_a)`, `g_b = h_c/(ρ c_p)·(Pr/Sc)^⅓`, `Pr = 0.71`, `Sc = 0.60`, solved jointly with radiation, convection and absorbed solar for `T_leaf` | Monteith & Unsworth; Campbell & Norman 1998 |
| Absorbed solar | `α G f_area`; `f_area` = 0.25 closed bodies, 0.5 foliage and engines, 1 surface/wall/glazing; α absent ⇒ 0 | authored-estimate |
| `fixed` | `T_s = surfaceTempC` | authored |
| `core-resistance` | `(T_core − T_s)/R = q_r + q_c + q_e − q_sw`, bracketed Illinois root solve to 10⁻¹¹ W/m² (≈10⁻¹² K) | ISO 7730 clothing balance |
| `passive` | `α G f_area = q_r + q_c` | energy balance |
| `operating` | `T_s = T_air + riseK × operating[operatingKey]` | authored |
| Activity | `ΔT_core = min(1.5 K, 0.15 K/met × (met − baseMet) × share)`, `share = metabolicShare/mean metabolicShare` (clamped 0…2, 1 if absent); not applied when `coreTempC` is overridden | **authored-estimate** (monotone, qualitative) |
| Psychrometrics | `p_s = 611.21 exp((18.678 − T/234.5)·T/(257.14+T))` (Magnus form, Buck 1996, within 0.05 % of IAPWS-95; Alduchov & Eskridge 1996 was rejected: 0.26 % off at 25 °C), `q = 0.622 e/(p − 0.378 e)`, `λ = 2.501·10⁶ − 2361 T` | literature |
| Respiration | latent `0.0173 M (5.87 − p_a[kPa])`, sensible `0.0014 M (34 − T_a)` (W/m², `M = 58.15·met`) × DuBois area `0.202 m^0.425 h^0.725`, scaled by mean breath-emitter strength, reported in `totals.emitter*` | ASHRAE Fundamentals ch. 9; DuBois 1916 |

Region overrides (`state.thermal.regions`): `surfaceTempC` fixes `T_s`; `coreTempC` replaces the core; `wettedness` replaces `w`; `exposedFraction` scales the powers.

## Emitters

- Position: `state.anchors[emitter.anchor].at`; direction = anchor bone world angle + `directionDeg`. Strength = `state.thermal.emitters[id]` (absent = 0), scales `velocityMS`.
- Duty: `periodS` with `dutyFrom`/`dutyTo` as period fractions; no period = continuous. `Streamline.active = strength × (in window ? 1 : 0)`.
- Steady sources (exhaust, vents, transpiration): free round turbulent jet, `u = u₀ min(1, 6.2 d/x)`, `ΔT, Δq = ·min(1, 5 d/x)` (Pope ch. 5).
- Pulsed sources (period and duty < 1, i.e. breath): puff after the vortex-ring formation number 4, `D = (x/4d)³`, `u = u₀ (4d/x)³` (Turner 1973).
- Dilution `D = max(1, x/5d)` (jet) with `x = air-frame path + |relative wind|·t` (cross-flow eddies dilute like path length; Briggs bent-over plume). Temperature excess **and vapour excess** both decay as `1/D`.
- Buoyancy: `v_b = g ΔT₀/T_a · t/D` (buoyancy impulse conserved while entrainment dilutes; Morton, Taylor & Turner 1956). Parcel ground velocity = relative wind + jet + `v_b ŷ`; Heun integration, step = 2 % of the jet length scale.
- Stop: temperature excess < 0.2 K **and** vapour excess < 2 % of source, **or** relative speed < 0.05 m/s (0.005 for vapour carriers), or 10 m from the source, or 60 s.
- Transpiration: temperature ≤ ambient (evaporative cooling), vapour-carrying; the legend reads the vapour **excess over ambient** (`plumeLegend`).
- Streamline: 24 points, uniform in arc length. Parcels: emitted every `period/max(8, ⌈8 period⌉)` (periodic) or 1/8 s, positioned on the same path at their age; lateral spread `0.7 · hash(id, slot) · max(d/2, 0.1 x)` with a fixed FNV-1a/murmur3 hash. Looping duty reproduces parcels after one period exactly (slot index modulo slots per period).

## Verification (third-party oracles)

| Case | Oracle | Tolerance |
|---|---|---|
| 🌡️ blackbody-exchange | `scipy.constants.Stefan_Boltzmann` + numpy | 0.5 % (the coordinator's "≈ 77 W/m²" is 75.40 W/m² for ε = 0.95, 33 °C vs 20 °C) |
| 💧️ vapour-pressure | CoolProp IAPWS-95 `PropsSI`, `HAPropsSI` | 0.2 % saturation (actual max deviation 0.05 %), 0.6 % humidity ratio |
| 🧍️ clothed-human | scipy `brentq` + CoolProp; PMV of the solved `t_cl` vs `pythermalcomfort.pmv_ppd_iso` | 0.1 K, 2 % fluxes, PMV 0.02 |
| 🍃️ leaf-transpiration | scipy `brentq`, CoolProp saturation pressure and air properties | 1 % λE, 0.1 K (observed ≤ 0.06 K; CoolProp vs. correlation air properties) |
| 🫁️ breath-plume / 🚗️ exhaust-wind | scipy `solve_ivp` RK45 (rtol 1e-10) of the same ODE | 1 cm / 5 mm polyline |

The ISO 7730 fixture lumps `ε·f_eff = 0.7` into an effective emissivity 0.6984 so the comparison isolates the solver. Plume oracles re-integrate the same documented ODE; they validate the numerics, not the plume physics.

## Non-claims

No CFD or measured validation of plumes; the free-jet and puff laws are textbook scalings applied to a frozen field. Activity-driven core rise, solar area factors, default wettedness and per-region parameters marked `authored-estimate` or `illustrative` are not physiological or engineering predictions. Conditions outside 0…60 °C, 90…105 kPa, and Re > 10⁶ are unchecked. No condensation, no sweat dynamics, no thermoregulation, no conduction between regions.
