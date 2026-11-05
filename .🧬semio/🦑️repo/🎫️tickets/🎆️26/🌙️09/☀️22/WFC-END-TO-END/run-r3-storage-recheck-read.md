# Native Storage Recheck Callgraph

This is the bounded read-only source follow-up requested by Root. No backing-tree walk, filesystem bulk probe, instrumentation, source/policy edit, process stop, or new queue was performed.

## Actual compiler reservation path

Repository `⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts:38-45` acquires or recovers the compiler-workspace allocation with image.port and returns that actual NativeStorageReservationV1. It does not replace recheck with a journal-only shortcut.

Framework `🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts:112-118` defines that reservation:
1. recheck is an accepted original operation.
2. It first awaits optional port.measure(backing, {mode: recheck, requestBytes: 0}, actual).
3. It then reads durable allocation custody, validates matching owner/kind/bytes/phase, and verifies any allocation namespace.

Therefore recheck is not necessarily only a bounded journal/local-allocation lookup.

## Fixed backing measurement path

`.../💾️storage/🧱️fixed/🟦️.ts:100-105` constructs a NativeDirectoryMeter for the full image root. Its measure port validates original backing, rechecks physical mapping, awaits meter.admit, and rechecks mapping again.

`.../💾️storage/📁️directory/🟦️.ts:97-112` defines the meter. A stale recheck calls refresh, whose default measure is usage.fresh. Authored policy is:
- stale interval: 2,000 ms;
- cumulative reserve interval: 16,777,216 bytes.

measuredAt is the measurement start time (lines 101-102), not its completion time. A walk longer than two seconds is consequently already stale at the next meter recheck. This is a source fact, not a diagnosis of the current await.

## Whole-root asynchronous walk

`.../💾️storage/📁️directory/📏️usage/🟦️.ts:33-79` performs the default measurement. It starts at the passed root and recursively enumerates directories and lstats entries throughout that root, not only the current compiler lane or durable allocation rows. It sums allocated blocks and logical sizes; hard links are counted once; links are not traversed; vanished entries are skipped.

The authored walker limits are concurrency 16, batch 8, maximum depth 64, and an entry bound derived from the physical envelope (at least 1,048,576 entries, otherwise ceil(envelopeBytes/4096)*2). Abort is checked between batches. There is no requirement that the complete tree remain unchanged for a walk to finish.

## Freshness does not wait for quiet logs

Usage cell implementation is at lines 101-143. It has one running and at most one queued trailing measurement. A caller arriving while a measurement runs joins a trailing walk that begins afterward. Arbitrarily many concurrent callers share those slots. It does NOT restart the whole walk merely because a compiler or GC log changes, and does NOT wait for a globally stable timestamp or quiet backing.

The registry is a process-local Map keyed by resolved root. It coalesces callers in one process; this code does not coordinate measurements across different launcher processes.

## Supported observation limits

Current normal `repo doctor --facts --only=leases,capacity,processes --json` facts show process ancestry, image-reader custody, capacity grants/waits and lease cycles. They do not expose the private usage-cell running/queued slots, visited-entry counters or exact current await. The public in-process usage API has peek/recent/fresh, but no remote status port for attaching to the existing process was found in this bounded path. Available repo MCP tool metadata in this session exposes ticket/goal operations, not a cache-status meter observer. No unsupported probe was introduced.

Conclusion: this code DOES permit stale storage rechecks to await an entire large shared backing-tree measurement, and multiple processes can independently perform such work. That can explain a long I/O interval in principle. It is not a stability wait caused by changing logs, and it does not establish that PID 54688 is currently inside that measurement. Original handle 4662 and frozen source remain preserved.

All source paths above are under `C:/git/semio/🧰️framework`; repository library paths begin `🛍️products/🦑️repo/🔨️modules/📚️library`, framework module paths begin `🔨️modules`.
