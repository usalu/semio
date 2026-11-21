# Viewer Live Maintenance Outcome Audit

The Stage53 Viewer console reports at epoch 33667 that live cleanup for instance 2 faulted with prior-outcome. The subsequent held-repeat cleanup is another report of the held failure, not evidence of another root cause.

In 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs, lines 127–134 reduce the checked-out JobOutcomeView::Fault or Cancelled to pump.faulted without retaining its payload. Lines 78–81 acknowledge the outcome; lines 99–102 retire the session and emit generic PriorOutcome. The alternative continuation branch at lines 120–125 sets the same boolean and calls debug_cleanup_continuation with its callback, qualification, or receipt qualification.

Inspect the existing checked-out outcome view and fault payload at line 128 before acknowledgment, and distinguish that branch from the checked-out continuation at line 120. This identifies the original maintenance failure without changing admission, retirement, or receipt semantics. The captured generic WIT failure alone does not establish which branch occurred.

The captured attempt 53 browser continuation was traced to the mounted replacement initializer worker's exact receipt comparison. The issued grant and expected grant both contained 3 items, 27,150 copy bytes, 16,775,776 capacity bytes, 1,046,860 release bytes, and depth 64. Its callback reported 4 copied items, 331 copied bytes, zero retained capacity, and 1,520 released bytes. The copied item axis alone exceeded authority, before the posted-effects comparison.
