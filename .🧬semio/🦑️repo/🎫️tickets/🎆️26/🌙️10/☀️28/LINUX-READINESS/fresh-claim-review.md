# Fresh Graph Claim Admission Review

Read-only Source review, 2026-10-29. No tests launched and no Source changed.

No concrete blocker found in the fresh-claim delta. The public process lease implementation exports acquireFreshResourceLeaseV1, and its TypeScript package barrel explicitly reexports it. The helper overrides caller timing to timeoutMs 0 and the existing canonical gate admission patience; it forwards the existing control and signal unchanged. Public acquisition still enters acquireResourceLease and its existing order guard; there is no new order-policy or reservation-floor exception.

The queued claim factory now requires three distinct functions: queued acquisition, ordinary nonwaiting probe, and fresh admission. The declaration file requires the same three fields and models tryHold(resource, {fresh:true}). All three production graph constructors, the process worker constructor and the control/kernel test constructors supply acquireFresh. Fresh admission appears only for newly generated custody tokens and prefetch claim tokens. Existing claim liveness and slot probes retain the ordinary true-zero-wait path. Queued hold/wait, shared-cohort behavior, follow cancellation and original signal/control wiring are unchanged.

For fresh requests, tryHold returns undefined only for the exact requested-resource timeout message without resourceLeaseAdmission metadata. Directory admission failures and unrelated timeout messages propagate. This preserves the distinction between resource collision and infrastructure admission failure. The admission helper still annotates exact admission-resource timeout errors with the requested resource and original request context. The existing broad ordinary-probe timeout matcher is unchanged by this patch and is not evidence that fresh failures are swallowed.

Reviewed Source boundaries: process/🔒️leases/🟦️.ts:287–310; process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts:1; library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.mjs:194–219 and its declaration; custody module:106; claim registry:120; graph constructors:152,530,560. These are beneath 🧰️framework/🔨️modules and 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️ respectively.

Execution status belongs to the owner: focused real SQLite admission tests reportedly passed; the combined claims/custody/kernel gate was live when requested. This report does not claim that gate completed.
