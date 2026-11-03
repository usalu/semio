# Handcrafted staffage in the architecture course clips: summary

The ticket is closed as of 2026-11-01. Abbreviations: S = `🎓️teaching/🏛️architecture/🧍️staffage`, V = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals`.

## Outcome

Every person, device, piece of furniture, machine, vehicle and building component in the architecture course clips is now a cast of the handcrafted staffage archive S. The ad-hoc drawings are deleted:

- `🫟️silhouettes`
- `person_glyph`, `seated_person_glyph`, `object_silhouette`, `lamp_glyph`, `radiator()`, `respiration_parts`
- the activity SVGs
- the primitive boilers, fans, pumps, valves, hobs, blinds, heat-pump and DEC parts, and the derrick

Energy clips use the animated `thermal` view, driven by staffage clips, transient environments, timelines and transport layers. The optimization comfort clips follow the same rule. Diagram icons stay glyphs: sun, flame, arrows, meters, clock, thermometer and the abstract `energy_tank` gauge. The guard `V/🧪️tests/🐍️staffage_only` enforces the rule.

## Pieces

- **Bridge:** the TS cast exporter `S/🔨️modules/📤️export/🎭️cast` has a schema, the views `elevation|plan|front`, a 600-frame cap and a content-keyed cache. Python `staffage`/`staffage_motion`/`fitted_staffage`/`pendant_lamp` use a lazy parser with a bounded cache. See `contract.md`, `bridge-ts-report.md`, `bridge-py-report.md` and `perf-p1-report.md`.
- **Manifest:** `V/🧍️staffage/🔣️.json`, one cast per device and clip.
- **New handcrafted assets** (the catalog check passes 144/144):
  - humans: standing adult, seated office worker, sleeping person, person cooking; plan views for humans and the car
  - a new 🔌️appliances category: laptop, monitor, pendant lamp, toaster, vacuum, fan heater, split AC, server rack, extractor hood, gas boiler, circulation pump, hot-water cylinder, MVHR unit, duct fan, decentralised HRV fan, shower head, room thermostat, thermostatic radiator valve, gas hob, electric kettle
  - machinery: oil pumpjack, scroll compressor, expansion valve, finned heat exchanger, sorption wheel, plate heat exchanger, evaporative humidifier
  - objects and furniture: cooking pot, chalkboard, crate stack, tealight, operating table, vintage car
  - front views: radiator, split AC, tilt window
- **Schema extensions:** plan geometry and `followTip`, the front viewpoint, and a load clamp in animation.

## Verification

- Visuals Python tests pass, including `integration`, `scenes` and the `staffage_only` guard (`verify-v2-report.md`). Staffage vitest: 994 passed. Catalog check: 144/144.
- Every changed scene was rendered and inspected by the render-audit and migration agents (`render-audit-r1..r3`, `migration-*`, `polish-f2`, `verify-v1/v2`).

## Known leftovers, not caused by this ticket

- `repo dependencies ensure` refuses with "immutable input capture work refused". Staffage vitest runs through ticket-local alias configs (`q2-report.md`).
- Optimization `$schema` draft assertion failure (pre-existing).
- BIM information and consistency clip test suites run for more than 9 minutes and were not fully verified. Their staffage call sites were rendered.
- The shower spray and steam are scene-drawn effects, because cast transport plumes inflate the bounds (`polish-f2-report.md`).

## Round 3 corrections

Added by worker `doc` (W20) on 2026-11-03. The sections above are the round-2 record and are not rewritten. Open items are in `r3-baseline.md`.

- **OI-74, meters are casts, not glyphs.** The Outcome sentence "Diagram icons stay glyphs: sun, flame, arrows, meters, clock, thermometer and the abstract `energy_tank` gauge" is wrong for meters. Gas and electricity meters are device casts (`gas-meter`, `electricity-meter`, see OI-03). The diagram glyphs that remain are the sun, flame, arrows and clock, the thermometer (`thermometer_glyph`, ruled in `r2-coordinator-rulings.md`, OI-41), and the abstract gauges (`energy_tank`, OI-42; OI-04 ruled keep). The contract should use the same list.
- **OI-109, the "all" claim.** The Outcome sentence "Every person, device, piece of furniture, machine, vehicle and building component ... is now a cast" is too broad. Read it as: every depicted person, device, furniture, machine, vehicle and building component in the clips is a cast, except the diagram instruments and abstract gauges named in the coordinator rulings (thermometers, `energy_tank`).
