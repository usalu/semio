# Energy Technology — Subsystem Checklist

## Scaffold
- [ ] energy/AGENTS.md, energy/engine project, workspace member, launch.json

## Foundation
- [ ] error, units, num, props (psychrometrics, water, steam, refrigerant, glycol)

## Model + site
- [ ] model (entities, validate), schedule, site (EPW, design days, solar, ground)

## Geometry + envelope
- [ ] geometry, material, envelope, fenestration, solar, daylight

## Zone domain
- [ ] zone_air, room_air, gains, air_exchange, airflow_network, iaq, comfort, controls

## HVAC
- [ ] hvac_topo, ideal_hvac, zone_hvac, terminal, air_system, fans, coils, evaporative, humidity_eq, heat_recovery

## Plant + specialized
- [ ] plant, shw, solar_thermal, refrigeration, electrical, water, faults, curves

## Kernel + outputs
- [ ] kernel, sizing, dispatch, output, meters, metrics, results, economics, sim

## Validation
- [ ] conservation tests, BESTEST/140-style cases, cargo test -p energy_engine
