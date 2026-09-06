# 🏛️ BESTEST oracle contract (W-B honeybee/EnergyPlus ⇄ W-C semio engine)

Two producers, one input, one output shape. The semio-native `s.energy.model` document is the single source of truth for every case; the oracle TRANSLATES it (never re-authors the case from the ASHRAE 140 spec independently, except as a cross-check).

## Locations (subset root `A` = `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any`)
- Weather (shared, committed once, owner W-B): `A/🧫️fixtures/🌦️denver-tmy/🌦️.epw` — Denver/Golden CO annual TMY EPW (prefer BESTEST's Denver `725650TY.epw` from NREL/BESTEST-GSR; fallback: EnergyPlus-bundled `USA_CO_Golden-NREL.724666_TMY3.epw`). Sidecar `A/🧫️fixtures/🌦️denver-tmy/🔣️.json` = `{ "source": "<url>", "sha256": "<hex>", "station": "...", "records": 8760 }`.
- Per case `<case>` ∈ {600, 600FF, 900, 900FF, 610, 620, 630, 640, 650, 910, 920, 930, 940, 950, 960}: dir `A/🧫️fixtures/🏛️bestest-<case>/`
  - `🔋️model.json` (owner W-C): the semio `Model` serialized with the engine's own canonical JSON (`pack::json::to_json_string(&model)` / `ToValue`), i.e. exactly the `model` field of an `EnergyModelSnapshot`.
  - `🔮️energyplus.json` (owner W-B): oracle results, shape below.
  - `⚙️semio.json` is NOT committed — the engine produces it at test time into the test work dir and compares.
- Comparison test (owner W-C): `A/🧪️tests/🏛️simulate-bestest-energyplus/` (`🥒️.feature` + `🦀️.rs` + `🐍️.py`), plus Rust unit tests mounted in the crate.

## Result JSON shape (both producers emit exactly this; hourly arrays are 8760 long, index 0 = Jan 1 00:00–01:00)
```json
{
  "schema": "semio.energy.bestest-results/1",
  "case": "600",
  "producer": { "name": "energyplus", "version": "25.2.0", "via": "honeybee-energy 1.123.32 → honeybee-openstudio 0.7.2 → OpenStudio 3.11.0" },
  "weather": { "file": "🧫️fixtures/🌦️denver-tmy/🌦️.epw", "sha256": "<hex>" },
  "timestepMinutes": 60,
  "annual": { "heatingKwh": 0.0, "coolingKwh": 0.0 },
  "peak": { "heatingKw": 0.0, "heatingHour": 0, "coolingKw": 0.0, "coolingHour": 0 },
  "freeFloat": { "minC": 0.0, "minHour": 0, "maxC": 0.0, "maxHour": 0, "meanC": 0.0 },
  "hourly": { "zoneAirTemperatureC": [], "heatingW": [], "coolingW": [], "transmittedSolarSouthWh": [] }
}
```
- `producer.name` ∈ `energyplus` | `semio-energy-engine`. For the semio producer `via` is the crate version.
- `freeFloat` is `null` for controlled cases; `annual`/`peak` are `null` for FF cases.
- `transmittedSolarSouthWh` optional (E+ `Surface Window Transmitted Solar Radiation Energy` on the south apertures, J→Wh); omit if unavailable.
- Numbers are plain JSON numbers (no strings), kWh/kW/°C/Wh as named.

## Case physics (ASHRAE 140 §5.2, shared authority for W-C's models and W-B's cross-check)
- 600: 8 m × 6 m × 2.7 m single zone (129.6 m³), 12 m² south glazing (two 3 m × 2 m windows), lightweight constructions, infiltration 0.5 ACH, internal gains 200 W continuous (60 % radiative / 40 % convective), heating setpoint 20 °C, cooling 27 °C, ideal loads unlimited capacity. 600FF: same, no HVAC. 900/900FF: high-mass walls/floor. 610/910: 1 m south overhang. 620/920: 6 m² glazing on each of east+west. 630/930: 620 + overhang + fins. 640/940: night setback 10 °C 23:00–07:00. 650/950: night ventilation 1703.16 m³/h 18:00–07:00, cooling only. 960: sunspace (two zones). Exact layer data: see `📓️explore-oracle-toolchain.md` §4 and ASHRAE 140.
- Comparison tolerances (initial acceptance, tighten later): annual heating/cooling within the ASHRAE 140 reference envelope widened by 15 %, or, where no envelope is available, within 20 % relative of EnergyPlus; peaks within 25 %; free-float min/max within ±2.5 K; hourly zone temperature RMSE ≤ 2.0 K. Every failure is recorded per metric in the test outcome (class `warning` if within 2× tolerance, `error` otherwise).
