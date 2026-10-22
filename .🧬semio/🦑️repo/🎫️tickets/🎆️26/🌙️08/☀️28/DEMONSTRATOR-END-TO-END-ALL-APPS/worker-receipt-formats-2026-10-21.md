# Current Optional Receipt Formats

Date:2026-10-21. Repair follows the actual SourceDemand9977 terminal failure before assertions: closure worker failed “Complete inference closure publication required” after bounded decompression. No private mounted-cache payload was read, so the exact offending physical leaf is not attributed.

## Proven mismatch and repair

Current source classed every inference np/ip/rr24hex JSON leaf as the newline-header inference format. Independent producer traces prove three distinct current formats:

| Coordinate | Current writer/reader | Format and validation |
| --- | --- | --- |
| np/<24hex>.json and top-level emoji-project-json/test-cases.json | observation observeInferenceComputation/readInferencePhysicalRecord | Brotli header + newline + JSON result; closed header, result SHA-256 and named observation blocks |
| ip/<24hex>.json | project-assembly precisionReceiptsV1; precision readReceipt/deriveProgramV1 | Brotli plain ProgramReceipt JSON; actual closed ProgramReceipt schema |
| rr/<24hex>.json | Cargo receipt encode/decode | Brotli plain sealed Cargo leaf JSON; owned Leaf schema, canonical bare-leaf check digest and crate-derived filename |

These valid current ip/rr bytes can lack the newline expected by the old worker. No legacy parser or corrupt-cache assumption is needed to prove this mismatch.

The owner now emits precision and cargo-receipt roles for ip and rr. The worker uses the same bounded Brotli budget and fatal UTF8 parser, then the actual ProgramReceipt schema or owned Cargo validator/canonical seal/filename helper. It keeps np/top inference header decoding unchanged and feeds every complete validated object, including property keys, into the existing reference scan. Physical descriptor stamps, member hashes, required publication failures, KernelEX/intents and all budgets remain unchanged.

A concurrent change had added two shallow path-selected alternate parsers under kind=inference. They bypassed exact schema/seal/filename rules. Those two branches were removed from the latest source, and explicit wrong-kind regressions now use real rr/ip parent directories so restoring either branch would make the tests fail. No generic optional catch or alternate historical representation remains.

## Test-first boundary

Before production role/decoder changes, saved:

- closures/🧪️tests/📇️formats/🧬️schema/🔣️.json
- closures/🧪️tests/📇️formats/🧫️cases.json
- closures/🧪️tests/📇️formats/🟦️.ts
- main closure test import and RetirementPlan member-kind schema additions

The15 language-neutral cases cover canonical/selected/dataset precision and Cargo roles, unchanged inference, precision extra-field refusal, Cargo seal/filename refusal, inference body-hash refusal, missing inference header and two explicit wrong-kind refusals.

Actual-function tests use the current precisionReceiptsV1 writer/readback, actual Cargo encode/decode and canonical validator, actual inference header/packer, and actual worker payload/bytesAt/reference functions with real bounded temporary files. Ajv independently checks the owned receipt schemas; SQLite independently projects roles and all text/property-key reference identities. No Native test was run outside repo.

## Defining Source

Closure's own loaded/current guard now captures its existing owner/schema/worker plus four exact producer dependencies:

- build/inputs/precision/schema
- build/inputs/Cargo-resolution TS
- build/inputs/Cargo-resolution schema
- build/inputs/Cargo-resolution/receipt MJS

All remain below the existing65536-byte per-file capture bound. The shared first-party validator import is already used by the proof owner and the source-inventory worker dependency. Root separately integrated the same four producer paths into storage consumer, Graph and inference authority inventories after schema/fixture tests.

## Changed paths

Relative to the library's graph/headroom/closures owner:

- 🟦️.ts
- 👷️read.mjs
- 🧬️schema/🔣️.json
- 🧪️tests/🟦️.ts
- 🧪️tests/📇️formats/🧬️schema/🔣️.json
- 🧪️tests/📇️formats/🧫️cases.json
- 🧪️tests/📇️formats/🟦️.ts

All exact new imports/read resources were checked to exist; new paths fit256 UTF16 units. Source review is not assertion evidence. Actual maintenance and unit/catalog/architecture/E2E verification remain pending root dispatch.


### Fresh Combined Verification

2026-10-21T13:05:03.218Z: Combined root/High/Low review approved the strict role-only repair and all four defining Source rosters. Focused SourceDemand9780 admitted normally with20,279MiB available and unchanged6,553MiB floor, then reached genuine repository:graph-closure-proof exclusive progress. No worker failure or Bun assertion result has been observed at this boundary. Other heavy goal checks remain held until snapshot/backend; no foreign action occurred.
