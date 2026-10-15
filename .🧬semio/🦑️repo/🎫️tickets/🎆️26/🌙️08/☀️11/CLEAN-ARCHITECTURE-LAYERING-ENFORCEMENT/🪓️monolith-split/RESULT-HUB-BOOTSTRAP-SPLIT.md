# Result: hub bootstrap split (`🌎️hub/🏗️bootstrap/🦀️.rs`)

Executor: Sonnet, foreground, no sub-agents. Status: **split done and statically verified; not compiled** (cargo cannot load the hub workspace right now, see section 6). The route families are **not in the `semio-hub` library**; they sit in 22 child module folders of the bin (section 2 says why).

## 1. Line counts

| File | Before | After |
|---|---|---|
| `🏗️bootstrap/🦀️.rs` (bin entry) | 11,963 | **579** |
| 22 child modules under `🏗️bootstrap/<emoji>️<kebab>/🦀️.rs` | 0 | 12,011 together |
| largest child | | `🛠️admin` 1,268, then `🔌️document-socket` 1,254, `📣️directory-events` 911 |

Every module is far below the 2,500-line ceiling. About 100 of each module's lines are its own `use` preamble.

## 2. Why the route families are bin modules and not lib modules

I moved them out of the entry file, but mounted them from the bin root with `#[path]`, not from `📦️packages/🦀️rust/🦀️.rs`. Three facts make a lib move wrong right now:

1. **`#[cfg(test)]` seams do not cross crates.** The moved code holds about 89 `cfg(test)` items and fields: `HubState` test gates, `TestLiveGate`, `LAST_DOCUMENT_PLAN_REFUSAL`, the document socket loop hooks, and `hub_worker_pool`'s per-state pool. The bin unit tests (`🧪️tests/🔬️bin-unit`, `use super::*`) need them. When the lib is compiled as a dependency of the bin's test build, `cfg(test)` is off, so those seams vanish and the 10k-line suite cannot compile. The only ways around it are a feature that is on for tests (a self dev-dependency or a mandatory `--features`, both infrastructure changes) or shipping the seams in production code.
2. **Parent-owned `📜️script.ts` runs bin tests by name:** lines 2654, 2703 and 3757 call `cargo test --bin os-hub … tests::<name>`, and 2704, 3370 and 3759 call `check --bin os-hub`. Moving the tests into the lib would break those without touching the script.
3. **Visibility.** A lib move needs `pub` on everything the bin and its tests touch, including every `HubState` field and method the tests use. That is a large, unreviewable public surface for no gain.

The modules are written so a later lib relocation is a mount change only: bodies keep `semio_hub::…` paths, cross-module references are `crate::<module>::name`, and nothing relies on the bin root except `main`.

## 3. The new modules

All are `#[path = "<folder>/🦀️.rs"] mod <name>;` in the bin root, all folder emojis are RGI, carry U+FE0F, are unique among siblings (existing siblings: `🎭️actors`, `🧾️provenance`) and are already used as folder prefixes elsewhere in the repo.

| Folder (module) | Contents | Lines |
|---|---|---|
| `🚀️startup` (`startup`) | `HubError`, startup catalog control and progress, CAS maintenance and saga drain supervisors, `StartupArtifactAuthority`, `configured_artifact_authority`, `compose_hub_server`, `instance_state`, `merge_policy_from_env` | 695 |
| `🧰️http-support` (`support`) | `DirectoryJson`, `now_ms`, `bearer`, `db_error_status`, `parse_content_hash`, `directory_error_status` | 74 |
| `👥️presence` (`presence`) | scope keys, presence leases, colour leases | 181 |
| `🧠️state` (`state`) | `HubState`, its test gates and plan-refusal statics | 704 |
| `🎟️socket-grants` (`socket_grants`) | socket grant ledger, binding gates, live-lease authority | 560 |
| `🧭️open-plan` (`open_plan`) | open-plan capability, authority, ledger | 473 |
| `🛡️guard` (`guard`) | `AuthOutcome`, `resolve_auth`, admin subject and principal, `validate_auth_startup`, admin authentication | 283 |
| `🩺️readiness` (`readiness`) | `HubMode`, readiness structs, checkpoint policy, liveness, boot readiness server, `/healthz` `/readyz` | 508 |
| `🚦️transport` (`transport`) | forwarded TLS, refusal and transport middlewares, rate limit, CORS, route metrics | 361 |
| `🔑️access-grants` (`access_grants`) | document and directory socket grants, open-plan routes, execution-target leases; the `include!` lives here | 671 |
| `🪞️pair-blobs` (`pair_blobs`) | canonical checkpoint pair, document status, blobs | 364 |
| `📌️check-in` (`check_in`) | checkpoint publication, GIS map checkpoint publisher, document check-in | 677 |
| `🔌️document-socket` (`document_socket`) | document WebSocket and its wire projection and rebootstrap | 1,254 |
| `📇️directory-commands` (`directory_commands`) | `AuthedUser`, directory views, commands, space administration pages, invite redeem | 877 |
| `📣️directory-events` (`directory_events`) | event pages, event stream, directory WebSockets | 911 |
| `🌱️artifact-creation` (`artifact_creation`) | artifact creation HTTP | 633 |
| `🪪️session-agent` (`session_agent`) | session, credential and agent delegation routes | 714 |
| `🛠️admin` (`admin`) | admin intents, operations, cursors, overview, listings | 1,268 |
| `🧩️assets` (`assets`) | extension assets, trusted plugin modules, administrator page | 171 |
| `💡️inference` (`inference`) | GIS map inference job routes | 199 |
| `🛣️routes` (`routes`) | `router`, `inference_routes`, `artifact_creation_routes` (the only place besides the boot router that calls `.route(`) | 118 |
| `🛑️lifecycle` (`lifecycle`) | `hub_worker_pool`, `connect_db`, `connect_artifact_cas`, `connect_directory`, socket drain, database close, termination signals | 333 |

What stays in the bin entry: crate docs, `extern crate directory as semio_framework_os_kernel` (cfg test; the tests use that alias), `use` preamble, module mounts and glob re-exports, `main`, `serve`, `hub_shutdown_record`, the two CLI command mounts and the three test mounts. `serve()` is unchanged, same text, same order.

`include!` (old line 3956): now `#[cfg(test)] include!("../../🧪️tests/🔬️standalone/🦀️.rs");` inside `🔑️access-grants`. The included fixture functions (`issue_document_socket_grant_fixture`, three `pause_*`, `execute_directory_command_fenced`, `pause_global_directory_send_for_test`) are called from other modules, so they became `pub(crate)` in that test file (`🧪️tests/🔬️standalone/🦀️.rs`, 6 lines). The two `#[path = "../…"]` command mounts keep their text and still resolve (bin root dir unchanged).

## 4. Visibility changes

No `pub` was added anywhere (everything stays in the bin crate). Top-level items referenced from another module, the root or any test file became `pub(crate)`: 839 `pub(crate)` tokens in total, computed from first-path-segment identifier sets of all modules, the root, the two command files and the six test files. Struct fields and inherent methods got `pub(crate)` only when their name appears in another module, and types named in a widened item's signature were widened too (closure). Trait-impl methods were not touched. The root re-exports every module with `use <module>::*;` (cfg(test) when only tests need it), so `use super::*` in `🔬️bin-unit`, `🤝️two-client-document`, the command modules' `use super::HubError` and `super::connect_directory` keep working. Imports are per module: only leaves whose binding is used, plus trait leaves whose method names are called, plus `crate::<module>::name` leaves for cross-module names, with `cfg` attributes copied from the defining item.

## 5. Test updates

- `🧪️tests/🔬️bin-unit/🦀️.rs` read the whole bootstrap file in 4 places. New truth:
  - line 223 (`readiness_publishes_each_hub_executed_inference_service…`): reads `🏗️bootstrap/🛣️routes/🦀️.rs`; the GIS route string `…/{document_id}/inference/gis-map/jobs` lives in `inference_routes` (1 occurrence).
  - line 1685 (`the_hostile_input_fixture_covers_every_registered_route`): scans `.route(` over `🛣️routes` **and** `🩺️readiness` (the boot router's `/healthz` and `/readyz` registrations; the original file had 64 occurrences, 62 + 2). Same assertion, same `>= 60` bound, so no route can ship without hostile vectors and none is skipped.
  - line 2117 (`a_committed_gis_map_approval_is_answered_with_its_receipt`): reads `💡️inference`, where both approval handlers are.
  - line 4652: `assert_eq!(…matches("ActorId(HUB_CATCH_UP_ORIGIN.into())").count(), 2, …)`. It counts the two catch-up sites (the hello tail and the FrontierAdvertise catch-up). Both are in `serve_document_socket`, so it now reads `🏗️bootstrap/🔌️document-socket/🦀️.rs`. Verified: exactly 2 there, 0 in every other file. The count and the message are unchanged.
- **Pre-existing break fixed:** lines 1, 1164 and 9529 used `crate::artifact_authority::creation::io::…`, which does not exist in the bin crate (introduced by an earlier sweep, commit `da607013f32` era 2026-10-09; the name-resolution oracle shows it as an error on the untouched original). Changed to `semio_hub::artifact_authority::creation::io::…`.
- Also updated: `🌎️hub/📦️packages/🦀️rust/📋️project.json` (nx input `🏗️bootstrap/🦀️.rs` became `🏗️bootstrap/**/🦀️.rs`, otherwise edits to the new modules would not invalidate the cache) and the GIS MCP oracle `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🌉️mcp/🧪️tests/🔬️oracle/🟦️.ts` (reads the routes module for the registered inference routes).

### Hazard for the parent (not touched, as instructed)

`🌎️hub/📦️packages/🦀️rust/📜️script.ts` reads the bootstrap file as one text in 16 gates and will fail on any split: lines 3012, 3280, 3425, 3470, 4144, 5468, 8537, 13405, 13894, 15413, 15978, 16165, 16327, 16393, 16674, 17022. The simplest fix is one helper that concatenates `🏗️bootstrap/🦀️.rs` and `🏗️bootstrap/*/🦀️.rs`. `🏗️bootstrap/🦀️.rs` still contains the `#[path = "../🧪️tests/🔬️bin-unit/🦀️.rs"]` string that line 3425 checks. Marker-to-module hints per site: `hubsplit-tools/script-ts-readers.txt`.

## 6. Verification actually performed

| Check | Result |
|---|---|
| `bun repo cargo --store=shared --configuration=prototype -- check -p semio-hub --bins --lib --message-format=short` (as given) | **Blocked.** `error: package ID specification 'semio-hub' did not match any packages`: the hub is an excluded standalone workspace, so the root manifest cannot select it. |
| same with `-- check --manifest-path 🌎️hub/Cargo.toml -p semio-hub --bins --lib …` (retry loop, ~40 min, last attempt just before this report) | **Blocked before rustc.** First `cargo fetch --locked` failed on a stale root `Cargo.lock` (peer edit), then a bun module missing in the repo tool (peer move), then permanently `no matching package named semio-framework-3d-brep-operations` at `🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/📦️packages/🦀️rust` required by `semio-s-artifact-procedural-generation3d` (the manifest exists on disk now, the lock does not resolve). The compiler-ingress source-boundary refusal was therefore not even reached. Logs: `🗑️generated/hubsplit-baseline.log`, `hubsplit-final-check.log`. |
| `--tests` check and `test -p semio-hub --bins --lib` | **Not run**, same reason. No baseline was obtainable; no test result is claimed. |
| Name-resolution differential: `rustc +nightly-2026-07-20 -Zunpretty=expanded --crate-type bin` on the original (copy plus symlinked siblings) vs the split tree, in 4 cfg sets: plain, `--cfg test`, `--cfg feature="integration-fixtures"`, `--cfg test` plus that feature | The sorted multiset of distinct `error`/`warning` header lines is identical in all 4 (only counts differ: every module repeats the extern-crate failures, 742 vs 1044 in plain). So there is no new `cannot find`, `private`, `ambiguous` or `defined multiple times` error; unused-import warnings from the oracle were pruned in an iterate-until-stable loop (`prune.json`). The tests build also shows the 3 `crate::artifact_authority` errors gone. Limits: this phase does not do type checking, so it cannot see private fields/methods, trait-in-scope for method calls, or `private_interfaces`. |
| Lossless line multiset (`hubsplit-tools/verify.py`, `pub(crate)` and moved `include!` path normalised, `use` preamble excluded) | 0 original code lines missing (24 missing lines are continuation lines of the rewritten multi-line `use` blocks), 0 extra lines beyond headers, mounts and imports. |
| `rustfmt --edition 2021 --emit stdout` on the root, the 22 children and the standalone file | all parse. `🔬️bin-unit/🦀️.rs` fails at line 1305 (`<… as A + B>`), identically on the HEAD version, so not mine. |
| Path literals (`#[path]`, `include!`, `include_str!`) in root and children | 29 checked, 0 missing. |
| Emoji admission (`\p{RGI_Emoji}` with VS16, uniqueness among siblings) | 24/24 folders ok. |
| `[DEBUG]` logs | none added. |

### Residual risks (need the first real compile)

1. **Trait imports for method calls** are kept by method-name matching against `.name(`, which over-keeps (axum's `.get(` matches `PayloadStorage::get`), so expect some unused-import warnings in e.g. `🛣️routes`; under-keeping is possible only if a trait method is invoked through a path that is not a `.call`.
2. **Typeck privacy:** member widening is name-based and conservative, but unverified by rustc (E0616/E0624 would show as missing `pub(crate)`).
3. The tests' `use super::*` glob chain: names reach the tests through root globs; no ambiguity was reported by the oracle in test cfg.
4. Unused `crate::…` imports gated `#[cfg(test)]` by the oracle (3 in `🧠️state`, 8 in `🔑️access-grants`) rely on non-test code not using them.

## 7. Domain leaks recorded (relocation out of hub is a follow-up)

- **GIS domain:** `💡️inference` (all job and approval routes, `HubGisMapApprovalIngressAuthorityV1`, `acquire_gis_map_approval_ingress`), `🛣️routes::inference_routes`, `🔌️check-in` (`GisMapApproval*Publisher*`, `publish_gis_map_checkpoint_change`), `🚀️startup` and `🧠️state` (`verified_gis_map_binding`, `gis_map_binding`, `inference_runtime`), `🩺️readiness` (`inferenceServices`).
- **DB drivers (sqlite/postgres/neo4j/fs):** `🛑️lifecycle` (`connect_db`, `connect_artifact_cas`, `connect_directory`) and the chunk-CAS storage imports in `🚀️startup`.
- **os-kernel imports (extern crate `directory`, `db`, `server`, `protocol`):** 106 `use` lines of those crates across the root and all modules; heaviest: bin root 14, `🧠️state` 9, `🔌️document-socket`, `📣️directory-events`, `📌️check-in` 7 each. Grouping is by module, so a later move of the HTTP layer pulls one module at a time.

## 8. Reproduction and tooling

`🪓️monolith-split/hubsplit-tools/`: `rsitems.py` (lexer and item parser), `spec.py` (module and range table), `split.py` (generator), `apply.py`, `oraclefix.py` and `loop.sh` (rustc oracle prune loop), `verify.py`, `cargo-retry.sh`, `orig-bootstrap.rs.txt` (untouched original, sha256 `746beedb…61fe`), `prune.json`, `traits.json`. Scratch oracle runs: `scratchpad/hubo/run.sh <label> [--cfg …]`. Once a compiler admits the crate: run `check --manifest-path 🌎️hub/Cargo.toml -p semio-hub --bins --lib`, then `--tests`, then `test -p semio-hub --bins --lib`; fix privacy errors (E0603/E0616/E0624) by adding `pub(crate)` at the definition and delete unused imports.

## Hardening (full-resolution differential and regions, 2026-10-15)

Full configs, baseline construction, scan tool and residual risks: `RESULT-HARDEN-MANIFEST-PLUGIN-HUB.md`.

- Oracle: bin crate `os_hub` (`--crate-type bin`) from the real root `🌎️hub/🏗️bootstrap/🦀️.rs`, baseline = sandbox with the root replaced by `hubsplit-tools/orig-bootstrap.rs.txt` (sha256 `746beedb...`, equals HEAD); configs native, `--test`, `--cfg 'feature="integration-fixtures"'`, and both. The bin is a separate crate from `semio_hub`, so every `semio_hub::...` is unresolved in both trees and all widenings are `pub(crate)` inside the bin. Errors about external crates are filtered (they repeat once per module by construction).
- Before fixes (test configs): 8 new errors, 5 distinct: `db_artifact_id` x2 and `document_scope_key_v1` (E0425, `🧪️tests/🔬️standalone`, included from `🔑️access-grants`), `DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVING` x2 (E0425, `🔬️bin-unit`), `HUB_SHUTDOWN_CLOSE_CODE` (E0425, `🤝️two-client-document`), `BTreeMap` x2 (E0433, `🔬️bin-unit`). Native and feature-only: 0.
- Fixes: `🔑️access-grants/🦀️.rs:30` `#[cfg(test)] use crate::presence::{db_artifact_id, document_scope_key_v1};`; `🧭️open-plan/🦀️.rs:87` `pub(crate) static` (inside `thread_local!`); `🛑️lifecycle/🦀️.rs:230` `pub(crate) const HUB_SHUTDOWN_CLOSE_CODE`; `🦀️.rs:115` `#[cfg(test)] use std::collections::BTreeMap;`; from the definition scan `🧠️state/🦀️.rs:298` `pub(crate) admin_dir` (used by `🧩️assets`, the root's `HubState { .. }` literal and `🔬️bin-unit`; E0616/E0451, invisible to the oracle).
- Final non-extern counts, baseline / split, 0 differing keys: native 4 / 4, `--test` 10 / 10, `integration-fixtures` 4 / 4, `--test` + `integration-fixtures` 11 / 11. Coordinator patches (strip `extern crate`, neutralise external globs) applied identically: 1 `extern crate` line, 0 external globs in the bin tree. The gated root imports are test-only: the non-test configs (native, `integration-fixtures`) show 0 name differences, and the oracle does flag a cfg-gated import used by non-test code.
- Definition scan: 142 private defs, 1154 field-name hits; 1 real (`admin_dir`, fixed); the rest are foreign-type fields or locals (22 receiver-unprovable groups read by hand). Item scan and trait-scope scan: 0 (no module calls `PayloadStorage`, `SessionStore`, `StreamExt`, `SinkExt` or `IntoResponse` methods without importing the trait).
- **Regions restored (all 36 original `//#region` / `//#endregion` markers accounted for).** The tree now holds 80 marker lines = 40 balanced pairs in 23 files; every one of the 18 original regions has the identical multiset of enclosed lines as in the original (State x7 files, Directory x8, Rest x3, AdminAuth x3, Main x3, AdminPage x2, Admin x2, Inference x2, one pair each for the other 10), except the `include!` path-depth line already documented in section 3. The parent keeps `Main` and `Tests`. Placement tool: scratchpad `hard/hubregions.py`, `hubplan.py`, `hubapply.py`, `hubverify.py` (item-accurate, uses `hubsplit-tools/rsitems.py`). rustfmt parses all 23 files.
- Residual: typeck-phase privacy and trait scope for extern traits unproven by rustc.
