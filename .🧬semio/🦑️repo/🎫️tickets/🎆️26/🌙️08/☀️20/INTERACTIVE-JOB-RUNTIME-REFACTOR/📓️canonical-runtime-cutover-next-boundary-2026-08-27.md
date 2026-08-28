# Canonical Runtime Cutover — Exact Next Boundary

## Coherent Prerequisite State

The runtime now uses the one neutral resident ledger; canonical document slots retain root owner 1 at that ledger's exact slot/epoch; output owner 2 is split from inside the assembly. Shared-reader close does not return the root credit. Typed final-root retirement returns credit only after the root's own descendants retire. R65 is the full UI contract gate: 155 passed. Runtime R41 is explicitly 101 passed with the original three real failures excluded.

The live runtime still contains `SurfaceReconciler.retained: SurfaceLinearMap<UiNodeId, UiNodeRecord, 128>` and `SurfaceReconcileCursor.new_retained` of the same type. No canonical payload was added alongside these old maps. Original R30/R31 remain untouched and failing at their last exact execution.

## Required Single-Authority Replacement

Replace the reconciler's payload map with its canonical `UiDocumentLease`; retain only an id-to-ordinal metadata index. Candidate storage is `UiDocumentAssembly`, opened with the actual job reservation and generation. The already admitted permit must move from `SurfaceReconcileRetained.credit` into that assembly, not be independently reserved by the active cursor. Old current/root epochs remain separate existing reservations.

Record placement must use retained `place_one` stages; the source record stays in a structural field across refusal, error, and unwind. Candidate finalization must retain assembly ownership until the complete simultaneous-owner census is available. Then call `shrink_resident`, `split_resident_output` if needed, and `finish_into` in separate admitted transitions. The ready patch owns the output permit; the reconciler's canonical root owns the original permit. Do not retain the former `persistent_credit` as an independently releasable copy.

Current-record lookup should use exact root/ordinal/id reads. Existing component comparison must own a root alias plus incoming component through `UiDocumentComponentCompare`, debit the measured 15,224-byte initialization/root move, and advance under 4 KiB work. A source/current owner stays structurally outside any unwind boundary. Pending copy and comparison states cannot inflate the existing cursor allocation silently; their real fixed owner and backing admission must be accounted.

## Test-Only Oracle and Producer Surfaces

The existing `#[cfg(test)]` synchronous `reconcile`, `diff_node`, `diff_existing`, `diff_children`, and `remove_subtree` mutate the old map directly. They need a coherent test-harness migration when that map is removed; do not keep a second production representation to satisfy these helpers. The semantic wire/order/assertion coverage must remain.

`SurfaceDocumentProducer` still clones each complete old record into a second builder. Replace that path with exact canonical root capture/alias. Preserve original root identity and distinguish a producer request's generation from root publication identity rather than silently retagging a root. The old public `UiDocumentLease::read_node_page` remains a cold whole-record clone; it is not a bounded exact-read primitive and is not credited by the new root-reader laws.

## Still-Open Accounting and Close Work

The root shrink method validates its fixed slot and actual node-table backing minimum, not a complete runtime census. The full meter must include source tree, flat/traversal/index storage, current and candidate roots, pending record/copy/comparison owners, output pages, and retired/captured-reader overlap before shrinking. Keep 32 KiB physical, 4 KiB component work, 8 MiB surface and 32 MiB aggregate limits unchanged.

The 64-slot document arena is now static fixed backing, not an allocation per open call. Its full fixed metadata footprint still needs an explicit once-only accounting convention in the resident inventory; do not equate an active-slot count with its entire physical storage or infer that the shrink minimum is a complete physical footprint.

Handback entry R40/R41 removes entry blocking, propagates poison/deferred-return faults, and keeps queued state in its slot through a step. Ordinary Drop/admission handback helpers still block; old nested record/tree cleanup still contains whole-value operations. These remain open, not hidden by the three entry tests.

The next decisive runtime gates remain the original R30/R31 unchanged assertions, real nine-reconciler-surface coexistence, retained-reader pressure/cancellation, full resident footprint, and fresh peer Process workshop acceptance. Canonical-root unit tests alone do not establish those outcomes.
