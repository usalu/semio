# Neutral Primary Recovery — Independent Read-Only Audit

## Scope And Verdict

Read the complete211-line [proposal](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️resident-primary-neutral-split-proposal-2026-08-28.md), actual resident authority1–840, and current allocator/consumer test infrastructure. No implementation/test edits, native command, Cargo, new source hold or competing API were made. Only this report is authored.

The proposed same-root primary anchor and two counted cursor pins are a viable **design candidate**, provided the obligations below are explicit in the source packet and genuine laws. They do not yet exist in resident e23ec4; rg finds no primary/recovery_pins/registration API there. All findings about the six methods are **proposed-API/coverage gaps**, not executed native defects or a reversal of R11's25PASS.

The principal must-fix declaration is Closing recovery: revoked pointerless continuations must not read freed nodes, but revoking every fresh Closing scan does not by itself recover a live C or prove close progress. The test must demonstrate actual live-source recovery under an explicitly bounded fair schedule and not generalize it to arbitrary repeated close/recovery interleavings.

Dag acknowledged the findings while authoring ticket-only tests. His new bodies are in flight and were not treated as a frozen/coherent packet or independently verified test results. The report/proposal and canonical e23/e81 endpoints remain unchanged.

## Current Safety Foundations

- [ResidentAccess118](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs:118) acquires one inline atomic gate. Payload/header/list access is serialized. The guard is not Send; a read facade holds it while exposing &C. No guard should be stored across resumed operations.
- [ResidentConsumer437](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs:437) contains root reference plus node pointer. Its Drop decrements aliases outside the gate; therefore successful capture must establish that alias before the root-owned found pin is released. Consumer release currently checks aliases/admissions before detaching.
- [prepare_consumer512](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs:512) retains a charged Pending page before allocation; null leaves it retained. Initialization and publication are separate. Current prepared_consumer is just the latest pointer, not durable original-primary identity.
- [close_step232](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs:232) handles Release first, then pending/admission records, pending consumer, consumer-list head. Live C and aliases/admissions can block it. New recovery logic must work despite those early blocked paths.
- [Release347](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs:347) already separates Destroy, Free, Refund, Clear. Pointerless poison close can finish numeric/root-local phases; it cannot traverse a live new cursor/anchor/list.
- Existing unsafe Send on the erased LedgerState/ConsumerPage is not new evidence for proposed pins. The extended justification must say that every pointer is backed by same-root list/Pending/Release custody or a counted pin, with C:Send, and no pointer escapes an unpinned/revoked continuation.

## Six-Method Review

| Method | Required precondition and no-loss boundary |
| --- | --- |
| reserve_primary_consumer | Open root, absent primary, exact partition capacity, full work grant and checked nonzero successor must all succeed before writes to charge/counter/anchor. Install Pending stamp before any allocator. A refused reservation must not consume a generation or clear ordinary prepared_consumer. |
| prepare_primary_consumer | Match C to the original stamp/page before any cast/initializer. Advance only the original retained allocate/init/publish phase; no fresh registration or reservation on retry. Null retains charge, pointer=None and initialized=false. Published replay must not replace the anchor. |
| begin_primary_recovery | Require Published, matching C and mode/root phase, no occupied cursor. Obtain the first pin while the list still owns that node and the gate excludes detach. Refuse before changing cursor if pin increment overflows. No lookup through latest prepared_consumer. |
| advance_primary_recovery | Check root/cursor phase, mode and revoked state before any saved-pointer load. Acquire/check exact successor pin before clearing current; compare registration plus original primary association, not type or positive count alone. All checked/fallible work precedes the fixed permutation. |
| capture_primary_consumer | Revalidate found stamp/type/mode under gate; preflight grant and atomically acquire alias successfully before releasing found pin or clearing cursor. Alias-count overflow or concurrent increment refusal retains the pin/cursor. Returned facade lifetime is tied to the borrowed original root. |
| begin_primary_consumer_close | Preflight all changed latch/revoked bool work. Latch whole-root close, not child retirement; do not free. Refused zero/short call changes neither root phase nor cursor. Existing ungranted begin_close is not a substitute inside the new frontier test. |

All returned continuations must be pointerless. A returned ResidentConsumer is different: it deliberately carries a pointer with an alias count, and cannot be treated as a pointerless stale continuation. Its drop must finish the decrement before a zero-alias detach is allowed; no subsequent pointer read belongs in that drop.

## Must-Fix Or Explicitly Verified Before Mount

### 1. Live C And Closing Recovery Have A Real Progress Contract

The proposal says Closing recovery can start after a revoked cursor is cleared, but also says close_step may revoke that new cursor again. Repeating “begin scan → close/revoke” can starve recovery forever while C remains installed. Root close then correctly refuses to destroy live C; cursor revocation alone cannot turn that refusal into completion.

The empty-C paused-next/found law proves pin cancellation and stale resumption safety, **not** this live-source path. The loss law must:

1. Install a real tagged/drop-observed live C in the original primary, behind a later same-type ordinary consumer.
2. Lose every forward facade, latch close, and show forward capture/install are unavailable without losing C.
3. Begin Closing recovery; deliberately revoke once with a real close_step; resume that revoked continuation and observe zero node loads/aliases.
4. Clear the revoked root cursor with actual granted turns; give the new Closing scan consecutive bounded grants.
5. Capture the original C, hand it into the preexisting test cleanup Option, drop the facade, then close all original root owners. Assert no C drop/free happened before handoff.

This proves one bounded **fair** recovery schedule, not arbitrary interleaving liveness. Dag confirmed he is making that schedule explicit in the existing loss law. Do not write that any suspended pointerless caller can never prevent close regardless of a still-live payload or continuous revocation.

Additional ordering case: root close may first block at an admission whose consumer is this live C, before reaching consumer-list selection. Closing capture must be usable while that admission exists; after actual handoff, its normal admission/record close can continue. A generic early “root is closing, reject recovery” check would deadlock this path.

### 2. Found Pin Must Become An Alias Without A Lifetime Gap

The root gate prevents concurrent node detach, but does not authorize creating an alias after pin release when a later check can fail. Checked alias increment happens first. If it fails (including usize exhaustion), found stays exact, next stays unchanged, and no cursor slot is consumed. External facade Drop can concurrently change aliases; a non-atomic load/add/store is not valid.

Likewise successor-pin overflow must leave the old pin and both cursor slots unchanged. The successor's pin must be taken while current.next is still protected by the same gate/list membership. No public stamp, cached pointer from a prior call, or same-type positive count is a substitute.

Small native coverage: existing short-grant pin law should include alias-overflow before capture and pin-overflow before advance, with injected scalar counters restored before actual cleanup; compare exact per-node counts and cursor identities. Dag specifically confirmed the capture-overflow addition, not yet its result.

### 3. Every Detach Path Must Eliminate Stale Anchor Access Before Free

A Published anchor is a non-owning pointer into the existing list. Before selecting that page for Release, validate both pointer and registration and set the anchor to pointerless Releasing in the same fixed transition. Only primary origin+same registration can clear it. Ordinary consumer detach must not accidentally clear an unrelated primary with the same type.

Release precedence is safe only if an active cursor can never pin a node already detached into Release. Begin must start from the current owned list, not a stale Published pointer. Once a node has entered Release, recovery for it must reject from pointerless phase metadata without loading that node. A prior found/next pin must have reached zero before detachment.

Update structurally_empty to include **both** primary and recovery, including an empty revoked cursor and a pointerless Releasing anchor. Otherwise final root close or poison admission could falsely ignore retained new control state. PrimaryPending/PrimaryConsumer Clear must validate the matching Releasing anchor before clearing either slot, using only scalar/TypeId/registration data after Free. A poisoned root with pointerless Release plus another live cursor cannot traverse that cursor as part of the allowed refund/clear shortcut.

Small native coverage: extend actual paused-next/found tests through Free→Refund→Clear, then resume the pointerless old call with zero loads/aliases; include unchanged ordinary same-type node/pins and final structural emptiness. Preserve R11 pointerless-poison regression; do not clear live poison for test cleanup.

### 4. Registration And Grant Refusal Must Be Atomic For Ordinary Preparation Too

The new counter is shared by primary **and ordinary** consumer reservation. That ordinary path currently modifies charge/pending/latest-pointer state. The checked successor must therefore be computed before state.reserve or any prepared-pointer reset; a resource-capacity refusal must not burn a registration, and an overflow refusal must not debit resources.

Test u64::MAX−1→MAX as the last successful nonzero registration, then refuse the next reservation on both paths with exact unchanged charge/pending/primary/prepared state and zero allocator entries. This is the u64 private identity domain, not the smaller safe-integer capacity domain. Null allocation retains its already consumed stamp; retry does not mint another, and cancellation does not reset/reuse the counter.

All declared work sums are conditional on the actual write sequence. They are not compiler measurements. In particular primary publication B+3P is sufficient only if the implementation does exactly those transfers and does not also write ordinary prepared_consumer or move a whole larger anchor temporary uncharged. The ordinary new registration/header path requires its own changed-field census; the primary table alone does not price it. Derive exact zero/short sums from private types, retain4096 no-fit refusal, and do not change old tests/limits to fit.

Dag states MAX/no-reset is in the seventh body; verify both primary and ordinary paths at the final coherent review. The seven-name count must not hide an untested changed ordinary path.

### 5. Null-Allocation Cancellation Must Never Destroy Or Free A Nonexistent Node

The primary reservation retains ConsumerPage with pointer=None. On null, no header/C exists and allocated_bytes must remain unchanged; original partition charge and stamp remain. Cancellation transfers this exact descriptor to PrimaryPending Release as **Refund { released_layout:None }**, not Destroy/Free. Clear removes only the matching original anchor after refund.

The initialized-but-unpublished path is different: actual allocated backing with source=None must Destroy then Free. A stale type or replay must not call an initializer twice or infer initialized=true solely from a pointer.

Small native coverage: at each reserve/allocate/init/publish frontier, observe actual System events and all three axes before/after. Reserved/null cases require zero Destroy and zero Free; allocated-uninitialized requires no typed destructor; initialized requires the actual empty-node destruction and one free. Failure selection must record exactly one entered matching allocator Layout and delegated=false/null=true; disarm before cleanup. Dag acknowledged explicit zero-Destroy/Free trace for pointer=None.

### 6. “Other Partition Unchanged” Must Be Scoped To The Selected Transition

Whole-root close intentionally retires unrelated consumers too. With a later ordinary node at list head, its release may legitimately happen before the primary. The cancellation law should compare the unaffected partition immediately around the selected primary Free/Refund/Clear and retain nonzero original charge elsewhere when needed. It must not force a new targeted-release ordering just to keep an unrelated empty consumer alive during all whole-root close turns.

Similarly, preserving one other live C means root terminal remains blocked until that C is actually handed back. Do not silently drop it, remove its charge from expected totals, or claim full-root terminal after only primary cleanup. This slice is explicitly not a live-child release API.

## Smallest Native Law Packet

Keep the proposed seven names and existing25 unchanged; strengthen bodies rather than manufacture a second harness:

- layout/all-short law: actual ordinary and primary registration writes, all phase sums, capacity and4096 no-fit.
- lost-returns law: the tagged live-C Closing recovery schedule above, plus an original admission that blocks normal root close until handoff. The returned public ResidentConsumer already supports that existing admission coupling.
- partial-cancel/null laws: actual pointer=None versus allocated versus initialized phases, all-axis charge and no nonexistent deallocation.
- short-pin law: exact per-node next/found changes, successor-pin and capture-alias overflow, no alias on short grant.
- paused-next/found law: real parked pointerless caller, root-held pins cleared before actual free, stale resume after close performs zero loads.
- busy/identity/MAX law: actual gate contention, equal-counter foreign root rejection, wrong type/mode/replay, primary and ordinary registration overflow/no reset.

These are proposed acceptance requirements, not an executed roster. A missing-API compiler RED can only establish that the six methods/types are absent. It cannot execute pin/ownership assertions. The later real implementation gate must retain all25 prior tests and all newly mounted laws, with exact source/hash capture and a separately granted native lease. No code is approved or run by this audit.

## Coordination And Source Identity

Dag directly confirms e23/e81 unchanged, proposal unchanged during reading; only new ticket test bodies and ready-for-mount report are in progress. He accepted the live-C fair schedule, capture-overflow and pointer=None cancellation additions. I did not edit or bless those unfinalized bodies.

Fresh observations:

```text
e23ec4068c261ef56020e4aaafd97e3bd304a6503a58e9dc1b7a3c6de576dbd3  🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs
e81bcca1121f724891f75f007322c61e61d46ea9dc71a601c413aeb6afbba175  🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs
7805b47687599a35123df781eb376e004bd2ecbf46e8c5914311aab959c020e7  /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️resident-primary-neutral-split-proposal-2026-08-28.md
```

This is a selected read-only boundary, not an immutable repository/compile closure. No newly executed safety defect, native success, live Runtime/App funding, Store receiver, callback-tail completion or SyncSession authority is inferred.
