# W1 GLTF Mutation Framework-Plugin API-Drift Remediation

## Scope

Resolved the framework-plugin compile drift identified in `🧪️w1-gltf-mutation-ocp-candidate-check.txt`, then handled the three newly exhaustive channel commands required by the current canonical SPR/store contract. Only `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` was changed; no glTF leaves, mutation integration, transports, AGENTS files, or kernel SPR/store/host files were edited.

## Contract Updates

| Drift | Canonical contract | Plugin implementation |
| --- | --- | --- |
| `AppFrame::Invocation` requires `messages` | Packed command-scoped `DispatchReport` | Runtime resets the report at command entry, records the actual `CommandReceipt` messages/worst level, and encodes that report into `Invocation.messages`. |
| `AppFrame::Error` requires `report` | Packed `DispatchReport` accompanies only `mutation.rejected` | A policy rejection becomes typed `mutation.rejected` and emits the exact report. Non-dispatch faults carry no synthetic report bytes. |
| `ArtifactStore::snapshot_with_conflicts` removed | `snapshot`, `conflicts`, and `open_conflicts` are the authoritative APIs | Deleted the obsolete `VcsArtifactApp` compatibility wrapper. |

## Merge And Conflict Commands

`SetMergePolicy`, `ResolveConflict`, and `ReadConflicts` now use only the authoritative store APIs:

- `SetMergePolicy` accepts only canonical ordinals `0..2`, updates local policy without a document/history write, and returns `Done`.
- `ResolveConflict` accepts only `Accept`/`Discard`, delegates to `ArtifactStore::resolve_conflict`, and returns a correlated packed `MergeReport` followed by the current open `Conflicts` projection.
- `ReadConflicts` returns the correlated packed open `Conflicts` projection.
- Unknown policy/resolution ordinals return typed `merge.invalid-policy` / `merge.invalid-resolution` faults and make no state change.

The focused test `merge_channel_commands_preserve_authoritative_policy_conflicts_and_payloads` seeds a valid degraded conflict, checks policy locality and invalid-ordinal atomicity, resolves the conflict through the store, and decodes the exact `MergeReport` and `Conflicts` frame payloads.

## Verification

- `rustfmt --edition 2021 --check 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — passed.
- `cargo test -p semio-framework-plugin --lib merge_channel_commands_preserve_authoritative_policy_conflicts_and_payloads` — passed: 1 passed, 0 failed, 217 filtered out. The first invocation with `--exact` compiled successfully but selected 0 tests because the full Rust test path is module-qualified; the substring invocation above executed the test.
- `cargo check -p semio-framework-plugin` — passed, exit code 0. Existing workspace/plugin warnings remain (84 warnings in this check); none were introduced or modified by this remediation.
