# Replay Bound Review

2026-10-28. Read-only current source and exact diff review under store/history/replay, engine delegate/root/cursor owners and the existing publication stage. No compiler, test or graph executed; no source edits.

## Findings

The proposed weak bounds match OriginalReplayStage and the representation of the completed replay publication: P:ToValue+Send+Sync+static and M:ToValue+FromValue+Mutation<P>+OpBinary+OpText+Send+Sync+static. OriginalReplayPublication and its metadata use original borrowed roots, move owned results and clone independent fixed records/Arc handles; removing generic P/M Clone and ColdMutation from those specific owners is semantically credible. M:Sync is a real existing representation constraint: OriginalReplayPublication, OriginalCheckpointCheckout and OriginalReplayMetadataPreparation already require Sync for M, and their staged retained/inverse/history sources carry shared M-backed ownership. It must be explicit in calling generic owners; Clone+ColdMutation should not be presumed to imply Sync without their trait contract.

The current proposal is NOT transitively complete. These exact strong-bound callees still prevent wholesale weakening:

- Checkpoint and completed replay admission call ArtifactStore.local_step_pending. Both its engine operations delegate and StoreReplay implementation currently require P:Clone+FromValue and M:Clone+ColdMutation. The body only tests pending_reprojection.local and is a valid pure-read split candidate; Rust has been notified.
- Metadata advance/close calls history_edit_read_issuer_demands and admit_history_edit_read_issuer from a strong ArtifactStore root impl. Their bodies quote/borrow the installed original mutation retirement issuer and delegate to edit_retirement; they do not clone P or M. They can be considered for a narrowly owned read-capability split, preserving unsupported-owner refusal and grant checks. Rust has been notified.
- Checkout.advance calls begin_selected_report_replay and step_derived_report_replay, while taking the result uses StoreCursor.take_derived_report_result with explicit P:Clone and Mutation:ColdMutation<P>. Checkout close calls derived_report_replay_retirement_demands/retire_derived_report_replay_controlled, also currently in strong owner impls. These are actual replay lifecycle operations, not just metadata accessors. Their deeper replay cursor/finish/control owners need separate examination before removing bounds. In particular, take_derived_report_result calls the replay cursor finish method after ownership/frontier/finished checks. Keep the necessary bounds on the methods that actually replay, or split underlying owners only if their own bodies and transitive calls justify it. Merely weakening checkout entire impl is insufficient.

ensure_durable_group_idle has already moved to the weak metadata delegate impl in the current peer-edited source and correctly preserves the wire refusal. The pure root parts_ref/parts paths remain direct structural borrows.

## Capability and Guard Review

No actual publication guard or accounting change was found in the requested bound packet. The original caller binding, native decoder identity, store address/frontier/issuance checks, selection identity, durable/local writer checks, original-history ownership, blocking semantic report refusal, cancellation, grant admission, progress charging and close custody remain in the bodies. Weak trait bounds do not themselves replace these runtime authorities.

Making admit_edit_step public is necessary after the consumer became a separate crate. Its same-original-selected-result and nonempty-ID checks remain, and it funnels into capture, which validates source frontier, source state, finalization intent and writer gates. Thus the visibility change does not expose an unchecked writer path.

## Coordination and Remaining Proof

Exact blockers sent to mutations and Rust agents before any native retry. No compile success or complete caller compatibility is claimed by this source audit. A follow-up owner split must retain all gates and qualify only methods that need the stronger replay capabilities; the owning native compile is the final proof once those transitive bounds agree.

## Peer Transitive Follow-Up

The mutations worker subsequently traced EditReplay constructors/with_base/with_drafts/with_retirement_factories/finish and ReplayOwnedState.new_counted: those current grouped strong bounds cover structural construction or owner moves and may be segregated without typed clones. Actual cold step/granted_step/step_operations retain genuine cold replay requirements. This supports precise owner partitions, not a blanket weakening of cold execution. Rust received the method roster; this review still makes no compiler-success claim.

## Original-Path Inspection During Segregation

Traced selected begin to bounded history plan and original Report stepping, then raw EditReplay.step_granted. In the pre-split source step_granted reached a common step_with_operation_cap loop containing both original prepared-operation dispatch and a cold replay_operation fallback. The precise proposed repair is a private optional cold-operation function pointer: strong cold wrapper supplies the actual cold operation, original step_granted supplies no callback and must supply its original grant/control. This is a capability separation rather than deleting semantic requirements. Original mode must preserve missing-issuer refusal; it must never mint the cold unbounded grant.

The loop original prepared-operation branch obtains prepared outputs through the installed ArtifactReplayPreparationFactory/Task interfaces. It transfers owned next/inverse/message results and charges the actual native decoder; it does not call ColdMutation methods on P/M. record only Arc-clones P, converge updates cursor bookkeeping, and step_convergence compares retained Arc roots (or a supplied equality function under the existing cold-only authority condition) before replacing/retiring owned roots. These bodies support narrow generic bounds if all their transitive owner methods are segregated consistently.

StoreLive.live_base likewise Arc-clones retained roots or shares the genesis snapshot. Its cfg(test) path calls assert_prefix_entry_is_live, so that oracle helper and forward_prefix_digests bounds must be checked during the move, rather than dropping test validation. The retained original-order pointer/length, Store registry/generation/revision and original-decoder identity checks observed in Store step_derived_report_replay_in_order remain intact. No borrowing lifetime or root ownership change is proposed.

## Final Exact Split Follow-Through

The current source supersedes the transitive blockers recorded earlier: Store original Report construction/selected planning/retirement owner now has the narrow bounds, with genuine reference cold operations retaining explicit method-local stronger bounds. StoreCursor.take_derived_report_result no longer asks for Clone/Cold merely to move a finished cursor result; it retains source registry/frontier admission and finished-state checks before taking the owner. ArtifactDerivedReplay.take_preparation_receipt moves the same receipt under its minimal Mutation+OpBinary representation. Pure local-step and original issuer accessors are separated without changing predicates or issuer/grant refusals.

Followed StoreLive selected planning through selected_history_edit: the first inspection caught that borrowed Edit helper still in a strong impl after its caller moved. Rust corrected it; final source keeps the identical explicit lifetime from borrowed StoreLog to returned Edit, key/position/selection routing and errors under the same weak group. live_base and its cfg(test) prefix oracle also moved together; Arc shares and genesis ownership remain unchanged.

Raw EditReplay.step_granted now calls private ReplayOwnedState.advance_with_operation_cap with Some(original grant), Some(native decoder), and no cold executor. The common helper computes cold authority only when original is absent, producer absent AND cold callback present. The genuine cold wrapper retains Clone/ColdMutation and supplies replay_operation as its callback. Native missing-producer paths still refuse UnsupportedOwner before any cold operation. Constructor/finalize/getter and native prepared-operation interface methods transfer or borrow original owners; actual cold semantic new/step/run/replay_operation keep their required bounds. Record/convergence/close-accounting bodies preserve their existing root/receipt/control logic.

The new original-versus-cold authority law checks original missing-producer refusal preserves state pointer, cursor, operation, progress and output extents with a zero receipt; the cold operation reaches the authored fixture state. Review caught an omitted native owner cleanup ladder in its first version; the final source now closes raw replay, closes/releases Store through existing controlled admission, closes decoder and retires caller exactly as the existing test lifecycle does.

No remaining actionable write-gate relaxation, borrowed-source lifetime change, or caller Sync mismatch was found in this exact reviewed path. M:Sync remains explicit where existing shared retained publication/metadata representations require it; lower construction and ownership-transfer methods impose only their actual representation requirements. This is source-review clearance for the precise segregation, NOT a claim that native compilation/tests pass or that unreviewed consumers have no diagnostics. Both writers were notified.
