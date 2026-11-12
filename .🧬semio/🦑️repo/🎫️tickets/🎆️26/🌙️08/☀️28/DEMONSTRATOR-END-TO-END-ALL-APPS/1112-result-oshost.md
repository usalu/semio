# oshost lane (11/12) result

## Step 1: semio-os host workflow-store
- Root cause: `WorkflowMutation: OpBinaryReceiving` missing (`StoreMutationFor`). A DslReceiving port was tried and reverted: the workflow graph carries hand-crafted DslField records (WorkflowMediaPort, MediaContract, WorkflowInput over foreign enums) with no receiving cursor.
- Fix: `WorkflowMutation` persisted binary op is now the pack-value encoding (`OpBinary` via encode/decode_pack_value, like InteractionConfigMutation/SpaceHistoryMutation) + `original_pack_value_operation_receiving!` + `artifact_persisted_binary_operation!` in workflow `🚪️io/💾️binary/🧬️mutations/🦀️.rs`. Greenfield: persisted workflow op bytes change.
- W-oshost-4 hub wasm `-p semio-framework-artifact-workflow-workflow-schema` rc 0.
- W-oshost-5: `-p semio-os` left 2 errors in host `backbone-document` (`close_step(grant).result`). Fixed with new store fn `close_unscheduled_retirement` (owners.rs, OriginalRetirementPump driven, no AuthoringWallet) used by `close_backbone_envelope`.

## Debt (per coordinator: not ported, not in demonstrator closure)
- draw drawing-model (68 errs) and layout-model/layout-diff-plan sqlite snapshot + retained-clone callers: receive_pack, `*_sqlite_snapshot_*` 4/5-arg signatures, `.result` on polls, retained_clone::paged import.
- stdio snapshot crates owned by lane stdiosnap.

## Browser target (wasm32-unknown-unknown)
- chk.sh only knows `native` and `wasm` (= `--target wasm32-wasip2`, via `slot.sh ... repo cargo --store=shared --configuration=prototype -- check --manifest-path M $T --keep-going`). No ticket script checks host-base, the wgpu shell or `wasm32-unknown-unknown`.
- Exact command needed (not run): `slot.sh <label> env SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path <manifest> --target wasm32-unknown-unknown --keep-going -p semio-os`, i.e. chk.sh with its `T` selection extended by a third kind `browser` -> `--target wasm32-unknown-unknown`.

## Verified
- W-oshost-6 hub wasm `-p semio-os` rc 0.
- Not verified: wasm32-unknown-unknown, native check of semio-os, tests.

STATUS: FIXED-verified W-oshost-6
