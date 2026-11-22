# Plugin Tests 08 Repair

Crate `semio-plugin-tests-08` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🍓️tests-08`), tests `plugin_builder_contract_tests_1` (`🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-1` plus `…-part-1-x/*`) and `bounded_initializer_laws`.

Result: `repo cargo --lang=en --store=shared --configuration=prototype --parallelism=2 -- check --tests --manifest-path <root>/Cargo.toml -p semio-plugin-tests-08 --lib` finishes with 0 errors (was 53, then 55 after the first wave unmasked body errors). The new `retained_window_input_recursive_document_archive_rejected_history_frame_ends_in_a_typed_fault_with_terminal_progress` compiles.

## Where each API moved

| Old use | Current expression |
| --- | --- |
| `TestMembers` | `semio_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::TestMembers` (imported in the part-1 root) |
| `ArtifactOwnedDisposer`, `bounded_document_store_disposer` | `semio_plugin_ownership::store_lifecycle` |
| `semio_framework_schema_state`, `semio_framework_schema_composition` (named by the `ArtifactSchema` derive) | dev-dependencies `semio-framework-schema-state`, `semio-framework-schema-composition` in the tests-08 `Cargo.toml` |
| `semio_plugin::Fault` | `Fault` (already imported) |
| `build_document_store_initialization_job(envelope, replacement, actor)` | `(envelope, operation, generation, actor)`; `generation.0 + 1` is the candidate generation |
| `RecursiveFixtureMutation: OpBinaryReceiving` | pack-value `OpBinary` plus `semio_os_kernel::original_pack_value_operation_receiving!` (the ad hoc `0x72` + `i32` bytes are gone) |
| `store::mutation_apply_preparation_factory` (needs `RetainedMutation`, which needs the `retained` derive) | `store::cold_mutation_apply_preparation_factory` and `ColdMutationApplyFactory` quote, bounds `Mutation<P> + ColdMutation<P>` |
| `recursive_fixture_snapshot!` (private macro left in `🔬️…-contract-x`, unreachable from tests-08) | moved next to its only use in `🧵️recursive-wire-open-and-producer`; now also declares `sqlite_snapshot_codec` and `member_store_owners_copy_byte_demand`; dead copy removed from `contract-x` |
| `MemberSnapshotOpenOperation::step/take_ready` | return `Result<_, OriginalTurnFailure>` (bodies kept as `advance`/`take_ready_snapshot`) |
| `ErasedSnapshotRetirement::close_step(grant)` | `close_step(body, &mut RetainedAllocationEffect) -> OriginalRetirementPoll`, ported with `RetirementPollExit::{resume, settle}` |
| `ArtifactEnvelope*FieldAuthority::close_step(grant)` | `close_step(&mut RetainedAllocationEffect) -> ArtifactEnvelopeClosePoll` via `artifact_envelope_close_settle` / `_resume_with` |
| `close_original_retained_step(cx, demand, \|grant\| …)`, `artifact_retirement_box_close_step(slot, grant)`, `runtime.settle_current_retirement_step(grant)`, `owner.step(grant)` | `close_original_retained_effect_step` / `close_original_effect_step` with `effect.nested(child)`, `settle_close_poll`, `with_original_effect`, `owner.step(body, effect)` |
| `plugin_render_surface(runtime, id, surface)` returning the tree | `(…, receive: &mut dyn FnMut(&mut Option<ComponentTree>, cx), cx) -> Option<Vec<PresenceUpdate>>`; the test takes the tree in the callback and releases the render with `close_render_work_request` |
| `app.composition`, `app.child_content_root` | `app.child_composition.composition`, `app.child_composition.child_content_root` |
| `test_child_admission_state` (`#[cfg(test)]` in `🧵️instance/🪆️children/📥️staging/🚪️ingress`, so invisible to every test crate) | gated `#[cfg(any(test, feature = "artifact-app-testing"))]` like its `ChildAdmissionTestState`; the `artifact-app-testing` feature of `semio-plugin-instance-children` now forwards to `semio-plugin-child` |
| `HistoryTransitionRecord.payload: Vec<u8>`, `DocumentArchiveLoadStatus.fault: Vec<u8>` | `CausalPayload::try_from_slice(&[0xff])`, `!status.fault.is_empty()` |
| trait methods `set_owner`, `envelope_pack_bytes`, `ColdMutation::diff`, `ArtifactReferenceText` | traits imported (`ReferenceSpaceMember`, `ColdMutation`, `ArtifactReferenceText`) |
| `RecursiveBranchSnapshot`, `RecursiveBranchDiff`, `ComposedParentDiff` | `pub` + `#[doc(hidden)]` (private-in-public is now an error through `SnapshotOpen` and `Mutation::Diff`) |

## Behavior changes found while running (assertions kept or strengthened)

- A registered member must already carry the exact parent owner (`child member does not match its exact admitted identity and ownership`). Fixtures now `set_owner` before `register_child`. `member_factory_closed_dialect_fresh_register_and_restore_publish_exact_parent_owner` additionally asserts that an owner-less member is refused (message, generation 0, member returned), and restores through an envelope owned by the restoring parent (a different parent id is refused by design).
- Close helpers for recursive parent and member fixtures use the mounted policy close grant; the 4096 copy-byte maintenance grant no longer covers a store close demand (4672).
- `composed_maintenance_turn` goes through `original_lifecycle_step` like `app_maintenance_turn`.
- `reserved_section_surfaces_render_accessor_maps_instead_of_app_bodies` now runs under an `OriginalFixtureCaller`; its assertions are unchanged (one more on the funded render turn).

## Run status of the touched tests (`cargo test --lib`, filter member_factory / reserved_section / retained_window_input_recursive / original_initializer)

Pass: `original_initializer_job_keeps_source_authority_cancel_arc_and_full_receipts`, `member_factory_closed_dialect_open_failure_retains_pin_and_drains_exact_member`, `member_factory_closed_dialect_fresh_register_and_restore_publish_exact_parent_owner`, `member_factory_parent_snapshot_restore_matches_neutral_corpus`.

Still failing at run time (compile is clean; not an API-port problem):

1. `reserved_section_surfaces_render_accessor_maps_instead_of_app_bodies`: `contract_app_raw` panics with `tool factory key 's.test.synthetic@1/*#editor/setActiveUtility' is already registered` (tests-05 `TestApp` tool registration against the reserved producer).
2. Five `retained_window_input_recursive_replacement_*` and `…_document_archive_*` tests: the replacement job never leaves `Initializing` (the fixture initializer is never stepped; the maintenance ladder answers zero-item progress under the fixed `COMPOSITION_MAINTENANCE_GRANT`, quoted `ladder demand` 39712 capacity / 0 copy fits) and the archive load never reaches a terminal state. Needs the replacement-job admission/currency path of the real replay initializer (`build_candidate` with `admit_original_construction`, `next_step_demands`, `bind_replacement`) mirrored in `ReadyComposedParentInitialization`, which is a framework-semantics task, not an API rename.
3. `…rejected_history_frame_ends_in_a_typed_fault_with_terminal_progress` (new, Nov 22): compiles; fails in the same archive load driver, so it is red before reaching its fault assertions.

## Preservation

`📦️units/📋️preserved.json` re-hashed with `plugin-units-preserved-check.ts --write` (three law bodies changed: the two above plus the new archive test body; assertions only grew: 7 to 9 and 8 to 9, total 1169 to 1172 recorded). `📋️surface-carriers.json` re-hashed with `plugin-units-carriers.ts --write` (3 bodies, no assertion drop). `repo test-file …/📦️units/🟦️.ts`: 9 pass, 2 fail; the two failures are the pre-existing stale `Cargo.lock` / feature-verdict dependency-list items from `plugin-units-preservation.md`.

## Files

Edited: `🔬️…-part-1/🦀️.rs`; `🔬️…-part-1-x/{🧱️composed-parent-snapshot-and-recursive-mutation,🔓️decode-authority-and-field-decoders,🌳️field-catalog-parent-app-and-branch-snapshot,🧵️recursive-wire-open-and-producer,🚀️ready-parent-initialization,🔧️replacement-fixtures-and-recursive-documents,📦️document-archive-and-replacement-publication,🗂️recursive-archive-rejection-and-stale-authority,📨️envelope-decode-ladder}/🦀️.rs`; `🔬️…-contract-x/🦀️.rs` (dead macro removed); `🌱️initialization/♻️retirement/🧪️tests/🦀️.rs`; tests-08 `Cargo.toml`; `🧵️instance/🪆️children/📥️staging/🚪️ingress/🦀️.rs` and its `Cargo.toml`; `📦️units/📋️preserved.json`, `📋️surface-carriers.json`.
