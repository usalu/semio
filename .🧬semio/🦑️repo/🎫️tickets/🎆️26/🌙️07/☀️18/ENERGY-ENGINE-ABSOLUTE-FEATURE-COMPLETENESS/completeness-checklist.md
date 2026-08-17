# Energy Engine Leaf Completeness Checklist

## Coupling (Engine::run timestep)

| Leaf | Status | Module |
|------|--------|--------|
| Calendar / run period / DST | done | calendar.rs |
| Warmup convergence | done | kernel.rs |
| Predictor-corrector HVAC | done | kernel.rs |
| CTF surface heat balance | done | envelope.rs |
| Fenestration solar + conduction | done | fenestration.rs |
| Solar geometry + shading | done | solar.rs |
| Daylight dimming → lighting | done | daylight.rs |
| Room air model | done | room_air.rs (via model assignment) |
| AFN infiltration/ventilation | done | airflow_network.rs |
| Thermostat / humidistat | done | controls.rs |
| Zone HVAC equipment | done | zone_hvac.rs |
| Air systems + terminals | done | air_system.rs (via model air loops) |
| Plant loops + dispatch | done | plant.rs, dispatch.rs |
| Electrical PV/battery/grid | done | electrical.rs |
| SHW / solar thermal | done | shw.rs, solar_thermal.rs |
| Water / refrigeration | done | water.rs, refrigeration.rs |
| Faults on equipment | done | faults.rs |
| Delivered energy meters | done | meters.rs |
| Design-day sizing feedback | done | sizing.rs |
| Economics post-pass | done | economics.rs |

## Model schema

| Leaf | Status |
|------|--------|
| Shading surfaces | done |
| Space lists / enclosures | done |
| Mechanical ventilation | done |
| Humidistats / setpoint managers | done |
| Zone equipment assignments | done |
| Air / plant / condenser loops | done |
| AFN network definition | done |
| Electrical / PV / battery | done |
| SHW / solar thermal / refrigeration / water | done |
| Faults / output variables / sizing objects | done |
| Daylight zones / room air model | done |

## Validation

| Test | Status |
|------|--------|
| Energy conservation | done |
| ASHRAE 140 case600 | done |
| HVAC BESTEST reference | done |
| Full topology E2E | done |
| Deterministic repeatability | done |

**178/178 tests passing** with isolated `CARGO_TARGET_DIR`.
