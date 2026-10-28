# Native Receiving and Original Post Custody Audit

Read-only source audit of the exact 23 paths in drawing-receiving-files.md plus the central preparation and MutationApply files below. No native compilation, macro expansion, or runtime test occurred; the required repo runner is unavailable. Grammar results are not compilation. No source was edited by this audit.

## Findings

1. Confirmed source-identity instability: `C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs`, lines155–159, records `addr_of!(self.snapshot_bytes)` as the snapshot address. MemberOpenRequest is not pinned. Moving the request changes the identity on its next loan despite unchanged sealed pages, operation and generation. Core confirmed and is repairing this with stable pages.slot_address plus explicit original range coordinates. Producer registration must wait for that repair.

2. Constructor/header receipt gap: `C:/git/semio/🧰️framework/🔨️modules/🎒️pack/🌱️value/🛬️decode/🫴️receiving/🦀️.rs`, lines35–44. Both admit variants accept zero copy-byte credit, construct and transfer the full RetainedRecordReceiving header, and return copied_items1 with copied_bytes0. Their grant gate checks only items/depth. Quote and fund the actual fixed construction/transfer metadata before construction, report that paid work, and test each undergrant axis. RecordBody symbol-table construction uses two empty PagedLists backed by Vec::new (list lines165–177), so this audit did not find immediate heap allocation in that constructor. Document constructor error paths contain internal close loops; preserve their allocation-free construction assumption or replace with explicit caller-funded cleanup if future constructor work acquires backing.

3. Confirmed existing central compile errors: `C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🦀️.rs`, struct lines243–270 declares cursor as S::Cursor while close accounting names S::Projection. Constructor lines202–206 and later handlers refer to mutation_source, mutation_authority, returned_edit, returned_edit_retirement, returned_post absent from the struct. TakeProjection lines466–467 places generic cursor.take output in Option<S>, although projection output may be Box<S>. These are pre-existing integration failures observed before Root’s in-progress protocol edit, not effects of the receiving manifest.

4. Required original-post storage integration: the generic projection output must be placed through its actual output contract into stable, funded backing; retaining a pointer to the movable preparation or edit fields is invalid. Hold S::Output or an explicit stable output-placement owner until all final reader/index/continuation and both original bindings close. Then obtain exclusive mutation capability and commit. Do not assume every output is S or unbox a generic output without its placement contract.

5. `C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧬️mutation-apply/🦀️.rs`, lines26–29 and51–53: preflight currently receives bare post:&S, then Stage::Commit mutates post while plan, authored source and semantics remain in self. Current ordinary logic does not itself prove a dangling loan, but the new borrowed final-reader integration must introduce an explicit reader-close barrier before Commit. Existing cleanup happens later in owner_close_step lines101–109. Re-check live version before publication.

## Reviewed Without a Confirmed Additional Defect

NativeValueSource has an object-safe reference-only interface; child references use explicit dyn NativeValueSource and projected original bindings. No generic methods or associated source types break its object safety. The trait’s Sync and static requirements keep recursive trait object recipients compatible with immutable issued ownership. This is source reasoning only.

Generated record and enum slots are boxed to cut concrete storage recursion. DrawingLayerNode intentionally suppresses its implementation where conditions; carrier/direct slot bounds still require native macro expansion and trait selection to verify. Direct tagged payloads project the actual variant_payload rather than manufacturing a JSON carrier. No confirmed undeclared direct slot was found in the generator.

PagedMap uses a separately retained paged scalar index and exact key byte comparison. Duplicate-key refusal returns the funded turn receipt; normal index close occurs after child/output custody and before original binding. Paged text subtracts bool metadata and one depth level before original append, returning added metadata receipt. All these owners still require native undergrant, cancellation, HeapWitness and retirement execution.

Body/Document Driver::one signatures and the two Materializer call sites inspected consistently pass source, range and maximum. RecordCursor forwards original ranges. This scoped observation is not a whole-repository call-site type check.

## Exact Central Paths

- `C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🦀️.rs`
- `C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧬️mutation-apply/🦀️.rs`
- `C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/🧬️semantic/🦀️.rs`

The complete receiving manifest is preserved in [drawing-receiving-files.md](drawing-receiving-files.md). Core has not mounted a real Draw MemberStoreProducer/SnapshotOpen; this audit does not approve catalog registration or claim native receive completion.
