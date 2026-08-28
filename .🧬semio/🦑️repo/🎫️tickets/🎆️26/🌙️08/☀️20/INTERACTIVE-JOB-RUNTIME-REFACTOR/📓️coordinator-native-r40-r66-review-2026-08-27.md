# Coordinator Native Prerequisite Review R40–R66

## Observed Evidence

Read complete native ActorBytePageR2 output: three passed,105skipped,0.084s. Neutral fixed storage and padding only; no returned-page authority or allocation permission.

Read complete handbackR40 output and source: three held-registry/poison laws passed,101skipped,0.056s, after actual REDs. Busy entry uses try_lock, poison is a typed fault, and queued state remains in its original registry slot while one step executes. Resident return errors propagate. Normal Drop/admission helpers and old tree retirement remain separate open boundaries.

Read complete R41/R65 reports and actual raw result lines: runtime101passed/3explicit exclusions0.665s; UI contract155passed/0skipped0.737s with75oracle checks. This is not a full runtime pass. Excluded original failures remain:

- surface_ownership_inline_fields_do_not_allocate_a_second_owner
- surface_ownership_existing_component_refuses_before_cloning_unadmitted_payload
- surface_ownership_existing_component_retains_comparison_and_copy_between_turns

Read R42 report and actual compile completion lines: existing runtime check-wasm succeeded for wasm32-wasip2 and wasm32-unknown-unknown (11.24s and3.80s). No consumed component execution or Plugin/WIT producer readiness follows. UI wasmR66 is executor-reported, not independently rerun by root.

## Continuing Single-Authority Cutover

Read the full canonical-runtime handoff. Native executor continues replacing production retained/new_retained payload maps with canonical UiDocumentLease/Assembly; id-to-ordinal metadata alone may remain. The pre-admitted permit must move into the assembly before payload exposure, and the real simultaneous-owner census must precede shrink/seal/output split. Both main SurfaceReconcileJob and transaction.rs::reconcile_tree require adoption. Transacted raw patches must retain paired output authority through publication and close, not hide a second permit in the root. Kernel overlap is explicitly coordinated with Dag.

The64slot static arena metadata needs explicit once-only accounting. The three original REDs, real nine-reconciler coexistence, reader pressure, Process workshop, physical32KiB/component4KiB/surface8MiB/aggregate32MiB and final strict8ms verdict remain acceptance obligations. No source/output cleanup, replacement publication, quota increase or native compile was performed by coordinator.

