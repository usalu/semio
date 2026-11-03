# Scene 1991 (wave 2): row-keyed live scene

Nothing compiled by me (no cargo, no builds). `- ready for rebuild: scene-1991 …` is in `🗑️generated/build-en1991/latest.md` and `📌️important/requests.md`.

## What changed
- The 14 `unit-*` projection units and the placeholder entities are gone. Scene entities are the snapshot rows (14 collections, id = row id) plus 16 explicit global entities (`kit::global`):
  - leaf scalar groups (no dependencies): `annex`, `wind-site` (windZone, enVb, terrainCategory, mixedTerrain*, orographyFactor, coastOrIsland, airDensity), `snow-site` (snowZone, altitude, enSk), `building` (depth, width, height, storeyCount + the building placement), `bridge` (structureKind, bridgeSpan, bridgeLaneWidth, bridgeTandemX + the bridge placement), `report` (sink: 20 scalars no projection reads, e.g. all `assumed*`, and every subjectless check row nobody binds);
  - drawing/aggregating entities: `envelope`, `bridge-deck`, `wind-zones`, `wind-qp`, `thermal`, `fire`, `crane`, `silo`, `selfWeight`, `deflection`. Thermal, fire, crane and silo own their scalars.
- Ownership is decoupled from the scene id: `Out::own(owner)` (replaces `Out::enter(unit)`) sets the owner cursor and says whether the part wants it. Every row loop starts with `if !out.own(&row.id) { continue; }`, global emitters with `out.own(global::X)`. Running totals are advanced before the guard (self-weight stack). `wind-zone-<L>` belongs to the first wind face of that zone, else to `wind-zones`; authored bridge lanes own their surface, label, UDL and LM1 tandem wheels, derived lanes belong to `bridge-deck`.
- `kit::entity_dependencies` / `dependencies_of`: declared, acyclic graph (leaf groups, then rows, then aggregates). Rows read leaf groups only (floor: annex, building; roof: annex, building, snow-site; first face of a zone: wind-site, annex, building + the other faces of the zone; lane: bridge, annex; ...). Aggregates read the rows whose check rows they bind (`bridge-deck`: lanes, vehicles; `thermal`: thermal sections; `wind-qp`: all faces; `deflection`: roofs, faces, surfaces, bridge wind cases, slender elements; `selfWeight`: all layers). The first roof's form/pitch/parapet is a stamp, not a dependency, so the roof's own dependencies (snow site) do not leak into wind and thermal.
- `kit::stamps` (the kit `layout_stamps`): index and running-total dependent poses (floor index, self-weight bottom z + pose, roof index + pose, accidental index/count, stage slots, surface span, slender element origin, explosion room origin, section index, crane/silo pose, faced wind letters) plus the dependency id list of every entity, so adding or removing a row refingerprints exactly the entities that list it.
- Shared (additive, defaults unchanged for every other artifact): `SceneKit::scalar_owner(field)` files a top-level scalar under its entity instead of the global bucket every fingerprint contains; `SceneKit::inert_reference(collection, field)` stops `roofSurfaces[].face` from filing a surface row under its wind face. Logged in `requests.md` as a shared fix.
- `kit::scene_report` / `report_owner`: check rows with an empty subject are filed by check-id prefix under the entity that reads them (`en1991.2.` -> bridge-deck, `.3.` crane, `.4.` silo, `.1-2.` fire, `.1-5.` thermal, `.1-3./1-4.` deflection, rest -> report), applied in `live_scene_of`. Without it, a global check row changing on any scalar edit would change the report stamp of every fragment (all fragments read subject `""`). Module code never reads by subject for these rows, so the full path keeps the raw report (the law test compares raw-full against remapped-incremental).
- Scene-wide parts moved to `kit::finish`: the unit-weight legend (from the layers present) and the traffic position track (from the tandem layers present). The fire time track stays owned by `fire`.
- Tests: `🧪️tests/🔬️live` rewritten: 120-step incremental==full law over 24 edit kinds (all scalar groups, add/remove of every row kind incl. wind faces by zone, lanes, stages, surfaces, vehicles, bridge wind, slender, impacts, explosions, sections, roofs, layers; placement drags of rows and globals); `one_leaf_edit_rebuilds_exactly_the_dependent_fragments` (36 edit kinds, exact miss count each, same frozen report so only document-driven rebuilds count); untouched document = 0 misses; acyclic dependency graph naming only entities; drag budget unchanged. Unit tests: dependencies, second face of a zone, every snapshot scalar has an owner that is an entity (reads the schema JSON), subjectless check remap.

## Exact rebuild counts (misses = rebuilt fragments + 1 for the merged global part; fixture `counted()`: 16 globals + 22 rows)
| edit | rebuilt | misses |
|---|---|---|
| floor area / floor drag | 1 | 2 |
| top / bottom self-weight layer thickness | 2 (layer, selfWeight) / 3 (+ layer above) | 3 / 4 |
| later roof pitch (deflection reads all roofs) | 2 | 3 |
| first roof pitch | 9 (roof, envelope, wind-zones, wind-qp, thermal, deflection, 3 zone-owning faces) | 10 |
| second face of a zone / face of a lone zone | 4 / 3 | 5 / 4 |
| roof surface, lane width, vehicle, section, slender element, bridge wind case | 2 | 3 |
| stage duration (slots of all stages) | 2 | 3 |
| impact, accidental case, explosion room | 1 | 2 |
| bridgeSpan / bridgeLaneWidth (crane default pose) | 8 / 9 | 9 / 10 |
| scalar no projection reads (`assumedBridgeUdl`) | 1 (report) | 2 |
| silo height, crane x, fire duration, tMax | 1 | 2 |
| enVb / enSk | 10 / 5 | 11 / 6 |
| annex / building depth / building drag | 21 / 23 / 22 | 22 / 24 / 23 |
| added floor / removed floor | 1 / 0 | 2 / 1 |
| added face of a new zone / stage / lane / slender element | 4 / 3 / 2 / 2 | 5 / 4 / 3 / 3 |

Closure verified with a Python model of the dependency table (`🗑️generated/scene-1991/closure.py`, delete with the generated folder). Before: any scalar edit or any edit of a row without a registered entity rebuilt all 14 units.

## Known imprecision (honest limits)
- Dependency keys are transitive in the projector, so aggregates re-key when any listed row re-fingerprints even if the edited field is not read (`wind-qp` on a face's `cS`, `deflection` on every row it binds). They are real binding dependencies (utilisation over those rows), not over-declarations.
- A leaf group is the granularity of a scalar dependency: `enVb` rebuilds everything reading any wind-site scalar.
- Tests freeze the report to isolate document-driven rebuilds; in production a check result change adds the fragments of that subject and its dependents (key includes the report stamp of subject + transitive dependencies + `""`).
- `stage_slots`, `surface_span`, `element_origin`, `room_origin` are O(n) per entity inside `stamps` (O(n^2) for n rows of those kinds; n is small for them, floors are index-only).

## Unverified (needs the first compile)
Everything. Likeliest rustc findings: `Which::Row(*collection, i)` / `&[][..]` lifetimes in `kit::Rows`, or-patterns with consts in `extra`, closure-to-`fn` coercion inside the 36-tuple array of the rebuild test, `elided_lifetimes_in_paths` if the workspace denies it, `Out::wants` unused warning if a module path changes. Hand-computed miss counts: if one differs, compare with `closure.py` first, then check whether a fragment reads a field its entity does not stamp (that would be a law-test failure too).

## Files
- Shared: `⚖️…/📕️norm/🎬️structural-scene/🎯️projection/🧲️scene-kit/🦀️.rs`.
- Scene: `🏋️en1991/…/🎬️scene/🦀️.rs`, `🧰️kit/🦀️.rs`, `🏢️envelope`, `🌬️wind`, `❄️snow`, `🏗️gravity`, `🔥️thermal-fire`, `💥accidental`, `🌉️traffic` (split into `deck_body`, `lane`, `tandem`), `🧱️collections`, `🫙silo`, `🌪️wave3`, `🌀️deflection`, `🎞️tracks`, `🧪️tests/🔬️live`, `🧪️tests/🔬️unit` (4 tests added/changed).
- Ticket: `📌️important/requests.md`, `🗑️generated/build-en1991/latest.md`, this report.
