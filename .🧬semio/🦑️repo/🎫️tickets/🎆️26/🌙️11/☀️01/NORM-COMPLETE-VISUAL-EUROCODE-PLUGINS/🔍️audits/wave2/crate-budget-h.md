# crate budget H (wave 2): pre-emptive splits and the budget test

No cargo/rustc, no git writes, nothing compiled. Scripts: `🧰️scripts/crate-budget/h_*.py` (idempotent; `h_lib` = shared helpers, `h_lower_core` = generic "root moves into a lower crate"). Log: `🗑️generated/crate-budget-h/mut-halves.log`. `verify build-owners sync` passed (added=3), `check-cargo-authority.py` 30 crates 0 errors. Cargo.lock blocks of every new crate were registered in the artifact lock, the hub lock, fixture-sweep and bridge; dev-dependency edges patched afterwards. Final scan: 0 crates over, 7 above 85 %.

## Test
`⚖️compliance/🧪️tests/🧱️source-budget/{🐍️.py,🥒️.feature}`: self-contained module-closure scanner (mod, `#[path]`, include_str, cfg(test) stripped). `python -I 🐍️.py` warns from 85 % of any limit and exits 1 above 100 %; `-v` lists everything above 75 %.

## Splits (all public paths kept by `pub use` / `pub use ... as`)
- en1998-inferences 262 KB -> 139 KB: `-inferences-details` (helpers, part details, geotechnical details), `-inferences-structural-system`.
- en1999-checks 261 KB -> 107 KB: `-checks-members` (8 check families).
- en1998-schema 258 KB -> 172 KB: `-schema-formulas` (na_de, part_1..6).
- en1995-schema 274 KB / 4850 lines -> 71 KB / 1905 lines: `-schema-timber` (158 KB) and `en1995-spec-trace`. Twin facets (ts/graphql/proto/json) were not touched or moved, so SCHEMA-CODEGEN's layout is intact.
- en1993-checks 234 -> 173 KB (`en1993-spec-trace`), en1997-checks 233 -> 77 KB (`-checks-geotechnics` 142 KB, `en1997-spec-trace`), en1997-checks-catalogue 225 -> 193 KB (`-checks-case-inputs`).
- en1993-mutations 239 -> 155 KB: `-mutations-edit-rules` (rule table, `h_edit_rules.py` is reusable).
- en1991-checks 227 KB -> `-checks-parts` (helpers + parts 1-1..4); the remaining catalogue stays in checks (see scan).
- Scenes: en1999-scene `-scene-core` (164 KB, scene keeps diagrams), en1997-scene `-scene-core` (178 KB), en1998-scene `-scene-core` (205 KB), en1993-scene-members -> `-scene-members-projection` (9 view modules + `fragment`), en1991-scene-members -> `-scene-members-projection` (traffic, hazards, snow, deflection + `fragment`; lane bands and stack level moved into `layout` to break a cycle); scene facades import from the projection crates.
- 83 mutation leaf crates above 24 files (not en1994) halved with `split_mounts` into `<crate>-more` (about 15 files each).
- JSON facet twins/fixtures reflowed with `h_compact_json.py` (semantics identical, idempotent, never expands a denser file). A regenerated twin must be reflowed again, otherwise en1991/en1993/en1995 schema and json-schema crates fall back over the gate.

## Open (not done)
- `structural-scene-foundation` (86 % lines) and every en1994 crate (`en1994-scene` 97 %, three `mut-*` at 31 files): deferred, en1994 freeze (`freeze.txt` still named build-en1994). Re-run `h_mut_halves.py` on a fresh scan after the freeze.
- `en1993-model` 98 % bytes: the `impl_norm_artifact_record!` ties the sqlite codec to the crate by the orphan rule and the `SNAPSHOT_LEAVES` rust facet is the model file itself, so it needs a schema-codegen decision (entity types into a lower crate) before it can be split.
- `vdi3805-inferences` 86 % lines (3452): `sheet-checks` and `document-checks` need a lower helper crate like en1999 members.
- `en1991-scene-members` still 77 %, `en1998-scene-core` 78 %, several scene crates 76-84 %.
- en1998 `mut-site-ground`, `mut-members-wall-rows` (and their `-more`) are missing from `en1998/Cargo.lock` (pre-existing); owner must re-lock.
- Unverified by compiler: private-to-pub visibility widening (`pub(super)`/`pub(crate)` -> `pub`) in moved modules, dev-dependency and feature forwarding of the lower crates (features such as `test-support` were not copied to the new crates), glob re-export ambiguities.

## Ready
`- ready for rebuild: crate budget H` appended to `build-en1990/1991/1992/1993/1995/1996/1997/1998/1999/latest.md` (not en1994).
