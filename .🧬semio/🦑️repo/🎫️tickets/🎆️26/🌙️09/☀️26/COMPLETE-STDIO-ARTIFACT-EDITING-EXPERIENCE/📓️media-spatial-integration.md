# Media and Spatial Editor Integration

## Scope and end-user surface

This workstream covers 54 editor roots across PNG, JPG, GIF, BMP, TIFF, SVG, WAV, MP3, MP4, AVI, OBJ, STL, PLY, glTF, IFC, STEP, DWG, DXF, LAS, and Semio. Every root retains its native preview/canvas and adds the split Details window with the six retained snapshot actions. The actions publish through each artifact's typed mutation, so history, undo, redo, pack save, and reopen stay on the artifact event lane.

The ticket's validation-only `📜️script.ts verify` checks all 54 roots for the editor trait, Details window declaration and body, split layout, exact editor controller identity, honest native window capabilities, removed no-op command variants, and required SetSnapshot mutation leaves. It performs no source rewrites or migrations.

## Exact action addressing

The controller audit resolves each editor's actual `ArtifactEditor::DIALECT` constant and requires exactly two matching addresses in the root: bounded-first-step proof authority and the rendered Details action target. It found and corrected 14 bad inherited addresses:

- JPEG baseline and TIFF baseline now address their `baseline` subsets.
- GIF 89a now addresses standard `89a` instead of `87a`.
- SVG basic and tiny now address `basic` and `tiny`.
- IFC 2x3 COBie, SAV, and CV20 now address their concrete subsets.
- STEP AP214 CC1 through CC6 now address their concrete conformance classes.

All 19 Semio subset roots likewise use their concrete dialect subset. The validation passes 54/54 assigned roots.

## Honest native canvases

Generic image, media, and mesh window kits no longer advertise `set-pixel-region`, `seek-media`, or `set-vertex` where the editor had no real handler. The corresponding dead `SeekMedia` and bare `SetVertex` command variants and no-op match arms were removed. Semio mesh and brep retain the editable mesh kit because their vertex command is real and event-sourced. The native image/media/mesh canvases remain available for preview and interaction while the Details tree owns complete structural editing.

## PNG large-raster path

PNG uses a lazy Details provider. The provider reports the true pixel count and reads only requested `/pixels/{index}` values, while metadata is projected without the raster. Schema-derived creation templates, missing optional properties, enum choices, and typed collection additions delegate to the metadata provider. Lossless typed JSON is the Details source for transport-sized snapshots.

Pixel set, insert, remove, and move commands emit `PngMutation::PatchPixels`, a bounded range event carrying only the affected bytes and exact inverse bytes. The leaf has a fourteen-field authority descriptor, JSON Schema, Rust/TypeScript/GraphQL/Protobuf declarations, direct text and binary codecs, and protocol tag 18. Whole-array replacement continues to use the truthful `ReplacePixels` mutation.

Metadata edits detach the raster before generic reduction. Header, palette, transparency, gamma, chromaticities, sRGB intent, physical dimensions, timestamp, background, text chunk insert/remove/replace, and unknown chunk insert/remove changes map back to existing compact native mutations when the edit affects that domain alone. This keeps ordinary metadata history independent of raster size. A typed whole-snapshot fallback remains for cross-domain/root/source changes.

Admission encodes the exact resulting native event and accepts it only when the one-item retained store can publish it. Large whole-array/root/source replacements are therefore rejected before the job starts instead of appearing writable and blocking during publication.

The large-raster runtime test uses 2,097,152 pixels and edits a pixel at index 1,500,000. It publishes compact gamma and pixel events through the real retained ArtifactStore, verifies the untouched 2 MiB sibling payload, and exercises undo and redo for both edits. The corrected final invocation passed.

## WAV large-sample path

WAV uses a lazy Details provider over `/data/value/{index}` and projects `fmt` and ancillary chunks without expanding the sample buffer. It delegates schema-derived create, nullable-property, and enum controls to the projected provider. Source replacement uses the shared lossless typed JSON parser and is advertised only when the exact action transport admits the encoded event.

Sample set, insert, remove, and move commands emit `WavMutation::PatchData`. The event carries a bounded typed sample range, validates the current `WavData` variant and indices, and retains exact inverse samples. `fmt` and ancillary edits emit `SetFmt` and `SetOtherChunks`, so they do not retain a large data chunk.

WAV uses the same exact encoded-event admission against the one-item store ceiling. This aligns source/action visibility, retained transport, and publication capacity.

The retained-store test uses 2,097,152 raw samples, edits index 1,500,000, verifies the binary event is below the one-item store limit, cancels a staged publication without changing generation or root identity, then publishes, undoes, and redoes the edit. It also changes sample rate through `SetFmt` against the same 2 MiB data and verifies the payload survives metadata undo/redo.

## JPEG, TIFF, and MP4 compact metadata paths

JPEG document edits apply a typed path patch directly to the snapshot. JFIF header, re-encode quality, restart interval, quantization tables, Huffman tables, and whole-pixel replacement map to existing semantic mutation leaves. The baseline editor similarly maps SOF marker, sample precision, arithmetic mode, and component sampling. Both editors encode the forward event and its exact inverse before admission. The representative 2 MiB test changes re-encode quality without retaining the raster, applies the inverse, and exports a real JPEG whose bytes differ at the new quality and reopen with the same geometry.

TIFF document edits apply a typed path patch directly across primary and secondary IFD pixel buffers. Byte order, IFD insert/remove, tag replace/remove, and whole primary pixels use existing semantic leaves. The baseline editor maps compression, photometric interpretation, bits per sample, strip offsets, and tile tag insertion. Exact inverse admission prevents removing a large IFD or tag when undo could not be retained. The representative 2 MiB test changes byte order without retaining pixels, applies the inverse, and checks a real big-endian TIFF export and reopen.

MP4 edits apply a typed path patch directly through tracks and sample payloads. File type, track dimensions, track codec, sample sync, track insert/remove, and sample insert/remove use native leaves. `InsertSample` and `RemoveSample` now have exact reciprocal inverses instead of whole-snapshot inverses. Admission rejects removal of a 2 MiB sample because its required insert inverse exceeds the retained item ceiling, while a file-type edit remains compact. The focused test also applies the exact inverse and exports/reopens a native MP4 while preserving the sample payload.

The earlier detach-and-restore reducers for JPEG document/baseline, TIFF document/baseline, and MP4 were lossy for edits addressed inside a detached payload: they accepted the command but restored the old bytes. Those five reducers now prepare a native `SnapshotPatch` and apply it directly to the typed snapshot through the exact dialect/document schema validator. Existing compact semantic mutation selection remains in place. Shared neutral payload cases cover whole-array replacement plus element set, insert, remove, and move and compare the entire post-event snapshot so unrelated details must remain byte-for-byte equal. Their native verification was still compiling at the time of this report update and is not yet recorded as passing.

## Native mutation vocabularies

Typed SetSnapshot leaves, aggregate variants, schemas, codecs, declarations, diffs, inverses, and protocol records were added where the assigned roots previously had no full snapshot capability: JPG document, TIFF document, SVG base, glTF any, and Semio object, mesh, table, brep, kit, graph, text, and drawing. glTF's strict source authority includes the set-snapshot taxonomy entry.

## Verification evidence

- `NX_DAEMON=false bun nx run @semio-tech/stdio-png-rs:check -- --features component-app-assembly` passed after `PatchPixels` was added.
- `NX_DAEMON=false bun nx run @semio-tech/stdio-png-rs:test -- --features component-app-assembly large_raster_metadata_and_pixel_edits_publish_and_replay_compactly` passed after the 2 MiB fixture dimensions were corrected: one focused test run, no publication block.
- `NX_DAEMON=false bun nx run @semio-tech/stdio-wav-rs:check -- --features component-app-assembly` passed with `PatchData`.
- `NX_DAEMON=false bun nx run @semio-tech/stdio-wav-rs:test -- --features component-app-assembly large_sample_edit_publishes_cancels_undoes_redoes_and_preserves_metadata` passed: one test run, 41 filtered out. It exercised a 2 MiB payload through the retained store, cancellation, publication, undo, redo, and a compact metadata edit.
- Focused glTF and Semio feature checks passed after their mutation vocabulary and controller changes.
- The final JPEG, TIFF, MP4, and newly extended PNG/WAV native-export assertions are queued behind the repository's concurrent Nx project-graph construction. They are not recorded as passing until their new invocations finish.
- `bun <ticket>/📜️script.ts verify` passes: controllers 54, rollout 54, truthful windows 54, dead commands 54, compact media schemas 2, compact media routes 5.

### 2026-09-27 execution handoff

The MP4 focused native-export/admission test remains active under unified exec session `10416` and Nx process `45121`. Its output is `🗑️generated/mp4-large-publication-test-3.log`; at handoff the runner had emitted only periodic elapsed-time messages through 450 seconds because the shared Cargo cache was compiling under heavy concurrent repository activity. No compiler or test diagnostic had been emitted. The process was deliberately left running for the parent coordinator to observe.

No new JPEG, TIFF, WAV, or PNG test invocation was started after this handoff request. The next exact checks are, in order: JPEG `large_raster_quality_edit_uses_compact_native_event`, TIFF `large_raster_byte_order_edit_uses_compact_native_event`, WAV `large_sample_edit_publishes_cancels_undoes_redoes_and_preserves_metadata`, PNG `snapshot_detail_edit_round_trips_through_native_history_and_codecs`, and PNG `typed_snapshot_source_preserves_ancillary_and_unknown_chunk_details`.

## Remaining capacity work

The format-specific compact paths now cover PNG, WAV, JPEG, TIFF, and MP4, but their semantic leaves do not cover every nested detail. Other generic roots still serialize the full snapshot for an edit and apply the 65,536-node admission bound. MP3 and AVI frame/chunk payloads, GIF/BMP rasters, glTF buffers, and large mesh/point-cloud arrays therefore still need the shared derived path codec and native `PatchSnapshot` leaf. The 54-root/43-aggregate implementation sequence is specified in `📓️media-spatial-patch-snapshot-rollout.md`. Full typed JSON source replacement is intentionally unavailable when its exact encoded action exceeds the transport ceiling; every field must instead remain reachable through bounded path events. These are real remaining limits until that rollout and its retained publication tests pass.

## Discriminator-preserving fallback correction

The WAV large-data reducer previously detached `data`, applied a generic edit to an empty same-variant carrier, then restored the old payload. A valid `/data/kind` edit therefore reported success while retaining the old discriminator. Its fallback now prepares one typed `SnapshotPatch` against the original snapshot and applies it through `apply_snapshot_patch_for_dialect` with the WAV dialect/schema. A regression changes `Raw([1,2])` to `Pcm8([1,2])`, applies the emitted native mutation, and verifies that native WAV encode/decode reopens as `Pcm8`.

PNG used the same detach/apply/restore shape for its pixel vector. Its direct lane already covers every valid pixel set/insert/remove/move operation and a byte vector has no independent discriminator, but the fallback is now the same typed schema-validated path to remove the latent accept-with-restored-payload behavior.
