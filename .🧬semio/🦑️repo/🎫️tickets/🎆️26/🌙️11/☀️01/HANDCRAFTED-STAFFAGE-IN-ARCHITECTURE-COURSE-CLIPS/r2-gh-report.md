# r2-gh report: rule A noun list derived from the catalog

## Change
- `V/🧪️tests/🔣️staffage_only.json` `rule_a.object_nouns` is now data, not a regex: `catalog` + `slug_depth` (`*/*` below `S/🗂️catalog`), a `stop_words` list (adjectives, materials, prepositions, structural or geometric words), and `extra_nouns` (nouns with no asset yet: derrick, occupant, tank, grille, plant, thermo, thermometer, clock, sink, cooker, evaporator, condenser, device, figure, cooler, walker, sleeper, student, cook, blackboard, lattice, bulb, shelf).
- `V/🧪️tests/🐍️staffage_only.py` builds the regex: `catalog_slugs`, `head_nouns`, `derived_nouns`, `object_noun_pattern`. Nouns now include collector, diffuser, humidifier, cylinder, pumpjack, scooter, etc. (148 asset slugs).
- New tests: head nouns of probe slugs (`rule_a.head_nouns`: solar-thermal-collector, gas-meter, ceiling-diffuser, hot-water-cylinder, evaporative-humidifier, ...); extra nouns must not be derivable (stale extras fail); every stop word must occur in a slug.
- New `rule_a.sources` cases: `_collector`, `_meter_box`, `_diffuser`, `_humidifiers`, `_cylinder` built from primitives are flagged; the same names wrapping `fitted_staffage`/`staffage` are not; `_heat_flow` and `_solar_gain` are not flagged (stop words).

## New hits over the whole course
None draws a real-world object from primitives. All four were allowlisted with a reason:
| File:line | Function | Why |
|---|---|---|
| `⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:322` | `_beam` | light ray of the physics lens (matches glulam-beam) |
| `🎯️optimization/🎬️clip/🎨️visuals/🏠️room/🐍️.py:1192` | `ramp_strip` | colour-ramp legend (matches wheelchair-ramp) |
| `🎯️optimization/🎬️clip/🎲️sampling/🐍️.py:217` | `_plane_group` | axis lines of a coordinate plane (matches london-plane) |
| `🏗️bim/🎬️clip/🎨️visuals/🏛️savoye/📋️plan/🐍️.py:496` | `_plan_ramps` | IFC ramp footprints of the Savoye plan (structure from model geometry) |

No solar collector or other asset-matching primitive helper was found, so no cast switch was needed. The cooling summer-strategy file (agent sc) is no longer flagged by rule A.

## Result
`staffage_only`: Ran 22 tests, OK (final run, after sc's collector cast reached the manifest).

## Open
- Transient: `test_every_literal_cast_a_clip_or_the_library_draws_is_in_the_manifest` failed in the first full run for `cooling-summer-strategy-collector` (`❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py`): sc's in-flight work, the cast is not yet in the manifest. Not mine.
- The full module takes about 6 minutes (the rglob scans over the course).
