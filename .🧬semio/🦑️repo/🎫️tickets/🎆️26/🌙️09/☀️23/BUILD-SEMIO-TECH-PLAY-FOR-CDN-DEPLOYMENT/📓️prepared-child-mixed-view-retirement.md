# Prepared Child Mixed View Retirement

The private planner retains the original live view and final future view. An intermediate immutable root can contain both old Model and new Brep entries, whose exact retaining views differ. Cancellation must choose the exact current root/page/entry retainer from these two already-owned views, retire intermediate scaffolds, and extract/return each original prepared reader before staged snapshot abort. No ambient registry lookup or fault fallback is allowed.

The initial future root is a pointer-identical Arc alias of the original live root. Current close_prepared_structure_step blocks that alias despite a separately retained exact same root guaranteeing no physical allocation is freed. External readers without such an exact known retainer must continue to block. The canonical repair will return only a proven alias with physical0; actual empty page/root final allocations still require a whole grant.

Schema-first six cases cover exact known root/page/entry identity, mixed original/future retention, an unrelated external root, and zero-item denial. The independent SQLite source oracle joins concrete owner identities and checks releasedBytes0. Existing prepared-child source family command15099 is pending with this absent-query baseline. Native prepared_child_content_ family9120 is pending: new real Arc/HeapWitness law asserts zero-items retain pointer, exact same-root alias releases0, unrelated external reader blocks, returned external alias frees0, final root one-below remains owned and exact whole root release matches allocator. Production alias behavior and new retainer query remain unchanged until actual baseline.

No runtime success or final planner clearance is claimed.
