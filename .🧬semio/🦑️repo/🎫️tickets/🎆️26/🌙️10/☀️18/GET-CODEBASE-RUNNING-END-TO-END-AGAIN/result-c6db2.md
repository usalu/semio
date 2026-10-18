# Result c6db2 (census 6, packages P27, P28, P29: rust, os/db)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no cargo / `bun repo` / nx. `C` = this ticket folder. `DB` = `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db`. Tools: `C/c6db2-tools/` (descendant of `c6db1-tools`, which descends from `F/c5b-tools`; `F` = `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`). Sandboxes and stages: `/Users/ueli/.cache/c6db2/`. Logs: `C/🗑️generated/c6db2-*`.

## 0. Verdict

**DONE (static evidence only; nothing compiled).** All 18 files of P27, P28 and P29 are split and installed on the live tree: 14 module splits (`#[path]` children, explicit import lists, no glob re-exports) and 4 flat test roots (`include!` fragments). 27,335 lines became 28,260 lines (wiring, one docstring per child, re-opened region markers, explicit import lists). Every root is at most 110 lines, the largest child is 653 lines (`sync/🎞️hello-replay-tail-and-execution`), 17 of the 18 files have no child over 600 lines. One source-gate fact the coordinator must know: the `semio-framework-os-kernel-db-sync` test closure is at 3,973 of 4,000 active lines (0.993); see section 4.

| tag | file (tail under `DB/`) | owning crate | before | root | children | max child | mechanism |
| --- | --- | --- | ---: | ---: | ---: | ---: | --- |
| cread | `⚙️engine/📖️catalog-read/🦀️.rs` | `…-db-engine-catalog-read` | 1,337 | 43 | 5 | 506 | modules |
| copen | `⚙️engine/🔭️capability-open/🦀️.rs` | `…-db-engine-capability-open` | 1,316 | 45 | 5 | 545 | modules |
| cboot | `⚙️engine/🥾️catalog-bootstrap/🦀️.rs` | `…-db-engine-catalog-bootstrap` | 1,430 | 45 | 5 | 502 | modules |
| wtests | `📝️wal/🧪️tests/🔬️unit/🦀️.rs` | `…-db-wal` (test) | 1,420 | 11 | 5 | 360 | fragments (25 tests) |
| snap | `📸️snapshot/🦀️.rs` | `…-db-snapshot` | 1,422 | 96 | 4 | 478 | modules |
| pg | `🗄️storage/🐘️postgres/🦀️.rs` | `…-db-storage-postgres` | 1,336 | 108 | 7 | 308 | modules (macro stays in the root, mounts wired after it) |
| atests | `🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | `…-db-artifact` (test) | 1,318 | 10 | 4 | 430 | fragments (24 tests) |
| eng | `⚙️engine/🦀️.rs` | `…-db-engine` | 1,067 | 102 | 5 | 349 | modules, 600-line `impl Database<E>` cut into 2 blocks |
| stests | `🔄️sync/🧪️tests/🔬️unit/🦀️.rs` | `…-db-sync` (test) | 1,034 | 6 | 3 | 450 | fragments (44 tests) |
| fs | `🗄️storage/📁️fs/🦀️.rs` | `…-db-storage-fs` | 1,287 | 39 | 7 | 375 | inline `mod fs_storage` moved to a shell folder with 6 children |
| mem | `🗄️storage/🗃️memory/🦀️.rs` | `…-db-storage-memory` | 1,165 | 54 | 4 | 537 | modules |
| sqlite | `🗄️storage/🪶️sqlite/🦀️.rs` | `…-db-storage-sqlite` | 1,135 | 12 | 5 | 442 | inline `mod sqlite_storage` moved to a shell folder with 4 children |
| aetests | `🗿️artifact/⚙️engine/🧪️tests/🔬️unit/🦀️.rs` | `…-db-artifact-engine` (test) | 1,013 | 9 | 3 | 369 | fragments (32 tests) |
| contract | `🗿️artifact/🧬️contract/🦀️.rs` | `…-db-artifact-contract` | 1,069 | 54 | 5 | 413 | modules |
| sync | `🔄️sync/🦀️.rs` | `…-db-sync` | 2,821 | 95 | 6 | 653 | modules |
| index | `🔢️index/🦀️.rs` | `…-db-index` | 2,764 | 88 | 8 | 578 | modules |
| stor | `🗄️storage/🦀️.rs` | `…-db-storage` | 1,036 | 108 | 4 | 277 | modules (fault storage mod carries `cfg(any(test, feature = "testkit"))`) |
| ioc | `🗄️storage/🧱️io-core/🦀️.rs` | `…-db-io-core` | 3,365 | 110 | 11 | 519 | modules (`thread_local!` item and `claimed_backend` mount stay in the root) |

96 new child folders (`<emoji>️<kebab>/🦀️.rs` next to each root; for fs and sqlite one shell folder `💽️fs-storage` / `💽️sqlite-storage` plus its children), all RGI (`rgi.mjs` bad=0), unique among siblings and different from the parent docstring emoji (`vmod.py`, sibling scan), and all 96 resolved by the live taxonomy (`tax.ts`: 96 dirs, 0 unresolved, using `readTaxonomyUnchecked`, because `loadTaxonomy` currently fails on an unrelated `generatorContracts[...].inputPatterns` ordering problem).

## 1. Method and decisions

- Same mechanism as census 5 and `c6db1`: `modsplit.py` (module splits, `inner` for the inline modules), `frag.py` (flat test roots), snapshots in `c6db2-tools/orig/<tag>.rs` (`SHA256`, `files.tsv`), specs `c6db2-tools/specs/<tag>.py`, fragment table `specs5.py`.
- **No glob re-exports.** The parent lists the moved names explicitly (`pub use child::{…}`, `pub(crate) use`, a plain `use` for items another file needs). Improvement over c6db1's list builder: a plain `use` is only emitted when a sibling child, the parent or a test file really references the name as a free identifier (not after a `.`); names referenced only by test files get `#[cfg(test)]`. This removed all unused-import warnings the oracle found (fs, sqlite). Wrap width of the lists is 180 columns (fewer wiring lines; this mattered for the source gate).
- Visibility: private items, fields (also tuple fields) and inherent methods whose name occurs in another file of the scope became `pub(super)`; `const _` items are never widened (the oracle found the lint "visibility qualifiers have no effect on `const _`").
- Impl re-opening (declared pre-edit): `eng` `impl<E: Emit + 'static> Database<E>` cut into 2 blocks at a method boundary. All other large `impl` blocks (500 to 531 lines) stay whole.
- Inline modules: `fs_storage` (1,259 lines) and `sqlite_storage` (1,126 lines) became `#[path = "💽️…/🦀️.rs"] mod …;` with a shell file holding the module's `use` lines, the `mod` mounts and the explicit imports; `super::` paths of the children were rebased one level (`rebase_super`), the multi-line SQL string of `SCHEMA` was not dedented, the `include!("…/directory-durability")` and `include!("…/sqlite-storage-unit")` test fixtures stay in the shell with one more `../`, and their text counts as an external test file for the widening/import decisions.
- Macro scope: `pg`'s `macro_rules! with_admitted_artifact` stays in the root and the `mod` block is wired after it (`wire_at`); `thread_local!` in `ioc` stays in the root (it exports a `pub static`).
- Tool fixes made on the way (all in `c6db2-tools`): `mounted()` also follows `include!("…")` literals (otherwise names used only by already fragmented test files were not widened: found by the oracle in `sync` test, 17 errors), `const _` never widened, usage heuristic above, `wire_at`, `innerlist.py`, `activelines.py`, inner-module aware `vmod.py`, `oracle5.py` groups for the 14 crates and 4 test roots.

## 2. Evidence (run on the live tree after the final install)

| Check | Tool | Result |
| --- | --- | --- |
| Lossless | `modsplit.py lossless <tag> <live dir>` | `missing 0`, LOSSLESS OK for all 14 module splits; extras are only `#[path]`, `mod`, explicit `use` lists, child docstrings, `use super::*;`, re-opened region markers and the declared impl pair |
| Fragments exact | `chk.py live wtests stests` | OK (gone=0, extra non-marker=0, test fns 25=25 and 44=44, include targets equal and existing). `atests`/`aetests` were OK at install (24=24, 32=32); a peer (`c6db1`) later appended source-law `const`s to one fragment each (section 3), so `chk.py` now reports their 13 and 12 extra lines |
| Structure | `vmod.py live` (14 tags) | VMOD OK: every child mounted once, docstring emoji first/equal to the folder/unique/different from the parent, regions balanced per file, brace depth 0, rustfmt parses every file, path length (repo-relative UTF-8 <= 240, absolute <= 256), include targets, no `[DEBUG]`, module-name clash |
| Definition scan | `defscan.py`, `modclash.py` | 0 cross-file private leftovers in the 12 flat-module tags (defscan does not read inline modules; their members were widened by the same planner and the oracle covers them), modclash OK |
| Resolution differential | `oracle5.py <group>` (original restored in one sandbox against the live tree in the other, `rustc +nightly-2026-07-20 --emit=metadata`, cfgs lib and test, extern crates stubbed away) | **CLEAN for all 18 groups**: equal error and warning multisets in all comparisons (examples: sync test 440/440, eng test 505/505, ioc lib 7/7). History: the first run found 1 real defect class (sync test: 17 unresolved names, fixed by the `mounted()` include fix), 4 unused-import warnings (fs, sqlite, fixed), 2 `const _` visibility warnings (ioc, index, fixed). Log `C/🗑️generated/c6db2-oracle5-final.log` |
| Typeck oracle | `posrun.py run families/c6db2.json --cfgs lib,test --label post2 --no-base` (stub layer for everything outside the 16 crates) | typeck canary reached (E0308, E0616, E0624). lib: privacy 0, resolve 2 and type 41 are stub rows. test: privacy 1 = `E0624 writer_admission_census is private` in the engine unit tests: **pre-existing** (`pub(crate) fn writer_admission_census` in `fs.rs` line 1039 of the snapshot, called from the engine crate's tests), trait-scope rows are `finish` on a stubbed `_` type and `WalRef::acquire_writer` in the artifact tests whose imports are unchanged. No row in a moved child is attributable to the split. Log `C/🗑️generated/c6db2-pos-post2.log` (the run is absolute, there is no pre-run: the originals were already installed when the oracle was set up) |
| Source gate | `census-own.ts` (canonical `readRustSourceClosureV1`, lib and cfg(test) closures) | all 16 owning crates inside 4,000 lines / 32 files / 256 KiB. Worst: **sync test 11 files, 213,564 bytes, 3,973 lines (0.993, was 0.964)**, ioc test 15 files 3,822 lines (0.956, was 0.918), artifact test 0.894 (c6db1 + fragments), engine test 0.824. Before and after: `C/🗑️generated/c6db2-census-{before,after}.jsonl` |
| Paths | `modsplit.py paths`, `vmod.py` | longest repo-relative path far below 240 UTF-8 bytes, absolute below 256 characters |
| Anchor census | `anchor_counts.py` (every string literal of the audited TS files that occurs in a snapshot, compared by count with the live module tree) | 231 count differences, 8 relevant: the region markers handled in section 3, 2 re-export names that now also appear in the root, and one generic `} else {` of an unrelated audit. No audit anchor with a declaration prefix was lost |

## 3. Text readers found and re-pointed

Methods: `readers.py` and `grepn.py` (path suffix), a static resolver for `readFileSync(join(owner, …))` in the `os` commands, `moved_literals.py` and `anchor_counts.py` (literal census), a Rust scan for `include_str!` / `read_to_string` of the 18 roots, a region-marker scan (`regionreaders`).

- `POLICY_SPLIT_SOURCE_ROOTS` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts`): 18 anchored entries added after the `app-surface` row (engine, catalog-read, capability-open, catalog-bootstrap, sync, sync tests, index, snapshot, storage, io-core, fs, memory, sqlite, postgres, contract, artifact unit tests, artifact-engine unit tests, wal unit tests). The interactivity audit, `verify` and the evidence readers now read these files as the one text the unsplit file held. `c6db1` appended its roots below mine.
- Direct `readFileSync` readers switched to `readModuleTreeSource` (each file parses with `bun build --no-bundle`):
  - `…/📦️packages/🦀️rust/🎛️commands/📄️document-checks/🟦️.ts`: engine root (2 reads), sync root, artifact unit-test root (2 reads) (+ import).
  - `…/🎛️commands/✍️wal-checks/🟦️.ts`: storage root (2 reads), sqlite root (2 reads), postgres root, engine root, wal unit-test root (+ import). `c6db1` had already converted `sourceOf`.
  - `…/🎛️commands/🗄️database-checks/🟦️.ts`: engine root.
- Region-cut readers: a region that wraps several children is re-opened in each child, so `text.slice(indexOf("//#region X"), indexOf("//#endregion X"))` stopped at the first child. New helper `interactivityRegionSource(text, label)` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🟦️.ts` returns the region with every junction (closing marker, skipped docs/`use super::*;`/outer markers, reopening marker) removed, `null` when absent. Unit-checked with `bun` against the snapshot texts: `CatalogRootRead`, `CapabilityOpen`, `CatalogBootstrapCas` and `RetainedHello` return the same lines as the unsplit file (0 missing, 0 extra). Applied to: `interactivityDatabaseCapabilityOpenFailures`, `interactivityDatabaseCatalogReadFailures` (database file), `interactivityDatabaseCatalogBootstrapFailures`, the sync-hello region and, for the `c6db1` splits whose regions are cut the same way, `HistoryReplay`, `Actor` (`🌱️database-lifecycle`) and `ArtifactHandle` (database file).
- Rust source laws that `include_str!` a split root (re-pointed to a `concat!` of every child, children first, root last, as `c6db1` did; `concat!(include_str!(…))` verified with `rustc`): `⚙️engine/📖️catalog-read/🧪️tests/🔬️unit/🦀️.rs`, `⚙️engine/🔭️capability-open/🧪️tests/🔬️unit/🦀️.rs`, `⚙️engine/🥾️catalog-bootstrap/🧪️tests/🔬️unit/🦀️.rs` (retry-guard scan), `🗿️artifact/🧬️contract/🧪️tests/🔬️unit/🦀️.rs` (new `CONTRACT_SOURCE` const used by two tests). `⚙️engine/🏗️catalog-create/🧪️tests/🔬️unit/🦀️.rs` (c6db1's file) read the engine root for a `catalog`/`health` slice: now reads `🛠️database-catalog-shutdown-and-hello/🦀️.rs`, where both fns live.
- Checked and unchanged: `🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json` (entry paths still exist), the hub/os/plugin TS files that matched generic literals only (checked by hand), `Cargo.toml` manifests, `📋️build-owners.json`, `🔒️dependencies.json`, launch files.

## 4. Open risks

- **Nothing compiled.** The resolution and typeck oracles run with the external crates stubbed or absent, so trait-impl coherence, borrow, lifetime, macro-hygiene and wasm-only cfg problems are not covered. `node_modules` is currently a dangling symlink, so the TS lanes below could not be executed; only parse checks (`bun build --no-bundle`) and the isolated unit check of `interactivityRegionSource` ran.
- **`semio-framework-os-kernel-db-sync` test closure: 3,973 / 4,000 active lines.** The split adds ~118 wiring lines to a closure that was already at 3,855. Any growth of 27 lines in the sync lib or its unit tests breaks the gate. Recommended follow-up (not done, needs manifests and a compile): carve the retained sync-hello machinery (≈2,100 lines) into a sibling crate. `io-core` test (3,822) and `artifact` test (c6db1's, 3,388 + fragments) are the next closest.
- `sync/🎞️hello-replay-tail-and-execution` is 653 lines (target 600, hard limit 1,000): it was merged from two children to buy 16 lines of gate headroom.
- `atests`/`aetests` fragments: `c6db1` appended `ARTIFACT_SOURCE` / `ARTIFACT_ENGINE_SOURCE` consts to one fragment of each; a re-split must keep them. `chk.py` against my specs reports those lines as extra.
- Pre-existing, unchanged by this slice: `FsStorage::writer_admission_census` is `pub(crate)` but called from the engine crate's tests (E0624 under the typeck oracle); `walSegmentStateOracle` in `wal-checks` asserts strings that no longer exist in the storage root (it reads `🗄️storage/🦀️.rs`, its markers live in io-core).
- Not verified at runtime: the interactivity audit (`verify interactivity` and the self-test lanes for sync-hello, compaction, db-io, catalog read/bootstrap/open), the `os` commands `wal-checks`, `document-checks`, `database-checks`, `bun test` of the module-tree reader.
- `fs`/`sqlite` children are `cfg(not(wasm32))` through their shell mount; the `defscan` cross-file private scan does not read inline-module shells (covered by the oracle only).

## 5. Crates to compile

`bun repo cargo --store=shared … -- check -p <crate> --lib --tests` (add `--features testkit` where the crate declares it; marked `*`):

`semio-framework-os-kernel-db-engine-catalog-read*`, `semio-framework-os-kernel-db-engine-capability-open*`, `semio-framework-os-kernel-db-engine-catalog-bootstrap*`, `semio-framework-os-kernel-db-wal*` (tests only changed), `semio-framework-os-kernel-db-snapshot*`, `semio-framework-os-kernel-db-storage-postgres`, `semio-framework-os-kernel-db-artifact*` (tests only changed), `semio-framework-os-kernel-db-engine`, `semio-framework-os-kernel-db-sync*`, `semio-framework-os-kernel-db-storage-fs`, `semio-framework-os-kernel-db-storage-memory*`, `semio-framework-os-kernel-db-storage-sqlite`, `semio-framework-os-kernel-db-artifact-engine` (tests only changed), `semio-framework-os-kernel-db-artifact-contract`, `semio-framework-os-kernel-db-index`, `semio-framework-os-kernel-db-storage*`, `semio-framework-os-kernel-db-io-core*`.

Source-text lanes to run afterwards: the interactivity audit (`verify interactivity`, p1q-b1-b6, p1w, p1x, p1y, p1z), `os` wal-checks, document-checks, database-checks, and `bun test` of `🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree`.

## 6. Files

- Created: 96 child folders with one `🦀️.rs` each next to the 18 roots (list: `C/c6db2-tools/newdirs.txt`).
- Edited (18 roots): the files of the first table.
- Edited (readers and laws): `…/📖️source-access/🟦️.ts` (18 roots), `…/🔬️interactivity-audit/🗄️database/🟦️.ts`, `…/🔬️interactivity-audit/🗄️database/🌱️database-lifecycle/🟦️.ts`, the three `🎛️commands` files above, the four Rust unit-test roots above, `⚙️engine/🏗️catalog-create/🧪️tests/🔬️unit/🦀️.rs` (one line).
- Tools, kept: `C/c6db2-tools/` (`modsplit.py`, `frag.py`, `vmod.py`, `chk.py`, `oracle5.py`, `posrun.py` + `pos-tools/`, `census-own.ts`, `tax.ts`, `rgi.mjs`, `anchor_counts.py`, `moved_literals.py`, `activelines.py`, `innerlist.py`, `specs/`, `specs5.py`, `orig/`, `families/c6db2.json`, `audit-probe.ts` (needs `node_modules`)). Logs: `C/🗑️generated/c6db2-*`.
