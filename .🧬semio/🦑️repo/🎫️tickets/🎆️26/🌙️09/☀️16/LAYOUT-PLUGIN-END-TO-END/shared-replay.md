# Shared Original Replay Interface Bounds

2026-10-28. Scoped repair for actual native diagnostics in mutations-native.log. The already removed granted-history API errors are stale and were not reintroduced. Rust owns current history-command state and shared Store/read accessors; this packet owns the original publication and checkpoint replay subtree.

## Exact Changes

Five files beneath framework/os/store/history/replay: root 🦀️.rs, checkpoint.rs, checkout.rs, metadata/🦀️.rs, checkpoint-order.rs. Every affected generic function now requires P:ToValue+Send+Sync+'static and M:ToValue+FromValue+Mutation<P>+OpBinary+OpText+Send+Sync+'static. P:Clone, P:FromValue, M:Clone and M:ColdMutation<P> were removed only from these original caller-driven methods. M:Sync is now explicit in these clauses because the original metadata/source owners and publication stage require shareable mutation owners; this is a capability correction, not a claim of pure bound weakening. The selected edit-step constructor is public because history-command is an independently owned crate.

No semantic body, writer gate, frontiers, decoder bindings, source issuances, publication decisions, grants, progress charges, cancellation, close transitions or retirement ownership was changed.

## Requirement Proof

OriginalReplayPublication captures and transfers its actual EditReplayResult and finalization request; advance prepares existing transition rows, metadata, diagnostics and pending reports; publish delegates the already-narrow OriginalReplayStage. Its body calls no Clone, snapshot FromValue or ColdMutation operation.

OriginalCheckpointPublication authors graph metadata and moves the original Change/Checkpoint/Alternative rows to the same publication gate. Its only clone is an Arc publication-decision handle; this imposes no Clone bound on P or M. Every published cursor/fact remains controlled by the same gate and original source checks.

OriginalCheckpointCheckout borrows selected original edit rows, runs native report preparation, stages existing inverses and replaces local projection/cursor roots after genuine admission. It has no typed payload clone or cold mutation invocation. The read-owner native methods and preparation-receipt access are maintained independently by the Rust agent.

OriginalReplayMetadataPreparation resolves/retires original read issuers and prepares metadata using actual original rows and factory owners. OriginalCheckpointEditOrder only borrows and selects checkpoint history keys into its existing bounded page stack. Neither helper invokes Clone, P decoding, or ColdMutation. Their historical broad bounds would otherwise transitively prevent the caller-side correction.

The existing OriginalReplayStage methods already require the same narrow typed bounds, proving that no write capability or publication check needed weakening. This change removes unrelated historical computation capabilities, retaining all actual original typed serialization, mutation, wire, send/share and lifetime requirements.

## Validation Status

Root received the exact proposed signatures and body proof for independent review before the next coordinated compiler run. The separate native diagnostic identifies history-command invoking OriginalReplayPublication::admit_edit_step, confirming the necessary cross-crate visibility. No other visibility was broadened. No Cargo command was launched by this agent. Actual compiler validation remains queued with Rust after concurrent prerequisite packets settle.

## Independent Transitive Review

The independent reviewer identified remaining prerequisite impl grouping in the Rust-owned Store/read partition: local_step_pending delegate and StoreLive gate; history_edit_read_issuer_demands/admit_history_edit_read_issuer; native derived-report creation/step/result/retirement helpers; and ArtifactDerivedReplay preparation receipt access. These call sites have not been bypassed. Native admission waits for body-preserving original/native interface segregation at those owners.

A deeper read confirms EditReplay::new_counted, with_base, with_drafts, with_retirement_factories and finish are currently grouped with cold step methods although their own bodies move/retain/transfer existing original owners. ReplayOwnedState::new_counted similarly constructs from the original Arc baseline without cloning P or applying cold mutations. This is evidence for a precise constructor/finalization split, not permission to weaken cold semantic execution methods. Exact deeper paths were sent to Rust; this packet has not edited them.

## Explicit Replay Execution Authority

The native original path and the unscheduled cold path now share one private bounded machine with an explicit optional typed cold-operation executor. The cold entry retains its real P:Clone/M:ColdMutation bounds and supplies its genuine replay_operation function. step_granted is the original entry and supplies the actual caller grant, original decoder and no cold executor. The machine can mint its existing cold authority grant only when no original grant or semantic producer is present AND the cold executor capability is present. Missing original producer therefore still returns the exact UnsupportedOwner refusal; it cannot invoke the cold operation.

This is the current semantic authority contract. It introduces no fallback or second replay loop. All prefix issuer refusal, original grants, decoder access, state transitions, post-edit settlement, record/convergence behavior and retirement remain in that same machine. The only loop changes are the explicit capability input, capability membership in cold selection, and invoking the admitted executor at the original cold call site.

Two additional files own the precise raw interface split: store/replay/prefix-digests-replay-progress-and-edit-replay/🦀️.rs and store/replay/replay-owned-state-operations/🦀️.rs. Move-only retain/new_counted/base/drafts/factory/finalize and scalar getter methods no longer inherit unrelated cold semantic bounds. Native preparation, diagnostic settlement and Arc-only record/convergence helpers use their actual Mutation+OpBinary interface; actual cold replay_operation/step/run remain under the genuine cold bounds. No third raw type file needed editing.

The existing report test owner now retains authority.json and a native law. Both cases borrow the existing independent authored/genesis JSON witness values: original authority without a producer must refuse with unchanged state pointer, cursor, operation, progress and output extents and zero native receipt; explicit cold execution must produce the authored witness after exactly one operation. Each case closes the raw replay, Store, native box, decoder and caller using the existing controlled lifecycle. serde_json independently decodes the neutral fixture, and the preexisting mutation fixtures remain unchanged.

Independent raw review confirmed that the original entry structurally excludes cold fallback and no semantic gate/accounting regression was found. The reviewer caught an initial missing Store lifecycle cleanup in the new law; the test now uses the exact established cleanup ladder. Actual compiler/native test validation remains coordinated by Rust after shared prerequisites settle.

Latest canonical normal source inventory: raw replay12inputs/153242bytes/2131active lines; original publication9inputs/189500bytes/587active lines. Both fit unchanged32/262144/4000 limits. Input retained as shared-replay-inventory.ts. Independent reviewer confirmed complete law lifecycle and found no remaining actionable raw constructor/finalize/getter or authority-seam defect; compiler remains the final proof.

## Actual Native Execution

The genuine repository-root Cargo package run completed successfully: semio-framework-os-store-replay-authority-tests,2passed/0failed/0filtered. Both exact named tests executed. Native original output was accepted=false,unchanged=true,error=UnsupportedOwner,state={counter:7,calls:0}; native cold output was accepted=true,unchanged=false,error=null,state={counter:11,calls:1}. These equal the independently executed fast-json-patch records from the same neutral fixture. No fixture-only or zero-filter result was counted. Rust retained the full command output under 🗑️generated/replay-authority-native-2.log until ticket cleanup.

Exact successful native command from /Users/ueli/Documents/semio: repo --service=off cargo --store=shared --configuration=prototype -- test --manifest-path Cargo.toml -p semio-framework-os-store-replay-authority-tests --lib -- --nocapture. Actual exit status0; session68410.
