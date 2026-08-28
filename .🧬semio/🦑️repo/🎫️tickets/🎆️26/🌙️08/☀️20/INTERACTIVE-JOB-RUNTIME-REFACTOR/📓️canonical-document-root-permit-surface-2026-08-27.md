# Canonical Document Root Permit — Concrete Authority Surface

## Neutral Resource Ledger

Factor the existing reconciliation ledger into an owned UI-domain module, keeping its actual limits unchanged: 64 reservation slots, 32 MiB aggregate bytes, 131,076 aggregate items, and at most 8 MiB per surface reservation. This is the same reservation authority, not a second quota. Native runtime wrappers migrate to its opaque lease rather than copying numeric slot/epoch fields.

Proposed domain types: `UiResidentLimits { items, bytes }`, opaque affine `UiResidentPermit`, typed `UiResidentFault`, and fixed progress. `try_reserve` uses nonblocking admission; `shrink` validates the exact sole owner; `split_output_into` mints the existing second output owner into a pre-admitted empty slot; `close_step` preserves the permit on contention. Scalar-only dropped permits publish an intrinsic per-slot deferred-return bit, and one maintenance step validates/drains it. No lock or allocation belongs in Drop, and a slot cannot be reused while any owner or deferred return remains.

The existing owner-bit semantics remain one reservation with root owner 1 and patch owner 2. Reader aliases do not mint another independently releasable credit: the root owner remains inside the canonical document slot until all readers and typed descendants retire. Opaque slot/epoch identity prevents arbitrary numeric release.

## Root Slot Binding

Replace the independent eight-slot document allocation decision with a canonical document slot bound to the resident permit's exact reservation slot/epoch. One admitted job epoch creates at most one candidate root; the previous root remains attached to its previous epoch, already counted in the same 64-slot ledger. Current/candidate overlap therefore consumes existing reservations, not an additional unmetered allowance. Captured readers retain that same root's original reservation.

`UiDocumentAssembly::open_with_permit` consumes the exact resident permit before any payload allocation, retaining it in the document slot. Its page allocation, fixed metadata initialization, and record placement remain separately measured. Fault/cancel returns the assembly through typed retirement; final slot release closes the root permit only after every descendant and reader is gone. Busy credit retirement leaves the document nonterminal and retries later.

## Finalization Order

The candidate remains an assembly until its complete resident census is available. The exact assembly owner shrinks its reservation, optionally splits the patch output proof, then seals into `UiDocumentLease`. The runtime should not publish a complete candidate reconciler before that ownership transition. This avoids a detached credit outside the root that could be released while a candidate or captured reader still holds physical data.

The runtime `SurfaceDocumentProducer` whole-clone path becomes an exact root read/alias operation, with no independent node copy. The live reconciler replaces its old retained record map; an id-to-ordinal metadata index may remain, but the only payload authority is the canonical document root. The existing-record comparison consumes an alias of that root plus the incoming component, with its already-measured 15,224-byte owner/root-move admission and 4-KiB comparison/copy progress.

## Required Native Laws

Schema-first neutral vectors and native tests cover nine small simultaneously admitted surfaces; aggregate/slot exhaustion; contention and retry; exact epoch reuse; candidate cancellation; output/root split ordering; a captured final reader that keeps both the payload and credit live; deferred-return contention; and slot reuse only after typed final retirement. Original runtime R30/R31 must turn GREEN through the actual path, not be excluded. Existing 32-KiB physical, 4-KiB component, 8-MiB surface and 32-MiB aggregate grants remain unchanged.

This is the concrete implementation proposal, not a completed native authority join. Current assembly R52/full UI R53 validate only the prerequisite storage/read API.
