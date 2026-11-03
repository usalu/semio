# R2 cross-clip depiction consistency audit (agent cc)

Read-only audit. No clip, manifest or catalog file was changed. Scripts are in `🧰️cc/` (group.py, scan.py, lit.py, dups.py, unused.py).

Method: grouped the 🔣️.json casts by catalog asset; AST-scanned all 98 energy clip scripts under `⚡️energy/**/🎬️clip` (tests skipped) for staffage calls (182 call sites); read the sizing expression (`unit=`, `height`, `width`, `staffage_height`) at each site; checked the catalog for real sizes (standing-adult 1.69 m, seated-office-worker 1.411 m, laptop 0.275 m wide, panel-radiator 0.71 m). Static only: no renders were compared. Scene context was read where the expression is a literal.

Test output: `m9_tests.sh ...⚡️energy...🧪️tests.🐍️staffage_only` gives **Ran 25 tests, OK** (`🗑️generated/cc/guard-run.txt`, deleted after this report).

## Findings table

Severity: H = visibly different the same asset in one series, M = clearly different scale or span across scenes, L = drift or cosmetic.

| Asset | Casts | Inconsistency (file:line) | Sev | Suggested fix |
|---|---|---|---|---|
| laptop | `cooling-internal-gains-laptop`, `-laptop-closed`, `shared-laptop-winter`, `shared-watt-laptop`, `cooling-heating-comparison-laptop(-summer)`, `heating-calculation-laptop` | Laptop width is 0.275 m, person 1.69 m, so the physical ratio is 0.16. Measured ratios: heating B5 icon `shared-laptop-winter` unit 2.4 next to person unit 0.8 (`🔥️heating/.../👥️internal-gains/🐍️.py:619,622`) gives 0.49, about 3x. Cooling Beat4 laptop 2.6 next to standing 0.6 (`❄️cooling/.../👥️internal-gains/🐍️.py:790,793`) gives 0.70, about 4.3x. Cooling Beat3 uses `unit * 1.5` (L588) and literal 1.5 (L650, L664). Demand recap uses 1x metre for the same `shared-laptop-winter` (`📊️demand/.../🔁️recap/🐍️.py:216`). The same winter laptop is 2.4x in one heating scene and 1x in another. | H | One laptop scale constant per series, derived from the room metre. Icon enlargement, if wanted, is one named constant applied to every laptop, not a per-scene literal. |
| standing-adult (`cooling-internal-gains-standing`) | 1 cast, 4 unit literals | Units 0.68 (Beat2 single, L481), 0.47 (Beat2 grid, L439), 0.19 (grid, L500), 0.6 (Beat4, L790). The same file derives every other unit from `room["h"]/2.5` (L133, L143, L582, L911). Beat2 single 0.68 vs Beat4 0.6 is a 12% jump inside one series. | M | Replace the literals with `room["h"]/2.5`, or with named icon-size constants that carry one physical meaning. |
| shared-person-pathlines (standing-adult) | `shared-person-pathlines`, `ventilation-need-occupant-mirrored` | Same mini-room family with the same `fy=-0.55` in `_use_glyph`: passive house `_use_cast(..., 0.77)` (`🌬️ventilation/.../📦️volume/🐍️.py:412`) and operating room `_use_cast(..., 0.62)` (L427). Both are standing adults at the same floor. 0.77 vs 0.62 is 24%. | M | One standing-person height constant (about 0.70 scene units) in `_use_glyph`. Per-kind code passes only x. |
| seated-office-worker | `ventilation-volume-office-worker` (buero, L415, height 0.62), `ventilation-volume-student` (klasse, L419, height 0.42, mirror) | Same asset, same mini-room, 32% smaller. Manifest also differs in `transportAgeS` (3 vs 2). | M | Use the same height. If a child is meant, add a real child asset. Do not keep the same asset at two sizes. Align `transportAgeS`. |
| shared-person-winter (standing-adult) | 1 cast, heating and demand series | Heating sizing is mixed: 0.7*storey/2/h (`🔥️heating/.../⚖️heating-comparison/🐍️.py:138`), `house["h"]/2/2.5` (`👥️internal-gains/🐍️.py:251`), literal 0.8 in B5 icon (`👥️internal-gains/🐍️.py:619`), 0.7*room_h/h (`🧮️calculation/🐍️.py:700`), room_h/2.5 (L1205), `_metre(H)` (demand recap L199), `metre` (conduction L252). Only the B5 literal is off-rule. 0.7*room_h/1.69 and room_h/2.5 differ by about 3.5% in calc. | M | Replace the B5 literal 0.8 with the storey fraction used in the other heating clips. Keep one helper for 70% of storey height. |
| panel-radiator | 8 casts | Six sizing rules for one asset: literal 1.2 (`🧲️physics/.../🧲️fundamentals/🐍️.py:1762`), `_ROOM_METRE*scale` (L122), `fe_metre` (L1917), `room['h']/2.5` (L2204), `room_h/2.5` (`🧪️practice/🐍️.py:153`), `hs['w_height']/2/2.5` (`🏭️primary-energy/🐍️.py:143`, `🏷️certificate/🐍️.py:262`), `_metre(H)` (recap), `metre` (system-losses L462, regulations L423, demand-power L256). Manifest `scale: 20` is set only on the two ventilation-natural radiators, which is the only drawing-scale override on a radiator. | M | One radiator metre helper. Replace literal 1.2 with it. Remove `scale: 20` unless the exporter needs it, or justify it in the manifest. |
| panel-radiator (`heating-calculation-radiator-mirrored`) | `heating-calculation-radiator` and `-mirrored` | Identical except `mirror`. Same thermostat-cycle clip, fps 15, same environment and span. | L | Merge to one cast. Apply mirror at the call site. |
| panel-radiator (`optimization-room-radiator`) | `shared-radiator-winter`, `optimization-room-radiator` | Identical except `mirror`. | L | Merge to `shared-radiator-winter`. Mirror at the call site. |
| split-ac-indoor-unit | `optimization-room-cooler`, `optimization-extrema-cooler` | Same pose, environment and span. Differ by mirror vs `viewpoint: front`. | L | Keep both only if the viewpoints are meant. Otherwise merge. |
| pendant-lamp (`shared-lamp-winter`) | 1 cast, 4+ sizings | Room-derived unit (`👥️internal-gains/🐍️.py:256`), literal 0.9 in B5 icon (L624), `0.3/staffage_height` (`🔁️recap/🐍️.py:212`), `height/_staffage_height` (`🧲️fundamentals/🐍️.py:548`). Warm-up cast uses `B4_LAMP_UNIT = 3.2` (`🔥️heating/.../👥️internal-gains/🐍️.py:520`). | L | Name one lamp scale from the room metre. Use it everywhere, including warm-up. |
| standing-adult (`physics-fundamentals-clerk`) | Span only | `thermalSpanK` 78 on the clerk. The other 12 thermal standing-adult casts use 25. | M | Set to 25 unless the 78 is needed for the pointing pose. |
| finned-heat-exchanger | `cooling-summer-strategy-heater` (70), `physics-fundamentals-evaporator` and `-condenser` (25) | Same asset, two spans, two series. | M | Use 25 everywhere, or state the span per clip in one place. |
| all energy casts | 9 casts with span 70 or 78 | Physics (78): vacuum, kettle, shower, clerk. Cooling summer-strategy (70): five casts. All others: 25. The same thermal colour means different temperatures between series. | M | Owner decision: one span (25) for the series, or a span shown in each clip caption. |

## Same-asset merge candidates (asset, pose, view, environment and span match)

- `shared-person-pathlines` and `ventilation-need-occupant-mirrored`: identical except `mirror`. Merge (mirror at call site).
- `heating-calculation-radiator` and `heating-calculation-radiator-mirrored`: identical except `mirror`. Merge.
- `shared-radiator-winter` and `optimization-room-radiator`: identical except `mirror`. Merge.
- `ventilation-volume-office-worker` and `ventilation-volume-student`: identical except `mirror` and `transportAgeS` (3 vs 2). Merge after aligning transport.
- Not merged: `shared-radiator-front-winter` and `shared-radiator-thermostat-front` (viewpoint and clip differ). `shared-laptop-winter` and `heating-calculation-laptop` (pose `open` vs `open-idle`). The catalog poses have the same lid angle, but the load state differs, so keep separate unless the owner wants one.

## Thermal vs dark-line in energy clips

- No asset is thermal in one energy clip and dark-line in another. Standing adults are thermal everywhere in energy. The dark-line standing variants (`shared-person-relaxed-line`, `excel-program-occupant`, `bim-consistency-decider`) are not used by any energy clip.
- Every dark-line cast used by an energy clip is in `rule_b.allowed` with a reason. Eight casts, all present in the manifest:

| Cast | Used at | Allowlist reason |
|---|---|---|
| shared-desk | heating-comparison L151, internal-gains heating L254, calculation L703, recap L215 | furniture |
| cooling-internal-gains-office-chair | cooling internal-gains L146 | furniture |
| cooling-heating-comparison-table | heating-comparison L144, L147 | furniture |
| ventilation-volume-blackboard | volume L418 | furniture |
| ventilation-volume-crates-low / -half / -high | volume L407 | storage |
| demand-primary-energy-tree | primary-energy L217 | landscape |

- The guard passes, so the allowlist is complete for these clips. Nothing outside the allowlist was found.

## Open items and caveats

- Static analysis. Scale ratios use catalog heights and the literal or room-derived expressions. No frames were compared, so the physical meaning of icon-size literals (grid icons, B5 bar icons) is a judgement, not a measurement.
- Manifest keys `scale`, `mirror`, `viewpoint` and `fps` are in the cache key. Whether the exporter applies `scale` (drawing denominator) was not verified in the TS exporter.
- `_ROOM_METRE` and `room_h` values per scene were not resolved. Fleet-wide, the expressions are consistent in intent (room metre) but not in value.
- Out of energy scope: `shared-person-relaxed-line` and `excel-program-occupant` are the same figure in dark-line, differing only by plan viewpoint. Not reviewed further.
