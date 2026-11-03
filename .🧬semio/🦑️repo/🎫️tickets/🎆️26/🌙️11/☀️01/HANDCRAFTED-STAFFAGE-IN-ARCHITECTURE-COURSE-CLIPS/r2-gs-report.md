# r2-gs: Rule C, name-independent structural guard

Paths: `V` = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals`, `A` = `🎓️teaching/🏛️architecture`.

## Heuristic evaluated
- Whole-function counting (3 or more distinct primitives, no staffage call, no chart call) gave 27 hits, but 14 of them were whole 100-300 line `construct` scenes. A scene is not an object, and one staffage call anywhere exempts the whole scene, so it is both noisy and blind.
- Chosen: composition units. A unit is a variable (everything assigned or `add`ed to it), a `VGroup`/`Group` call, a `return` value or a `self.add`. Names referenced in a unit are followed to their own units. A function is flagged when some unit reaches 3 or more distinct primitives (the `rule_a` list) with no `staffage_calls` call and no context call (Axes, NumberPlane, NumberLine, Table, BarChart, Graph, plot, c2p, and so on) in its reach, and it is the lowest such unit (no flagged child unit).
- Name independent: `_thing`, `_icon`, `_shape` and inline `construct` blocks are all caught. Probe cases cover each of these.

## Hits over all clips (13 functions) and hand classification
| Function | Verdict |
|---|---|
| cooling internal-gains `Beat2_HumanFactor.construct` (scale_assembly) | chart: activity scale, ticks, data dots |
| cooling system-sizing `Beat3_IsolateAirflow.construct` (balance) | diagram: abstract equality balance. Borderline: beam, pivot and pans are a device shape, but it is a symbol for "equals", not a weighing device. The catalog has no balance asset |
| demand certificate `_document` | UI card with header band and ruled lines |
| physics fundamentals `Beat7_SensibelLatent.construct` | fade-out list of chart parts, no new object |
| optimization index `Board._build` | score rows and badges |
| optimization room `RoomVisuals.plan` | floor plan and compass |
| optimization room `RoomVisuals.room_section.build` | structure, daylight wedge, lamp halo and cone (the lamps themselves are casts) |
| optimization sampling `_orientation_glyph` | mini plan glyph |
| bim consistency `leak_marks` | annotation of the build-up detail |
| excel workbook `Window._title_bar` | UI chrome |
| grasshopper capsule `_icon`, `_table` | legend glyphs, table |
| grasshopper anatomy `_eye` | UI preview glyph |

True ad-hoc objects (person, device, furniture, machine, vehicle): none found, so no cast switch was needed and no manifest row was added. Catalog gap to note: no balance/scale asset (see the balance row).

The 13 legitimate hits are within the 60 limit, so Rule C was implemented.

## Implementation
- `V/🧪️tests/🐍️staffage_only.py`: `call_name`, `target_key`, `function_defs`, `composition_units`, `composed_objects` and 3 tests (cases, every clip is allowlisted, allowlist has a reason and no stale entries).
- `V/🧪️tests/🔣️staffage_only.json`: new `rule_c` block (`minimum_primitives`, `group_calls`, `collect_methods`, `context_calls`, `allowed` with a reason per `file::Class.function` entry, and 12 language-agnostic `sources` cases, positive and negative: group, add chain, name-split parts, self.add, below threshold, staffage, Axes, Table, text only).
- Primitives and staffage calls are shared with `rule_a`.
- Scripts: `🧰️gs/gs_proto.py` (prototype), `🧰️gs/gs_detail.py` (lists primitive lines of one function).

## Tests
See the result block at the end of this file.

## Open
- The balance scale (system-sizing) is the only borderline hit; switch it to a cast if an asset is added.
- Rule C only sees what composes at least 3 distinct primitives. An object of 1 or 2 primitive kinds under a non-object name still escapes (Rule A catches those by name).

## Test results
- `staffage_only` (guard incl. 3 new rule_c tests): 0 failures or errors. The rule_c tests alone ran 3 tests, OK after the allowlist (the full module takes about 3 minutes).
- Whole visuals tests folder (`🐍️`, icons, integration, labels, scenes, section_consumers, sections, transforms, staffage_only): Ran 86 tests, 36 errors. Every error is `RuntimeError: Staffage export failed ... Spread syntax requires ...iterable not be null or undefined` raised while exporting a cast (`shared-radiator-front-winter`, `heating-calculation-radiator`, `shared-person-*`, `ventilation-*-occupant`, ...). It comes from the staffage export tool under concurrent edits by other agents, not from the guard. Not fixed here; needs the staffage-export owner.
