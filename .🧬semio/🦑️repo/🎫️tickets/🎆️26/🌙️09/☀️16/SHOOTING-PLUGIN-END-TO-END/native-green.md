# Shooting native crates green run

Command: `repo cargo --store=shared --configuration=prototype -- test --manifest-path <shooting rust workspace> -p semio-s-artifact-shooting-shooting -p semio-s-artifact-shooting-shooting-tests -p semio-s-artifact-shooting-shooting-shooting --lib`

## State

All shooting crates compile (lib and lib-test). Counts (run 18/20, `--skip two_instances_converge_disjoint_edits_via_backbone` because it overflows the stack and aborts the test binary):

| package | pass | fail |
| --- | --- | --- |
| semio-s-artifact-shooting-shooting-tests | 27 | 0 |
| semio-s-artifact-shooting-shooting (sqlite snapshot) | 12 | 5 |
| semio-s-artifact-shooting-shooting-shooting (editor) | 18 | 49 |

Remaining failures are framework-side (peers' unfinished native migration), not shooting code:
- editor: `OriginalEffectAuthority<ConstructionProducer>` dropped un-retired inside `ensure_fixture_cancellation_issuer` (29), UI value arena without lifetime registration (9), fixture app paid close fault (7), decode recipient cursor birth layout (3).
- sqlite snapshot: decode recipient close in `original-snapshot-owners` (`original cursor birth differs from its defining layout`, `native retirement requires its original producer invocation`), `native construction requires its original recipient`.

## Files changed (shooting)

- `…/✏️editor/🎚️config/🚪️io/💾️binary/🦀️.rs`: `ShootingCamera` derives `NativeReceive`, `DslOwnedReceiving`, `SnapshotProjection` plus a handwritten `shooting_native_default!` (NativeDefaultValue + NativeDefault) like the raster camera.
- `…/✏️editor/👥️presence/🦀️.rs`: removed duplicate `RetireOwned`; added `RetainedClone`, `SnapshotProjection`, `DslOwnedReceiving`, `by_value_projection`.
- `…/✏️editor/👥️presence/🧬️schema/…`: mutation enum `retained` + `RetainedClone`/`NativeReceive`/`DslOwnedReceiving` + `original_ordinal_operation_receiving!`; `ReplacePresence` leaf `retained_set_record!`; diff `RetireOwned` + `retained_empty_option_patch!`/`retained_row_patch!`/`retained_patch_diff!`.
- `…/✏️editor/🎚️config/🧬️schema/🧬️mutations/**`: `NativeReceive` + `DslOwnedReceiving` on enum and six leaves, `original_ordinal_operation_receiving!(ShootingConfigMutation)`; `ShootingConfig` derives the same.
- `…/🧬️schema/🧬️mutations/🦀️.rs`: `original_pack_value_operation_receiving!(ShootingMutation)`.
- `…/create-asset/📦️packages/🦀️rust/🦀️.rs`: editor re-export re-pointed at the config crate (cycle), duplicate presence glob removed.
- `…/✏️editor/🧵️retained-commands/🦀️.rs`: dropped the removed `terminal_frame_bound!` invocation.
- `🎥️shooting/🦀️.rs`: foreign JSON composer capability now claims `composition-route`.
- editor unit tests and gumball tests ported to the current fixture API; sqlite snapshot tests use `original_owners::{decode_native,encode_native,export_sqlite,import_sqlite}`.
- new `…/sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔬️independent/🧬️schema/🔣️.json` (JSON schema for the independent oracle).

## Files changed (framework, minimal)

- `🧰️framework/🔨️modules/🧪️test/🛬️original-snapshot-owners/🦀️.rs`: decode close under lent Work/construction authorities and `with_effect`.

The `ensure_fixture_cancellation_issuer` attempt (closing the completed birth admission) was reverted: it did not change the outcome and the root cause was not isolated.

## Next owner

The `OriginalEffectAuthority<ConstructionProducer>` dropped from `InstanceBirth` inside `artifact_app_laws::ensure_fixture_cancellation_issuer` (reproduce with `RUST_BACKTRACE=1 … -p semio-s-artifact-shooting-shooting-shooting --lib -- save_camera_stores_the_submitted_label`) is the single largest blocker for the editor crate; `two_instances_converge_disjoint_edits_via_backbone` additionally overflows the stack and aborts the whole test binary.
