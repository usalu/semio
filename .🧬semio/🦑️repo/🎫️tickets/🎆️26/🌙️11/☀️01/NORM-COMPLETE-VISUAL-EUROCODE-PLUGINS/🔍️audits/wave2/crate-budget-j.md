# crate budget J (wave 2, blocking builds): en1992-model 191 % and en1997-model 131 %

No cargo/rustc, no git writes, `freeze.txt` absent for the whole run. Scripts: `🧰️scripts/crate-budget/j_showcase.py`, `j_mut.py` (idempotent). Scratch output only in `🗑️generated/crate-budget-j/`.

## Measurement (module closure of the lib root, `include_str!` counted)
| crate | total | schema twin `🦀️.rs` | sqlite codec (+ddl, cells, admission) | showcase fixture |
|---|---|---|---|---|
| en1992-model | 240 KB / 7656 lines | 64 KB / 1244 | 55 KB / ~400 | **121 KB / 6010 lines** (pretty JSON) |
| en1997-model | 196 KB / 5247 lines | 60 KB / 1390 | 66 KB / ~390 | **69 KB / 3457 lines** (pretty JSON) |
| en1993-model | 250 KB / 3131 lines | 123 KB / 2847 | 92 KB / ~280 | 34 KB (compact) |

Cause: not generated twins and not the codec, but the **showcase fixture** `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json`, pulled in by `En19xxSnapshot::full_showcase()` (`include_str!`) and written indented by the en1992/en1997 builders (en1993/en1998 already used `compact=True`). The en1992 fixture alone is 150 % of the line gate. The rest of both models was at 45 to 48 %.

Why no `-model-core`/`-members` split: the record structs and their `impl`s live in the schema module's `🦀️.rs`, which is spliced by name by the generator (`🦀️rust/🟦️.ts`, "leaving every other item untouched") and is the lib root; the sqlite codec is tied to the type by the orphan rule (`impl_norm_artifact_record!`). A domain split would need the schema module cut into several schema modules, which is a schema-codegen decision and not needed once the fixture is out.

## Changes
1. **New crates** `semio-s-artifact-norm-en1992-showcase`, `-en1997-showcase`, `-en1993-showcase` (root `✳️any/🎪️showcase/🦀️.rs`, free function `full_showcase()` returning the snapshot, deps: model + `semio-framework-pack-json`). `En19xxSnapshot::full_showcase()` was removed from the three snapshot files (hand-written impl region, no twin touched). Consumers rewritten to `semio_s_artifact_norm_en19xx_showcase::full_showcase()`: en1992 and en1997 editor (lib dependency, unit test), en1993 editor (dependency), scene tests and checks tests (dev-dependencies). The other six artifacts keep their method (models at most 80 %, en1998-model 80 %); a uniform pass is possible later with `j_showcase.py` (add the key to `CONSUMERS`).
2. **Fixtures compact**: en1992 121 -> 63 KB, en1997 69 -> 37 KB, one line each (`json.dumps(separators=(",", ":"))`, semantics identical). `🧰️scripts/showcase/en1992.py` and `en1997.py` now call `finish(..., compact="🎪️showcase" in rel)` so the builders reproduce the committed layout (check-outcome fixtures stay indented, not embedded).
3. Registered: artifact workspace members and `workspace.dependencies`, `📜️script.ts` package lists, consumer `Cargo.toml`, Cargo.lock of en1992, en1997, en1993 plus hub and fixture-sweep (new block `[name, version, dependencies]`, consumer edges; sorted; bridge does not know the editors).
4. **Pre-emptive file-count splits** (31 of 32 files, 97 %), via `j_mut.py` (half of the kind mounts into `-more`/`-rest`): `en1997-mut-investigation` -> `-investigation-more` (rebalanced by hand: insert/remove pairs of lab-test and layer-water back in the base, because `inverse` files use `crate::insert_*`/`crate::remove_*` of their counterpart, found by the split-refs scanner), `en1993-mut-member-warping` -> `-more`, `en1998-mut-foundations` -> `-foundations-more`, `en1998-mut-bridges-piers-more` -> `-bridges-piers-rest`. `e_test_support.py` run: `test-support` forwarding between base and new crates (`--check` now 0 changes).
5. Fixed a pre-existing Cargo.toml defect found by `featck`: in `en1998-mut-foundations` the dependency lines of `-foundations-water` and `-foundations-sliding` stood inside `[features]`; moved to `[dependencies]` (and now forwarded in `test-support`).
6. `⚖️compliance/🧪️tests/🧱️source-budget/🥒️.feature`: scenario "Embedded fixtures count against the limit".

## Result
- Source-budget: **0 crates over** (was 2). en1992-model and en1997-model dropped below 75 % (no longer listed with `-v`); en1993-model 95 % -> 82 %; en1998-model 80 %; showcase crates below 75 %. All other model crates below 75 %.
- `check-cargo-authority.py` 31 crates, 0 errors. `verify build-owners sync` passed (added=7: three showcase crates and four split crates) right after the registrations; the repeat after the last `test-support`/featck edits (no crate added or removed) waited in `heavy.sh` behind a running build and was stopped, so it is not re-confirmed. tomllib over 1129 Cargo.toml/Cargo.lock in the norm tree, hub and fixture-sweep: 0 bad; the four artifact locks, hub, fixture-sweep, bridge, norm: no new missing dependency names (pre-existing: en1998 lock lacks `mut-placements-tower`/`-bridge-tower` and is unsorted; hub/fixture-sweep/bridge lack `en1995-kernel` edge of schema-timber and `en1996-masonry-tables`; owners must re-lock).
- split-refs scanner: 25 crates, all the known false positives (io macros, opaque globs, app-surface externs); none in showcase or split crates. `#[path]`/`include_str!` targets of en1992, en1997, en1993, en1998 trees: 0 missing.
- compile-gate-h `featck` over the four artifacts: clean after item 5. `collide`: one finding, `en1992 checks/📋️members` `use stud_layout::stud_layout` vs local mod: the same benign module/function name pair as the already-filtered `diagrams::diagrams` family (not a compile error).

## Still above 90 % (not model crates, not changed)
- `en1998-mutations` 95 % bytes: `🚪️io/📝️text/🧬️mutations/🦀️.rs` (179 KB: `parse_en1998_mutation` 65 KB and `print_en1998_mutation` 55 KB, two monolithic matches over `En1998Mutation`) plus the aggregate enum (65 KB). Needs the text codec cut per domain behind a sub-decoder trait or the enum moved to a lower crate (schema-codegen decision, like H's en1993 note).
- `en1998-scene-core` 92 %: `ground-and-bridges` 60 KB, `handles` 42 KB, `horizontal-elements`, `buildings` 40 KB each; modules share `kit`/`layout`/`handles`, so a split needs a `-scene-base` lower crate like I did for en1994.
- 85 to 90 %: `structural-scene-foundation`, `en1992-scene`, `en1996-scene`, `en1991-inferences`, `en1998-diff`, `en1993-schema`, several `mut-*` at 28 files.

## Unverified by compiler
Free `full_showcase()` return type and `pack_json::from_json_str` inference in the showcase crates; unused-import warnings in the editors (`En19xxSnapshot` is still used there); `pub use ..._more::{..}` and `test-support` forwarding of the four mutation splits; the moved `crate::` references of the splits rely on the scanner only.

## Ready
`- ready for rebuild: crate budget J` appended to `🗑️generated/build-en1992`, `build-en1997`, `build-en1993`, `build-en1998` `latest.md`.
