# Allocation Budget Semantics

The retained byte limit bounds requested payload capacity before reservation and accepts actual collection capacity only after checking it against the remaining limit. It cannot promise a ceiling for all temporary allocator work, allocator private bookkeeping, or resident process memory. Fixed traversal frames and borrowed source owners are separately bounded and excluded from this payload capacity budget.

Rust documents that `try_reserve_exact` can receive more capacity than requested, so a pre-reservation size check alone cannot justify a hard physical-memory ceiling. See the primary [Vec::try_reserve_exact documentation](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.try_reserve_exact).

The audit follow-up keeps the requested capacity limit and actual-capacity admission check explicit. An oversized empty reservation must be rejected before payload initialization and must never become an accepted partial value; releasing that empty reservation has no nested payload destructor. Neutral requested/allocated/budget cases inject larger reservation capacities and verify both the pre-allocation and post-reservation rejection boundaries. No allocator-private memory ceiling is claimed.

Implementation and native validation of the injected reservation tests are pending. This resolves the contract direction; it is not passing evidence.

## Validation Result

Both reservation boundaries now have injected capacity tests. `canonical-close-receipt-green-2.log` and `clone-close-zero-green-2.log` are terminal GREEN, 5/5 native tests each. Requested capacity is checked before reservation and reported actual capacity is checked before admitting empty storage; public contracts now state this meaning explicitly.
