# 🧪 RESULT OS-KERNEL-WP8 (Sonnet executor, prefix `okwp8`, 2026-10-16)

Status: COMPLETE as far as static verification reaches. Nothing was compiled: `bun repo cargo` fails at its own start (section 9). The first real `cargo check` is the hand-off.

Paths: `S` = `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store`, `K` = `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust` (kernel facade), `F` = this folder. Tools and snapshots: `F/okwp8-tools/` (kept, `touched.txt` lists every written path); logs: `F/🗑️generated/okwp8-*`.

## 1. Verdict

| Task | Result |
| --- | --- |
| 1. WP8: 18 companion test crates, `pub`-API-only tests (D4), the 12.6k-line `S/🧪️tests/🔬️unit/🦀️.rs` cut by test function, the 2,856-line sync unit test in its companion | Done (section 2, 3). The old unit file is deleted; its content lives in 6 bin chunks (bin-01..06), the member-open chunk, and 2 fixture crates. |
| 2. D5: kernel tests take the fixtures package as dev-dependency, `DemoSnapshot` / `MemberFactory` orphan rule | Done (section 5). The kernel no longer mounts the store-testing tree. |
| 3. Every crate within budget, every file exactly one owner, registration | Budgets met (section 3). Owners: 0 duplicates inside the 16 WP8 crates and the kernel; 3 pre-existing cross-crate shared files remain, listed with proof (section 6). Registration done (section 7). |
| 4. Static verification per Wave 6, test-name multiset | **661 before = 661 after, missing 0, extra 0** (section 4). Compile not run. |

## 2. Design

Crate roles (all `publish = false`, lib roots `#![cfg(test)]` except A and B):

* **A** `semio-framework-os-store-testing-fixtures` (`S/🧪️testing/`, package `📦️packages/🦀️rust`): the shared demo document. Modules `🧭️prelude` (names the store tests import, with trait-in-scope `as _` imports), `🧮️snapshot` (`DemoSnapshot`, `RetainedTextSnapshot` and their `ArtifactDsl` impls), `🧵️operations` (demo operation types and codecs), and the 19 existing `🧬️mutations/**` files (test mounts removed, `dsl::` and `crate::os_spr::` rewritten to the owning crate, `pub(crate)` to `pub`). It does not depend on the kernel facade (dev-dependency cycle avoided).
* **B** `semio-framework-os-store-testing-doubles` (`S/🧪️testing/🎭️doubles/`, new, project `os-store-testing-doubles-rs`): the doubles that need the Store engine. Modules `🗄️store` (test store wrapper, member store producer and wire preparation), `♻️retirement` (retirement doubles, the extracted `close`), `🏭️preparation`, `🧾️callers` (`OriginalMemberCaller`), `📨️envelopes`. D6 split: the 19-file mutation tree plus snapshot and operations stay one-component-per-file in A; the engine-bound doubles are B, so neither crate exceeds the 32 file / 256 KiB / 4000 line budget.
* **10 store bins** `os-store-tests-01..10` (`S/🧪️tests/<bin>`), **2 sync bins** `os-store-sync-tests-1/2`, **member-open** (`S/🧩️composition/🚪️open/🧪️tests/🍰️companion`), **durable-group** (`S/🧩️composition/🗄️durable-group/🧪️tests/🥧️companion`): `#![cfg(test)]` lib roots that mount test files by `#[path]`. The big unit file was cut at item level: types, attached impls, macros and the F-closure of dependencies go to A/B; every `#[test]` unit goes to a chunk file `<bin>/🔬️unit/🦀️.rs`; first-fit packing by bytes, files and lines. Cross-unit sibling references (clamp, raw_replay, reader, canonical_edit, supersede_replay, tool_transaction, replay_retirement) were resolved by extracting `close` into B and grouping the units.
* The 76 previously mounted test files were rewritten in place with path rewrites only (`crate::os_*`, `store::`, `super::tests::`, `::semio_framework_os_kernel::…` to the owning crate or the store test prelude); `include_str!` / `concat!(env!("CARGO_MANIFEST_DIR"), …)` / `.join("…")` fixture paths were rebased. New consolidated fixture tests: `S/🧪️tests/🧪️fixture-{demo,lossy,severity,timestamped,validated}/🦀️.rs` (replacing 14 per-leaf files).
* Kernel: `S/🦀️.rs` lost 35 test `mod` mounts and the `hot_path_census` import; `K/Cargo.toml` takes A as dev-dependency, pruned dev-dependencies that only the removed tests used (ledger, replay, job-worker, cursor, job, schema, blake3, pack, sync, sync-native, tokio-tungstenite).

## 3. Census and budgets (canonical counter `okwp0-tools/closure.ts`, native cfg; lib = production cfg, test = test cfg)

Budget per crate: 32 files, 262,144 B, 4,000 lines.

| Crate | lib files/B/lines | test files/B/lines | test fns | note |
| --- | --- | --- | --- | --- |
| testing-fixtures (A) | 24 / 93,246 / 1,608 | 24 / 93,246 / 1,608 | 0 | corpus row ceiling 1,700 |
| testing-doubles (B) | 10 / 155,581 / 1,753 | 10 / 155,581 / 1,753 | 0 | corpus row ceiling 1,900 |
| tests-01 | 1 / 2,396 / 0 | 17 / 188,874 / 2,248 | 57 | |
| tests-02 | 1 / 2,569 / 0 | 21 / 200,141 / 2,013 | 45 | |
| tests-03 | 1 / 2,058 / 0 | 9 / 190,997 / 2,520 | 57 | |
| tests-04 | 1 / 2,299 / 0 | 7 / 201,023 / 2,132 | 89 | |
| tests-05 | 1 / 2,181 / 0 | 25 / 200,992 / 2,220 | 77 | |
| tests-06 | 1 / 2,387 / 0 | 17 / 204,469 / 1,870 | 38 | |
| tests-07 | 1 / 2,148 / 0 | 14 / 229,117 / 1,023 | 27 | |
| tests-08 | 1 / 2,615 / 0 | 27 / 191,135 / 1,781 | 30 | |
| tests-09 | 1 / 2,523 / 0 | 15 / 180,743 / 1,977 | 35 | |
| tests-10 | 1 / 2,598 / 0 | 12 / 94,894 / 825 | 24 | |
| sync-tests-1 | 1 / 2,085 / 0 | 5 / 222,635 / 3,216 | 49 | wasm cfg 4 / 219,805 / 1,266 |
| sync-tests-2 | 1 / 2,302 / 0 | 9 / 209,924 / 2,311 | 40 | |
| member-open-tests | 1 / 2,722 / 0 | 21 / 214,321 / 2,172 | 20 | |
| durable-group-tests | 1 / 2,424 / 0 | 11 / 213,023 / 2,295 | 38 | |
| **os-kernel facade** | 5 / 111,318 / 1,397 | 16 / 144,626 / 1,681 | 6 | baseline: lib 5 / 116,802 B / 1,629; test 241 files / 3,163,369 B / 34,338 lines |

All 17 rows are inside the budget (largest: bytes 229,117 in tests-07, files 27 in tests-08, lines 3,216 in sync-tests-1). Log: `F/🗑️generated/okwp8-census.out`.

## 4. Test-name multiset (`okwp8-tools/testcount.py`, attribute-level count of `#[test]` / `async_test` / `tokio::test` fns)

* before (kernel lib test closure, pristine list `F/🗑️generated/okwp8-tests-before.tsv`): **661**; after (union of the 16 crate closures plus the kernel's own remaining closure plus the kernel `[[test]]` public targets): **661**; `missing 0 {}`, `extra 0 {}`.
* Distribution: 626 in the 14 test crates (column `test fns` of section 3), 6 stay in the kernel lib, 29 sit in the kernel `[[test]]` targets `sqlite_snapshot_native_admission` and `artifact_command_retirement` (section 6): 626 + 6 + 29 = 661.
* Item-level losslessness (multiset of top-level items of the old sources against the new crates): `okwp8-tools/lost.py`, no lost item.

## 5. D5, orphan rule

* The kernel tests dev-depend on A; the `S/🧪️tests/🔬️unit` and store-testing mounts are gone from `S/🦀️.rs`. `S/📦️codec/🎮️operation/🧪️tests/🦀️.rs` imports `semio_framework_os_store_testing_fixtures::{DemoSnapshot, demo::DemoMutation}`.
* `DemoSnapshot` family: the `ArtifactDsl for RetainedTextSnapshot` impl moved to A, next to the type.
* `MemberFactory` / `MemberStoreProducer` orphan rule: `DemoMemberStoreProducer<Op>` and every impl live in B, with the type. The remaining impls in the bins are `impl MemberStoreProducer<DemoSnapshot, LawOp> for DemoMemberStoreProducer<LawOp>` (tests-08 `supersede-law`, sync-tests-2 `replay-retirement`, durable-group `deferred-reprojection`): the trait is foreign, `Self` is foreign, but the trait argument `LawOp` / `FailClosedOp` / `CountedOp` is a local type with no uncovered type parameter before it, so rustc accepts them (orphan audit in `audits.py`: only these three, all local-argument).

## 6. Duplicate-mount scan (every file exactly one owner)

Method: closure rows of the 16 crates and the kernel (`census-<crate>.json`, non-embedded) give 119 owned `.rs` files; owner count per path: **0 paths owned by more than one of them**. Then `rg` over all `#[path = …]` declarations of the repo (37,883), resolved relative to the declaring file, counted for the 119 owned files.

Fixed in this slice:
* `S/📬️command/♻️retirement/🧪️tests/🦀️.rs` was mounted by bin-03 and by the kernel `[[test]] artifact_command_retirement`: bin-03 mount removed; the `[[test]]` is the owner.
* `S/📦️codec/🪶️snapshot-capability/🪶️native-{encoding,decoding}/🧪️tests/🦀️.rs` were mounted by the bin-10 aggregator `🧪️tests/🦀️.rs` and by the kernel `[[test]] sqlite_snapshot_native_admission`: the aggregator's two mounts are removed (it keeps its own test); the `[[test]]` is the owner. (Both targets pre-existed; the kernel lib used to carry the second copy.)

Remaining, all pre-existing at `HEAD` and across crates I cannot make dependents of each other (verified with `git show HEAD:`):
1. `🪐️space/🧪️tests/🪶️sqlite/🔬️oracle/🦀️.rs` and `…/📏️preflight/🦀️.rs`: mounted by `S/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` (bin-07 owns that file) and by two plugin-side artifact snapshot tests (`🗂️collection`, `🪐️space`). Three crates.
2. `S/👥️presence/🧪️testing/♻️retirement/🦀️.rs`: mounted by bin-06, by the `🫧️ephemeral` crate root and by `🌱️value/📸️snapshot/👁️read/🦀️.rs`. The value crate is below the Store, so it cannot depend on a Store test-support crate. Fix needs a new base-level helper crate or deleting two of the three users.
3. nothing else in the 119 files.

## 7. Registration

* `Cargo.toml` (root): B added as member (the 16 other WP8 crates were already members; `verify_reg.py` confirms unique/sorted for WP8; the only FAIL is a pair of peer members out of byte order, not mine).
* `📋️build-owners.json`: row for B (the other rows existed).
* `project.json`: new for B (`@semio-tech/os-store-testing-doubles-rs`); the other 16 existed.
* `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: 6 routes for B (build/check/test x prototype/release, existing order and naming); the other 16 crates already had their 6 routes. Gate `⚖️gate🏢️semio-tech🎡️play🏪️store-owners` (it ran the kernel filter `os_store::component::tests`) now runs `bun repo run-many -t test --projects=<tests-01..10, sync-tests-1/2, member-open, durable-group> --skip-cache --skip-remote-cache --parallel=1 -- --lib --nocapture` in both files.
* `Cargo.lock` (root): blocks hand-derived for the 16 crates and the kernel (no cargo ran).
* Corpus `K/../🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json`: rows for A (ceiling 1,700 lines) and B (1,900). `bun test` of `🧪️tests/📦️crate-boundaries/🟦️.ts`: both new rows pass (manifest name, lib path, 6 launch routes, forbidden dependencies, source ceilings, forbidden tokens); the whole file: 106 pass with the default 5 s timeout, the 5 remaining failures are the "enforce active source caps" cases which only time out under load and pass (6 pass, 0 fail) with `--timeout 120000`. The test crates have no corpus rows (`#![cfg(test)]` roots, same as the earlier companion crates).
* Taxonomy: `smg-tools/taxo.ts` resolved all 326 directories under the five touched roots; 318 RESOLVED, the 8 PROBLEM rows are old fixture-case directories (`🧫️fixtures/basic-remote-ops` etc.), not mine. No taxonomy edit needed.

## 8. Readers and pins re-pointed (old unit file `S/🧪️tests/🔬️unit/🦀️.rs` and `S/🦀️.rs` mounts)

Absolute references to the old file are now 0 (`rg -F 'store/🧪️tests/🔬️unit/🦀️.rs'`). Patched, by reader:
* `✏️s/…/✏️editing/🧪️tests/🔬️unit/🟦️.test.ts` to the bin-03 chunk.
* `🧰️framework/🔨️modules/🧵️job/🎟️context/🪙️loan/🧪️tests/🟦️.ts` to the bin-01 + bin-06 chunks.
* `S/🧩️composition/📨️emission/📦️owned/🧪️tests/🟦️.ts`: helper `readStoreUnit` over A, B and the 8 chunks (7 reads + the rustfmt list); one pin rebased: `ExactDemoInitialSnapshotRetirementFactory` is now bin-02 local, the two function slices end at `\n}\n` instead of a sibling that moved.
* `S/🧩️composition/🚪️open/🧪️tests/🔬️unit/📏️retirement/🟦️.ts` (4 reads to bin-06).
* `S/🧵️canonical-edit/🧪️tests/🔬️canonical-error-progress/🟦️.ts` (B store, B preparation, bin-03 + bin-06 with a rustfmt loop).
* `S/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` (bin-06), `S/🧩️composition/🚪️open/📜️history/🏭️factory/🧪️tests/🛬️receiving/🟦️.ts` (B callers), `S/🧪️tests/🔬️unit/🟦️.ts` (rustfmt over the 16 unit-origin files; 8/8 tests pass).
* `S/🏗️initialization/🏠️owners/🏭️member/📥️source/🔣️.json` and `🧬️schema/🔣️.json` (path and const to bin-05; JSON parses).
* `K/🖥️host/📦️packages/🦀️rust/📜️script.ts`: `storeTests` now reads the member-open chunk (the generated `RetainedTestMembers` slice equals the fixture's `generatedDeclarations`, 2 rows checked with bun); the open/mod-tests pin now checks `mod store_unit_tests;` in bin-02; the macro pin `super::super::super::super::tests::` is now `super::store_unit_tests::`.
* Two real bugs found by the reader sweep and fixed in the moved test files: `super::super::super::tests::OriginalMemberCaller` (receiving) and `super::super::super::tests::RetainedTestMembers` (factory) did not resolve in the companion; now `semio_framework_os_store_testing_doubles::OriginalMemberCaller` and `super::store_unit_tests::RetainedTestMembers`.
* Exact-law selectors (`runRepositoryExactCargoLaws`): every group that addressed `semio-framework-os-kernel` with a moved test now addresses the hosting crate with the bare test name (selector matching is `name === s || name.endsWith("::s")`): hub gis `durable-three-store-assembly`, `member-dialect-checks`, `document-checks` (durable 16 laws; the sync law was done before), `database-checks`, `source-and-native-tests`, host `📜️script.ts` (4 groups). Split groups keep a kernel group for the laws that stay (`lawscan.py` / `lawgroups.py` report none left).
* Widening (section 10) touched no TS pin: 180 removed lines of the 23 widened files were searched in all TS/JSON files; the only hit is the generic `pub(crate) fn new(` inside an unrelated renderer file.

Test runs (bun, 2026-10-16): the readers above that I touched pass where they read only the moved files (emission `counted originals`; retirement pins checked with an old-vs-new differential on the same 4 pins, identical results; factory/receiving fixture test; `S/🧪️tests/🔬️unit/🟦️.ts` 8/8; crate-boundaries). The other tests in those files fail the same way they did at `HEAD` because they read S-root or plugin text that earlier waves moved (e.g. emission 9 of 25, retirement 39 of 63, canonical-error-progress 6 of 15); I checked three of them against the pristine S root to confirm they predate this slice, not all. Logs: `F/🗑️generated/okwp8-bun-*.log`.

## 9. Commands run

* `python3 okwp8-tools/census.py`, `testcount.py`, `verify_reg.py`, `audits.py` (bound names, orphan, path existence, definition scan), `lost.py`, `lawscan.py`, `lawgroups.py`, `lawrewrite.py --apply` (6 files), `retarget.py` patches.
* `rustfmt --edition 2021 --emit stdout --config skip_children=true` on the edited Rust files (receiving, factory, aggregator, bin-03) and, through bun tests, on all 16 unit-origin files: parse OK.
* `bun test` of the files named in section 8 and of the crate-boundaries corpus.
* `bun repo cargo --store=shared -- cargo check -p semio-framework-os-store-testing-fixtures --lib --offline`, run under the fleet mutex (`🗑️generated/cargo-mutex.lock`, taken and released): **exit 1** before any compilation, `error: Exact canonical persistent Native publication required` at `inspectPersistentReceiver` (`🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts:108`). `bun repo cargo --help` fails the same way, so this is the repo tool's native-image store, not this slice. Log: `F/🗑️generated/okwp8-cargo-check-fixtures.log`.
* No git command that modifies anything, no ticket tool, no sub-agent.

## 10. Widening (visibility needed because the tests are now outside the owning crate)

136 added lines in 23 carved-crate files (123 items; nothing else widened): `pub(crate)` or private to `pub` on fields and methods that the moved tests reach by dot or associated path. Files (all under `S/` except the last): composition `🚪️open/🏭️operation/📦️catalog`, store root `🦀️.rs`, `♻️retirement/📦️backing`, `📒️ledger` (3), `📦️codec` (2 + owned-schema fields lifted), `📬️one-item-owners`, `🔁️replay`, `🔄️sync` (2), `🔗️backbone`, `🔗️read`, `🚪️io/💾️binary/🌱️genesis`, `🧩️composition/🗄️durable-group` (root 13 lines, wire 50 lines), `🧩️composition/🦀️.rs`, `🧬️snapshot-clone`, `🧵️operation-wire`, `🫧️ephemeral/📢️publication/🧩️preparation`, and `🪡️list-delta`. Chosen by a definition scan (field and method visibility against the names the test crates use, `defscan.py`, `widen.py`); an item widened only when the test crate uses it and the owner is not local to the test.

## 11. Residual risks (nothing compiled)

1. Field and method privacy is resolved by name only. A test that reaches a private field or method of a type with the same name in another crate can still raise E0616 / E0624 at the first real build.
2. Hand-derived `Cargo.lock` blocks: `cargo update --workspace --offline` may reorder or add a dependency edge; the workspace-members out-of-order pair belongs to peers.
3. The kernel `[[test]]` targets (section 6) compile files that I rewrote to canonical crate paths; they need those crates (all normal kernel dependencies) but were never built after the rewrite.
4. The source-text law (`document-checks`, "authority fields are private") fails at `HEAD` and after, because live authority fields are `pub`; unchanged.
5. `artifact_retire_struct!(HistoryColumn…)` in the S root and the S-root region imbalance are pre-existing.
6. Two bin roots (`bin-07..10`) mount other moved tests only; they hold no `🔬️unit` chunk by design.
7. Cross-crate shared harness files of section 6 (items 1 and 2) stay multi-owned.

## 12. Hand-offs

* First real `cargo check` / `cargo test --lib --no-run` of A, B, the 14 test crates and the kernel once `bun repo cargo` starts; expect only privacy and import findings (risk 1).
* `cargo update --workspace --offline` for `Cargo.lock`.
* A base-level shared test-helper crate (or removing two users) for the presence retirement helper and the sqlite oracle/preflight files (section 6).
* The stale tests listed in section 8 (S-root and plugin text pins from earlier waves) are not WP8 and were not changed.

## 13. Files

Created (23 `A` in `okwp8-tools/touched.txt` plus the chunk roots and modules written through the same writer): crate roots, chunk files `<bin>/🔬️unit/🦀️.rs`, B (root, 5 modules, `Cargo.toml`, `📋️project.json`), A modules `🧭️prelude`, `🧮️snapshot`, `🧵️operations`, 5 fixture tests. Deleted (16): `S/🧪️tests/🔬️unit/🦀️.rs`, 14 `🧪️tests/🧪️fixture-<family>-<leaf>/🦀️.rs`, the fixtures-package placeholder root. Modified: 147 paths (`touched.txt`, `sort -u` 179 unique entries). Pristine copies of everything the tools wrote: `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/okwp8-pre/` (session scratch, may disappear).
