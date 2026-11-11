# 1108 space result
Crates: semio-plugin-window-config-original (7), semio-framework-artifact-space-space (12), semio-framework-artifact-space-collection (50). Manifest 🌎️hub/Cargo.toml wasm.
## Edits 
- space-space + space-collection `io/sqlite/snapshot/🦀️.rs`: ArtifactSqliteSnapshotHost output-slot shape (NotStarted pre-checks, `decode/encode_sqlite_snapshot_record_native(.., output)`); validate_* overrides dropped (trait defaults).
- collection `io/sqlite/snapshot/🫴️receiving`: owner.receive_sqlite_snapshot replaced by the space-history `receive!` macro (TypedFileSettlement via store::space_history); equality-validation branches dropped; Frame.result.
- collection `📥️body-receiving`: ArtifactBodyReceiving delegates the new OwnedFieldReceiving shape (select_advance/advance_nested/advance_local/take_work_demand/close_work_demand, effect take_into/close_step); borrowed receive_value gets the output slot.
- space-space: DslReceiving derives on SpaceKind/Visibility/Role/User/CollectionRef/InstalledExtension/SpaceMutation + `original_ordinal_operation_receiving!` and `artifact_persisted_binary_operation!` for SpaceMutation.
- store `🦀️.rs` line 426: re-export `artifact_persisted_binary_operation` through the kernel facade.
## Debts
- collection `receiving/🧪️tests/🦀️.rs` (+ 🟦️.ts law via rust-package-commands) still uses the pre-733 grant API and validate_decoding/encoding (removed); not compiled by the lib check.
## Rerun
`"$C/1105b-tools/chk.sh" W-space-<n> 🌎️hub/Cargo.toml wasm -p semio-plugin-window-config-original -p semio-framework-artifact-space-space -p semio-framework-artifact-space-collection`

## Verification
W-space-3 (wasm, 🌎️hub/Cargo.toml, 3 crates, rc0, Finished): window-config-original, space-space, space-collection all green. window-config-original needed no edit from space (flowdoc2's capture/epoch identity fix cleared its 7).
No [DEBUG] logs added. Tests not run (receiving law tests are pre-733, see debts).

STATUS: FIXED-verified W-space-3
