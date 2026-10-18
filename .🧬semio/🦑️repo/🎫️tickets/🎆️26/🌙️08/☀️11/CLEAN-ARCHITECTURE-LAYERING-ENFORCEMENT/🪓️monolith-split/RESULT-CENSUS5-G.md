# Result CENSUS5-G (prefix `c5g`)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool, no cargo/bun repo/nx. `F` = this folder. `PL` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`. `H` = `PL/🖥️host`. Tools: `F/c5g-tools/` (a copy of `c5b-tools` with `.cache/c5b` re-pointed to `.cache/c5g`, plus `resolve_readers.py`, `tsedit6.py`, `modtree.py`, `asserts.py`, `litdiff.py`). Logs: `F/🗑️generated/c5g-*` (nothing deleted). Sandboxes: `/Users/ueli/.cache/c5g/`.

## 0. Verdict

**DONE (static evidence) for package 6 of `MONOLITH-CENSUS-5.md` section 4**: 5 files, 8,883 lines, split into 33 children; no file of the package is over 1,000 lines any more (largest child 562).

- Found state: the five files were already cut and installed on 2026-10-18 00:13 by the CENSUS5-B executor, which was cut off at about 00:19 before it verified the readers or wrote a result (`RESULT-CENSUS5-B.md` still says "package 6: not started"; its tools hold the specs `he, hg, hr, hs`, the fragment table `hc` and the snapshots `orig/{he,hg,hr,hs,hc}.rs`, hashes equal to the git index). I did not re-cut anything. I verified the install on the live tree, re-pointed the source-text readers it had not handled, and wrote this report.
- Lossless, structure, definitions, glob ambiguity, name clash, formatting debt, census, scoped patched differential and typeck-level oracle all pass (section 2).
- 18 source-text readers re-pointed (section 3). The TS runtime is not available (see risks), so those edits are proven by a Python twin of `readModuleTreeSource` and a literal differential, not by `bun test`.

## 1. What was cut (before to after, lines)

| tag | file (tail under `H/`) | before | root | children | largest child | total after |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| he | `⚡️effects/🦀️.rs` | 1,432 | 78 | 5 | 484 | 1,476 |
| hg | `🎭️guest-runtime/🦀️.rs` | 1,848 | 94 | 7 | 388 | 1,930 |
| hc | `🧪️tests/🔬️guest-cold-relay/🦀️.rs` | 1,012 | 20 | 6 | 217 | 1,025 |
| hr | `🧭️routers/🦀️.rs` | 1,924 | 104 | 6 | 445 | 1,987 |
| hs | `🧵️shard/🦀️.rs` | 2,667 | 89 | 9 | 562 | 2,750 |

(The index copy of `hg` is 1,848 lines because a peer had staged one edit; HEAD is 1,843. The snapshot equals the index.) 8,883 lines became 9,168 (wiring, one docstring per child, repeated region markers).

Children (folder, lines), in mount order. Mechanism: `he`, `hg`, `hr`, `hs` are `#[path]` modules with `pub use child::*` (`modsplit.py`); `hc` is a flat test root with `include!` fragments (`frag.py`, 26 tests).

- **he**: `🆔️actor-registries-and-envelope-injector` 216, `🧾️completion-sink-and-fault-encoding` 118, `🌉️router-effect-handler-and-jobs` 408, `💾️storage-capabilities-backbone-and-metrics` 172, `🎛️async-effect-executor` 484
- **hg**: `🧱️guest-types-and-runtime-trait` 194, `🤖️mock-guest-runtime` 388, `🧠️owned-runtime-adapter` 81 (`pub(crate) use`), `🏠️wasmtime-host-state-and-wit-hosts` 331, `🐎️wasmtime-runtime` 247, `🗂️wasmtime-guest-runtime-and-codec-dispatch` 375, `🎛️guest-runtimes-and-test-doubles` 220
- **hr**: `🚪️io-router` 445, `💡️inference-router` 401, `🕸️plugin-graph-and-instance-directory` 243, `🎯️mutation-router` 239, `🧾️transaction-coordinator` 311, `🧩️app-router-and-opening-resolver` 244
- **hs**: `📨️shard-frame-and-budget-bridge` 234, `📊️shard-outcome-and-loop-state` 238, `📄️fixed-replay-seed` 327, `🚦️deferred-owner-ring` 205, `🏗️shard-loop-construction-and-replay-drive` 562, `🌀️shard-loop-registry-and-pump` 350, `🎬️shard-loop-frame-turns-and-job-control` 461, `🔬️replay-lifecycle-and-fault-helpers` 95, `🛰️shard-transports-and-test-doubles` 189 (the 1,934-line `impl ShardLoop` was cut into four `impl ShardLoop { .. }` blocks at method boundaries, cut lines 1590 and 1934 of the snapshot)
- **hc**: `♻️relay-lifecycle-fixtures-and-test-relay` 171, `🗝️registry-bounds-reaper-and-publication-bounds` 119, `🧰️relay-session-helpers-and-waits` 181, `🪂️dropped-futures-wake-fallback-and-cancellation` 210, `💥️panics-and-cleanup-cancel-failures` 217, `🛣️concurrent-routes-and-poisoned-slot` 107

All 33 folders sit next to their root. `hs` also holds children of other slices (`🧪️tests/🔬️unit` of c5f, `🧵️executor`, `🎟️grant`, `🪪️identity`, `🔁️lifecycle`); the `hs` root carries no mount for the c5f child, so no line of it was touched.

## 2. Evidence (all run in this session on the live tree)

| Check | Command (tools in `F/c5g-tools/`, env `C5B_SCR=/Users/ueli/.cache/c5g`) | Result |
| --- | --- | --- |
| Snapshots vs git | `cmp` of `orig/*.rs` with `git show HEAD:` and the index | he, hr, hs, hc equal to HEAD and index; hg equal to the index (a staged peer edit of 5 lines) |
| Lossless | `modsplit.py lossless he hg hr hs <dir>` | `LOSSLESS OK` x4, `missing 0`; extras only wiring (`mod`, `pub use`, `use super::*;`), docstrings, markers, the declared `impl ShardLoop` re-openings (`declared 2 }`); regions orig/new he 15/15, hg 8/20, hr 9/16, hs 6/6 (endregions equal) |
| Fragments exact | `chk.py live hc` | `CHK OK`: `gone=0 extra(non-marker)=0`, tests 26 to 26, spliced sequence equal, include targets equal and existing |
| Structure | `vmod.py live he hg hr hs` | `VMOD OK`: every child mounted once, docstring emoji first/equal to folder/unique/different from root, no sibling emoji clash, path lengths, brace depth, rustfmt parse, include targets (hs 1 to 1), no `[DEBUG]`, module-name clash |
| Definitions | `defscan.py he hg hr hs` | 0 cross-file private leftovers (28, 4, 21, 21 declarations scanned) |
| Glob ambiguity | `ambig.py live he hg hr hs` | `AMBIG OK`, 0 candidates |
| Name clash | `C5B_LIVE=1 modclash.py he hg hr hs` | `modclash OK` |
| Taxonomy | `rgi.mjs` on the 33 names, `tax.ts` on the 33 dirs, emoji list `c4p43/folder-emojis.txt` | `bad=0`, `{"dirs": 33, "unresolved": []}`, all 33 emojis already used as folder prefixes |
| Formatting debt | `fmt.py he hg hr hs` | rustfmt hunks original/new he 16/16, hg 44/44, hr 9/9, hs 43/43; changed-line multisets identical; no parse failure |
| Census (canonical `readRustSourceClosureV1`, `census-own.ts`) | lib / test files, bytes, active lines (ratio of 32 / 262,144 / 4,000) | he 1 to 6 / 4 to 9 files (0.357 / 0.492); hg 5 to 12 / 10 to 17 (0.774 / 0.846); hr 1 to 7 / 10 to 16 (0.494 / 0.791); hs 5 to 14 / 6 to 15 (0.801 / 0.805); hp (post-turn-relay, mounts hc) test 6 to 12 (0.862); shard-laws test 14 to 23 (0.723). All inside the gate. Log `c5g-census-own-after.jsonl` |
| Scoped patched differential | `oracle5.py he hg hr hs hp` (original restored in a sandbox against the live tree, lib and `--test`, logs `c5g-oracle-pkg6-{a,b}.log`) | **identical** in all 10 comparisons: he 86 / 196, hg 309 / 442, hr 237 / 443, hs 275 / 284, hp 293 / 435 errors (lib / test), warnings equal, only-orig 0, only-new 0 |
| Typeck-level oracle | `posrun.py` family `c5b6` (run by the c5b executor, pre 00:13 and post 00:19, files untouched since; reports `c5b-c5b6-{pre,post}-report.txt`) | canary (E0308, E0616, E0624) reached before and after; privacy 0/0, trait-scope 0/0, resolve 0/0, type 45 / 73 and other 1 / 3 equal (lib / test); only stub-involved rows grew (3,310 to 3,381 lib, 4,899 to 4,999 test) |

## 3. Source-text readers

Search: `git grep -F` of the dir segments and file tails (`c5g-readers-wide.txt`), plus `resolve_readers.py` (every string literal ending in `.rs` resolved against the file directory and the repo root, plus the segment literals; `c5g-resolve-readers{,-after}.txt`), plus a read of every candidate. The helper `F/c5g-tools/tsedit6.py` applied the edits (each asserts its match count; reads of the cut roots become `readModuleTreeSource(<same path>)`, the import of the module-tree reader is added with a repo-relative path).

| Reader (tail under `🧰️framework/🛍️products/`) | What it read | Action |
| --- | --- | --- |
| `H/⏱️deadline/🧪️tests`, `H/♻️publication/🧪️tests`, `H/🔔️wake/🧪️tests` (2 sites) | the concatenation host root, guest-runtime root, relay, routers root | **edited**: guest-runtime and routers entries read their module trees (relay and host root unchanged, order unchanged) |
| `H/🎟️context/🤝️admission/🧪️tests` | same list; the shard root; the `GuestRuntime` trait slice up to the `MockGuestRuntime` region | **edited**: list as above; shard root by `readModuleTreeSource`; the slice by `sliceModuleTreeSource(<guest-runtime root>, "pub trait GuestRuntime", "//#region 🔖️MockGuestRuntime")` (a plain `indexOf` slice breaks because the root now holds the region marker before the trait child) |
| `H/🧪️tests/🧱️ports` | guest-runtime root (public port closure, tree-sitter parse) | **edited** (module tree) |
| `H/🧬️component-codec/🎟️retained-turn/💼️jobs/🧪️tests` (2 sites) | guest-runtime root (job signatures) | **edited** (module tree) |
| `H/🧵️shard/🎟️grant/🧪️tests` (3 sites), `.../🎟️supplier` (2 sites) | the shard root (`ShardFrame` wire text, `granted_budget`, `execute_turn_for`) | **edited** (module tree) |
| `os/🔨️modules/🛎️services/🧪️tests/🧮️compute-retained` (2 sites) | effects root (`|outcome|` closure, `JobOutcomeView::Complete`) | **edited** (peer-edited file; only the two `effects` reads) |
| `PL/🧪️tests/🧪️shardclient-reserved-response-settlement` | the shard root (`pub enum ShardFrame {..} impl ShardFrame`) | **edited** (module tree; the regex still matches, body identical to the snapshot) |
| `H/📦️packages/🦀️rust/📜️script.ts` (`guest-fault-check`, `ui-patch-marshalling-check`, `router-effect-source-custody`) | guest-runtime root (2 sites); shard root parsed for `ShardLoop` struct/impl/fn nodes | **edited**: guest-runtime by module tree; the shard root of the syntax loop by module tree (`ShardLoop` struct, 4 impl blocks and `test_identity_issuer` are in the tree, so `nodes.length >= 3` holds) |
| `🦑️repo/.../📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run`, `.../🔬️interactivity-audit/🏁️run`, `.../🎛️commands/✅️verify` | the shard root through `policyReadFileSafe` (fixed-replay, executor and ingress laws) | **edited** (`policyReadModuleTreeSource`) |
| `.../📚️library/🔍️discovery/📖️source-access` `POLICY_SPLIT_SOURCE_ROOTS` | list of roots read as one text (also used by `policyReadRustPolicySource`, the interactivity audit file list and `interactivity-live-reconcile`) | **edited**: `H/🧵️shard/🦀️.rs` registered after the host root (one anchored line); the `rust-policy-source-evidence` law iterates the list and still holds by construction |
| `PL/⚡️effect-backbone/🟦️.ts` doc comments, other `🧵️shard/🦀️.rs` mentions in doc comments and `//!` prose | prose | no edit |
| crate manifests (`Cargo.toml` lib `path = "../../🦀️.rs"`), `📋️build-owners.json`, `project.json`, workspace members | roots only | no edit; no crate, target, command or owner row changed |

Proof of the TS edits without a TS runtime:
- `modtree.py` is a Python twin of `readModuleTreeSource` (parent first, `#[path]` and literal `include!` children in declaration order, `🧪️tests` mounts and paths leaving the scope skipped). `asserts.py` evaluated every `includes(..)`/`toContain(..)` literal (positive and negated) of the edited tests against the corpus they now read: deadline, publication, wake (the `pump_close` slice stays in the relay, the missing `outcome_payload` end marker behaves as before), admission, grant tests, supplier: 0 failures except the pre-existing ones in the risks. The admission slice returns the trait file with `async fn admit_actor_context` and `async fn close_actor_context`.
- `litdiff.py`: every string literal of at least 8 characters in the 13 edited reader files and the two law files (`live-replay`, `database`) that occurs in a snapshot also occurs in the same root's tree: 0 regressions. The 32 `shardSource.includes(..)` checks of `interactivityShardExecutorFailures` give the same verdict on the snapshot and on the tree, and its 3 regexes match neither. The five `shard.indexOf(..)` anchors of `toolJobLiveFixedReplayExact` are unique in the tree (all in the new children), so first-occurrence slicing is order independent.

## 4. Open risks

- **No compile and no TS run.** The compiler authority is not mine to call, and the TS runtime cannot start: `node_modules` is a dangling symlink to a swept cache (`ajv`, `web-tree-sitter` and the workspace packages do not resolve). All TS edits are checked by the Python twin and the literal differential only; the `ports` test and the host script parse the concatenated tree with tree-sitter (children start with `//!` lines and `use super::*;`, which are comments and a `use` to that grammar) and that parse was not run.
- **Pre-existing failures that predate the cut (not mine, not touched):** `🔔️wake/🧪️tests` asserts `pub struct GuestRelayWakeContextTurn` and `context.consume_retained(receipt.spent)?` in the host root `H/🦀️.rs` (absent there and in the snapshots); `guest-fault-check` of the host script asserts the literal `original.map_err(retained_turn_wire::failure_from_wit)` (the source spells the path out in full, in `🗂️wasmtime-guest-runtime-and-codec-dispatch`); the two test-name anchors `replay_seed_max_plus_one_returns_the_exact_spawn_owners_unchanged` and `mounted_replay_rejects_wrong_route_seed_generation_and_worker_before_work_and_closes_one_owner_per_sub_eight_ms_opportunity` in `toolJobActorProgressOverlayExact` live in `🧵️shard/🧪️tests/🔬️unit/🦀️.rs` (c5f), never in the shard root.
- **Shard trees include the pre-existing mounts.** `readModuleTreeSource(shard root)` also reads `🔁️lifecycle`, `🎟️grant/📨️event`, `🪪️identity` and `🎟️grant` (they were separate files before the cut too). I checked every negated assertion against that wider text: unchanged.
- **Catalog entry not repointed:** `🦑️repo/.../📚️library/🖼️assets/📽️nested-cargo-package-projection/🔣️.json` lists `H/🧵️shard/🦀️.rs` as an authored consumer of transform `wgpu-owner-runtime-shard` (the doc comment `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/ 🎠️runtime.rs` now lives in child `🏗️shard-loop-construction-and-replay-drive`). The catalog is sha-pinned in `taxonomy.json` (`authorityCatalogSha256`), its wgpu package generation is blocked by design, and the entry is metadata whose source token is absent from the file even in HEAD, so I left it; the wgpu owner should move the path when that package is generated.
- `RESULT-CENSUS5-B.md` still lists package 6 as not started. Not edited (another slice's file).

## 5. Compile queue

Crates to compile (lib and cfg(test); `hc` is a test fragment set of the post-turn-relay crate):

```
cargo check -p semio-framework-plugin-host-effects -p semio-framework-plugin-host-guest-runtime -p semio-framework-plugin-host-routers -p semio-framework-plugin-host-shard -p semio-framework-plugin-host-post-turn-relay -p semio-framework-plugin-host-shard-laws --lib --tests
```

(through `bun repo cargo --store=shared ... -- check ...` as in `RESULT-CENSUS5-A.md` section 7).

## 6. Files

- Cut and installed earlier by c5b (verified here): the 5 roots of section 1 and 33 children, one `🦀️.rs` per new folder next to its root (folder list in `F/c5b-tools/newdirs.txt`; this slice's 33 in `/Users/ueli/.cache/c5g/dirs6.txt`).
- Edited by me (readers): `H/⏱️deadline/🧪️tests/🟦️.ts`, `H/♻️publication/🧪️tests/🟦️.ts`, `H/🔔️wake/🧪️tests/🟦️.ts`, `H/🎟️context/🤝️admission/🧪️tests/🟦️.ts`, `H/🧪️tests/🧱️ports/🟦️.ts`, `H/🧬️component-codec/🎟️retained-turn/💼️jobs/🧪️tests/🟦️.ts`, `H/🧵️shard/🎟️grant/🧪️tests/🟦️.ts`, `H/🧵️shard/🎟️grant/🧪️tests/🎟️supplier/🟦️.ts`, `H/📦️packages/🦀️rust/📜️script.ts`, `os/🔨️modules/🛎️services/🧪️tests/🧮️compute-retained/🟦️.ts`, `PL/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts`, `🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts`, `.../⚖️laws/🔬️interactivity-audit/🏁️run/🟦️.ts`, `.../🎛️commands/✅️verify/🟦️.ts`, `.../🔍️discovery/📖️source-access/🟦️.ts`.
- Tools kept in `F/c5g-tools/`; logs `F/🗑️generated/c5g-*`; no `[DEBUG]` line added anywhere.
