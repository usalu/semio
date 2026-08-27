# Selected Flow Copy Review

## Source Boundary

The coordinator read the full retained selected-copy implementation and all five native laws after the explicit allocation-admission patch. Private rooted projections keep the immutable source Arc alive across task/worker boundaries. The copier walks typed records, vectors, strings, options and boxes; unchanged owned ordered maps, sets and Dictionary roots share pointers. Cancellation retains partial outputs and tasks for typed retirement before releasing the source.

The constructor no longer reserves payload storage. A dedicated phase checks single and cumulative byte allowances before try_reserve_exact; later string pages fill only disjoint initialized slices, and vector children fill pre-reserved slots. Allocation failure latches the copy as failed while retaining the original owner for explicit close. External root retirement is checked for overgrant and terminal-empty; the factory Arc is included in the final owner sequence.

Five native laws are queued: serde equality/shared-root identity, cancellation/absent projection, strict unclosed Drop, hostile root-retirement overgrant/factory cleanup, and allocation admission/no-reallocation. Source review is not their execution.

## Remaining Acceptance Boundaries

The allocation test records the fixture's measured reservation time while allowing 16 MiB per reservation and 32 MiB cumulative. It does not exercise those maximum admitted sizes or assert the 8 ms ceiling. Final concrete factory acceptance must derive memory limits from its actual envelope, bind them to global worker admission, and time/assert maximum-envelope turns. A local counter is not itself a process-wide memory reservation.

Generic root-retirement factories may own arbitrary data. Concrete interactive owners must supply a stateless or separately retained-close factory, not infer bounded final destruction from an Arc alone.

These boundaries remain open until the actual Flow/Procedural2d/Procedural3d parameter commands are registered and verified through Store publication, cancellation, undo and fresh Wasm.
