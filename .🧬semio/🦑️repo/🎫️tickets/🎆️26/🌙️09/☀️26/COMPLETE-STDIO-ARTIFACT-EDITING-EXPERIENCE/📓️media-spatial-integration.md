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

The large-raster runtime test uses 2,097,152 pixels and edits a pixel at index 1,500,000. It publishes compact gamma and pixel events through the real retained ArtifactStore, verifies the untouched 2 MiB sibling payload, and exercises undo and redo for both edits.

## WAV large-sample path

WAV uses a lazy Details provider over `/data/value/{index}` and projects `fmt` and ancillary chunks without expanding the sample buffer. It delegates schema-derived create, nullable-property, and enum controls to the projected provider. Source replacement uses the shared lossless typed JSON parser and is advertised only when the exact action transport admits the encoded event.

Sample set, insert, remove, and move commands emit `WavMutation::PatchData`. The event carries a bounded typed sample range, validates the current `WavData` variant and indices, and retains exact inverse samples. `fmt` and ancillary edits emit `SetFmt` and `SetOtherChunks`, so they do not retain a large data chunk.

WAV uses the same exact encoded-event admission against the one-item store ceiling. This aligns source/action visibility, retained transport, and publication capacity.

The retained-store test uses 2,097,152 raw samples, edits index 1,500,000, verifies the binary event is below the one-item store limit, cancels a staged publication without changing generation or root identity, then publishes, undoes, and redoes the edit. It also changes sample rate through `SetFmt` against the same 2 MiB data and verifies the payload survives metadata undo/redo.

## Native mutation vocabularies

Typed SetSnapshot leaves, aggregate variants, schemas, codecs, declarations, diffs, inverses, and protocol records were added where the assigned roots previously had no full snapshot capability: JPG document, TIFF document, SVG base, glTF any, and Semio object, mesh, table, brep, kit, graph, text, and drawing. glTF's strict source authority includes the set-snapshot taxonomy entry.

## Verification evidence

- `NX_DAEMON=false bun nx run @semio-tech/stdio-png-rs:check -- --features component-app-assembly` passed after `PatchPixels` was added. A later focused test build is still running against the final schema-control delegation and compact metadata routing.
- `NX_DAEMON=false bun nx run @semio-tech/stdio-wav-rs:check -- --features component-app-assembly` passed with `PatchData`.
- `NX_DAEMON=false bun nx run @semio-tech/stdio-wav-rs:test -- --features component-app-assembly large_sample_edit_publishes_cancels_undoes_redoes_and_preserves_metadata` passed: one test run, 41 filtered out. It exercised a 2 MiB payload through the retained store, cancellation, publication, undo, redo, and a compact metadata edit.
- Focused glTF and Semio feature checks passed after their mutation vocabulary and controller changes.
- `bun <ticket>/📜️script.ts verify` passes: controllers 54, rollout 54, truthful windows 54, dead commands 54, compact media schemas 2.

## Remaining capacity work

Generic roots still serialize the full snapshot for an edit and apply the 65,536-node admission bound. Large JPEG coefficient/pixel payloads, MP3 frame payloads, MP4/AVI box or chunk payloads, and large mesh/point-cloud arrays therefore still need the same lazy provider plus compact range-event design used by PNG and WAV. Full typed JSON source replacement is also intentionally unavailable when its exact encoded action exceeds the 16 MiB transport ceiling. These are real remaining limits; Details reach for ordinary files is complete, but arbitrary-size editing for every assigned binary family is not yet proven.
