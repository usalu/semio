# crate-budget C (wave 2)
Scripts: `🧰️scripts/crate-budget/{scene_members,masonry_formulas,masonry_catalogue,en1993_move_tests}.py` (+ `model_twin_leaves.py`, `split_mounts.py` from A/B). No cargo/rustc run; all new crates registered (artifact `Cargo.toml` member + workspace dep, parent manifest dep, `Cargo.lock`s, `📜️script.ts`). Checks: `source-boundary-scan.py` no OVER; `check-cargo-authority.py` 0 errors; `verify build-owners sync` passed.

## Results (bytes / files / lines before -> after)
- en1999-model: 174624 B / 3966 lines -> 93708 B / 1287 (4 facet twin crates `-typescript|-graphql|-json-schema|-proto`, `SNAPSHOT_LEAVES` -> `<crate>::LEAF`).
- en1994-mut-beam-design: 31 files -> 16 (+ `-bridge` sibling with 16 files; five leaf mounts moved, old crate `pub use`s them).
- en1996-masonry: 230707 B / 19 files / 3865 lines -> 99864 B / 8 / 1777.
  - `en1996-masonry-formulas` (schema/📐️masonry-formulas): `📏️factors-and-geometry`, `🔥️fire-and-design-effects` (pure, depend only on tables/model/compliance) plus the shared helpers `loc, clause, wall_subject, wall_path, load_case_path, concentrated_path` made `pub`.
  - `en1996-masonry-catalogue` (schema/📇️masonry-catalogue, 140 KB): the whole `📇️catalogue` dir; `document_cases`/`document_case_results` are now `pub` (was `pub(super)` across the boundary). Dependency order: tables, model -> formulas -> catalogue -> masonry facade (assembly: conformance, evaluate-wall, detailing, units); the cycle is gone (catalogue never calls the assembly).
  - the superseded-forms test scans all three source roots (`SOURCE_ROOT`, `CATALOGUE_SOURCE_ROOT`, `FORMULAS_SOURCE_ROOT`).
- en1991-scene: 249475 B / 4007 lines (over) -> facade 59519 B / 711 + `en1991-scene-members` 191391 B / 3320 (new dir `✳️any/🎬️scene-members`).
- en1993-scene: 243306 B / 3687 -> facade 82156 B / 988 (diagrams, entity-diagrams, annex-diagrams, SceneKit, live scene, tests) + `en1993-scene-members` 161971 B / 2725. The unit tests that reached `eigen`/`detail_sketches` internals (and `🔮️oracles/🌀️modes.py`) moved next to those modules (`🧪️tests/🔬️unit`, `#[cfg(test)] mod unit`).

## Scene split design
`<code>-scene-members` = kit, layout, `SceneInput`, `fragment`, all module families and drag handles (items `pub(crate)`/`pub(super)` of top-level modules are `pub`); the facade `<code>-scene` keeps `diagrams`, the `SceneKit` impl, `live_scene!`, `live_scene_of`, `project`, `summary`, golden/law tests, and `pub use`s the old API (`Layout`, `Placement`, `handles`, `layout`, `entities_of`, ...). `SceneKit`/`live_scene_of` bodies are unchanged. `extern crate ... as members|scene_members` makes `super::kit::...` paths in diagrams resolve through private `use`.

## Not done / follow-ups
- A third `-scene-kit` crate was not cut: kit calls up into `collections/wave3/gravity/tracks` (stage_slots, surface_span, element_origin, room_origin, weight_legend, traffic) and the handles<->modules graph is cyclic. Members stay at 3320 / 2725 lines (the en1991 one is near 3500): next cut would first move those up-call helpers into kit, then split handles+modules from kit.
- en1991 tests that read internals were not needed (they use the public API only); en1991 unit tests stay in the facade.
- Compile-untested: expect fixes for `private_interfaces` warnings, unused imports in the facade roots (`use <alias>::{...}` lists), and any item that was private (not `pub(crate)`) but used across modules via `use super::*` globs. Owners must edit the moved code in the new dirs (`🎬️scene-members/`, `📐️masonry-formulas/`, `📇️masonry-catalogue/`, `mut-beam-design-bridge`); regeneration scripts that append en1994 `beam-design` mounts will target the old crate (it `pub use`s the bridge sibling, add new mounts where the budget allows).
- Remaining near-budget (not touched): en1993-checks-special 234 KB, en1995-scene 230 KB, en1998-mutations/en1993-mutations ~231 KB.
