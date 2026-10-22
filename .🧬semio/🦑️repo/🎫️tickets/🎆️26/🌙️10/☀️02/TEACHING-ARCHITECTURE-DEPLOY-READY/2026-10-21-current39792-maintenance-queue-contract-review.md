# Current39792 Maintenance Queue Contract Review

Bounded current Source audit only; cargo remains sole actual39792 handle owner. Reported EX position2of3 with earliest shared request77217 age501s is diagnostic context, not kernel-holder evidence. No process/lease/cache probes, Native command or Source execution.

Current persistent backend maintenance142 first drains accepted cleanup readers, stops borrower admission and awaits closeNativeImageBorrowerV1; only afterward clears guard, enters gap and calls exclusive acquisition. The close helper awaits both reader cleanup and borrower close, propagating failures before EX request. Fixed backend98 has equivalent close-before-gap ordering. Thus this concrete receiver does not deliberately retain its original shared borrower while waiting for its own exclusive image guard. Other actual independent readers cannot be classified from this static trace.

Graph storage93–98 maintenance consumes one receiver through EX and returns truthful false on nonacquisition/partial. Writer maintain149 opens fresh image per round and closes in finally; namespace writer selection occurs later151+, so this initial optional maintenance does not wait for EX while already holding its later namespace shard. Existing temporary-shard readiness loop releases before reacquiring image as documented in prior call-order audit.

Both backends derive maintenance child with requested0 under the existing control; this retains available ancestor budget rather than minting a new deadline. Current wait helper handles actual child/parent cancellation and preserves earlier acquisition/observer failures (separately reviewed repair). Duration alone neither proves expiry nor exact remaining ancestor grant; only actual refusal/terminal evidence can do that.

QueueWaitInfo404–408 reads earliest retained queue ticket mode/pid and computes holderSeconds from ticket arrival. Position counts predecessor requests; queued is at least position. Displayed shared PID is not independently observed kernel ownership. Shared-cohort admission may still be delayed by an earlier EX/noncohort predecessor not established by the single displayed head. Ticket age does not prove staleness or justify cleanup.

No concrete self-hold, deadline-reset fault or cycle proved in these bounded paths. No scheduler redesign or runtime gating recommendation is made; the ongoing wait itself remains a live owner-managed operation until actual result.

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs — 45154/94fcdb240798a7723c7a250c68b6e69e43a1bfd7f9dafa2c01ab898e06b96e4e
- 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts — 128564/fb94efef6385b63474cefa7e4d56998a4fae67d41121a96493029cdcd16679ed
- 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts — 129948/9e9edf3d619232d52b95486fed0b7761112f57bdb0a901e75900e79347aba04c
- 🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts — 75284/03f0c550dff79a518cd0ed8225f0f6da1bd54a89fd848494763a9ff532600673
- 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/⏳️wait/🟦️.ts — 3047/4a52f66d88cbcb443d5ac1818cdbb5554224cc5b38830481c077c2433e0f537a
