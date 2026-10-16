# Nested Phase Lease Storage Failure

Actual permanent NESTED41927 terminated1 before task action: SQLite initialization for resource leases returned SQLITE_FULL. The actual parent awaited the planner and test child exit; no automatic retry or later phase ran. Full log: `🗑️generated/bazel-direct-phase-nested-registry-current.log`. Reported Bun aggregate0pass/2fail/1assertion/18.53s retains its enrolled failure separately from the law.

Root read-only filesystem observation03:25:43UTC found the canonical lease mount `/dev/disk13s1` at64,472KiB total,64,448KiB used,24KiB available. The host volume had95,733,388KiB available. Tools, graph, dependency and product mounts also had available blocks. A subsequent exact df receipt is retained in `🗑️generated/nested-sqlite-full-filesystem.json`; those are later point-in-time measurements, not a historical allocation trace.

High is investigating exact canonical lease-file accounting and the existing safe reclamation path. No quota expansion, raw cache/database deletion, ledger clearing, peer termination or cleanup mutation has been authorized. No culprit, allocation time or canonical reclaimability is inferred from df alone. All owned runtime/store custody is closed while diagnosis proceeds.

## Existing Claims-Only Owner Result

Ticket diagnostic10010 invoked exactly the canonical `openRepositoryCachePartitionV1(root,"leases",genuinePhase)` once, which runs the existing claim-retirement owner, then rechecked and closed the image. Actual terminal0 confirmed03:30:58UTC; log and JSON are `lease-claims-only-recovery.*` under generated. No maintenance exclusion or ordinary/system-file retirement was requested.

The helper's before03:30:27.096UTC and after03:30:28.676UTC observations had the same mount device16777260/inode2,4,669 ordinary database names,44 claim database names and118,784 available bytes. There was no net capacity/count recovery during this operation. Earlier different census values are separate observations; they do not identify an actor, mutation time or the canonical owner responsible. All helper/native-image custody and children closed.

A safe long-term ordinary-database retirement protocol remains under Source-only design and independent review. It must govern direct acquisitions and raw identity readers, prevent old-inode ABA, account for claim-subdirectory gates and prove canonical image exclusion for already-running peers. New gate bootstrap must also work within the unchanged finite image using owner-enrolled headroom; no raw reserve truncation is allowed.

## Later Read-Only Capacity Observation

At03:44:58.456UTC a later statvfs observation found15,339,520available bytes on the same mount device16777260/inode2. This supersedes any assumption that the earlier24KiB/118KiB condition persists indefinitely; it does not change the historical actual SQLITE_FULL result or identify an actor, cleanup operation or operating-system event. Receipt: `lease-pressure-current-observation.json`. Runtime admission remains authoritative. Root has not reclaimed ordinary databases or system files.
