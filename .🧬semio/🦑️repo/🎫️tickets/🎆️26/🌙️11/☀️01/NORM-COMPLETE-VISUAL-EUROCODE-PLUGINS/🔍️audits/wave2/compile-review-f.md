# Compile Review F (wave 2): crate-split moves (static, no cargo)

Scope: files moved into sibling crates tonight (en1991/en1993 `-scene-members`, en1996 `-masonry-formulas`/`-masonry-catalogue`, en1994 `-mut-beam-design-bridge`, facet twin crates, mutation family crates, en1999 `-check-parts`, en1993 `-clauses-core/-special`, `-checks-core/-special/-special-frames`, en1992 checks core/resistances/specials, en1997 `-checks-catalogue`, io-orphan `#[path]` mounts).

## Method (scripts in the session scratchpad, derived from `🗑️generated/compile-review-a/rsck.py`)
- rsck run on all 15 artifacts (use/path resolution, heads, arity, struct shapes), then a stricter copy: `pub(crate)`/`pub(super)` counted as non-public across crates, private `mod` across crates, and cross-crate glob imports no longer expose restricted items. Result: 0 `non-pub` findings anywhere.
- `super::`/`self::` body paths resolved in every crate (0 misses); `crate::` paths covered by rsck.
- `include_str!`/`include_bytes!`/`include!`/`#[path]`: 13666 references checked against disk, 0 missing (one empty dir found later, see open). `CARGO_MANIFEST_DIR` relative paths of moved tests checked (scene-members `modes.py` oracle exists; en1996 three `*_SOURCE_ROOT`s).
- No file is mounted by two crates; no unmounted `.rs` in the moved trees (except a new language-agnostic adapter `🧪️tests/🧮️clauses/🦀️.rs`, test harness owned).
- Dependency graph of 3845 packages: 0 cycles over normal dependencies; dev-dependency cycles only in already existing families (and en1999 diff <-> mut-member, see open).
- `depck` (declared vs used `semio_*` crates, members, workspace deps, path deps): clean except two known false positives.
- Glob re-export collisions between sibling-crate globs: only `ENTRIES` in en1992 checks specials/resistances, always used qualified (`part::ENTRIES`), so no ambiguity error.
- `#[cfg(test)]` pub helpers used across crates: none (only `snapshot_wire_codec` inside io crates). Macro use: every macro invoked is imported/defined in its crate; the 12 `macro_rules!` are local; no `$crate` issue.
- Restricted-visibility methods/fields used across the new sibling boundaries: heuristic scan found nothing relevant.

## Fixed (minimal, after re-reading)
| file | issue | fix |
|---|---|---|
| en1994/en1992/en1997 `🧬️schema/🦀️.rs` | `Vec<Placement>` without import (E0412, same as en1993 fix in review A) | added `use ..._model::Placement;` |
| en1998 `🧬️schema/🦀️.rs:11` | `En1998Placement` not imported | added to model import |
| en1998 `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:4` | `En1998Diaphragm/Accelerogram/Pier/Bearing` used at 111-114, not imported | added to `use crate::{..}` |
| en1999 `⚖️checks/📏️parts/🦀️.rs:51` (check-parts) | `record_ndp_reads` unqualified in `na_de` | `semio_s_plugin_norm_compliance::record_ndp_reads` |
| en1997 `⚖️checks/🏗️project-checks/🦀️.rs:7` | bare `catalogue_demo_cases()` after catalogue moved into `-checks-catalogue` (only `catalogue` is re-exported by the root glob) | `catalogue::catalogue_demo_cases()` |
| en1995 root `Cargo.toml` | test `mutations/🧪️tests/🔬️unit` uses `mut_panel_layers`/`mut_fastener_positions` | dev-deps added |
| en1998 root `Cargo.toml` | test uses `mutations_demo` | dev-dep added |

## Open (routed to `📌️important/requests.md`, "from: compile-review F")
- en1993 `special-frames` test calls the facade `check_full_steel_structure` (upward dependency; cycle if fixed by dep).
- en1997 `mutations::resolve` and `demo_case_id` undefined (in-flight work).
- en1992 scene `case_law` mount points at an empty dir.
- en1999 diff/mut-member dev-dep cycle and unmounted diff unit test.
- en1995 `Cargo.lock` root package misses the new mut family crates (lock sync); source gate: `en1995-scene` 263404 B and `en1994-model` 4001 lines over budget.
- Transient: mid-run all 15 artifact-root `Cargo.toml` had a duplicate `-model` dev-dependency key (feature-line edit by another agent); gone at the end (all TOML parse).

## Gates at the end
`source-boundary-scan.py`: 2 OVER (en1995-scene, en1994-model, above). `check-cargo-authority.py`: 28 crates, 0 errors.
Limits: no type checking, no borrowck; false positives of the rsck family (derive names, closures, enum globs) ignored as in review A.
`- ready for rebuild: compile-review F` appended to build-en1992/1994/1995/1997/1998/1999 `latest.md`.
