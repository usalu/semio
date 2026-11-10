# retained: semio-framework-replication-retained (reply-contract port)

## Step 1 (in progress)
- Claimed in 1105b-claims.md.
- Design (debt, see below): decide/inverse/close stay synchronous in this crate; every RetainedFieldPreparation child is run via `retained::native_child(effect,&mut work,|e,reply| child.xxx_into(..))` (reply left Pending = invariant error). Work performed by the child is added to the caller total so `settle`/`owe` stay exact.
- `close_step` of every owner/trait (Owner, LeafPreparation, MutationPreparation, DiffApply, RowPatchFields, Span, Diagnostic, Diff owner, Patch apply, set-field leaf) now takes `body:usize` before `effect`; demands use `body` instead of `grant.maximum_copy_bytes`; `RetainedCloneClose::step_granted(body,..)`, `RetainedCloneBinding::close_one(binding,body,..)`.
- set-field `check_paid` now takes the effect (`check_binding_paid(&binding,effect)->Option<bool>`).
- Done so far: span, patch(+apply+macro), diff owner, diagnostic, set-field, lib. TODO: diff/sequence (ProjectionField/OriginalVectorConcat/OriginalPostRead new APIs), compile, tests.

## Step 2 (lib green)
- native lib W-retained-3 rc 0, wasm32-wasip2 W-retained-4 rc 0.
- sequence (🔺️diff/🪢️sequence): `intermediate`/`admission` removed; new `previous:OriginalPostRead<P::Output>` replaces the intermediate source; `candidate_read` owns the candidate via `capture_into`; after each ordinal `previous` returns its authority (prepare Mutable, return_into, retire via candidate_retirement) and `previous`/`candidate_read` swap. Projection field is closed in new phase 13 (take_output no longer releases it). Messages concat `begin_close_into` is paid inside close_step (RetainedDiffSequence::begin_close no longer touches it).
- DEBTS: (1) decide/inverse/close are synchronous with native-only children (`native_child`: Pending reply = invariant error); non-native (original-controller) field/projection children need Flight-routed reply-shaped parents (trait signatures would change). (2) `RetainedMutationAdvance` kept. (3) Tests (all 🧪️tests/**) still on the pre-733 effect helpers; not yet ported. (4) downstream expanders of retained_row_patch! / consumers (spr, replication re-export, paged-one-item mutation-apply) unchecked.
Rerun: chk.sh W-retained-N 🌎️hub/Cargo.toml native -p semio-framework-replication-retained

STATUS: BLOCKED tests not ported (lib FIXED-verified W-retained-3/4)

## Step 3 (reply-shaped, coordinator ruling)
- Native lib W-retained-6 rc 0. Wasm W-retained-7/8 fail only in peer crate value-native-schema-engine (`dbg_push` missing in native_decoding).
- native_child is now used only for native-completing children (take_into, begin_close_into, capture/return/push/finish/take); every pending-capable child (field/projection/post-read prepare, concat close, child diff owner, retirement step) runs under `retained_routed!` with the owner's `Flight` + recover entries; poll children via `polled`.
- Owners reply-shaped: Owner (decide/inverse/close), LeafPreparation/MutationPreparation traits, SetFieldLeaf, RootMutation, RetainedDiffApply trait, PatchDiffApply, RowPatchApply (stage/close), RowPatchFields macro, DiffOwner (preflight/close), DiagnosticPreparation, DiffSequence. See INTERFACE in claims log.
- Debts: tests (all 🧪️tests/**) not ported, no Pending-path test exists; sequence close/preflight pending paths unexercised; binding `close_one` kept as poll (native alias release).

STATUS: BLOCKED wasm verify waits on peer value-native-schema-engine; native FIXED-verified W-retained-6

## Step 4 (facade + actor-retained-context)
- Facade `semio-framework-replication` and `semio-os-spr` re-export the reply-shaped names; their verification (W-retained-9) is blocked by peer reds (ui-contract-values, history-ledger/vcs/channel; lock-123 pending).
- `semio-framework-actor-retained-context`: Source birth is staged through `OriginalCallerSourceAdmission` (new field `source_admission`; stages prepare / admit_backing / publish, quotes per stage); Source/payload/cancel close take an effect; `OriginalSourceCloseFailure` -> `OriginalActorContextError::Original`; Source Pending poll -> typed refusal (close_inner is not reply-shaped: debt). Added additive `pub fn with_detached_original_effect` at the end of `🧵️job/⏱️context/🦀️.rs` (effect without a StepContext).
- Native W-retained-11 rc 1, stopped upstream in semio-framework-job-native-turn (birth/🦀️.rs:116, driver lane); the crate's own errors are unknown until that compiles. Wasm unrun.
- Debts: `admission_actual` is now dead state; bootstrap_birth_reserve work sum approximate; actor-retained-context tests not touched; mutation-apply callers belong to mutapply.
- Rerun: chk.sh W-retained-N 🌎️hub/Cargo.toml native -p semio-framework-actor-retained-context (then wasm), then -p semio-framework-replication -p semio-os-spr.

STATUS: BLOCKED native-turn upstream red (retained lib FIXED-verified native W-retained-6; facade, actor-retained-context unverified)
