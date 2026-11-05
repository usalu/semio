# Producer Page ACK Lifecycle Bug

Confirmed by bounded source inspection; no compilation or runtime execution.

FullOp acknowledge_result_page425–458 verifies the token, acknowledges the exact lane publisher and moves nonterminal lanes back to Publishing. It retains result_page and result_page_presented. The only production result_page_retirement_step invocation is589 inside retirement_step;588 refuses any stage except Retiring. The result-page pending/demand/step methods are otherwise private and used by the old fixture-only close helper.

Host acknowledge_typed_operation_result42 calls FullOp ACK and43–44 returns immediately for nondownload pages. TypedCtx publication entry begins with cancellation handling and has no acknowledged-page closure branch. queue_page349–351 rejects an occupied result_page. Therefore a nonterminal host ACK cannot release its original producer page before the next page; the next queue fails interactive-job.result-page-state or publication stalls. The old private close_fixture_original_page test calls manually filled the missing production transition.

A distinct destructive assignment exists in FullOp close_step648: result_page=None before entering Retiring. It is cancellation cleanup, not a legitimate acknowledged-page advance path, and cannot justify the missing normal route. Original page retirement needs its actual funded context and receipt before this handoff as well.

Recommended test: drive an actual operation producing at least two pages through public presentation and host ACK, physically close only the presented host-owned page, then resume actual TypedCtx publication without private fixture producer cleanup. Require funded original-page closure before the second page is queued, complete terminal delivery, exact receipts and terminal emptiness. Repeat cancellation after nonterminal ACK. A narrow original-context producer-page advance port can share the actual production path with tests; widening the private fixture close helper would preserve the false-green.

## Authored Neutral Regression Inputs

Created permanent FullOp/🧪️tests/🧫️fixtures/📬️ack/🔣️.json and sibling📋️schema.json. Ten scenarios cover progress/artifact ACK, four denied currencies(items/copyBytes/releaseBytes/depth), terminal ACK with full eventual retirement, and invalid operation/receiver/sequence tokens. Expectations explicitly retain original custody immediately after ACK, require original closure before next publication, preserve same page/presented state when denied, and prohibit private close_fixture_original_page.

Independent Ajv validates all10cases; names are unique. Maximum absolute path149UTF16codeunits. This validates test data only. Actual Rust tests and production original-context advance port remain root-owned; no producer execution or green model substitute is claimed. A zero-release denial must exercise an actual release-requiring phase while preserving all other authored limits.

## Close Transition and Original Source Checks

Confirmed second bug: nonRetiring close_demands633–635 quotes only depth1, with zero copy/release. close_step648 assigns result_page=None and reports one item with no release bytes653. Actor TypedOperationResultPage Drop146 asserts that its body is Cold or Empty unless already panicking. A real Original/Source/Projection page therefore panics on this cancellation transition; cold try_new test pages can hide it. Preserve page and presentation custody across the Retiring transition; existing retirement page closure can then price and retire it under the real original context.

validate_original_producer_source in producer3–5 enforces source_identity and receiving decoder only when original_emission_caller exists; producer_source is checked only when present. A manually constructed page fixture with all emission fieldsNone and producer_sourceNone passes that validator regardless of its borrowed source_identity. Worker advance independently checks binding214–216; a new page advance port should likewise check exact binding unconditionally, with wrongcontext refusal coverage.

Actual result tokens contain generation and attempt in addition to operation/receiver/sequence. The initial authored neutral ACK fixture deliberately covered the latter three dimensions only; a direct real-token deserializer must add the two remaining fields rather than silently manufacture identity. Root notified before native test mapping. No code/build performed.

## Real Owned Page and Expanded Inputs

Actor TypedOperationResultPage::from_original84 accepts exact token/lane, Vec backing with capacity4096, and logical length. It creates the enforced Original body. Public present91 advances through staged admission into Source and yields a Projection; host projection close_step96 must finish before ACK. Existing actor receiving/tests10 descriptor_source demonstrates pointer retention, bounded presentation and physical receipt checks. There is no public from_source(cx) constructor; initial backing/frame birth should be admitted using the original construction interface and genuine OriginalFixtureScope, not Cold try_new.

Permanent neutral fixture now has13scenarios: generation/attempt added to every token, invalidgeneration/attempt added, plus wrong-original-source refusal after valid ACK. Required sourceMatches and expected.advanceRefused describe exact source ownership explicitly. Independent Ajv validates13cases; no native execution. This supersedes the earlier10case metadata.

## New Public Port Review

advance_acknowledged_result_page now checks exact source binding unconditionally, then existing producer authority, and posts physical page progress through original_lifecycle_step. TypedCtx normal publication calls it under retained loan before cancellation handoff, preserves progress for the pump unposted-receipt calculation, and returns after that physical unit. No obvious error-conversion or double-posting issue found in this narrow source review.

One route boundary remains to cover: mounted-run-and-pump65–76 chooses owned-child, pending-child, interaction and inline routes before the ordinary typed-operation-unit entry containing cleanup. These routes can execute before an acknowledged producer page retires. Root notified to guard the pump front or share the guard across each special entry; regression should exercise a child page followed by another published lane, not only ordinary progress. Native execution remains pending.

## Pump Routing Follow-up

Latest pump65–77 excludes every special route while acknowledged-page retirement is pending, so the ordinary cleanup entry now precedes child/interaction publication. A concrete accounting issue remains: with acknowledged_page=true and owned_child_groupSome, ordinary branch87 adds the physical page unit to spent, then103 adds the same context delta again because the child owner still exists. Root notified to charge the child delta only when that actual route executed, not merely when its owner remains present. Wrong-source validation remains before mutation.

Neutral Rust mapping must preserve real lane authority: Artifact ACK requires pending_artifact_publication.acknowledge, so a manual page-only fixture cannot fabricate a successful Artifact ACK. Build the real publisher for that row. Progress/Terminal can exercise page-only owned custody. Denied-release requires an actual release-demanding phase; Original-to-Closing may progress without releasing while preserving page identity. No native test result or fixture execution is claimed.

## Actual Rust Helper Review

Caught and reported an unmatched opening parenthesis in ack_page_only_operation; root corrected it and initialized result_sequence from actual token. Caught final close_mounted misuse: that shared helper accepts alreadyRetiring owners only, while successful nextqueue and invalid/denied rows remain AwaitingAck/Publishing. Root replaced final cleanup with actual funded close_step to Retiring and terminal checks.

The original_ack_page action-bus constructor closure matches current Result<(owner,progress),ValueError> API. from_original uses4096byte backing and the public funded presentation produces a real Source/Projection pair. Names/import paths for OriginalFixtureScope and shared page helpers match current owners. Port source/refusal and physical heap receipts are asserted, not replaced by a Cold page.

Coverage boundary: direct public FullOp advance tests do not alone prove TypedCtx pump priority or once-only turn accounting. A real pump regression with an owned child still present is needed to catch the earlier doublecharge. Artifact receipt bytes from real pending publication differ from generic ACKpage fixture payload; actual artifact row must use its real expected serialized receipt instead of assuming these payload bytes. No heavy compile or native execution performed by this review.

## Artifact Integration and Host Continuations

Source review caught artificial operation generation17 against fresh store0; root kept actual initial generation. A second frontier distinction remains: artifact publication updates mounted.operation.generation to receipt.generation_after before emitting its actual page. Replacing the real committed receipt page with neutral generation0 after commit would conceal an invalid producer token. Root notified to compare replacement token with actual emitted receipt token, and distinguish precommit generation from committed generation. The shared neutral fixture now uses generation0, invalidgeneration1; original fixture caller generation17 is a separate authority dimension.

Exactphase document fixture directly invokes publication unit repeatedly. Each preparation ownership receipt remains in mounted.publication_ownership_progress unless taken; a subsequent preparation unit asserts that field empty. Actual pump takes and reconciles unposted receipt. Root notified to do the same exact accounting in direct-unit fixture loops, not discard or double-post it.

Host continuation calls329/364 now invoke current PluginApp publication(identity,samecontext) under retained loans. Runtime typed command ingress and latestwins continuation share the same context, percase registry authority is recreated, and authored1000clock is explicitly compared to actual sourceclock. No stale admit_command_wire, app-level typed dispatch, test_tool_clock or blockingstate.lock remains in bounded current host search. No compilation run by this review.

Independent corpus oracle now executed: permanent XState/Ajv Bun suite14pass0fail15assertions189ms. This validates13neutral row expectations against an independent machine, not native physical execution. See📬️ack-independent-oracle.md.
