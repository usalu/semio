# FEM 3D artifact: native Rust check state (check7)

Command (3d workspace, `C:\semio\✏️s\🔌️plugins\🏗️fem\🗿️artifacts\🧊️3d`):
`cargo check --all-targets --keep-going --message-format short` with `CARGO_TARGET_DIR=G:\t\3d`,
`CARGO_BUILD_BUILD_DIR=G:\t\3db`, `TMP/TEMP/TMPDIR=G:\t\tmp`, `CARGO_INCREMENTAL=0`.
Result `rc=101`, 245 errors, log `🗑️generated/a3d-native-check7.log`, digest `🗑️generated/a3d-errors-digest.txt`.

## Findings

- Framework crates compile. The earlier `semio-os-store-document`, `folder_archive_load` and
  `collect_window_kind_ids` errors were mid-edit snapshots and did not recur.
- All 3D library targets compile (model, diff, load, load-2, boundary, mesh-*, analysis, material, editor lib).
- Only three targets of `semio-s-artifact-fem-3d-editor` fail, all test targets that are stale
  against the current library API:
  - `lib test`: 8 errors
  - `test editor_app_2`: 68 errors
  - `test editor_app`: 170 errors

## Error classes (counts over the 245 errors)

| Class | Count | Typical cause |
| --- | --- | --- |
| E0425 / E0433 / E0422 (unresolved names) | 146 | tests call `handle`, `ConfigView`, `edit::`, `fem3d_entity_target`, `fem3d_load_target`, `FEM3D_RETAINED_TOOL_IDS`, `create_fem3d_app`, `semio_s_fem_node` that no longer exist or are not linked as dev-dependencies |
| E0308 (mismatched types) | 52 | `Fem3dSnapshot` now arrives as `Result<_, TextError>`, `Fem3dSnapshotValue`, `SealedShared<String>`, `Arc<ViewModel>`, `WindowConfigAuthorityIdentity` |
| E0603 (private import) | 10 | `AddNode`, `AddNodalLoad`, `Patch*`, `FocusEntity` no longer re-exported |
| E0599 / E0609 | 19 | `from_value` removed, `WindowConfigPack.files` removed, `load_window_config_pack` removed |
| E0061 | 8 | `handle`-style functions gained an argument |
| E0782 / E0432 / E0277 | 8 | bare trait types, `assert_registered_presence_round_trip` moved, `ArtifactHistoryLedger` is no longer an iterator |

## Status

No source was changed. Next step is to port these tests to the current editor API, command module by
command module (`🎮️commands/*`, `🎭️modes/✏️edit/🪟️windows/*`, `📌️panels/*`, `🪟️window-config-ownership`),
then run `cargo test` for the editor crate.
