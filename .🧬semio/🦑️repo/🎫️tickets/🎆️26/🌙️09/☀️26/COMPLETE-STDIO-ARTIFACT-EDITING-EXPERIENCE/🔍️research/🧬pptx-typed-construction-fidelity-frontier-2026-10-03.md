# PowerPoint Typed Construction Fidelity Frontier

The current full native suite passes132/132, including the eight previously failing cases. A separate source-level fidelity issue remains in the base schema derived construction implementation.

PptxBuilderConstruction::from_snapshot derives a typed presentation view from canonical OPC/XML. Its add_slide and add_paragraph methods edit that view, then rebuild calls build_minimal_pptx on the view. The projection only contains modeled slide content, so running these constructors on an imported presentation replaces the original package/XML with a minimal package and can discard custom parts, unused relationships, unknown properties, slide sizes, notes, master/theme details and unmodeled XML. This is source-supported risk; no dedicated runtime witness has yet executed. The current editor mutation route uses canonical XML addresses and is separate.

Required fix: remove the competing cached construction view as an editing authority. Append slides/shapes/paragraphs directly to canonical XML/OPC using resolved namespaces, unused relationship and shape identities, stable part ownership and current revision addresses. Preserve existing parts and subtree contents, including unknown extensions. Blank construction may synthesize a minimal package only when starting from a truly empty builder. Invalid imported packages must retain diagnostics.

Required neutral witnesses: import presentation with custom part/archive comment/unmodeled node properties; append paragraph and assert old subtree is an exact prefix plus one new paragraph; append slide and assert existing parts are identical while only presentation list and new required references change. Include noncanonical prefixes and Strict namespaces, plus independent ZIP/XML inspection of saved/reopened artifacts. Preserve existing132tests.

This frontier must be handled before claiming every editing boundary is lossless. It is queued behind current browser/full-composition repair rather than described as already fixed.

