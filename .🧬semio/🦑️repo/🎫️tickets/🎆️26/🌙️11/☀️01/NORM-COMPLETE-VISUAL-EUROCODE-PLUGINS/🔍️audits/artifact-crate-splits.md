# Artifact crate splits (Eurocode source boundary)

Budget per crate: <= 262144 bytes, <= 32 files, <= 4000 lines (target <= 200 KB, <= 24 files, <= 3200 lines). Nothing was built; "ready for rebuild: crate splits" is appended to `build-en1992/1993/1997/1998/latest.md`. New crates change `Cargo.lock` (synchronize rewrites it). Scripts (re-runnable are idempotent only for registration; splits are one-shot): `🧰️scripts/crate_split.py`, `split-en1992-checks.py`, `split-en1997-checks.py`, `split-en1993-checks.py`, `split-en1998-mutations.py`, `compact-schema-json.py`, `schema_json.py`, `crate-split-deps.py`.

Scan result before: over-budget Eurocode crates = en1992-checks, en1997-checks, en1993-model, en1993-diff, en1993-schema, en1998-mut-{buildings,silos-tanks,members,bridges-towers,foundations}. en1990/1991/1994/1995/1996/1999 had none. Near the limit and fixed too: en1993-checks (233 KB, 3996 lines). After: no Eurocode crate is over budget; the scan only lists non-Eurocode vdi3805-model, iso16757-model (left alone) and app-surface (integrator).

Method: files stay in place; each new crate has its own folder next to the old modules (`<module>/<part>/📦️packages/🦀️rust/Cargo.toml`, lib root `🦀️.rs`) and mounts them with `#[path = "../<module>/🦀️.rs"]`. Same preamble (extern crate aliases, imports) in every root so `use super::*` keeps working; the old crate does `pub use new::*` (module paths and flat names unchanged). Items that were `pub(super)` across a moved boundary became `pub`; private `struct Shear/Tors/Punch/Group` (en1992) became `pub struct`. Layers are acyclic (checked with symbol scans).

## en1992 (checks 385 KB / 24 files / 6410 lines)
| crate | KB | files | lines |
|---|---|---|---|
| en1992-checks (helpers, anchor, member-evaluation, new `🗄️registry` split out of catalogue) | 128 | 11 | 2295 |
| en1992-checks-core (catalogue types, NDP tables, materials, design effects) | 58 | 5 | 1004 |
| en1992-checks-resistances (section, shear, punching, detailing, fastenings, serviceability) | 138 | 7 | 2199 |
| en1992-checks-specials (prestress, fire, bridges, analysis models) | 62 | 5 | 964 |
Dependency order: core <- resistances <- specials <- checks. `catalogue` was split: types in core, `registry()/entry()/evaluate_*` in `🗄️registry` (top).

## en1997 (checks 243 KB / 17 / 4523)
| en1997-checks | 134 | 7 | 2398 |
| en1997-checks-catalogue (`🧬️catalogue-core`: catalogue + design-approach parameters) | 111 | 11 | 2144 |

## en1993
- model 245 KB / 5 / 7300 -> 180 KB / 5 / 2022; diff 241 / 5 / 7443 -> 184 / 5 / 2421; schema 187 / 5 / 5770 -> 125 / 5 / 699. Cause: the pretty-printed facet-leaf JSON schemas (5310 / 5128 / 5103 lines) were each above the line limit and cannot be split by entity. Fix at the generator: `schema_json.dump_schema` (one line per `$defs` entry) used by `en1993-special-gen.py`, and the three existing leaves were rewritten with `compact-schema-json.py` (content identical, only whitespace). No crate split needed. Any other tool that rewrites these three files with `indent=2` would undo the fix.
- checks 233 KB / 12 / 3996 -> en1993-checks 54 / 6 / 1122 + en1993-checks-core (root helpers, `Scope`, structure-helpers) 27 / 2 / 524 + en1993-checks-special (special-element checks) 154 / 6 / 2389.

## en1998 mutation crates (file count)
Each leaf = 3 files (leaf, diff, inverse). Max 7 leaves per crate (22 files). Leaves grouped by entity; insert/remove pairs and leaves referencing each other stay together. Old crates re-export the moved leaf modules with `pub use`.
| crate | before (files) | after |
|---|---|---|
| mut-buildings | 115 | 22 + new -annex-building (19), -storey (22), -storey-2 (13), -site (22), -system (22); existing -2 untouched (16) |
| mut-silos-tanks | 61 | 22 + -silo-tank (19), -tank (22) |
| mut-members | 49 | 22 + -masonry-member (7), -assessment (22) |
| mut-bridges-towers | 49 | 22 + -bridge-tower (7), -tower (22) |
| mut-foundations | 46 | 22 + -foundation (4), -retaining-wall (22) |
All new crates are about 4-31 KB and 87-578 lines.

## Near the limit (not over, not touched)
en1996-schema 209 KB / 23 / 3932 lines (code, 23 modules), en1999-diff 3792 lines and en1998-diff 3595 lines (pretty facet JSON of 1.7k and 2.2k lines: apply `compact-schema-json.py` plus the `dump_schema` generator change if they grow), en1993-clauses 25 files, `*-io` crates 25 files.

## Risks for the builders
- Not compiled. Likely first-build issues: unused-import warnings (roots carry the full preamble), a symbol I classified as local but that is shared (fix by moving the module one layer up), `Cargo.lock` sync.
- JSON leaf compaction assumes consumers parse the JSON (tests read it via serde/`JSON.parse`).
