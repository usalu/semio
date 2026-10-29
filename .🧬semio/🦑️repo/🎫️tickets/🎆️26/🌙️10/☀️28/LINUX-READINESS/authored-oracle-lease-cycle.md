# Authored Oracle Graph Lease Cycle

The registered search oracle request (handle 35573) ended with status one before entering its test leaf. The complete request output remains in `🗑️generated/search-oracle-target.log`; it has not been replaced or retried. It includes both the selected guard victim and the parent error propagation.

The enforced error was `LeaseOrderErrorV1`, code `SEMIO_LEASE_ORDER`, kind `cycle`, at timestamp `1793220225554`. Both reported processes used implementation identity `3a919ca870e12533416066c6c90f98168955cdd046e3abb57bb76cec670f`.

| Process | Held | Requested |
| --- | --- | --- |
| 382037 | Repository graph facts shard 7; generation `1a2aa20d6098af716af43425` | Exclusive graph-custody admission |
| 382302 | Repository graph facts shard 7; generation `e1889b4ea0010f721850f2d6`; graph-custody admission | Generation `984cf3b6060091640d8ffb5f` |

The violation record identifies the held facts lease as shared, rank four, class `graph`; requested admission is exclusive, rank zero, class `unclassified`. The guard selected process 382037 to fail. Enforcement remained enabled.

The recorded source stack begins at:

- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts:287`, `react`, throwing `LeaseOrderErrorV1`.
- The same source at line 334, `contention`.
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts:665`, `acquireWaitingOutsideGate`.

Parent stderr then records `OwnedCommandFailure: build:graph-plan failed (1)` from `🎛️owned-execution/🟦️.ts:338`, followed by `OwnedCommandFailure: repository:command failed (1)` at line 185. Both outcomes have status one and a null signal. The runtime reports Bun 1.4.2 on Linux x64.

These source locations are the URLs and line numbers reported by the captured runtime, not a claim that subsequently edited live files have identical bytes. The parent owns the graph acquisition-order repair; the read-only auditor is locating its exact acquisition sites. The Go candidate remains frozen, and the independent Go/SQLite/projection gates remain passed. The two existing generator requests remain live; no lease guard was altered.
