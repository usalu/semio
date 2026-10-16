# Result: PLUGIN-HOST-CARVE (`phc`)

Executor: Sonnet 5.5, foreground, no sub-agents, no ticket tool, no modifying git command. Tools: `phc-tools/`. Logs: `🗑️generated/phc-*` (nothing deleted).
Slice: `semio-framework-plugin-host` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host`, `H` below) and its test component.

## 0. Verdict (rewritten after each milestone)

CARVED AND REGISTERED, STATICALLY PROVEN, REAL COMPILE BLOCKED BY THE HEADROOM GATE (last update 2026-10-16, clock 10:56).

- The 32-file / 1,026,642 B / 14,532-line crate is now 14 producer crates plus a thin facade (15 packages). Every lib and test closure, native and wasm, is inside the target budget (largest: 3,429 lines, 6 files, 199,917 B for the post-turn-relay test closure; largest bytes: 218,501 B for the guest-runtime test closure).
- Stitched `rustc +nightly-2026-07-20 --emit=metadata` differential against the monolith, run on the final on-disk tree: native 370 / 370 errors, 0 only-split, 0 only-base; all 15 test configurations 0 only-split.
- Registry verifier (manifests, byte-sorted workspace members and owner rows, 84 launch routes in each of the two launch files, dependency DAG including dev edges, testkit forwarding): `ALL REGISTRY CHECKS PASSED`.
- TypeScript tests: the 19 test files under `H` give exactly the pre-carve result (8 files pass, 11 fail as they already did, same failing test names).
- NOT DONE: a real `bun repo cargo … check`. The launcher reaches the physical-headroom gate and is refused there (about 32 MB short, same as `RESULT-COMPILE-PASS-2.md`); I have no compiler result for any new crate. See section 7. Until that exists, treat the carve as "differential-clean, not compile-proven". The brief's rule 2 is not satisfied by static evidence alone.

## 1. Measurement (before)

Counter: `readRustSourceClosureV1` through `🗑️generated/census.ts` and `phc-tools/closure.ts` (`🗑️generated/phc-closure-before.json` lists every file).

| closure | files | bytes | active lines |
| --- | --- | --- | --- |
| lib, native | 32 | 1,026,642 | 14,532 |
| lib, wasm32-unknown-unknown / wasip2 | 30 | 1,016,502 | 14,328 |
| test, native | 103 | 3,389,794 | 23,195 |

The test closure embedded 33 files; one of them was the generated scale registry (`💻️os/🧪️testing/⚖️scale/🤖️generated/📇️registry/🔣️.json`, 1,742,341 bytes) that a metrics law included with `include_str!`. It is now read at run time through the function `scale_fixture_registry_json` (embedded asset turned into a runtime port, as the brief asks).
The test component `H/🧪️testing/🧩️component` (626 lines) is refused only through `semio-framework-plugin` (oversized, peers `pi1`/`pwp7`); it depends on nothing of the host crate, so this carve cannot change its row.

## 2. Plan: clusters and item-level graph

Method: the hub carve engine (module model, in-memory item moves, global path rewrite, export finalizer, member scan), copied into `phc-tools/` and extended. Item graph from `itemgraph.build` over the real lib root: 943 items, 2,599 edges, 4 cyclic SCCs (`ShardDriveWake`/`ShardExecutor`/`poll_retained_drive_once` inside the executor; `GuestRuntimes` and `SnapshotCall`; `ShardLoop` and `defer_completion`; `GuestRelayMountedReaper` and `GuestRelayMountedRegistry`). No SCC spans two target crates.

File graph facts that fixed the partition:

- `⚡️effects` is a leaf. `📥️imports` depends on effects, operation and the engine only.
- `🧭️routers` depends on the relay only through `PluginInstanceHandle`.
- `🔔️wake` depends on the relay only through two small types (`GuestRelayWakeReceipt`, `GuestRelayWakeReceiptReceiver`), which moved into the wake crate.
- `🚪️io/🪶️snapshot` holds an inherent `impl GuestRuntimes`: the god-object rule keeps it with the guest runtime.
- The monolith root held the shared engine, the session glue and the only `HostWithStore<ActorHostState>` impl. The engine items form a leaf crate; the impl moved to the guest runtime (orphan rule: it needs `ActorHostState`).
- `bindgen!` (the one `actor_bindings` module) is used by the guest runtime, cold pair, UI patch, retained-turn wire, operation, snapshot and imports; it became the bottom crate together with every kernel to WIT conversion.
- `GuestRuntimes` had a test variant `Recording(shard::RecordingRuntime)`: an upward edge. `RecordingRuntime` moved into the guest runtime next to `MockGuestRuntime`.
- `ShardTransports::Loopback` named `tests::LoopbackProbe`: `LoopbackProbe` moved next to `LoopbackTransport` in the shard crate.
- `ShardTransports::SharedThread(SharedThreadTransport)` named a type that lived in the executor, which in turn drives `ShardLoop`: a crate cycle shard to executor to shard. `SharedThreadTransport` (struct and its `ShardTransport` impl) moved into the shard crate next to the other transports; the executor imports it. See section 6.

Crate table (all under `H`; package `semio-framework-plugin-host-<suffix>`, id `os-plugin-host-<suffix>`, project `@semio-tech/framework-plugin-host-<suffix>-rs`):

| suffix | lib root | role |
| --- | --- | --- |
| `wit` | `🔗️wit-bindings/🦀️.rs` (new) | `bindgen!` of `world actor`, `wit_*` aliases, conversions, cold pair, UI patch, retained-turn wire |
| `engine` | `🔧️engine/🦀️.rs` (new) | shared Wasmtime engine, epoch deadlines, store limiter, compiled-artifact cache, isolated compile, worker pool |
| `operation` | `🚪️io/🎛️operation/🦀️.rs` | operation slot, Wasmtime request bridge, borrowed return walk, native authority fixture |
| `effects` | `⚡️effects/🦀️.rs` | async effect execution |
| `imports` | `📥️imports/🦀️.rs` | host-async imports |
| `guest-runtime` | `🎭️guest-runtime/🦀️.rs` | `GuestRuntime`, mock and recording doubles, owned and Wasmtime runtime, snapshot calls, component codec, instance opening |
| `guest-laws` | `🧪️tests/🏛️guest-laws/🦀️.rs` (new) | test-only: owned and Wasmtime runtime laws, patch ownership, snapshot refusal, shared engine laws |
| `wake` | `🔔️wake/🦀️.rs` | original wake currency, root, driver policy |
| `post-turn-relay` | `🔀️post-turn-relay/🦀️.rs` | relay pool, slots, leases, cold relay jobs, mounted registry, `PluginInstanceHandle` |
| `routers` | `🧭️routers/🦀️.rs` | IO, inference, plugin graph, mutation, instance directory, transaction coordinator, app, opening resolver |
| `process-transport` | `🧵️shard/🚚️process-transport/🦀️.rs` | process and stdio shard transports |
| `shard` | `🧵️shard/🦀️.rs` | `ShardLoop`, identity, grant, lifecycle, event custody, all transports' enum |
| `shard-executor` | `🧵️shard/🧵️executor/🦀️.rs` | `ShardExecutor` |
| `shard-laws` | `🧵️shard/🧪️tests/🏛️laws/🦀️.rs` (new) | test-only: loop, lifecycle, identity and transport laws |
| (facade) `semio-framework-plugin-host` | `H/📦️packages/🦀️rust/🦀️.rs` | explicit `pub use`, session and metrics glue, activation, package assets, schema-parity test |

DAG (lib edges, downward, verified by the registry verifier from the real manifests): wit < operation < guest-runtime < {shard, post-turn-relay} < {shard-executor, routers}; engine < {guest-runtime, process-transport}; effects < imports; wake < post-turn-relay; process-transport < shard; shard < post-turn-relay; everything < facade.

## 3. Census rows (after)

Canonical counter (`phc-tools/closure.ts`, five configurations per crate: lib native, lib wasm32-unknown-unknown, lib wasm32-wasip2, test native, test wasm32-unknown-unknown; with and without feature `testkit`), maximum over the configurations; log `🗑️generated/phc-closure-after.log`. The repo census (`census.ts`, `🗑️generated/phc-census3.log`) lists all 15 packages as `overLimit: false`.

| package (`semio-framework-plugin-host-…`) | files | bytes | active lines |
| --- | --- | --- | --- |
| `wit` | 11 | 72,812 | 972 |
| `engine` | 2 | 31,495 | 541 |
| `operation` | 12 | 61,134 | 333 |
| `effects` | 4 | 123,834 | 1,922 |
| `imports` | 2 | 58,813 | 987 |
| `guest-runtime` | 10 | 218,501 | 2,295 |
| `wake` | 7 | 69,536 | 289 |
| `post-turn-relay` | 6 | 199,917 | 3,429 |
| `routers` | 10 | 181,009 | 3,097 |
| `process-transport` | 1 | 23,005 | 484 |
| `shard` | 6 | 207,146 | 3,079 |
| `shard-laws` | 14 | 189,100 | 2,261 |
| `guest-laws` | 12 | 69,669 | 869 |
| `shard-executor` | 6 | 89,157 | 1,364 |
| facade (no suffix) | 11 | 89,090 | 1,451 |

All rows are under the target (3,500 lines / 28 files / 230,000 B) and the hard limit (4,000 / 32 / 262,144). Every new manifest carries `maxSourceBytes = 262144`, `maxSourceFiles = 32`, `maxActiveLines = 4000` (the facade has none, as before). The census marks 11 of 15 as `closureCompilable: false` only because of `oversizedDeps: 1`; that dependency is an oversized foreign crate (`semio-framework-plugin` and friends, peers `pi1`/`pwp7`), not one of mine.

## 4. Registration (recipe)

- Manifests: 14 new `Cargo.toml` + 14 `📋️project.json` (dependencies computed from usage, original comments carried, optional/dev split, `testkit` forwarded along family edges); facade manifest keeps identity, `[lib]`, `[[bin]] semio-shard`, `[[test]] original_host_factory`.
- Root `Cargo.toml` workspace members: 14 added in byte order. `📋️build-owners.json`: 14 rows in byte order. Launch routes: 84 (14 crates x build/check/test x prototype/release) in `.vscode/launch.json` and in `.vscode/🧩️launch.seed.jsonc`, order bands `900.33xxx` / `922.33xxx` / `903.33xxx`, `env` line included.
- Taxonomy (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, `crate-module-children`): added `🏛️guest-laws`, `🏛️laws`, `🔀️conversion`, `🔗️wit-bindings`, `🔧️engine` (68 to 73 names, byte-sorted).
- Host script `H/📦️packages/🦀️rust/📜️script.ts`: `HOST_FAMILY`, family-wide check/test, remapped law groups to the crate that now owns each law, lifecycle rewrite and the source-text readers (153 insertions, 82 deletions).
- Name-keyed references outside the slice (`git grep` over every non-Rust, non-manifest file for `semio-framework-plugin-host` / `os-plugin-host`): only the two scripts of the re-points below, `.vscode` (the new routes), the root `Cargo.toml`, a hub GIS manifest-text test and the test-component script. The last two name the facade (`semio-framework-plugin-host`, still a package) or `semio-framework-plugin-host-test-component` (another package), so they stay valid. No observed-profile corpus row names the host crate in that search; I did not read the corpus files themselves.
- Region markers preserved (region report: no missing name, no unbalanced file).

### Consumer re-points (rule 4: one-line edits outside the slice)

1. `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts`: the imports law group `package: "semio-framework-plugin-host"` becomes `"semio-framework-plugin-host-imports"` and the law path loses the `component::imports::` prefix (`effect_conversion_tests::request_inference_proposal_preserves_the_closed_kind`).
2. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts`: the SnapshotSqliteRefusal law groups move from package `semio-framework-plugin-host` to `semio-framework-plugin-host-guest-laws` (the group list text is otherwise identical; the editor also re-spaced the object literal).

No Rust consumer is touched: every consumer-named path resolves through the facade's explicit `pub use` list (section 5, item 8).

## 5. Verification (real results)

1. Stitched differential on the final tree (`phc-tools/post_diff.py`, log `🗑️generated/phc-postdiff2.log`): `[native] base 370 / split 370; only-split 0 only-base 0`; all 15 `test:<crate>` configurations `only-split 0`. (`only-base` in the test lines is the other crates' test modules that a one-crate configuration does not mount.) The union is blind to typeck and field/method privacy, trait scoping and to Cargo-level facts (cycles, undeclared dependencies); items 3 to 6 cover those.
2. Registry verifier (`phc-tools/verify_registry_host.ts`, log `🗑️generated/phc-registry-verify.log`): `ALL REGISTRY CHECKS PASSED` (15 crates, 2,633 owner rows, 12,811 launch / 9,843 seed configurations).
3. Independent dependency check (`phc-tools/depcheck.py`): regex over every file of each crate against the manifest. Only finding: `semio_framework_trace` in guest-runtime is a dev-dependency but a regex hit sits in a `#[cfg(test)]` module not under a `tests` folder (`🚪️io/🪶️snapshot/📨️metadata/🦀️.rs`); verified test-only, so correct.
4. Definition scan, orphan scan, lossless and region checks against the pre-carve text kept in the persisted state (`phc-tools/checks_post.py checks_host`, log `🗑️generated/phc-checks-post.log`): 1162 items before, 1162 after; lost 1 / gained 1 = the registry constant became `scale_fixture_registry_json` (intended); test inventory 198 / 198, lost 0, gained 0; orphan scan empty; regions fine. Lossless line multiset shows only rewritten path lines (`crate::` to crate names).
5. Item-body skeleton check (`checks_post.py bodycheck`, log `🗑️generated/phc-body-post.log`): 1117 items, 21 with differences, all single-segment path prefixes the collapse does not normalize (`crate`, `super`, `shard` to a crate name) plus the three intended rebases below.
6. Path rebases forced by the move of `CARGO_MANIFEST_DIR`: process-transport shard-kill law (`../../🧵️shard/🪪️identity/…` to `../../../../🪪️identity/…`, 3 sites), `SCALE_COMPONENT` in the UI patch law (5 to 7 `..` hops), `bindgen!` path (`../../../🧬️schema` to `../../../../🧬️schema`). `owned-instance-open` walks to `nx.json`, so it needs none; `schema-parity` and the metrics law live in the facade, whose package dir did not move.
7. `rustfmt` parse of the 62 changed Rust files: 0 failures; longest path 157 bytes (limit 240); new folder names pass the emoji admission check; every `#[path]` mount resolves.
8. Consumer API (`facade_probe.py` vs `facade_probe_base.py`, log `🗑️generated/phc-facade-probe.log`): of 75 distinct paths the repo's Rust names below the facade root, 73 resolve and 2 do not, and the same 2 do not resolve in the monolith either (pre-existing, foreign): `semio_framework_plugin_host::Budget` (hub tests `🌎️hub/🧪️tests/🔏️trusted-catalog-profile/🪶️count/🦀️.rs`, `…/🔬️bin-unit/🪶️count-lease/🦀️.rs`) and `semio_framework_plugin_host::shard::ShardIdentityIssueRefused` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs:1537`). I did not add re-exports for them (no compatibility shims, not my error).
9. TypeScript (`phc-tools/ts_run.py`, `🗑️generated/phc-ts-before.json` vs `phc-ts-after.json`): 19 files, identical pass/fail counts and identical failing test names in all 19. The 11 failing files fail the same way before and after (their source-text expectations were already stale in the monolith); not mine to repair.
10. Trait-in-scope scan (`checks_post.py traitscope`, log `🗑️generated/phc-traitscope-post.log`): 5 first-party-trait hits, all false positives (`collect` is `Iterator::collect` in four files; `checkpoint` in `🚪️io/🪶️snapshot/📨️metadata/🦀️.rs` is the inherent `NativeEncodeControl::checkpoint`). The remaining output lists external `use` names that the carved file no longer binds and does not name; it is informational.
11. Workspace lints are `warn` only (`[workspace.lints.rust]` in the root `Cargo.toml`), so unused-import style warnings from the carve cannot fail a build; the differential compares errors only.
12. Seeded regressions (`phc-tools/sensitivity_post.py`, log `🗑️generated/phc-sensitivity-post.log`), all caught:
    - `pub` removed from `build_shared_engine`: differential reports 1 only-split error naming it.
    - the bindings `use` removed from the guest runtime: 14 only-split errors.
    - struct `PackageRef` deleted: definition scan reports it lost (next to the intended constant).
    - `shard` manifest given a dependency on its executor: the cycle check reports 2 cycles (0 on the real manifests).

## 6. Defects the verification found in my own carve

- **Cargo cycle shard to shard-executor** (found by the registry verifier, invisible to the differential, which mounts every crate in one sandbox): `ShardTransports::SharedThread(semio_framework_plugin_host_shard_executor::SharedThreadTransport)`. Fixed by moving `SharedThreadTransport` into the shard crate and removing the shard to executor manifest edge; census, differential and registry verifier re-run afterwards.
- A manifest scan that read the facade's `[[bin]]` from disk instead of the staged text; fixed.
- Path-relative law inputs (item 6 above) were found by reading every `CARGO_MANIFEST_DIR` and `env!` use, not by any gate.

## 7. Real compile: status

- 06:25 to 06:50: `bun repo run @semio-tech/framework-plugin-host-wit-rs:check --configuration=prototype --parallel=1 -- --lib --offline` (log `🗑️generated/phc-cargo-after-wit.log`). It stayed in launcher graph capture (`build:selected-compile-imports` / `command-source-imports` / `selected-compile-closure`) for 25 minutes and my `alarm 1500` killed it (SIGALRM, exit 142). It never reached cargo; no compiler diagnostic exists.
- 06:50: the canonical command from the brief (`bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p <all 15 packages> --lib --keep-going --message-format=short`, no `--manifest-path` because the root workspace owns these crates; log `🗑️generated/phc-cargo-check-lib.log`) was refused by the physical-headroom gate: `Insufficient physical compiler initial headroom`, available 4,262,920,192 B, required 4,294,967,296 B (32,047,104 B short; the retirement plan had `selected: 0`). This is the same blocker `RESULT-COMPILE-PASS-2.md` §6 records. No gate was bypassed and nothing was deleted to make room.
- My first retry loop (started 06:51, `phc-cargo-lib-try1.log`) predated the coordinator's single-slot rule and called the launcher without the mutex. When the rule arrived I killed that loop (it was sleeping between attempts; one refused attempt, no compile was running) and replaced it with a loop that takes `🗑️generated/cargo-mutex.lock` exactly as written in `📓️WAVE5-BRIEF.md`, makes one batched launcher call for all 15 packages, releases the mutex right after, and on `Insufficient physical compiler initial headroom` sleeps 600 s and retries (up to 6 attempts; logs `🗑️generated/phc-cargo-lib-m<N>.log`, marker `phc-cargo-lib-loop.done`). Attempt 1 (clock 10:55) was refused again: available 4,262,895,616 B, required 4,294,967,296 B, 32,071,680 B short, retirement plan `selected: 0`. The mutex was released. The loop is a background child of this turn and may die with it.
- The earlier probe (`🗑️generated/phc-cargo-before.log`, 05:52) died with `SQLITE_CANTOPEN` in the lease store (infra, `RESULT-INFRA-UNBLOCK-2.md`).

Consequence: no new crate has a compiler verdict. Everything in section 5 is static evidence; the differential and the dependency checks do not see unused-code or type errors that only the full compiler reports (typeck of method calls on moved receivers, field privacy, trait selection across crate boundaries, orphan and coherence rules). The most likely classes of first-compile errors, by construction: a missing `pub` on a field or method that only a different crate now uses; a missing `testkit` feature forward on a dev edge; a trait import lost in a moved file; a coherence conflict from the split `impl` blocks (guest runtime holds the `HostWithStore<ActorHostState>` impl and the `impl GuestRuntimes` for that reason).

## 8. Foreign errors and hand-offs

- Two pre-existing consumer mismatches (section 5, item 8).
- Eleven TypeScript source-text tests under `H` were already failing before the carve (list in `phc-ts-before.json`): `⏱️deadline`, `♻️publication`, `🎟️context/🤝️admission`, `🔔️wake/📃️policy/⚛️actor`, `🔔️wake/📃️policy`, `🔔️wake`, `🧪️tests/♻️relay-lifecycle`, `🧪️tests/🧱️ports`, `🧬️component-codec/🎟️retained-turn/💼️jobs`, `🧵️shard/🎟️grant/🧪️tests/🎟️supplier`, `🧵️shard/🎟️grant/🧪️tests`. They assert text of files that earlier carves already moved. A TS-reader repair slice should own them.
- `🧵️shard/🚚️process-transport/🧪️tests/🔬️unit/🦀️.rs` keeps a law that is `#[ignore]` and env-gated (`SEMIO_SCALE_FIXTURE_WASM`) and reads `CARGO_BIN_EXE_semio-shard`; the `semio-shard` binary belongs to the facade package, so that law can never run from the `shard-laws` crate (it could not run as a lib unit test before either, cargo exports that variable only to integration tests). Left as it was.
- Census rows of the crates that remain `oversizedDeps: 1` change when peers `pi1`/`pwp7` finish carving `semio-framework-plugin`.

## 9. Tools

`phc-tools/` (all Python 3.9 except bun scripts): `plan_host.py` (crate table, structural edits, pass sequence), `carve.py`/`hcmod.py` (copies of the hub engine, patched), `fixers.py`, `autovis.py` (privacy repair from diagnostics, decisions in `vis-decisions.json`), `union_host.py`/`oracle_host.py` (stitched analysis-only rustc differential, `bindgen!` stubbed), `drive_all.py`, `state.py`, `sync_state.py` (pulls post-flush disk edits back into the state), `post_diff.py`, `extern_set.py`, `census_host.py`, `closure.ts`, `regs_host.py`/`finalize_host.py`/`flush_host.py`, `facade_gen.py`, `lawmap.py`, `tsfix_host.py`/`apply_tsfix.py`/`edit_host_script.py`, `taxonomy_host.py`, `verify_registry_host.ts`, `verify_static.py`/`static_all.py`, `checks_host.py`/`checks_post.py`, `bodycheck.py`, `depcheck.py`, `facade_probe.py`/`facade_probe_base.py`, `ts_run.py`, `sensitivity.py`/`sensitivity_post.py`.

## 10. Files changed

62 Rust files under `H` modified or created, 1 deleted (`H/🧪️tests/🔬️standalone/🦀️.rs`; its items live in the owning crates, the definition scan reports no lost item), 14 new `Cargo.toml`, 14 new `📋️project.json`, facade `Cargo.toml` and root; `H/📦️packages/🦀️rust/📜️script.ts`; the two consumer scripts of section 4; root `Cargo.toml`; `📋️build-owners.json`; `.vscode/launch.json`; `.vscode/🧩️launch.seed.jsonc`; the taxonomy file. Exact list: `git status` under `H` plus the files named above.

## 11. Hand-off: how to get the compile verdict

When the gate admits a compile (take the mutex first), one call covers everything of this slice:

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-plugin-host-wit -p semio-framework-plugin-host-engine -p semio-framework-plugin-host-operation -p semio-framework-plugin-host-effects -p semio-framework-plugin-host-imports -p semio-framework-plugin-host-guest-runtime -p semio-framework-plugin-host-wake -p semio-framework-plugin-host-post-turn-relay -p semio-framework-plugin-host-routers -p semio-framework-plugin-host-process-transport -p semio-framework-plugin-host-shard -p semio-framework-plugin-host-shard-laws -p semio-framework-plugin-host-guest-laws -p semio-framework-plugin-host-shard-executor -p semio-framework-plugin-host --lib --keep-going --message-format=short
```

Then the same with `--tests` instead of `--lib` (all crates except `wit`, which has no tests of its own beyond its closure; include it anyway). Dependents of the facade (`semio-framework-os`, hub crates) should be checked in the same wave, bearing in mind the two pre-existing unresolved consumer paths in section 5 item 8. Errors in the new crates are mine to fix (re-run `sync_state.py --apply` after any on-disk edit so the static tools see it).
