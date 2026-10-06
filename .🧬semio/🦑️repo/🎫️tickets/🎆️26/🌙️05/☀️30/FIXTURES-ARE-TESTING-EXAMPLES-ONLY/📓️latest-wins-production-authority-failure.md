# Latest-Wins Actual Production Authority Failure

The actual helper execution failed in generated/expanded-owner-checks.log before the final inline constant-schema cleanup, and failed again in generated/final-inline-oracles.log. This is an observed earlier task run, not a pre-task baseline claim. The current obligation list and exact predicate match the saved pre-transform consumer bytes: true. The late edits only replace fixture-constant equality validators and preserve ToolLatestWinsScope validation. No production Rust source was edited by that cleanup.

Current actual production source: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs

## Unsatisfied exact body obligations

- dispatch_typed_command_inner requires self.can_admit_typed_operation(operation_id.0); named source body exists=true
- advance_typed_operation_publication_one requires next_id_from(self.typed_publication_cursor); named source body exists=true
- take_typed_operation_result_page requires next_id_from(self.typed_result_cursor); named source body exists=true
- has_runnable_work requires MountedTypedCommandFullOperationStage::AwaitingAck => !self.result_page_presented; named source body exists=true

## Other failing predicate clauses

- publication contains .await

The predicate remains enforced and its hostile token-removal checks remain unchanged; no runtime/native pass is claimed.

## Current Owner Semantics Behind The Literal Mismatches

Actual dispatch uses `self.admit_typed_operation_slot()` in `dispatch_typed_command_inner`; `can_admit_typed_operation(operation_id.0)` occurs elsewhere in ordinary-worker admission. Publication cursor selection is in `advance_typed_operation_publication_unit`, delegated by `advance_typed_operation_publication_one`, and uses the fixed `ARTIFACT_LIVE_OUTPUT_SLOTS` ring. Result selection uses `(0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map` and updates the ring cursor. The existing text helper selects the last `has_runnable_work` method, which belongs to `ActiveArtifactEnvelopeDecodeState`; it does not select the old Mounted AwaitingAck state branch. The publication method is actually async and its body contains await. These are concrete source differences; no native runtime safety conclusion follows without the exact native laws, which are currently blocked by shared compilation failures. The unchanged strict source predicate remains red rather than being relaxed to accept this implementation.
