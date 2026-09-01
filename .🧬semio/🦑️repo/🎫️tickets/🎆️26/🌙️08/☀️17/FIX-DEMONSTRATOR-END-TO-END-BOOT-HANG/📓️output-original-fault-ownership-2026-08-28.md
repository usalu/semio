# Original Output Fault Ownership

## Concrete Source Defect

The existing response slot retained its handle before constructor finalization, but did not retain the thrown constructor value. Empty cancellation could then unlink that still-faulted slot. The async outcome path assigned `Object.freeze({kind,value})` only after finalization succeeded; a finalizer failure could replace or discard the original successful/refused value. These defects are separate from receiver admission and source/UI evidence settlement.

## Canonical Repair Scope

One new exact Slot fault word owns the first constructor/outcome-finalizer failure. The outcome record must be assigned to the original slot before its finalizer; finalizer failure never replaces that record. A fault-held slot cannot dispatch or use empty cancellation. A distinct later arbitrary fault stays caller-owned rather than overwriting the first. No cold discard, fault normalization, getter inspection, new transport format or cleanup sink is added.

The existing pending metadata census changes Slot10→11 fields and pending560→576 logical bytes, with the same four records. This is source inventory, not an admitted request record or a physical heap bound. The previously admitted1280 return-roster subset excludes all per-output slots and therefore is not repurposed to fund this added field. Actual request/Promise/receiver/backing/parser admission remains required before live dispatch.

## Test-First Boundary

The language-neutral packet is output/fault/schema and fixture, with before/after constructor and outcome finalizers, returned/refused values, null/undefined/false/zero/opaque8193-byte faults and no empty cancellation. Actual first gate:4PASS/3FAIL/164skip171 at02:49:11,682ms,Nx1, seven selected inputs stable. The old constructor test genuinely reports retained=false where the new fault-held contract requires true; the two new groups initially fail at the missing private matcher. A separate outcome-specific run is capturing the original-before-finalizer behavior before implementation. All raw outputs are retained in the ticket.

No end-to-end app rendering, fresh native/Wasm, arbitrary-fault retirement, whole response cleanup or raw InputAck claim follows from this packet.

The second preimplementation run executed0PASS/2FAIL/169skip171 at02:50:06,708ms,Nx1. It genuinely intercepted the attempted outcome finalizer and found that the original outcome was not installed in its parent slot. That behavioral RED is separate from the constructor group's then-missing matcher. The implementation now assigns the original outcome before finalization, retains the first raw finalizer fault in its added private slot word, and leaves faulted slots linked instead of permitting empty cancellation.

The corrected output cohort passed7/164skip171 at02:50:58,754ms,Nx0 with seven stable selected inputs. Ten actual constructor fault cases and twenty returned/refused outcome-finalizer cases execute, alongside the original ownership tests. Output SHAea3c74f5970a8b3ebc54b2e9c2e65d69318e8a0076bf7a00b9b89f0a52c36b27. This is exact fault containment, not arbitrary fault retirement.

A subsequent parent-dispatch test first ran1PASS/1FAIL/170skip172 at02:52:57,921ms. Its failure was a test-only runtime import missing for a production type-only import, not the proposed parent behavior. The test now imports the actual output class explicitly; parent forward-dispatch refusal is being reproduced separately.

The corrected test then genuinely reproduced the parent gap:1PASS/1FAIL/170skip172 at02:53:44,909ms,Nx1, because `state.failed` remained false after the retained output constructor threw. The narrow reserve catch now sets that existing failure flag and retains the first exact value in the existing `state.fault` word before rethrowing. No field, allocation envelope or ABI was added. A second execute is refused before any post; the original child remains in its roster.

The focused parent gate passed2/170skip172 at02:57:14,990ms,Nx0. Eight selected source/schema/fixture hashes stayed stable. The new group executes ten before/after-finalizer × arbitrary-fault cases with zero getter reads and zero later dispatches. Shard SHAa5d2079fb9d7cb9c4c9a359e767dd1a53d0e4261ceb2d225e67a008dbef905c3. This is controlled transport fault containment, not admitted per-request storage, native execution or final retirement.

The broad actor gate then ran169PASS/3FAIL172 at02:57:54,3.13s,Nx1. All22 selected join inputs stayed stable. The three remaining failures are UI first-fragment/cancellation/page integration, not these output-fault tests. Renderer strict checking returned41 diagnostics:15 old UI counter accesses,19 private-pool fixture constructors,7 tutorial joins; none in Shard, Kernel input or output. The raw full/strict logs remain in this ticket. There is no full actor/renderer green or live guest claim.
