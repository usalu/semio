# crate budget K (wave 2): en1998-mutations 95 % and en1998-scene-core 92 %

No cargo/rustc, no git writes, `freeze.txt` absent. Scripts (idempotent): `🧰️scripts/crate-budget/k_mutations_text.py`, `k_scene_geometry.py`; backup of the old codec in `🗑️generated/crate-budget-k/text-before.rs`.

## Mutations text codec (179 KB monolith) per family
The codec was hand-edited (no generator owns it; schema-evolve only touches a `use model::{..}` line that does not exist here). Now:
- `-mutations-text-kit` (`🚪️io/📝️text/🧬️mutations/🧰️kit`): `Args`, `required`, `enc_str/dec_str/enc_json/dec_json`, `opt_index`, `tokenize_args/parse_args`.
- One crate per mutation leaf family, `-mutations-text-{buildings 96 kinds, members 66, bridges-towers 60, foundations 54, silos-tanks 22, seismic-slopes 8, tie-beams 7, placements 6}`: per kind `print_<kind>(&Payload)` and `parse_<kind>(&Args)`; each depends only on kit and its `-mut-<family>` crate (no cycle with the enum).
- `-mutations` text module keeps `COMPONENT_GRAMMAR_*`, `decode_en1998_mutation_json`, the `OpText` impl and thin `print_en1998_mutation`/`parse_en1998_mutation` dispatchers (one match arm per kind, routed to the family crate by keyword); public API and paths unchanged. The ballast of 300 unused module imports in `mutations_codec` was dropped.
- Result: `en1998-mutations` 95 % -> below 75 % (about 152 KB), largest new crate buildings 71 KB.
- Open (generic tool): a new mutation kind now needs its print/parse pair in the family crate plus the two dispatcher arms; `schema-evolve` `clone_registries` would clone the dispatcher lines only.

## Scene core
`-scene-geometry` (root `🎬️scene/📐️geometry`): kit, layout, tracks, horizontal elements, ground and bridges, handles, walls, `SceneInput` and the shared extern/re-export header (these form one dependency cycle: handles <-> ground-and-bridges/horizontal-elements/walls, kit -> tracks). `-scene-core` keeps buildings, vessels-and-towers, joint-rows and the live-scene contract and re-exports every moved module and name (`kit`, `layout`, `handles`, `horizontal_elements`, `Layout`, `SceneInput`, ...). `pub(crate)`/`pub(super)` became `pub` in the moved files. `scene-core` 92 % -> below 75 %.

## Mutation file-count splits (needed for the 85 % target)
`mut-members-joints-more` (28 files) -> `-members-joints-rest`, `mut-bridges-pier` (28) -> `-bridges-pier-more` via `j_mut.py`; the split-refs scan found `remove_joint` inverse needing `crate::insert_joint`, so `insert_joint` moved into `-rest` and is re-exported from `-more`.

## Registrations
Workspace members and `workspace.dependencies`, `📜️script.ts` package lists, Cargo.toml dependencies of `-mutations` and `-scene-core`, Cargo.lock blocks in the en1998 lock (and hub, fixture-sweep, bridge for the codec crates).

## Gates
- Source-budget: 0 crates over 100 %; no en1998 crate above 85 % except `en1998-diff` 89 % (3563 of 4000 lines, 2851 of them generated twins `🟦️.ts` 1083, `🔗️.graphql` 806, `🛰️.proto` 825, `🔣️.json` 137 plus 709 spliced Rust lines). Not touched: a fix needs the schema-codegen (macro for the 33 `KeyOf` impls or fewer twin lines).
- `check-cargo-authority.py`: 31 crates, 0 errors. tomllib over 1141 Cargo.toml/Cargo.lock (norm, hub, fixture-sweep): 0 bad. `e_test_support.py --check`: 0 changes. split-refs scan: only the known false positives (io macros, app-surface).
- `verify build-owners sync` through heavy.sh: started, still queued behind a build in heavy.sh when this report was written (not confirmed); no crate was added or removed after it started.

## Unverified by compiler
Orphan/visibility of the pub widening in the moved scene modules; glob `use super::*` in the three remaining layer modules now resolving through the core re-exports; trailing `unused_imports` in family codecs (`#![allow(unused_imports)]`); `test-support` was not needed on the new crates (no serde derives there).

## Ready
`- ready for rebuild: crate budget K` appended to `build-en1998/latest.md`.
