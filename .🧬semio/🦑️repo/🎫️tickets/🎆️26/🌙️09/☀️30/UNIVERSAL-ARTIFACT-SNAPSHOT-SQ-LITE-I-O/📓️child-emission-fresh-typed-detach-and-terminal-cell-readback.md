# Child Emission Fresh Typed Detach and Terminal Cell Readback

Bounded read-only audit of current child-emission-owned-capsule inputs, especially 🦀️.rs and emit-prepare.rs; no mount, compilation or runtime credit.

The earlier typed-detach defect is repaired: retire_one checks size_of::<M>().max(1) before current.take and before remaining.next, including the close lane. A terminal erased retirement is retained until the grant covers size_of_val of its concrete backing. The original operation-vector allocation is retained until its full backing_bytes grant. Builtin retirement uses a function pointer; custom factory remains owned and prevents Ready until its genuine typed handback. These are concrete source guards, not allocator observations.

The worker checks the preparation fixed cell before stepping, checks output growth before reserve, and publishes only after Ready. The typed take_ready checks readiness, closing state, schema/current/retirement/remaining and factory before taking the wire prefix. The following terminal check is consequently supported by those invariants on the normal Ready path. The owned FIFO queue capacity is released under its full capacity grant when empty.

Remaining required evidence is the coherent member publication-prefix restoration and the actual caller/worker mount, including a refusal after an accepted prefix and a refusal while retiring that accepted typed owner. The protocol transport cause explicitly remains AwaitingInput for a genuine provider handoff; it cannot receive terminal credit from this capsule alone. Allocation of the retirement adapter itself and output reserve overcapacity still require actual owning control/allocator evidence; the static size checks do not establish those observations.
