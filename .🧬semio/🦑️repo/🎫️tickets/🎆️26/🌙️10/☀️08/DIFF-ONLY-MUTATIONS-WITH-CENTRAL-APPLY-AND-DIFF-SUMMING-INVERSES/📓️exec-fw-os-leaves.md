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
