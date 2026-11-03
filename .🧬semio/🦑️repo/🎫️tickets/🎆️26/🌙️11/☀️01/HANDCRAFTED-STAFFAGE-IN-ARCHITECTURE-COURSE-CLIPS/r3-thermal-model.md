# R3 thermal staffage model: state, gaps, API

S = `🎓️teaching/🏛️architecture/🧍️staffage`. Full model description, equations, sources and validation table: `ARCHITECTURAL-STAFFAGE-ARCHIVE/thermal-v2.md` (ticket 26/10/29). This file covers the requirement matrix of round 3 and the API for the other workers.

**Illustrative engineering model**: lumped regions, no CFD, steady or first-order transient, no clinical, building-code or research-grade claim. Validity: air -30..60 C, 80..105 kPa, Re < 1e6 (outside unchecked). ISO 7730 PMV only inside its own limits (flagged by `valid`).

## Requirement matrix

| Requirement | State before R3 | Evidence | R3 action |
|---|---|---|---|
| Colours from an explicit model, unit and legend | met: `QUANTITY_SPECS` (value first, then diverging ramp, legend with quantity, unit, range, centre, sign, conditions) | `🎨️thermal-legend` | none (ramp: `🖍️palette divergingColour`, centre = air temperature or 0) |
| Surface temperature vs radiative flux | met: separate quantities `surface-temperature`, `skin-temperature`, `emitted-radiant-flux` (eps sigma T^4), `net-radiative-flux` (+ = loss, sky/ground/surroundings split) | `🌡️blackbody-exchange` (scipy CODATA) | none |
| Sensible vs moisture vs latent | met: `sensible-heat-flux`, `latent-heat-flux`, `moisture-flux` (g/m2h), totals separate | `🍃️leaf-transpiration`, `🫕️phase-change` | none |
| Absolute vs comparative illustration | partial: range modes existed, legend only said "normalised" | | **legend now says "Absolute values" (shared, fixed) or "Comparative illustration" (auto, span), en/de**; test in `🎨️thermal-legend` |
| clo (ISO 9920), met (ISO 8996) | partial: layers in m2K/W and `activityMet` existed, no clo/met API, no comfort index | | **new `🌡️thermal/🛋️comfort`**: `cloToM2KW`, `m2KWToClo`, `metToWM2`, `relativeAirSpeedMS`, `clothingInsulationClo(asset)`, `pmvPpd`, `bodyComfort` |
| Material, emissivity | met: `Material.emissivity` required (all 149 assets), outermost layer decides | catalog audit | none |
| Ambient T, MRT, wind, equipment state | met: `Environment` (+ timeline), `operating` keys | | none |
| Breathing 12-16 /min, inhale/exhale timing | model met (patterns with fractions, half-sine flow); data unchecked | | **new catalog test `🫁️breathing-catalog`**: all 28 resting adult poses (<= 1.2 met) bind 12..16 bpm, exhale >= inhale, fractions sum 1 |
| Directional nose/mouth emitter on the rig | met in data (80 breath emitters, anchors on head bones) | | test: emitter kind breath, anchor exists, bone exists, head bone (passengers: own bone) |
| Equipment exhaust follows operating state | met (`level = operating[key] x emitters[id]`, ignition/shutdown test) | `💨️transport-field` | test: every `exhaust/flue/fan` has a declared `operatingKey`, every `region` exists |
| Plumes: buoyancy, wind, strength, dissipation | met: integral entrainment model (Morton-Taylor-Turner, Kaminski) RK4 on a 1/120 s world grid | `🌀️parcel-jet` (closed-form jet) | none |
| Emitted particles in WORLD space | met: parcel born at the anchor's world position of the birth instant, integrated alone; turning/moving source leaves the trail | `💨️transport-field` (turn, walking, loop) | none |
| Tree transpiration = vapour + latent cooling | met: `transpiration` emitter reads `latentW` of a leaf region, carries `vapour`, no momentum; leaves cool (Penman-Monteith) | `🍃️leaf-transpiration`, `🍃️penman-monteith` (pyet) | catalog test: every vegetation asset has transpiration emitters on leaf regions and no plume over a leaf region |
| Invisible vapour never fog unless condensation or labelled explanatory | partial: vapour drawn as hatched discs, no explicit label | | **parcel legends carry an "Explanatory visualization ... not smoke or fog" assumption (en/de)**; visible droplets only from the separate `condensation` layer (saturation adjustment); condensation veil doc corrected (ink 12 %, not white) |
| Deterministic, seeded, play = pause = seek = loop | met: pure function of absolute time, hashed gaussian from (seed, parcel, step), `periodSteps` for loops | `💨️transport-field` (bitwise play = scrub, loop repeat) | none |

## Findings on parameters (not changed)

- Skin vasodilation gain `DILATION = 120` in `🫀️physiology` equals pythermalcomfort 4.8 `two_nodes_gagge` (`c_dil = 120`, source comment: "ashrae says 50"; Gagge 1986/ASHRAE 55 text quotes 200). The oracle in `🫀️two-node-gagge` is therefore consistent only with 120; changing to 200 would break the oracle match (core 0.04 K, skin 0.96 K observed). Decision: keep 120, documented here; revisit only with a different oracle. Coordinator finding F6 not applied for this reason.
- Four adult activity assets have no resting pattern in the 12-16 band as default `patterns[0]`: jogger (`run` 41.67, poses bind explicit patterns), cyclist (`recovery` 17), worker-carrying-box (`carrying` 20). Poses bind patterns explicitly, so no resting pose is affected; the default only scales chest motion and unbound poses. Children (mass < 40 kg) breathe faster by design (20-22 at rest).
- Pose `activityMet` 1.3..1.6 with speech (`talking` 17, tour-guide `talk` 11, pregnant `deep-stretching-breath` 10) intentionally leave 12-16.

## API notes (for W08 people, W09 vegetation, W14/W15 clips)

```ts
import { solveThermalAt, thermalSession, pmvPpd, bodyComfort, clothingInsulationClo, cloToM2KW, metToWM2, QUANTITY_SPECS } from "S/🔨️modules/🌡️thermal/🟦️.ts";
import { solveTransport } from "S/🔨️modules/💨️transport/🟦️.ts";
```
- Clips: cast manifest `view: "thermal"` uses `solveThermalAt` + `thermalSession` (transient, play = scrub). `transport: [layers]` + `transportAgeS` (warm-up seconds so the plume is already developed in frame 0); `transportTrailS`, `transportStrokeMm` style the pathlines; `thermalSpanK` gives the comparative span range.
- People (W08): breathing is data: `respiration.patterns[]` (rateBpm 12-16 at rest, `route`), `noseEmitter/mouthEmitter` on head anchors; poses bind `thermal.breathing` and `activityMet`; clips may key a `breathing` step track. Rules are enforced by `🫁️breathing-catalog`. New adult assets must satisfy it.
- Vegetation (W09): add a leaf region (`model: "leaf"`) and `transpiration` emitters with `region` = leaf region and `anchors` over the crown; do not add `plume` to leaves. Transpiration is drawn as vapour parcels labelled explanatory; never use fog styling.
- Comfort numbers for captions: `bodyComfort(asset, environment, activityMet)` returns PMV, PPD, sensation, `valid`, `violations`; clo from the asset's clothing layers. Plain parameters: `pmvPpd({ airTempC, meanRadiantTempC, relativeAirSpeedMS, relativeHumidity, met, clo })`. Always show `valid`; PMV is a population statistic for indoor conditions.
- Absolute temperature figures: use `range: shared` or `fixed`; use `span`/`auto` only as comparative illustration (legend says so).

## Sources

ASHRAE Handbook Fundamentals ch. 9 (thermal comfort; respiratory heat, Lewis relation), ch. 1 (psychrometrics); ISO 7730:2005 (PMV/PPD Annex D, relative air speed), ISO 8996 (1 met = 58.15 W/m2), ISO 9920 (1 clo = 0.155 m2K/W), ISO 7726 (operative temperature), Gagge/Stolwijk/Nishi 1971, Gupta/Lin/Chen 2010 (breathing flow), Morton/Taylor/Turner 1956, Kaminski/Tait/Carazzo 2005, Allen et al. 1998 FAO-56. Oracles: pythermalcomfort 4.8 (`two_nodes_gagge`, `pmv_ppd_iso`), CoolProp, psychrolib, pyet, scipy.
