# FEM 2D artifact Rust workspace: items #1, #2, #5, #6 (agent a)

Workspace: `C:\semio\✏️s\🔌️plugins\🏗️fem\🗿️artifacts\◻️2d\Cargo.toml`
Native env: `CARGO_TARGET_DIR=G:\t\2d CARGO_BUILD_BUILD_DIR=G:\t\2d-b`, later also `TMP/TEMP/TMPDIR=G:\t\tmp CARGO_INCREMENTAL=0`, `--config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""'`.

## Result of the check (green for my crates)
`check -j 8 -p semio-s-artifact-fem-2d-snapshot-tests -p semio-s-artifact-fem-2d-results-state -p semio-s-artifact-fem-2d-editor --tests --keep-going` finished with zero errors (`a-check5.log`, 34m31s). That covers lib tests of snapshot-tests and results-state, the editor lib tests (the other agent's files were already ported by then) and the `editor_boot`, `editor_app_2`, `editor_app` test targets (#6: no source errors, the earlier failures were disk-full). `semio-s-artifact-fem-2d-io` lib test (#2) also checked clean (`a-check4.log`).

## Fixes (files)
Snapshot-tests crate (#1), ported from the already migrated 3D twin (`original_owners::with_decoding_granted/with_encoding`, `semio_s_fem_receiving::testkit::retire`, `u128` work fuel, `.nodes.get().get(i)`):
- `🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/🦀️.rs` (adds `original_owners`, `Lend`, `with_lend`)
- `.../🧪️tests/identity-index-tests.rs`, `native-tests.rs`, `write-pack-tests.rs`, `loads/receiver-tests.rs` (2D-specific content kept: planar nodes, Bar/Beam, Area with `region_id`, no solid test), `examples-tests.rs` (drain through testkit `retire`)
- `.../🧪️tests/📦️packages/🦀️rust/Cargo.toml`: dev-dependency `semio-framework-test-original-snapshot-owners`

SQLite snapshot tests (#2) in `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`: authoring state is now `Fem2dSnapshotValue` and owned through `Fem2dSnapshot::from_reference` (`Own` wrapper retiring via `retire_sqlite_snapshot`); `every_word` edits the value type (non-finite words skipped where validation rejects them, as in 3D); the `public_populated_owner` module that needed the non-dependency `semio_plugin_reactor_app` and a cyclic editor/viewer closure was removed (same as the migrated energy twin); codec calls use `original_owners::{export_sqlite,import_sqlite,encode_native,decode_native}`.
- `🚪️io/📦️packages/🦀️rust/Cargo.toml`: dev-dependencies `semio-framework-test-original-snapshot-owners`, `semio-s-fem-material`, `semio-s-fem-node`.

Results state (#5): `🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧪️tests/🔬️unit/🦀️.rs` `close_step(4096, effect)` now driven through `semio_s_fem_receiving::testkit::granted`, result read from `RetainedWorkPoll::Complete`.
- `🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/📦️packages/🦀️rust/Cargo.toml`: dev-dependencies `semio-os-kernel` with `protocol-laws` and `mutation-testing` (playback-clock law test needed `os_spr::protocol_laws`) and `semio-s-fem-receiving` with `testkit`.

## Test run
See the section below.

## Test run status (NOT complete)
`cargo test --workspace --no-fail-fast` was attempted 7 times (`a-test1..7.log`); none reached test execution because of environment issues: link.exe stalls on the saturated G: HDD, C: at 0 bytes (os error 112 in rustc/link), and the shared C:\t\2d-a build dir lock held by other cargo runs. Stopped on coordinator order (C: full).

Real compile errors found by `a-test6.log` in lib test of `semio-s-artifact-fem-2d` (it mounts the io sqlite snapshot tests, #2) and fixed afterwards, not yet re-verified by compiler:
- `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`: removed the stale `native_caller` mount (framework file `🪐️space/🧪️tests/🪶️sqlite/🫴️native-caller/🦀️.rs` is out of date vs the retained-work API: `close_retirement_recipient` gone, `.result` on `RetainedWorkPoll`; it should be ported or deleted by the framework owner, the 3D/energy twins still mount it); `materials[0].name.as_str()` for the sealed name; `real_interior` match arms made error-type uniform with `.is_err()`.
- snapshot-tests crate: the one error in `a-test4.log` was disk-full (LLVM IO failure); `a-test5.log` compiled it cleanly.

## Next
After the coordinator wipes C:\t\2d-a/2d-ab: rerun with `CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 -j 6` (`TMP=C:\t\tmp CARGO_INCREMENTAL=0`), check `semio-s-artifact-fem-2d` lib test compiles, then run the whole test pass and fix failures.
