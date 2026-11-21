# Activity Witness Audit

The existing lease standing is not a sufficient witness of fresh authorized work for a Ready deadline grace.

- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:268,384` routes issued resource-control publication through the actual Original Script invocation to `noteLeaseProgressV1`.
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🟦️.ts:39-67` parses phase and counters, but its state is process-global, allows counter decreases and phase replacement, and updates `progressAtMs` for every line. Lines 72-88 also advance that timestamp for CPU activity. It is not an invocation-scoped monotone work high-water.
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts:85-105` binds a row to process PID, birth and the inherited Ready owner, but writes a row only while that process holds or waits for a lease; it deletes the row otherwise and disables the best-effort registry on write failure. There is no guarantee the process publishing dependency work has a live row at that instant.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts:631-639,813-817,886,924-945` derives activity and freshness from a mutable log, log modification time, and general progress/CPU observations. A valid-looking log line alone cannot establish which stage invocation produced it or that the authorized counter advanced.

The cheapest trustworthy writer seam is the existing issued resource-control `publish` path at invocation/process lines 268 and 384, coupled to an exact Ready launch-owned stage grant or owned slice report that captures the declared work counter and its high-water. The reader must bind that evidence to stage, PID, process birth, and launch and require a fresh monotone advance for the same authorized work identity. The existing lease registry can corroborate a live owner but cannot alone provide this guarantee.
