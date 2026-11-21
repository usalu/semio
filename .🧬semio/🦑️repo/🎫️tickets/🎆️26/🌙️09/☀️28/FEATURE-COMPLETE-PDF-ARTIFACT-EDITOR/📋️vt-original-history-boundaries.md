# VT Viewer Original History Boundaries

## Archive operation wire

The checksum failure comes from an archive produced during the failing acceptance test. The fixture calls the live `PluginApp::document_archive` method, which generation-fences the document and invokes `print_document_pack`. That emits the parent Pack and its SPR. `print_document_spr` calls `history_edit_from_edit` and `history_op_payloads` for forwards and inverses. The latter calls Base PDF 1.7 `PdfMutation::encode_op`. It writes format byte 1, a one-byte leaf tag, a varint JSON length and canonical JSON.

The Original hydration path reaches `step_native_mutation` in DecodeForward and DecodeInverse. It borrows that exact binary span and calls `PdfMutation::admit_receiving`. The PDF schema uses `original_pack_value_operation_receiving!(PdfMutation)`, so the retained receiver opens a complete Semio Pack instead. Its retained anchor computes CRC32C over bytes 0–19 and compares bytes 20–23 as a little-endian checksum before checking the Pack magic. A schema-valid neutral insert-object specimen produced 141 bytes with prefix `010689017b226d75`. The stored header word was `4f747265`, while Google CRC32C computed `94c0df0d`. This independently reproduces the offset-20 failure condition observed in the native test. The archive did not come from a stale checked-in byte fixture.

## Scope of the alternate writer

The candidate in `📜️vt-archive-wire-stage.py` matches the existing Pack receiver, including its whole-number float normalization. It is a wire experiment and stays staged. The aggregate binary trait is shared by these producers and consumers.

| Path | Effect of changing the aggregate OpBinary |
| --- | --- |
| Archive | Both forward and inverse SPR payload bodies differ. Reference parsing calls `Mutation::decode_op`; retained parsing calls `OpBinaryReceiving`. A full archive changes at the raw frame and commit-chain level. |
| Commands | `write_command_ops` length-prefixes the aggregate `encode_op` output. `read_command_op` uses `decode_op`; `RetainedArtifactCommandReceiving` admits the operation receiving cursor. Supersede uses the same aggregate bytes. |
| Reference causal sync | `mutation_envelope_from_edit` creates per-operation payloads through `OpBinary`. `edit_from_operation_envelope_cold` calls `decode_op` on forward and inverse bytes. `StoreLog::same_operation_identity_and_payload` compares schema and exact forward payload bytes for an established id. Two binaries using different aggregate formats could reject the same typed operation under the same entity/subtype schema. |
| History supersession | `store_read_model` serializes replacements via `encode_op`. `admit_replacement` checks matching schema, decodes and compares the re-encoded bytes with the original payload. This equality and stored replacement bytes are wire-sensitive. |
| Raw hashes | SPR commits and frame materialization hash raw data. The foreign-step cycle detector hashes `step.payload` with BLAKE3. These can change as a result of physical wire bytes. |
| Original authored identity and reconstructed replay | The installed canonical operation wire quotes `b"op"` plus the typed JSON tree through `resolve_operation_wire` and `OperationIdentityCursor`. The Original replay digest uses `NativeEditDigest::advance_with_inverse` on typed edit records and metadata. These mechanisms do not call the aggregate `encode_op` themselves. Equal reconstructed typed edits and metadata can therefore preserve their native digest. This gives no invariant for old raw archives, causal bytes, or canonical replacement re-encoding. |

The narrow format-preserving repair is a single bounded `PdfMutation` receiving cursor for format 1. It can reuse the existing native funding model and `JsonGrammarCursor<DslValue>` with duplicate member rejection. It needs to bind the original source range, parse the fixed header and canonical varint incrementally, hold the unmodified byte span, enforce the existing maximum payload length and exact consumption, and compare the selected registry leaf with the JSON mutation discriminator before binding the typed value. It must retire parser, partial value, and typed output on cancellation or refusal under the caller's existing grant. This leaves aggregate and direct-leaf byte identity, replay metadata, subtype schema, and raw hashes unchanged. The existing `OriginalJsonReceiving` consumes the different `b"op"` owned-projection lane, so it cannot be installed directly on length-framed aggregate bytes. The existing IFC original command cursor exercises the borrowed JSON grammar mechanism. The new receiver should test exact current bytes, noncanonical length, tag mismatch, duplicate members, cancellation during framing/JSON/typed conversion, and replay retention of known raw and typed hashes.

## Semantic and retirement issuers

`Pdf17VtEditor` presently delegates to the default bounded store catalog. That catalog constructs snapshot and mutation retirement factories and batch digest authority. It has no replay preparation issuer. The editor's `snapshot_details_editor_support!` separately registers a routed one-item publication factory. `step_derived_report_replay_in_order` requires the installed replay, initial snapshot and mutation retirement issuers and fails closed when one is absent. A mounted acceptance test exercises the same owner path. Retiring an already-refused frontier also reported an undeclared preview or authoring owner.

`📜️vt-replay-owner-stage.py` pairs `document_store_owners_source_demands` and the macro's catalog hook with the existing `cold_replay_document_store_owners` helpers. Its concrete ColdMutationApplyFactory is a single Arc. The shared semantic constructor checks item, copy, capacity and depth before the allocation, stores both aliases, and reports the quoted copy and retained capacity. The publisher selected independently by the bounded-native editor macro still routes its original tool mutations. The catalog's subsequent ordered ticket preparation and refusal/cancel cleanup run through the same Original staging effect and the constructor's `close_uninstalled_owners_step`. Only a constructor-complete owner catalog may install.

The generic native constructor law exercises zero items, one-byte-short copy and capacity, shallow depth, zero-release birth, and cancellation at each of its nine frontier cuts, matching receipts to observed allocations. The corresponding existing independent Ajv/SQLite alias law passed in this turn. The new VT hook delegates to those exact implementations. The staged VT source parsed with pinned rustc, and its expected digest matches its staged receipt. Concrete VT caller grant/cancel execution remains unverified. The isolated metadata probe was blocked by the changed WASI unit epochs. The old plugin/viewer require earlier `semio_os_store_text_io` Rust metadata that could not be recovered from the local receipts. The live viewer/native gates can test the concrete owner after coordination.

## Source envelopes

The repository's closure reader measured the replay-owner and archive-encoder candidates together on macOS, Linux, Windows and WASI. Each target returned the same counts. Counts are files, active lines and source bytes.

| Owner | Library | Test |
| --- | --- | --- |
| VT editor | 10 / 1,357 / 76,893 | 31 / 3,287 / 200,159 |
| VT viewer | 2 / 176 / 8,199 | 5 / 296 / 14,730 |
| Aggregate binary | 27 / 1,761 / 103,829 | 32 / 2,106 / 123,827 |
| Binary testkit | 20 / 1,901 / 116,583 | 32 / 3,778 / 232,136 |

All candidate Rust source files passed syntax parsing. The two digest-guarded stages and source cap probe are ticket-local. The raw checksum witness is `🗑️generated/vt-archive/old-frame.json`, produced by `📜️vt-archive-wire-oracle.py`. No archive or replay Rust candidate was applied.
