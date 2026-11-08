# DIN V 18599 target: scene, editor and norm coverage

## 1. Norm parts and what the artifact covers

| Part | Subject | Status in the artifact | Gap |
|---|---|---|---|
| -1 | General balancing, primary energy, reference building | zones, `derive_balance`, GEG reference run (`q_p_ref`, limit factor), checks `din18599.1.*`, `din18599.geg.qp` | system boundaries per service not modelled |
| -2 | Net heating and cooling demand, monthly balance | monthly balance per zone (`heating_months`: Q_T, Q_V, Q_i, Q_s, eta, Q_h, Q_c), utilisation factor with a = 0.95 | single zone-wide a (no time constant), simplified solar geometry, no night setback |
| -4 | Lighting | zone power density limit check, control factor | daylight supply factors, constant-illuminance control missing |
| -5 | Heating final energy | system efficiency product, energy carrier, check `din18599.5.heating-efficiency` | boiler part load, generator temperature levels |
| -6 / -7 | Ventilation, cooling | heat recovery eta, fan power, cooling EER | humidity, ventilation duct losses |
| -8 | Domestic hot water | demand, storage and distribution loss, limit check | circulation, solar fraction |
| -9 | Renewables | PV area, efficiency, solar thermal | PV tilt, battery |
| -10 | Boundary conditions, usage profiles | profiles WFH / office / school, Potsdam climate | all remaining profiles |
| -11 / -12 | Building automation, tabular method | BACS factor, tabular q_p check | remaining tables |

## 2. Visual targets

- Scene: zones are translucent boxes (side sqrt(A), height V/A), envelope elements panels on the face of their orientation; colour = utilisation of the checks naming the entity.
- Diagrams: `monthly-balance:<zone>` (losses, gains, net heating bars, net cooling bars), `energy-flow:building` (useful, final, primary energy bands per service), `element-u:<element>` (U against GEG reference).
- Handles: element U (drag along the face normal), zone storey height (writes the zone volume A * h).

## 3. Tests

- Rust unit tests in `🎬️scene/🧪️tests/🔬️unit`: scene laws, determinism, live == full, diagram laws, focus, handle round trip.
- Language-agnostic: `🎬️scene/🧪️tests/🎬️scene-din18599-1/🥒️.feature`.
- Independent oracle: `🎬️scene/🔮️oracles/🐍️.py` (numpy), compared month by month with `zone_heating_months` at 1e-9; it agrees with the older annual oracle `🔮️oracles/⚖️compliance/🐍️.py` (checked 2026-11-08 on the change-delta-u-wb fixture: Q_h 525.245017, Q_c 167205.839609).
