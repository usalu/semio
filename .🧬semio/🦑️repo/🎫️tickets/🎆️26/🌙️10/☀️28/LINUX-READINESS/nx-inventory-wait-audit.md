# Nx Inventory Silence

The observed identical count of 21 inventory progress events does not establish that either request remains in inventory traversal. This review inspected Source only; no Native, graph, compiler, or process mutation was invoked.

Owner: 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts.

- Line143 bytes publishes only every16 visited entries; the final partial batch has no completion event. Line144 inventory publishes once before each namespace and then calls bytes. Consequently21 is a combined namespace/checkpoint count, not a completed-entry count.
- Lines241–242 acquireNamespace can finish inventory, check physical statfs capacity, take a zero-wait shared namespace lease, and then await retainNxGraphChildV1 inside the pool pass. There is no explicit inventory-complete or child-admission progress event at that transition.
- Lines46–49 openNxGraphNativeChildV1 observes persisted native allocations, may recover and commit an existing reserved charge, and then calls existing child-storage admission; a new child uses funded child storage. Line70 retainNxGraphChildV1 awaits its ready promise until that body receives a native image. These are concrete downstream silent await sites, not evidence of a proven bottleneck.
- Lines179–181 handle a busy namespace by releasing the pool and awaiting its shared lease. Lines202–206 take the queued eviction gate and then a pool roster. Capacity retry progress at186 happens only after an eviction attempt throws capacity exhaustion; no nx:graph-capacity event therefore does not exclude a prior eviction gate or native admission wait.
- Lines214–220 eviction may recheck store, measure bytes, await native child retirement, release victim custody, then publish. Those operations are also after inventory output and can be silent until completion.

The next discriminating evidence is the actual blocked resource/native child operation or process stack/CPU from the owner’s existing diagnostics. The finite inventory count alone supports no hot-walk or deadlock claim. Existing cancellation control is threaded into native lifetime admission and lease waits; no timeout or guard change is recommended from this observation.
