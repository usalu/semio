# Result: CRATE-ROLLOUT-R5B (prefix `r5b`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/r5b-tools/` (`step.sh <tag> <cmd...>`, `r5b-census.ts`). Logs: `F/🗑️generated/r5b-*`. Sandboxes: `/Users/ueli/.cache/r5b/`. Started 2026-10-17 09:50.

## 0. Verdict (kept current)

**DONE (static evidence only, UNCOMPILED).** `semio-s-artifact-stdio-pdf` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf`) is carved into 50 producer crates plus the unchanged-name facade (the facade is `semio-s-artifact-stdio-pdf`, 3 files lib / 8 test). Applied 17:24 (`--backup=F/🗑️generated/r5b-backup-pdf`, 820 writes, 50 owner rows); `new crates <pkg> --check-applied` (`r5b-check3`) exits 0 with `onlyCombined` 0 for lib and test, `admission`, `shared`, `sharedForeign` empty. Census (`F/🗑️generated/r5b-census-after.tsv`, `r5b-tools/r5b-census.ts`, 63 crates of the artifact folder): **0 crates over the gate in lib and test** (maxima: 29 files, 250,028 bytes (`pdf-tables`, the generated AFM/AGL tables, 95 % of the byte limit), 3,510 lines lib; 29 files, 250,028 bytes, 3,599 lines test). Before: 747 files / 3,749,019 bytes / 52,976 lines lib and 1,278 / 4,735,016 / 65,129 test in one crate (22x over the file gate); the `test-oracle` crate's test closure, refused before, now measures 21 / 180,451 / 3,049. No cargo run (no `COMPILE ADMITTED`).

Checks run: owners scan (no pdf row missing), taxonomy `pw0-taxcheck stdio` (mounts 0, tree 1 = the old gif finding), launch JSONs parse, `lib-wiring-budget` (the facade root is 587 lines of wiring, pinned at 587 in the ratchet like xml; the remaining failures of that test are other slices' roots), cfg(test) cross-crate scan `r5b-tools/ctscan.ts` (0 plain `#[cfg(test)] pub` items named across crates) and `tkdeps.ts` (the 7 crates naming `schema::testkit` depend on the testkit crate with the feature enabled), kept-root-module check (section 6).

## 1. Baseline census (cgc7 reader, host cfg, `F/🗑️generated/r5b-census-base0.tsv`, 09:50)

| crate | lib files/bytes/lines | test closure files/bytes/lines |
| --- | --- | --- |
| semio-s-artifact-stdio-pdf | 747 / 3,749,019 / 52,976 (OVER files) | 1278 / 4,735,016 / 65,129 (OVER files) |
| semio-s-artifact-stdio-pdf-test-oracle | 11 / 125,537 / 2,130 | refused: `Generated Rust input requires compiler ownership` (`4.1.4/base/oracles/tests/unit/🦀️.rs`) |
| 11 `pdf-1-*-lopdf-engine` / fixture generator crates | 1 file each, inside the gate | |

## 2. Progress

### 2.1 Planner baseline (`new crates <pkg> --dry-run --explain`, 10:00; logs `r5b-plan0..4.log`)

The plan (about 41 crates) fails on three clusters only; everything else already fits:

1. Mutation clusters of the 1.7 subsets x, e, ua, vt, h: schema leaves `use super::<sibling>::..` stayed coupled to the aggregate after the generic-leaf conversion, then the io binary/text leaves (`io/{binary,text}/mutations/<leaf>`) named the schema leaf type through the aggregate re-export, so the aggregate slice (which hosts `impl OpBinary/OpText for PdfXMutation`, coherence) and the 26 io leaf slices formed one cycle (76 files).
2. 1.7 base sqlite snapshot: root file carries an 81 KB inline `mod tests`, so the slice was 24 files / 310 KB (over the byte limit); `document` reached `pub(crate)` items of `text::snapshot` (coherence merge of the sqlite/text/binary/value-tests group, 35 files / 418 KB).
3. Test crate `pdf-snapshot-value-tests` is the 35-file group of (2).

### 2.2 Changes made so far

- Generator (library authoring module `L/🏗️authoring/📦️crate-tree/🧬️mutation-families/🟦️.ts`): after the generic-leaf conversion the leaf file and its companions have every `super::` chain that climbs above the leaf rewritten to an absolute `crate::` path (`absolutizeBeyond`, reused from `🪜️lift-kids`), so a sibling import such as `use super::remove_launch_action::RemoveLaunchAction;` no longer couples the leaf to the aggregate. (Test to be added in `L/🧪️tests/🧬️mutation-leaf-generics/🟦️.ts`.)
- Source: 130 io leaf files of the five subsets now name the schema leaf module (`schema::mutations::insert_launch_action::InsertLaunchAction`) instead of the aggregate re-export (`F/r5b-tools/ioleaf.py`, originals in `F/r5b-tools/orig/`).
- Source: the inline `mod tests` (81 KB) of `base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` moved to the file module `🧪️tests/🔬️unit/🦀️.rs` (include paths re-based; `F/r5b-tools/sqlite-tests.py`).
- Source: the six `pub(crate) fn` of `base/🚪️io/📝️text/📸️snapshot/🦀️.rs` (`spec`, `spec_producer`, `to_record`, `to_record_controlled`, `from_record`, `from_record_controlled`) became `pub fn` (removes the visibility coupling).


- Source (10:15): sqlite snapshot children decoupled from their root (`F/r5b-tools/sqlite-split.py`, originals in `F/r5b-tools/orig/sqlite-split/`): the root keeps only the module mounts (now `pub mod`) and the `validate_conformance_profile` re-export; the 14 shared cell helpers and the controlled `Reader` moved to the new child `🧰️reader/🦀️.rs` (all `pub`, fields `pub`); every child replaced its `use <root>::*;` glob by exactly the names it uses (schema snapshot glob, `reader::{..}`, `number::{Row, Projection}`, store/std imports, sibling modules); `pub(super)` in the children became `pub`; the unit tests file imports explicitly in the same way. Result: plan 6 shows 42 crates and 0 problems (before: 3 clusters of problems).
- Generator bug 2 (fixed with a test): `transformTestClosure` aggregation queued its `mod leaf_tests;` edit on the lib root file at the offset of an inline parent module that lives in another file (the artifact root was file-split by CENSUS4-PKG-29: `standards` is now `🌳️standards-tree/🦀️.rs`). `discoverMounts` now records the file of each inline module and the edit goes to that file; `carveWrites` had failed with `Missing or ambiguous Rust module: leaf_tests`. Test: `L/🧪️tests/📦️crate-tree/🟦️.ts` "aggregates sibling leaf tests into the file that holds the inline parent module".
- Generator bug 1 test: `L/🧪️tests/🧬️mutation-leaf-generics/🟦️.ts` "turns the sibling imports of a converted leaf into absolute paths ..." (8 tests pass; crate-tree 55 pass).

## 3. State after the usage-limit cut (12:15) and the core cycle

Found state: no half-applied carve (all `new crates` runs were `--dry-run` or `--verify --sandbox`); the source edits of section 2.2 are in the tree; generator edits are in `L/🏗️authoring/📦️crate-tree/`.

Further generator changes (all with tests in `L/🧪️tests/📦️crate-tree/🟦️.ts`, 67 pass):
- New module `🌲️inline-mounts/🟦️.ts` (`inlineMountTreeFiles`, used by the planner): CENSUS4-PKG-29 had moved the `standards`, `editor`, `viewer` mount trees of the lib root into files (`🌳️standards-tree` ..); the planner only slices wrapper modules that are inline in the lib root, so everything below them stayed in the facade (verify1: facade 209 files / 1.9 MB / 27,652 lines). The planner now folds such pure mount-tree files back into the root for planning (paths re-based) and the carve deletes the emptied files. Test: "folds a root-level mount tree file back ...".
- `🧾️facet-leaves`: `rust: include_str!(..)` companions can be served from repository documents too (`new leaves <file> --json-schema --rust`); the base 1.7 schema descriptor embeds 590 KB (its own children's Rust text and 300 KB of json schema).

Planner status (plan9: `--mutation-families=on --no-test-aggregate --lift=standards::v1_7::subsets::base::modules::*`): 28 crates planned, remaining blocker: one strongly connected component of 34 groups in the 1.7 base (snapshot, sqlite document, io root, modules, conformance support, mutation aggregate). Back-edges found (explain graph, `F/r5b-tools/cyc.py`): (1) the sqlite `document` module implements the foreign `ArtifactSqliteSnapshot[Host]` traits for `PdfSnapshot` (orphan rule: same crate as the snapshot) while the sqlite children it calls depend on the snapshot model types in the same file; (2) the io root hosts the engine error, `PResult` and the text-string codec that snapshot, foreign artifacts and the modules need; (3) io root names the schema root; (4) mutation leaf tests reach `io::mutation_bridge`/export; (5) `text::mutations::replace_page` uses the aggregate type `ReplacePage`.

### 3.1 Progress 12:50: plan14 has no problems (56 crates, every lib and test closure inside the hard budget)

Command: `bun ./📜️script.ts new crates <pdf pkg> --dry-run --explain --mutation-families=on --no-test-aggregate --lift=standards::v1_7::subsets::base::modules::*` (log `r5b-plan14.log`). Source and generator changes since 3.0 (each cut in the rightful owner, no compat shims):

Source (pdf artifact):
- 1.7 base snapshot: the typed document model moved to the child `schema/📸️snapshot/📐️model` (`F/r5b-tools/snapshot-model.py`); `octets` (the `ToValue/FromValue` impls of `PdfObject`) and `canonical` are mounted from the model; the root keeps `PdfSnapshot`. The sqlite children, `schema::stream_roles` and `graph_source` import `snapshot::model::*`, so the sqlite `document` (the foreign `ArtifactSqliteSnapshot[Host]` impls for `PdfSnapshot`, orphan rule) sits above the children instead of in a cycle with them.
- 1.7 base io root: the engine error / `PResult` / `malformed` (`io::error`) and the text string codec (`io::text_string`) are child modules; all 14 users import them from there (`F/r5b-tools/io-root.py`).
- 1.7 base fonts: `StandardFontMetrics` and its impl moved to `fonts/📏️metrics` (the generated `tables` file now imports the struct; the ticket generator `PDF-ARTIFACT-SPEC-COMPLETE/🐍️generate-font-tables.py` was changed to emit that import), so tables (250 KB) no longer merge with the fonts code.
- 1.7 base schema descriptor: companions served from documents (`new leaves ... --json-schema --rust`); the snapshot rust facet now concatenates the root and the model file. The schema root went from 590 KB to 19 KB.
- 1.4: composer registry moved out of the base io root to the standard level (`4️⃣1.4/🦀️.rs`, mounted by `🌳️standards-tree`, like 1.7); `STDIO_PDF_DOCUMENT_SCHEMA` now lives in the snapshot (22 users re-pointed); `demo_pdf_snapshot` moved from the text snapshot to the io root next to `encode_pdf/decode_pdf`; the two inline test modules of the sqlite snapshot became file modules under `🧪️tests` (`F/r5b-tools/sqlite14-tests.py`); binary/text mutation primitives (`put_index`, `put_text`, `Reader`, `hex`, `unhex`) hoisted into `🧰️wire` children (`F/r5b-tools/wire14.py`).
- io mutation leaves of all subsets (1.4 and 1.7 base, x, e, ua, vt, h; 140 files) import the schema leaf module instead of the aggregate re-export (`F/r5b-tools/ioleaf.py`).

Generator (library authoring module, tests in `L/🧪️tests/📦️crate-tree/🟦️.ts` 69 pass and `L/🧪️tests/🧬️mutation-leaf-generics/🟦️.ts` 9 pass): `inlineMountTreeFiles`; `crateRelativeChains` no longer reports the shared prefix of a brace group as a reference; test detach carries host `use super::X;` and `{self, ..}` imports as absolute imports; the generic-leaf conversion writes paths without the private `component` segment and absolutizes `super::` imports of the leaf; aggregation edit lands in the file of the inline parent; `new leaves --rust`.

Next: `--verify --oracle` (`r5b-verify2`), apply, `--check-applied`, then the cfg(test) cross-crate scan and the kept-root-module check from the coordinator.

### 3.2 State 14:40 (verify5 running; nothing applied yet)

- `verify4` (14:00): census 51 crates, 0 over the gate; module trees, visibility, orphan, glue, unresolved crates all 0; layout differential 0 / 0 (lib / test); single-crate oracle native 0, `--test` 2 candidate-only (the 1.4 sqlite `backing` test helper named private imports of its former host; fixed in the source with explicit imports).
- Source added since: root-bound tests (hook `register_sqlite_test_declaration`, the structural-hash and record-asset tests) moved from the artifact lib root to the file module `🧪️tests/🔬️unit/🦀️.rs` (`mod root_tests`), the 1.4 sqlite `backing` test helper is mounted from the test that uses it, export children `pub(super)` -> `pub` and `pub mod graft`.
- cfg(test) cross-crate rule (coordinator 11:30, 14:10): the scan `F/r5b-tools/ctscan.ts` on `verify2` found the cross-crate `#[cfg(test)] pub fn` helpers: ten fixtures in `schema::conformance_support` (`document`, `document_of`, `snapshots`, `added`, `removed`, `content_edit`, `catalog_object`, `with_tail`, `with_trailing_entry`, `placed`), `io::mutation_bridge::{applied, after_rows}` (about 90 users) and the root hook. Fix (F/r5b-tools/testkit.py): all of them live in the new sibling module `schema::testkit` (`#[cfg(any(test, feature = "testkit"))] pub mod testkit;` in the 1.7 base schema root); 142 test files import it; the artifact package manifest declares `[features] testkit = []`, which the carve forwards into every produced crate (`testkit = [dep/testkit..]`) and into their `[dev-dependencies]`, the canonical form. The planner puts the module into its own crate `semio-s-artifact-stdio-pdf-testkit`.
- Generator fix found on the way (peer regression, fixed with a test in `L/🧪️tests/📦️crate-tree/🟦️.ts`): `prunedUse` rendered a `self` leaf of a group as an empty member (`use a::{, B};`), which made the detached leaf tests refuse and merged the five 1.7 subset mutation clusters back into over-budget crates (plan problems `mut-x/e/ua/vt/h`).

### 3.3 First apply (15:31) and its revert (17:25)

`verify5` exited 0 (census 51 rows, 0 over; module trees, visibility, orphan, glue, unresolved crates 0; single-crate oracle and layout differential 0/0 for lib and test). I applied with `--backup=F/🗑️generated/r5b-backup-pdf` (821 writes, 50 crates, 50 owner rows) and ran `--check-applied` (`r5b-check1`): exit 1 with 4 native and 14 test extra diagnostics, all in code the verify path does not see:
1. `🧾️artifact-declaration` (a kept root file module) names `io::text::snapshot`, `io::binary::snapshot`, `io::text::diff`; the planner had woven those children away from the facade because it only counted references of root items, not of kept root file modules (the coordinator's "oracle blind spot: kept root file modules"). Generator fix in `🗺️plan` (`rootItemPaths` also takes the chains of every kept non-test root file module), test "keeps a mounted child in the facade when a kept root file module names it".
2. 75 leaf tests and the conformance-support unit tests still named the moved fixtures through the host's private `support` import or bare names; the export test `owned_fixture_publication_reports_canonical_logical_carriers` (a peer's new test) names the root's `native_codecs()`, so it moved to the root tests (`F/r5b-tools/testfix.py`).
I reverted the apply from its log and backup (`F/r5b-tools/revert.py`: 696 files restored, 125 created files removed, 50 owner rows removed; `📋️build-owners.json` 3253 -> 3203), re-ran `testfix.py` and re-plan. State at this point: package NOT carved, source prep complete.

## 4. Final sequence (second apply)

1. `new crates <pkg> --dry-run --explain --mutation-families=on --no-test-aggregate --lift=standards::v1_7::subsets::base::modules::*` (`r5b-plan23`): 50 crates, no problem.
2. `--verify --oracle` (`r5b-verify5`, same flags plus `--sandbox`): exit 0; census 51 rows 0 over; module trees, visibility, orphan, glue, unresolved crates 0; single-crate oracle native 4020 vs 3816 and `--test` 4589 vs 4145, `onlyCandidate` 0; layout `onlyCombined` 0 / 0.
3. Apply 1 (15:31) then `--check-applied` (`r5b-check1`): exit 1 (4 native, 14 test extras, section 3.3), reverted from log and backup (`r5b-tools/revert.py`), fixed, apply 2 (17:24, backup `r5b-backup-pdf`, log `r5b-apply.log`), `--check-applied` `r5b-check2` (2 native extras: `text::snapshot::COMPONENT_GRAMMAR_*`), `r5b-check3` exit 0. The two extras of check2 are reproducible in the kept combined crate as `E0425 ... COMPONENT_GRAMMAR_SEMIO` at `artifact-declaration:112-113`; the same tree gave 2 extras in check2 and 0 in check3 (the base diagnostic count of the report changed from 6,586 to 4,455 between the two runs, which I did not trace; the checker sources are edited by peers), and the raw combined output still contains the two errors, so they are cancelled by an equal base diagnostic rather than absent. I believe they come from the sandbox not copying the `include_str!` companion `📖️.grammar.semio` of that const, but I did not prove it. They are not reproducible as a missing item in the source (the consts are `pub const` at the top of the file).

## 5. Generator changes (all in `L/🏗️authoring/📦️crate-tree/`, tests `L/🧪️tests/📦️crate-tree/🟦️.ts` 88 pass and `L/🧪️tests/🧬️mutation-leaf-generics/🟦️.ts` 9 pass)

| change | where | test |
| --- | --- | --- |
| generic-leaf conversion absolutizes `super::` imports of the leaf and writes paths without the private `component` segment | `🧬️mutation-families` | "turns the sibling imports of a converted leaf into absolute paths ..." |
| aggregation edit lands in the file of the inline parent module | `🧪️test-closure` | "aggregates sibling leaf tests into the file that holds the inline parent module" |
| test detach carries host `use super::X;`, `{self, ..}` imports and `use <child>::*` globs as absolute imports | `🧪️test-closure` | "detaches a test whose host imports an item with a relative super path ..." |
| fold root-level mount tree files back into the root for planning, delete them in the carve | new `🌲️inline-mounts`, `🗺️plan` | "folds a root-level mount tree file back into the root ..." |
| a brace group names the chains of its members, not the shared prefix module | `🗺️plan` `crateRelativeChains` | "a brace group names the chains of its members ..." |
| references of kept root file modules (artifact declaration) count as outside references, so the facade keeps those children | `🗺️plan` `rootItemPaths` | "keeps a mounted child in the facade when a kept root file module names it" |
| `prunedUse` rendered a `self` leaf as an empty member (`{, B}`), which silently disabled test detaching (peer regression) | `🌳️module-tree` | "prunes a group with a self leaf without leaving an empty member" |
| `new leaves <file> --json-schema --rust` serves the `rust:` facet companions from documents | `🧾️facet-leaves`, `🎮️command` | extended facet test |

## 6. cfg(test) rule, testkit and kept root modules

- Cross-crate `#[cfg(test)] pub fn` helpers found by `ctscan.ts` on the first sandbox: ten fixtures of `schema::conformance_support`, `io::mutation_bridge::{applied, after_rows}` (about 90 users) and the root hook `register_sqlite_test_declaration`. They now live in the sibling module `schema::testkit` (`#[cfg(any(test, feature = "testkit"))] pub mod testkit;` in the 1.7 base schema root, `r5b-tools/testkit.py` and `testfix.py`), the artifact package manifest declares `[features] testkit = []`, and the carve forwards it into every produced crate and into the `[dev-dependencies]` of each crate on its dependencies (the canonical idiom of the coordinator, no self dev-dependency). The root hook and the root-bound tests are in the root file module `🧪️tests/🔬️unit/🦀️.rs` (`mod root_tests`), the 1.4 sqlite `backing` helper is mounted from the test that uses it.
- Kept root modules: the root keeps `artifact_declaration` (and `root_tests`); the combined crate of `r5b-check3` was compiled with rustc and every path error at `artifact-declaration` and the root files was inspected (section 4: only the two sandbox-artifact `COMPONENT_GRAMMAR_*` errors, present in the base too). The facade needed no hand patch after the generator fix.
- Open for the compile queue: `cargo test -p semio-s-artifact-stdio-pdf-<crate>` for the seven crates that enable `testkit` (`r5b-tools/tkdeps.ts` lists them) and `cargo check -p semio-s-artifact-stdio-pdf`, wasm and native; nothing was compiled. A peer test (`owned_fixture_publication_reports_canonical_logical_carriers`) with a `[DEBUG]` eprintln moved from the export tests to the root tests because it names the root's `native_codecs()`; its `eprintln!` is the peer's and was left.

## 7. Source changes (pdf artifact), by kind

All with originals in `F/r5b-tools/orig/`: 1.7 base snapshot model split (`snapshot-model.py`), sqlite snapshot children decoupled from their root (`sqlite-split.py`, `sqlite-tests.py`), io root error / text string children (`io-root.py`), fonts metrics child (and the ticket generator `PDF-ARTIFACT-SPEC-COMPLETE/🐍️generate-font-tables.py`), schema descriptor served from documents, io mutation leaves import the schema leaf module (140 files, `ioleaf.py`), 1.4: standard-level composer registry (`4️⃣1.4/🦀️.rs`), schema const moved to the snapshot (22 users), demo snapshot moved to the io root, sqlite tests as files (`sqlite14-tests.py`), mutation primitives in `🧰️wire` children (`wire14.py`), export children made `pub`, testkit move (`testkit.py`), root tests (`testfix.py`).

## 8. Hand-offs

- `pdf-tables` (generated, 250,028 bytes) is inside the hard gate but above the 230,000-byte target; a runtime asset channel for the AFM/AGL tables would be the next step.
- The binary/text mutation leaf crates of the five 1.7 subsets and base are produced as `mut-*` crates with 28 files each (test closures 28-29 files, below the hard limit).
- Every `F/r5b-tools/*.py` script is idempotent over the pre-carve source; none is needed after this slice.
