# Executable Body Tessellation Concept

The goal and ticket remain active. This is one explicit released concept, with caller-supplied target applicability; it is not full Reference View, Alignment View or all-MVD conformance.

## Source And Contract

The IFC4x3 Body Tessellation module pins buildingSMART Standards-builds commit610ee3f95db9d0380b1eb8de4dc6e17833531518 and the released concept HTML SHA256 b8f905fdf6d7bcb4afa69d20111600589e52f4bef43043b8a7814e0b5da78d32. Its closed definition hand-translates the three released clauses: RepresentationIdentifier equals Body, RepresentationType equals Tessellation, and every Items member is a subtype of IfcTessellatedFaceSet. The separate source/test plan records authenticated URLs and the applicability limits of the element usage table.

Requests identify unique positive canonical u64 representation IDs and explicit true, false or unknown applicability. False targets are excluded without lookup; unknown applicability is incomplete. An unresolved or ambiguous source, target, attribute or reference is incomplete; a known violated predicate fails. Aggregate precedence is incomplete, fail, pass, then all excluded. Empty Items satisfies this universal concept predicate; the separate EXPRESS cardinality is not silently added as a fourth clause. Reports explicitly keep schemaConformance and viewConformance not-evaluated.

## Actual Implementation

The public IFC4x3 TypeScript entry exports the evaluator and first-party contract types. Closed request admission precedes source admission, both using the central owned copier. A private ID index preserves ambiguous IDs rather than choosing one. Predicate evaluation uses the shared declaration/defined-type/subtype machinery, inherited simple attributes and declared complex constituent attributes. Typed labels must actually follow the IfcLabel constructor chain. Diagnostics include bilingual labels and exact source pointers, including constituent and item ordinals.

Original captured controls cover request census, source copy, indexing, concept evaluation and unavailable-result construction. Interior and completion boundaries honor the original cancellation; generator cleanup retires pending work. No callback exception is replaced with a domain failure. Central snapshot capture is private after completion, while callers must keep their input stable during capture.

## Executed Evidence

The concept suite passes31 tests /146 assertions. It covers nine neutral independent cases, closed AJV contracts, immutable provenance, full u64 IDs, typed labels, complex constituents, exact pointers, missing/ambiguous values, explicit applicability and aggregate precedence, request-before-source/no-getter refusal, private source capture, original interior/completion cancellation and callback failure identity. Targeted strict TypeScript checking passes.

The independent IfcOpenShell0.9.0.post1 oracle creates physical authored triangle, polygon and polyline examples and records exact representation IDs. The actual production parseIfc4x3Step to validateIfcBodyTessellation endpoint matches all nine independently traversed statuses. These minimum authored examples are not mislabeled as official example files or complete EXPRESS-valid models. The latest complete registered run includes this suite:680 pass /15935 assertions /12 genuine gated skips /zero failures across42 suites in124.01 seconds.

## Native Boundary

The native worker has authored the same explicit evaluator under original source/request/caller pins, the shared neutral source index and immutable static metadata, with retained diagnostic allocation and retirement. Neutral and adversarial prefix/close tests are authored. Rustfmt parses the source. Authentic Cargo still refuses at its compiler version probe before rustc, so there is no native type/runtime/receipt/heap proof. See body-tessellation-native-implementation.md and the independent audit for the current source boundary.
