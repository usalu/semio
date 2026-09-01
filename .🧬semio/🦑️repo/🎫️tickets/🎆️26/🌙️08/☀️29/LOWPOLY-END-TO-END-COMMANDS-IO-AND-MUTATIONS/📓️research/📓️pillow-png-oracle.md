# Pillow PNG Oracle

## Result

`io-lowpoly-png-1` now owns the former PNG row from `io-lowpoly-1`. Its Rust subject exports the
fixture with lowpoly's production `serialize_bytes()`, retains the import round-trip law, and publishes
those exact bytes. Its Python oracle opens those bytes through `PIL.Image.open` and `verify()`, then
compares independently decoded width, height, RGBA pixel format and bytes, plus the PNG `tEXt` keyword.
It never reconstructs a PNG or derives the lowpoly DSL text.

The explicit `@oracle-input-subject-raw` test-plan contract carries a subject raw artifact into a
byte-decoding oracle. The existing execution order is preserved for all other cases; this explicit
case runs the subject first.

## Verification

- `bun ./📜️script.ts test discover` reports 172 cases and discovers `io-lowpoly-png-1` with Rust and
  Python adapters.
- Direct `validateCaseContract` for `io-lowpoly-png-1` returns no breaches.
- A Pillow 12.2.0 adapter probe accepts a valid RGBA PNG with the required `tEXt` chunk and rejects a
  corrupted stream.
- The targeted root `test contract` and `test run` commands are blocked before case execution by
  repository-wide pre-existing contract breaches (1,814 and 1,808 respectively); neither report
  contains an `io-lowpoly-png-1` or `lowpoly-io-png-pillow` breach.
- The direct parity phase reached the generated Rust host but exceeded its 300-second Cargo budget
  without compiler output, reporting shared target-directory lock contention from concurrent work.
- The test-platform TypeScript lint is blocked by five pre-existing type errors outside this change.
