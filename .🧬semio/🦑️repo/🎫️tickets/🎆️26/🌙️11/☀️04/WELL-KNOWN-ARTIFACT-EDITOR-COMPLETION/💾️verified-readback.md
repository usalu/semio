# Generic Verified Readback

Extracted awaitVerifiedReadbackV1<T> with exported VerifiedReadbackListenerV1<T> in the existing catalog acknowledgment module. It retains cancellation, timeout, subscription lifetime, submission/probe ordering and current-owner checks. Acceptance is supplied as accepts(T). The catalog wrapper still uses the exact existing admit/retire predicate, including selected surface and unrelated concurrent document tolerance. No external runtime dependency was added.

Schema-first byte fixtures use independent Buffer.compare and Ajv. New tests establish that persisted alone does not acknowledge content; older archives and byte-length differences remain pending; accepted exact bytes settle once; cancel, retirement, timeout and submit/probe exceptions clean listeners and timers; snapshots before submit cannot acknowledge; synchronous abort during subscription still unsubscribes exactly once.

Test-first red:7 new failures because generic helper was absent, while existing12 catalog tests passed. After extraction:19/19pass across7 byte,6 catalog predicate/waiter and6 actual hook tests,15.12seconds. Focused config vitest.verified-readback.config.mjs.
