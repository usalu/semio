# Prepared Scene Text Byte Reader

## Owned API

`OwnedUiSceneDocument.beginTextBytes(id, offset?, length?)` and the corresponding `OwnedUiPreparedScene.beginTextBytes` create an exact captured field reader. The root remains immutable and privately owned. Negative/non-integer range metadata rejects before a reader capture; bounds exceeding the actual field reject before any payload bytes are emitted. Offsets are UTF-8 byte offsets, intentionally allowing a raw slice through a multibyte scalar for a downstream incremental lexer. This is not a decoded JavaScript substring operation.

Each emitted chunk is at most 256 bytes, with 2×count+32 accounting for the source read, owned output write and fixed metadata. The read step therefore emits at most 544 accounted bytes after the existing retained index lookup/closure phases. No whole field string or byte array is constructed. Existing raw source and index retirement cursors still own all roots; releasing the prepared owner does not invalidate a held byte reader, and cancelling the last reader returns its root through explicit close. These logical bounds are not a new platform timing certificate.

## Executed Evidence

Canonical target: `bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t SceneTextBytes'`.

- R1: one FAIL, 611 skipped, 612 discovered; 10.45 seconds. The new schema had a missing closing brace and failed Vite JSON parsing. No semantic range test ran.
- R2: one FAIL, 611 skipped, 612 discovered; 11.68 seconds. With the schema corrected, strict Ajv and the prepared fixture reached missing `beginTextBytes`.
- R3: one PASS, 611 skipped, 612 discovered; 14.50 seconds. Six exact UTF-8 range vectors, four invalid ranges, an 8000-byte Unicode field, bounded multi-chunk output, prepared-owner release before readers, zero-grant cancellation and last-reader byte invalidation execute. Node Buffer is the independent byte/slice oracle.

The fixture is language neutral at `ui/contract/retained/fixtures/scene-text-bytes{,.schema}.json`. A targeted diff whitespace check passed for both scene modules and both fixtures. The broader canonical `-t Scene` regression R4 passed 15 tests, 600 skipped, 615 discovered, 15.02 seconds. It includes the existing generic Scene, typed projection and scene-binding tests. Neither result is a full renderer pass.

## Remaining Scope

This reader supplies a needed owned byte source for the next JSON lexer/parser; it is not that parser. Nested JSON duplicate-key ordering, escaped strings, exact numeric-token policy, typed host projections, managed React scene byte reads and live renderer adoption remain open. No whole-object compatibility conversion or empty unsupported-host fallback was added.
