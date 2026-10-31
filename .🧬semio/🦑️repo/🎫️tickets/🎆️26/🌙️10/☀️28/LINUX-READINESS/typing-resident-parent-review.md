# Typing Resident Parent Review

Current Source was inspected after actual36178 compiler refusal and Root parent rewrite. No Source edits, compiler/tests, Native, lease or process mutations were performed.

## Concrete Remaining Issues

1. In typing/🦀️.rs, cold APIs live under impl<M:static> TypingLedger<M> (line346), and send397/commit409/abort448 now invoke retained_read_pending. That method still exists only under impl<M:Send+Sync+static> at line473. The cold impl does not establish those bounds, so the call cannot resolve for its declared generic M. The query only examines the slot has_read state; use its actual necessary bounds rather than widening all cold API constraints without a reason. This is a signature-level prospective compile blocker.

2. In read/transfer/🦀️.rs, withdraw_original now invokes the supplied busy predicate before mutation, and TypingLedger supplies original_write_pending. However current install_resident_original does not invoke that predicate before installing into a vacant resident slot. Root stated both paths should exclude an active Original writer, so installation is an incomplete guard application. A retained key may remain valid while OriginalTypingEdit holds its writer; generic installation must respect the same owner exclusion before exchange, not rely on an incidental later revision refusal. Preserve all existing key/grant/slot admission checks as well.

## Verified Parent Delta

TypingState now has the intended net Vec. TypingLedger now has Vec<TypingWindow<M>>, source revision/live count and writing state; resident mounts immediately after the module inner docstring. Its sorted position and original_slot methods provide one coherent lookup path. The conflicting TypingLedger map_slots macro invocation is removed; Gesture remains independently mapped. Retained readers/cursor and cold run/keep/commit/lapse/abort methods use current row/slot/net APIs instead of BTreeMap indexing. No leftover map get(window)/keys/values/range operation was found in the current typing parent. SharedUtf8::lease_demand is called without the erroneous question mark.

Current resume/persist convert between cold ToolTransaction positional keyed entries and canonical net. These are real ordinary cold paths with their existing Clone bound, not proof that Original edit exchange clones its net. Actual heap/identity tests must still establish funded native paths and all consumer updates. The earlier large sorted-Vec insertion/growth liveness concern remains a separate required runtime proof; this review does not restate it as an observed compiler failure.

This review is deliberately scoped to the parent current signatures/guards and does not claim all plugin commit/follow/schema consumers are already repaired or compiled. Root owns the next actual registered tests and any causal fix.
