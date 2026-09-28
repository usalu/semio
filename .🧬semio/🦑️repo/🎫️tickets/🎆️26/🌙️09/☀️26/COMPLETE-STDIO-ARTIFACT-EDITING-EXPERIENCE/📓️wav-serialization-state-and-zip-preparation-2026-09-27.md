# WAV Serialization State and ZIP Preparation

## Scope

This slice closes the three WAV serialization-state losses identified by the fidelity checkpoint audit, mounts retained Store preparation for ZIP archive-name and archive-comment edits, and makes ZIP entry/header serialization policy explicit. OPC publication guards are owned by the parallel OPC workstream and are exercised by the same focused native package target.

## WAV exact serialization state

The WAV schema and TypeScript facet now admit only snapshots whose retained state is exactly representable in RIFF/WAVE:

- a nonzero `fmtPadByte`, `dataPadByte`, or auxiliary `padByte` requires an odd wire payload length;
- `fmt.ext` is limited to the unsigned 16-bit `cbSize` ceiling of 65,535 bytes;
- auxiliary FourCC values must contain exactly four printable ASCII wire bytes.

`validate_wav_serialization` is applied to every native aggregate mutation after its candidate diff is applied and before that diff can become document state. `try_encode_wav` repeats the same invariant at the codec boundary and uses checked `u16` conversion for `cbSize`. Artifact pack encoding returns a schema error for invalid state rather than truncating it.

The language-neutral `serialization-boundaries` fixture is consumed by both TypeScript and Rust. Native codec coverage uses the independent `riff` crate to inspect the exact 65,535-byte `fmt` body declaration, primary and auxiliary payloads, and manual wire offsets for three distinct odd-chunk pad bytes. It also verifies typed edit, save, reopen, and second-save equality.

Fixture audit found no committed WAV fixture with a nonzero pad on an even payload. The existing ordered-chunk fixture retains its nonzero pad only on the three-byte `JUNK` payload.

## ZIP retained preparation

The base and ISO 21320 ZIP editors now mount the same artifact-owned `BoundedNativeEditingEditor::native_edit_preparation_route` for `RenameEntry` and `SetArchiveComment`. Recognized mutations never fall through to generic atomic preparation.

The preparation owns these bounded phases:

1. page the exact inverse strings with `RetainedTextCopy`;
2. page the post snapshot schema, every entry name, every entry payload through `RetainedBytesCopy`, and the resulting archive comment;
3. validate the rename target occurs exactly once and the replacement has no collision during the same bounded entry scan;
4. construct the exact forward/inverse edit and hand it with the post `Arc<ZipSnapshot>` to the live authority's one-item sealer;
5. retire partial copies, sealed owners, descriptions, base reads, and authority owners under item and byte grants after cancellation or close.

ZIP now supplies artifact-owned retirement for snapshots and mutations. Its byte owner truncates `String` and `Vec<u8>` storage by the exact byte grant, avoiding the generic `Vec<u8>` retirement path that treats every byte as a separate semantic item.

A language-neutral bounds schema and fixture drive native tests for multi-page payload copying, exact inverse and post snapshots against the mutation algebra, cancellation of a partial payload copy, terminal emptiness, and byte-grant retirement.

Preparation now copies and retires every nested header owner as well as member payloads: ordered local and central extra records, Unicode-path legacy bytes, Unicode-comment legacy bytes, entry comments, timestamps, flags, versions, attributes, compression choice, and data-descriptor signature. The structural ceiling is 4,096 entries plus extra-field records; each encoded header text or extra block remains bounded by the ZIP 16-bit length ceiling of 65,535 bytes.

## ZIP header fidelity

`ZipEntry` now owns `ZipEntryMetadata`, split into explicit local- and central-header state. Stored/Deflate selection, flags, DOS timestamps, version fields, ordered extras, legacy name/comment bytes, internal/external attributes, entry comments, and data-descriptor signature survive decode, edit, save, and reopen. `ZipSnapshot.commentUtf8` records the EOCD comment encoding choice. Snapshot, artifact, and diff facets carry the same state in Rust, TypeScript, JSON Schema, GraphQL, and Proto declarations.

CRC, compressed/uncompressed sizes, local offsets, ZIP64 values, and Unicode-extra CRC/payloads remain derived. Their extra-field positions are represented by empty payload markers, and encoding rejects stale nonempty derived payloads, duplicate derived records, missing legacy owners, unsupported compression/encryption state, and states that cannot reopen as equal snapshots. Unknown extra records retain their bytes and order.

The neutral `header-fidelity` fixture covers a stored directory with empty payload/comment and separate local/central unknown extras, a Deflate Unicode member with CP437 legacy path/comment bytes and a signed data descriptor, a NUL/high-byte CP437 entry comment, attributes, timestamps, and a CP437 EOCD comment. The native test compares exact typed reopen equality and asks the independent `zip` crate to verify the local and central unknown extra bytes, directory metadata, compression, names, comments, and decompressed payload.

Archive-comment edits now carry both text and the resulting encoding through the mutation, diff, inverse, retained post-copy, canonical mutation representation, and Rust/TypeScript facets. A CP437 source stays CP437 only when the replacement has an unambiguous CP437 wire representation; otherwise the primary editor promotes it to UTF-8. Intentional encoding changes remain available through the snapshot Details state and are checked by serialization validation. Rust and TypeScript tests cover CP437 `é` to an emoji-bearing UTF-8 comment, save/reopen equality, and exact undo back to the original `0x82` EOCD byte.

## Verification

- `NX_DAEMON=false bun nx test @semio-tech/stdio-wav --skip-nx-cache`: passed after the natural-audio and fidelity updates, 9 tests and 31 assertions. The final assertions cover a two-page 18,000-sample wide-frame removal at a stable logical index and exact forward/inverse state under the shared 16,384-byte payload bound.
- `bun nx test @semio-tech/stdio-wav-rs --skip-nx-cache`: reached tests after the shared native queue; 12 passed, 1 failed, and 24 were cancelled by fail-fast. The failure was a test-fixture mismatch: it paired extensible format `0xfffe` with `WavData::Pcm8`, while decode correctly represented that unknown/extensible format as raw bytes. The fixture now uses PCM format `1`; no post-fix native rerun has completed, so WAV native is not claimed green.
- `NX_DAEMON=false bun nx test @semio-tech/stdio-wav-rs --skip-nx-cache`: the post-fix focused rerun stopped after 4m29s before WAV compilation or tests because the current shared `semio-framework-os-kernel` dependency has 30 `RetainedClone` trait/close compile errors. The captured terminal result is `🗑️generated/wav-native-current-8.log`; it is not a WAV test result.
- `NX_DAEMON=false bun nx test @semio-tech/stdio-zip --skip-nx-cache`: passed, 13 tests and 102 assertions, including metadata parsing, CP437 comment promotion, and required mutation-encoding schema state. Earlier daemon-backed attempts did not reach tests because concurrent workspace writes repeatedly restarted Nx graph construction.
- `NX_DAEMON=false bun nx test @semio-tech/stdio-zip-rs --skip-nx-cache`: failed after 36m58s. Nextest ran 15 of 64 tests: 14 passed, `opc::component::tests::publication_rejects_duplicate_archive_member_names` failed during fixture construction because `snapshot_to_zip(...).unwrap()` received `Malformed("ZIP member names must be nonempty and unique")`, and fail-fast cancelled 49. The captured terminal result is `🗑️generated/zip-native-all-current-7.log`. This run compiled before the later invariant-lifetime repair and did not execute the remaining ZIP metadata laws, so no ZIP/OPC native pass is claimed.
- PNG native has not run in this slice.
