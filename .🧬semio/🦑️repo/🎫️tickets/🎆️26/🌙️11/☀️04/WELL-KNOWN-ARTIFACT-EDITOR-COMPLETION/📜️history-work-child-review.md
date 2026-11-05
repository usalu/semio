# History Work Child Review

Read-only review of root's current work patch.

Two concrete child paths invalidate a universal parent last_work=1:

- decode/🧵️edit/🦀️.rs auxiliary_step delegates ErasedSnapshotRetirement.close_step and admit_owned_retirement(meta_partial), selecting only .result. Actual delegated work is lost. Both source child_step and child_close_step can reach this owner.
- decode/⚔️conflict/🦀️.rs:36 auxiliary_step repeats the same .result loss for retirement and message_partial admission.

These actions can carry actual work3 while physical items0, precisely the neutral regression. Parent Child::Edit/Conflict needs the real child's reset/selected/delegated work port; replacing the whole call with1 is not truthful.

Parent step_result currently catches an error and sets last_work=1 when it was0. That fabricates work for pre-action errors, including auxiliary control.charge refusal before an actual retirement/admission attempt. Remove the fallback inference; explicit action sites or genuine nested outcomes must determine work.

Validation child step is a single explicit step_inner after its own source/state/receipt/grant gates; its close is a single clear frontier. However no blanket approval is made for all nested children: the concrete Edit/Conflict delegated ownership paths above must be repaired first.

No code changes, compiler runs, or repeat neutral tests were performed.
