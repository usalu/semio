# Compile Review G (wave 2): files changed in the last 3 hours (v5-v8 rounds, nested rows, shared v4/v5, laws wiring, spec-trace, NDP tables)

Scope: every `.rs` and `Cargo.toml` under `📕️norm` with mtime within 180 minutes at 01:15 (3884 files, dominated by generated mutation leaves, 10 spec-trace tables, 22 NDP table files, 42 app-surface files). No cargo or rustc was run.

## Method (scripts in `🗑️generated/compile-review-g/`, reusing review A and E)
- `bal.py` delimiter balance over all changed `.rs`: clean.
- `rsck.py` (use/path/arity/field/struct-literal resolution) run per artifact and per shared crate in parallel after the first pass went stale (owners saved fixes during the run), filtered by mtime and diffed against review E's baseline: 1772 new raw lines, all but the findings below are known macro/derive/closure noise (enum-variant globs, `Diff`/`ToValue` derive names, grouped `use`, local closures `pick`/`chord`/`put`/`note`, macro bodies).
- `exh.py`, `leafck.py`, `applyck.py`, `diffck2.py`, `serdeck.py` over en1990-en1999 diffed against review E: only checker quirks (tuple matches with wildcards, nested-row inverses read the action row correctly) and the known serde item.
- New: `mutck.py` (enum variants vs `KINDS` vs imports for all 17 mutation enums: counts equal, every `mod` imported), `mountck.py` (every `use ..._mut_X::leaf` resolves through group crates and `pub use` facades: 0 unmounted), `tomlck.py` (742 manifests parse, every `workspace = true` dependency declared, every path dependency, `[lib] path`, `[[test]]` path and workspace member exists: clean), NDP script (2920 ids, sorted and unique in all ten tables; every `ndp_*("ndp...")` literal is an id or a documented prefix of ids).
- Manual reads: `RemedyBound::Remove` (enum, chips symbol match, en1995 `flipped` wildcard, edit resolution), spec-trace module and tests against all ten generated tables (`SpecTrace::new` arity 5, `SpecTraceTable` literal fields, const-context promotions, closure/borrow types in `unreached_fragments`/`assert_reaches_all_but`), `BendShape.both_ends` (all literals set, tessellation and en1992 members), `stiffener_spacing` (en1993 `PlatedPanel`, en1999 member, snapshot defaults, sqlite, diff patch, leaves), `bracing_system` (en1996 default, derives `Copy, PartialEq` for `min_by_key`/`!=`, sqlite `choice!`), placement-drag `derived_yaw` (closure parameter, not the function), `chord`/`pick` arities (closures).

## Fixed (certain, after re-reading)
| file | issue | fix |
|---|---|---|
| en1994 `🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs` (mounted in the umbrella as `diff_unit`) | nested-rows round made the payloads id-keyed (`RemoveBeam { id }`, `ChangeBeamSpanM { id, .. }`, `ChangeBeamActionQAreaPa { beam_id, id, .. }`) but this test still built `{ index, action_index }` (E0560/E0063, 6 literals) | `span(base, index, value)` resolves the id from the base, closures take the intermediate snapshot (`\|mid\| span(mid, 1, ..)`, `RemoveBeam { id: mid.beams[1].id.clone() }`), literals use ids |
| vdi3805 `🦀️.rs` and `🧬️schema/📸️snapshot/🧮️sheet-attributes/🦀️.rs` | half-finished split already in HEAD: the model's `DslField for SheetAttributes` calls `sheet_attributes_controlled`, which lived in the umbrella root (together with `sheet_attribute_text`/`_optional_text`) and used model types it did not import (E0425 in the model crate, E0412 in the root) | moved the three functions into the sheet-attributes file next to `vdi3805_invalid` and the `sheet_attribute_*` helpers; removed them and the three now unused imports from the root |

The stale en1994 `🧰️testkit` hits of the first pass (index literals) were fixed by its owner at 01:16 and are not counted.

## Open (routed to `📌️important/requests.md`, "from: compile-review G")
1. Source gate: `semio-s-artifact-norm-en1995-mutations` 4786 lines and `semio-s-artifact-norm-en1995-model` 4726 lines (limit 4000). Nothing else is over.
2. E0277 risk unchanged from review E item 2 (mutation payload serde under `cfg_attr(test, ..)` used from other crates' tests); now also the en1995 mutation unit test with `mut-*` payload types and the sqlite snapshot tests of en1990-en1995 that serialise `En19xxMutation`. Copy the en1992 `any(test, feature = "test-support")` pattern.

## Gates
`check-cargo-authority.py`: 28 crates, 28 members, 0 errors. `source-boundary-scan.py`: 2 OVER (above).

## Limits
Expression-level typing, borrowck and trait resolution beyond the scripts are not covered. Churn continued while reviewing (spec-trace tables regenerated 01:28, NDP tables 01:29, en1994/en1992 mutation roots 01:15-01:25); files saved after ~01:30 were not seen. The `mdep.py` (extern head vs Cargo dependency) run was still going at the end and had reported one macro-body false positive only (`norm_live_commands!`).

`- ready for rebuild: compile-review G` appended to `build-en1994/latest.md` (vdi3805 has no build log).
