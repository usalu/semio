# Graph Cache Capacity

Selected12 failed during graph assembly with ENOSPC on the declared 4 GiB graph filesystem. This is a current integration failure, not a backend acceptance result. Its original producer was cancelled by its owner; no peer was stopped and no raw deletion or image resize was used.

## Read-only allocation evidence

The graph filesystem had about 13 MiB available. Its inference area occupied about 3.63 GiB, including about 2.397 GiB of selected inference and 968 MiB of canonical native project caches. One selected namespace alone occupied 2,157,544 KiB, almost entirely optional `np` records. Snapshots occupied 2,732 KiB; build input and lock closures occupied 177,932 KiB and 5,080 KiB. These measured allocations disprove a count-only retained-namespace policy as a sufficient capacity bound.

The graph reset API intentionally retains Source facts, closures and immutable snapshots. Its selection-qualified state lock is not a global maintenance lock. It cannot lawfully recover this optional cache allocation by recursively removing the inference namespace.

## Approved contract

The optional accelerator pool is limited to 1 GiB, with a 512 MiB maintenance low-water mark and at least 1 GiB of physical headroom. Maintenance uses genuine whole-image exclusive custody, followed by reopening. Every qualified fact writer uses one original reentrant global exclusive writer lease. Optional persistence reserves exact peak allocation before writing, including replacement pack/index and temporary publication allocations. Denial may skip only optional accelerator persistence. Computed values, complete observed Source facts and original producer publications remain required. Graph, authority, pin and producer publication failures remain hard failures.

The optional namespace whitelist must identify cache data precisely. Mandatory producer publications and receipts can be nested inside inference, so the entire inference directory cannot be treated as disposable cache data. Record-store read descriptors must close within original writer custody before maintenance can reclaim files.

## Verification

The permanent language-neutral `graphCacheQuota` fixture and existing graph suite now contain six focused laws. The final genuine SDK/storage invocation passed **6 tests, 171 assertions, 13.26 seconds**, exit 0. Its retained log is `🗑️generated/graph-cache-quota-final.txt`.

- Independent SQLite arithmetic and Ajv schema admission agree with exact pool/headroom boundaries, pending reservation competition, replacement peaks before retirement, invalid requests and token settlement. A genuine original finite envelope expires without admitting a reservation.
- Actual record pack and JSON index file sizes equal the reserved peak. Compaction accounts for the replacement before retiring old files. Three denied flushes clear all pending records and create no persisted files.
- The strict optional leaf whitelist admits only declared cache records under canonical, selected and dataset namespaces. It excludes mandatory publications, PID producer records, receipts, canonical graph, authority, snapshots, closures and path escapes.
- A genuine enforcing fact-store and dataset hold the remaining issued optional quota while the actual nested NP/node/dependency observation pipeline runs. It returns the real value and complete SHA Source facts, publishes both mandatory original producer files and omits the reusable receipt and optional records. Replacing only that owned fixture publication with a directory makes the original OS publication error propagate.
- A failing original closer does not prevent a later closer from closing an actual file descriptor; the independent OS probe returns EBADF and the first failure remains visible.
- A genuine writer holds its real store while an independent original 150 ms invocation expires without entering the body. A separate genuine 20-second original first acquires the real leases store, then waits on the exact same global writer resource. Its matching progress callback awaits the original port and a short delay before applying a narrower owned cancellation; the lease never admits, and the original signal stays intact. Exact same-invocation reentry returns the same store, a genuinely issued different nested invocation is refused, and a fresh original enters after the first releases. Each long-lived fixture retains its separate original 20-second budget; the short contender is never extended. The separate progress proof removes an invalid assumption that a cold 150 ms admission must reach its first progress callback.

The earlier four-law invocation performed genuine whole-image exclusive maintenance through the current API. Its log, `graph-cache-quota-integration.txt`, reports optional bytes **2,793,004,707 → 525,234,119**, with **2,926,776,320 bytes available** afterward. There was no raw live cache deletion, resize, lease override or peer termination. The original fixture control remained 20 seconds; only Bun's unrelated default 5-second test timeout was aligned to 25 seconds so the original control could govern the operation. The outer genuine SDK gate remained 120 seconds.

The first missing-allocator red is retained in `graph-cache-quota-red.txt`; `graph-cache-quota-pure.txt` records the independent three-law stage. An earlier record attempt failed before tests at a concurrently incomplete Cargo import. Another initial invocation lacked the required generated-output environment, and a subsequent genuine-store invocation hit Bun's default test timeout; neither was acceptance evidence.

These focused results prove bounded optional persistence and the observed controlled recovery. They do not prove a current selected or all-workspace graph publication, backend task execution or final planner/Stdio coverage. Independent source review and the coherent full suite remain separate gates.

## Investigated mandatory publication size

A 14.3 MB NP header led to a provisional concern that per-project mandatory publications might repeat those arrays thousands of times. The actual observer filters original inference publications to the same directory as the dependency receipt. NP project records live in a different directory, disproving that extrapolation. No compact publication format, schema or consumer change was implemented. Any future format change requires an actual outer-publication size failure, rather than this disproven projection.

## Current producer progress snapshot

Root's independent read-only `selected13-graph-capacity.txt` snapshot observed the genuine Selected13 producer at 240 assembly events, beyond Selected12's ENOSPC point at 159. The graph filesystem was 40% used: 1,531,500 KiB used and 2,325,868 KiB available, about 2 minutes 50 seconds into that producer. This is evidence of bounded storage at a later assembly point, not a completed graph, pin, backend run or final ALL acceptance. No cache mutation was performed for the snapshot.
