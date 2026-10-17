# Result CENSUS4-PKG-38 (prefix `c4p38`): hub rust files split into cohesive children

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command (read-only `git diff`, `ls-files`, `grep`), no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`, `H` = `🌎️hub`. Tools and snapshots: `F/c4p38-tools/` (snapshots `orig/<n>.rs` with `orig/SHA256`, rows `hash n.rs repo-path`). Logs: `F/🗑️generated/c4p38-*` (nothing deleted). Stable stage and sandbox path: `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/c4p38-work/` (`stage/<key>/`, `scoped/<group>/{orig,new}`, `orc/`; the `pos.py` sandbox for prefix `c4p38` is the same directory). The install journal is `F/🗑️generated/c4p38-installed.json`.

## 0. Verdict

**DONE (static evidence; real compile not admitted).** All seven files are split and installed: 7 roots rewritten, 42 children created (27 `#[path]` module children under the four production roots, 15 `include!` test fragments under the three unit roots), every child at most 735 lines. Lossless (0 removed lines, 164 added wiring lines), rustfmt hunks 69 = 69, region markers whole per child (counts equal), test function names identical (hostile 20, session 41, open-plan 23), the scoped patched differential identical on all seven groups and every configuration (one macro-stand-in warning explained in section 5), the full-crate typeck oracle identical (`POSDIFF IDENTICAL`, lib 2,884 = 2,884, test 6,125 = 6,125, canary reached) with seeded controls caught, E0451 scan 0, definition scan 0 (control 212), census inside the gate, taxonomy resolved (42 of 42), duplicate-mount scan clean for these files. Readers re-pointed in five repo files (section 4). No `COMPILE ADMITTED` line existed in `📓️FLEET.md` at 09:05 and the mutex folder is held by a peer, so the real `bun repo cargo` check is queued (command in section 7).

## 1. Slice (before → after lines, root and children)

`wc -l` of the live files. Children are `<emoji>️<kebab>/🦀️.rs` next to the root (test fragments under the unit folder).

| n | File (relative to `H`) | Before | Root after | Children (lines) | Mode |
|---|---|---:|---:|---|---|
| 1 | `🗿️artifact-authority/🧱️chunk-cas/🦀️.rs` | 1,645 | 54 | 🧬️object-model 185, 🚪️storage-port 57, 🧮️manifest-codec 340, 🧠️memory-storage 90, 📁️filesystem-storage 350, 🗄️sqlite-storage 202, 🐘️postgres-storage 169, 🕸️neo4j-storage 117, 🔀️store-selector 64, 🫙️blob-store 86 | mod |
| 2 | `🚀️local-bootstrap/🦀️.rs` | 1,082 | 64 | 🧾️wire-frames 242, 🔁️exchange-window 59, 🚰️transport-state 97, 🛰️transport-session 306, 🔌️transport-port 90, 🎫️issuance-service 137, 📨️frame-io 135 | mod |
| 3 | `🧩️compositions/🪐️space/⚙️engine/🪐️space/🦀️.rs` | 1,084 | 138 | 🧰️document-helpers 132, 🕹️space-command 57, 🏠️space-app 541, 📜️space-manifest 244 | mod |
| 4 | `🧪️tests/🔬️bin-unit/🎲️hostile-input/🔬️unit/🦀️.rs` | 1,369 | 33 | 🧰️law-harness 186, ⚡️quick-hostile-laws 735, 🐢️long-hostile-laws 424 | include |
| 5 | `🧪️tests/🔬️bin-unit/🔐️session/🔬️unit/🦀️.rs` | 1,413 | 56 | 🔑️credential-sign-in-laws 292, 🤖️agent-delegation-laws 503, 🛡️production-posture-laws 256, 📝️observability-laws 165, 🗄️instance-store-laws 157 | include |
| 6 | `🧪️tests/🔬️bin-unit/🧭️open-plan/🔬️unit/🦀️.rs` | 1,841 | 61 | 🧰️plan-fixtures 77, 🎟️plan-ledger-and-exchange-laws 206, 🎯️execution-target-laws 285, 🛣️plan-route-laws 422, 🔁️supersession-laws 390, 🔌️socket-grant-laws 207, 🗂️scoped-directory-socket-laws 215 | include |
| 7 | `🧪️tests/🔬️bin-unit/🧰️testkit/🦀️.rs` | 1,235 | 73 | 🎁️stdio-bundle 181, ⏱️law-guards 106, 🏗️test-states 182, 🌱️document-seeding 253, 🌐️servers-and-sockets 257, 🪪️sessions-and-open-plans 225 | mod |

Totals: 9,669 → 479 lines in the seven roots, 42 children (10 + 7 + 4 + 3 + 5 + 7 + 6 = 42), largest child 735 lines, longest new path 116 UTF-8 bytes (limit 240).

Why two modes (as in `RESULT-CENSUS4-PKG-15.md`): production roots (1, 2, 3, 7) become real `#[path]` modules (`#[path = "x/🦀️.rs"] mod x; <vis> use x::*;`, children `use super::*;`) so each child can later become a crate; the public paths are unchanged because the root re-exports the children's items. Test roots (4, 5, 6) use `include!("<child>/🦀️.rs");` fragments, so the module identity and every test path (`tests::name`, `tests::quick::name`, `tests::long::name`) stay identical. The fragment docstring is a `//` line (a `//!` is illegal inside `include!`).

Root contents kept on purpose (not moved): the doc, every `use` line (including `use super::file_fence::{try_acquire, FileFence};` of chunk-cas, hoisted to the top of the root because a `super::` path means something else inside a child), the pub consts and the cfg'd type alias of local-bootstrap, the `#![allow(async_fn_in_trait)]` and `extern crate` aliases of the space root, the `#[path]` mounts of `🧪️tests`, `📚️examples`, `count-lease`, `hostile-generative` (a `#[path]` inside an `include!`d fragment would resolve against the including file) and the inline `pub mod modes` / `pub mod panels` blocks.

## 2. Found state and baselines (before the first edit)

- Fresh slice: no `c4p38-*` file existed. At my start the seven live files had exactly the census line counts. File 3 is index-modified (a peer edit of 6 lines, `git diff HEAD`), so the snapshots are the live bytes, not `HEAD`. SHA256 of the snapshots: 1.rs `183d9228…`, 2.rs `7bf6bb35…`, 3.rs `64f98472…`, 4.rs `e5a9338e…`, 5.rs `88cc6fc0…`, 6.rs `00929cd7…`, 7.rs `422d2c11…` (full hashes in `c4p38-tools/orig/SHA256`).
- Peers in the same trees (left alone): the CENSUS4-PKG-15 split is installed uncommitted; `HEAD` is `b84bf526c7b` and does not contain the pre-edit tree, so the typeck oracle was run absolute on the live tree before (`a0`) and after (`b2`) and the two runs compared with `posdiff38.py`.
- Typeck oracle baseline `a0` (family `c4p38-tools/families/c4p38.json`: artifact-authority, local-bootstrap, space-app, route-testkit, hostile-input-laws, session-route-laws, open-plan-laws; `POS_PREFIX=c4p38 python3 pos-tools/pos.py run … --cfgs lib,test --label a0`): lib 2,884 errors, test 6,125 errors, canary (E0308 E0616 E0624) reached in both. Log `c4p38-pos-a0.log`.
- Census baseline (own `c4p38-census.ts` with the canonical `readRustSourceClosureV1`; files / lines / bytes; lib and `--tests`): artifact-authority 4 / 2,545 / 148,691 and 9 / 3,371 / 210,287; local-bootstrap 1 / 1,055 / 45,352 and 4 / 1,401 / 75,054; space-app 11 / 2,092 / 157,186 and 19 / 2,910 / 223,195; route-testkit 2 / 1,236 / 78,242 (both); hostile-input-laws tests 4 / 1,490 / 161,931; session-route-laws tests 5 / 1,459 / 113,272; open-plan-laws tests 4 / 1,847 / 164,245.

## 3. Data and tables

Scan of the seven files for static tables, large literals and embedded assets: none. The only large literals are code (the `app_commands!` invocation of the space root, a macro call listing 40 command rows, which moved whole into `🕹️space-command`; the stdio bundle builder; format strings). Nothing moved to run-time files, so no byte-identity proof is due. Relative `include_str!` fixtures (session 1, open-plan 2, testkit 1) stay as they are, re-based by one `../` (section 6).

## 4. Source-text readers and fixtures re-pointed

Found by `git grep -F` over `*.ts`, `*.tsx`, `*.json`, `*.rs`, `*.toml` (the hit list is `F/🗑️generated/c4p38-readers-grep.txt`); each is a one-line to few-line change:

| # | File | Change |
|---|---|---|
| 1 | `H/🧪️testing/📖️source-readers/🟦️.ts` (`readHubBinTestSource`) | each of the seven unit and testkit roots is read through `readModuleTreeSource` instead of `readFileSync`, so the text of the new children is included (caller: `🧪️tests/🌐️relay-checks`) |
| 2 | `H/📇️directory/🔐️authorization/🔌️socket-grant/🧪️tests/📋️native-law-plan/🟦️.ts` | the table `hubBinaryLaws` has a fourth column, the child that now declares the law (13 rows; the two directory-route rows keep an empty child); `source` becomes `…/🔬️unit/<child>/🦀️.rs` |
| 3 | `H/🧪️tests/🧫️fixtures/🧱️socket-grant-command-source/🔣️.json` | the same 13 `nativeStages[].source` values (open-plan 9: `🗂️scoped-directory-socket-laws` 4, `🔌️socket-grant-laws` 5; hostile-input 4: `⚡️quick-hostile-laws` 3, `🐢️long-hostile-laws` 1) |
| 4 | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🧪️tests/🧫️fixtures/🔣️.json` | `rustReceivers` gained the 6 children that now hold `descriptor_digest` receivers (open-plan `🧰️plan-fixtures`, `🎟️plan-ledger-and-exchange-laws`, `🎯️execution-target-laws`, `🛣️plan-route-laws`; testkit `🌱️document-seeding`, `🪪️sessions-and-open-plans`); the roots stay listed and still parse |
| 5 | `H/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts` (`interactiveJobCatalogOracle`) | the studio surface got `children` (the four space children) and the source text is root plus children; the other two surfaces are untouched |

Not needed, checked: the compute-ownership consumer of the space file (`🏛️ownership/🧮️compute`, fixture, schema, `project.json`) reads only the root for `semio_framework_2d::compute::EngineHandles`, which stays in the root `use` block; `project.json` inputs `🧪️tests/🔬️bin-unit/**/🦀️.rs` and the named input of the socket-grant fixture already glob the children; `📏️lib-wiring-ratchet` pins no file of this slice; README mention of `🚀️local-bootstrap/🦀️.rs` stays true (the module tree).

## 5. Verification actually run (commands and real results)

| Check | Command or method | Result |
|---|---|---|
| Lossless line multiset (live, independent of the stager) | `python3 c4p38-tools/c4p38lossless.py` (`c4p38-lossless.log`) | `LOSSLESS OK removed 0 added 164 wiring 164`; per key removed 0 and added only wiring (`#[path]`, `mod`, `use <child>::*`, `include!`, child docstrings, `use super::*;`); the two declared rewrites (`pub(super) ` prefix, one extra `../` on relative include paths) are reversed before the comparison |
| Plan equals live | `python3 c4p38-tools/c4p38.py verify` | all 49 files (7 roots + 42 children) byte-equal to the rebuilt plan; `reinstall` was used for three renames and the import hoist (journal kept) |
| Live-tree structure | `python3 c4p38-tools/c4p38check.py` (`c4p38-check-live.log`) | `CHECKS OK`: no missing mount, no duplicate mount in any crate closure, every child reached exactly once, docstring emoji unique in each crate closure (two clashes with existing `🔌️adapters` and `🎯️active-instance` were found and renamed `🚪️storage-port`, `🕹️space-command`), regions equal (space 9/9, session 6/6, open-plan 1/1, testkit 1/1), no region pair split, test fns equal, max child 735 lines, max path 116 bytes |
| Names, RGI, taxonomy | `python3 c4p38-tools/c4p38names.py`; `bun c4p23-tools/c4ptaxcheck.ts newdirs.txt` (`c4p38-taxcheck.log`) | `NAMES OK 42 dirs` (RGI emoji, already a folder prefix in the repo, no sibling clash, equals the docstring emoji; one clash with the sibling `📦️packages` renamed `🎁️stdio-bundle`); `checked=42 unresolved=0` (21 `crate-module-children`, 21 `members-of-tests`) |
| rustfmt | `python3 c4p38-tools/c4p38fmt.py` (`rustfmt +nightly-2026-07-20 --check` with the repo config, `skip_children`; `c4p38-fmt.log`) | hunks 69 (snapshots) = 69 (live roots and children); parse of all 49 files: 0 failures. One extra hunk (hoisted `use super::file_fence` placed after the std group) was found and fixed by hoisting it to the top |
| Scoped patched differential, stage and live | `python3 c4p38-tools/scoped38.py stage\|live <group>` (the `RESOLUTION-ORACLE.md` method: original restored in a sandbox, `extern crate` removed, external globs neutralised, inner attributes moved; `rustc +nightly-2026-07-20 --emit=metadata -o /dev/null`; `c4p38-scoped-live.log`) | live, all groups: `cas` lib 86 = 86, test 136 = 136, libfeat 88 = 88, testfeat 136 = 136; `boot` 62 = 62 / 102 = 102; `hostile` 218 = 218 / 308 = 308; `session` 156 = 156 (both configs); `openplan` 250 = 250 (both); `testkit` 190 = 190 (both); `space` lib 480 = 480, test 698 = 698, libfeat 598 = 598, wasm32-wasip2 480 = 480. Error multisets identical everywhere; warnings identical except one `unused import: space_command::*` in the space group: the sandbox neutralises the `app_commands` macro import, so the generated enum does not exist there (the real macro is `#[macro_export]` and reaches the child through `use super::*`; its expansion references only items named in the invocation). Two message-text normalisations: the `: not found in this scope` label suffix is dropped (an unrelated similar-name suggestion for the nonexistent constants `SPACE_CONFIG_MAXIMUM_BYTES` and `SPACE_CONFIG_TEXT_BYTES`, which the stale space unit tests use, appears only when the private `SPACE_CONFIG_RETAINED_BYTES` is visible in the root) |
| Differential sensitivity | removing the `pub(super)` of `space_digest` in the sandbox | `+new 18x E0425 cannot find function space_digest` (so the differential sees widening) |
| Full-crate typeck oracle | `POS_PREFIX=c4p38 C4P23_OVERLAY=<live copy> python3 c4p23-tools/posrun.py run families/c4p38.json --cfgs lib,test --label b2 --no-base`, then `python3 c4p38-tools/posdiff38.py a0 b2 lib test` | `POSDIFF IDENTICAL`: lib 2,884 = 2,884, test 6,125 = 6,125, canary E0308 E0616 E0624 reached in both. The plain run without the glob emulation (`b1`, installed tree) differed by 8 and 12 rows that were all stub-layer trait renames (`Tr`) of items imported through the parent glob, the case the `posrun.py` emulation exists for (it injects the parent's `use` items into each child's sandbox view; 6 emulation rows inside the injected blocks are not counted) |
| Oracle sensitivity (seeded, sandbox only) | `--mutate` label `m1` on the same run | `+m1 18x E0425 space_digest` and `+m1 2x E0616 field channel_key of InheritedLocalBootstrapTransport is private`; the first attempt with underscore crate names silently skipped both mutations and was repeated with the package names |
| E0451 literal scan | `POS_PREFIX=c4p38 python3 rpv-tools/lit.py families/c4p38.json live` (`c4p38-lit-live.log`) and the probe control (`c4p38-lit-selftest.log`) | 0 candidate sites (77 brace structs, 77 files); the control reports exactly the 2 probe literals |
| Definition scan | `python3 c4p38-tools/c4p38defscan.py` (every private fn/struct/enum/const/static/type/trait, struct field and inherent method of a mod-mode child whose name another file of the crate mentions) | 0 hits; control `C4P38_STRIP=1` (widening prefixes removed in memory) gives 212 hits |
| Census inside the gate | `bun c4p38-census.ts --summary [--base] [--tests] <crate dir>` (`c4p38-census.log`, `c4p38-census-base.log`) | see the census table below; all far below 32 files, 4,000 active lines, 262,144 bytes |
| Reader equivalence | `bun c4p38-tools/c4p38-readers.ts` (`c4p38-readers.log`) | `readHubBinTestSource` old versus new: 9,887 / 9,947 non-blank lines, lost 4 and gained 4, all four being the re-based `include_str!` paths; chunk-cas and local-bootstrap trees lost 0 gained 0; space interactive-job text lost 0 gained 0; hostile tree lost 0; session and open-plan trees differ only by their re-based `include_str!` lines |
| Space interactive-job oracle | by hand (the script needs the launcher envelope): the 42 needles of `interactiveJobCatalogOracle` (factory type, controller id, 40 migrated ids) over old versus new text; the 40 negative `BatchOnlyPendingRewrite` patterns | 42 of 42 present in the new text (same as old), negative hits 0 = 0 |
| Socket-grant native plan | `bun test ./🟦️.ts` in `H/🧪️tests/🧱️socket-grant-command-source` (`c4p38-bun-socket-grant.log`) | 7 pass, 0 fail, including "native law planning is source-bound" (each of the 13 moved declarations has exactly one current declaration in its child) |
| Descriptor-digest receivers | `bun test ./🟦️.ts` in `…/🔐️descriptor-digest/🧪️tests` (`c4p38-bun-descriptor-digest.log`) | 6 pass, 2 fail; the two failures are the pre-existing `ENOENT … 📇️directory/🧬️schema/🟦️.ts` (foreign, identical to the PKG-15 finding); "native hash receiving imports … grammar accepted" passes with the 6 added files |
| Duplicate mounts, workspace wide | `bun dm2-tools/modowners.ts <generated dir> c4p38-modowners.tsv` | 19 files owned by more than one package (all in `🔌️plugin/🧪️testing`); 0 involve these files |
| `[DEBUG]` | grep over the new children | none added |

Census (files / active lines / bytes, canonical counter; before reads the snapshots and hides the new children):

| Crate | Before | After |
|---|---|---|
| `semio-hub-artifact-authority` lib | 4 / 2,545 / 148,691 | 14 / 2,624 / 150,945 |
| same, `--tests` | 9 / 3,371 / 210,287 | 19 / 3,450 / 212,541 |
| `semio-hub-local-bootstrap` lib | 1 / 1,055 / 45,352 | 8 / 1,110 / 49,005 |
| same, `--tests` | 4 / 1,401 / 75,054 | 11 / 1,456 / 78,707 |
| `semio-hub-space-app` lib | 11 / 2,092 / 157,186 | 15 / 2,124 / 158,133 |
| same, `--tests` | 19 / 2,910 / 223,195 | 23 / 2,942 / 224,142 |
| `semio-hub-route-testkit` lib and `--tests` | 2 / 1,236 / 78,242 | 8 / 1,284 / 79,623 |
| `semio-hub-hostile-input-laws` `--tests` | 4 / 1,490 / 161,931 | 7 / 1,502 / 162,441 |
| `semio-hub-session-route-laws` `--tests` | 5 / 1,459 / 113,272 | 10 / 1,480 / 114,167 |
| `semio-hub-open-plan-laws` `--tests` | 4 / 1,847 / 164,245 | 11 / 1,876 / 165,441 |

Retries and tool faults: no ENFILE or EMFILE occurred (`ulimit -n 65536` set for every oracle run). The first `--base` census run printed `Bun v1.3.14` error traces; the cause was not a segfault but my own `orig/SHA256` format (two columns, the census tool needs the repo path as the third); I rewrote the file in the three-column form (same hashes) and re-ran once, which succeeded. The first `modowners.ts` call failed for a missing argument and was re-run with the generated-dir argument. `bun repo cargo` was not run (section 7).

## 6. Widening, imports and rewrites

- `pub(super)` widening (mod mode only, name-driven over-approximation by the stager, then proven by the oracles above): cas 17 sites, local-bootstrap 193 (140 are private fields of the wire structs that the transport impl reads), space-app 2, testkit 0; include-mode roots need none. A staging defect found by the scoped differential and fixed in `c4p38.py`: a single-line private tuple struct (`struct SecretChannelKey([u8; 32]);`) lost its `pub(super)` when its field was widened on the same line; the stager's `Widener` now combines both edits.
- Re-based relative paths: `include_str!` in session 1, open-plan 2, testkit 1 got one more `../` (a fragment or child resolves relative to its own file; the differential shows the same unreadable or readable target as the original).
- Root `use` lines that only children use are counted as used through the glob imports; `unused_imports` and `private_interfaces` warnings may still move under the real compiler and are listed as review points in section 7.

## 7. Real-check commands for the compile queue

No `COMPILE ADMITTED` line in `📓️FLEET.md` at 09:05; `F/🗑️generated/cargo-mutex.lock` is held by a peer (created 08:42). Run one launcher call at a time under the fleet mutex. Prefix: `cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 --`

1. Lib: `check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-artifact-authority -p semio-hub-local-bootstrap -p semio-hub-space-app -p semio-hub-route-testkit --lib --keep-going --message-format=short`, then once more with `--features semio-hub-artifact-authority/testkit,semio-hub-space-app/testkit`.
2. Dependents: `-p semio-hub-artifact-creation -p semio-hub-artifact-checkin -p semio-hub-lag-rebootstrap -p semio-hub-state -p semio-hub-routes -p semio-hub-space -p semio-hub` with `--lib --keep-going`.
3. Tests: `check … -p semio-hub-artifact-authority -p semio-hub-local-bootstrap -p semio-hub-space-app -p semio-hub-hostile-input-laws -p semio-hub-session-route-laws -p semio-hub-open-plan-laws -p semio-hub-space-laws --tests --keep-going`, and the three laws crates with `--all-features`.
4. Then `test -p semio-hub-hostile-input-laws -p semio-hub-session-route-laws -p semio-hub-open-plan-laws --lib` (the names must equal the pre-split names; the socket-grant native stages use `tests::<name>` with `--exact`).
Expected review points: `unused_imports` of root `use` lines, `private_interfaces`/`unreachable_pub` warnings of the `pub(super)` items, the real expansion of `app_commands!` inside `🕹️space-command`, and the `pub(super)` tuple field of `SecretChannelKey` (the E0451 scan is clean, rustc will confirm).

## 8. Foreign findings, hand-offs

- **Pre-existing exact-filter mismatch (not mine, not changed).** The four hostile-input native stages run `--exact tests::<name>`, but those tests live in `mod quick` / `mod long`, so their real paths are `tests::quick::<name>` and `tests::long::<name>`; `cargo test --exact` would match zero tests. The module structure is unchanged by this split (the test paths are identical before and after). Recommendation for the owner of `socket-grant-command-source`: add the module segment to the stage args.
- **Pre-existing stale space unit tests.** `🪐️space/🧪️tests/🔬️unit/🦀️.rs` uses `SPACE_CONFIG_MAXIMUM_BYTES` and `SPACE_CONFIG_TEXT_BYTES`, which exist nowhere (E0425 in the `a0` baseline and after).
- **Pre-existing missing input.** `🧰️framework/…/📇️directory/🧬️schema/🟦️.ts` does not exist; two tests of the descriptor-digest suite fail on it before and after (also reported by PKG-15).
- **Peers.** `H/📇️directory/🔐️authorization/🔌️socket-grant/🧪️tests/📋️native-law-plan/🟦️.ts`, the socket-grant fixture and the descriptor-digest fixture were already edited by the PKG-15 executor; I re-read each file before editing and touched only the lines named above. The space file carries a peer edit of 6 lines (part of the snapshot).
- Not verified: no cargo/rustc build of the real crates, no run of any Rust law, no run of `interactive-job-catalog-check` through its launcher (needs the process envelope; the needle check by hand is above), no vitest gate beyond the ones listed.

## 9. Files

Created (42 children, each `🦀️.rs` in its folder): section 1; the list is `F/c4p38-tools/newfiles.json`. Rewritten roots (7): the seven targets. Edited readers and fixtures (5 repo files): section 4 (`📖️source-readers/🟦️.ts`, `📋️native-law-plan/🟦️.ts`, `🧱️socket-grant-command-source/🔣️.json`, the descriptor-digest fixture `🔣️.json`, `🪐️space/📦️packages/🦀️rust/📜️script.ts`). Tools and snapshots: `F/c4p38-tools/` (`c4p38.py` stage/install/reinstall/verify, `plans38.py`, `c4p38lib.py`, `c4p38check.py`, `c4p38names.py`, `c4p38fmt.py`, `c4p38lossless.py`, `c4p38defscan.py`, `scoped38.py`, `posdiff38.py`, `injlib.py`, `c4p38-census.ts`, `c4p38-readers.ts`, `outline38.py`, `families/c4p38.json`, `newfiles.json`, `orig/` with `SHA256`). Rollback: copy `c4p38-tools/orig/<n>.rs` back over the seven targets (n = 1..7 as in section 1) and delete the 42 child folders listed in `newfiles.json`, then revert the five reader files (their changes are listed in section 4).
