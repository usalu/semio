# Canonical Document Cutover — Exact Root Admission Boundary

The new assembly/read API is native-green, but the active reconciler still owns its old record map. Replacing that authority remains required; no second persistent old tree has been added.

Before the cross-file cutover, the existing capacities need an explicit join. `UI_DOCUMENT_LEASE_SLOTS` is 8, whereas `SURFACE_RECONCILE_ADMISSION_SLOTS` is 64. The document arena previously held requested transport documents; using it as the canonical root for every live reconciler makes those eight slots a new constraint on all live surfaces, plus current/candidate overlap. A ninth otherwise-admitted live reconciler could become permanently unpublishable. Increasing a constant silently would conceal this change.

The proper ownership join is to bind canonical document root admission to the already-owned reconciliation reservation, accounting current/candidate overlap and final readers, rather than add a second independent eight-slot refusal condition. The existing per-turn 32-KiB physical grant, 4-KiB component work grant, and 8-MiB surface resident limit must remain unchanged. The exact number/lifecycle of canonical root slots needs to follow existing reservation authority, not an invented larger grant.

Further source facts: current document producer `try_new` has no located repository call sites; its implementation nevertheless performs whole `credited_clone` per node and cold builder placement. Its replacement should alias the exact canonical root, not perpetuate that duplicate path. Current native cold `.reconcile` helpers directly mutate the old record map and therefore also need an honest test-oracle migration during root replacement.

Current runtime R30/R31 are actual semantic RED and remain open. UI R52 is only the assembly prerequisite. No Process acceptance, all-app capacity, or resident total is established by it.

## Captured Reader Credit Ownership

The slot count is not the only join. `SurfaceReconciler::retire_one` releases its persistent reconciliation credit, while a canonical document read alias can keep the physical root alive. Replacing `retained` with a lease and releasing the credit after a nonfinal read close would undercount a still-live captured document. Credit must follow the final canonical root owner, including cancelled component comparisons and renderer readers; increasing the document arena size alone does not solve that.

The existing ledger can split one reservation into exactly owner bits 1 and 2 (reconciler/patch). It does not yet provide arbitrary captured-document credit aliases. A sound next packet must retain one exact root-associated credit owner through all readers and hand the credit back only after actual final root retirement. No numeric slot lookup or bare copied generation may authorize releasing another root's credit. All shared aliases must use the same final-owner protocol, and terminal retirement must be nonblocking under ledger contention.
