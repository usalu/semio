# Record Store Retirement

Whole-workspace preservation gate86491 completed with exit1 before any test leaf: a retained record handle tried to stat an index already retired by another handle. New language-neutral operation traces reproduced the failure. Rewriting now captures existing and absent immutable leaf identities, accounts only present bytes, revalidates before any publication write, and reconciles a changed retirement snapshot under a schema-owned three-attempt bound. The requested key array is retained across those attempts. Only the old snapshot pack names are retired; newly published peer packs remain.

A second causal regression showed same-pack readers overwrote each other in the process descriptor LRU. Each handle now owns a unique internal symbol slot, preserving the32descriptor bound. The Linux kernel /proc/self/fd census independently confirms the actual descriptor count for40same-pack readers.

Neutral stale traces, immutable-event SQLite projection, index Ajv/fast-glob checks, peer-publication/continuous-churn/decline/nonfile cases and physical FD law pass:7tests,1886assertions,exit0. Existing descriptor-limit tests also passed in a focused pure run. The actual three-root TypeScript program (implementation/mainlaws/descriptorlaws) has zero diagnostics over its entire imported closure, exit0.

Publication custody was independently reviewed through observation→optional-publication→ledger.publish→exclusive withGate: it spans snapshot precheck, publication and retirement. No lease rank, quota, writer guard or corruption refusal was weakened. Actual canonical per-file target and workspace gate remain unproved until their executions settle.

Evidence: generated/record-compaction-stale-red.log; record-shared-descriptors-causal-red.log; record-retirement-accounting.log (caught key-iterator loss during retry); record-retirement-physical-fd.log; record-store-types.log. See record-store-retirement-review.md.

Direct nested C/script test records was refused before execution: Original parent Script process invocation required. This received-only script needs its real outer RootCLI or Nx caller; its guard was preserved. Submitted the actual outer RootCLI test/cache/records entry, no fabricated envelope.

Outer RootCLI test/cache/records also correctly refused before tests: its fresh-birth infrastructure route admits declared codeSuites only, while records is a process suite requiring the canonical Nx target. No code or grant was widened to force a run. Both commands terminal1 immediately and no Native job remains. Actual repo:test-records and records-processes must run via their intended target after daemon/linker readiness.

Canonical actual85617 AUTH0: normal repo run-many --projects=repo --targets=test-records,test-records-processes --parallel=1 completed both prototype targets plus1prerequisite through their intended Nx route. The earlier direct-route capability refusals were not weakened. NX_DAEMON=false selected; full caller/default daemon acceptance remains separate. The framework graph/Source freshness path executed normally; no capacity/lease/quota bypass.
