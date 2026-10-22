# Result rust-health (executor rust-health, wave 1022a)

Commands: `nice -n 10 bun repo cargo --store=shared --configuration=prototype -- check --workspace --all-targets --keep-going --message-format=short` (root workspace, 567 members) plus `--manifest-path` for the other workspaces. One cargo at a time, detached with `C/chain-tools/detach.py`, retry wrapper `C/rust-health-tools/run.sh` (retries lease/loader refusals), summarizers `sum.py|show.py|files.py|missing.py` (per-crate error tables), `adddep.py` (path dependency adder), `closure.mjs` (path-dependency closure). Logs: `C/🗑️generated/rust-health-logs/ws*.log` (ws20 is the latest root run). Each run costs 25 to 60 min (source admission 1,527 units + shared capacity queue).

## Status by workspace
| workspace | check --all-targets | note |
|---|---|---|
| `🎓️teaching/Cargo.toml` (course proctor, 3 crates; closure 113 crates incl. 110 framework crates) | GREEN, rc 0 | `x1-teaching.log` |
| root `Cargo.toml` (567 members) | RED, ws20: 17 failing units, 15 libs (table below); everything else compiles | |
| `🧰️framework/Cargo.toml` (411 members) | NOT VERIFIED (infra refusals) | needs `--exclude semio-framework-async` (source gate 4,092/4,000 active lines with test targets), lock refresh; queued in `chain9` as `ws20-fw.log` |
| `🌎️hub/Cargo.toml` demonstrator component | NOT VERIFIED | lock refresh then check queued in `chain9` (`ws20-hub.log`) |
| `✏️s` plugin workspaces | not mine (apps-a/apps-b); they report only `plugin-window-config` and `plugin-operation` left | handed to apps-b |

## Root workspace, units still red in ws20 (libs)
| crate | errors | cause / owner |
|---|---|---|
| semio-framework-plugin-window-config | 12 | handed to apps-b (WindowConfigSnapshot.authority, SharedUtf8, WindowConfigStore, re-export visibility) |
| semio-framework-plugin-operation | 1 | `install_document_store_owners_exact` bounds; handed to apps-b |
| semio-framework-plugin-guest-contract | 6 (+10 tests) | tests mounted in the lib use pre-696 `RetainedTurnInput`/`Box<DslValue>` API |
| semio-framework-plugin-host-effects | 5 | router effect job still on the old compute-pool API (`prepare_job_params`, `close_step`) |
| semio-framework-os-infinite-board-engine | 1 | E0117 foreign-trait impl |
| semio-framework-os-config-merge-policy | 6 | Mutations source-authority gate: aggregate file sits below the leaf folder |
| semio-framework-os-kernel-db-io-registry | 2 | fixed after ws20 (custody fields pub), unverified |
| semio-framework-os-kernel-db-actor | 1 | `emit` on `Arc<NullEmit>` |
| semio-framework-os-mcp-port / -probe-document / -hub-remote | 1 / 1 / 4 | monolith `store::os_store::*`, `store::pack`, `inference` module paths |
| semio-framework-repo-tickets / -hooks | 1 / 1 | `decode_agent`, `get_editor_provider` missing (peer refactor of repo CLI) |
| semio-framework-artifact-workflow-run-admission | 1 | fixed after ws20 (`ColdMutation::diff`), unverified |
| semio-framework-io-sqlite-snapshot-testing-backing | 1 | fixed after ws20 (`sqlite_snapshot::ArtifactSqliteSnapshot`), unverified |
| semio-framework-os-flow-host-source | 5 | fixed after ws20 (`#[value(send_only)]` on 3 owners), unverified |
Test targets still red: `semio-framework-os-store-one-item-batch` `mutation_apply` (55, suite half-ported to the immutable Document design), `os-infinite-world-model` `world_model_unit` (2), plus companion tests of kernel, dsl, command, spr-laws, host-space, flow laws, artifact-infinite-dag, workflow-schema-laws, mcp.

## What was fixed (this wave)
Libs: spr protocol law, ui-wgpu-mounted-layout (`WalkFrame`), neural-engine-budgeted (`send_only` x4), infinite-world model/components/scene/bridge/cursors/interaction/host (pub types, call-site ports of 12 authority-core cursor steps and 4 host calls, self-enabling testkit dev-dependencies), ui-wgpu-render (testkit-only upload arms), media-slots, durable-group-wire, dashboard-terminal/source-view/inventory, pets-traversal, repo-mcp (`Schema.min_length/pattern`, `Content::json`), value decode (`NativeDecodeCloseControl` for the forwarded control), `ArtifactCanonicalTreeSource` moved down into `os-store-canonical-json`, ephemeral transfer/preparation factories moved down into `os-store-ephemeral`, store-config/space/space-member/member-open-store/testing-doubles bound propagation (`Clone`, `FromValue`, `ColdMutation<P>`, `Sync`), kernel native VCS command (extension traits instead of an impl on the foreign `ArtifactStore`), plugin ownership (`ColdMutation` bounds, `advance` arity, `send_only`), repo-cli entrypoint (`script_index`, `build_verbs`), diagnostic `Fault.domain` box, db-io-core (`DbIoFault.custody`, pub custody API), plugin-abi (`ColdMutationDiff` on `DerivedArtifactSpec::Diff`), `derive(Mutations)` direct dependencies (`os-command`, `schema-state`) in workflow-schema, dag-vcs, spr-laws-tests, mcp-catalog.
Tests: services native-io moved in-crate, directory-tests-1/-client-tests, codec-registry, space-schema, engine derived-operation, store-read, snapshot-clone, os-inference, list-delta, presence retirement, host-wake, closure, scene/world-model close loops.
Cargo.lock: several `update --workspace --offline` (dev/normal edge changes only) and one manual edge removal (ephemeral no longer dev-depends on one-item-batch).

## Findings for other owners
- 223 `#[mutations(...)]` derive sites lack `reference`/`reference_diff`, so they have no `ColdMutation`; apps-b added them for M-Z plugins. Every store bridge needs the bound.
- Source gates: `os-store-engine` was at its limit (store-split fixed), `semio-framework-async` exceeds 4,000 active lines when test targets are counted.
- Several `#[cfg(test)]` fields/arms are used from `cfg(any(test, feature = "testkit"))` code (db-io ledger `owner`, sync-native actor fields, wgpu uploads): feature unification turns that into compile errors in `--workspace --all-targets`; fixed where seen.
- Infra: repeated transient `[admission] Uncaptured implementation schema read refused`, `Authenticated captured build command entry required`, `Original capture invocation changed`, `SEMIO_LEASE_ORDER` hold-and-wait refusals while peers edit the loader/launcher; `run.sh` retries them.
- Tests of the course closure: not run by me (teaching workspace test targets compile). `rust-tests` runs the framework chunks 088 to 109.

## Open / next
1. Read `ws20-fw.log` and `ws20-hub.log` when the chain9 chain finishes (pid in `chain9.out` run), then fix the framework workspace tests.
2. Re-run the root check after the post-ws20 fixes (db-io-registry, run-admission, sqlite backing, flow-host-source).
3. Teaching/course `cargo test` runs (`--manifest-path 🎓️teaching/Cargo.toml --workspace`) not started.

## PAUSE (coordinator order 10-22, graph re-inference is the critical path, swap 6.9/8 GB)
No cargo check/test is running or queued from rust-health. I cancelled the queued `test --manifest-path 🎓️teaching/Cargo.toml --workspace` (it never started; waiting in the capacity queue) and the chained root re-check (`chain10`). Resume with `sh C/rust-health-tools/chain10.sh 1 <logprefix>` (teaching tests, then root check) or `chain9.sh <logprefix>` (root, framework, hub) once the coordinator announces the graph is sealed.
State at pause: teaching workspace check GREEN; hub demonstrator component check red only in `plugin-window-config` (apps-b); framework workspace check blocked by the `semio-framework-async` source gate (4,092/4,000 active lines, also with `--exclude`; needs the gate owner); root workspace: 15 libs red in ws20 (table above), fixes after ws20 for db-io-registry, run-admission, sqlite backing and flow-host-source are written but unverified.
