# 🔍️ Audit: Diff-Only Mutations In `🧰️framework` (OS Scope)

Ticket: `DIFF-ONLY-MUTATIONS-WITH-CENTRAL-APPLY-AND-DIFF-SUMMING-INVERSES` (read-only audit; no source edited, no builds, no git writes).

Scope: every `impl ... MutationKind<`, `CompositeMutationKind<`, hand `impl ... Mutation<` with a `diff` fn, under `🧰️framework/🛍️products/💻️os` plus the framework's other mutation impls outside `🔨️modules/📡️replication/🎮️mutation`. Violation codes and laws are from `📋️design.md`.

## Method

1. Listed impl sites with a brace-aware scanner over the 236 framework files that contain a mutation trait (`🔍️` helper scripts in this folder).
2. Extracted the `diff`, `inverse`, `apply`, `absorb` bodies (comments and strings masked) and the shared helpers they call.
3. Read each body; patterns checked: `.apply(`, `&mut` on snapshot types, `base.clone()` followed by field writes, `between(`, `inverse` calling `.diff(`, unconditional `Vec::new()`, whole-snapshot diff types, shared generic helpers.
4. Coverage (V4): searched test modules for an inverse round-trip that names the kind (`inverse(`, `assert_operation_round_trip`, `assert_store_roundtrip`, `assert_mutation_inverse_law`, fold/restore loops); adjudicated the heuristic misses by reading the test files.

Counts: 150 kinds (114 `MutationKind`, 7 `CompositeMutationKind`, 29 hand `Mutation` with `diff`), including 9 macro-generated UI-preference kinds; 95 kinds with no violation code (V4 aside); 24 with no L3 test found.

## Summary By Artifact / Aggregate

| Artifact / aggregate | Kinds | V1-SNAP | V1-GEN | V2-DIFF-DERIV | V2-RESTORE | V2-EMPTY | V3-LEAF | V3-HAND | V4 untested |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Plugin test fixtures and host | 31 | 0 | 1 | 3 | 0 | 0 | 3 | 0 | 3 |
| Store space history and demo fixtures | 27 | 0 | 0 | 0 | 17 | 0 | 0 | 0 | 3 |
| OS config settings | 21 | 20 | 0 | 0 | 1 | 0 | 10 | 0 | 1 |
| Workflow | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 7 |
| DAG | 14 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| SPR counter laws | 13 | 0 | 0 | 4 | 0 | 1 | 4 | 0 | 2 |
| Flow | 10 | 1 | 0 | 0 | 1 | 0 | 1 | 0 | 1 |
| Workflow run | 5 | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 4 |
| Space and collection | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| DB test fixtures | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Print chart | 1 | 0 | 0 | 1 | 1 | 0 | 1 | 1 | 0 |
| Replication causal fixture | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| MCP probe fixture | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 |
| Renderer probe fixture | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 |
| Plugin interaction config | 1 | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| **Total** | **150** | **22** | **2** | **8** | **21** | **5** | **19** | **1** | **24** |

Coverage notes for V4 (per-kind inverse test found by name or by an aggregate round-trip loop):

- V4 kinds (no inverse round-trip found): workflow `ChangeParameter`, `UnbindParameterField`, `ConnectPorts`, `DisconnectEdge`, `UnbindInput`, `BindParameterField` (only op-line codec tests), `UpdateNodePorts`; run `StartRun`, `StartRunNode`, `AppendRunLog`, `SealRun`; flow `ReplaceFlowHostSnapshot`; SPR `AddUncheckedCounter`, `AddObservedCounter` (leaf test is codec-only); fixtures `AddN`, `CountedOp`, `WitnessOp`, `ReloadCountedOp`, `RecursiveFixtureMutation` (x2), `ProbeMutation`, `NativeSocketProbeMutation`, `RetainedLoadCameraConfigMutation`, `SetInteractionState` (set-state test calls `inverse` once, not for this kind by name).
- Covered (round-trip or inverse-law test naming the kind, or a loop over the kind's ops): DAG (named node round-trip tests), flow widget/synapse/layout ops (index-driven `ordered_collection_and_inverse_laws`, `widget_add_patch_remove_round_trip`, `set_layout_round_trip`), workflow `AddNode`, `MoveNode(s)`, `SetNodePositions`, `AddParameter`, `RemoveParameter`, `AddInput`, `RemoveInput`, `BindInput`, `BindOutput`, `UnbindOutput`, `RemoveNode` (`assert_operation_round_trip`), run `FinishRunNode`, space history (all six), space and collection (partial, see below), OS config (host `mutate-os-config-*` inverse law per family), UI preferences (`every_persisted_ui_preference_folds_and_inverts`), chart (`mutations_replay_inverse_and_reject_atomically`), SPR counters (`mutation_inverse_law_holds_for_add`, `counter_fixture_mixed_inverse_stored_order`), publication fixtures (`publication_leaves_apply_inverse_preserve_identity_diff`), set-dummy/surface/transaction fixtures.
- Only generic law helpers exist for the whole tree: `assert_mutation_inverse_law` and `assert_mutation_diff_absorb_law` (`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs:517,572`) are called only for the counter fixture (`⚖️protocol-laws-unit/🦀️.rs`). No test asserts `Σ inverse diffs == m.diff(base).diff().inverse(base)`; `DiffAlgebra` is implemented only for `ChartDiff` and the test `CounterDiff`.
- The derive-emitted aggregate law (`mutation_inverse_rows_failures`, `🔨️modules/📡️spr/🎮️command/🦀️.rs`) checks the inverse row count against the declared rows, not that the inverse restores `base`.
- `collection` and `space` round-trips cover 8 and 10 variants respectively (`CollectionMutation`: CreateFolder, DeleteEntry, DeleteFolder, MoveToCollection, MoveToFolder, RenameCollection, RenameEntry, RenameFolder; `SpaceMutation`: AddCollection, InstallProgram, RemoveCollection, RemoveUser, RenameCollection, SetExtensionEnabled, SetName, UninstallExtension, UninstallProgram, UpsertUser). The rest of each aggregate has no inverse round-trip.

## Per-Kind Audit

`file:line` is repo-relative. `[fixture]` marks kinds defined in test or fixture modules. `V4` means no inverse round-trip test was found for the kind.

| file:line | kind | violation codes | one-line evidence |
|---|---|---|---|
| `FW/🛍️products/📓️print/🧬️schema/🧬️mutations/🦀️.rs:34` | ChangeChartValue (Mutation) | V3-HAND-MUTATION, V3-LEAF-APPLY, V2-DIFF-DERIVED-INVERSE, V2-RESTORE-INVERSE | diff calls `diff.apply(base)` to validate (L4); inverse calls `self.diff(base).diff()` (L2); array-parent branch restores the whole array |
| `OS/🎚️config/🧬️schema/🧬️mutations/🧹clear-default-app/🦀️.rs:27` | ClearDefaultApp (MutationKind) | V1-SNAPSHOT-DIFF | diff is whole `OpeningPreferences`; no-op branch returns `base.clone()` as the diff |
| `OS/🎚️config/🧬️schema/🧬️mutations/📎️attach-local-folder/🦀️.rs:85` | AttachLocalFolder (MutationKind) | V1-SNAPSHOT-DIFF | diff is whole `LocalFolderBindings` rebuilt from a base copy; inverse concrete (detach or re-attach prior) |
| `OS/🎚️config/🧬️schema/🧬️mutations/📌️set-default-app/🦀️.rs:26` | SetDefaultApp (MutationKind) | V1-SNAPSHOT-DIFF | diff is whole `OpeningPreferences`; inverse concrete (set prior or clear) |
| `OS/🎚️config/🧬️schema/🧬️mutations/🛡️change-merge-policy/🦀️.rs:43` | ChangeMergePolicy (MutationKind) | V1-SNAPSHOT-DIFF | diff type is `MergePolicySetting` (the whole snapshot); inverse restores prior policy |
| `OS/🎚️config/🧬️schema/🧬️mutations/📥️admit-local-document/🦀️.rs:87` | AdmitLocalDocument (MutationKind) | V1-SNAPSHOT-DIFF | diff is whole `LocalCatalog` rebuilt from a base copy; inverse concrete |
| `OS/🎚️config/🧬️schema/🧬️mutations/📤️retire-local-document/🦀️.rs:24` | RetireLocalDocument (MutationKind) | V1-SNAPSHOT-DIFF | diff is whole `LocalCatalog` (filter copy); inverse re-admits prior |
| `OS/🎚️config/🧬️schema/🧬️mutations/✂️detach-local-folder/🦀️.rs:25` | DetachLocalFolder (MutationKind) | V1-SNAPSHOT-DIFF | diff is whole `LocalFolderBindings` (filter copy); inverse re-attaches prior |
| `OS/🎚️config/🧬️schema/🧬️mutations/🚪️sign-out/🦀️.rs:20` | SignOut (MutationKind) | V1-SNAPSHOT-DIFF | diff is whole `IdentitySetting(None)`; inverse re-signs prior identity |
| `OS/🎚️config/🧬️schema/🧬️mutations/🗂️set-named-layout/🦀️.rs:24` | SetNamedLayout (MutationKind) | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff does `let mut next = base.clone()` and writes named layouts (L4); diff is whole `UiPreferencesDiff` |
| `OS/🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🦀️.rs:87` | SignIn (MutationKind) | V1-SNAPSHOT-DIFF | diff is whole `IdentitySetting(Some(..))`; inverse signs out or restores prior identity |
| `FW/🔨️modules/📡️replication/🔗️causal/🧪️tests/🔬️unit/🦀️.rs:63` | CausalAddOp (Mutation fixture) | none | sparse diff via `payload`; inverse concrete undo |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/↔️move-node/🦀️.rs:18` | MoveNode (MutationKind) | none | sparse diff via `WorkflowDiff::MoveNode`; inverse concrete undo read from base |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/🗑️remove-node/🦀️.rs:16` | RemoveNode (MutationKind) | none | sparse diff via `WorkflowDiff::RemoveNode`; inverse concrete undo read from base |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/🧩add-parameter/🦀️.rs:16` | AddParameter (MutationKind) | none | sparse diff via `WorkflowDiff::AddParameter`; inverse concrete undo |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/🔒bind-parameter-field/🦀️.rs:15` | BindParameterField (MutationKind) | V4-LAW-UNTESTED | diff `BindParameterField { binding }` sparse; inverse `UnbindParameterField`; only op-line codec test |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/📤bind-output/🦀️.rs:15` | BindOutput (MutationKind) | none | sparse diff via `WorkflowDiff::BindOutput`; inverse concrete undo read from base |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/🩹change-parameter/🦀️.rs:18` | ChangeParameter (MutationKind) | V4-LAW-UNTESTED | sparse diff via `WorkflowDiff::PatchParameter`; inverse concrete undo read from base |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/🚮remove-input/🦀️.rs:16` | RemoveInput (MutationKind) | none | sparse diff via `WorkflowDiff::RemoveInput`; inverse concrete undo read from base |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/🔌bind-input/🦀️.rs:15` | BindInput (MutationKind) | none | sparse diff via `WorkflowDiff::BindInput`; inverse concrete undo read from base |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/⛔️unbind-output/🦀️.rs:16` | UnbindOutput (MutationKind) | none | sparse diff via `WorkflowDiff::UnbindOutput`; inverse concrete undo read from base |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/🔓unbind-parameter-field/🦀️.rs:16` | UnbindParameterField (MutationKind) | V4-LAW-UNTESTED | sparse diff via `WorkflowDiff::UnbindParameterField`; inverse concrete undo read from base |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/🧹remove-parameter/🦀️.rs:16` | RemoveParameter (MutationKind) | none | sparse diff via `WorkflowDiff::RemoveParameter`; inverse concrete undo read from base |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/🧬️mutations/✅️finish-run-node/🦀️.rs:15` | FinishRunNode (MutationKind) | none | sparse diff via `RunDiff::NodeFinished`; inverse concrete undo read from base |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/🧬️mutations/▶️start-run-node/🦀️.rs:15` | StartRunNode (MutationKind) | V2-EMPTY-INVERSE, V4-LAW-UNTESTED | diff `RunDiff::NodeStarted` changes node state; inverse is `Vec::new()` |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/🧬️mutations/🚀️start-run/🦀️.rs:23` | StartRun (MutationKind) | V2-EMPTY-INVERSE, V4-LAW-UNTESTED | diff `RunDiff::Start` changes run status; inverse is `Vec::new()` |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/🧬️mutations/🪵️append-run-log/🦀️.rs:18` | AppendRunLog (MutationKind) | V2-EMPTY-INVERSE, V4-LAW-UNTESTED | diff appends a log entry; inverse is `Vec::new()` |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/🧬️mutations/🔏️seal-run/🦀️.rs:15` | SealRun (MutationKind) | V2-EMPTY-INVERSE, V4-LAW-UNTESTED | diff seals the run (`sealed` flag); inverse is `Vec::new()` |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/📍️set-node-positions/🦀️.rs:35` | SetNodePositions (MutationKind) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/🚚️move-nodes/🦀️.rs:21` | MoveNodes (MutationKind) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/🔗connect-ports/🦀️.rs:15` | ConnectPorts (MutationKind) | V4-LAW-UNTESTED | sparse diff via `WorkflowDiff::ConnectPorts`; inverse concrete undo |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/🔄update-node-ports/🦀️.rs:13` | UpdateNodePorts (MutationKind) | V4-LAW-UNTESTED | diff `SyncNodePorts` is a no-op on the snapshot (apply `{}`); inverse `Vec::new()` is consistent; no direct L3 test |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/➕️add-node/🦀️.rs:15` | AddNode (MutationKind) | none | sparse diff via `WorkflowDiff::AddNode`; inverse concrete undo |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/📥add-input/🦀️.rs:15` | AddInput (MutationKind) | none | sparse diff via `WorkflowDiff::DeclareInput`; inverse concrete undo |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/🚪unbind-input/🦀️.rs:16` | UnbindInput (MutationKind) | V4-LAW-UNTESTED | sparse diff via `WorkflowDiff::UnbindInput`; inverse concrete undo read from base |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/✂️disconnect-edge/🦀️.rs:16` | DisconnectEdge (MutationKind) | V4-LAW-UNTESTED | sparse diff via `WorkflowDiff::DisconnectEdge`; inverse concrete undo read from base |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/✏️rename-node/🦀️.rs:17` | RenameNode (MutationKind) | none | sparse diff via `WorkflowDiff::PatchNode`; inverse concrete undo read from base |
| `OS/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:316` | ProbeMutation (Mutation) | V4-LAW-UNTESTED | fixture: diff `ProbeDiff(whole value)`; inverse `SetValue(base.0)` (single-field snapshot); no L3 test found |
| `OS/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs:331` | DemoMutation (Mutation fixture) | none | sparse diff via `DemoMutation::SetN`; inverse concrete undo |
| `OS/🔨️modules/🏪️store/🧬️schema/🧬️mutations/🔀️switch-space-alternative/🦀️.rs:22` | SwitchSpaceAlternative (MutationKind) | V2-RESTORE-INVERSE | inverse is the `RestoreActiveSpaceAlternative` variant (restores the whole value via base); diff sparse |
| `OS/🔨️modules/🏪️store/🧬️schema/🧬️mutations/🗑️remove-space-checkpoint/🦀️.rs:22` | RemoveSpaceCheckpoint (MutationKind) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/🏪️store/🧬️schema/🧬️mutations/🧹️remove-space-alternative/🦀️.rs:22` | RemoveSpaceAlternative (MutationKind) | V2-RESTORE-INVERSE | inverse is the `RestoreActiveSpaceAlternative` variant (restores the whole value via base); diff sparse |
| `OS/🔨️modules/🏪️store/🧬️schema/🧬️mutations/🌿️create-space-alternative/🦀️.rs:22` | CreateSpaceAlternative (MutationKind) | V2-RESTORE-INVERSE | inverse is the `RestoreActiveSpaceAlternative` variant (restores the whole value via base); diff sparse |
| `OS/🔨️modules/🏪️store/🧬️schema/🧬️mutations/📌️commit-space-checkpoint/🦀️.rs:17` | CommitSpaceCheckpoint (MutationKind) | none | sparse diff via `payload`; inverse concrete undo |
| `OS/🔨️modules/🏪️store/🧬️schema/🧬️mutations/🎯️restore-active-space-alternative/🦀️.rs:40` | RestoreActiveSpaceAlternative (MutationKind) | V2-RESTORE-INVERSE | inverse is the `RestoreActiveSpaceAlternative` variant (restores the whole value via base); diff sparse |
| `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:19148` | NativeSocketProbeMutation (Mutation) | V4-LAW-UNTESTED | fixture: diff whole value; inverse `Set(base.0)` (single-field snapshot); no L3 test found |
| `OS/🔨️modules/🏪️store/👥️presence/♻️retirement/🧪️testing/🧬️mutations/🔢️set-value/🦀️.rs:14` | SetValue (MutationKind fixture) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/🏪️store/🧪️tests/🧪️replay-retirement/🦀️.rs:137` | FailClosedOp (Mutation fixture) | none | sparse diff via `Fault::Message`; inverse concrete undo read from base |
| `OS/🔨️modules/🏪️store/🧪️tests/🧪️deferred-reprojection/🦀️.rs:280` | CountedOp (Mutation fixture) | V4-LAW-UNTESTED | delegating test wrapper: diff/inverse forward to inner op; no L3 test found |
| `OS/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:4105` | RetainedTextMutation (Mutation fixture) | none | sparse diff via `Self::SetRetainedText`; inverse concrete undo read from base |
| `OS/🔨️modules/🏪️store/🧪️tests/🧪️supersede-law/🦀️.rs:57` | LawOp (Mutation fixture) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs:1125` | WitnessOp (Mutation fixture) | V4-LAW-UNTESTED | delegating test wrapper: forwards diff/inverse to inner op; no L3 test found |
| `OS/🔨️modules/🏪️store/🧪️testing/🧬️mutations/🧮️demo/🧬️mutations/➕️add-n/🦀️.rs:14` | AddN (MutationKind fixture) | V2-RESTORE-INVERSE, V4-LAW-UNTESTED | inverse is the `RestoreN` variant (restores the whole value via base); diff sparse |
| `OS/🔨️modules/🏪️store/🧪️testing/🧬️mutations/🧮️demo/🧬️mutations/↩️restore-n/🦀️.rs:14` | RestoreN (MutationKind fixture) | V2-RESTORE-INVERSE | inverse is the `RestoreN` variant (restores the whole value via base); diff sparse |
| `OS/🔨️modules/🏪️store/🧪️testing/🧬️mutations/🧮️demo/🧬️mutations/🔢️set-n/🦀️.rs:14` | SetN (MutationKind fixture) | V2-RESTORE-INVERSE | inverse is the `RestoreN` variant (restores the whole value via base); diff sparse |
| `OS/🔨️modules/🏪️store/🧪️testing/🧬️mutations/🧮️demo/🧬️mutations/🗑️delete-n/🦀️.rs:12` | DeleteN (MutationKind fixture) | V2-RESTORE-INVERSE | inverse is the `RestoreN` variant (restores the whole value via base); diff sparse |
| `OS/🔨️modules/🏪️store/🧪️testing/🧬️mutations/⏱️timestamped/🧬️mutations/↩️restore-n/🦀️.rs:15` | RestoreN (MutationKind fixture) | V2-RESTORE-INVERSE | inverse is the `RestoreN` variant (restores the whole value via base); diff sparse |
| `OS/🔨️modules/🏪️store/🧪️testing/🧬️mutations/⏱️timestamped/🧬️mutations/🔢️set-n/🦀️.rs:15` | SetN (MutationKind fixture) | V2-RESTORE-INVERSE | inverse is the `RestoreN` variant (restores the whole value via base); diff sparse |
| `OS/🔨️modules/🏪️store/🧪️testing/🧬️mutations/🚦️severity/🧬️mutations/↩️restore-n/🦀️.rs:14` | RestoreN (MutationKind fixture) | V2-RESTORE-INVERSE | inverse is the `RestoreN` variant (restores the whole value via base); diff sparse |
| `OS/🔨️modules/🏪️store/🧪️testing/🧬️mutations/🚦️severity/🧬️mutations/🛑️set-fatal-n/🦀️.rs:19` | SetFatalN (MutationKind fixture) | V2-RESTORE-INVERSE | inverse is the `RestoreN` variant (restores the whole value via base); diff sparse |
| `OS/🔨️modules/🏪️store/🧪️testing/🧬️mutations/🚦️severity/🧬️mutations/🔢️set-n/🦀️.rs:19` | SetN (MutationKind fixture) | V2-RESTORE-INVERSE | inverse is the `RestoreN` variant (restores the whole value via base); diff sparse |
| `OS/🔨️modules/🏪️store/🧪️testing/🧬️mutations/🚦️severity/🧬️mutations/🚫️set-error-n/🦀️.rs:19` | SetErrorN (MutationKind fixture) | V2-RESTORE-INVERSE | inverse is the `RestoreN` variant (restores the whole value via base); diff sparse |
| `OS/🔨️modules/🏪️store/🧪️testing/🧬️mutations/🚦️severity/🧬️mutations/⚠️set-warning-n/🦀️.rs:19` | SetWarningN (MutationKind fixture) | V2-RESTORE-INVERSE | fixture: diff `DemoDiff::value(Some(n))` with warning; inverse `RestoreN(base.n)`; no L3 test found |
| `OS/🔨️modules/🏪️store/🧪️testing/🧬️mutations/🛂️validated/🧬️mutations/↩️restore-n/🦀️.rs:19` | RestoreN (MutationKind fixture) | V2-RESTORE-INVERSE | inverse is the `RestoreN` variant (restores the whole value via base); diff sparse |
| `OS/🔨️modules/🏪️store/🧪️testing/🧬️mutations/🛂️validated/🧬️mutations/🔢️set-n/🦀️.rs:19` | SetN (MutationKind fixture) | V2-RESTORE-INVERSE | inverse is the `RestoreN` variant (restores the whole value via base); diff sparse |
| `OS/🔨️modules/🏪️store/🧪️testing/🧬️mutations/🪤️lossy/🧬️mutations/🔢️set-n/🦀️.rs:44` | SetN (MutationKind fixture) | none | intentional negative fixture: diff `LossyDiff {}` drops the value; inverse re-issues the same op (L3 must fail by design) |
| `OS/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🦀️.rs:711` | CollectionMutation (Mutation) | none | sparse diff via `CollectionMutation::RenameCollection`; inverse concrete undo read from base |
| `OS/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🦀️.rs:567` | SpaceMutation (Mutation) | none | sparse diff via `SpaceMutation::SetName`; inverse concrete undo read from base |
| `OS/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/➕️create-node/🦀️.rs:13` | CreateNode (MutationKind) | none | sparse diff via `payload`; inverse concrete undo |
| `OS/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/↔️move-node/🦀️.rs:14` | MoveNode (MutationKind) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/✏️rename-node/🦀️.rs:13` | RenameNode (MutationKind) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🧮️change-node-operator-kind/🦀️.rs:13` | ChangeNodeOperatorKind (MutationKind) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🔡️change-node-abbreviation/🦀️.rs:13` | ChangeNodeAbbreviation (MutationKind) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/✂️disconnect-nodes/🦀️.rs:12` | DisconnectNodes (MutationKind) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🔀️reorder-nodes/🦀️.rs:12` | ReorderNodes (MutationKind) | V1-GENERIC-DIFF | diff stores the whole `order` list (`reordered_nodes`); inverse rebuilds the whole base order |
| `OS/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🖼️change-node-icon/🦀️.rs:13` | ChangeNodeIcon (MutationKind) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🔗️connect-nodes/🦀️.rs:17` | ConnectNodes (MutationKind) | none | sparse diff via `payload`; inverse concrete undo |
| `OS/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🗑️delete-node/🦀️.rs:12` | DeleteNode (MutationKind) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🔤️change-node-name/🦀️.rs:13` | ChangeNodeName (MutationKind) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/📐️resize-node/🦀️.rs:14` | ResizeNode (MutationKind) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🔁️replace-node-kind/🦀️.rs:13` | ReplaceNodeKind (MutationKind) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🗃️replace-node-properties/🦀️.rs:13` | ReplaceNodeProperties (MutationKind) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/🛢️db/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:944` | Add (Mutation fixture) | none | sparse diff via `payload`; inverse concrete undo |
| `OS/🔨️modules/🛢️db/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:1616` | HashMutation (Mutation fixture) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧪️tests/📥️retained-pack-load/🦀️.rs:125` | RetainedLoadCameraConfigMutation (Mutation fixture) | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V4-LAW-UNTESTED | test fixture: diff is the whole config; inverse is `Snapshot { base.clone() }` |
| `OS/🔨️modules/🔌️plugin/🏗️builder/🧪️testing/🔗️dependency-contribution/🧬️mutations/➕️add-value/🦀️.rs:18` | AddValue (MutationKind fixture) | none | sparse diff via `payload`; inverse concrete undo |
| `OS/🔨️modules/🔌️plugin/🏗️builder/🧪️testing/🔗️dependency-contribution/🧬️mutations/➕️add-value/🦀️.rs:40` | AddValue (CompositeMutationKind fixture) | V3-LEAF-APPLY, V2-DIFF-DERIVED-INVERSE | composite: diff via `fold_plan_diff`; inverse via generic `fold_plan_inverse` |
| `OS/🔨️modules/📡️spr/🧪️testing/🧬️mutation-laws/🧬️mutations/⛔️add-rejected-counter/🦀️.rs:15` | AddRejectedCounter (MutationKind fixture) | none | fixture: diff is `fatal` (no state change); inverse `Vec::new()` is consistent |
| `OS/🔨️modules/📡️spr/🧪️testing/🧬️mutation-laws/🧬️mutations/🚫️add-missing-counter/🦀️.rs:15` | AddMissingCounter (MutationKind fixture) | none | fixture: diff is `error` (no state change); inverse `Vec::new()` is consistent |
| `OS/🔨️modules/📡️spr/🧪️tests/⚖️protocol-laws-unit/🦀️.rs:288` | ColdCounterMutation (Mutation fixture) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/📡️spr/🧪️testing/🧬️mutation-laws/🧬️mutations/👁️add-observed-counter/🦀️.rs:19` | AddObservedCounter (MutationKind fixture) | V2-EMPTY-INVERSE, V4-LAW-UNTESTED | test fixture: diff `CounterDiff::delta(count)` changes the counter; inverse is `Vec::new()` |
| `OS/🔨️modules/📡️spr/🧪️testing/🧬️mutation-laws/🧬️mutations/🐛️add-unchecked-counter/🦀️.rs:15` | AddUncheckedCounter (MutationKind fixture) | V4-LAW-UNTESTED | fixture: diff `CounterDiff::delta(1)`; test `assert_leaf` checks codec and diff-level inverse only |
| `OS/🔨️modules/📡️spr/🎮️command/🧪️tests/🧪️mutation-payload/🦀️.rs:65` | Unpublished (Mutation fixture) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/📡️spr/🎮️command/🧪️testing/📔️registry/🧬️mutations/📛️rename-mini/🦀️.rs:17` | RenameMini (MutationKind fixture) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/📡️spr/🎮️command/🧪️testing/🧬️mutation-laws/🧬️mutations/➕️add-counter/🦀️.rs:15` | AddCounter (MutationKind fixture) | none | sparse diff via `payload`; inverse concrete undo |
| `OS/🔨️modules/📡️spr/🎮️command/🧪️testing/🧬️mutation-laws/🧬️mutations/🔢️add-counter-sequence/🦀️.rs:16` | AddCounterSequence (CompositeMutationKind fixture) | V3-LEAF-APPLY, V2-DIFF-DERIVED-INVERSE | composite: diff via `fold_plan_diff` (calls `diff.apply`); inverse via generic `fold_plan_inverse` |
| `OS/🔨️modules/📡️spr/🎮️command/🧪️testing/🧬️mutation-laws/🧬️mutations/🌐️add-counter-then-notify/🦀️.rs:17` | AddCounterThenNotifyForeign (CompositeMutationKind fixture) | V3-LEAF-APPLY, V2-DIFF-DERIVED-INVERSE | composite: diff via `fold_plan_diff` (calls `diff.apply`); inverse via generic `fold_plan_inverse` |
| `OS/🔨️modules/📡️spr/🎮️command/🧪️testing/🧬️mutation-laws/🧬️mutations/✌️add-counter-twice/🦀️.rs:16` | AddCounterTwice (CompositeMutationKind fixture) | V3-LEAF-APPLY, V2-DIFF-DERIVED-INVERSE | composite: diff via `fold_plan_diff` (calls `diff.apply`); inverse via generic `fold_plan_inverse` |
| `OS/🔨️modules/📡️spr/🎮️command/🧪️testing/🧬️mutation-laws/🧬️mutations/4️⃣add-counter-four-times/🦀️.rs:16` | AddCounterFourTimes (CompositeMutationKind fixture) | V3-LEAF-APPLY, V2-DIFF-DERIVED-INVERSE | composite: diff via `fold_plan_diff` (calls `diff.apply`); inverse via generic `fold_plan_inverse` |
| `OS/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️testing/🧬️job-test-mutations/🧬️mutations/➕️add-value/🦀️.rs:14` | AddValue (MutationKind fixture) | none | sparse diff via `payload`; inverse concrete undo |
| `OS/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️testing/🧬️job-test-mutations/🧬️mutations/➕️add-value/🦀️.rs:30` | AddValue (CompositeMutationKind fixture) | V3-LEAF-APPLY, V2-DIFF-DERIVED-INVERSE | composite: diff via `fold_plan_diff`; inverse via generic `fold_plan_inverse` |
| `OS/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🧬️mutations/♻️replace-flow-host-snapshot/🦀️.rs:17` | ReplaceFlowHostSnapshot (MutationKind) | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V4-LAW-UNTESTED | diff carries the whole `host_snapshot`; inverse is `ReplaceFlowHostSnapshot { base.clone() }` |
| `OS/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🧬️mutations/🗑️remove-widget/🦀️.rs:16` | RemoveWidget (MutationKind) | none | sparse diff via `FlowDelta::Widgets`; inverse concrete undo read from base |
| `OS/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🧬️mutations/➕️add-widget/🦀️.rs:17` | AddWidget (MutationKind) | none | sparse diff via `FlowDelta::Widgets`; inverse concrete undo |
| `OS/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🧬️mutations/↔️move-widget/🦀️.rs:16` | MoveWidget (MutationKind) | none | sparse diff via `FlowDelta::Widgets`; inverse concrete undo read from base |
| `OS/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🧬️mutations/✂️remove-synapse/🦀️.rs:16` | RemoveSynapse (MutationKind) | none | sparse diff via `FlowDelta::Synapses`; inverse concrete undo read from base |
| `OS/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🧬️mutations/🩹change-widget/🦀️.rs:17` | ChangeWidget (MutationKind) | none | sparse diff via `FlowDelta::Widgets`; inverse concrete undo read from base |
| `OS/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🧬️mutations/🔄change-synapse/🦀️.rs:16` | ChangeSynapse (MutationKind) | none | sparse diff via `FlowDelta::Synapses`; inverse concrete undo read from base |
| `OS/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🧬️mutations/🔀️move-synapse/🦀️.rs:16` | MoveSynapse (MutationKind) | none | sparse diff via `FlowDelta::Synapses`; inverse concrete undo read from base |
| `OS/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🧬️mutations/🔗️add-synapse/🦀️.rs:16` | AddSynapse (MutationKind) | none | sparse diff via `FlowDelta::Synapses`; inverse concrete undo |
| `OS/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🧬️mutations/📐️change-layout/🦀️.rs:16` | ChangeLayout (MutationKind) | V3-LEAF-APPLY | inverse clones `base.layout` into a local map and mutates it per entry (L4 smell); diff is sparse `FlowDelta::Layout` |
| `OS/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🗿️artifacts/🚫️snapshot-refusal/🧬️schema/🧬️mutations/🦀️.rs:7` | Mutation (Mutation fixture) | none | sparse diff via `payload`; inverse concrete undo |
| `OS/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🧬️schema/🧬️mutations/🦀️.rs:7` | Mutation (Mutation fixture) | none | sparse diff via `payload`; inverse concrete undo |
| `OS/🔨️modules/🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs:105` | ReloadCountedOp (Mutation fixture) | V4-LAW-UNTESTED | delegating test wrapper forwarding to `store::Mutation::inverse`; no L3 test found |
| `OS/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs:2694` | InertLabelOp (Mutation fixture) | none | sparse diff via `TestMutation::SetLabel`; inverse concrete undo read from base |
| `OS/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/📨️emission/🦀️.rs:204` | RefusingChildOperation (Mutation fixture) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/📨️emission/🦀️.rs:269` | TrackedChildOperation (Mutation fixture) | none | sparse diff via `payload`; inverse concrete undo |
| `OS/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs:130` | RecursiveFixtureMutation (Mutation fixture) | V4-LAW-UNTESTED | fixture: diff `ComposedParentDiff`/`RecursiveBranchDiff { Some(value) }`, inverse restores base field; no L3 test found |
| `OS/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs:631` | RecursiveFixtureMutation (Mutation fixture) | V4-LAW-UNTESTED | fixture: diff `ComposedParentDiff`/`RecursiveBranchDiff { Some(value) }`, inverse restores base field; no L3 test found |
| `OS/🔨️modules/📡️spr/🧪️testing/🧬️mutation-laws/🧬️mutations/➕️add-counter/🦀️.rs:17` | AddCounter (MutationKind fixture) | none | sparse diff via `payload`; inverse concrete undo |
| `OS/🔨️modules/🔌️plugin/🧪️tests/🧒️children-fixture-mutations/🦀️.rs:8` | ChildrenTestMutation (Mutation fixture) | none | sparse diff via `payload`; inverse concrete undo |
| `OS/🔨️modules/🔌️plugin/🧪️testing/📡️contributed-mutation-wire/🧬️mutations/➕️add-value/🦀️.rs:19` | AddValue (MutationKind fixture) | none | sparse diff via `payload`; inverse concrete undo |
| `OS/🔨️modules/🔌️plugin/🧪️testing/📡️contributed-mutation-wire/🧬️mutations/➕️add-value/🦀️.rs:45` | AddValue (CompositeMutationKind fixture) | V3-LEAF-APPLY, V2-DIFF-DERIVED-INVERSE | composite: diff via `fold_plan_diff`; inverse via generic `fold_plan_inverse` |
| `OS/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🦀️.rs:42` | SetDummyCount (MutationKind fixture) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🧬️document/🧬️mutations/🧒️set-slot-children/🦀️.rs:18` | SetSlotChildren (MutationKind fixture) | V1-GENERIC-DIFF | test fixture: diff `TestDiff { slot: Some(children) }` replaces the whole children list |
| `OS/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction/🦀️.rs:41` | SetTransactionCount (MutationKind fixture) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🦀️.rs:14` | SetLabel (MutationKind fixture) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count/🦀️.rs:14` | SetCount (MutationKind fixture) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/⏩️set-transaction/🦀️.rs:41` | SetTransactionCountWithoutPreflight (MutationKind fixture) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config/🦀️.rs:64` | ChangeTestConfigSelection (MutationKind fixture) | none | diff `TestConfigDiff::Clear/Set` sparse; inverse restores prior selection |
| `OS/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📣️set-transaction/🦀️.rs:41` | SetTransactionCountAndNotify (MutationKind fixture) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/🔌️plugin/🕹️interaction/🧬️mutations/🔁️set-state/🦀️.rs:37` | SetInteractionState (MutationKind) | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V4-LAW-UNTESTED | diff carries the whole new state; inverse is `set_state(base.clone())` |
| `OS/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🦀️.rs:41` | SetSurfaceCount (MutationKind fixture) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/1standard/🌐️any/🧬️mutations/📝️set-value/🦀️.rs:15` | SetValue (MutationKind fixture) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/1standard/🔒️strict/🧬️mutations/📝️set-value/🦀️.rs:15` | SetValue (MutationKind fixture) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/🔌️plugin/🦀️.rs:12383` | NoConfigMutation (Mutation) | none | uninhabited enum (`match *self {}`); nothing to audit |
| `OS/🔨️modules/🔌️plugin/🦀️.rs:12484` | NoPresenceMutation (Mutation) | none | uninhabited enum (`match *self {}`); nothing to audit |
| `OS/🔨️modules/🔌️plugin/🦀️.rs:12574` | NoTransientMutation (Mutation) | none | uninhabited enum (`match *self {}`); nothing to audit |
| `OS/🔨️modules/🔌️plugin/🧪️testing/📢️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication/🦀️.rs:56` | ChangePublicationTransient (MutationKind fixture) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/2standard/🌐️any/🧬️mutations/📝️set-value/🦀️.rs:15` | SetValue (MutationKind fixture) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🔨️modules/🔌️plugin/🧪️testing/📢️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication/🦀️.rs:56` | ChangePublicationPresence (MutationKind fixture) | none | sparse diff via `payload`; inverse concrete undo read from base |
| `OS/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🦀️.rs:14` | SetAppearance (MutationKind via optional macro) | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | macro-generated diff: `let mut next = base.clone()` writes one field (L4); diff is whole `UiPreferencesDiff(next)`; inverse concrete from base; covered by ui-preferences fold-and-invert test |
| `OS/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🦀️.rs:14` | SetTheme (MutationKind via optional macro) | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | macro-generated diff: `let mut next = base.clone()` writes one field (L4); diff is whole `UiPreferencesDiff(next)`; inverse concrete from base; covered by ui-preferences fold-and-invert test |
| `OS/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🦀️.rs:14` | SetLocale (MutationKind via optional macro) | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | macro-generated diff: `let mut next = base.clone()` writes one field (L4); diff is whole `UiPreferencesDiff(next)`; inverse concrete from base; covered by ui-preferences fold-and-invert test |
| `OS/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🦀️.rs:14` | SetLayout (MutationKind via optional macro) | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | macro-generated diff: `let mut next = base.clone()` writes one field (L4); diff is whole `UiPreferencesDiff(next)`; inverse concrete from base; covered by ui-preferences fold-and-invert test |
| `OS/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🦀️.rs:14` | SetTerminology (MutationKind via optional macro) | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | macro-generated diff: `let mut next = base.clone()` writes one field (L4); diff is whole `UiPreferencesDiff(next)`; inverse concrete from base; covered by ui-preferences fold-and-invert test |
| `OS/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🦀️.rs:14` | SetDriver (MutationKind via optional macro) | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | macro-generated diff: `let mut next = base.clone()` writes one field (L4); diff is whole `UiPreferencesDiff(next)`; inverse concrete from base; covered by ui-preferences fold-and-invert test |
| `OS/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🦀️.rs:15` | SetCustomTheme (MutationKind via keyed macro) | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | macro-generated diff: `let mut next = base.clone()` writes one field (L4); diff is whole `UiPreferencesDiff(next)`; inverse concrete from base; covered by ui-preferences fold-and-invert test |
| `OS/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🦀️.rs:15` | SetKeybindingOverride (MutationKind via keyed macro) | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | macro-generated diff: `let mut next = base.clone()` writes one field (L4); diff is whole `UiPreferencesDiff(next)`; inverse concrete from base; covered by ui-preferences fold-and-invert test |
| `OS/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🦀️.rs:15` | SetCustomDriver (MutationKind via keyed macro) | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | macro-generated diff: `let mut next = base.clone()` writes one field (L4); diff is whole `UiPreferencesDiff(next)`; inverse concrete from base; covered by ui-preferences fold-and-invert test |

## Shared Generic Helpers

| Helper | file:line | Callers | Finding |
|---|---|---|---|
| `fold_plan_diff` | `🔨️modules/📡️spr/🎮️command/🦀️.rs:833` | 7 handwritten composite kinds + derive(CompositeMutation) template (`🗣️dsl/✨️derive/🦀️.rs:1329`) | calls `diff.apply(&current)` per local step and `base.clone()` into `current` (L4: apply + base clone in framework helper reached from leaves) |
| `fold_plan_inverse` | `🔨️modules/📡️spr/🎮️command/🦀️.rs:871` | same 7 composites + derive template | generic inverse walker over the forward plan (not the diff); concatenates per-step leaf inverses in forward order (L2 generic-helper smell) |
| `Planner::call` | `🔨️modules/📡️spr/🎮️command/🦀️.rs:730` | every composite plan (via fold_plan_inverse / fold_plan_diff) | `self.base = diff.apply(&self.base)?` advances the planner snapshot inside the framework (L4 applier outside central path) |
| `apply_collection_mutation / inverse_collection_mutation / collection_diff_from_mutation` | `🔨️modules/🌿️vcs/🦀️.rs:1818-1878` | 0 kinds; re-exported to spr and used by spr unit tests only | generic `&mut Vec` applier and generic inverse helper over `CollectionMutation<TId,TItem,TPatch>`; not used by the collection or space leaves |
| `flow_wire_index` | `🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs:80` | 4 (RemoveWidget, MoveWidget, RemoveSynapse, MoveSynapse inverses) | index-to-wire conversion; not a violation |
| `workflow_parameter_entity_id` | `🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🦀️.rs:1251` | 3 (AddParameter, ChangeParameter, RemoveParameter) | entity-id accessor; not a violation |
| `workflow_targets_invariant` | `🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/🦀️.rs:105` | 2 (SetNodePositions, MoveNodes) | target validation; not a violation |
| `split_dag_endpoint` | `🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧬️schema/📸️snapshot/🦀️.rs:667` | 2 (RenameNode, DeleteNode) | endpoint parsing; not a violation |
| `dag_index_to_wire` | `🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🦀️.rs:179` | 2 (DisconnectNodes, DeleteNode inverses) | index conversion; not a violation |
| `DagDiff::from(DagDelta)` | `🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🦀️.rs:~450` | 14 DAG kinds | sparse typed delta constructor (one `Option` per edit); not a violation |
| `FlowDiff::from(FlowDelta)` | `🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🔺️diff/🦀️.rs:59` | 10 flow kinds | sparse typed delta constructor; `FlowDelta::HostSnapshot` is the one whole-snapshot variant |
| `SpaceHistoryDiff` | `🔨️modules/🏪️store/🦀️.rs:28177` | 6 space-history kinds | sparse struct (Option fields); not a violation |
| `attach_local_folder / admit_local_document / sign_in / set_named_layout` | `🎚️config/🧬️schema/🧬️mutations/{📎️attach-local-folder,📥️admit-local-document,🪪️sign-in,🗂️set-named-layout}/🦀️.rs:69/71/71/14` | 2 + 2 + 2 + 1 (inverse rows and fold-back) | constructor helpers used by inverses; concrete, not violations |
| `inverse_<family>_config_mutation(_steps)` | `🎚️config/🧬️schema/🧬️mutations/{📎️,📥️,🪪️,🛡️}*/🦀️.rs (6 files)` | host adapter tests (6 mutate-os-config-* suites) | thin pass-throughs to `mutation.inverse(base)` for fixture adapters; name matches the generic-inverse-helper policy pattern |
| `ChartDiff::inverse (DiffAlgebra)` | `🛍️products/📓️print/🧬️schema/🔀️diff/🦀️.rs:121` | 1 (ChartDiff) | diff-level inverse = `between(apply(base), base)`: applies and differences a snapshot copy (V1 at diff level) |
| `DemoDiff::value / CounterDiff::delta` | `fixture diff constructors (store tests, spr mutation-laws)` | 11 / 4 fixture kinds | test-only single-field diffs; not violations |

## Cross-Cutting Findings

- L5 is not implemented: `MutationDiff::apply(&self, base: &P)` (`🔨️modules/📡️replication/🎮️mutation/🦀️.rs:104`) takes no `ApplyCapability`, so any leaf can call it (ChangeChartValue does).
- L4 leak in the framework planner: `Planner::call` and `fold_plan_diff` call `MutationDiff::apply` (`🔨️modules/📡️spr/🎮️command/🦀️.rs:741, 853`); every composite diff therefore applies inside a framework helper.
- The reference violation pattern (`drawing_selection_inverse(base, diff(payload, base))`) has no exact twin in `🧰️framework`; the closest is `ChangeChartValue::inverse` (`diff(base).diff()`), plus `ChartDiff::inverse` (diff-level `between(apply(base), base)`).
- Whole-snapshot diffs are the dominant V1 pattern: config settings (diff type equals the snapshot), flow host snapshot, interaction state, UI preferences (via `base.clone()`), and the retained camera config fixture.
- Inverse-empty for state-changing kinds is concentrated in the workflow run artifact (`StartRun`, `StartRunNode`, `AppendRunLog`, `SealRun`). `FinishRunNode` is the only run leaf with a concrete inverse.

## Method Limits

- Body reading is mechanical plus manual review of every diff/inverse; helper internals were read only where a kind depends on them (`fold_plan_*`, `Planner::call`, `ChartDiff`).
- V4 uses a whole-framework search of test files for inverse calls or round-trip helpers within 300 characters of the kind name (excluding `fn inverse` definitions and the kind's own impl block), then reading the candidate tests. Kinds listed as covered were not all re-read line by line; a few (`SetValue`/`AddValue` declaration and wire channels, `SetSlotChildren`, `SetLabel`, `ChangeTestConfigSelection`) compare inverse rows or round-trip through a roster and may be L2 rather than L3 checks. `UpdateNodePorts` has a host-unit reference that was not inspected.
- V5-ABSORB (new in the Rulings) was not assessed in this pass. The absorb impls observed include `ChartDiff::absorb` (`edits.extend`, no same-path coalescing), `DagDiff`/`RunDiff` sequence appends, `CounterDiff` sums and the transient `*self = other`.
- Single-field fixture inverses (`ProbeMutation`, `NativeSocketProbeMutation`, `SetValue` channels) are a set of the one field, not a whole-snapshot restore; they are not coded as V2-RESTORE. The store fixture `RestoreN` family is coded V2-RESTORE because the Rulings remove `Restore` inverse variants.
- No build or test was run (read-only audit). Test-coverage statements are from reading test code, not from executing it.
- Not audited: `🧰️framework/🔨️modules/📡️replication/🎮️mutation` (excluded by scope) and `transient_root!` / `window_transient_*` macro outputs (no invocation in `🧰️framework`; invocations live in apps outside this scope).

