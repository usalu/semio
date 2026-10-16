# CPU Custodian Final Source Audit

2026-10-15. Read-only source audit of the direct-child custodian foundation after reading `cpu-custodian-foundation-current.md` and `cpu-custodian-foundation-independent-current.md`. No tests, probes, compilation or runtime execution were performed. Only this ticket report was written.

## Findings

No new actionable findings in the inspected direct-child foundation scope.

The accepted server operation now sends the bounded refusal and rethrows the exact caught capture error. `withIssuedResourceOperationV1` tracks that rejected promise through its actual control ancestors; its failure array records even an `undefined` rejection by array length, and settlement drains accepted promises before reporting the recorded failure. The server finally block waits its pending set before releasing capacity. The requesting client separately rejects the refused request inside its registered operation. Consequently the server failure is retained on its own Original ledger as well as on the client ledger; catching the client rejection does not erase either ledger.

Detached client `port.run` work is registered before its asynchronous body sends the request. Callback return sends drain and joins actual custodian close; accepted server captures are drained before capacity release, and the client control subsequently drains its tracked requests. Source failure tracking retains detached rejection even after its promise leaves the pending set. The callback path uses an explicit `failed` boolean and throws `failure` before any cleanup failure, preserving direct thrown `undefined` as the primary failure.

The client port has no local compiler-spawn capability. Jobs are declared before physical admission, the actual custodian acquires existing capacity with an empty marker environment, concurrent requested counts are checked against the paid pool, and job birth stays inside accepted server operations. Disconnect/cancel stops admission and aborts captures while retaining the server-owned reservation through their settlement.

## Limits

This is source confirmation only. Existing runtime/strict results are reported by the foundation owner and were not independently rerun here. The test source includes detached/refusal actors, callback Error/undefined preservation actors and a missing-program server-ledger actor, but this audit does not independently establish their runtime outcomes.

The qualification covers owned direct children only. Arbitrary native descendants, custodian crash survival, actual job thread-count enforcement, all-platform native ownership and the production inherited-marker bypass remain unqualified. No production integration or native tree provider was reviewed or claimed.
