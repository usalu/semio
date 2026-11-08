# Receiver Backing Borrowed Close

The lower native receiver protocol now defines `NativeReceiverBackingClose`. It retains the exact child result before the callback returns and separately pays for its route, reply and Work-binding headers. It freezes the actual original invocation, grant, body, and physical backing addresses across Pending and unwind. Real native lifecycle evidence is checked against the actual receiver before moving or dropping its boxed header; the real destructor runs under paid release. The removed helper's raw deallocation and owned callback-return bridge are gone. Close-start now uses borrowed producer replies as well. `NativeSlotProducer` delegates its actual tuple's retirement support.

The new strict language-neutral schema and SQLite oracle passed (1 test, 4 assertions). A real native missing-owner red was captured in session 3822. Successor 29838 compiled and reached the recorded child-close unwind case, but failed because the shared allocator observer remained active after a panic. This is not native acceptance. No existing test was removed or disabled.

## Allocator Observer Recovery

Two added native regressions exposed the observer's missing unwind cleanup and its replacement of the outer counter when rejecting a nested observation: session 56678 had 3 existing passes and 2 new failures. Its separate strict schema/SQLite oracle passed (1 test, 3 assertions). The observer now validates an idle thread-local before installing its own counter and uses a native guard to clear only its own active observation on unwind. A full five-law native successor is running.

## Remaining Integration and Proof

The higher receiver clients still call the removed owned backing helper or owned close-start API and need coherent borrowed migration. They must retain the actual new dispatcher as a field, include its demand/Work, route original control through its borrowed ports, and include it in terminal checks. No compatibility alias or owned-result bridge has been introduced.

The helper authenticates the Work scope selected when its close operation first begins. Admission-to-close Work association must also be established by the actual enclosing owner; the standalone positive test uses the same real finite Source bank for storage allocation, seeding and every close action, but is not a foreign-admission-scope proof. Original-birth settlement, foreign scope, denial/refusal, and allocating child unwind cases remain open. The interrupted test child changes only inline bools; no general allocator-unwind receipt claim is made.

The allocator guard successor58235 passed the full native observer suite5/5, preserving the existing reallocation, nested trace and real null-allocation laws as well as the two new unwind/nested-rejection regressions. Backing lifecycle successor now running against that correction.

Native successor43810 progressed beyond the corrected observer, then exposed an actual dispatch guard error. Diagnostic successor10766 isolated it: Core narrows each invoked effect to the genuine action’s remaining Work, so remaining Work legitimately becomes zero when recovering an already-recorded child. The dispatcher now pins the actual immutable original invocation/Work-action identity and its saved child quote, while reborrowing only the genuine remaining Work. It no longer treats per-call remaining Work as a caller-selected original ceiling. No new credit, action, binding or receipt is issued. The original grant, owner, invocation and scope checks remain.

## Selected Native Acceptance

Successor91831 passed1/1 native test covering both complete and recorded-close-unwind cases. Both use one genuine finite Source/Work bank for the original backing allocation, seed, child close, boxed ownership transfer, actual real destructor and deallocation, and final dispatcher binding cleanup. Zero Work preserves the original backing and wallet. The interrupted case reenters the same action with zero remaining Work, forwards the original resident child result without invoking its recovery callback or billing the child again, and still completes paid physical/header cleanup. Each case observes one actual close call, zero recover calls and one actual destructor. The original native allocator observations match actual capacity/release progress on successful physical turns.

This acceptance is limited to the selected fixed-inline bool child. Higher clients, original-birth protocols, foreign admission scope, native refusal injection and allocating child unwind are not accepted by this result. The protocol's preexisting pure-publication native target is being run alongside the new target as a regression.

Protocol regression62779 passed both actual native integration targets2/2 (new backing1 and preserved pure-publication1), with no ignored or filtered tests. This verifies the lower protocol closure, not the higher receiver clients.

The real native shared-field production closure now compiles (94672). Its only next gate after protocol migration was three producer references to the wrong parent module; they now name the defining crate’s actual NativeSharedSlots export. This also compiles Root’s earlier paid failure-data/Work-binding field source changes and IO’s resident cancellation source changes, but does not establish native field behavior. The selected paged interruption law is rerunning against this coherent production closure.

The dependent paged selected native law56955 now also passes1/1 across all4 interrupted lifecycle cases; details and explicit open corpus limits are in [Paged Clone Borrowed Migration](paged-clone-borrowed-migration.md). End-to-end norms remains unverified and the goal/ticket remain open.
