# IFC Model View History Inputs

The 19-crate Stdio capture executed actual native history assertions for IFC2x3 Cobie, CV20 and SAV and found zero committed editable cases. Existing native schema owners use externally tagged `SetViewDefinition { view }`; all three intentionally share the native Part21 document and the same header editing primitive. The new literal committed records preserve that authority: each starts with the IFC2X3 file schema and a `CoordinationView_V2.0` stamp, then changes only the stamp to `StructuralAnalysisView`. These are metadata intent cases, not a claim that a resulting document conforms to every model view.

The language-neutral source test uses the actual snapshot and leaf JSON schemas through Ajv and independently computes the exact before/after document with RFC6902. The first run `tools-ifc-mvd-committed-history-source-red` failed on an incorrect test relative path and is retained as a harness failure. The corrected test run `tools-ifc-mvd-committed-history-source-red-b` actually failed 0/1, five expectations, after schema and RFC6902 checks reached the absent committed record. The three records were then authored manually. `tools-ifc-mvd-committed-history-source-green` actually passed 1/1 at 10:04:09.695 UTC with three visible DEBUG receipts. No mutation semantics, schema, native codec or replay authority was changed.

After the old PDF/MD focused controllers were terminal and the 10:07 physical census found their literal selectors absent, the exact native strict successor `tools-ifc-pdf-md-committed-history-native` (42959) started. It selects IFC (15), PDF (30) and Markdown (3) named laws: 48 required assertions. This is pending runtime proof; it adds no editor ledger credit. The original full19 and35 obligations remain intact.

## 10:30 UTC Actual Editor Vocabulary Audit

Current strict 48-law capture executed all three IFC2x3 subset history laws and refused each because zero committed fixture mutations decoded. The three subset editors use the base `Ifc2x3Mutation`, while the authored view records use each subset’s distinct `SetViewDefinition` vocabulary. Their independent source proof does not establish editor acceptance. No mutation type or assertion was changed to disguise that mismatch.

The snapshot’s native EDM preamble has a schema-typed producer string and is handled by the editors’ actual base `PatchSnapshot` vocabulary. A new neutral test validates this producer change with Ajv and RFC6902, preserving the Part-21 document; its committed fixture is withheld until the intended missing-record RED executes. Current native PDF/Markdown laws remain live in the same capture, with no per-crate receipt yet.

## 10:34 UTC Actual Producer Case TDD

Corrected neutral capture 57266 ran the existing view test successfully, then failed only the missing EDM producer committed record after Ajv snapshot validation, shared patch validation, RFC6902 mutation, and unchanged Part-21 document checks: 1 pass, 1 fail, 19 expectations. Three literal subset records now use the actual editor `Ifc2x3Mutation::PatchSnapshot` wire at `/edmPreamble/producer`, with the complete native preamble fields. Source GREEN capture 58638 is pending; no native producer or editor type changed. The existing own-vocabulary view records remain useful domain proof and do not establish generic editor acceptance.

## 10:34 UTC Actual Source GREEN

Capture58638 actually passed2/2,26expectations at10:34:27.952Z. Six visible DEBUG receipts distinguish the own-vocabulary model-view tests from the base-editor EDM producer tests for all three subsets. The compiled native48-law capture still contains the prior three history refusals; a current native recapture is pending.
