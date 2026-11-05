# Independent Drawing Null Paint Audit — 2026-11-04

Read-only source audit, no builds/tests/source edits. Read the null-paint implementation report and current diff plus bounded schema/model/admission/oracle/scene sources.

No actionable regression established in the two changed guards. Canonical schema DrawingLayerNode.attributes fill and stroke each admit their paint definition or null. TypeScript DrawingAttributes explicitly declares optional nullable fields at schema/🟦️.ts:41; parser preserves null at lines131/133. The Rust attributes model uses Option for both fields. Skipping null/undefined in the admission generator therefore matches the existing typed contract, while unchanged nonnull validation still checks colors, stroke width/dashes and gradient geometry/stops.

Scene plan Paint carries fill/stroke as nullable values. Authored gradient preparation treats absent/null fill as no gradient rows, and owned plan preparation handles explicit null fill and stroke without dereference. No changed renderer inheritance or clear logic was introduced by this fix.

The5 added neutral cases cover each null paint, both null paints, and each null side paired with a malformed nonnull opposite paint. Existing checkDrawingAdmission iterates every corpus.cases row, applies its RFC6902 patch through fast-json-patch, compares independent graphValid acceptance to vector.valid, then compares actual original-source cursor acceptance to the same expectation. The independent graph oracle uses Ajv scalar/paint projections and separate Kahn graph traversal. It deliberately accepts absent/null paint via its existing truthy guard. This is an independent projection oracle, not a claim that it compiles the complete canonical Drawing schema.

The new bun:test entry calls checkDrawingAdmission(graphValid). The declared Draw test router invokes that entry for admission selection and at the start of the default test run, and forwards the Original cancellation signal. Manifest runtimeSourceEntries includes the exact executable test module. The shared Rust fixture consumer reads the same physical corpus file. Source wiring establishes execution eligibility; the attempted commands stopped at source admission, so actual green execution remains pending and is not inferred.

No new runtime dependencies or fixture copies observed. No extra feature or rendering behavior is required by this narrow repair.
