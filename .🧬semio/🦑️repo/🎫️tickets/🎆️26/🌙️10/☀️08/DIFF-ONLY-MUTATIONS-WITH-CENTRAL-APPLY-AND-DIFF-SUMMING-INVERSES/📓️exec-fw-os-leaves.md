# 📓️ exec-fw-os-leaves — framework kinds, hand impls and diff types

Status: WRITTEN, PARTLY VERIFIED. Only `cargo check -p semio-framework-os-config` (lib) was green (before the break below). Nothing else
compiled or ran. Since then `semio-framework-replication` fails to compile in a peer-owned file
(`📡️wire/🏠️local-interaction/🌳️root/🦀️.rs` and `🌳️root/🩹️update/🦀️.rs`: `expected RetainedCloneGrant, found Grant`,
`SharedOwner<String>` vs `Arc<String>`), so every dependent check/test is blocked. Not mine, not touched. Re-run when it is fixed.

## Per kind: before -> after

| Area | Before | After |
|---|---|---|
| Config opening (set/clear-default-app) | V1 whole `OpeningPreferences` diff | sparse `OpeningDiff` (absolute `(dialect, role)` pin rows); snapshot kept in canonical (dialect, role) order so inverses are exact |
| Config ui-preferences (10 kinds, 2 macros) | V1/V3 `base.clone()` + whole diff | macros deleted, each leaf concrete; sparse `UiPreferencesDiff` (`SettingEdit`, `KeyedEdit`, `NamedLayoutEdit`) |
| Config merge-policy, identity (sign-in/out), local-catalog (admit/retire), local-folders (attach/detach) | V1 whole-record diff, V3 `*snapshot = ...apply(...)` | `MergePolicyDiff`, `IdentityDiff`, `LocalCatalogDiff`, `LocalFoldersDiff`; `apply_*_config_mutation(_reporting)` bridges moved out of the leaves to `🎚️config/🧬️schema/🦀️.rs`, now `protocol::apply_diff` |
| Print `ChangeChartValue`/`ChartDiff` | V3 `apply` in leaf, derived + whole-array-restore inverse | `ChartEdit::authored` (read-only), concrete per-slot restore for array removal, `ChartDiff::inverse` simulates edits without `between`, sound adjacent coalescing in `absorb` |
| Run `StartRun`/`StartRunNode`/`AppendRunLog`/`SealRun`/`FinishRunNode` | V2-EMPTY inverses, mutation-level applier | `RunDiff` = ordered `RunStep`s; `apply_run_operation` deleted; inverses via four NEW kinds `SetRunHeader`, `SetRunSeal`, `RetractRunLog`, `RetractRunNode` (schemas, descriptors binary tags 5-8, wire witnesses added) |
| Workflow (20 kinds) | no algebra, last-wins absorb | `WorkflowDiff::Sequence`, concrete `negation`, `between`, flattening `absorb`; leaf inverse fixes in bind-parameter-field, change-parameter, update-node-ports |
| Space history (6 kinds) | single-slot diff, `Restore*` inverse | ordered `SpaceHistoryStep`s; `RestoreActiveSpaceAlternative` -> `SetActiveSpaceAlternative`; create-alternative inverse order swapped |
| Store fixtures (demo/severity/validated/timestamped) | `RestoreN` inverse variant | renamed `AssignN` (dirs, fixtures, TS, rs) |
| Flow `ChangeLayout` | clone-and-mutate map in inverse | per-entry previous read; `ReplaceFlowHostSnapshot` left (ruling: replace-<entity> whole record, inverse is the same kind with base) |
| Interaction `SetInteractionState` | aggregate was its own whole-state diff | `InteractionStateDiff` (`DomainEdit` rows), `Default`/diff impls on the aggregate removed |
| `NoConfig`/`NoPresence`/`NoTransient` stubs | `impl MutationDiff<X> for X` | shared `NoStateDiff` |
| `transient_root!` macro | state is its own diff | `TransientDiff<S>` (replacement option, algebra), exported from `app` |
| `RetainedLoadCameraConfigMutation` fixture | whole-config diff | `RetainedLoadCameraConfigDiff` |
| DAG, flow deltas, space/collection | already had algebra | untouched (see open issues) |

## Tests (all written, none run)

`assert_mutation_inverse_sum_law` added to: 29 config fixture test files, print (`change_chart_value_inverse_diffs_sum_to_the_negative_diff`,
fixtures + array removal), workflow (all 20 kinds), run (all 9 kinds), store (space history roster, demo roster), spr
counter fixtures (lawful + 4 `should_panic` negatives). Dev-deps added: config (`os-kernel` + `protocol-laws`, `async-macros`),
workflow, workflow-run (`protocol-laws`). Print dev-dep was extended by a peer. Config diff fixtures (`🔺️diff/🔣️.json`, 29) regenerated
by `T/🗑️generated/fw-os-leaves/config_fixtures.py` and config test docstrings/types updated by script.
Run test command once unblocked: `"$T/🚦️gate.sh" fw-os-leaves -- cargo test -p semio-framework-os-config -p semio-framework-print -p semio-framework-artifact-workflow-run -p semio-framework-artifact-workflow-workflow`, then `-p semio-framework-os-kernel` filtered on `inverse_diffs_sum`, and flow/plugin crates.

## Open issues

- Behaviour changes: print schema validation now happens at apply, not at diff time; sign-in/out and empty config no-ops return an empty diff; `update-node-ports` on desynced ports returns an inverse error (no kind restores port lists); run/ui clocks (`store::now_iso()`) are still read in `diff` (replay non-determinism pre-existing).
- Unsound slot-style `absorb` remains in `SpaceDiff`/`CollectionDiff` (last-wins per slot); converting needs the `diff_text!/diff_binary!` codecs checked. Not done.
- `impl_whole_record_config!` still has ~50 plugin users; delete after plugin waves. `transient_root!` users keep a whole-root replace by ruling.
- `ProbeDiff` (mcp), `NativeSocketProbeMutation`, `AddObserved/Unchecked` fixtures: unchanged beyond negative tests; ProbeDiff default is Null, not identity.
- Array/object removal inverses restore element values but object key order can differ (`DslValue` equality is order-sensitive).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json` is generated and now stale (renames, four run kinds): run `bun ./📜️script.ts schema generate`.
- R9 gate: `protocol::apply_diff(` remains only in `config/🧬️schema/🦀️.rs` bridges (non-leaf).

## Wave 3 (verification round)

Commands (all via `"$T/🚦️gate.sh" fw-os-leaves --`):

- `cargo check -p semio-framework-plugin -p semio-framework-os-config -p semio-framework-print -p semio-framework-artifact-workflow-run -p semio-framework-artifact-workflow-workflow -p semio-framework-artifact-space-space -p semio-framework-artifact-space-collection -p semio-framework-os-infinite --message-format=short` -> **PASS** (Finished, lib targets), after three fixes:
  1. plugin: stale `TransientDiff` entry in the `app` re-export list removed (fw-spine replaced my `TransientDiff` by `sparse_record_diff!` in `transient_root!`).
  2. run: `BorrowedDslField` for `RunStatus`/`RunNodeStatus` and `BorrowedDslRecord`+field for `RunTrigger` added (the derive now requires them; the crate was broken for all run leaves, not only mine).
  3. run: `retract-run-*` use approved verb `remove`; store `AssignN` fixtures use verb `set` (`assign` is not in `APPROVED_VERBS`).
- `cargo check -p semio-framework-artifact-flow-flow -p semio-framework-os-flow`: FAIL in peer files (`flow/🧵️retained`, `🌿️vcs/../🚪️io/🪶️sqlite/..` `expected RetainedCloneGrant, found Grant`); my flow edit (`ChangeLayout`) is not in the errors. Not re-run after the foundation change.
- `cargo test -p semio-framework-os-config`: first run compiled the lib tests and found 7 errors in my files (moved `apply_*` bridges not imported by two unit tests, `DefaultApp` import, two folder fixture tests still decoding the diff as the snapshot) -> fixed. Re-runs are blocked: `semio-framework-value`/`semio-framework-replication` retirement refactor by a peer does not compile (foundation.status RED 17:35 - 18:17, files `📡️replication/🔗️causal/🔀️transition/🔁️fold`, `🌱️value/♻️retirement`). Test pass/fail counts: **0 run** (not claimable).
- `bun ./📜️script.ts schema generate` -> exit 0, `schema-catalog.json` regenerated: 0 hits for `RestoreActiveSpaceAlternative`, 2 for `retract-run-log`, 3 for `set-active-space-alternative`.
- Gate burn-down items taken: print outcome/apply codes -> frozen vocabulary (`mutation.apply.invalid-path|missing-target|invalid-index|precondition-drifted|schema-invalid` in `ChartDiff`; the leaf maps them to `mutation.invariant`/`mutation.target-missing`/`mutation.target-mismatch`; the TypeScript twin updated the same way, plus per-slot array-removal inverse); store fixtures `fixture.refused` -> `mutation.apply.fixture-refused`/`mutation.target-missing`, `empty-targets` -> `mutation.invariant`. The 13 R14 hits in `gate-run-2.log` name the old `↩️restore-n` paths (git inventory still lists the pre-rename files; they disappear once the rename is tracked). R16 hits are in `📡️replication/..map/..shared-map-delta-native` and `🔌️plugin/🏗️builder/🦀️.rs:764` (not my files).
- `SpaceDiff`/`CollectionDiff`: already converted by a peer (`keyed_absorb`/`KeyedDelta`); not touched.

To run once foundation is GREEN: `cargo test -p semio-framework-os-config -p semio-framework-print -p semio-framework-artifact-workflow-run -p semio-framework-artifact-workflow-workflow`, then `-p semio-framework-os-kernel inverse_diffs_sum`.

## Wave 3b (gate run 3 follow-up)

The `RestoreN` -> `AssignN` rename was found reverted on disk (old `↩️restore-n` leaf dirs and `🧪️fixture-*-restore-n` test dirs back, 14 `RestoreN` in the store unit tests). Redone: 4 leaf dirs + 4 test dirs renamed to `assign-n`, tokens `RestoreN/restoreN/restore-n/restore_n` replaced in 45 files under `🏪️store` (verb `set`). `bun ./📜️script.ts schema generate` re-run afterwards.
`bun ./📜️script.ts verify mutation-outcome-law` (exit 1, plugin breaches only): **0 breach lines under `🧰️framework`** (`grep -a '^\[verify' | grep -c 🧰️framework` = 0; the 2 `🧰️framework` matches in the raw log are stack-trace frames). No cargo run (foundation RED).

## Wave 4 (T24, T25 reconcilers)

Verified by grep over `🧰️framework` and `✏️s`:

- `reconcile_workflow_snapshot` (`🖥️host/🦀️.rs:657`) has exactly one caller, `OsWorkflowStore::snapshot_with_conflicts(&self)` (`:869`): a `&self` read projection over `inner.snapshot()`. It is not on any edit/dispatch path; the only callers of `snapshot_with_conflicts` are the host unit test. Production edits go through `dispatch_apply` with concrete `WorkflowMutation` kinds. No conversion needed.
- `reconcile_collection_integrity` / `reconcile_space_atelier_invariant` (`🪐️space/🗿️artifacts/...:1083/:1147`): take a snapshot by value and return `(snapshot, messages)`, no store handle; the only references outside the defining crates are the `pub use` re-exports in `🪐️space/🦀️.rs` and unit tests. No production caller on an edit path.
- Tests added (WRITTEN, NOT RUN; foundation RED at 19:09): `🪐️space/🧪️tests/🔬️unit/🦀️.rs::load_repair_reconcilers_never_write_history_rows` (real `ArtifactStore` for space and collection; repair changes the projection, `envelope().vcs` and the stored snapshot stay identical), and two history assertions at the end of `🖥️host/🧪️tests/🔬️host-unit/🦀️.rs::concurrent_delete_and_wire_reconciles_without_a_dangling_edge` (`store.document()` equal before and after `snapshot_with_conflicts`).
- Run when GREEN: `cargo test -p semio-framework-os load_repair_reconcilers` and `cargo test -p semio-framework-os concurrent_delete_and_wire` (both live in the `semio-framework-os` crate).
