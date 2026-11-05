# Retained Semantic Preparation Reuse

Read-only source audit, 2026-11-04. No tests/builds or production changes. Scope is actual norm artifact readiness from owned inspector input.

## Finding

There is a reusable retained private-projection and semantic engine. There is no discovered existing factory that also constructs the norm typed mutation from owned inspector action arguments and supplies genuine retained norm leaf semantics. Borrowing a cached answer, or putting synchronous resolve_norm_edits behind a guessed receipt, would not fill this seam.

## Reusable Metadata-Free Engine

Base path: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation`.

`🧬️native/🦀️.rs:19` declares `ArtifactReplayPreparationRequest<P,M:Sync>`: owned `SnapshotRead<P>`, owned `RetainedOwnedProjection<M>`, replay context, and original snapshot/mutation retirement factories. It has no publication actor/edit/sealer metadata.

`NativeSemanticPreparation<P:RetireOwned+Sync,M:RetireOwned+Sync,C:SemanticProjection<P>,E:SemanticEdit<P,M>>` owns source capture, projection, private post, original input, semantic edit, inverse rows, messages, resulting Arc and cleanup. Public APIs: `new(request,projection,edit)`, `demands(body)`, `advance(body,grant,&mut NativeDecodeControl)`, `take()`, `begin_close`, `close_demand`, `close_step(grant,&mut dyn OriginalRetirementControl)`, `terminal_is_empty`. The private projection is computed and edited under quoted grants; its output includes candidate, inverse and diagnostics rather than a borrowed boolean.

`SemanticProjection<P>` prices/begins/advances/takes the actual private projection. `SemanticEdit<P,M>` prices/begins/advances actual semantics with genuine original before/input references and a private post capability; native receiving authority is explicit.

`🦀️.rs:93-111` exposes `ArtifactReplayPreparation` and `ArtifactReplayPreparationFactory<P,M:Sync>:FactoryRetirement+Send+Sync`; the factory supplies `preparation_birth_demand` and `begin(request,grant)`, returning either producer+receipt or the original rejected request. `ArtifactReplayPreparationTask::step` validates generation/revision and latches decoder ledger plus retirement-recipient identity (lines 129-148); cleanup validates the same identity.

The concrete factory implementation is in `🏪️store/📬️paged-one-item/🦀️.rs:984-996`: `PagedOneItemPreparationFactory<S,M,E>` implements this replay factory with `S:SnapshotProjection<Output=S>`, `M:ArtifactCanonicalJsonTree+RetireOwned+Mutation<S>+Send+Sync+'static`, `E:PagedOneItemEdit<S,M>`. Birth is quoted, recognizes/preflight checks run, and it creates `NativeSemanticPreparation<PagedProjection<S>,PagedSemanticEdit<S,M,E>>`.

Actual consumer `🌱️derived/🧩️store/🦀️.rs:20` is store `begin_derived_operation`, consuming a genuine input projection and original Store read with installed factory. Its sibling demand/step/take methods validate the actual Store issuer and frontier. This is relevant reusable artifact infrastructure, not evidence that norm forms already call it.

## Genuine Retained Mutation Semantics

`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🦀️.rs` defines `RetainedMutation<P>:Mutation<P,Diff:RetainedDiff<P>>+RetireOwned+Send+Sync+'static`, with associated `Preparation:RetainedMutationPreparation<P,Self>`.

`RetainedMutationLeafPreparation<P,M:Mutation<P,Diff:RetainedDiff<P>>,L>:Send+'static` requires new, decision_demand/decide/take_decision, inverse_demand/inverse/take_inverse, and exact close lifecycle. Payload, original and yielded diff are genuine `RetainedCloneRef` values; the semantic port cannot mutate post directly. `begin_retained_mutation`, `decide_retained_mutation`, `preflight_retained_diff`, `inverse_retained_mutation` and `commit_retained_diff` form the actual central decision/preflight/inverse/commit route.

`🏪️store/📬️paged-one-item/🧬️mutation-apply/🦀️.rs:140` provides `mutation_apply_preparation_factory<S,M>()` requiring `S:SnapshotProjection<Output=S>`, `M:RetainedMutation<S>+ArtifactCanonicalJsonTree`. Its `MutationApplyEdit` has real Birth/Decide/Source/Plan/Preflight/Inverse/Commit/Join/Publish stages and can refuse with original diagnostics. It uses the retained semantic APIs above, rather than generic synchronous Mutation replay.

Neither these leaf traits nor this factory constructs M from inspector action arguments: they require an already typed native payload.

## Typed Source and Publication Boundaries

`🧬️native/🔗️input/🦀️.rs:5` provides `ArtifactMutationSource<M:Send+Sync+'static>`. It quotes constructor demand, admits an existing `&mut Option<M>` with its actual retirement factory, advances source admission, and transfers a genuine `RetainedOwnedProjection<M>`. It also closes every authority/admission/source owner. It owns an existing mutation; it is not an action-to-mutation constructor.

`🏪️store/🎮️one-item/📬️publication/🦀️.rs:418` provides `ArtifactStoreOneItemPreparationRequest<P,Input,Mutation>`: operation/generation/revision/lane, Store-minted live authority, SnapshotRead, owned input, exact mutation/snapshot retirement factories. The preparation interface advances with `ArtifactStoreOneItemGrant` and native receiving control, produces `ArtifactStoreOneItemPrepared`, and has cancellation/original-close lifecycle. This publication path also retains real private semantics, but readiness must not fabricate its Store-minted publication metadata. The metadata-free replay request above is the closer existing fit.

## Current Norm Consumer Gap

`✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🧵️retained-commands/🦀️.rs:232-237` installs `cold_mutation_apply_preparation_factory<A::Snapshot,A::Mutation>`. No production `RetainedMutation`, `RetainedMutationLeafPreparation`, `mutation_apply_preparation_factory` or `begin_derived_operation` use was found under norm in this pass.

`plugin/🧵️retained-command/🔄️full-operation/🏭️producer/🦀️.rs` validates original source identity/receiving recipient and calls `A::build_tool_job` under the real StepContext, retaining the original producer request or reserved job. This is the real command producer lifecycle; it is not a standalone staged readiness constructor. Its wire module being present restores a dependency seam, not semantic form acceptance.

Therefore reuse the existing typed source, metadata-free private projection and retained semantics lifecycle once genuine native norm construction and leaf issuers exist. The missing production adapter must own the actual action input, construct its actual typed operation under original receiving authority, run the real retained semantics against a private artifact candidate, retain refusal diagnostics, and close/cancel under the same original authority. No ready flag should be inferred solely from source availability, cached descriptors, or a cold helper result.

## Correction After Inspecting the Installed Cold Factory in Full

The earlier conclusion that genuine retained norm leaf issuers were required to reuse the current authoritative semantic path was too strong. The installed cold factory already implements the metadata-free replay factory. Its explicit synchronous callbacks and receipt limits must be assessed on their actual behavior; its name alone does not reject reuse.

Exact implementation: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/❄️cold-apply/🦀️.rs`.

### Publication Preparation

- Lines 57-62 return `Arc<dyn ArtifactStoreOneItemPreparationFactory<S,M>>`, with `S:Clone+RetireOwned+Send+Sync+'static`, `M:Mutation<S>+ColdMutation<S>+ArtifactCanonicalJsonTree+RetireOwned+Send+Sync+'static`. The default has no domain byte bounds; bounded sibling at 66-71 accepts mutation/snapshot measure callbacks.
- Lines 96-107 preflight Document lane and either fixed maximum footprint or supplied mutation bytes. Lines 110-116 quote frame birth and separately expose request+frame copy demand.
- Lines 119-135 validate actual publication operation/generation/revision and authority, optionally measure snapshot, admit birth capacity, and move the already typed M and exact read/issuers into OneItemOwners. This does not construct typed M from action args. Returned receipt is the birth-demand receipt; begin itself does not include the separately quoted header-copy bytes in its returned progress.
- Phase 0, lines 155-170, runs synchronous `ColdMutation::inverse`, `ColdMutation::diff`, and `apply_diff_cold`, retaining (post,inverse,original mutation). The central apply implementation `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🔺️diff/🦀️.rs:97-103` clones base once and calls the actual diff apply capability. This is real domain semantics, not a shape check.
- Default clone_bytes is zero (156); bounded factory uses the declared snapshot measure. The phase checks only that copy extent and emits corresponding copy progress. It does not receive/use native control, split cold callbacks, or report their nested heap capacity/releases.
- Phase 1, lines 172-187, calls Store authority next_edit, Arc::new(post), prepare_one_item, retains any refused edit/post, and returns Prepared with default progress. Thus real ownership exists, but this phase does not report those physical allocations as an exact measured receipt.
- Lines 205-234 cancel advancement and delegate quoted closure to OneItemOwners. `🏪️store/📬️one-item-owners/🦀️.rs:158-202` quotes each active/prepared/mutation/inverse/candidate/refused/base/authority/factory owner; line 202 onward closes one granted child at a time. This substantive retirement ladder must not be dismissed because preparation callbacks are atomic.

### Existing Metadata-Free Preview Port

Lines 423-440 implement `ArtifactReplayPreparationFactory<S,M>` for the SAME concrete `ColdMutationApplyFactory<S,M>`. It quotes producer frame birth, verifies grant, and creates `NativeSemanticPreparation<S,M,ColdReplayProjection<S>,ColdReplayEdit<S,M>>` with existing actual semantics. Therefore private preview without minting Store publication metadata is already supported by the concrete factory.

The convenience function returns only the publication trait object, so callers cannot assume trait-object downcast/upcast to the replay factory. The concrete factory itself implements both interfaces; existing owner catalog/store installation may expose a separately retained replay factory. Reusing that installed issuer or exposing its actual replay port is the precise interface task, not inventing publication authority.

Projection (274-310) checks quoted demand, synchronously clones source at 296, and transfers owned post. Default quote is size_of<S>; bounded quote is supplied snapshot measure. No nested allocation capacity is priced there. Cancellation retains clone until quoted RetainedCloneClose completes (313-339).

Edit (358-397) quotes size_of<S> copy and size_of<M> capacity, admits/charges that capacity against actual NativeDecodeControl (388-389), then synchronously calls actual ColdMutation diff, central in-place apply and inverse (391-394). It returns those quoted header amounts, inverse, empty diagnostics and false foreign flag. It does not measure the actual variable-sized allocations inside diff/inverse/apply and does not split these callbacks. The replay engine still owns the original typed input, private candidate and cleanup under exact control.

Neither publication nor replay calls a separate generic admit here: ColdMutation diff/apply/inverse are the actual selected domain callbacks. Any required diagnostic/refusal interpretation beyond apply errors must be checked against those actual norm domain implementations, rather than assumed from this factory.

### Norm Bounded First Step and Producer Guarantees

Norm `app-surface/🧵️retained-commands/🦀️.rs:241-250` checks tool/command identity and invokes `admit_retained_tool_producer` with maximum work items 1 and a boxed BoundedArtifactCommandWork. It uses the ordinary constructor helper, not the measured callback variant.

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🏭️producer/🦀️.rs` validates original source binding, validates original encoded command bytes, quotes outer job/work layout, reserves construction scope and transfers the actual owned request. Its measured variant can quote/return additional work-owned allocation. Ordinary helper supplies zero extra work demand/progress. Norm's closure allocates a dynamic work Box inside that zero-extra callback; no exact physical-receipt claim should be inferred for this nested allocation from the declaration alone.

`retained-command/🔖️checkpoint-and-work-contract/🦀️.rs`, BoundedArtifactCommandWork implementation: work_demands returns depth=1 only; reports_progress=false; step marks consumed then calls its entire reducer synchronously, ignoring StepContext. This proves one-shot execution and repeat refusal; it does not prove fine-grained native decoding or physical receipts for mutations constructed by the reducer.

### Precise Remaining Seam

Reuse the current ColdMutationApplyFactory replay implementation to run the same authoritative semantics privately, subject to the explicit atomic-callback bounds above. Still needed: actual owned inspector action-to-typed M construction and its source/receipt/closure contract, a route to the concrete installed replay issuer (without fabricated publication metadata), and real readiness interpretation of the private semantic result/refusal. If the task requires exact bounded physical accounting inside atomic clone/diff/inverse/apply, the existing cold implementation does not prove it; add measured/bounded ports at those concrete seams rather than claim wrapper header grants account for all nested effects. No tests were run to establish behavioral or allocation acceptance.
