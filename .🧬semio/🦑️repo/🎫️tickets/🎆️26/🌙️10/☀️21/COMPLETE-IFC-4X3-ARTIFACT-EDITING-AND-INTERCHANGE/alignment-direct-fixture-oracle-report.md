# Direct Counterpart Fixture Oracle

The authored input `alignment-direct-fixture-oracle.py` references the four pinned original assets in place and creates independent minimal ADD2 graphs. Outputs are in `🗑️generated/alignment-direct/oracle.json`, sixteen authored physical files, and the separate mixed-case result. IfcOpenShell 0.9.0.post1 parsed the original four and all sixteen authored physical cases. Four malformed internal graph laws deliberately have no physical oracle. No production files changed; no native execution was attempted.

## Original observations

All original source SHA256 values match `alignment-direct-counterpart-vectors.json`. Headers remain exact IFC4X3_ADD2. H1101, V2001 and C6001 expose their own Product.Representation → ProductDefinitionShape.Representations → ShapeRepresentation.Items → CurveSegment, with actual IfcLengthMeasure zero. C149 has no own Representation. Its parent curve's zero segment does not establish a direct counterpart. The existing source plan records the exact IDs and hashes; originals were never copied or rewritten.

## Authored observations and validity

The base model includes a named project, real context/unit assignment, actual local placement, horizontal design parameters, typed curve measures and a CompositeCurve providing the CurveSegment.UsingCurves inverse. Eight of the sixteen main physical inputs have zero full EXPRESS diagnostics: N/A missing request target, known wrong requested kind, selected zero, ParameterValue zero, negative zero, repeated same curve path, two representations, and duplicate request target. Request mistakes are independent of physical validity. Repeated paths retain identical physical curve IDs and are not collapsed by the plain traversal.

The remaining inputs intentionally or incidentally have diagnostics, retained verbatim rather than described as fully valid examples:

| Case | Actual full EXPRESS diagnostics / observation |
|---|---|
| nonzero business + missing own representation | orphaned ProductDefinitionShape.ShapeOfProduct; business length is 1 |
| missing own representation | orphaned ProductDefinitionShape.ShapeOfProduct |
| nonzero direct + unrelated zero decoy | decoy lacks UsingCurves inverse; selected direct measure remains 2 |
| two distinct direct items | second item lacks UsingCurves inverse; both distinct identities observed |
| missing item reference | missing references invalidate both Segments and Items |
| raw SELECT zero | SegmentLength domain rejection |
| unknown SELECT tag | SegmentLength domain rejection and positive-length zero WHERE failure |
| malformed wrapper arity | parser silently retains first argument, then reports zero diagnostics |

The last row is a concrete oracle limitation: IfcOpenShell's physical parser normalizes `IFCLENGTHMEASURE(0.,0.)` to an ordinary one-item measure. It cannot certify our required rejection of malformed internal wrapper arity. Preserve the authored source text and test that internal law directly. Likewise unknown complex constituents, ambiguous owners, duplicate physical source IDs and nonfinite native words require our language-neutral graph tests; no complex physical support is inferred.

The supplemental mixed-case physical tag `iFcLeNgThMeAsUrE(0.)` parses to IfcLengthMeasure and has zero EXPRESS diagnostics. The implementation should canonicalize valid IFC identifier tags, without accepting arbitrary one-item wrappers.

## Integration obligations

Consume the neutral borrowed query result including status and qualified declaration owner; do not use the existing MVD convenience helper's matches-only result to infer uniqueness. Continue scanning unknown constituents after finding two matches. Attribute owners are IfcProduct.Representation, IfcProductRepresentation.Representations, IfcRepresentation.Items, IfcAlignmentSegment.DesignParameters and IfcCurveSegment.SegmentLength. Resolve H/V/C design length on its owning declaration.

Explicit applicability false must not demand the missing selected segment. The selected-zero clause does not assert whole-layout terminality. A known nonzero business precondition fails before geometry lookup; parent-only geometry is incomplete; one unrelated zero item must not satisfy a nonzero selected direct item. Repeated paths and multiple representations remain explicit supported-projection ambiguity, without asserting a universal IFC cardinality restriction. Numeric acceptance distinguishes the exact declared SELECT branch from its numeric payload; negative zero may satisfy zero while source words remain unchanged.

The oracle validates observations and schema separately; it does not execute the forthcoming concept endpoint, establish geometric parent correspondence, or claim full MVD/view conformance.