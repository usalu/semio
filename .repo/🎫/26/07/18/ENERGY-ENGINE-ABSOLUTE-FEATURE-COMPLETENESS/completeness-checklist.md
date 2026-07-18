# Energy Engine Leaf Completeness Checklist

## Coupling (Engine::run timestep)

| Leaf | Status | Module |
|------|--------|--------|
| Calendar / run period / DST | pending | calendar.rs |
| Warmup convergence | pending | kernel.rs |
| Predictor-corrector HVAC | pending | kernel.rs |
| CTF surface heat balance | pending | envelope.rs |
| Fenestration solar + conduction | pending | fenestration.rs |
| Solar geometry + shading | pending | solar.rs |
| Daylight dimming → lighting | pending | daylight.rs |
| Room air model | pending | room_air.rs |
| AFN infiltration/ventilation | pending | airflow_network.rs |
| Thermostat / humidistat | pending | controls.rs |
| Zone HVAC equipment | pending | zone_hvac.rs |
| Air systems + terminals | pending | air_system.rs |
| Plant loops + dispatch | pending | plant.rs, dispatch.rs |
| Electrical PV/battery/grid | pending | electrical.rs |
| SHW / solar thermal | pending | shw.rs, solar_thermal.rs |
| Water / refrigeration | pending | water.rs, refrigeration.rs |
| Faults on equipment | pending | faults.rs |
| Delivered energy meters | pending | meters.rs |
| Design-day sizing feedback | pending | sizing.rs |
| Economics post-pass | pending | economics.rs |

## Model schema

| Leaf | Status |
|------|--------|
| Shading surfaces | pending |
| Space lists / enclosures | pending |
| Mechanical ventilation | pending |
| Humidistats / setpoint managers | pending |
| Zone equipment assignments | pending |
| Air / plant / condenser loops | pending |
| AFN network definition | pending |
| Electrical / PV / battery | pending |
| SHW / solar thermal / refrigeration / water | pending |
| Faults / output variables / sizing objects | pending |
| Daylight zones / room air model | pending |

## Validation

| Test | Status |
|------|--------|
| Energy conservation | pending |
| ASHRAE 140 case600 | pending |
| HVAC BESTEST reference | pending |
| Full topology E2E | pending |
| Deterministic repeatability | pending |
