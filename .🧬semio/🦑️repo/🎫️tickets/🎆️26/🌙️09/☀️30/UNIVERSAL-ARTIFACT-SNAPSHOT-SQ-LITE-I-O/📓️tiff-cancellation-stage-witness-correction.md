# TIFF Native Cancellation Witness Stage Correction

## Authentic diagnosis

Root's original TIFF19 receipt d60e2c2b… had 19 run, 18 pass, one fail and 101 outside. Root added encoding/phase/remaining context to the original assertion and reran. Authentic Nextest e3c48687-10fb-462a-ab5f-1f71c207337c again ran 19 tests, 18 passed, one failed, 101 outside in 86 ms. The precise refusal was Text/DecodeNative with remaining 16,777,216 equal to maximum 16,777,216, reached true and canonical ValueError Canceled/native decoding canceled. Receipt: 🗑️generated/root-authentic-tiff19-residual-interior-admission-context-current.log.

Current Text decode validates its borrowed hexadecimal body under begin_stage(body.len()) before allocate_vec(count / 2). The phase-only callback canceled that borrowed pass. The unchanged allocation_stage correctly settled native.owned_bytes() == 0. Binary decode and both encoding directions had already admitted backing at their large progress frontiers. This is a witness stage mismatch, not evidence of a refund.

## Closed neutral correction and tests

allocationBridge retains maximumBytes 16,777,216, refusedBytes 1, copyBytes 100,000, cancelAt 65,536 and passes 2. Its exact closed schema/fixture now add textDecodeStages: borrowedHexValidation start 1 with zero admitted byte copies; ownedHexMaterialization start 2 with one admitted byte copy.

The Native law counts stage starts only when total equals the actual canonical hex body length and completed equals zero. Text Decode cancellation requires the second existing stage, after the existing hex backing admission. It keeps the original positive-backing assertion and additionally requires exact remaining allowance maximum minus body_len / 2. It separately selects the first stage, checks canonical Canceled, and requires zero admission. All Binary/Text input/output directions, positive native admission, exact admission replay, second-pass cumulative refusal, tiny refusal and original payload/limits/threshold remain. No new Native test was added: Root's selector still has 19 tests.

The independent Source law uses Ajv to validate the closed stage contract and reject an unregistered stage property. Buffer's real UTF-8 and hex codecs independently round-trip the 100,000-byte ASCII field. DataView authors and inspects a real TIFF6 carrier with an extra ASCII tag 65535, field type 2, count 100001, explicit value offset and unchanged RGB pixels. A borrowed Buffer view shares the DataView's backing and mutations; Buffer.from(hex, "hex") materializes distinct backing. Those independent library facts establish zero borrowed payload-copy bytes versus one exact owned carrier-byte copy, without mirroring TIFF's validation/copy loops.

## Source TDD and parser receipts

First the schema and independent Source law were staged while the neutral fixture still lacked textDecodeStages. The registered Source route actually failed: 12 pass, 2 fail, 1,665 assertions, 14 tests, 3.56 seconds Bun / 9.0 seconds Nx, exit 1. The failures were closed-schema validation and missing cancellation stage contract. Receipt: 🗑️generated/tiff-cancellation-stage-source-red.log.

After fixture correction and Native witness selection, the same registered Source route passed: 14 pass, 0 fail, 1,710 assertions, 14 tests, 3.56 seconds Bun / 8.6 seconds Nx, exit 0. Receipt: 🗑️generated/tiff-cancellation-stage-source-green.log.

Command: SEMIO_TEST_LEVEL=quick NX_DAEMON=false NX_ISOLATE_PLUGINS=false NX_WORKSPACE_DATA_DIRECTORY=<unique ticket generated directory> bun nx run @semio-tech/stdio-tiff-rs:test-snapshot-sqlite-source --skip-nx-cache. The registered Source wrapper explicitly skips Native execution when source is selected. No Cargo command was run in this lane.

The changed Rust law passes rustfmt --edition 2021 --emit stdout parser validation, exit 0. Receipt: 🗑️generated/tiff-cancellation-stage-native-parser.log. No native compile/runtime success is claimed for the corrected witness; Root owns the next authentic TIFF19 run.

## Exact changed files

- [📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts)
- [🧬️schema/🪶️sqlite/🚦️cohort/🔣️.json](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🧬️schema/🪶️sqlite/🚦️cohort/🔣️.json)
- [🧫️fixtures/🪶️sqlite/🚦️cohort/🔣️.json](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🚦️cohort/🔣️.json)
- [🧪️tests/🪶️sqlite/🚦️cohort/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🚦️cohort/🦀️.rs)

This correction edits only Source/Native tests and closed neutral schema/fixture. No production implementation, controller, semantic schema, limit, feature mount or callback was changed. Earlier typed producer prerequisite reports are separate.
