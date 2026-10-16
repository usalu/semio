# Pack and Quiz File Splits (slice PACK-QUIZ-FILE-SPLITS, prefix `pqs`)

Executor: Sonnet, 2026-10-16. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. `Q` = `🧰️framework/🛍️products/❓️quiz`. `P` = `🧰️framework/🔨️modules/🎒️pack`. Tools: `F/pqs-tools/` (Rust: `pqslib.py`, `splitter.py`, `stubgen.py`, `oracle.py`, `plan_{value,catalog,quizschema}.py`, `install.py`, `lossless.py`, `pathcheck.py`, `parsecheck.py`; TS: `ts/` with `split.mjs`, `splitmod.mjs`, `rewriteimports.mjs`, `mkcfg.py`, `cfg-{hg,pc,session}.mjs`, `lossless.mjs`, `summ.mjs`, `taxmatch.mjs`; `vitest.sh`). Logs, snapshots and sandboxes: `F/🗑️generated/pqs-*`. Pre-edit copies of the three TS files and the quiz schema: `F/pqs-tools/ts/orig/`. No sub-agent, no ticket tool, no modifying git command, no `[DEBUG]` line.

## 1. Verdict

All six targets are split and installed. The coordinator's added target (`Q/🧬️schema/🦀️.rs`) is included; I re-read it first and it was unchanged since the census (1,385 lines, `ftc` had only touched the crate's tests).

- **Rust (3 files, 4,248 lines in the parents before, 160 after):** lossless by line multiset, stub-layer differential IDENTICAL in every configuration, no new warning, parse clean, paths clean, 0 new duplicate mounts, public paths unchanged. **Nothing was compiled** (the launcher is still refused by the compiler gate), so there is no compile claim. The real-check commands are in section 6.
- **TS (3 files):** the quiz-react vitest suite has the identical 923 test titles and all 923 pass, exactly as the baseline; `tsc` on the whole `quiz-react` project shows the identical single pre-existing error; no import cycle.
- **One target stays over the size line:** `Q/🎯️targets/⚛️react/🔨️modules/🧭️session/🟦️.ts` is 979 lines. The file is now one class (`QuizSession`, 857 lines with its docstrings) plus its imports. Section 3.3 gives the reason and the cut I would take next.

| Target | Lines before | Parent after | Children (lines) | Largest child |
| --- | ---: | ---: | --- | ---: |
| `P/🌱️value/🦀️.rs` | 1,505 | 57 | 5 (242 to 470) | 470 |
| `P/📐️format/🗂️catalog/🦀️.rs` | 1,358 | 50 | 4 (79 to 628) | 628 |
| `Q/🧬️schema/🦀️.rs` | 1,385 | 53 | 5 (109 to 392) | 392 |
| `Q/🎯️targets/⚛️react/🔨️modules/🧭️session/🟦️.ts` | 1,450 | 979 | 3 (57 to 393) | 393 |
| `Q/🧪️tests/🏠️home-grid/🟦️.tsx` | 1,246 | 550 | 3 topics (100 to 244) + 1 kit (165) | 550 |
| `Q/🧪️tests/🐾️pet-companions/🟦️.tsx` | 1,396 | 464 | 2 topics (367, 402) + 1 kit (154) | 464 |

## 2. Rust

### 2.1 What was cut
Children are `<emoji>️<kebab>/🦀️.rs`, wired in the parent with `#[path = "<folder>/🦀️.rs"] mod <snake>;` plus explicit `pub use` (public items) or explicit private `use` (items that siblings or the test companions still name). Every `//#region` marker moved with its cluster, except for the catalog, whose one region (`🔖️RetainedCanonicalSource`) stays in the parent around the four mounts. Every file starts with a `//! <emoji>` docstring, unique among its siblings. Widened to `pub(super)` (the narrowest form that works): 37 lines in value, 74 in catalog, 0 in the quiz schema (all its items were already `pub`).

**`P/🌱️value/🦀️.rs`** (crate `semio-framework-pack-record`; the heavyweight `pack::encode_json_value` path, split only, no body edited):
`🧾️canonical-forms` (243: record specs, depth check, packed-numeric eligibility, `EncCtx`, the string-interning pre-pass), `📝️record-encode` (310), `📖️record-decode` (470: `DecSource`, `DecCtx`, the materialization admission owners, the tag decoders), `🧮️table-soa` (242), `🚪️document-codec` (246: options, `DecodeReport`, `encode_document` and the record-body entry points). The parent keeps the root imports that tests and `🌳️intrinsic` still reach through `super::`/`use super::*` (the test-only ones are `#[cfg(test)]`), the `pub use` of `EncodeOptions`/`DecodeOptions`, the tests region and the two existing child mounts.

**`P/📐️format/🗂️catalog/🦀️.rs`** (crate `semio-framework-pack-format-catalog`):
`📋️catalog-records` (79), `🔤️phase-cursors` (275: manifest, UTF-8, symbols and chunks cursors), `🗃️symbol-table` (369: the limits, the aliases, `RetainedPackSymbolTable`), `🧭️catalog-cursor` (628: `RetainedPackCatalogCursor` and its `Drop`). The two existing child mounts (`original_symbol_storage`, `retained_inline_symbols`) and the two test mounts stay in the parent, so their `super::` paths are untouched.

**`Q/🧬️schema/🦀️.rs`** (crate `semio-framework-quiz-schema`, lib `quiz_schema`):
`📚️quiz-documents` (378: scalars, quiz, catalog regions), `🃏️sheets-and-results` (297: sheet, answer, result regions), `🔁️commands-and-events` (201), `👁️read-views` (392), `👥️presence-wire` (109). The `🧰️Testkit` and `🧪️Tests` regions and their mounts stay in the parent. Each child imports `serde::{Deserialize, Serialize}` itself (derive macros are not inherited through `super::`); the parent keeps `Serialize` for the test kit and `BTreeMap` for the unit test, both cfg-gated.

Source-boundary gate figures (4000 active lines, 32 files, 262144 bytes per crate): files in the lib closure rose from 5 to 10 (pack-record), 6 to 9 (catalog) and 1 + 2 tests to 6 + 2 (quiz schema), all under 32; bytes rose by the docstrings and mount lines only.

### 2.2 Verification (all static; log files in `F/🗑️generated/pqs-*`)

**Lossless (`pqs-tools/lossless.py`).** Stripped non-empty line multiset of the snapshot versus all installed files:

| Target | removed | added | explained by |
| --- | ---: | ---: | --- |
| value | 40 | 93 | 37 lines widened to `pub(super)` (37 removed + 37 added), 3 root import lines pruned or gated, 51 wiring lines, 5 docstrings |
| catalog | 75 | 110 | 74 widened lines, 1 import line, 32 wiring lines, 4 docstrings |
| quiz schema | 0 | 35 | 30 wiring lines, 5 docstrings (every original line is present) |

**Stub-layer differential (`oracle.py`, `RESOLUTION-ORACLE.md`, with the type-checking stub layer).** `rustc +nightly-2026-07-20 --emit=metadata`, baseline sandbox from the pre-edit snapshot versus a sandbox copy of the live tree, the same patches on both. Configurations: lib, `--test`, `wasm32-unknown-unknown`, `wasm32-wasip2`, the two wasm targets with `--test`; the quiz schema adds `testkit` and `testkit + all` for lib and test. The "frs" stub layer left rustc stopping after name resolution (`E0532` on tuple variants), so field and method privacy was invisible. I extended `stubgen.py` (sandbox only, never the repo): enum variants named anywhere as `Enum::Variant` now exist with their tuple arity, an enum without variants is a struct, and `NativeSchemaControl` is a trait. With that, typeck runs: baseline error counts are real type noise (value 679 lib / 1,339 test, catalog 515 / 994, quiz schema 162 to 180 / 419) and identical.

| Crate | Result on the final installed tree |
| --- | --- |
| value | IDENTICAL in all 6 configurations (`pqs-run-value-st4.log`, installed tree `pqs-run-value-live1.log`) |
| catalog | IDENTICAL in all 6 (`pqs-run-catalog-st4.log`, installed `pqs-run-catalog-live1.log`) |
| quiz schema | IDENTICAL in all 10 (`pqs-run-quizschema-st2.log`) |

The differential found two real defects before install, both fixed in the splitter and re-proved: (a) a one-line struct (`struct ControlledMaterialization{control:…,maximum:u64}`) was not widened, so the document child read a private field (`E0616` on `.control`); the splitter now widens fields of one-line structs; (b) three child imports that came from enum-variant names (`Catalog`, `Category`, `Profile`, `Introduction`, `Leaderboard`, `Quiz`) are now dropped by a per-child `super_drop` list.

**Negative controls (mutating the installed copy only).** Un-widening `ControlledMaterialization.control` gave 2x `E0616` (`pqs-run-value-neg2.log`); un-widening `check_items` and `value_slots` gave 2x `E0624` each (`neg3`); in the catalog, un-widening `RetainedPackSymbolTable.symbols` gave 9x/11x `E0616` and `scalar_ptr` gave `E0624` (`pqs-run-catalog-neg3.log`). Un-widening names that no other file touches (for example `used:`, `charge`, `remember`) gave no error, so the scan over-approximates slightly, the safe direction. `E0451` (private field in a struct literal) is a late privacy pass that does not run once typeck has errors, so the one-line-struct literals were checked by the widening scan, not by rustc.

**Warnings (`oracle.py --warn`, message level).** Unused-import warnings: value baseline 1 and now 0 (the pre-existing one was an unused import, gone with the pruning), catalog baseline 1 / 4 (lib / test) and now 1 / 4, quiz schema baseline 0 / 1 and now 0 / 1; new = 0 in every configuration. Root imports that only tests need are `#[cfg(test)]`; the dead ones are dropped (listed in `plan_value.py` and `plan_catalog.py`). The unused `#[cfg(test)] use …::PackSink` of the catalog was already unused before; I kept it (`root_keep`) so no import is removed without a reason.

**Parse and rustfmt (`parsecheck.py`).** `rustc -Zunpretty=normal` loads every out-of-line module: 0 errors for all three crates. `rustfmt --check`: 0 parse errors; diff hunks baseline to now: value 40 to 48, catalog 99 to 105, quiz schema 3 to 13. The extra hunks are formatting of the added mount and import lines; nothing was reformatted.

**Paths (`pathcheck.py`).** Every `#[path]`/`include!` target exists, each new file is reachable exactly once, longest repo-relative path 106 bytes (limit 240), no new file over 628 lines, regions balanced per file, docstring emoji unique among siblings: `PATHCHECK OK`.

**Taxonomy.** `pqs-tools/taxcheck.ts` (the repo's own `matchDirectoryKind` over the whole ancestor chain) for all 14 new Rust module folders: 14 resolve to `crate-module-children`, 0 unresolved, 0 ambiguous (`pqs-taxcheck.log`). No taxonomy edit needed for Rust.

**Public paths.** The set of top-level `pub` item names (including `pub use`) of each parent: value 16 old / 16 new, catalog 12 / 12, quiz schema 103 / 103, nothing missing, nothing extra. No caller changed.

**Duplicate mounts.** `dm2-tools/modowners.ts` (the unchanged scan) over the whole repo at the end: 2,540 manifests, 31,028 file-package pairs, 1 file owned by more than one package (a reactor `ops`/`patches` file of another slice); none of my files, so 0 new (`pqs-dm.log`).

## 3. TypeScript

### 3.1 Baseline (before the first edit)
- Tests: `F/pqs-tools/vitest.sh base-all` runs the `quiz-react` vitest config (`Q/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`, 25 test files): **923 tests, 923 passed, 0 failed** (`pqs-base-all.json`, rows in `pqs-base-all.rows`). `🏠️home-grid` had 105 tests, `🐾️pet-companions` 45.
- Type-check: `tsc --noEmit -p tsconfig.json` in `Q/🎯️targets/⚛️react/📦️packages/🟦️typescript`: 1 error, `TS7016` in `🧪️tests/🚏️navigation/🟦️.tsx(13,23)` (no declaration file for `jsdom`), pre-existing (`pqs-tsc-base.log`).

### 3.2 Test files (method of `RESULT-RENDERER-TS-TESTS-SPLIT.md`: `split.mjs`, TypeScript compiler API, statements never rewritten)
Statements go to a topic by line range; a helper used by one topic moves into it, a helper used by several goes into one kit module (exported, nothing else changed); each file gets exactly the imports it references. Following the sibling `🧰️learner-journey-kit` convention, kits are sibling folders of `Q/🧪️tests/` with no test of their own and are not in the vitest include list. The original file docstring moved verbatim to the topic that keeps the old folder name. Titles are unchanged: the outer `describe("🐾️ pet companions")` is re-opened in each of its three files with the same header, so all full names are the same.

| Folder (under `Q/🧪️tests/`) | Lines | Tests | Holds |
| --- | ---: | ---: | --- |
| `🏠️home-grid` (kept) | 550 | 45 | the home overview suite |
| `🎫️quiz-card-challenges` | 195 | 10 | the challenge on the quiz cards and pages |
| `🥇️home-leaderboards` | 244 | 18 | the leaderboard of the new contract and the four leaderboards |
| `📱️home-text-and-viewport` | 100 | 32 | text size, the opened page and the short viewport |
| `🛠️home-grid-kit` | 165 | | fixtures, stylesheet readers (lightningcss), `Home` harness, `afterEach` |
| `🐾️pet-companions` (kept) | 464 | 21 | choice, liveliness, scene, layer |
| `🎪️pets-on-stage` | 402 | 13 | the pets on stage, the client |
| `🎛️pet-allowances` | 367 | 11 | what the learner allows, the hand and the topics, play with the pets |
| `🪛️pet-companions-kit` | 154 | | doubles, motion preference, render targets, screen sessions |

`tools: ts/lossless.mjs` against the originals: every non-import line is present; the only differences are the added `export` on kit declarations, the new docstrings, and the replicated `describe("🐾️ pet companions", …)` header and its closing `});` (2 + 2 lines). `quiz-react` registration: the five new test files are added after their old neighbours in the vitest include list (`🧪️tests/🎚️config/🟦️.ts`) and in the package `tsconfig.json` include list. **Launch JSONs: no change.** There is no per-file entry for these suites; `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` hold only the whole-package `bun repo run @semio-tech/quiz-react:test` and `:typecheck`, which already cover them. Taxonomy: the 7 new test folders are added to `members-of-tests.memberNames` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (one hunk at the head of the list; `taxmatch.mjs` resolves all 7 as `members-of-tests`; JSON validated).

### 3.3 The session module
`splitmod.mjs` (compiler API) cut `🧭️session/🟦️.ts` by line range and computed the imports, the cross-file `export`s and the relative-path rebasing:
- `🧮️state/🟦️.ts` (393): the whole `🧭️State` region (steps, trail, notices, boards, `QuizState`, `QuizClientEvent`, `evolveQuizState` and the pure questions).
- `💾️restore/🟦️.ts` (57): the `💾️Persistence` region (`restoreQuizState`, `persistQuizState`, the `restored*` readers).
- `📋️session-model/🟦️.ts` (90): `SessionFailure`, `RefreshOutcome`, `SubmissionPhase`, `QuizConnection`, `QuizSnapshot`, `QuizSessionOptions`, the two timing constants and the small deciders around a command (`Decided`, `decision`, `failureOf`, …).
- The parent keeps the module docstring, the `🎮️Session` region and the class. No child imports the parent (checked), so there is no cycle.

No re-export barrel: all 17 import and export statements of 15 files that named the old path were rewritten per owner by `rewriteimports.mjs` (14 module files under `🎯️targets/⚛️react/🔨️modules/` plus the package entry `🎯️targets/⚛️react/🟦️.tsx`, whose public `export { … } from` lines now point at the owning children; the package's public names are unchanged). Two `@see` comments were re-pointed (`🚏️navigation` to `🧮️state`, `⚖️legal` to `💾️restore`). No test or fs reader read the session source.

**Why 979 lines remain.** `QuizSession` is one class of 857 lines whose 40 private methods call each other (loading, adoption, the deputy gate, the outbox settling). A TypeScript class cannot be spread over files without a design change: either a base class with `protected` members and abstract declarations of the intents it calls back (`refreshLearner`, `loadRun`, …), or a collaborator that owns the deputy gate (`ahead`, `behind`, `direct`, `decided`, `decidable`, `patient`, `deputise`, `held`, `fold`, `holding`, `saved`, `ownStanding`, about 150 lines) behind a narrow port. Both change the constructor order and the class shape, which this slice (split only) does not do. I took the cut that does not change one line of the class: everything that is not the class moved out. The collaborator extraction is the next step, in its own ticket, with the quiz-react suite (923 tests) as its law.

### 3.4 Verification
- **Test-title multiset and status.** `summ.mjs` rows (`status<TAB>fullName`) of `pqs-final-all2.json` versus `pqs-base-all.json`: `diff` empty, 923 / 923 passed, 0 failed. `pqs-final-all2.rows` equals `pqs-base-all.rows`.
- **Flaky runs, recorded honestly.** Two earlier full runs under a machine load of 90 to 150 had 3 to 4 failures that are not in the baseline: `pqs-after-all.json` (2 `🚏️navigation`, 2 `🚶️journey-and-tabs` tests; assertion on focus and `STACK_TRACE_ERROR`) and `pqs-final-all.json` (3 `🎪️pets-on-stage` tests, `layer()` null). Every one passed when its files were re-run alone (`pqs-rerun1.json`: navigation 40 + journey 14 pass; `pqs-rerun2.json` and `pqs-rerun3.json`: all 45 pet tests pass), and the third full run (`pqs-final-all2.json`, load about 37) is identical to the baseline. The failing tests were different in each run and are timing-dependent whole-app renders (they poll with the default 1 s `waitFor`), not caused by the split; I did not change any timeout.
- **tsc.** Whole `quiz-react` project after the change (`pqs-tsc-after.log`, repeated after the last comment edits as `pqs-tsc-final.log`): the identical single `TS7016` jsdom error, nothing else.
- **No new `no-circular`.** Import graph of the 13 touched TS files: 12 edges, no cycle.
- **Ownership test of the pack record crate.** See section 4.

## 4. Source-text readers and edits outside the targets
- `P/🌱️value/🧪️tests/🧱️ownership/🟦️.ts`: read `🦀️.rs` as plain text and required `pub fn encode_document(` and the other entries in it (and the forbidden-substring scan of the same file). It now reads the module tree with `readModuleTreeSource(…, { crates: false })`. `bun test ./P/🌱️value/🧪️tests/🧱️ownership/🟦️.ts`: 2 pass, 0 fail, 31 expectations. The `test-ownership` script also runs `tsc --strict` on that file; it reports one error at the unchanged `expect(native.lib).toEqual(contract.library)` line (`{ path }` versus `{ name, path }`), which is pre-existing and not mine.
- Searched and unchanged: no other TS, JSON or script reads the three Rust files or the session module as text (`git grep` over the product, the pack tree and the repo for the file paths, the type names and the item names; the `🎁️packs/🟦️.ts` mentions are comments about `build_symbols` and `encode_record_fields` only). The `project.json` inputs of pack-record cover `🌱️value/**/*`.
- Doc comments that named the old path (one line each, re-pointed to `📝️record-encode`): `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🔨️modules/🏠️host/🦀️.rs` (line 2891) and `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs` (line 281).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: 7 test folder names (section 3.2).
- The quiz oracle registry text (`Q/🔮️oracles/🔣️.json`) still names the suite `🧪️tests/🏠️home-grid`, which still holds the lightningcss assertions; unchanged.

## 5. New folders
Rust, all `crate-module-children`:
- `P/🌱️value/`: `🧾️canonical-forms`, `📝️record-encode`, `📖️record-decode`, `🧮️table-soa`, `🚪️document-codec`
- `P/📐️format/🗂️catalog/`: `📋️catalog-records`, `🔤️phase-cursors`, `🗃️symbol-table`, `🧭️catalog-cursor`
- `Q/🧬️schema/`: `📚️quiz-documents`, `🃏️sheets-and-results`, `🔁️commands-and-events`, `👁️read-views`, `👥️presence-wire`

TS:
- `Q/🎯️targets/⚛️react/🔨️modules/🧭️session/`: `🧮️state`, `💾️restore`, `📋️session-model` (`crate-module-children`)
- `Q/🧪️tests/`: `🎫️quiz-card-challenges`, `🥇️home-leaderboards`, `📱️home-text-and-viewport`, `🛠️home-grid-kit`, `🎪️pets-on-stage`, `🎛️pet-allowances`, `🪛️pet-companions-kit` (`members-of-tests`)

Each folder holds one `🦀️.rs`, `🟦️.ts` or `🟦️.tsx`. All emoji are used as folder prefixes elsewhere in the repo and are unique among their siblings.

## 6. Real checks for the compile queue
Not run: `bun repo cargo` is refused by the compiler gate, so there is no claim beyond the static evidence above. Run under the compile mutex, one call:

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-pack-record -p semio-framework-pack-format-catalog --lib --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-quiz-schema --lib --tests --keep-going --message-format=short
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-quiz-schema --lib --features testkit --keep-going --message-format=short
```

(`semio-framework-quiz-schema` is a member of the root `Cargo.toml`, not of `🧰️framework/Cargo.toml`; pack-record and the catalog are framework members.) Add `--target wasm32-unknown-unknown` and `--target wasm32-wasip2` variants with `--lib` for all three crates. Pack tests, existing registered launch commands (no new one was needed):
- `bun repo run @semio-tech/framework-pack-record-rs:test-ownership` and `:test-native` (the unit tests and the laws; run `:test-refusals` and `:test-schema-storage-source` too, they share the crate)
- `bun repo run @semio-tech/quiz-schema-rs:test --configuration=prototype --parallel=1 -- --lib --offline`
- for the catalog, the laws are in `P/📐️format/🧪️tests/🔬️retained-pack-catalog-laws`; they run in the pack-record dependents' native tests and via `cargo test -p semio-framework-pack-format-catalog --lib`.

Hand-off on any compile error: `E0603`, `E0616`, `E0624` name an item that needs `pub(super)` in its child; `E0599` on a trait method means the trait import is missing from that child (the cargo `use super::*` of the unit tests still sees every root import and every privately re-exported item).

## 7. Foreign and pre-existing findings
- `P/🌱️value/🛬️decode/🦀️.rs` is 1,395 lines (a different crate, `pack-record-retained`): the next pack split candidate; not touched.
- `TS7016` for `jsdom` in `🚏️navigation` and the `test-ownership` `tsc --strict` error are pre-existing.
- The test flakiness under load (section 3.4) predates the split in kind; a longer `waitFor` default for the whole-app tests would remove it, but is not part of this slice.
- The machine load stayed between 37 and 150 during the TS runs.

## 8. Files
Created, Rust: the 14 `🦀️.rs` listed in section 5. Rewritten parents: `P/🌱️value/🦀️.rs`, `P/📐️format/🗂️catalog/🦀️.rs`, `Q/🧬️schema/🦀️.rs`.
Created, TS: `Q/🎯️targets/⚛️react/🔨️modules/🧭️session/{🧮️state,💾️restore,📋️session-model}/🟦️.ts`; `Q/🧪️tests/{🎫️quiz-card-challenges,🥇️home-leaderboards,📱️home-text-and-viewport,🛠️home-grid-kit,🎪️pets-on-stage,🎛️pet-allowances,🪛️pet-companions-kit}/🟦️.tsx`. Rewritten: `🧭️session/🟦️.ts`, `🧪️tests/🏠️home-grid/🟦️.tsx`, `🧪️tests/🐾️pet-companions/🟦️.tsx`.
Edited: the 15 importers of the session module (14 under `Q/🎯️targets/⚛️react/🔨️modules/` and `Q/🎯️targets/⚛️react/🟦️.tsx`), 2 `@see` comments, the vitest config and `tsconfig.json` include lists of the quiz-react package, `P/🌱️value/🧪️tests/🧱️ownership/🟦️.ts`, `taxonomy.json`, two doc comments (section 4).
Tools (kept): `F/pqs-tools/`. Snapshots, sandboxes, logs: `F/🗑️generated/pqs-*`.
