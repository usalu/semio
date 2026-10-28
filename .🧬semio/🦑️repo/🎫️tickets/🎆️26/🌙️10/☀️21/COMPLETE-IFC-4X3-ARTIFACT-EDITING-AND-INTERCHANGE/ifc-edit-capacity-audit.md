# Retained Snapshot Edit Capacity Audit

Read-only source trace. Important correction: current one-invertible-item command capacity is a semantic underdeclaration for multi-primitive type switches, but source does not establish that it makes those batches unpublishable. Do not report an observed Store failure without an actual retained regression.

## Exact Authority Paths

Shared editing tool-job-factories snapshot_edit_extent returns SNAPSHOT_EDIT_WORK_CAPACITY.rows_for_items(1), and build_snapshot_edit_tool_job passes that constant work_items as payload maximum. ArtifactRetainedWorkCapacity in plugin/🧵️retained-command/🔖️checkpoint-and-work-contract/🦀️.rs defines rows as two per point-invertible item and rows_for_items refuses zero/overcapacity. Retained-command-job Preflight lines528–535 only checks nonzero extent<=maximum_work_items. Complete emission then moves through completion.complete; this job has no emitted mutation count check.

Actual mounted publication authority is plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs lines275–281: independently calls ArtifactStoreOneItemFootprint::for_gesture on actual emit.artifact_mutations, then store.admit_apply_batch. Command extent/ceiling is not passed there. Store cursor admit_apply_batch in store/⚙️engine/🧱️store-components/📥️lifted/🏪️store/🖱️store-cursor-operations/🦀️.rs captures actual original mutation vector and admitted_items=mutations.len(), beginning footprint at zero for source census. Source publication merges per-item footprint. Thus Store uses actual emitted leaf declarations, not the command's constant2.

Store/🎮️one-item/🦀️.rs for_leaf derives rows=1+mutation.inverse_rows(); for_gesture sums these actual declarations and rejects above65536. Retained bytes ceiling1048576 is independent. Store fold and PreflightingCommit enforce actual forward+inverse count against measured gesture footprint. Existing store batch-publication-and-stamped-clock-laws tests explicitly prove multi-item gesture-wide merged accounting.

## Correct Hook Scope

Add artifact-declared maximum capacity and cheap admitted command extent hooks to SnapshotEditingEditor, using defaults identical to current ordinary editors. Use same artifact declaration for payload maximum and work extent; update factory bounded proofs only if changing execution contract. Do not change raw-wire maximum, publication contracts, original completion or Store footprint issuers to solve the semantic extent discrepancy. Count rows, not merely mutations: point-invertible assumption needs proof for each emitted leaf; aggregates can declare more inverse rows. A safe fixed maximum is acceptable for admitted bounded gestures when exact count cannot be known cheaply, provided produced rows are checked against the declared command extent/maximum if the hook is intended as an enforceable semantic cap.

Terminal frame release remains size_of SnapshotEditWork if hooks add no heap state; depth1 work demand and consumed guard must remain unchanged. New stored planner state would require corresponding real close ownership demands rather than merely increasing semantic rows. Original caller fuel/cancel and final guards around publication validation remain required.

## Current Publication Precheck Limits

Shared check_publication_limits only encodes each forward and each mutation.inverse(original snapshot), checking per-item bytes. It does not check batch count/total rows and does not evolve the base between dependent mutations. It is not the final Store inverse authority. New planners that emit create-then-edit or multiple dependent replacements must test this precheck with the actual sequence; do not bypass a refusal or assume independently computed original-base inverses are a valid batch undo. Actual Store preparation must retain sequential inverse authority and reverse undo order.

## Required Retained Proof

Test a real IFC details type switch emitting at least three primitives through registered SnapshotEditToolJobFactory, original command wire/decode/preflight/contextual work, mounted completion and Store admitted batch to terminal publication. Assert exact resulting entity, one history gesture, actual forward/inverse row count, and exact full snapshot restoration by undo. Include expansion and shrinking tuple switches, not only equal-length name/slot changes. Drive terminal close and assert empty original owners. Also assert cancel/fuel exhaustion leaves original snapshot/history unchanged, stale base refusal, declared-capacity+1 refusal, and unchanged ordinary single scalar editor behavior. Emit/apply unit tests alone do not exercise this path; a retained job completion collector without Store mounted publication proves only the job half.

No native retained execution occurred in this audit; main/High own test implementation and validation.
