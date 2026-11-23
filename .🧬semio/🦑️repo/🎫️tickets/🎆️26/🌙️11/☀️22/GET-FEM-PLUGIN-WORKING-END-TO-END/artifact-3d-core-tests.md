# FEM 3D core crates: test state (agent a3dcore)

Env: native cargo, `CARGO_TARGET_DIR=C:\t\3d` (earlier `C:\t\3dx`, deleted by the coordinator), `-j 6`, debug info off. Repo cargo (`repo cargo --store=shared`) was tried once and sat in source admission for 9+ minutes, so native cargo was used.

## Counts (before from `a3d-test-rest.log`, after from my last green runs)

| Crate | Before | After |
| --- | --- | --- |
| engine | 0 pass / 20 fail | 21 pass / 0 fail (log `a3dcore-engine2.log`) |
| io | 3 / 13 fail | 16 pass / 0 fail (`a3dcore-iomv3.log`) |
| mesh | 2 / 1 fail | 3 pass / 0 fail |
| viewer | 2 / 2 fail | 4 pass / 0 fail |
| model `original_fem_receiving` | 2 / 2 fail | 4 pass / 0 fail (`a3dcore-r4.log`, after the coordinator removed the framework `[DEBUG]` print: the ~1 KB heap mismatch hypothesis is confirmed) |
| snapshot-tests | 38 / 5 fail | 38 pass / 5 fail (unchanged, `a3dcore-r4.log`) |

## Fixes (files, all under `C:\semio\✏️s\🔌️plugins\🏗️fem\🗿️artifacts\🧊️3d`)

Retirement (tests dropped a sealed `Fem3dSnapshot`):
- New `⚙️engine/🧊️3d/🧪️tests/🧰️persisted/🦀️.rs` (mirror of the 2D twin, plus a `Persisted` guard that drains through `ControlledRetirement` on drop, also while unwinding). Mounted from `⚙️engine/🧊️3d/🧪️tests/🔬️unit/🦀️.rs`, `🗺️mesh-preview/🧪️tests/🔬️unit/🦀️.rs`, `🎵️modal-buckling/🧪️tests/🔬️unit/🦀️.rs`. `⚙️engine/🧊️3d/📦️packages/🦀️rust/Cargo.toml`: dev-dependency `semio-framework-value`.
- New testkit guard `Fem3dSnapshotRetiring` (mirror of `Fem2dSnapshotRetiring`): `🧬️schema/📸️snapshot/retiring.rs`, `🧬️schema/📸️snapshot/🦀️.rs`, feature `testkit` in `🧬️schema/📦️packages/🦀️rust/Cargo.toml`.
- io: `🚪️io/🧪️tests/🔬️unit/🦀️.rs`, `📚️examples/{🎬️demo,🌲️concrete-forest,🏠️house}/🧪️tests/🧩️example/🦀️.rs`, `🚪️io/📦️packages/🦀️rust/Cargo.toml` (dev-deps model with `testkit`, `semio-s-fem-analyses-model`).
- viewer: `👁️viewer/🧪️tests/🔬️unit/🦀️.rs`, `👁️viewer/🎭️modes/👁️view/🪟️windows/🧱️model/🧪️tests/🔬️unit/🦀️.rs`, `👁️viewer/📦️packages/🦀️rust/Cargo.toml`.

Other causes (harness, not retirement):
- Result maps are keyed by encoded result-source identity (`scope:1:8:loadCase5:point`), not raw ids: engine, concrete-forest and house tests now look up through `encode_result_source_parts`.
- Buckling tests used a transverse cantilever load, which has no compression and so no positive factors (the 2D twin had the same fix): engine buckling tests now use a compressed fixed-free column, assert the Euler factor within 2 %, and a separate test asserts a transverse load gives no positive factors.
- Viewer scene instance ids are scoped entity addresses; the test now builds the expected id with `fem3d_entity_target`. `semio_plugin::artifact_app_laws` moved to `semio_plugin_testing_laws::artifact_app_laws` (dev-dependency with feature `artifact-app-testing`).
- mesh `move-selection` unit test compared `Float(5.0)` with the language-neutral vector's `UInt(5)`; it now compares numbers by value (`assert_same`). File: `🕸️mesh/🧬️schema/🧬️mutations/🧭️move-selection/🧪️tests/🔬️unit/🦀️.rs`.

## Product bug fixed
- `⚙️engine/🧊️3d/🗺️mesh-preview/🦀️.rs` `fem3d_nodal_von_mises(doc, case_id)` looked up the raw case id in the map that `fem3d_solve_all` keys by encoded identity, so it always failed with `LoadCaseNotFound`. It now encodes `("loadCase", case_id)`. Doc comment of `fem3d_solve_all` corrected (`⚙️engine/🧊️3d/🦀️.rs`).

## Open
- model `original_fem_receiving`: resolved, see counts.
- snapshot-tests (5): `native_tests::{native_owner_receives_actual_persistent_rows_and_closes_every_cancelled_prefix ("native partial template release changed its original Work"), actual_chain_receiving_identity_and_reference_work_scales_below_threefold, receiving_charges_below_sixty_four_kibibytes_of_retained_capacity_per_node, actual_owner_receives_a_ten_thousand_node_fixture_with_bounded_identity_work (all "snapshot cursor original grant cannot admit its frontier": receiving spends ~65 KB capacity and ~230 KB copy per node, exhausting the grants)}`, `load_tests::every_actual_checkpoint_prefix_and_original_grant_denial_closes_without_publication ("native receiving original type, field or phase is invalid")`. Looks like a framework/receiving cost regression, not harness; root cause not found.
- The framework value layer (`🧰️framework/🔨️modules/🌱️value`: persistent list, decode/receive, retirement) has many uncommitted modifications from other agents; the five snapshot-tests failures reproduce deterministically on the final state and point there (per-node receiving cost far above the 64 KiB/node law). Not fixed in this scope.

## Measurements of the 5 snapshot-tests failures (final run `🗑️generated/a3dcore-r4.log`, identical to the first run; 38 pass / 5 fail, 873 s)

All deterministic, `RUST_BACKTRACE=1`. Test file `🧪️tests/native-tests.rs`, `🧪️tests/loads/receiver-tests.rs`.

| Test | Failure | Numbers at failure |
| --- | --- | --- |
| `actual_owner_receives_a_ten_thousand_node_fixture_with_bounded_identity_work` (10000 nodes, copy/capacity/release grant 512 MiB, items 10M) | `receive_authoring` -> `OwnershipLimit "snapshot cursor original grant cannot admit its frontier"` (loop in `🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot/🦀️.rs:376`: no progress, no work, no transition) | copied_items 875090, copied_bytes 536870120 (grant 536870912, only 792 B left), retained 151849822, released 151444152 |
| `receiving_charges_below_sixty_four_kibibytes_of_retained_capacity_per_node` (1000 nodes, capacity limit 64 MiB = 64 KiB/node) | same error | copied_items 374032, copied_bytes 231374922, retained 65522300 (limit 65536000), released 65230592; i.e. ~65 KB retained and ~231 KB copied per node, almost all retained bytes later released (transient) |
| `actual_chain_receiving_identity_and_reference_work_scales_below_threefold` (1024 nodes + 1023 bars, 128 MiB wallet) | same error | copied_items 218305, copied_bytes 134216320 (wallet 134217728), retained 38116024, released 37840160 |
| `native_owner_receives_actual_persistent_rows_and_closes_every_cancelled_prefix` | testkit retire (`🧩️receiving/🧪️testkit/🦀️.rs:31`) -> `InvariantViolated "native partial template release changed its original Work"` | override_progress copied_items 1, copied_bytes 680, retained 0, released 0 |
| `load_tests::every_actual_checkpoint_prefix_and_original_grant_denial_closes_without_publication` | `🧪️test/🛬️original-snapshot-owners/🦀️.rs:82` -> `InvalidValue "native receiving original type, field or phase is invalid"` while closing the decode recipient after a cancelled/denied receive | all progress counters 0, override_progress None |

Observations: the first three exhaust the grant on a different dimension each time (copy, capacity, copy) at only 1000-10000 rows, so receiving of the persistent node list costs ~87-374 items and 50-230 KB of copy per node, far above the 64 KiB-per-node law. A `native-tests.rs` probe at 125..2000 nodes with an unlimited grant (to see whether the cost is linear or quadratic) was written but not run (disk emergency); it was removed again. The 3D `native.rs` receives nodes via the framework `NativeReceiveSlot<Vec<FemNodeValue>, SealedShared<PersistentList<FemNode>>>`; the framework value layer (persistent list, decode/receive, retirement) has many uncommitted modifications by other agents.
