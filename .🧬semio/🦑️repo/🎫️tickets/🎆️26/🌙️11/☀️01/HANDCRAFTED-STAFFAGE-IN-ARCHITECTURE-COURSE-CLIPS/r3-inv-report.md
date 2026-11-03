# r3-inv: architecture course clip inventory and migration matrix (W01, haiku-inventory)

W01 was read-only and returned this report; the coordinator saved it. Paths are relative to A = `🎓️teaching/🏛️architecture`.

## 1. Counts per series

| series | videos | chapters | manifest segments | unique scene classes |
|---|---|---|---|---|
| ⚡️energy | 5 | 26 | 113 | 105 (physics 8, heating 33, cooling 17, demand 25, ventilation 22), plus 1 scene outside the manifest (`FullFinalCalculationVideo`) |
| 🎯️optimization | 6 | 6 | 6 | 6 |
| 🏗️bim | 6 | 6 | 6 | 6 |
| 📊️excel | 4 | 4 | 4 | 4 |
| 🦗️grasshopper | 5 | 5 | 5 | 5 |
| nakagin, typology, morph, parametric, fit, pets | 0 | 0 | 0 | 0 (no scenes; nakagin has visuals and the building model only) |
| **total** | **26** | **47** | **134** | **127** |

The manifest reuses 8 scenes across segments:
- physics Beat6 is reused in heating 05 and cooling 03
- solar Beat2, Beat3 and Beat5 are reused in cooling 04
- `AnlagenVerluste` is reused in EB04

## 2. Matrix

Mode key: T = thermal, G = general, M = mixed, SF = structural force. View key: T = thermal, DL = dark line.

### Physics (`⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py`)

| scene:line | objective | staffage (call line) | cast ids | view | mode |
|---|---|---|---|---|---|
| Beat1_Leistung:561 | power = energy / time | vacuum l.694; kettle and shower l.711–712 | physics-fundamentals-vacuum, -kettle, -shower | T elev | T |
| Beat2_EnergieImAlltag:889 | kWh on the meter | clerk l.986; shower l.1089 | physics-fundamentals-clerk, -shower | T | T |
| Beat3_Energieerhaltung:1315 | energy changes form | person l.1576; lamp l.1384, l.1483 | shared-person-winter, shared-lamp-winter | T | T |
| Beat4_Waermepumpe:1664 | heat pump lifts heat | heat-pump parts l.1746–1749; radiator l.1762, l.1917 | physics-fundamentals-evaporator, -condenser, -compressor, -valve; shared-radiator-front-winter | T | T |
| Beat5_Strahlung:2017 | short-wave radiation, blind | blind l.2219, l.2260; radiator l.2204 | shared-blind-lowered-mirrored, shared-radiator-winter | T | T |
| Beat6_ThermischeMasse:2296 | thermal mass and ΔT | none (room and window as building) | none | T | T |
| Beat7_SensibelLatent:2668 | sensible vs latent heat | hob l.2742, pot l.2747; person l.2937 | physics-fundamentals-hob, -pot; shared-person-winter | T | T |
| Beat8_Zusammenfassung:3073 | recap | none | none | none | G |

### Heating

| file / scene:line | objective | staffage (call line) | cast ids | mode |
|---|---|---|---|---|
| fundamentals Beat1–5 (`_stage` l.151) | three modes of heat transfer | radiator `_radiator` l.122; occupant `_occupant` l.127; brick wall `_brick_wall` l.108 (building) | shared-radiator-front-winter, shared-person-winter | T |
| fundamentals Beat6, Beat8, Beat9 | R/U arithmetic | none | none | G |
| fundamentals Beat7:1151 | resistance per layer | wall section l.1359 (building) | none | T |
| conduction Beat1–3, Beat6 | conduction, R, U, thermal bridges | wall layers l.93, slab l.234 (building) | none | T |
| conduction Beat4:838 | heat output of person, toaster, tealight | `_devices` l.287 | shared-person-winter, shared-toaster-winter, heating-conduction-tealight | T |
| conduction Beat5:999 | envelope | radiator thermostat l.1034 | shared-radiator-thermostat-front | T |
| convection Beat1:139 | ventilation heat loss | none (primitives not checked) | none | T |
| internal-gains Beat1:281 | winter gains | person l.252, desk l.255 (DL), laptop l.256, lamp l.257 | shared-person-winter, shared-desk, shared-laptop-winter, shared-lamp-winter | M |
| internal-gains Beat2:349 | heat of one person | worker l.373 | heating-internal-gains-office-worker | T |
| internal-gains Beat3:420 | equipment heat | worker l.446; server tower l.448 | heating-internal-gains-office-worker, heating-internal-gains-tower-server | T |
| internal-gains Beat4:524 | lighting becomes heat | pendant l.549; person l.620; laptop l.623; lamp l.625 | heating-internal-gains-pendant-lamp-warm-up, shared-person-winter, shared-laptop-winter, shared-lamp-winter | T |
| solar-gains Beat1–4, Beat8 | solar gain, g-value, sun angle | sun icon | none | T |
| solar-gains Beat5:848 | shading | blind slats l.826 | shared-blind-lowered-mirrored | T |
| calculation ReviewingHeatLosses:333 | review of losses | house l.361 (building); lamp l.223 (untraced) | shared-lamp-winter | T |
| calculation ReviewingHeatGains:645 | review of gains | person l.700, l.703; desk l.706 (DL); laptop l.708 | shared-person-winter, shared-desk, heating-calculation-laptop | M |
| calculation UltimateEnergyBalance:1052 | balance | person l.1086; thermostat l.1089 | shared-person-winter, shared-radiator-thermostat-front | T |
| calculation AnlagenVerluste:1283 | system losses | `_plant_house` l.1200–1282 | shared-person-winter, shared-radiator-thermostat, heating-calculation-thermostat, shared-tank-charge-up, shared-boiler-heating-cycle | T |
| calculation FullFinalCalculationVideo:1481 | alternate full chapter video | composed from the beats above | none | T |

### Cooling

| file / scene:line | objective | cast ids | mode |
|---|---|---|---|
| heating-comparison Beat1:252 | winter gains vs summer | shared-toaster-winter, cooling-heating-comparison-laptop, shared-lamp-winter, shared-person-winter, shared-person-fanning, cooling-heating-comparison-table (DL), shared-desk (DL) | M |
| heating-comparison Beat2:320 | summer overheating | cooling-heating-comparison-toaster-summer, -laptop-summer, shared-lamp-summer | T |
| heating-comparison Beat3:387 | air handling unit (AHU) cooler | cooling-heating-comparison-cooler | T |
| internal-gains Beat1:176 | office room | cooling-internal-gains-office-chair (DL), -seated-worker | M |
| internal-gains Beat2:242 | heat by activity | cooling-internal-gains-sleeper, -seated-worker, -walker, -jogger, -standing | T |
| internal-gains Beat3:540 | devices and lighting | cooling-internal-gains-laptop, -laptop-closed, -server-rack, -lamp-dimmed; shared-lamp-summer | T |
| internal-gains Beat4:759 | cumulative load | cooling-internal-gains-standing, -laptop; shared-lamp-summer | T |
| internal-gains Beat5:873 | mitigation | shared-lamp-summer, cooling-internal-gains-lamp-dimmed | T |
| transmission-humidity Beat1:205, Beat3:352 | transmission and air load | none (house) | T |
| solar-radiation Beat1:180, Beat5:396 | solar load | none (sun icon) | T |
| system-sizing Beat2:200 | volume flow | cooling-system-sizing-diffuser | T |
| system-sizing Beat3:357 | airflow | none | T |
| summer-strategy Beat1:149 | lower the load first | shared-person-fanning, cooling-summer-strategy-blind | T |
| summer-strategy Beat2:270 | desiccant evaporative cooling (DEC) | cooling-summer-strategy-wheel, -exchanger, -humidifier, -heater, -collector | T |
| summer-strategy Beat3:473 | comfort strategy | none | T |

### Demand

| file / scene:line | objective | cast ids | mode |
|---|---|---|---|
| recap Beat1:394 | balance boundary | shared-person-winter, shared-radiator-front-winter, shared-lamp-winter, shared-desk (DL), shared-laptop-winter | M |
| recap Beat2:551 | loss side | thermometer pair (glyph) | G |
| recap Beat3:879 | gain side | shared-radiator-front-winter | T |
| recap Beat4:1180 | winter and summer | none | G |
| regulations Beat1:525 | standards | shared-person-winter, shared-lamp-winter, shared-radiator-winter | T |
| regulations Beat2–4 | monthly balance, climate, degree days | none | G |
| demand-power Beat1:384 | power vs energy | shared-radiator-winter, shared-person-winter, shared-lamp-winter | T |
| demand-power Beat2:570 | design heat load | shared-lamp-winter | T |
| demand-power Beat3–4 | duration curve, sizing | none | G |
| system-losses Beat2:532 | hand-over, distribution, storage | shared-person-winter, shared-boiler-heating-cycle, shared-heat-pump-heating-sweep, shared-tank-charge-up, shared-radiator-winter, shared-gas-meter-run, shared-electricity-meter-run | T |
| system-losses Beat3:705 | generation | none | G |
| system-losses Beat4:938 | plant effort figure | shared-gas-meter-run, shared-electricity-meter-run | T |
| primary-energy Beat1:369 | three boundaries | demand-primary-energy-pumpjack, shared-pump-run-high, shared-boiler-heating-cycle, shared-person-winter, shared-radiator-front-winter | T |
| primary-energy Beat2:563 | primary-energy factor | demand-primary-energy-tree (DL), shared-boiler-heating-cycle, shared-gas-meter-run, shared-electricity-meter-run | M |
| primary-energy Beat3–4 | worked example, reference building | none (pipes, house) | G |
| certificate Beat1:296 | demand vs consumption | shared-person-winter, demand-certificate-sleeper, shared-lamp-winter, shared-radiator-front-winter | T |
| certificate Beat2:490 | energy scale | none | G |
| certificate Beat3:712 | mandatory information | demand-certificate-buyer, shared-person-neutral | T |
| practice Beat1:248 | calculation vs billing | shared-radiator-front-winter, shared-person-winter, shared-person-neutral | T |
| practice Beat2:656 | hydronic balancing | demand-practice-valve, shared-radiator-front-winter, shared-pump-run-high, shared-gas-meter-run, shared-boiler-heating-cycle | T |
| practice Beat3:992 | series close | shared-lamp-winter, shared-lamp-off-winter, shared-radiator-front-winter, shared-boiler-heating-cycle; `energy_tank` store l.1144 (OI-04) | T |

### Ventilation

| file / scene:line | objective | cast ids | mode |
|---|---|---|---|
| need Beat1:210 | CO₂ and moisture build-up | shared-person-pathlines (via `_room_occupants` l.148) | T |
| need Beat2:406 | air path | none | T |
| need Beat3:576 | roadmap | shared-mvhr-normal | T |
| volume Beat1:448, Beat2:687 | net volume, air change | prop helper l.392–428: ventilation-volume-crates-* (DL), shared-mvhr-normal, shared-person-pathlines, shared-office-worker-pathlines, ventilation-volume-blackboard (DL), -cook, -hood-mirrored, -operating-table | M |
| volume Beat3:890 | V·n airflow | none (room) | T |
| heat-flow Beat1:261 | heat capacity of air | ventilation-heat-flow-heater | T |
| heat-flow Beat2–4 | sensible and latent flow | none (house) | T |
| natural Beat1:305 | window rules | shared-person-pathlines; `window_glyph` | T |
| natural Beat2:531 | burst vs tilted ventilation | ventilation-natural-radiator-on-mirrored, -off-mirrored | T |
| natural Beat3–6 | cross ventilation, stack effect, night ventilation, limits | none (room, windows) | T |
| mechanical Beat1:246, Beat2:421 | types, heat recovery | ventilation-mechanical-duct-fan | T |
| mechanical Beat3:705 | MVHR vs decentralised | ventilation-mechanical-mvhr, -pendulum-fan, shared-person-pathlines | T |
| mechanical Beat4:902 | duct cross-section | none | T |
| handoff Beat1–2 | summary, hand-over | none | G |

### Optimization

All six optimization scenes use `RoomVisuals.room_section` (`🎯️optimization/🎬️clip/🎨️visuals/🏠️room/🐍️.py:571`). It draws:
- optimization-room-occupant
- shared-radiator-winter
- optimization-room-cooler-mirrored
- shared-lamp-neutral and shared-lamp-off-neutral

Extra casts:
- extrema (shared-radiator-front-winter, optimization-extrema-cooler)
- experiments (optimization-experiments-occupant, optimization-experiments-window)
- index (shared-lamp-neutral)

Mode is T throughout.

### BIM, Excel and Grasshopper

| scene | cast ids | view | mode |
|---|---|---|---|
| bim information Beat1 | bim-information-car (l.212) | DL plan | G |
| bim objects, representations, models | none | — | G |
| bim exchange Beat5:169 | shared-person-relaxed-line (l.875); slab, piloti and structural check | DL | SF |
| bim consistency Beat6 | shared-person-relaxed-line (l.273), bim-consistency-decider (l.1105) | DL | G |
| excel program Beat3 | excel-program-occupant (`occupant_spots` l.59, l.368) | DL plan | G |
| excel anatomy, references, energy | none | — | G |
| grasshopper (all 5) | none (Nakagin capsule and tower building geometry) | — | G |

## 3. Shared infrastructure

**V bridge**
- `V/🐍️.py`, with these line positions:
  - `fitted_staffage` l.295
  - `window_glyph` l.838
  - `house_section` l.899
  - `room_section` l.951
  - `radiator_wall` l.783
  - `meter` l.548
  - `thermometer_glyph` l.1073
  - `energy_tank` l.1104
  - `pendant_lamp` l.1126
  - `shared-watt-*` l.260–267 (no caller)
- `V/🧍️staffage/🐍️.py`, with these line positions:
  - `staffage_height` l.698
  - `staffage` l.758
  - `staffage_motion` l.913
- `V/🧍️staffage/🔣️.json`, with 94 casts.
- `V/🧪️tests/🐍️staffage_only.py`

**Authoring**
- theme and fonts
- output
- speech
- camera
- animation
- render
- `🪟️presentation/📚️collection/🔣️.json`, which holds the lazy chapter ids

**Series visuals**
- optimization `🎨️visuals/🏠️room`
- bim, excel and grasshopper `ClipScene` bases
- `🏢️nakagin/🎨️visuals` (`NakaginDrawing3D`), used by excel and grasshopper
- `🏢️nakagin/🐍️.py`
- `🏗️bim/🎬️clip/🧱️model` (`_car` l.508)

## 4. Findings

1. **Nakagin furniture is still built from boxes** in `🏢️nakagin/🐍️.py` (OI-46), at these lines:
   - reception l.1419–1421
   - cafe l.1424–1433
   - office l.1437–1441
   - bath l.2248–2270
   - shelves l.2323–2352
   - bed, desk and sofa l.2449–2567
2. **Building contexts are drawn ad hoc** (house, room, wall, roof, pipe and window drawings). The coordinator ruling on whether they are in scope is pending.
3. **Mixed thermal and dark-line scenes:**
   - heating internal-gains Beat1
   - calculation ReviewingHeatGains
   - cooling heating-comparison Beat1
   - cooling internal-gains Beat1
   - recap Beat1
   - primary-energy Beat2
   - volume Beat1 and Beat2
4. **Missing canonical combinations:**
   - Thermal plan casts are missing.
   - Front views are missing for people and lamps.
5. **Untraced call sites:**
   - calculation l.223
   - recap l.201–218
   - regulations l.320–425
   - demand-power l.250–300
   - volume l.392–447
   - heating-comparison l.77 and l.138–151
   - `shared-watt-*`
6. **German voice-over by design:** `set_vo_language("de")` is set in natural, volume, heat-flow, mechanical, need and handoff.

## 5. Questions

1. Are the house, room, wall, roof, pipe and window drawings in scope?
2. Should the Nakagin furniture move into the archive?
3. Are mixed thermal and dark-line scenes allowed?
4. Should the volume prop set be recomposed?

These are answered in `r3-coordinator-rulings.md`, round 3, rulings R1–R4.
