# Result CENSUS4-PKG-15 (prefix `c4p15`): hub rust files split into cohesive children

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command (read-only `git cat-file`, `diff`, `status`, `grep`), no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools and snapshots: `F/c4p15-tools/`. Logs: `F/🗑️generated/c4p15-*` (nothing deleted). Sandboxes and the stage: scratchpad `c4p15-work/`. Baseline revision of the oracle: `b84bf526c7b` (equal to the live files at my start; the seven live files were also snapshotted to `c4p15-tools/orig/<n>.rs` with SHA256).

## 0. Verdict

**All seven files are split, installed and verified statically. Nothing was compiled** (Wave 6, the launcher is refused). 43 new files in 43 new folders; every child is at most 572 lines (limit about 1,000). The line multiset is lossless (0 removed lines; the only added lines are wiring, child docstrings and impl wrappers, plus the include path re-base, section 4). The typeck oracle reaches body type-check (canary reached) and reports 0 new rows; `lit.py` reports 0 sites; rustfmt hunks are 103 before and 103 after; no duplicate mount involves these files.

| # | Target (path tail under `🌎️hub/`) | Lines | Root after | Children | Largest child | Mode |
|---|---|---:|---:|---:|---:|---|
| 1 | `💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs` | 1,611 | 28 | 5 | 535 | include (test paths unchanged) |
| 2 | `💡️inference/🪶️sqlite/🦀️.rs` (crate `semio-hub-inference-ledger`) | 1,397 | 32 | 5 | 524 | mod, inherent `impl` members split over 3 children |
| 3 | `📇️directory/🧪️tests/🔬️unit/🦀️.rs` | 1,779 | 34 | 6 | 540 | include |
| 4 | `🔨️modules/🛡️admin/🧪️tests/🔬️dist-assets-bll2-7qi-unit/🦀️.rs` | 1,476 | 13 | 5 | 495 | include (orphan, section 7) |
| 5 | `🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs` (crate `semio-hub-trusted-catalog`) | 1,585 | 64 | 9 | 412 | mod |
| 6 | `🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs` | 2,769 | 29 | 8 | 572 | include |
| 7 | `🗿️artifact-authority/🔏️trusted-catalog/🧾️descriptor/🦀️.rs` (crate `semio-hub-trusted-catalog-descriptor`) | 1,145 | 39 | 5 | 352 | mod |

**Correction (SPLIT-HARDENING-7, 2026-10-17).** Target 4 (the orphan `🔨️modules/🛡️admin/🧪️tests/🔬️dist-assets-bll2-7qi-unit`) is no longer in the tree on purpose. Because nothing mounted or read it, the coordinator moved the root and its five children to `F/🗑️generated/coord-orphan-hub-dist-assets-test/🔬️dist-assets-bll2-7qi-unit/` (`📓️FLEET.md`, entry 00:2x). So the live package 15 has **6 roots and 38 children (44 files)**, not 7 roots and 43 children (49 files); `c4p15-installed.json` key `dist` and the rows "4" in the tables below describe the state at the time of the cut. The region markers the audit found missing are the 6 pairs (12 marker lines) of that moved folder; the other six targets carry no region marker at all (HEAD snapshots 1, 2, 3, 5, 6, 7 have 0), so no marker was lost. Do not restore the orphan. The taxonomy member name `🔬️dist-assets-bll2-7qi-unit` in `🔣️taxonomy.json` is now unused.

Why two modes: production roots (2, 5, 7) become real `#[path]` modules (`mod x; pub use x::*;`) so that each child can later become a crate; their root keeps only the doc, the `use` block and the mounts, and every public path is re-exported unchanged. Test roots (1, 3, 4, 6) use `include!("<child>/🦀️.rs");` fragments, so the module identity, every test path (`unit::name`, `unit::quick::name`) and every exact-law selector stay identical, with no visibility widening. The fragment docstring is a `//` line, because `//!` is illegal inside `include!` and `///` would attach to the first item.

## 1. Found state (recovery after the session restart)

The restart came after install and during verification. State found and checked before continuing: `c4p15-tools/` complete (`c4p15.py`, `plans15.py`, `c4p15lib.py`, `c4p15check.py`, `c4p15names.py`, `c4p15fmt.py`, `c4p15-census.ts`, `c4p15-readers.ts`, `c4p15-nativeplan.ts`, `families/c4p15.json`, `orig/` with SHA256); `F/🗑️generated/c4p15-*` 115 files; the old scratchpad stage directory still present. `c4p15.py stage` then `verify` showed **all 7 targets and 43 children byte-equal to the plan** (`live files equal the staged plan: YES`, 49 files), `c4p15check.py` `CHECKS OK`, names `OK`, rustfmt 103 = 103. No half-applied batch existed, so no rollback was needed. One edit of this slice was still missing when the restart hit and was done afterwards: the descriptor-digest fixture receivers (section 5, row 9) and the `include!` follow in `hub_module_tree` (row 8).

## 2. Layout (new folders, `<emoji>️<kebab>` with U+FE0F; all RGI, all existing repo folder prefixes, unique among siblings, equal to the docstring emoji)

Production, mod mode:
- `🔏️trusted-catalog/`: `📮️publisher` (71), `🧊️guest-residency` (320), `📈️load-progress` (53), `🗜️guest-codec` (226), `🔗️native-codec` (93), `📚️verified-catalog` (235), `🗃️verification-cache` (89), `🏗️loader` (412), `🎭️fixture-builders` (87, mounted under `#[cfg(any(test, feature = "integration-fixtures"))]` so a normal build has no empty module).
- `🧾️descriptor/`: `🔌️native-provider` (95), `💾️retained-assets` (352), `🎯️open-targets` (228), `📋️bundle-validation` (347), `🔍️descriptor-validation` (119).
- `💡️inference/🪶️sqlite/`: `🧱️ledger-storage` (145; named so the module does not share a name with the glob-imported `fn storage`), `🧾️ledger-types` (87), `🎬️job-lifecycle` (524, inherent impl members), `✅️approvals` (274, plus the reconciliation vocabulary), `🗺️gis-map-undo` (374). The single 1,082-line `impl InferenceJobLedgerV1` became three `impl` blocks, one per child; all methods were already `pub`.

Tests, include mode (under the `🧪️tests/🔬️unit/` of each root):
- runtime: `🧰️law-fixtures` (344), `🤝️handoff-and-owner-laws` (190), `📮️committed-publication-laws` (314), `🔚️terminal-and-comparison-laws` (215), `⚡️quick-and-binding-laws` (535).
- directory: `🪪️identity-laws` (115), `🧰️artifact-fixtures` (364), `📒️admin-and-event-log-laws` (190), `🧺️artifact-cas-laws` (540), `🏘️space-and-membership-laws` (166), `🎟️invite-and-index-laws` (388).
- trusted-catalog: `🧊️guest-residency-and-codec-laws` (451), `📮️publication-laws` (246), `🎭️fixture-harness` (572), `🧭️provider-selection-laws` (177), `🛡️loader-trust-laws` (458), `🧪️owned-guest-laws` (277), `🎯️open-target-laws` (341), `📦️plugin-module-laws` (242).
- orphan: `🧰️shard-harness` (101), `🚀️pump-and-spawn-laws` (260), `⏸️suspend-resume-laws` (255), `📨️frame-budget-and-bridge-laws` (367), `🚦️lane-priority-and-replay-laws` (495). Region markers stay whole inside one child (6 start, 6 end markers before and after).

Widening (mod mode only, `pub(super)`, name-driven so an over-approximation): tc 27 sites (guest-codec 18, native-codec 4, verified-catalog 2, three single sites), desc 1, ledger 10 (nine helper fns in `🧱️ledger-storage` and the `connection` field). Longest new path 133 bytes (limit 240). Taxonomy: `c4ptaxcheck.ts` resolves all 43 folders (`crate-module-children` or `members-of-tests`), 0 unresolved, 0 ambiguous.

## 3. Data and tables

Scan result (json!/array/const spans and long lines over all seven files): no static table. The only large literals are interpolated builders: `ledger_schema` (102 lines of SQL with `format!` bounds) and `descriptor_json` (106 lines with role and schema expressions); moving them to run-time files would change semantics (production code must not read the source tree), so **no data was moved and no byte-identity proof is due**. Four relative `include_str!` fixtures per file stay as they are (re-based, section 4).

## 4. Verification actually run

| Check | Command or method | Result |
|---|---|---|
| Lossless line multiset | `c4p15.py stage` (stripped non-blank lines of the snapshot versus all staged files, with `pub(super) ` and the include re-base reversed) | removed 0 in all seven; added only wiring (`#[path]`, `mod`, `use x::*`, `include!`), child docstrings, `use super::*;` and the two `impl InferenceJobLedgerV1 {` wrappers. Exactness: `c4p15.py verify` rebuilds every plan and compares it with the live files byte for byte: 49 of 49 equal. |
| Typeck oracle (own prefix `c4p15`, family `c4p15-tools/families/c4p15.json`: trusted-catalog, -descriptor, inference-ledger and the three laws crates) | `cd F/pos-tools && POS_PREFIX=c4p15 python3 pos.py run ../c4p15-tools/families/c4p15.json --cfgs lib,test --label <a0\|b1\|b2>`; logs `c4p15-pos-a0/b1/b2.log` | Baseline a0 (before the first edit): lib 1,875 and test 6,170 errors, canary E0308 E0616 E0624 reached. After install b1 and after the final names b2: **canary reached, NEW 0 in lib; in test NEW 2 / GONE 2, both stub-flagged re-keyings of pre-existing errors** (the missing `📇️document-index-v1/🔣️.json` include and an E0271 stub row, only their line and file key moved). privacy 0, trait-scope 0, resolve 0, type 0, other 0. |
| Seeded sensitivity | `--mutate` on the sandbox copy only, label `m2` (`c4p15-pos-m2.log`) | caught: E0616 `field component of GuestArtifactComponent is private`, E0624 `method verified is private` (verified-catalog) and E0423 x136 for the private `fn storage`. An earlier mutation set (`m1`) touched only same-module uses and was correctly silent; it is reported because it showed that uses must cross a child to be seen. |
| E0451 | `F/rpv-tools/lit.py c4p15.json` (`c4p15-lit-live.log`) | **0 candidate sites** (54 brace structs, 56 definitions, 51 files in the module tree). Control `LIT_SELFTEST=1`: reports exactly the 2 probe literals. |
| Fragment path semantics | micro test with `rustc +nightly-2026-07-20 --emit=metadata --out-dir <scratch>` | `include_str!` inside an `include!`d fragment resolves relative to the fragment (control with equal bytes fails), so every relative `include_str!` got one more `../` (tc unit 8, directory unit 4, runtime unit 3, dist 2, others). The oracle shows the re-based path resolving to the same file as before. |
| rustfmt | `rustfmt +nightly-2026-07-20 --check --config-path <repo> --config skip_children=true` (`c4p15fmt.py`) | hunks 103 (snapshots) versus 103 (live roots and children); parse of all 49 live files: 0 failures. One extra hunk (double blank line in a joined range) was found and fixed. |
| Paths and reachability | `c4p15check.py` | every child reachable exactly once from its crate root (`closure` counts equal files), 0 missing mount targets, 0 duplicate mounts inside any crate, longest path 133 bytes, every region pair whole, test functions identical (runtime 8, directory 30, trusted-catalog 48, orphan 41, multiset equal). Docstring emoji unique inside each crate closure (one clash, `🗂️` of the root against `🗂️verified-catalog`, was found and fixed to `📚️`). |
| Duplicate mounts, workspace wide | `bun F/dm2-tools/modowners.ts` (`c4p15-modowners.tsv`) | 15 files owned by more than one package, all in `🔌️plugin/🧪️testing/🖥️test-app-mutations` and its tests; **0 involve these files**. |
| Census inside the gate | `c4p15-census.ts` (the `dm2-tools/closure.ts` counter, `--base` reads the snapshots and hides the new files), `c4p15-census.log` | see the table below; all far under 32 files / 262,144 bytes / 4,000 active lines. |
| Reader equivalence | `c4p15-readers.ts` (module-tree text of the new readers versus the snapshot text) | `readTrustedCatalogSource` lost 0, gained 0; ledger tree, trusted-catalog tree, descriptor tree: lost 0, gained 0. Test trees differ only by the re-based `include_str!` lines. |
| Native law plan | `c4p15-nativeplan.ts` (`assertSocketGrantNativeLawSources`) | passes, 19 stages. `named inputs == fixture inputs` (27 entries) checked in python. |
| Descriptor-digest receivers | `bun test ./🟦️.ts` in `📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🧪️tests` | the receiver grammar test (tree-sitter) passes with the four added files; 2 tests fail on a missing `📇️directory/🧬️schema/🟦️.ts`, which is foreign and unrelated. |
| `[DEBUG]` | grep over the new children | none added. |

Census (active lines / bytes / files; lib closure unless named; "after" counts cfg-gated fixtures only where the crate cfg enables them):

| Crate | Before | After |
|---|---|---|
| `semio-hub-trusted-catalog` lib | 1 / 107,965 / 1,492 | 9 / 103,643 / 1,545 |
| same, `--tests` cfg | 1 / 107,965 / 1,586 | 10 / 110,545 / 1,659 |
| `semio-hub-trusted-catalog-descriptor` | 1 / 67,217 / 1,146 | 6 / 68,543 / 1,186 |
| `semio-hub-inference-ledger` lib | 1 / 83,297 / 1,389 | 6 / 84,536 / 1,432 |
| same, tests | 5 / 129,886 / 1,783 | 10 / 131,125 / 1,826 |
| `semio-hub-trusted-catalog-laws` tests | 10 / 211,822 / 2,769 | 18 / 213,261 / 2,801 |
| `semio-hub-inference-laws` tests | 5 / 126,430 / 1,617 | 10 / 127,293 / 1,637 |
| `semio-hub-directory-laws` tests (missing JSON tolerated as empty in both) | 12 / 208,570 / 2,629 | 18 / 209,564 / 2,653 |

## 5. Source-text readers re-pointed (each a one-line change; all callers re-read the whole module tree now)

| # | File | Change |
|---|---|---|
| 1 | `🌎️hub/🧪️testing/📖️source-readers/🟦️.ts` | `readTrustedCatalogSource` reads the module tree of the trusted-catalog, contract and descriptor roots (`readModuleTreeSource`, already imported); feeds relay-checks, trusted-stdio-gis-bundle-check, execution-target-lease-checks and open-plan-checks |
| 2 | `🌎️hub/🧪️tests/🌍️gis-map-fixtures/🟦️.ts` | three reads of the ledger root and one of the trusted-catalog root use `readModuleTreeSource` |
| 3 | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🚀️local-hub/🟦️.ts` | import added; the catalog loader text is read through `readModuleTreeSource` (`validate_bundle` now lives in `📋️bundle-validation`) |
| 4 | `🌎️hub/📇️directory/🔐️authorization/🔌️socket-grant/🧪️tests/📋️native-law-plan/🟦️.ts` | stage `typed-capabilities` source is the child `🪪️identity-laws/🦀️.rs` (the law must have one declaration in its file) |
| 5 | `🌎️hub/🧪️tests/🧫️fixtures/🧱️socket-grant-command-source/🔣️.json` | same source path; input glob `🔬️unit/**/🦀️.rs` |
| 6 | `🌎️hub/📦️packages/🦀️rust/📋️project.json` | named input `🌎️hub/📇️directory/🧪️tests/🔬️unit/**/🦀️.rs` (equal to row 5, checked) |
| 7 | `🧰️framework/…/📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🧪️tests/🧫️fixtures/🔣️.json` | `rustReceivers` gained the four runtime children that now hold the `descriptor_digest` receivers (`⚡️quick-and-binding-laws`, `📮️committed-publication-laws`, `🔚️terminal-and-comparison-laws`, `🧰️law-fixtures`); the roots stay listed and still parse |
| 8 | `🌎️hub/🧪️tests/🗂️artifact-root/🦀️.rs` (`hub_module_tree`) | also follows `include!("…");` lines, so a Rust source-count law over a split file still sees the fragments; no current caller reads these seven files through it. rustfmt parse OK, 0 hunks |

Not needed (checked by grep over the repo): `hub_source(…)` and `include_str!` readers of these seven files (none), `moduleRustSource` walkers (they read every `.rs` below a directory and see the children), `trusted-catalog-fixtures` (`moduleRustSource(root)`).

## 6. Real-check commands for the compile queue

Run one launcher call at a time under the fleet compile mutex. Prefix: `cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 --`

1. Lib: `check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-trusted-catalog-descriptor -p semio-hub-trusted-catalog -p semio-hub-inference-ledger --lib --keep-going --message-format=short`, then with `--features semio-hub-trusted-catalog/integration-fixtures,semio-hub-trusted-catalog/testkit` (exercises `🎭️fixture-builders` and the `Vec<u8>` footprint impl).
2. Dependents: `-p semio-hub-artifact-authority -p semio-hub-native-openable -p semio-hub-inference-runtime -p semio-hub-inference-committer -p semio-hub-local-bootstrap -p semio-hub-lag-rebootstrap -p semio-hub` with `--lib --keep-going`.
3. Tests: `check … -p semio-hub-trusted-catalog-laws -p semio-hub-inference-laws -p semio-hub-directory-laws -p semio-hub-inference-ledger --tests --keep-going` (the ledger crate for its own `🧪️tests/🔬️unit`), and `--all-features` for the trusted-catalog laws.
4. Then `test -p semio-hub-trusted-catalog-laws --lib` and the same for the other two laws crates; names must equal the pre-split names (`unit::<name>`, `unit::quick::<name>`, `unit::long::<name>`).
Expected review points: `unused_imports` of root `use` lines that only the children use are counted as used through the glob, but `unused_qualifications` and `private_interfaces` warnings may move; the `pub(super)` fields of `GuestArtifactComponent` and `VerifiedNativeArtifactCodec` (E0451 lit scan is clean, rustc will confirm).

## 7. Foreign findings, hand-offs, limits

- **Orphan file (target 4).** `🔨️modules/🛡️admin/🧪️tests/🔬️dist-assets-bll2-7qi-unit/🦀️.rs` is mounted by nothing, read by nothing, and is an older copy of the plugin-host shard unit tests (it tests `ShardLoop`; the live version is `🧰️framework/…/🧵️shard/🧪️tests/🔬️unit/🦀️.rs`, 1,660 lines, diverged). It was split as asked (and cannot be oracle-checked because it is unmounted; evidence is losslessness, rustfmt parse and region pairing). Recommendation for the owner: delete it or re-home it; not done here.
- **Pre-existing missing input:** `🧰️framework/…/📇️directory/🧬️schema/📇️document-index-v1/🔣️.json` (an `include_str!` in the directory unit tests; also `🧬️schema/🟦️.ts` for the descriptor-digest test) does not exist, identically before and after; it breaks the census of `semio-hub-directory-laws` without a tolerance switch.
- `bun test` crashed (Bun segfault, 52 s) on `🧪️tests/🧱️socket-grant-command-source/🟦️.ts`; the checks it makes were replayed by hand (rows 4 to 6 of section 4 and 5).
- Peers: `🌎️hub/🧪️tests/🏛️trusted-catalog-fixtures/🟦️.ts` is staged by someone else; `RESULT-HUB-SCHEMA-CATALOG.md` names none of these files. The widening is name-driven; a private item that only a macro-generated or cfg-gated use needs would be seen by rustc, not by the oracle (stub layer).
- Not verified: no cargo/rustc build of the real crates, no run of any Rust law, no vitest/bun gate beyond the ones listed.

## 8. Files

Created (43 children, `🦀️.rs` each) in the folders of section 2. Rewritten roots (7): the seven targets. Edited readers and fixtures: section 5 (8 files). Tools and snapshots: `F/c4p15-tools/` (`c4p15.py` stage/install/sync/reinstall/verify, `plans15.py`, `c4p15lib.py`, `c4p15check.py`, `c4p15names.py`, `c4p15fmt.py`, `c4p15-census.ts`, `c4p15-readers.ts`, `c4p15-nativeplan.ts`, `families/c4p15.json`, `newfiles.json`, `orig/` with SHA256). Rollback: copy `c4p15-tools/orig/<n>.rs` back over the seven targets (n = 1..7 in the table order of `orig/SHA256`) and delete the 43 new folders listed in `🗑️generated/c4p15-installed.json`.
