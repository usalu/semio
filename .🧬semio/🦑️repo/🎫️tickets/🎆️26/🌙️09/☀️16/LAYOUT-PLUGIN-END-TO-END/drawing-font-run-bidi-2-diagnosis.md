# Run Metric and Pixel Oracle Diagnosis

The captured actual test request83953 ended outer1 with8 pass/2 fail; strict36085 ended outer2. All66 pre/testpost/strictpost/current inputs remain exact. No repair or repeat has run.

## Actual Runtime Boundary

All existing and new Bidi laws passed, as did Run original-denial and every resident-child cancellation law. The first failed metric is the second glyph origin: expected18.94921875, actual16.72265625. Actual and reference first glyph identity/advance checks precede that assertion and passed. The difference is exactly2.2265625 pixels, or190 font units at24/2048. The pixel law builds its independent path from the same reference() positions and therefore inherits this displacement; its observed maximum channel delta is255. No pixel tolerance or geometry mutation is proposed.

## Installed Primary Source and Actual Font Metadata

The installed OpenType Font.getKerningValue uses position.defaultKerningTables initialized from getDefaultScriptName(). The defining selector chooses DFLT before latn. Position.getKerningValue returns the first matching lookup, rather than summing per-lookup results.

Read-only actual Anta table-directory metadata is retained in generated/drawing-ts-font-run-bidi-2/static-font-metadata.json: unitsPerEm2048; no kern table; DFLT kern lookups[1]; latn kern lookups[1,0]. This is static input metadata, not an executed new glyph oracle or parity proof.

Our production FontKerningJob explicitly prefers latn, collects every selected kern lookup and sums its actual adjustments. This matches the existing independent owning Font test interface, which calls foreign.position.getKerningTables("latn","dflt") and sums foreign.position.getKerningValue([lookup],left,right) per lookup. The new Run reference helper instead used foreign.getKerningValue(previous,glyph), silently selecting DFLT and omitting the additional Latin lookup.

This evidence identifies an oracle selection mismatch as the concrete candidate cause of both failures. A corrected actual run is still required; no new reference result is claimed.

## Proposed Exact Delta

Preserve all production66 Source bytes, glyph coordinates, stock, limits, fuel and pixel threshold. Add explicit neutral test script/language selectors latn/dflt. Within the private external-tool reference interface, authenticate the installed position method shape, select those declared tables and sum its per-lookup numeric results; only the legitimate absent-GPOS kern-table path uses the foreign public kern API. Do not copy our parser, table arithmetic or implementation; the independent library still supplies every adjustment.

The two strict diagnostics are test-only in the unchanged foreign corpus mirror loop: JSON mirrors infer number[][] so destructured index may be undefined. Add a closed-pair length/value guard before indexing the actual foreign Uint8Array. All21 original expect sites and fixtures remain.

No Source changes have been applied. Root review and a fresh successor command/closure are required before either normal run or strict repeat.
