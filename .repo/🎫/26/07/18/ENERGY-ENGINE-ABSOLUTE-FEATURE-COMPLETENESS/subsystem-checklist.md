# Energy Technology — Subsystem Checklist

## Scaffold
- [x] energy/AGENTS.md, energy/engine project, workspace member, launch.json

## Foundation
- [x] error, units, num, props (psychrometrics, water, steam, refrigerant, glycol)

## Model + site
- [x] model (entities, validate), schedule, site (EPW, design days, solar, ground)

## Geometry + envelope
- [x] geometry, material, envelope, fenestration, solar, daylight

## Zone domain
- [x] zone_air, room_air, gains, air_exchange, airflow_network, iaq, comfort, controls

## HVAC
- [x] hvac_topo, ideal_hvac, zone_hvac, terminal, air_system, fans, coils, evaporative, humidity_eq, heat_recovery

## Plant + specialized
- [x] plant, shw, solar_thermal, refrigeration, electrical, water, faults, curves

## Kernel + outputs
- [x] kernel, sizing, dispatch, output, meters, metrics, results, economics, sim

## Validation
- [x] conservation tests, BESTEST/140-style cases, cargo test -p energy_engine (168 tests)
