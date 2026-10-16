# 🏭 Result OS-KERNEL-ENGINE: recovery of A1, carve of WP5/WP6, facade reduction

Executor: Sonnet 5.5, foreground, no sub-agents, no git-modifying command, no ticket tool. Slice prefix `okeng` (tools `okeng-tools/`, logs `🗑️generated/okeng-*`). Resumes `okwp56-tools/` (previous executor, cut about 04:16). Plan: `PLAN-OS-KERNEL-CRATES.md` (5.3, 6, 7, 10.1 WP5/WP6, 13). `K` = `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust`, `M` = `🧰️framework/🛍️products/💻️os/🔨️modules`, `S` = `M/🏪️store`.

## 0. Verdict (rewritten after each milestone)

**Status 07:00: A1 recovered and consistent, A2 applied (18 crates carved, kernel reduced to a gate-compliant facade by the census), statically verified; NOT compiled** (the shared Rust image is below the 4 GiB initial-headroom gate, see section 6; the fleet mutex protocol is followed and a retry is pending).

* A1 (the `ArtifactStore` composer of six component structs) was fully applied by the previous executor and needed no repair, except two inconsistent signatures in the durable group (`bind`, `recover`), fixed.
* A2 carved **18 crates** (the 17 of `okwp56-tools/crates56.txt` plus `os-store-member-open`, which `os-store-engine-owners` depends on and the previous list had left out): `initialization-runtime`, `durable-group-wire`, `member-open`, `engine-owners`, `engine-core`, `engine-live`, `engine-stage`, `engine`, `codec-factory`, `initialization-replay`, `space-member`, `document`, `member-open-store`, `config`, `durable-group`, `composition`, `space`, `test-support`. Rank order of `okcrates.RANK`, 0 back edges, no cycle (`okeng-tools/graph.py`).
* Store root `S/🦀️.rs` 11,416 -> 1,345 lines. Census (lib closure): **kernel 5 files / 1,624 lines / 116,845 bytes, `overLimit=false`** (before: 56 / 26,162 / 1,775,815), all 18 crates inside the hard limits (table in section 4). `os-store-engine` is 3,452 lines / 254,589 bytes: inside the hard limits (4,000 / 262,144) but 10.7 percent above the 230,000 byte target (spill valve D3 not used, see residual risks).
* Static evidence: pipeline with 0 violations and 0 back edges, lossless audit (0 lost), rustfmt/path/region checks, patched differential (section 5), boundary corpus test 109 pass / 0 fail.
* Open: real `bun repo cargo check` of the 18 crates (lib and `--tests`), Cargo.lock refresh, the text-pinned TS scanners of plan 8.5, WP7 `sync` (owned by `oksql`).

## 1. Found state (before any change of mine)

Established by `okeng-tools/a1found.py` (live files against `okwp56-tools/pre-a1/` and the A1 overlay `🗑️generated/okwp56-a1-overlay-applied.json`), by `okwp56-tools/a1lint2.py`, by the stale-ness of the A2 logs and by a listing of the 17 crate roots.

### 1.1 A1 (the in-monolith decomposition of `ArtifactStore`) is fully applied

* The previous executor applied A1 at about 03:48 to 03:54 (`touched.txt` lists the 8 files). Six of the eight live files are byte-identical to the A1 overlay: `S/🧩️composition/🗄️durable-group/🦀️.rs`, `S/🧩️composition/📬️publication/🤝️group/🦀️.rs`, `S/🧩️composition/🗄️durable-group/🧪️tests/🔬️unit/🦀️.rs`, `S/🔗️backbone/✂️detach/🧪️tests/✂️detach/🦀️.rs`, `S/🧪️tests/🧪️supersede-replay/🦀️.rs`, `S/🧪️tests/🧪️deferred-reprojection/🦀️.rs`. Each differs from its `pre-a1` snapshot, as intended.
* The remaining two files differ from the overlay only by edits of other hands: `S/🦀️.rs` (12,222 lines; pre-A1 11,309) carries (a) the SQLITE-HOST slice's mount rewrites (`serialization`, `codec_registry`, `text_format`, `space_history`, `native_snapshot`, `sqlite_snapshot_*` now `use`/`pub use` of the crates text-io, codec-registry, io, space-history; the `composer_registry_tests` relocation), made 04:29, and (b) the previous executor's own C2 step at 04:16: `StoreMintToken` (private `_sealed` field), `impl one_item_publication::StoreMintWitness for StoreMintToken {}`, `StoreLog::mint_witness()` and the two `ArtifactStoreOneItemLiveAuthority::mint_for_store(store_log.mint_witness(), …)` call sites; `S/🧪️tests/🔬️unit/🦀️.rs` carries the SQLITE-HOST one-line caller rewrite `sqlite_codec` -> `ArtifactSqliteSnapshotOperations`. The C2 counterpart in the publication crate exists (`pub trait StoreMintWitness {}` and `mint_for_store(&dyn StoreMintWitness, …)` in `S/🎮️one-item/📬️publication/🦀️.rs`).
* Item audit (`a1found.py`, multiset of top-level items with impl headers normalised, nested inline modules to depth 3): 0 lost in seven files; in the root 1 key `('const', '_')` that I could not locate (const names of pre and live are identical, so it is a key-normalisation artefact of the SQLITE-HOST mount edits, not a lost declaration), 25 extra items (the six component structs, `StoreMintToken`, `ArtifactStoreCloseHost` and their impls). Every one of the 485 distinct method names of the pre-A1 impl blocks exists in the live tree (`pre method names missing in live: 0`), 592 method definitions live against 576 before (delegates and the component split).
* `okwp56-tools/a1lint2.py` (delegate lint against the pre-A1 method table): **203 delegates checked, 0 mismatches** on the live root.
* The previous executor's recorded oracle rows for A1 (`🗑️generated/okwp56-u-base.log`, `okwp56-u-a1b.log`): lib internal 927 against 927, test internal 2510 against 2510 (external 3281 / 8836 against 3237 / 8792, the new external names are the component paths). I did not re-run these on the current tree before my changes; the post-change differential is in section 5.

### 1.2 A2 (the carve of the 17 crates) was never applied

* `🗑️generated/okwp56-a2-b1.log` is empty (04:16); no `okwp56-out-b1.json` dump exists.
* Of the 17 target crates, 11 still have only the 8-line WP0 stub as crate root (`initialization-runtime`, `durable-group-wire`, `engine-core`, `engine-live`, `engine-stage`, `engine`, `codec-factory`, `space-member`, `config`, `space`, `test-support`) and 6 still use a monolith module file as root (`engine-owners` 668 lines, `initialization-replay` 911, `document` 204, `member-open-store` 1,001, `durable-group` 3,692, `composition` 878). None of them is in the kernel `Cargo.toml` dependency list. Nothing of A2 needs rolling back.
* `okwp56-tools/touched.txt` (the 8 A1 files) is the complete list of repository files the previous executor changed through its tools, apart from the C2 edit of the publication crate and the root.

### 1.3 World around the slice at 05:00

* The SQLITE-HOST slice (earlier executor `oksh-tools`) already carved `os-store-text-io`, `-codec-registry`, `os-io`, `-space-history`, `-owned-schema-fields`, `-owned-schema-decode` (all in the kernel dependency list, kernel manifest last written 04:29). The census row of the kernel at 04:50 was 56 files / 26,162 lines / 1,775,815 bytes, `overLimit=true`, `carvableLeaf=true`.
* `os-store-member-open` is not carved and is not in `crates56.txt`, but `os-store-engine-owners` (planned dependent) lists it as dependency; see section 3 (decisions) for how this is handled.

## 2. What changed

Tools: `okeng-tools/` (`a2x.py` driver, `fixuse.py`, `fixlit.py`, `regions.py`, `remanifest.py`, `corpus.py`, `cc.py` and `waitcc.py` for the mutex-protected launcher, `edges2.py`, `cfgtest.py`, `dslroot.py`, `pins.py`, `snap.py` with the pre-A2 snapshots in `okeng-tools/pre-a2/`, `touched.txt` for every repository file the driver wrote). The WP2/WP3 pipeline of `okwp23-tools` was extended, not forked: `rewrite.py` (re-export shim heads in paths and `use` leaves, `shim_extra` raw lines resolved, shim-module aliases dropped), `widen.py` (a `a::b` path in a struct body was read as a field `a:`; it inserted a bare `pub` into generics).

1. **Pipeline defects fixed before the batch could be correct** (each was found by a dump, none by guessing): the store root was classified as a file of `os-store-engine` (its main crate), so engine items were never lifted; now the root is a kernel-owned split source (`KBatch.build_kernel_source`: cut the moved items, rewrite the remainder in kernel mode, import the moved public names back). `ns.resolve` did not model the extern prelude, so every `::semio_framework_os_kernel::os_store::…` in a `where` clause stayed a kernel path (32 sites); the kernel aliases (`store`, `vcs`, `dsl`, `spr`, `semio_format`, `dsl_grammar`, `semio_framework_os_kernel`) now resolve in the kernel world. `sibling_imports` imported kernel shim modules from the lifted staging (back edges to engine); skipped for `mod` items. `_lower_hits` was quadratic (200 s to 18 s for the root; a dump now takes 3.5 minutes).
2. **Source edits ahead of the dump** (all in files of this slice): `bind`/`recover` of the durable group now take `&StorePartsRef` as their callers already assumed (A1 had converted the callers only); the inline `mod snapshot_input_tests` of `🏭️operation/🦀️.rs` moved to `🏭️operation/📏️birth/🧪️tests/📥️snapshot-input/🦀️.rs` (it called a member-open test helper through `super::super::tests`); the 806-line inline `pub mod test_support` of the root became the module file `🧰️test-support/🦀️.rs` (the crate root of `os-store-test-support`).
3. **Apply** (`a2x.py b1 apply`, 78 file operations, 0 concurrent-edit merges): crate roots, 20 lifted staging files (`📥️lifted/<source-slug>/🦀️.rs`: `lifted_store`, `lifted_document`, `lifted_durable_group`, `lifted_operation`, `lifted_replay`), the 18 manifests, the kernel manifest (+18 dependencies and the `durable-group-testing` forward), the store root cut to 1,345 lines. Nine test files that need kernel test doubles were relocated to the kernel root as `#[cfg(test)]` modules (rule 9.2; WP8 moves them into companions).
4. **Post-apply repairs** on the written tree: relative `include_str!` literals of lifted items re-resolved (`fixlit.py`, 19, all in test-support); region markers of 10 staging files balanced (`regions.py`; the 3 remaining imbalances exist at HEAD); 107 `use` statements with kernel-shim heads (`use snapshot_read::…`, `use ledger::…`, `use serialization::…`) resolved to the extern crate paths they re-export (`fixuse.py`, 364 leaves, 0 unresolved); manifests recomputed from the files on disk (`remanifest.py`: +66 dependency lines in 10 crates, `composition` lost an unused `engine` edge); three stray `pub ` inserted into generic arguments removed; `member-open` lost the upward `pub use history::factory::MemberOpenDeclaration` (the kernel shim has it); the `mounted` alias of the mounted-pack session is now a local re-export module; the three `MIRROR_ORACLE_*` constants and `StoreReplay::assert_mirrors_are_live` lost their `#[cfg(test)]` (a dependency is compiled without `cfg(test)`; `#[doc(hidden)] pub`); `DurableStoreVerifiedOutcomeV1` is `pub`; `durable-group` forwards its `durable-group-testing` feature to the wire crate; 10 skeleton docstrings replaced.
5. **Boundary corpus**: 18 producer rows and 7 cut rows in `🧪️tests/🧫️fixtures/📦️crate-boundaries/{🔣️,🧬️cuts}.json` (`corpus.py`).
6. **Derive path root (coordinator request)**: see section 3.

## 3. Decisions and the derive root

* `os-store-member-open` joined the batch (engine-owners depends on it; its `MemberStoreOpenRetained` goes to member-open-store, as the plan says).
* C1 (`ArtifactStoreCloseHost` in engine-owners, implemented by the composer) and C3 (`StoreParts` in engine-live) as in the plan. C2: the previous executor's `StoreMintWitness` marker trait (publication crate) with `StoreMintToken` (private field) in engine-core, minted by `StoreLog::mint_witness`; not sealed against a hostile `impl` (the plan's unit-struct variant cannot be named below engine-core), the source law `one_item_publication_source_denies_generic_or_unbounded_shortcuts` still forbids raw field fabrication.
* **Derive root.** The `dsl_derive` expansions that name the kernel are `DslArtifact` (3 paths), `diff_text`/`diff_binary` and `CompositeMutation` (29 paths); `Mutations`/`MutationLeaf` name only lower crates (`::protocol`, `::semio_framework_os_command`, value, dsl-record, ui-locale, schema-state, and `os-store-one-item` in the generated test). Scan over all carved crates (`okeng-tools/dslroot.py`): users are `os-store-space-schema` (7 files: `dsl::MutationLeaf`, `dsl::Mutations`), `os-store-durable-group-wire` (3 `DslArtifact`), `os-store-durable-group` (1), `os-store-backbone` (3 `#[dsl(base64)]`, a helper of `DslEnum`, no kernel path). Fix at the root: `DslArtifact` now emits `::semio_framework_dsl_record::{RecordSpecProducer, BorrowedRecordSpecProducer}`, the same items the kernel re-exported (`M/🗣️dsl/✨️derive/🦀️.rs`, 3 lines; default behaviour unchanged for the 2,857 external files, which already depend on dsl-record through `DslRecord`); `space-schema` names `semio_framework_os_dsl::{MutationLeaf, Mutations}` and its manifest gained `os-dsl`, `replication`, `os-command`, `schema-state`, `dsl-record` and the dev dependency `os-store-one-item`. `diff_text`, `diff_binary` and `CompositeMutation` keep their kernel paths: no carved crate uses them (they need the plan's path-root attribute only when one does).

## 4. Census rows (`bun 🗑️generated/census.ts` with `ONLY=…`, lib target, 2026-10-16 07:10; `okeng-census-2.log`)

| crate (`semio-framework-…`) | files | bytes | active lines | overLimit |
|---|---:|---:|---:|---|
| os-kernel (facade) | 5 | 116,845 | 1,624 | **false** (was 56 / 1,775,815 / 26,162, true) |
| os-store-initialization-runtime | 6 | 57,982 | 885 | false |
| os-store-durable-group-wire | 2 | 60,290 | 1,048 | false |
| os-store-member-open | 10 | 123,230 | 2,292 | false |
| os-store-engine-owners | 5 | 68,781 | 941 | false |
| os-store-engine-core | 3 | 184,239 | 2,693 | false |
| os-store-engine-live | 3 | 191,674 | 2,720 | false |
| os-store-engine-stage | 2 | 48,652 | 692 | false |
| os-store-engine | 4 | 254,589 | 3,452 | false (bytes 10.7 % over the 230,000 target, 97 % of the hard 262,144) |
| os-store-codec-factory | 2 | 22,960 | 359 | false |
| os-store-initialization-replay | 3 | 45,638 | 891 | false |
| os-store-space-member | 5 | 110,522 | 1,733 | false |
| os-store-document | 3 | 116,184 | 1,826 | false |
| os-store-member-open-store | 2 | 73,218 | 1,035 | false |
| os-store-config | 3 | 72,423 | 1,122 | false |
| os-store-durable-group | 1 | 125,629 | 2,013 | false |
| os-store-composition | 4 | 93,128 | 1,484 | false |
| os-store-space | 3 | 24,634 | 392 | false |
| os-store-test-support | 21 | 72,995 | 907 | false (19 embedded schema files) |

The kernel census row turned `overLimit=false`; its `closureCompilable=false` only because a dependency outside this slice (`oksql` sync crates or peers) is still refused by the source boundary. Test closures were not counted: the nine relocated test files now sit in the kernel root under `cfg(test)` until WP8.

## 5. Verification (commands run, real results)

| gate | tool / command | result |
|---|---|---|
| A1 delegate lint | `okwp56-tools/a1lint2.py` | 203 delegates, 0 mismatches |
| A1 item audit | `okeng-tools/a1found.py` | 0 lost in 7 files; root: 1 key `('const','_')` not locatable (const names identical), 0 pre-A1 method names missing |
| back edges, item level | `okeng-tools/edges2.py --notests` | before the fix: 3 (`demand` helper, two durable-group outcome impls naming `ArtifactStore`); fixed by assignment override and by `bind`/`recover` over `StorePartsRef` |
| pipeline | `okeng-tools/a2x.py b1 dump` / `apply` | 0 violations, 0 back edges, 832 extern paths of which 10 unproven (derive re-exports `ToValue`/`FromValue`/`DslArtifact` that the name model cannot see) |
| crate graph | `okeng-tools/graph.py` | 0 back edges against `okcrates.RANK`, no cycle (after the manifest recompute) |
| lost items | `okwp56-tools/lossless56.py` | pre 2,502 items, post 2,503; 9 header keys differ only by rewritten paths; 0 lost |
| paths, rustfmt, regions | `okeng-tools/verify_paths.py` | 348 files, 622 literals, 0 rustfmt failures, 3 region findings that exist at HEAD |
| cfg(test) across crates | `okeng-tools/cfgtest.py` | 5 items; the ones another crate names were ungated (section 2.4) |
| boundary corpus | `bun test ./🧪️tests/📦️crate-boundaries/🟦️.ts` in `💻️os` | 109 pass, 0 fail |
| patched differential, lib (`u56.py okeng-m0` baseline = pre-A2 snapshot with the 18 crates unmounted, `okeng-m2` after) | `cmp56.py` | internal 875 -> 812; no new E0412/E0422/E0423/E0425-of-local/E0603/E0616/E0624/E0432 of an internal path after the fixes; the remaining new keys name `crate::__os_store_read/space_schema/ledger/…` items that only exist behind extern globs the sandbox neutralises (the rewriter proved each extern path with the registry: 832 checked) |
| patched differential, wasm32-unknown / wasip2 | `okeng-m2` wasm, `okeng-t1` wasip2 | wasm 823 / wasip2 808 internal (baselines 877 / 772 of the first run, same classification) |
| patched differential, test | `okeng-t0` (baseline) / `okeng-t1` (after) | internal 2,229 -> 2,350; the only structurally relevant new keys are `E0422` in the sync unit test (`oksql`, `crate::__os_spr` glob artefact) |
| TOML | every written manifest parsed by `remanifest.py` through the generator, kernel manifest +18 deps | ok |
| `bun repo cargo … check` | `okeng-tools/cc.py` / `waitcc.py` under the fleet mutex | **not run to completion**: every attempt from 06:56 to 09:31 ended with `Insufficient physical compiler initial headroom` (image available 4,262,895,616 B, required 4,294,967,296 B; constant for 2.5 h, nobody else held the mutex). Earlier attempts before the mutex protocol: `okeng-cc-runtime.log` (same refusal), `okeng-cargo-1.log` (wrong `--manifest-path`), `okeng-cargo-2.log` (ENFILE) |

## 6. Foreign errors

None observed yet (no compiler output exists). Defects of other slices seen by the static tools: the `os-store-sync` unit test (`oksql`) `E0422` keys of the test differential (extern-glob artefact); the SQLITE-HOST note on `space-schema` is fixed here (section 3). A stray `S/🦀️.rs.orig` (868,716 bytes, 03:48) sits beside the store root; it is not mine and was left in place.

## 7. Hand-offs and residual risks

1. **Real compile of the 18 crates plus `space-schema` (lib, then `--tests`), and the Cargo.lock refresh, are the first thing to do when the image has 32 MB more headroom.** Command, as prepared: `python3 okeng-tools/waitcc.py lib1 okeng-tools/b2.crates` (does `update --workspace --offline`, then one batched `check -p … --lib --keep-going`). Expect compile errors in three classes the static tools cannot see: borrow splits in composer use-case methods (plan 13.1), method and field privacy beyond the `widen` pass, and trait scope.
2. **`os-store-engine` is 254,589 bytes** (target 230,000, hard 262,144). Spill valve D3 (`ingest_remote_merge`, `advance_apply_batch`, `begin_apply_batch_owned` as bundle methods in a second crate) was not used: it rewrites 792 lines of borrow-sensitive code that nothing could check.
3. **Text-pinned TS scanners (plan 8.5) are not re-pointed.** `okeng-tools/pins.py` lists the files with needles that existed in the pre-A2 root and are gone: root `📜️script.ts` of the kernel package (6), `🖥️host/…/📜️script.ts` (3), `🧩️composition/📨️emission/📦️owned/🧪️tests/🟦️.ts` (34), `🧵️canonical-edit/🧪️tests/🔬️canonical-error-progress/🟦️.ts` (28), plugin `⏪️time-travel/♻️retirement` (11), `🔬️tool-job-latest-wins` (2), replication `🌐️foreign` (2) and `🔀️transition/📝️drafts` (1), `🔍️discovery/🟦️.ts` (2), and a flow editor test (2). Several of them fail for other reasons as well (`okeng-ts-1.log`); no baseline per file was taken. The `emission` test also pins the old `ArtifactStoreOwnedDisposer` signature (`store: &ArtifactStore`), which C1 replaced by `&dyn ArtifactStoreCloseHost`.
4. **Derive root.** `diff_text`, `diff_binary` and `CompositeMutation` still emit `::semio_framework_os_kernel::…`; fine for every consumer above the kernel, a defect for any future crate below it.
5. **Kernel manifest/test closure.** The kernel test closure now holds the relocated tests (9 files) plus the root-mounted ones; WP8 companions are nobody's slice in wave 5.
6. **Mutex.** Every launcher attempt held and released the mutex (`cc.py`), none was stale.

## 8. Files

Everything written through the drivers is listed one per line in `okeng-tools/touched.txt`; plus by hand: `S/🧩️composition/🗄️durable-group/🦀️.rs`, `S/🧩️composition/🚪️open/🏭️operation/🦀️.rs` and its new test file, `S/🧰️test-support/🦀️.rs`, `S/🧬️schema/🧬️mutations/**` (7 derive paths), `S/🧬️schema/📦️packages/🦀️rust/Cargo.toml`, `S/🧩️composition/🗄️durable-group/📦️packages/🦀️rust/Cargo.toml`, `M/🗣️dsl/✨️derive/🦀️.rs` (3 lines), the two hydration files (`pub` in generics), the 10 crate roots (docstrings), the two corpus fixtures, and the tools `okwp23-tools/{rewrite,widen}.py`, `okwp56-tools/{u56,a2}.py`.
