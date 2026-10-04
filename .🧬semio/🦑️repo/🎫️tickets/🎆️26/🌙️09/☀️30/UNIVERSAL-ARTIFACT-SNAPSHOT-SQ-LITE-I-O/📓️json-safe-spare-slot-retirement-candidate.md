# JSON Safe Spare Slot Retirement Candidate

Read-only independent algorithm review,2026-10-03. No implementation mounted, no Cargo/runtime claim. Root's proposed safe owned-value continuation can avoid layout overlays and unsafe pointers.

## Precise Candidate Invariant

`current:JsonValue` owns the active value. `pending:JsonValue` is either Null (no suspended frame) or an Array/Object whose last slot is a continuation sentinel containing the previous pending value. Every pending non-Null frame must have that sentinel, even when the previous pending value is Null. The sentinel is never an authored child.

When an Array pops an authored child, its Vec has a proven spare slot. If authored siblings remain, push the old pending value into that same slot, put the parent Array into pending, and descend to the child. If no siblings remain, drop the now-empty parent Vec immediately and descend with pending unchanged. Object does the same, drops the popped member key, and uses a zero-allocation empty String key for the sentinel JsonMember. Its value holds old pending. Scalar parents require no frame.

When current finishes, if pending is Null traversal is complete. Otherwise take its Array/Object frame, pop exactly the last sentinel into pending, discard the sentinel's empty object key if applicable, and resume the remaining authored parent. A resumed flag avoids recounting that branch as a new authored node; a subsequent sibling descent installs a new sentinel in its freshly vacated slot.

## Safety and Allocation Proof

Vec::pop reduces len by one without reallocating. Pushing exactly one sentinel back cannot exceed pre-pop capacity, so it requests no allocation. String::new has no allocation. Moving enum values and Vec headers allocates nothing. Scalars release their original number/string payloads; popped object keys release immediately; arrays/objects release backing storage exactly once after the last authored child is moved out. Existing vector ownership expresses every continuation and no custom frame representation or pointer provenance is needed.

An active pending chain must never be dropped recursively. The loop takes and unwraps each sentinel before retiring its frame; no return, fallible operation, callback, formatting, assertion failure or panic branch belongs while that chain is live. Valid matches are only Array/Object for a non-Null pending frame, established by construction. Error handling through unwinding would otherwise invoke ordinary recursive enum drop.

## Linear Work Proof

Each authored node enters once, each child is popped once, each retained parent sentinel is pushed and popped once per child descent that has siblings, and every suspended frame resumes once. The number of sentinel operations is at most the number of authored edges. A wide parent does not revisit previously retired children or recensus its Vec. Counting only first visits gives exact authored node count; counting operational visits still has a fixed linear bound. Auxiliary stack is constant: current, pending, a parent being resumed and move temporaries; retained continuation storage reuses existing vector allocations.

Array-only/object-only single-child deep chains need no pending frames because their parent vectors become empty immediately. Alternating branches with siblings exercises the real suspended chain. Root Null and empty branches retire directly; scalar Null children do not confuse the sentinel because retrieval is determined by pending frame position, not by child variant.

## Critical Interpretation and Counterexample

The condition 'if remaining nonnull' must mean **if the parent has remaining authored siblings**, not 'if old pending is non-Null'. Skipping sentinel insertion solely because old pending is Null is incorrect: root Array `[String("a"),String("b")]` pops b, retains a, then resume would pop a as the supposed sentinel and misclassify it as pending. Unconditional sentinel insertion for every retained parent, including Null old pending, fixes this. Equally, a retained parent with no siblings must still insert a sentinel if it is retained at all; the empty-parent optimization instead drops it and leaves pending unchanged.

This proof is source-level reasoning. Actual all-allocation-refused narrow-stack and complete live-byte release laws from the future lifecycle design remain required before claiming JSON runtime retirement is allocation-free or linear.
