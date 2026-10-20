# rs-sweep4 — Rust name-resolution sweep (🧰️framework outside value family, 🌎️hub)

Checker: `rs-sweep-tools/` (run: `cd rs-sweep-tools && RS_BARE=1 bun main.ts --scope 🗑️generated/rs-sweep4-run/scope.txt --out <dir> --tag <tag>`).
Result: 433 findings (baseline run s1) -> 148 (run s4, before the final pets-staging dependency fix). Cargo/tests: **not run** (`🗑️generated/deps4-loop.log` never showed DONE; last lines `52 rc=1`, `50 rc=142`).

## Checker changes (rs-sweep-tools)
- parse.ts: `#[cfg]` on match arms, fields, enum variants, parameters now applies to their bodies (wgpu program-bridge false positives 24 -> 0); `#[derive(Mutations)]` generates `register_<snake>_descriptors` items.
- checks.ts: wasm32 environment is evaluated both ways, but only for packages with wasm-only target deps or cdylib and their transitive deps (`wasmClosure`); native-only `os-mcp` no longer yields false wasm findings.
- Limitation: feature flags of dependencies (e.g. ui-host `testkit`) are not applied while resolving.

## Per crate: before -> after (s1 -> s4)
| crate | before | after |
|---|---|---|
| plugin-tests-11-2 | 29 | 0 |
| plugin-tool-run | 26 | 4 |
| os-store-composition | 24 | 24 |
| os-store-one-item-batch | 20 | 20 |
| plugin-window-config | 19 | 19 |
| pack-record-encode | 18 | 8 |
| semio-hub-inference-contract-laws | 14 | 0 |
| plugin-tests-09-2 | 14 | 2 |
| pets-staging | 14 | 2 |
| os-store-durable-group | 14 | 0 |
| os-store-durable-group-tests | 13 | 0 |
| pets-staging-draft | 12 | 0 |
| os-store-engine-stage | 12 | 12 |
| os-store-ledger | 10 | 0 |
| plugin-instance-children | 10 | 0 |
| repo-dashboard-terminal | 10 | 0 |
| os-store-space | 10 | 10 |
| os-renderer-wgpu-kernel-protocol | 10 | 0 |
| plugin-tests-18 | 9 | 9 |
| plugin-reactor-turn | 9 | 0 |
| os-store-space-member | 8 | 8 |
| os-store-config | 6 | 0 |
| replication-mutation-retained | 6 | 0 |
| os-store-engine-core | 6 | 4 |
| os-infinite-board-schema | 6 | 0 |
| os-renderer-wgpu-native-probe | 6 | 2 |
| os-kernel-db-sync-laws | 6 | 0 |
| semio-hub-stdio | 5 | 0 |
| os-store-space-schema | 4 | 2 |
| replication | 4 | 0 |
| plugin | 4 | 0 |
| plugin-reactor-lifecycle | 4 | 0 |
| os-mcp | 4 | 0 |
| os-store-durable-group-wire | 4 | 0 |
| plugin-tests-12-2 | 4 | 2 |
| os-store-codec-registry | 3 | 0 |
| plugin-tests-01 | 3 | 3 |
| os-renderer-wgpu-runtime-mailbox | 3 | 0 |
| plugin-child-group-wire-laws | 3 | 0 |
| os-infinite | 3 | 0 |
| plugin-instance-dispatch | 3 | 2 |
| os-kernel | 3 | 0 |
| os-store-child | 2 | 2 |
| plugin-instance-jobs | 2 | 2 |
| pack-format | 2 | 0 |
| job | 2 | 0 |
| os-kernel-db-artifact-contract | 2 | 0 |
| plugin-tests-03 | 2 | 0 |
| async | 2 | 0 |
| os-config | 2 | 0 |
| plugin-tests-16 | 2 | 1 |
| os-kernel-db-sync | 2 | 0 |
| os-store-operation-wire | 2 | 2 |
| os-store-testing-prelude | 2 | 0 |
| plugin-instance-app | 2 | 2 |
| ui-host-tests | 2 | 2 |
| os-store-engine-live | 2 | 2 |
| os-store-replay | 2 | 0 |
| os-renderer-wgpu | 1 | 0 |
| plugin-tests-07-3 | 1 | 0 |
| plugin-tests-14-2 | 1 | 1 |
| plugin-interpreter | 1 | 0 |
| os-renderer-wgpu-runtime-mailbox-tests | 1 | 0 |
| os-store-tests-04 | 1 | 1 |

## Changes (grouped; all re-points to moved/renamed definitions or completing a half-written API)
- tool-run `👥️member-owners-and-visitors`: `ToolRunMemberFold` gains `grant`, `receiving`; new `ToolRunMemberFoldRefusal` (+ `From<Fault>`); staging file re-pointed.
- store testing prelude: `FixtureStoreReceiving` added; durable-group tests use it (dep `value-native-owned` added).
- durable group: `DurableOwnedGroupTestMember` + public member-based `durable_group_test_outcome`; `from_returned`/`begin_returned_close`/`admit_returned` widened for the cross-crate test user; feature chain `durable-group-testing` through one-item-batch and one-item-publication.
- replay/ledger: `ColdFoldProjection` (replay) replaces ledger `FoldProjection`; `fold_operation_into` exported; engine-core batch fold, kernel store root, `io/📝️text`, engine-live store and replay-owned-state-operations re-pointed; ledger `📥️lifted/🏪️store` removed.
- cold migration: `apply_diff` -> `apply_diff_cold` (hub stdio editor-catalog test, gis native-codecs test, hub wal unit test with `diff_semio_*_mutation`); `envelope_from_operation_cold`; `ReferenceConfigStoreHydration` -> `RetainedConfigStoreHydration`.
- hub: shipped-fleet sqlite owners fixture restored from `8f91299152c^` and re-included; gis path re-points; `law-fixtures`.
- wgpu: kernel task ingress mounted in renderer-kernel-protocol; runtime-mailbox dead import removed; host-plumbing + ui-host `testkit` gating; native-socket-probe snapshot impl gated native.
- pack: format unit test helper `build_plain_pack`; record-encode cursor test helper; `pack::intrinsic::*` extern-name fixes; os-pack record path.
- plugin: window-config `WindowConfigBorrow` export; reactor `result_publication` module mounted and callers re-pointed; `TestRuntimeApps` public; `Framework*JobFactory` imports; `fixture_pending_effects`; interpreter testkit split; mutation-retained `testkit` feature + `testing_apply_capability`; dev-deps (`replication`, `plugin-job`).
- space-schema/replication facade: merged duplicate `MutationKind` impls, removed duplicate root `retained` re-export and unused `apply_diff` imports.
- db-sync: `WelcomeResponse`, `BootstrapPageRangeCopy`, `bootstrap_page_range_copy`, `lower_bootstrap_plan`, `build_welcome` all gated `any(test, feature = "testkit")`; laws `test_allocation` + allocator + dev-dep; async/ `worker_parking`/`native_pool` wasm gating.
- pets: dead imports removed; `Venture`/`geared`/`reaches`/`set_out` made `pub` (used by choice); `Body` (alias `Solid`, per the TypeScript twin) imported from `pets-clearance` in spacing, courses and population with the dependency added to `pets-staging-draft` and `pets-staging` Cargo.toml (the `Solid`/`Body` fix in staging was **not re-checked** after the final manifest edit; `Cargo.lock` refresh pending).
- dashboard terminal docking test: `ui_tui_core::geometry::Rect`.

## Verification
- Checker s1 (433) -> s2 (199) -> s3 (154) -> s4 (148); outputs in `🗑️generated/rs-sweep4-run/findings-*.jsonl`.
- cargo check / cargo test / bun tests: not run.

## Open items (not completed, evidence in checker output)
1. `ReferenceSpaceMember` facet split (store-composition 24, space-member 8, space 10, engine-stage 12): facet file at `🧩️composition/🪪️member-facets/🧊️reference/🦀️.rs` is unmounted; design confirmed by the `🧪️testing/🎭️doubles/🗄️store` doubles. Needs 19 reference/cold methods removed from `SpaceMember`/`ArtifactStore`/`NoMembers`/`space_members!`, `reference_space_members!` relocation and plugin instance bound migration. A trial removal was reverted.
2. `apply_diff` in generic engine code (engine-core 4, one-item-batch mutation_apply 20, space-schema tests) needs the `ColdMutation` bound migration (`P: Clone`).
3. Durable-group original-retirement cluster: `retire_binding_member` never defined; `DurableOwnedThreeStorePreparedV1`/`…MemberDecisionV1` live in a higher crate than their impls.
4. window-config (19): preview `WindowConfigStore`, fixtures `IdentityWindowOwner`/`RetainedLoad*`; in-crate fixture mounting was tried and reverted (46 new findings).
5. `ChunkLocator` (pack-record-encode cursor, 8) and `Std1AnySnapshot`/`Std1StrictSnapshot`/`Std2AnySnapshot` (plugin tests-18, 9) are defined nowhere in the repo (never-written APIs; tests-18 types likely came from a removed macro); `PersistedChildHeaderNativeReceiver`, `OriginalReplayEdits` (defined `pub(super)`), `ArtifactOperationImage`, `parse_document_pack_envelope`, `HostCallCensus` (declared inside a fn), `TypedChildEmitPreparation` (private), `SnapshotProjection` (value family), instance-app/dispatch/jobs bare names, `RetainedWireByteSource`/`OriginalWireSourceIdentity` (value family, not edited).
6. retained-command `test_rejected_completion_close` oracle never written; ephemeral store arity mismatch for `advance_publish_one`.
7. wasm-only: `os-mcp` classified native-only; native-socket-probe `JsValue` (2) needs a wasm-gated import check; stray `🦀️.rs.orig` in mcp workspace/remote.
8. pets-choice still has free consts/fns the checker cannot detect (bare fn/const detection absent).
