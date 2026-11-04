# Original Fixture Page Projection — 2026-11-04

## Actual Failure and Contract

Native runtime r5 compiled, completed window genesis within its unchanged original limits, then failed during productive actions with `original fixture page sealing omitted its next projection`. The refusal came from the generic ArtifactApp fixture helper; the retained-command test-only twin had the same two-call assumption.

The real TypedOperationResultPage producer now moves Cold/Original backing into StagedSourceAdmission, advances that owned admission across funded turns, retains Source authority, and only then emits the real immutable Projection. Intermediate funded turns correctly return None with nonzero original progress. Two calls cannot guarantee a projection. The production presentation, recipient/token, one-shot mounted delivery, ACK and physical close protocols are correct and remain unchanged.

## Minimal Correction

Changed only the two test-only fixture helpers to continue observing the real finite page sealing state while its unchanged original grant produces nonzero progress. They return the actual projected page, or return None when the actual producer has no admitted work/progress. Every production call still propagates its primary error and checks progress fits the original grant and exactly matches observed heap allocation/release receipts. No fabricated/default projection, relaxed guard, larger grant, budget, timeout, ID or production API was introduced. The loop follows actual page completion or genuine admission refusal, rather than adding an arbitrary larger call ceiling.

Files changed: plugin/🧪️testing/🧪️artifact-app-laws/🦀️.rs and plugin/🧵️retained-command/🔄️full-operation/🦀️.rs. The latter method remains cfg(test/artifact-app-testing).

## Regression and Validation

Strengthened the existing genuine actor receiving law and neutral golden fixture at actor/🖼️wire-turn/📬️typed-operation/📥️receiving/🧪️tests. The fixture now records minimumPresentationTurns=3. The existing real page helper retains its 32-turn bounded test horizon and unchanged originalHost grant (4096 copy bytes, 65536 capacity, 262144 release, depth 64), asserting at least three real turns before projection, exact token and unchanged backing pointer. Existing serde_json neutral/golden bytes, per-turn heap receipts, wire receiving, ACK-equivalent alias closure and original backing retirement checks remain intact.

The actual runtime omitted-projection failure is current red evidence. Native validation of the strengthened actor law and corrected wrappers is pending the globally coordinated Cargo window; this lane ran no Cargo. diff --check passes. Native owner is semio-framework-actor, explicitly in 🧰️framework/Cargo.toml; exact unit test path is component::typed_operation_page::receiving::tests::original_host_wire_receiver_preserves_golden_bytes_and_physical_receipts. A substring original_host_wire_receiver_ also selects existing actual receiving tests; validation must confirm a nonzero selected test count.

Source batch is stable and handed directly to Runtime, alongside its separate latest-wins investigation. No manifests, dependencies or native graph producers changed.

## Receiver Custody Follow-up After r6b

Runtime r6b compiled and no longer failed the obsolete two-call presentation assumption. It now reached actual result pages and failed the real page Drop guard in productive action and task-resume cases. Source inspection identifies a pre-existing consumer omission: the generic settle_registered_typed_operation helper decoded the page then acknowledged its raw token while the receiver projection was still live, and dropped that projection at loop exit. The presentation helper transferred valid genuine custody; making it complete its actual sealing merely exposed this consumer defect.

Correct peer fixture consumers (slot/publication fairness and wp3 test-host settlement) already close the real receiver through close_fixture_result_page before sending the exact ACK. Matched that protocol in exactly one Source file: plugin/🧪️testing/🧪️artifact-app-laws/🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs. Sequence is now borrow lane and fault bytes while live, fund physical receiver closure under unchanged mounted_policy.close, obtain the helper's original token, ACK that token, then record lane and return any decoded primary fault as before. The page's owned backing/projection, token, production one-shot presentation, ACK behavior and Drop guard were not changed.

Actual r6b productive Drop panics are the red evidence. Parent reports two genuine focused actor receiving laws are already green, validating the correct funded receiver-close path independently of this shared consumer correction. diff --check passes. Runtime received SourceStable handoff for the next combined editor test run; this lane ran no Cargo. Native productive-consumer validation remains pending, so this report does not yet claim those action/resume tests pass.
