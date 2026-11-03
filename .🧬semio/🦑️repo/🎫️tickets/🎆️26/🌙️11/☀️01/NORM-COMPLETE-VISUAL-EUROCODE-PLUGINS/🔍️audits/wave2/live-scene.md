# Live-scene rollout, wave 2 (nine artifacts)

Nothing compiled by me (no cargo). One `- ready for rebuild: live-scene …` line per artifact is in `🗑️generated/build-en199x/latest.md`. Goldens are 0-byte files in all nine artifacts (en1998: digest files created on first run), so "byte-identical" is vacuous; nothing was blessed.

## Recipe applied to every artifact
- `Out` of the scene kit: scoped by `Part` (`Out::scoped`), owner cursor (annotations and tracks get an `Ownership` row), loop guards (`out.wants(id)` or `out.own(collection, id)`), `into_fragment` (entities, annotations, legends, tracks, owners, bounds), free `finish(scene, description)` (layers, legends in front of the fragment field legends, description, plus tracks that scrub scene-wide layers/fields).
- Root: `<Code>Kit: SceneKit` (`entities_of`, `entity_dependencies`, `fragment`, `finish`, `layout_stamps`), `fragment(input, part)`, `project` via `project_whole` (or `project_parts` for en1991, own merge for en1992 because `build` also needs the handles), `live_scene!(pub mod live = Kit<..>)`, `live_scene_of(document, report)`.
- Editor and viewer `scene_of`/`render_with` call `live_scene_of`; unused `placements_of`/`revision_of` imports removed.
- Tests `🎬️scene/🧪️tests/🔬️live/🦀️.rs` (mounted as `live_law`): incremental equals full on 60 random edit sequences (own rng, per-artifact edit arms incl. add/remove entity, scalar edits, placement drags), rebuild count after one edit (`misses <= N`), drag-preview p95 < 12 ms (x20 in debug) on 60 to 100 entities.

## Per artifact
| artifact | entities (owners) | notes |
|---|---|---|
| en1996 | walls (sub-entities `wall:tag` project with the wall) | bond unit budget is now per wall (`UNIT_BUDGET`), it was shared across walls and therefore not entity-local; fire track only when a wall has `fireReiMin`, owned by nobody (global) |
| en1995 | members, connections, cases | fire track added in `finish` from the merged scene (char layers of members and cases) |
| en1994 | beams, columns, slabs | mode entities exist only if a column mode and a girder mode both exist: the flag `modes=bool` is part of every beam/column fingerprint via `layout_stamps`; mode track added in `finish` |
| en1991 | 14 projection units (`unit-envelope` … `unit-deflection`) plus empty placeholder entities for every row and the placed singletons (they only keep the rows out of the global stamp) | module groups own their scene entities; fingerprints = global scalars + `layout_stamps` (poses and ids of the read collections); units that look checks up by check id (thermal, fire, bridge, crane, stages, surfaces, silo) depend on the rows of every collection; fire/traffic tracks are owned by their unit |
| en1992 | members, anchors | `build()` merges one fragment and takes the handles; `live_scene_of` uses `stored_placements` (poses are inline leaves); fire/creep/shrinkage tracks added in `finish` from the field channels present |
| en1993 | 21 collections (`rows_of`), `Out::own(collection, id)`; member action glyphs belong to their member | `entity_dependencies` completed: member now depends on `dependents(..)` (fatigue, fire, bridge, tower, crane, storey, restraint, built-up, weld, hollow, frame bars) and frame bars on their nodes (nodes on incident bars); layout stamp of a member carries the indices of its action rows (glyph stagger follows the row index); mode track in `finish` |
| en1997 | 11 collections + `unit-groundwater`, `unit-settlement` | layer fragments carry their top depth and the site bounds in `layout_stamps`; scene bounds = fragments united with `layout.aabb()` in `finish`; settlement unit depends on every footing |
| en1998 | 9 collections | seismic joints belong to the first building of the pair; mode and phase tracks in `finish` |
| en1999 | 7 collections | fire scenarios (two loops), fatigue, sheets, shells, members with their weld runs; mode track in `finish` |

## Shared fixes (logged in requests.md too)
1. `Kit::fingerprints`: the global stamp no longer contains the whole `doc.placements` Debug. A single drag step changed every entity key, so a drag preview rebuilt the scene (and the 12 ms test could only pass on tiny scenes). Placement rows name their entity and are in its texts; derived poses are in `layout_stamps`.
2. `projection::project_parts::<K>` (additive): whole scene exactly as the projector assembles it (global fragment, then each entity in `entities` order). Used by en1991, whose scene entity ids do not follow entity order, where `canonical_order` would sort differently from the incremental path.

## Behaviour changes to expect
- Scene entity order is entity-major (per wall/member/…) instead of module-major for multi-entity documents (annotations and tracks sorted by owner). Unit tests that index into `scene.entities` by position may need a look; tests that look up by id are unaffected.
- Report slices: a fragment sees the rows of its own subject, the global subject `""` and its dependencies only. Modules that look checks up by id with a subject outside that set read nothing. The law test is the guard; the known wide readers are listed above (en1991 id-based units).

## Not done / risks (unverified by the compiler)
- Everything. Most likely compile risks: `Option<Aabb>` from `Aabb::of_points`, `impl Iterator<Item = &..>` returns in `en1994/tracks`, glob visibility of `Part`/`Fragment` next to en1995 `cases::Part`, `canonical_order`/`merge` imports in en1992.
- Coarse fingerprints: any scalar field, any row that names no registered entity (sections, materials, load cases, grades, layers in en1992/en1993) dirties every fragment. Registering those rows as entities with deps is the next step if the rebuild test is too weak.
- Rebuild-count thresholds in the tests are artifact specific (4 for the entity-local artifacts, 8 en1997, 12 en1991); adjust after the first green run.
- en1992 snapshot just gained a `placements` collection next to the inline `placement*` leaves; the live path keeps `stored_placements` (owner decision pending).
- Diagrams and handles stay outside the kit (`Kit::diagrams/handles` return empty); editor `handles_of` is unchanged.
