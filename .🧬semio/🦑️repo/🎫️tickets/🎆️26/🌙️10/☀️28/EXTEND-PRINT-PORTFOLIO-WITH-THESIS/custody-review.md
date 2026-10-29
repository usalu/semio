# Graph Queue Handoff Source Audit

Read-only review of generation custody, loader and queue-yield modules. No claims, processes or caches were modified.

## Exact Wait Mechanism

The yield front door consumes/removes its ticket and then polls only whether pointer generation advances beyond the recorded generation or wakeMilliseconds elapses. It does not query claim liveness, CPU/memory headroom or normal lease holder counts in this wait. Core reports policy wake600000milliseconds. Therefore a live frontdoor alone with no child and empty normal lease status can intentionally wait the entire bounded interval even when no current builder exists; this is not itself proof of an external owner or a stranded self-claim.

The next start receives original requestedAt and incremented round. Graph yielding is allowed only with offered ticket, bounded maximum rounds and exceeded yield-after time. Loader timer yields only while held=false; after it enters builder callback, held=true suppresses timer-yield.

## Self-Wait and Release Analysis

Custody admission excludes only the exact same token, not every claim of the same process. Two separate unreleased own tokens could conflict in principle. However the reviewed host wraps actual builder callback in try/finally await claim.release; release deletes in-memory own row and claim document then releases lease. Loader attempts also release the claim handle after normal refresh; release is idempotent. An exception during discovery bubbles through the host finally, so that ordinary fallback path should not retain the same claim. No concrete nested claim acquisition before prior release was found in these reviewed paths.

Process exit while queued can leave a claim document, but another custody instance probes its claim lease and treats it as dead only when it acquires that resource; no stale/dead status was assumed here. Exact live custody rows through an authorized read-only metadata adapter remain needed to establish current blockers. Core owns that observation. Ordinary repo leases0holders/waiters alone does not inspect this separate custody state.

Root reports canonical CLI69241 currently waiting frontdoor-only following a runtime Source-entry validation fallback. These observations plus source explain bounded passive wait; they do not prove canonical execution success or identify a live external blocker. No heavy process launched by reviewer.
