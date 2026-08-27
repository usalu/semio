# Retained Numeric Index Checkpoint

The first persistent dual-index packet is implemented in framework value `ordered/numeric`. Both trees use fixed-size AVL nodes; numeric lookup is independent of the insertion-ordinal tree. Replacement retains an ordinal; delete/reinsert appends. IDs preserve the full nonnegative safe-integer range. Ordinals are two safe-integer words; exhaustion rejects before allocating a candidate entry or changing the source. Constructor seeds are copied, not retained caller objects.

Captured owners retain both exact roots in constant work. Edit and reader cursors retain their source. Cancellation transfers frames and candidate nodes to an explicit one-item retirement queue, with borrowed frames released before their source roots. A final entry hands its unchanged payload to the owning domain exactly once; the primitive does not recursively destroy arbitrary domain values. No public root/node/entry mutation or root-rebinding API is exposed.

Canonical command: `NX_DAEMON=false NX_ISOLATE_PLUGINS=false NX_CACHE_PROJECT_GRAPH=false bun x nx run @semio-tech/value-numeric-index:test --skip-nx-cache`.

Executed green r2: 10 semantic/grant cases, 37 cancellation/concurrent-reader laws, 2 ordinal laws, 3,072 differential operations, 5 invalid-ID cases; strict TypeScript component diagnostics zero. Strict Ajv validates the language-neutral fixture. Existing Immer plus native Map independently supplies values and insertion order. Full stdout is `🧪️numeric-index-green-r2-2026-08-27.txt`. Earlier lifecycle RED exposed a test assuming that a ready notification had already been emitted at the candidate-construction boundary; cancellation now closes that still-private candidate without extracting it.

The minimum metadata admission is one item/256 bytes. Edits/reads/retirement return actual logical field-work counts, never more than 256 bytes per step; larger 4,096-byte grants are also tested. This is not an empirical JavaScript allocation/GC or browser eight-millisecond certification. Payload retirement belongs to the domain caller. Unretired abandoned JavaScript owners remain a caller lifecycle obligation; this implementation does not claim Rust-style destructor enforcement.

The actual renderer patch/decode/validation/hash/notification/ACK path is not yet replaced. This checkpoint certifies the isolated prerequisite only; the executor continues into that integration.
