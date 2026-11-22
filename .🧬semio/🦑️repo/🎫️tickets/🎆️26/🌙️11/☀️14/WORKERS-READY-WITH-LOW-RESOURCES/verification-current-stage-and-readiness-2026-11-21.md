# Stage and Low-Resource Readiness Verification

## Stage propagation

The coordinator committed the 248-entry stage `7d8c0b059bcaed05be881279fe59b4fa8305d514` at 17:42:08 UTC on 21 November 2026. All nine workers reporting at 17:43:20 UTC attested that exact tree. The eight workers with current attestations in the final sample still agreed and were running replica digest `5833bd94f16502c16334f2093620201382811632f28abe01fca301731285a7c9`. The three stale origins were `g1-m02`, `g1-m04` and `g1-m06`. The origin `g1-m09` supplied no attestation. Every fresh worker cursor also covered exact journal PUTs for the 15 latest replica, graph and dependency files.

A previous 1,703-entry stage spent about 84 seconds publishing Git prerequisites. During partial intake two physical workers reported an intermediate index tree. The author now commits one journal head per stage, and the follower verifies the committed author position before applying the stage. Source change hints survive restart until their work settles, including partial and temporarily blocked receipts. Successful outbound commits renew the store deadline. The 40-test real-Git replica suite and 36-test filesystem race suite passed.

[Stage investigation and rollout](audit-durable-replica-stage.md) · [Windows replica retry work](audit-ordinary-replica-retry-allocation.md)

## Low-resource graph work

A graph reader waiting for rebuild memory can now use a compatible local seal that another producer publishes. The ordinary shared read validates its exact manifest and source coverage. Missing data can retry with bounded backoff. The loader and whole-gate suites passed 187 tests, and the final exact-head focused suite passed three tests. [Local seal admission](audit-local-seal-during-rebuild-wait.md)

A current graph query previously walked over 178,816 parser-source listing operations and reached 106,496 dependency verification operations under each of two custody modes before issuing its no-sealed-generation diagnostic. Current graph reads now skip task preparation, and a Bun launcher path change no longer invalidates a receipt for the same pinned producer. The native rerun issued the same no-seal diagnostic with no parser-source scans or dependency-generation verification passes. Capacity admission still contributed at least 55 seconds to its 85.9-second elapsed time. Sixteen graph runtime tests and 17 runnable dependency receipt tests passed. The broader dependency run reproduced five baseline failures and recorded one skip. [Native current-query verification](audit-current-graph-runtime-preparation.md) · [Dependency receipt provenance](audit-dependency-bun-launcher-provenance.md)

## Remaining physical acceptance

| Requirement | Current evidence |
| --- | --- |
| Native bootstrap and current Ready on the enrolled 16 GiB fleet | Zero of twelve Ready. Live workspace jobs continue to report progress. Some started before the new implementation reached the stage. |
| Fresh isolated all-stage run at 8,192 MiB and four cores | The original launcher stopped reporting while waiting for disk, and its process is gone. Fresh preflight admitted the path envelope but measured 41.1 GB free against the required 60.1 GB. No machine receipt was exported for that attempt. |
| Complete authenticated graph and adoption on a compatible second physical worker | The coordinator still lacked a sealed whole graph at the last native check. The live reader was waiting for a 14,336 MiB rebuild reservation. The signed adoption verifier returned `receipt-missing` for all twelve worker destinations. |

[Workspace readiness evidence](audit-workspace-after-exact-stage.md) · [Physical admission evidence](audit-current-admission-pulse.md)
