# Result c6db1 (census 6, packages P24, P25, P26: rust, os/db)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no cargo/bun repo/nx. `C` = this ticket folder. `DB` = `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db`. Tools: `C/c6db1-tools/` (descendant of `F/c5b-tools`, `F` = `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`). Logs: `C/🗑️generated/c6db1-*` (nothing deleted). Sandboxes and stage trees: `/Users/ueli/.cache/c6db1/`. Last update 2026-10-18 03:5x.

## 0. Verdict

**DONE (static evidence only; nothing compiled).** All 15 files of P24, P25 and P26 are split and installed: 27,535 lines became 28,626 lines (wiring, one docstring per child, re-opened region markers, explicit import lists). Every root is now at most 122 lines, the largest child is 573 lines (`hist/⏳️replay-future`, one 495-line `impl`), no file of the package reaches 1,000 lines. 13 lib roots became `#[path]` children, 2 flat test roots became `include!` fragments. Census 5 found no peer on these files this time; two peers (`c6db2` fleet and an unknown test splitter) touched neighbouring unit-test files while I worked, see section 4.

| tag | file (tail under `DB/`) | owning crate | before | root | children | max child | after (sum) |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: |
| cli | `⌨️cli/🦀️.rs` | `semio-framework-os-kernel-db-cli` | 1,548 | 84 | 5 | 341 | 1,601 |
| query | `🔍️query/🦀️.rs` | `…-db-query` | 1,464 | 90 | 5 | 366 | 1,535 |
| state | `🔘️state/🦀️.rs` | `…-db-state` | 1,505 | 63 | 5 | 397 | 1,550 |
| neo | `🗄️storage/🌐️neo4j/🦀️.rs` | `…-db-storage-neo4j` | 1,497 | 117 | 6 | 358 | 1,557 |
| hist | `🗿️artifact/📜️history/🦀️.rs` | `…-db-artifact-history` | 1,601 | 58 | 4 | 573 | 1,668 |
| fix | `🗄️storage/🚦️io-driver/🧪️tests/🔬️db-io-retained-fixtures/🦀️.rs` | `…-db-io-driver` (test) | 1,559 | 13 | 7 fragments | 279 | 1,574 |
| handle | `⚙️engine/🎫️handle/🦀️.rs` | `…-db-engine-handle` | 1,996 | 72 | 7 | 444 | 2,079 |
| etests | `⚙️engine/🧪️tests/🔬️unit/🦀️.rs` | `…-db-engine` (test) | 1,777 | 13 | 7 fragments | 358 | 1,804 |
| ioreg | `🗄️storage/📇️io-registry/🦀️.rs` | `…-db-io-registry` | 1,972 | 89 | 7 | 415 | 2,078 |
| iodrv | `🗄️storage/🚦️io-driver/🦀️.rs` | `…-db-io-driver` | 1,862 | 80 | 6 | 436 | 1,938 |
| art | `🗿️artifact/🦀️.rs` | `…-db-artifact` | 1,725 | 122 | 5 | 530 | 1,806 |
| ccreate | `⚙️engine/🏗️catalog-create/🦀️.rs` | `…-db-engine-catalog-create` | 2,109 | 63 | 7 | 437 | 2,194 |
| wal | `📝️wal/🧾️log/🦀️.rs` | `…-db-wal-log` | 2,473 | 98 | 8 | 411 | 2,575 |
| compact | `🗜️compact/🦀️.rs` | `…-db-compact` | 2,371 | 119 | 7 | 522 | 2,489 |
| aeng | `🗿️artifact/⚙️engine/🦀️.rs` | `…-db-artifact-engine` | 2,076 | 75 | 7 | 384 | 2,178 |

93 new child folders (`<emoji>️<kebab>/🦀️.rs` next to each root), all RGI (`rgi.mjs` bad=0), unique among siblings and different from the root docstring emoji (`vmod.py`, `chk.py`).

## 1. Method and decisions

- Mechanism: `modsplit.py` (module splits) and `frag.py` (flat test roots) from census 5, in `C/c6db1-tools`. Pre-snapshots `c6db1-tools/orig/<tag>.rs` with `SHA256`; specs `c6db1-tools/specs/<tag>.py`, fragment table `specs5.py`.
- **No glob re-exports** (new in this slice). The census-5 `pub use child::*` is replaced by explicit lists (`modsplit.py explicit_reexports`): `pub use child::{…}` for `pub` items, `pub(crate) use` for `pub(crate)`, a plain `use` for private items that another file of the split scope names, and `#[cfg(test)] use` for names only test code or `#[cfg(test)]` items need. Names reached only by a `crate::NAME` / `super::NAME` path are imported too (the iodrv `crate::controller_pool` case). Items keep their own `#[cfg]` attributes on the import line; `const _` is never imported or widened. `use super::*;` stays in the children (an import, not a re-export); one child that names nothing of its parent (`cli/🔚️command-close`) has none.
- Visibility: private items, fields and inherent methods whose name occurs in another file of the split scope became `pub(super)` (`widen2.py`, 20 to 170 edits per file). `defscan.py` finds 0 cross-file private leftovers in all 13.
- Impl re-opening (declared pre-edit, only a closing `}` and a repeated header line are new): `ccreate` `impl DatabaseCreateCatalogState` (1,020 lines) cut into 3 blocks, `aeng` `impl ArtifactEngine` (928 lines) cut into 4 blocks, at method boundaries outside inner regions.
- Fragment externals: `specs/lib.py mounted()` now also follows `include!("…")` literals, otherwise widening misses names used by already fragmented test files (this was a real defect found by the oracle in `iodrv`).
- Child order equals source order and every child is a contiguous range, so position-sensitive source laws keep their relative order (section 4).

## 2. Evidence

| Check | Tool | Result |
| --- | --- | --- |
| Lossless (line multiset) | `modsplit.py lossless <tag> <live dir>` | LOSSLESS OK 13 of 13, `missing 0`; extras only wiring, docstrings, explicit import lines, region markers, the 2 declared impl re-openings |
| Fragments exact | `chk.py live etests fix` | OK: gone=0, extra(non-marker)=0, test fns 42 = 42 and 29 = 29, include targets equal and existing |
| Structure | `vmod.py live <13 tags>` | VMOD OK: mounted once, docstring emoji, regions balanced, brace depth 0, rustfmt parses every file, path length, no `[DEBUG]` |
| Module clashes | `modclash.py`, `defscan.py` | OK, 0 leftovers |
| Resolution differential | `oracledb.py <group>` (original restored in one sandbox against the live tree in the other, `rustc +nightly-2026-07-20 --emit=metadata`, cfgs lib, test, lib+testkit, test+testkit) | **CLEAN** for cli, query, state, neo, hist, handle, ioreg, iodrv, art, ccreate, wal, compact, aeng and engine (etests): equal error and warning multisets in all 56 comparisons. History: the first run found 3 unused-import warnings, 1 real defect (`db_io_wait_task_retirement`, stale external list) and 1 `crate::` path defect (`controller_pool`); all fixed and re-run. Logs `c6db1-oracle-*.log`, `c6db1-oracle2-*.log` |
| Typeck differential | `posrun.py run families/<k>.json --cfgs lib,test` (round-2 `pos` oracle, base = originals through an overlay, `POS_OVERLAY`) | privacy, trait-scope, resolve, type and other rows: 0 new non-stub rows in all 14 families and both cfgs; typeck canary reached. Remaining NEW = GONE counts (handle 2, iodrv 5/8, art 2, ccreate 2, compact 1, engine test 3) are stub-layer rows that changed only their key. Logs `c6db1-pos-*.out` |
| Source gate | `census-own.ts` (canonical `readRustSourceClosureV1`, lib and cfg(test)+testkit) | all inside the gate. Worst: `db-io-driver` test 16 files, 193,246 bytes, 3,797 lines (ratio 0.949, was 0.923); `db-artifact` test 15 files, 234,345 bytes (0.894); `db-compact` test 0.863. Before and after: `c6db1-census-before.jsonl`, `c6db1-census-after.jsonl` |
| Paths | `modsplit.py paths`, `chk.py` | longest repo-relative path far below 240 UTF-8 bytes, absolute below 256 characters |

## 3. Text readers found and re-pointed

Registered in `POLICY_SPLIT_SOURCE_ROOTS` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts`, 15 anchored insertions after the `📝️wal/🧪️tests/🔬️unit` row, a peer had already added the other db roots): cli, query, state, neo4j, io-registry, io-driver, io-driver fixtures, artifact, history, artifact-engine, engine unit tests, handle, catalog-create, wal-log, compact. This makes `interactivity-audit` (all `INTERACTIVITY_AUDIT_DB_*` files and the storage, engine, artifact and wal families), `verify` and the tool-job coverage readers see the module tree.

Direct `readFileSync` readers switched to `readModuleTreeSource` (found with the new static resolver `tsreaders.py`, 11 hits before, 0 plain reads after):
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts`: `sourceOf` (neo4j and postgres), the artifact root three times, the compact root, the neo4j root.
- `…/🎛️commands/📄️document-checks/🟦️.ts`: artifact root three times, engine unit tests (now fragments).
- `…/🎛️commands/🗄️database-checks/🟦️.ts`: artifact root (+ import).
- `🌎️hub/🧪️tests/🪪️check-in-process/🟦️.ts`: artifact-engine root (+ import).
All four parse with `bun build --no-bundle`.

Rust `include_str!` source laws re-pointed to a `concat!` const of every child (children first, root last, so `find("//#region …")` / first `//#endregion` style slices and order assertions still work) via `reread.py`:
- `🗿️artifact/📜️history/🧪️tests/🔬️unit/🦀️.rs` (11 literals, 2 region ends switched to `rfind`), `⚙️engine/🎫️handle/🧪️tests/🔬️unit/🦀️.rs` (16), `📝️wal/🧾️log/🧪️tests/🔬️unit/🦀️.rs` (2), `⚙️engine/🏗️catalog-create/🧪️tests/🔬️unit/🦀️.rs` (2, region end `rfind`).
- Peer-split fragments: `🗿️artifact/🧪️tests/🔬️unit/📦️durable-group-journal-and-source-laws/🦀️.rs` (4) and `🗿️artifact/⚙️engine/🧪️tests/🔬️unit/↩️preview-query-and-actor-laws/🦀️.rs` (1), const appended at the end of each fragment. Re-read right before editing; a peer re-split of those fragments must keep the const.
- Re-run of the oracle after these edits: CLEAN for hist, handle, wal, ccreate, art, aeng.

Checked and unchanged: `db/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json` (entry paths still exist; the gate test uses the canonical closure counter), `renderer … ShellHelpers` (comments only), `readers.py` found no other reader.

## 4. Open risks and notes

- **Nothing compiled.** The resolution and typeck oracles run with the external crates stubbed or absent, so trait-impl, borrow and macro-hygiene problems are not covered. Compile list below.
- `bun tax.ts` (live taxonomy resolution of the 93 new folders) could not run: `loadTaxonomy` currently fails on unrelated `generatorContracts[...].inputPatterns must be unique and lexically ordered` (a peer is editing the catalog). Names were validated by `rgi.mjs` only; re-run `bun C/c6db1-tools/tax.ts C/c6db1-tools/newdirs.txt` after that catalog is fixed.
- `art` children hold only `#[cfg(not(target_arch = "wasm32"))]` items, so on wasm they are empty modules with an unused `use super::*` warning (warning only, not compiled for wasm here).
- Rust source-law consts are in peer-owned test files (art and aeng fragments, plus the four unit roots); concurrent edits there may drop them.
- `etests` and `fix` fragments keep `use super::*` in the root and the exact test names (42 and 29 test fns).
- Per-root `#[path]` ordering follows spec order; the five roots with `//#region` wrappers keep empty region pairs pruned.

## 5. Crates to compile

`bun repo cargo --store=shared … -- check -p <crate> --lib --tests` (add `--features testkit` for crates that declare it):

`semio-framework-os-kernel-db-cli`, `semio-framework-os-kernel-db-query`, `semio-framework-os-kernel-db-state`, `semio-framework-os-kernel-db-storage-neo4j`, `semio-framework-os-kernel-db-artifact-history`, `semio-framework-os-kernel-db-engine-handle`, `semio-framework-os-kernel-db-io-registry`, `semio-framework-os-kernel-db-io-driver`, `semio-framework-os-kernel-db-artifact`, `semio-framework-os-kernel-db-engine-catalog-create`, `semio-framework-os-kernel-db-wal-log`, `semio-framework-os-kernel-db-compact`, `semio-framework-os-kernel-db-artifact-engine`, `semio-framework-os-kernel-db-engine` (tests only changed).

Source-text lanes to run afterwards: the interactivity audit (`verify`), `os` wal-checks, document-checks, database-checks and the hub check-in-process law.
