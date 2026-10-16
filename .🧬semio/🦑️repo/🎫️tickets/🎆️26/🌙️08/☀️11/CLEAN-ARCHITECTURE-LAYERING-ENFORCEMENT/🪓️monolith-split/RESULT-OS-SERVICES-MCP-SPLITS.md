# 🛎️ Result: OS-SERVICES-MCP-SPLITS (prefix `osm`, WP-A of `WAVE7-CANDIDATES.md`)

Executor: Sonnet 5.5, foreground, no sub-agents, no ticket tool, no modifying git command.
`F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`, `M` = `🧰️framework/🛍️products/💻️os/🔨️modules`.
Tools: `F/osm-tools/` (`osmeng.py` split engine, `plans.py` the five split plans, `run.py`, `oracle.py`, `verify.py`, `lintprune.py`, `suspect.py`, `census.ts`, reader checks). Logs and sandboxes: `F/🗑️generated/osm-*` (nothing deleted there).

## 0. Verdict

All five targets are split. No real `cargo check` could run (raw cargo refused; the source-boundary gate refuses the crates and the compile mutex was not taken), so the proof is static: lossless line multiset, scoped patched full-resolution differential in six configurations (identical), rustfmt parse, path and duplicate-mount scans, region balance, public-surface identity, source-text readers re-pointed and exercised, taxonomy resolution. Real-check commands for the compile queue are in section 7.

| Target | Before | After |
| --- | ---: | --- |
| `M/🛎️services/🦀️.rs` (facade plus logic) | 2,666 | 105-line facade (docs, `#[path]` mods, explicit `pub use`) plus 10 children (50 to 397 lines) |
| `M/🛎️services/🧪️tests/🔬️component-unit/🦀️.rs` | 1,561 | 16-line facade plus 7 children (24 to 659 lines) |
| `M/🌉️mcp/🚚️transport/🦀️.rs` | 1,955 | 75-line parent plus 6 children (150 to 845 lines) |
| `M/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs` | 1,922 | 74-line parent plus 7 children (77 to 607 lines) |
| `M/🌉️mcp/🧵️bridge/🦀️.rs` | 1,749 | 49-line parent plus 6 children (134 to 527 lines) |

Largest child anywhere: 845 lines (`🚚️transport/🔁️http-transport-state`). Longest path: 153 UTF-8 bytes (limit 240).

## 1. Found state and baseline

No earlier osm work existed. Before the first edit I snapshotted `M/🛎️services` (18 files) and `M/🌉️mcp` (254 files) into `F/🗑️generated/osm-work/snap-*` and recorded the sha256 of the five files (`snap-hashes.json`). Every apply refuses to write if a live file differs from the last state I know (`applied-hashes.json`); no peer touched the five files during the work. Peers edit other files of the `🌉️mcp` tree at the same time (plugin-runtime, foundation, workspace tests); I did not touch them. A peer left `🏠️workspace/🔗️remote/🦀️.rs.orig` and `🧪️tests/🔬️unit/🦀️.rs.orig`; they are not mounted and I left them.

Baseline oracle (original tree, all six configs) is in `F/🗑️generated/osm-oracle-base0-*` (the only errors are the sandbox's unresolved external crates, plus 17 to 18 unresolved-name errors in the unrelated `🚪️native-io` file of the services tree, identical before and after).

## 2. What was cut

Method: scripted cuts from 1-based line ranges with a coverage check (every non-blank, non-marker, non-import line is claimed exactly once), region nesting repaired at every cut (a region that spans children is repeated, balanced, in each, as in `RESULT-REGION-RESTORE.md`), each child starts with an emoji `//!` docstring, folders are `<emoji>️<kebab>` with an emoji used elsewhere in the repo and unique among the siblings.

### 2.1 Services root (`M/🛎️services`)

The old root held all logic (crate glue `📦️packages/🦀️rust/🦀️.rs` does `mod component; pub use component::*`). Now `🦀️.rs` keeps only the crate docs, `#[path]` mods and explicit `pub use` lines (plus the unchanged `🧪️tests` mount and the `original_compute_parameters` mount with its `pub use`, which stay in the root so the module-tree readers still follow `🧮️compute/…`).

| Child (module) | Lines | Content (original lines) |
| --- | ---: | --- |
| `🚂️host-runtime` (`host_runtime`) | 395 | GlobalWorkerPool and TokioHostRuntime regions: pool handle, `PoolFutureTask`, `ScopeTable`, `TokioHostRuntime` (49-418) |
| `⏲️timer-wheel` (`timer_wheel`) | 236 | TimerWheel region (420-645) |
| `🧰️compute-pool` (`compute_pool`) | 219 | ComputePool region without the params mount (647-672, 676-848) |
| `🌐️http-contract` (`http_contract`) | 342 | request and response vocabulary, body cancellation, terminal custody, blocking transport (850-1182) |
| `🔌️http-socket` (`http_socket`) | 338 | socket transport (1184-1500) |
| `🪣️http-pool` (`http_pool`) | 397 | token bucket, outstanding credit, `HttpPool`, `HttpPoolBody` (1502-1876) |
| `📄️fixed-file-storage` (`fixed_file_storage`) | 266 | FixedFilePage and RetainedFixedFileDocument regions (1878-2137) |
| `💾️storage-scheduler` (`storage_scheduler`) | 251 | StorageScheduler region (2139-2377) |
| `📮️event-router` (`event_router`) | 242 | EventRouter region (2379-2613) |
| `🧾️completion-sink` (`completion_sink`) | 50 | CompletionSink region (2615-2660) |

The public surface is identical (50 `pub` names before and after, none lost or gained; checked from the outlines of the snapshot and the live root).

Tests (`🧪️tests/🔬️component-unit`): `🧷️support`, `🚂️host-runtime` (runtime and scope table), `🧰️compute-pool` (compute plus the original admission law), `⏲️timers`, `💾️storage`, `📮️events` (router and sink), `🌐️http` (pool, socket). I first cut 14 test children and merged them down to 7 after the census showed the services lib+test closure would pass 32 files (it is 27 now). The `include_str!` paths moved one level deeper and were re-based (`../` prefix); all three targets were checked to exist.

### 2.2 MCP files

| File | Children |
| --- | --- |
| `🚚️transport/🦀️.rs` (crate `semio-framework-os-mcp-http-transport`) | `🧾️http-surface` 421, `🔁️http-transport-state` 845, `🪪️http-authority` 150, `📬️http-dispatch` 197, `🔌️websocket-codec` 216, `🛡️origin-and-handlers` 168 |
| `🏠️workspace/🔗️remote/🦀️.rs` (crate `…-hub-remote`) | `📏️binding-limits` 116, `🗂️authorized-views` 77, `🛰️remote-binding` 607, `🛠️binding-helpers` 319, `📦️component-assets` 246, `🧲️native-driver` 443, `💡️inference` 225 (keeps the Inference region with both of its impl blocks) |
| `🧵️bridge/🦀️.rs` (crate `…-bridge-codec`) | `📏️wire` 134 (the inline `mod wire { … }` promoted to a file, same module name), `🧱️shared-types` 172, `📤️shell-to-gateway` 135, `🔍️shell-decode-cursor` 450, `🧬️shell-materialize-cursor` 527, `📥️gateway-to-shell` 321; the parent keeps `BRIDGE_VERSION` and the empty BridgeServer and Tests regions |

Public names: transport 10 `pub` + 37 `pub(crate)`, remote 47 + 11, bridge 21 + 1: all `pub` names unchanged. The `pub(crate)` re-exports in the parents are exact: the 46 that only the `cfg(test)` modules of the crate use got `#[cfg(test)]` (35 transport, 11 remote); the one that nothing uses (`ValidatedShellFrame`, checked with `git grep` over the whole `🌉️mcp` tree) was dropped. No other `pub(crate)` name changed.

### 2.3 Visibility rule

A private item, field or inherent method is widened to `pub(super)` only when its name appears in code of another child, the parent or the test children. `pub(super)` is exactly the scope the item had in the unsplit module, so nothing becomes wider than it was. The rule is receiver-agnostic (a name match, not type-aware), so it can only over-widen, never under-widen. Counts (items/fields/methods): services 9/78/21, services tests 4/16/0 (the 3 shared helpers of `🧷️support` among the items), transport 8/14/9, remote 7/3/3, bridge 1/1/19. Tuple structs are widened only when a constructor call or a `.N` access appears in another child. Free `struct` and `enum` definitions that moved into a child are widened by the same item rule (this covers the "splitter blind spot" of the oracle note).

### 2.4 Imports

Each child gets its own precise imports instead of `use super::*`: the original private `use` lines were removed and regenerated per child from the identifiers the child really uses (scope identifiers only: not after `.` or `::`, not a field or binding name), including `cfg` attributes of the original imports, `use super::<sibling>::{…}` for names defined in a sibling, and `use super::super::<service child>::{…}` in the test children. Method-only trait imports (`HostAsyncRuntime`, `ToValue`, `FromValue`, `Write`, `NotificationSink`, local traits such as `HttpBody` and `CompletionSink`) are added when a method of the trait is called in the child; the method tables for external traits were read from their definitions (`plans.py`). Each trigger was reviewed by hand against its call sites (false triggers such as `pool.now_ms()` on `WorkerPool` and `handle.complete()` are suppressed with `no_trait`). Two imports that a name collision produced (`wall_now_ms` parameter, `ApprovalDecision` enum variant) were dropped; the lint also suggested dropping `CANONICAL_CHECKPOINT_RESOURCE_SCHEMA`, which I kept because it is used inside an unresolved external macro (the sandbox cannot see it).

## 3. Source-text readers, taxonomy, registries

Edited (all small, no shims):
- `M/…/📖️source-access/🟦️.ts` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access`): five new entries in `POLICY_SPLIT_SOURCE_ROOTS` (services root, services tests root, transport, remote, bridge). This re-points the interactivity audit constants `INTERACTIVITY_AUDIT_OS_SERVICES_FILE`, `…_MCP_HTTP_TRANSPORT_FILE`, `…_MCP_BRIDGE_FILE`, which now read the whole module tree through `policyReadRustPolicySource`.
- `…/📖️source-access/🦀️rust-evidence/🟦️.ts`: the declared canonical test file is read as a module tree when it is a split root (so the test evidence keeps the whole test text; `fixed_file_page_exact_max_plus_one_matches_system_oracle_and_preserves_last_valid_page` is still found).
- `M/🌉️mcp/📦️packages/🦀️rust/📜️script.ts`: two `remote` reads now use `readModuleTreeSource` (new import).
- `🌎️hub/🔐️auth/🔌️client/🧪️tests/🧬️source/🟦️.ts`, `🌎️hub/🔐️auth/🧪️tests/🧭️credential-source-order/🟦️.ts`, `🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts`: the `remote` text now comes from `readModuleTreeSource` (new import each).
- `M/🏪️store/🧩️composition/📨️emission/📦️owned/🧪️tests/🟦️.ts`: the services pool text now comes from `readModuleTreeSource`.
- Readers that already walk directories (`readMcpGatewaySource`) need nothing. `🌉️mcp/🧪️tests/🧹️hygiene/🟦️.ts` only mentions the transport path in a comment.

Taxonomy: no change needed. All 36 new folders resolve with the existing rows (`crate-module-children`, `members-of-tests`, `members-of-modules`). Check 1: the real `loadTaxonomy` and `semanticDirectoryKindId` along the parent chain (`osm-tools/taxonomy_check.ts`), run on the 29 production folders (final names) plus the 14 test folder names of my first cut and 3 existing siblings: 46 of 46 resolved. Check 2, for the 7 final test folder names (`🧷️support`, `🚂️host-runtime`, `🧰️compute-pool`, `⏲️timers`, `💾️storage`, `📮️events`, `🌐️http`): `semanticDirectoryMemberAdmitsName` against the `members-of-tests` row of the HEAD taxonomy (`osm-tools/taxonomy_names_check.ts`): 7 of 7 admitted. The final real-loader rerun was impossible because a peer's in-flight edit of `📚️library/🔣️taxonomy.json` made `loadTaxonomy` throw (`generatorContracts["jco-package-adapter"].inputPatterns must be unique and lexically ordered`); that edit is not mine.

`.vscode/launch.json` and `🧩️launch.seed.jsonc`: no new executable command exists (no new script, test entry or crate), so nothing was registered. Cargo manifests, owners and locks: unchanged (no new crate).

## 4. Verification (commands and real results)

All oracle runs: `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --emit=metadata -o /dev/null`, from the real lib root of each crate, baseline snapshot versus live tree, with the two sandbox patches of `📓️RESOLUTION-ORACLE.md` (external glob imports and missing `extern crate` neutralised). Errors about unresolved external crates are removed from both sides before comparing (the comparison key is error code, message and primary label); their raw counts are in the logs.

1. Scoped patched differential on the live tree: `python3 osm-tools/oracle.py final all lib,test,wasm,wasmtest,p2,p2test` (services rerun as `final2` after a sandbox-only fixture copy of mine was removed): **IDENTICAL in 24 of 24 crate/config pairs** (services lib/test/wasm/wasmtest/p2/p2test 17/18/17/18/17/18 baseline local errors each side; transport 0/0/0/0/0/0; remote 0/2/0/8/0/8; bridge 16 each). Logs: `osm-oracle-final-*`, `osm-oracle-final2-*`, summary json.
2. Negative controls (the oracle is sensitive): deleting one sibling import in `🧰️compute-pool` gave `+1 E0425 cannot find function global_worker_pool`; reverting one `pub(super)` gave `+2 E0603 function global_worker_pool is private` (`osm-oracle-ctl1/ctl2-*`).
3. Unused-import pass (`OSM_LINTS=1 oracle.py`, then `lintprune.py` over lib, test, and for remote and bridge also wasm and wasmtest): drove `osm-tools/overrides.json` (35+11 `cfg(test)` exports, 3 `cfg(test)` imports, 3 drops). The wasm-only unused warnings of `remote` are the same property the unsplit file had (native-only code under `cfg(not(wasm32))`). Limit stated honestly: imports of unresolved external crates are never flagged by rustc in this sandbox, so unused external imports are judged only by the identifier scan; `suspect.py` lists every import whose every occurrence looks like a binding (3 hits, all false positives of the heuristic: `Context`, `ComputeJobCompletion` after `&mut`, `ComputeJobParamsPreparation` as a parameter type).
4. Lossless line multiset (`verify.py live`): non-blank, non-marker lines of the original versus all new files of that split. After normalising the only edits (`pub(super) ` insertions, `../` re-basing), the removed lines are exactly the old import lines (services 6 multi-name import lines, tests 3, transport 25, remote 33, bridge 9: the 4 `use` lines of the old private imports plus `mod wire {`, its closing `}` and its 3 doc lines turned into the child docstring). Added lines are the new docstrings, imports, `mod` lines and re-exports. Lists: `osm-verify-<label>-removed.txt` and `…-added.txt`.
5. rustfmt (`rustfmt +nightly-2026-07-20 --edition 2021 --emit stdout --config-path rustfmt.toml` per file): 0 parse failures in 41 files (11 services, 8 tests, 7 transport, 8 remote, 7 bridge, parents included).
6. Paths: all `#[path]` literals resolve (services 24 mounts, mcp tree 130; missing 0; the baseline had 1 missing mount that a peer has since fixed). Longest new path 153 bytes. Emoji folders: all unique among siblings, all used elsewhere in the repo (counts in `verify.py` output).
7. Duplicate-mount scan over `M/🛎️services` and `M/🌉️mcp`: 0 duplicated mount targets, 0 new.
8. Regions: every new file balanced; every original region name still present (13/13 services, 11/11 tests, 3/3 transport, 2/2 remote, 8/8 bridge); the region copies per split are the repeated, balanced pairs of spanning regions.
9. Public surface: compared per root (see 2.1 and 2.2): 50/10/47/21 `pub` names unchanged; so no consumer needed a re-point (the glue `pub use component::*` and the facade `pub use` of `semio-framework-os-mcp` see the same names).
10. Source readers: `bun osm-tools/readers_check.ts` (policy reader and `readModuleTreeSource` return the whole tree: services 251,263 chars with tests, transport 95,419, bridge 81,428, remote 121,000; all needles found); `bun osm-tools/audit_check.ts`: `interactivityMcpHttpTransportFailures` returns the **same failure list before and after** (both lists carry the same pre-existing failures); `bun osm-tools/hub_reader_check.ts`: `mcpCredentialSourceOrderConforms` is `true` with the original remote text and with the module-tree text; `bun osm-tools/services_text_check.ts`: the FixedFilePage slice is intact (2,481 UTF-16 units against 2,477 code points). `bun build --no-bundle` parses all 7 edited TS files.
11. `bun test ./🟦️.ts` in `M/🏪️store/🧩️composition/📨️emission/📦️owned/🧪️tests`: 14 pass, 11 fail. The edited test (`Services original backing and five-axis retirement…`) fails at the same assertion as before the split (`schedule_rejected_compute_job` is not in the original root either: 0 matches in the snapshot), so the failure set is unchanged. I did not run the hub `bun test` files (the W4 note records a Bun segfault on them); their reader change was exercised through step 10.
12. Source-boundary census of the lib and lib+test closures (`bun osm-tools/census.ts [snap]`, counter `readRustSourceClosureV1`, host cfg):

| Crate | Closure | Files before | Files after | Active lines before | after | Bytes before | after |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `semio-framework-os-services` | lib | 5 | 15 | 3,474 | 3,659 | 192,166 | 199,306 |
| same | lib + test | 10 | 27 | 5,314 | 5,591 | 306,039 | 318,145 |
| `…-mcp-http-transport` | lib | 2 | 8 | 1,862 | 1,958 | 91,498 | 96,346 |
| same | lib + test | 6 | 12 | 3,322 | 3,445 | 162,293 | 167,141 |
| `…-mcp-hub-remote` | lib | 3 | 10 | 2,146 | 2,324 | 127,976 | 135,682 |
| same | lib + test | 5 | 12 | 3,571 | 3,761 | 215,537 | 223,243 |
| `…-mcp-bridge-codec` | lib / lib + test | 2 | 8 | 1,755 | 1,800 | 79,741 | 81,902 |

   Every lib closure and every mcp test closure stays inside 4000 lines, 32 files and 262,144 bytes. The services test closure was already over lines and bytes before the split (5,314 and 306,039) and still is; the split did not add a file-count violation (27 of 32). The per-crate limits come from the manifests (`rust-source-boundary` in the mcp crates equals the hard limits).

## 5. Compile queue (not run: gate refuses, no mutex taken)

Canonical commands (WAVE5 brief), one launcher call at a time under the fleet mutex (`mkdir 🗑️generated/cargo-mutex.lock`, release after):

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-os-services -p semio-framework-os-mcp-http-transport -p semio-framework-os-mcp-hub-remote -p semio-framework-os-mcp-bridge-codec --lib --keep-going --message-format=short
```

Same call with `--tests` instead of `--lib`. Wasm (the two crates that build for it):

```
… -- check -p semio-framework-os-mcp-hub-remote -p semio-framework-os-mcp-bridge-codec --lib --target wasm32-unknown-unknown --keep-going --message-format=short
```

Direct dependents (public names unchanged, so these should be clean): `-p semio-framework-os-mcp -p semio-framework-os-mcp-workspace -p semio-framework-os-mcp-routing -p semio-framework-os-mcp-bridge -p semio-framework-os-mcp-consent -p semio-framework-os-mcp-server -p semio-framework-os-mcp-shell-channel -p semio-framework-os-mcp-tools-gateway -p semio-framework-os-mcp-hub-pair -p semio-framework-os-directory-client -p semio-framework-os-store-sync-native -p semio-framework-os-kernel -p semio-framework-plugin-host-effects -p semio-framework-plugin-host-imports -p semio-framework-plugin-host-shard` (`semio-framework-os-directory-client` and `semio-framework-os-kernel` take services through an optional feature, so add `--features sync` for the directory client). Run tests: `test -p semio-framework-os-services --lib` and `test -p semio-framework-os-mcp-http-transport -p semio-framework-os-mcp-hub-remote -p semio-framework-os-mcp-bridge-codec --lib`.

What the first real compile is most likely to expose (the oracle cannot see these because the sandbox lacks the external crates, so typeck never runs):
1. Field and method privacy (E0616, E0624) across children: covered by the receiver-agnostic widening, but unproven by a compiler.
2. A method-only external trait import that the call-site tables missed (E0599).
3. `unused_imports` warnings for external imports the identifier scan kept (warnings only; the workspace lints do not deny them).
4. `private_interfaces` warnings for `pub(super)` types in `pub` signatures (same shape as before the split).

## 6. Foreign findings and hand-offs

- Pre-existing, unchanged: 11 failing tests in the store emission test file (including the services one); `[DEBUG]` lines inside the original services tests (`🧰️compute-pool` test child line ~225 and `🔬️native-io-unit`), `🧮️compute/…/🧪️tests/🦀️.rs`; they are not mine and I did not touch them. Stale line-number comments elsewhere (`🔌️plugin/🖥️host/⚡️effects/🦀️.rs:330` cites `🛎️services/🦀️.rs:1539`) are left to their owners.
- `semio-framework-os-kernel` is still over the source gate (peer), which is why no compile can run.
- The `.rs.orig` files in the remote folders are a peer's leftovers.
- Hand-off to the compile queue: section 5. Hand-off for a later carve: each services child is a self-contained module with its own imports, so a crate carve can lift `🚂️host-runtime`, `⏲️timer-wheel`, `🧰️compute-pool`, the three HTTP children, the storage children, `📮️event-router` and `🧾️completion-sink` into crates in dependency order (`completion-sink` and `host-runtime` at the bottom). The services test closure still needs that carve to pass the gate.

## 7. Files changed

New (36 folders, each with a `🦀️.rs`): 10 + 7 under `M/🛎️services` (production and `🧪️tests/🔬️component-unit`), 6 under `🚚️transport`, 7 under `🏠️workspace/🔗️remote`, 6 under `🧵️bridge`. Rewritten: the five roots listed in section 0. Edited: the 7 TS readers of section 3. Tools and plans: `F/osm-tools/`. This file.
