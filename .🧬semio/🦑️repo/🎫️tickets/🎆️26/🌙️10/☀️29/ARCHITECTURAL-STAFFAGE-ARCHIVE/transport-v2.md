# Transport v2 (transport worker)

S = `🎓️teaching/🏛️architecture/🧍️staffage`. Modules: `S/🔨️modules/💨️transport` (`🫁️respiration`, `🌫️air`, `🌬️ambient`, `🎲️noise`, `🔥️sources`, `🧮️parcels`, `〰️streamlines`),
`S/🔨️modules/🎨️depiction/💨️transport-view`. Replaces the v1 plume code in `🌡️thermal` and the cases `🫁️breath-plume`, `🚗️exhaust-wind`.
Illustrative engineering model, **not CFD, not validated against measurements**. Status (session 2, 2026-10-29): module loads (a duplicate `query` declaration from session 1 was fixed), tests green except the condensation fixture pending its CoolProp run (see Validation).

## Interface for CORE (delivered first, stable)

`S/🔨️modules/💨️transport/🫁️respiration/🟦️.ts` (also re-exported by `S/🔨️modules/💨️transport/🟦️.ts`):

```ts
respirationState(asset: Asset, query: EvaluationQuery): RespirationState | null   // null: asset has no thermal.respiration
respirationOffsets(asset: Asset, state: RespirationState): Readonly<Record<Slug, { angleDeg: number; dy: number }>>
```

- `query.timeS` is the absolute timeline time (pose queries: `timeS ?? 0`). For a looping clip pass the **unwrapped** time: the breathing phase runs on the absolute timeline; pattern requests wrap internally.
- `respirationOffsets` returns **additive** deltas per bone (`angleDeg`, parent-frame `dy`) = `Respiration.motion × volumeL / patterns[0].tidalVolumeL`.
  Convention (authoring guide must say it): the **first pattern of every `Respiration` is the resting pattern** and defines the motion reference volume (a running pattern with 3× the volume moves the chest 3×).
- Transport calls `evaluate(asset, {…query, timeS})` itself at emission times (needs `anchors[*].at`, `bones[*].angleRad`, `thermal.operating`, `thermal.emitters`) and `solveThermal(state, environment)` (plume/transpiration strengths). Both are injectable through `TransportOptions.evaluate/thermal`.
- Depiction: `transportView(ctx, field, layers, range) → { overlay(canvas), legends }` in `S/🔨️modules/🎨️depiction/💨️transport-view/🟦️.ts`. Proposed plug-in (CORE): `Analysis.transport?: TransportField`; in `depict()` after the shapes, if `options.transport.length > 0`:
  `const t = transportView(ctx, analysis.transport ?? solveTransport(state.asset, state.query, options.environmentTimeline ?? options.environment ?? state.asset.thermal.environment), options.transport, options.range); t.overlay(canvas); legends.push(...t.legends)`.
  It is independent of the selected view (thermal, forces or line views can carry it) and draws into the layers `field` and `annotations`.

## Model

### Respiration (`🫁️respiration`)
- Breaths are consecutive: breath `k` starts at `t_k` (breath 0 at t = 0) and lasts `T_k = 60/rate` of the pattern requested at `t_k`. Phase `φ(t) = k + (t − t_k)/T_k` = ∫ rate(t)/60 dt of the rate in effect, continuous and monotone.
  A pattern change (pose `thermal.breathing`, clip key pose, `breathing` step track, transition at half duration) is **latched at the next breath boundary**, where the lung is at FRC: the phase never jumps, lung volume stays continuous and every breath moves exactly its tidal volume.
- Cycle: inhale / pause / exhale / pause with the pattern fractions. Flow of an active phase of duration `T_p`: `π V_T/(2 T_p) · sin(π τ/T_p)` (half-sine, ∫ = V_T exactly; sign: + exhaling). Sinusoidal breathing flow after Gupta, Lin & Chen (2010), Indoor Air 20:31–39 (existence verified, flow shape S in `references-physiology.md`).
  Lung volume above FRC: inhale `V_T/2 (1 − cos πx)`, pause `V_T`, exhale `V_T/2 (1 + cos πx)`.
- Requested pattern of a clip without track: the key pose binding of the current segment, switching to the next key's at the segment midpoint (looping clips wrap); no binding ⇒ first pattern.
- Route split: nose / mouth / `nose-and-mouth` = 50/50 (`NOSE_SHARE_MIXED`, authored-estimate); a missing emitter hands its share to the other opening.
- Exhaled temperature / humidity: the authored `Respiration.exhaledTempC` / `exhaledRelativeHumidity` (reference environment) are used as given. **No ambient dependence is modelled**: Höppe 1981 (T_ex = f(T_a, x_a)) is UNVERIFIED in `references-physiology.md`; the only support for "expired air up to ≈ 37 °C, saturated" is secondary (DTIC ADA210378). The authored 34 °C / 95 % is `authored-estimate`.
- Nostril/mouth opening areas and jet angles (0.71 cm², 1.2 cm², nose ≈ 60° downward) are UNVERIFIED (authored-estimate). Real mouth/nose jets (Gupta 2010) spread faster than the ≈ 8.7° half angle of the entrainment law near the opening — limitation.
- Respiratory heat: `respiratoryHeat(respiration, pattern, environment)` = mean over the cycle of the enthalpy that warms and humidifies the inhaled air to the exhaled state (dry-air mass flow = V_E / v(T_ex, w_ex)); latent = Δw·h_fg(T_ex).

### Parcels (`🧮️parcels`) — integral Lagrangian "slice" model
State per parcel: world position, velocity `u` relative to the local wind, dry-air mass `m_d`, total enthalpy `h` and total water `w_t` per kg dry air, slice duration `τ` (= its emission interval).

| Quantity | Equation |
|---|---|
| Volume, radius | `V = m_d v(T, w_v)`; `b = √(V/(π u_e τ))`, `u_e = √(|u|² + σ_u²)` (cross-section radius of the stream the slice belongs to) |
| Entrainment | `dV_e/dt = 2π b ℓ α u_e = 2 α u_e V/b` ⇒ `db/dt = 2α|u|`; momentum jet `u = U₀/(1 + 2αx/r₀)`, `Q ∝ x` (round turbulent jet, Pope 2000 ch. 5; top-hat equivalents) |
| Coefficient | `α(Ri) = α_j + (α_p − α_j) min(1, Ri/Ri_p)`, `Ri = |g'| b/u_e²`, `Ri_p = 8α_p/5`, `α_j = 0.076`, `α_p = 0.116` (top-hat = √2 × Gaussian 0.0535 / 0.083; Fischer et al. 1979, Morton–Taylor–Turner 1956, Kaminski–Tait–Carazzo 2005) |
| Momentum | `du/dt = g' ŷ − (ṁ_d/m_d) u`, entrained air at rest relative to the wind |
| Buoyancy | `g' = g (ρ_a V − m)/m` with true densities (temperature, vapour deficit **and** liquid loading) |
| Mixing | `d(h, w_t)/dt = (ṁ_d/m_d)(ambient − parcel)`; temperature, vapour and condensate by saturation adjustment (conserves enthalpy including latent heat and total water) |
| Advection | `dx/dt = u_wind(y) + u` (RK4, h = 1/120 s), uniform or log wind profile, wind from the environment timeline |
| Dispersion | per step `σ_u √(2 T_L h) ξ` per axis, `σ_u = max(0.1 U, 0.03 m/s)`, `T_L = 1 s`, ξ from a murmur3 hash of (seed, parcel key, step index) |
| Ground | `y ≥ 0`, `u_y ≥ 0` there |
| Cull | age > 30 s, or `|ΔT| < 0.05 K` and `|Δq| < 0.05 g/kg` and no condensate |

Turner (1973, *Buoyancy Effects in Fluids*) supplies the thermal/puff entrainment picture and the plume constants; the pure-plume closure `Ri_p = 8α_p/5` follows from the MTT similarity solution (derived in the module header).
Not modelled: added mass, shear production, vortex-ring formation of short puffs (Turner 1957; short coughs would dilute faster than slices), stratification, ice, room walls, recirculation.

**Absolute timeline / purity.** Emission grid: parcel `k` of an emitter is born at step `kS + S/2` of the global 1/120 s grid (S = 4 breath = 1/30 s, 8 exhaust/vent/flue/fan = 1/15 s, 12 plume = 1/10 s, 60 transpiration = 1/2 s) from the world position, direction and speed of the asset evaluated at that instant (clip loops continue; core's `cycle`/root advance on the absolute timeline). Integration runs on integer step indices; the state at `t` is the grid state of `⌊120 t⌋` plus one RK4 sub-step of `t − n/120` (no noise). Parcels of time `t` are those born within `maxAgeS`, so the cost is bounded independent of `t`.
Cache: per (asset, query kind, environment, settings) track keeps parcel states and memoised pure functions (evaluated states, ambient, emissions, thermal samples at 0.5 s). Forward play advances incrementally; scrubbing backwards re-integrates from birth with identical arithmetic ⇒ bitwise equal (tested).

### Sources (`🔥️sources`) — never reuse breathing for other categories
- **breath**: exact exhaled volume of the emission interval (∫ of the half-sine flow; Σ over a breath = V_T), routed nose/mouth; only during exhalation; `u₀ = V/(Δt A)`. During inhalation no parcels are released and earlier ones keep evolving; a weak inhalation point sink `Q/(4πr²)` is included in the **streamline field only** (parcels do not feel it).
- **exhaust / vent / flue / fan**: `L = operating[operatingKey] × emitters[id]` scales flow, temperature rise and vapour excess (ignition/shutdown follow the tracks).
- **plume** (bodies, warm surfaces): convective heat `Q` of the emitter `region` (or total) from THERMAL `RegionField.sensibleW`, released continuously as air with the fixed numerical source excess `ΔT₀ = 2 K` (flow `Q/(ρ c_p ΔT₀)`), momentum from the flow through `diameterM`; strength × `emitters[id]`.
- **transpiration**: vapour mass flow `latentW(region)/h_fg` split equally over `anchors`, carried by air at ambient temperature with zero momentum and half the leaf-saturation vapour excess; no sensible jet; parcels `carries: "vapour"`.

### Streamlines and condensation
- Streamlines (`〰️streamlines`): velocity field = wind + Gaussian-weighted mean of the parcel velocities relative to the wind + inhalation sinks; seeds: 3 stations × 3 transverse points per active emitter, 4 points around each inhalation sink, a 5-point upwind rake; Heun, 2 cm arc-length steps, ≤ 2.4 m per side. They are instantaneous and differ from pathlines in unsteady flow (tested).
- `condensation`: parcels with liquid water from the saturation adjustment (visible condensation of exhaled 34 °C/95 % air mixing with cold ambient), listed separately and drawn in a separate layer labelled "modelled visible condensation".

## Parameter classification

| Parameter | Value | Class |
|---|---|---|
| Top-hat α_jet / α_plume | 0.076 / 0.116 | literature-derived (Fischer 1979; MTT 1956; Gaussian 0.0535 / 0.083 × √2) |
| Pure-plume Ri_p | 8α_p/5 | derived from MTT similarity (exact) |
| Gravity, cp_air 1006, cp_v 1860, cp_w 4186, L₀ 2.501e6, R_d 287.05 | | literature |
| Saturation pressure | Buck 1996 form | literature, within 0.05 % of IAPWS-95 (CoolProp) |
| Turbulence σ_u = max(0.1 U, 0.03 m/s), T_L 1 s | | illustrative |
| Plume source excess 2 K, transpiration source excess ½ leaf-saturation | | numerical / illustrative (fix the source volume, not the heat) |
| Mixed-route nose share 0.5 | | authored-estimate |
| Exhaled 34 °C / 95 %; opening areas; jet angles | asset data | authored-estimate (Höppe 1981 and opening areas unverified) |
| Emission intervals 1/30, 1/15, 1/10, 1/2 s; step 1/120 s; maxAge 30 s; cull 0.05 K / 0.05 g/kg | | numerical |
| Breath pattern latching at breath boundaries | | modelling decision (documented) |


## Session 2: fixes and verification

- **Loading bug fixed**: `💨️transport/🟦️.ts` declared `query` twice (parameter and local) and did not parse; renamed the local `now`.
- **Saturation above the boiling point**: `saturationRatio` returns `+Infinity` once `p_sat >= p` (hot exhaust at 150 K excess is never saturated; before, a negative ratio sent the saturation adjustment into nonsense).
- **Near-field stiffness fixed**: the jet near-field time scale `r0/(2αU0)` (6.6 ms for a 5 cm exhaust at 25 m/s, 11 ms for a 1 cm breath jet at 3 m/s) is shorter than the 1/120 s grid step; plain RK4 was off by 14 to 60 % against scipy. `rk4` now splits a grid step into `ceil(rate·h/0.2)` sub-steps (`MAX_RATE_STEP`, ≤ 256), a deterministic function of the state, so purity and play = scrub bitwise are unchanged.
- **Momentum with total mass**: `du/dt = g' − (ṁ_t/m_t) u` with `m_t = m_d(1+w_t)`, `ṁ_t = ṁ_d(1+w_a)` (the vapour of a 7 % steam-rich exhaust carries momentum; with dry mass only the jet decayed 4.6 % too fast against the closed-form law).
- Fixtures: `carrier` thermal stub added; parcel-jet case `momentum-jet` carries the ambient specific humidity (4.318 g/kg) so it is isothermal in vapour; respiration ASHRAE latent tolerance 0.25 (our 6.75 W vs regression 8.51 W at the resting pattern of 6.5 L/min: the regression scales ventilation with M, the model uses the authored pattern; CoolProp agreement is 2 %).
- New test `🧪️tests/💨️transport-assets` runs the real evaluator on the real assets (walking commuter, excavator, oak): play = scrub bitwise through loop boundaries, continuity across 1.2 s loop boundaries, nose anchor equals the evaluated anchor, excavator ignition/idle/rated, oak vapour-only and no respiration, representation independence.

## Validation (numbers)

| Check | Result | Tolerance |
|---|---|---|
| round-jet centreline `x(t)`, `u(x)`, `b(x)` vs closed form `u = U0/(1+2αx/r0)` | < 1 % | 1 % |
| parcel ODE (jet, hot jet 150 K + 7 % vapour, condensing puff) vs `scipy.integrate.solve_ivp` RK45, rtol 1e-10, independent psychrometrics (`🧪️tests/🌀️parcel-jet/🐍️.py`) | position < 2 mm, radius, velocity < 0.5 %, ΔT < 0.05 K, condensate < 0.02 g/kg | as listed |
| saturation, enthalpy, mixing-line condensate vs CoolProp `HAPropsSI`/`PropsSI` + brentq (`🧪️tests/☁️condensation/🐍️.py`) | fixture written by the oracle run | 0.15 K, 0.03 g/kg, 0.2 %, 0.4 % |
| half-sine ∫flow = V_T (scipy quad); respiratory heat vs CoolProp | volume 1e-9 rel.; heat < 2 % | |
| play = scrub (24, 25, 30, 60 fps, reverse, cold cache; synthetic and real assets) | bit-identical | exact |
| depicted scene (`depict` with transport layers) played vs scrubbed in reverse with a cold cache, 4 evidence cases, 62 sampled frames | identical SHA-256 of the scene JSON | exact |

Evidence (not committed; `T/🗑️generated/transport/evidence`): `commuter` (1:50, cold −2 °C / 92 %, wind 0.3 m/s then a head-on 3 m/s change at 6 to 8 s, pattern change walk-cycle), `headturn` (1:20, head pitched 50° at 3 to 3.8 s mid-exhalation: earlier parcels stay, new breath leaves along the new direction), `excavator` (1:50, ignition 0.8 s, idle 0.2, rated 1.0 from 6 to 8 s), `oak` (1:100, vapour parcels, pathlines and streamlines): stills, streamline stills and mp4 clips. Observed: breath trails stay in world space and condense visibly (cyan layer) only where the mixing line is supersaturated; pathlines (solid grey) and streamlines (dashed) are separate layers with separate legends.

## Export findings (READ only; export worker)

- `🎬️cli` builds `defaultDepictionOptions(...)` with `transport: []`: transport layers are not exported at all. Flags `--transport parcels,pathlines,streamlines,condensation` and `--environment` are needed.
- `frameSchedule` samples `t ∈ [0, duration)` of **one** loop. Transport is a pure function of absolute time, so frame `n` equals the scrubbed state at `t = n/fps`, but a looping video that restarts at t = 0 shows an empty air field at its first frame (no history). Transport exports need `--start` (seconds of absolute timeline before the first frame, ≥ maxAge for a settled field) and a duration independent of the clip duration; `evaluate` with `timeS` beyond the clip already continues the absolute timeline.
- `depict` accepts a precomputed `analysis.transport`; the exporter should solve per frame through `solveTransport` (the track cache makes forward frame sequences incremental).
- Verified here with the same pipeline (`evaluate` → `solveTransport` → `depict` → `writeSvg` → `rsvg-convert` → `ffmpeg`): scenes at the same time are identical whether played or scrubbed (see Validation).
- Core observation: with `office-laser` and `large-format-plot` profiles `walking-commuter` is `unsuitable` at 1:50 (`RepresentationError`); evidence used a profile with zero minima.

## Per-asset emitter audit (transport solver over every asset with emitters, 32 of 48)

Script `T/🧰️scripts/transport/audit-assets.ts` (poses ≤ 6 and every clip at t = 4.37 s and 9.11 s; anchors, outline probes, volume check). **`passengers-missing`**: `thermal.passengers` is required (`[]`) but missing in these assets; the thermal solver throws (`asset.thermal.passengers.map`), so the transport solver cannot run on them as authored (audit and tests normalise to `[]`). Tidal-volume integral error over 3 breaths is ≤ 1e-15 L for every asset.

"back of the head" means the breath direction has a backward horizontal component although the opening lies in front of the head centre (jet pointing into the neck/chest); "on the ground" means the nose anchor is within 5 cm of the ground in a grazing pose (legitimate for a grazing animal, but breath then lies on the ground).

| asset | category | emitters (! = finding on that emitter) | data | findings for content workers |
|---|---|---|---|---|
| cyclist | humans | breath-nose (breath), breath-mouth (breath), plume (plume) | defect | passengers-missing |
| elderly-with-cane | humans | breath-nose (breath), breath-mouth (breath) !, plume (plume) | defect | passengers-missing; breath-mouth: not emitted in the sampled poses/clips (route unused there) |
| jogger | humans | nose-breath (breath) !, mouth-breath (breath), plume (plume) | defect | passengers-missing; nose-breath: breath points towards the back of the head (anchor at 0.078 m from centre, dir x -0.23) |
| leaning-person | humans | nose-breath (breath), mouth-breath (breath), plume (plume) | defect | passengers-missing |
| parent-with-stroller | humans | breath-nose (breath), breath-mouth (breath), plume (plume), child-breath (vent) | defect | passengers-missing; passenger-stand-in: child-breath (vent) stands in for a living passenger; use ThermalModel.passengers with respiration (ruling 4) |
| person-with-umbrella | humans | nose-breath (breath), mouth-breath (breath), plume (plume) | defect | passengers-missing |
| running-child | humans | nose-breath (breath), mouth-breath (breath), body-plume (plume) | ok | none |
| seated-reader | humans | nose-breath (breath) !, mouth-breath (breath), body-plume (plume) | defect | nose-breath: breath points towards the back of the head (anchor at 0.050 m from centre, dir x -0.29); nose-breath: breath points towards the back of the head (anchor at 0.048 m from centre, dir x -0.31); nose-breath: breath points towards the back of the head (anchor at 0.056 m from centre, dir x -0.24); nose-breath: breath points towards the back of the head (anchor at 0.054 m from centre, dir x -0.26) |
| tour-guide | humans | nose-breath (breath), mouth-breath (breath), plume (plume) | defect | passengers-missing |
| walking-commuter | humans | nose-breath (breath), mouth-breath (breath), body-plume (plume) | ok | none |
| wheelchair-user | humans | nose-breath (breath), mouth-breath (breath), body-plume (plume) | ok | none |
| worker-carrying-box | humans | breath-nose (breath), breath-mouth (breath), plume (plume) | defect | passengers-missing |
| dairy-cow | animals | breath-nose (breath) !, plume-back (plume) | defect | passengers-missing; breath-nose: breath points towards the back of the head (anchor at 0.049 m from centre, dir x -0.40); breath-nose: anchor at y=0.020 m (on the ground); breath-nose: breath points towards the back of the head (anchor at 0.050 m from centre, dir x -0.40); breath-nose: anchor at y=0.025 m (on the ground); breath-nose: anchor at y=0.018 m (on the ground) |
| domestic-cat | animals | breath-nose (breath), breath-mouth (breath) !, plume-back (plume) | defect | passengers-missing; breath-mouth: not emitted in the sampled poses/clips (route unused there) |
| horse | animals | breath-nose (breath) !, plume-back (plume) | defect | passengers-missing; breath-nose: breath points towards the back of the head (anchor at 0.122 m from centre, dir x -0.37); breath-nose: breath points towards the back of the head (anchor at 0.123 m from centre, dir x -0.37); breath-nose: breath points towards the back of the head (anchor at 0.088 m from centre, dir x -0.44); breath-nose: anchor at y=0.045 m (on the ground); breath-nose: breath points towards the back of the head (anchor at 0.126 m from centre, dir x -0.37) |
| labrador-dog | animals | breath-nose (breath), breath-mouth (breath), plume-back (plume) | defect | passengers-missing |
| pigeon | animals | breath-nostril (breath) !, plume-back (plume) | defect | passengers-missing; breath-nostril: anchor at y=0.044 m (on the ground) |
| roe-deer | animals | breath-nose (breath) !, plume-back (plume) | defect | passengers-missing; breath-nose: anchor at y=0.027 m (on the ground); breath-nose: anchor at y=0.029 m (on the ground) |
| date-palm | vegetation | transpiration-fronds (transpiration) | defect | passengers-missing |
| english-oak | vegetation | transpiration-sun (transpiration), transpiration-shade (transpiration) | defect | passengers-missing |
| hornbeam-hedge | vegetation | transpiration-face (transpiration), transpiration-core (transpiration) | defect | passengers-missing |
| lombardy-poplar | vegetation | transpiration-crown (transpiration) | defect | passengers-missing |
| miscanthus-grass | vegetation | transpiration-blades (transpiration) | defect | passengers-missing |
| multi-stem-birch | vegetation | transpiration-crown (transpiration) | defect | passengers-missing |
| scots-pine | vegetation | transpiration-sun (transpiration), transpiration-shade (transpiration) | defect | passengers-missing |
| weeping-willow | vegetation | transpiration-curtain (transpiration) | defect | passengers-missing |
| street-lamp | objects | lamp-plume (plume) | ok | none |
| city-bus | vehicles | hvac-vent (vent) | ok | none |
| city-car | vehicles | tailpipe (exhaust) | ok | none |
| excavator | machinery | exhaust-stack (exhaust) | ok | none |
| forklift | machinery | motor-vent (vent) | ok | none |
| tower-crane | machinery | winch-vent (vent) | ok | none |

Summary: 9 of 32 assets run cleanly as authored (humans `running-child`, `walking-commuter`, `wheelchair-user`; `street-lamp`, `city-bus`, `city-car`, `excavator`, `forklift`, `tower-crane`); 22 need `thermal.passengers: []` (8 humans, 6 animals, 8 vegetation); `parent-with-stroller` additionally uses a `vent` stand-in for the toddler (move to a `Passenger` with its own `respiration`); `seated-reader`, `jogger`, `dairy-cow` and `horse` have nose directions with a backward component; ground-level noses (pigeon, roe-deer, dairy-cow) need their grazing-pose jet direction checked. Mouth emitters of `elderly-with-cane` and `domestic-cat` never fire because no sampled pattern routes through the mouth (add a mouth or mixed pattern to a pose, e.g. panting).

## Limitations (additions)

- The model is a vertical-plane section (x, y); an out-of-plane crosswind cannot be represented, so "crosswind change" means a change of the in-plane wind (speed and direction).
- Head "turn" in an elevation drawing is a pitch (rotation in the drawing plane); transport handles any bone rotation identically.
- Near-field sub-stepping assumes the ambient is constant over a grid step (it uses the half-step ambient); wind steps are therefore resolved to 1/120 s.
- Slice parcels do not form vortex rings; short puffs near the nozzle are not validated against measurements; mouth/nose jets spread faster than the entrainment law gives.
- Transpiration parcels carry vapour at ambient temperature; evaporative cooling of the surrounding air is not modelled.
