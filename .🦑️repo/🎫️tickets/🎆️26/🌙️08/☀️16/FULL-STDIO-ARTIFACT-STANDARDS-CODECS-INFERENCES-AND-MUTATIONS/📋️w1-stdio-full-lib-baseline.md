# Stdio Full Rust Library Baseline

## Gate

`CARGO_TARGET_DIR=<ticket>/🎯️target/stdio-outcome cargo test -p semio-s-plugin-stdio --lib`

Executed on 2026-08-16 after the complete `MutationOutcome` compile migration. The preceding `--no-run` gate completed successfully.

## Result

- Passed: 3,436
- Failed: 75
- Ignored: 3
- Measured: 0
- Filtered: 0
- Runtime: 63.51 seconds

This is a functional baseline, not a green claim.

## Failure Families

- BCF 2.1: shipped fixture differs from the deterministic current ZIP/XML encoding.
- Binary raw: empty extent inference default does not equal the computed empty extent.
- DOCX: fixture/grammar and integrated package round-trip drift.
- DWG AC1024 and bridges: native entity/mesh/path/unknown-object round-trip failures.
- DXF R12: hand-built bounds expectation drift.
- glTF: text/binary leaf inference transport round-trip plus three runtime-capability registry parity failures.
- IFC 2x3/COBie: fixture/grammar/ops and raw-mutation construction failures.
- PDF 1.7: grammar, fixture, mutation/diff, native-byte lifecycle, and derived profile construction/composition failures.
- PPTX: fixture/grammar/ops/diff/package preservation and strict/transitional composition failures.
- Semio CAD/drawing/mesh DWG bridges: relocated codec integration failures.
- SVG 1.1/basic/tiny: fixture/grammar/ops, mutation retention/field sweep, derived construction, and demo inference determinism failures.
- XLSX: fixture/grammar and mutation codec retention failures.
- XML 1.0/valid: committed grammar parse, fixture, state/ops grammar, and integrated round-trip failures.
- ZIP 2.0: rich archive order, fixture/state grammar, and empty inference-default failures.

## Repair Order

1. Preserve the now-green compile boundary and repair failures in artifact-family shards.
2. Reject expectation weakening: fixtures must be regenerated only from verified deterministic encoders, and defaults must represent computed empty inputs.
3. Re-run focused tests per family, then the complete library gate.
4. Keep glTF runtime-capability registration separate from create-scene mutation acceptance.
