# IFC 4x3 `any` Plugin: Runtime Fixes, Round 2

Package `semio-s-artifact-stdio-ifc-4x3`, `cargo test --lib`, native cargo (`ifc-cargo.sh`, `ifc-one.sh`, `ifc-retry.sh` in this folder set only the native env, a log filter and a retry loop; other agents' mid-edit framework, pdf and job-testing crates broke the build several times and were waited out, never touched). Final log: `🗑️generated/ifc14.txt`.

## Result

| Run | Pass | Fail |
| --- | --- | --- |
| Before (`stale-ifc8.txt`) | 42 | 38 |
| After (`ifc14.txt`) | 50 | 30 |

The error `original projection must be driven or returned through its resident frame` is gone from every path. No assertion was weakened, no budget or stock was raised. Fixed tests: `binary_select_values_...`, `complex_component_type_controls_...`, `type_switches_preserve_identity_...`, `official_4x3_tessellation_...`, `native_factory_transfers_original_payload_...`, `prepared_native_source_cancel_and_wrong_revision_...`, `prepared_native_source_actual_original_endpoint_...`, and the other admission-only cases. 30 remain, grouped under "Open items".

## 1. Admission frame (done)

Root cause: `StagedSourceAdmission::borrowed(lease)` stops at phase `projectionStart`. The framework requires its caller to keep an `OriginalProjectionFrame` and the unsettled `OriginalEffectAction` resident between turns and to drive them stage by stage (`projection_selection`, `projection_step`, and for `Drive` the frame's own `authority().drive(..)` under the Source effect). `SnapshotReadSourceStaging` in the framework is the correct caller; IFC had none of it.

Fix: new product type `SourceStaging<T, A>` (`🔎️details/📨️wire/source_staging.rs`) owns the admission, the frame and the action. It keeps the surface the five call sites already used (`borrowed`, `next_demand`, `advance`, `close_demand`, `close_into`, `terminal_is_empty`) plus `next_work_demand`, `close_work_demand`, `original_authority`. `advance` quotes and funds every projection stage (Begin, Drive, Transfer, Accept, Finish), hands a recorded projection refusal back as the admission error, otherwise delegates to the plain admission. `close_into` settles an open projection (transfer, accept, discard, or return the unanswered owner) before returning inputs. It implements `RetireOwned` through `SourceStagingCursor`, so the derived retirement of `SourceLoan` handles a frame in flight. Verified in the log: Begin, Drive, Transfer, Accept, Finish and the final clear run in order under the Source effect.

Call sites switched: `source_loan.rs` (`SourceLoan`), `🔎️details/work.rs`, `✍️apply/work.rs`, `✍️apply/staging-native/🦀️.rs`, `👁️mvd/🧩️concepts/population.rs` and `preparation.rs` (the mvd crate now depends on `semio-s-artifact-stdio-ifc-details-wire` and re-exports its `funded_turn`, removing the duplicate).

Further product defects exposed once the path ran (all fixed):
- `🔎️details/🧶️guidance/type_guidance_request.rs` (`RequestCopy`, owned admission): phase `projectionStart` now runs `project_owned` under its quote; the admission is dropped only at `terminal_is_empty` (it was dropped with custody unpaid); `request()` also sees the donor while it is inside the admission (guided `context_matches` was refusing `staging.stale`); the cancel path discards the projection first; phase 5 closes the source with `close_step` until terminal instead of asserting it already is.
- `🔎️details/🔤️type/constituent_roster.rs`: the retyped-tuple read matched the primary entity too (its `origin` is 0, so `guard_part == origin + 1` hit the first complex part); now `!primary`, like the neighbouring `renaming` guard.
- `🔎️details/🧭️profile/retained.rs`: `receive_effect` asked for `maximum = cx.fuel_remaining()`, but the receiving loan lends only what remains after reserved return work (framework log: `denied work quote 1000 observed=0 available=Some(500)`); it now uses `with_native_receiving_work` and the lent `units`.

## 2. Admission close (done)

The close ran in a plain `with_original_effect` (`closed_in`), while the admission birth is authenticated against the Source effect. New `closed_in_source` (`source_loan.rs`, re-exported by `✏️editor/details.rs`) runs the quoted close under `with_original_source_effect` and settles through `receive_original_retained_work`. Used by `IfcTemplateSourceWork::close_step`, the template apply work and `IfcDetailsWork::close_step`. `prepared_native_source_cancel_and_wrong_revision_keep_output_unpublished` passes; `prepared_native_render_owner_retires_the_same_donor_at_partial_frontiers` still reports the birth mismatch (open item 4).

## 3. UI tree serialization (rewritten, same expectations)

`serde_json::to_string(&BuiltNode)` is refused by design for populated children. New test helper `✏️editor/tree-probe-tests.rs` (`built_tree_serialization`, `retired_tree_serialization`) serializes every node's key, component, accessibility, bindings, menu and packed text depth first, then retires exactly that tree with `BuiltTreeRetirement` so the UI arena closes. The `contains(..)` expectations of the three tests are untouched. `official_4x3_tessellation_...` passes. `revisioned_host_render_...` and `native_view_variants_...` still fail (open item 3).

## 4. Individual assertions

| Test | Root cause | Status |
| --- | --- | --- |
| `binary_select_values_...` (tests.rs:356) | `IfcDiff::apply` validated indices only, so the invalid Part 21 binary `"3F"` (3 padding bits over a non-zero nibble) was applied. | Fixed: `IfcValue::binary_literals_admitted` (`📸️snapshot`) plus `validate_entities_diff` / `validate_args_diff` (`🔺️diff`) refuse modified, added and entity-added binary literals with `mutation.apply.invalid-binary-literal`. |
| `complex_component_type_controls_...` (tests.rs:436), `type_switches_...` (tests.rs:108) | Variant label and `$ifcType` used `declaration.name` (`IfcWall`); STEP, tests and fixtures use the catalog key (`IFCWALL`). | Fixed in `🔎️details/👁️provider/🦀️.rs`. |
| `native_factory_transfers_original_payload_...` (apply/tests.rs:177) | Followed the admission fix. | Fixed. |
| `..._text_birth_denies_each_independent_axis` (staging-native/tests.rs:48), `..._clone_error_forwards_partial_effects_...` (:52) | Receipt and context accounting of `receive_original_retained_work` differs from the test's grant model. | Open (item 4). |

## Open items (30 failures, evidence only; nothing guessed)

1. Retained edit family, 9 tests: `actual_publication_wire_...`, `constituent_domains_...`, `contextual_derived_transitions_...`, `domain_insertion_...`, `every_publication_turn_cancels_...`, `every_reduction_cancellation_prefix_...`, `header_tuple_...`, `same_primary_empty_global_id_...` (tests.rs:708) and `oversized_exact_inverse_...` (:735). The edit stalls at the domain stage (stage `W:ifc-edit-domain`, 99 422 pending turns of 100 000). Arithmetic from the instrumented run: `DomainWork::demands()` = 4 * `size_of::<DomainWork>()` + 4 * 256 + 256 * `size_of::<usize>()` = 4 * 37 312 + 1 024 + 2 048 (+1) = 152 321 bytes, against the fixture loan `source-owners.json` `contiguousOwnerLoan` = `maximumItems 1`, `maximumCopyBytes 32 768`. `DomainWork` is 37 312 bytes because it embeds `DomainCursor` (fixed `LIMIT` frame and node arrays); nothing under `📚️catalog/⚖️domain` differs from HEAD. A fix needs a design decision: shrink the quote (page or box the cursor under the paid-birth rules) or raise the language-agnostic fixture to at least 152 321 bytes. Not done, per the no-unproven-budget rule.
2. Guided-type controllers, 14 tests (`type_*_guidance`, `guided_controller_builds_...`, `required_type_*`): the admission, request copy, source loan and body build now run; they end in their draft assertions (`"missing"`, `"point-one-dimension-rule"`, `"real-missing"`, `"unit-subtype"`, `turn < 999999`) or in the constituent roster (`IFC primary payload owner occupied`: a value cursor reassigned in phase 20 is not terminal-empty at phase 14). They need a separate pass through `constituent_roster.rs` and the guidance phases.
3. `revisioned_host_render_...`: the probe works, but the editor test's details tree contains only the `schema` row (583 bytes), so `Coordinates` is absent (the mvd tree serializes at 15 453 bytes). `native_view_variants_...` still ends with `UI arena descendants remain borrowed at lifetime closure`.
4. Staging-native: `prepared_native_render_owner_retires_the_same_donor_at_partial_frontiers` (`admission birth changed its original Work or Source` from the fixture root retirement), `prepared_native_partial_seed_expired_deadline_...` (`original Source clock declined new construction`), `prepared_native_identity_drafts_...` (`UI value arena source is unavailable`), and the receipt-model pair above.

## Files changed

Product (IFC details crates, under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details`):
- `📨️wire/source_staging.rs` (new), `📨️wire/🦀️.rs`, `📨️wire/source_loan.rs`
- `work.rs`, `🧶️guidance/type_guidance_request.rs`, `🔤️type/constituent_roster.rs`, `🧭️profile/retained.rs`, `👁️provider/🦀️.rs`

Product (IFC 4x3 and 4 model, under `.../🏗️ifc/🏅️standards`):
- `🔖️4x3/🪆️subsets/✳️any/✏️editor/details.rs`
- `🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏷️templates/✍️apply/work.rs`, `✍️apply/staging-native/🦀️.rs`
- `🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts/lease.rs`, `population.rs`, `preparation.rs`, `👁️mvd/📦️packages/🦀️rust/Cargo.toml` (and the workspace `Cargo.lock`, updated by cargo)
- `4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`, `4️⃣4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`

Tests (rewrites only, expectations kept):
- `🔖️4x3/🪆️subsets/✳️any/✏️editor/tree-probe-tests.rs` (new), `✏️editor/🦀️.rs` (module line), `✏️editor/tests.rs`, `🧬️schema/👁️mvd/header-editor-tests.rs`

Ticket folder: `ifc-cargo.sh`, `ifc-one.sh`, `ifc-retry.sh`, this file, `🗑️generated/ifc14.txt` (final log; older logs deleted). All `[DEBUG]` diagnosis prints were removed again. The test binary of the build was deleted.
