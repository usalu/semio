# Queue Read Independent Review — 2026-11-05

Root reviewed the authored production `process/leases/windows/readLeaseQueueTicketV1`, exact queueAhead/awaitQueuedTurn wiring, neutral queue-read corpus and Bun/Python oracle laws. Engine owns RED/GREEN execution and final code. UItext8367 failed before Cargo while opening an exact peer ticket with EPERM; it is not a codec test failure or pass.

The repair retries only the same canonical name, under the existing signal and caller remaining check. Actual queue wiring waits through waitForLeaseContinuation. Existing declared four pauses bound retries; persistent failure is thrown, not interpreted as absence. Only ENOENT represents a released ticket. POSIX sharing-like permission failures remain immediate. Caller/cancellation checks happen before reads, after failed physical reads before classification, and after successful physical reads, outside the catch that classifies physical errors. No peer ticket, ACL, lock, lease policy cap or queue ordering was changed by this review.

Root found no concrete custody/ordering defect in the authored implementation. Requested one missing neutral regression: a successful physical read followed by the same EPERM-coded Original refusal must throw that refusal without retry or publication. Existing authored stages covered before-read, after-failure and during-wait only. Also requested null-safe unknown-error classification so an unusual thrown value cannot be replaced by an accidental TypeError. These are review requests, not a claim that final tests passed.

Engine reported actual RED21076, then GREEN3077 queued under normal admission. Current native workspace71724, Hub52625, component17951 and image62574 remain retained; no duplicate builds or skipped authority checks are authorized by this review. Exact execution and final modified-file listing belong in engine's retained report.

## Review Requests Addressed in Source

Root reread current source and neutral corpus after engine response. The corpus now includes after-success with one physical read and no sleep; helper classification uses null-safe optional access and keeps the post-success caller check outside the physical-read catch. Engine also authored primitive/null physical-throw preservation cases. GREEN3077 remains admitted/queued pending actual execution; root has not independently run it or claimed pass.

## Actual Focused Law Result

Engine reports GREEN3077: all4 new queue laws pass, including after-success, unusual physical thrown values and Python filesystem/JSON oracle. Combined existing Windows selection reports7pass1skip1fail157assertions; the sole failure is an existing real lease-retirement law refusing beforebody because direct repo-tools Bun does not forward an Original process envelope. It is not a queue-law failure, and no fabricated envelope or capability downgrade was used. Authoritative registered-target birth is being checked for that existing law. UItext8367 may now be retried once against the repaired exact-ticket reader because its actual terminal EPERM prerequisite has a verified bounded fix. Image62574 remains retained.

## Corrected Birth and Actual Combined GREEN

Root independently inspected the actual process suite table: leases budget300000, artifactstrue, tree leases, all3hostplatforms. Updated existing Windows test declaration matches this authored policy and wraps the real retirement law in withBuildTestInvocationV1; there is no manually constructed process environment. Engine reports corrected combined54278 exit0:8pass1expectedPOSIX-onlyskip0fail173assertions,9tests across2files in2.54s. All4newqueue laws and the realSQLite/Python retirement-custody law executed green. The skip is the actual POSIX permission test on nativeWindows; platform classification table still exercises all3platformrules. This closes the earlier prebody-refusal validation gap, and does not establish native plugin or other-OS runtime success. Image62574 and UItext35598 remain preparing.
