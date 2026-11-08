STATUS: GREEN lib. semio-framework-replication-retained: wasm32-wasip2 rc 0 (W-replretained-2), native rc 0 (replretained-3). 162 errors -> 0. Test targets NOT touched/compiled.

Crate was not splittable further; done in one lane (span, patch, patch/apply, diagnostic, set-field, diff, diff/sequence, root).

API (callers in plugins/layout/draw/idml/wfc/remodel/energy must adapt):
- RetainedMutationPreparation / RetainedMutationLeafPreparation: decide/inverse take `effect`, begin_close_paid(effect), close_step(context, effect)->OriginalRetirementPoll.
- RetainedMutationCloseContext.mutations is now `&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>` (FactoryVectorRetirement needs the Arc); issuer identity (data ptr + TypeId) restored locally in the root after upstream removed it.
- begin_retained_mutation(max_work, effect); decide_retained_mutation / inverse_retained_mutation / owner close_step on effect.
- RetainedRowPatchFields (macro retained_row_patch!), RetainedRowPatchApply::stage/close_step, RetainedDiagnosticPreparation::advance(_targets)/close_step, RetainedByteSpan::advance/close_step, RetainedDiffApply::{preflight(.., effect, receiving), commit(.., effect, cap), close_step(effect)}, preflight_retained_diff/commit_retained_diff, RetainedDiffSequence::{preflight_demand(+max_work), preflight(+max_work, effect), commit(effect), close_step(effect)}.
- ProjectionField no longer has begin_close: the sequence starts projection close with begin_close_paid inside close_step.
- Shared helpers: retained-field now exports owe/settle/settle_exit/left; this crate adds settle_value/settle_advance (single charge: children record on the effect, the owner records only the owed remainder; work refusals merged, never dropped).
- Pending from a child during preflight (not close) becomes a typed WorkLimit refusal carrying its work.
- expect removed in root/diff/sequence/diagnostic paths touched; remaining legacy expects live in untouched quote fns of other files (set-field/diagnostic demand fns, sequence take_messages).
Not ported: stale macros `retained_set_field` (uses old `source`/`original` trait methods, pre-existing), tests under 🧪️tests, `exchange` of RetainedRowPatchApply (pure grant-based, charged by commit).

## Round 2 (coordinator order, main-147)
GREEN wasm32-wasip2 (W-replretained-5 rc 0) and native (replretained-6 rc 0): semio-framework-surface-node-graph (18) and semio-framework-time-travel (16). Test targets not compiled.
- node-graph scene: SceneDecodeCursor::new(.., effect), advance(units, cancelled, effect) (own units not recorded by pack children are paid on the effect, never twice), close_step(effect)->OriginalRetirementPoll via RetirementPollExit::settle; payload RetireOwned via an internal PayloadOwners record (viewport is foreign Copy data); encode pause/resume errors keep their ValueError.
- time-travel: TimeTravelChoice/TimeTravelEvent now derive RetireOwned (manual macro deleted); TimeTravelDiscardCursor and TimeTravelCommandCustody close_step(effect)->OriginalRetirementPoll (trait impl delegates).
- Tests under 🧪️tests (call close_step(grant)) are stale and unported.
