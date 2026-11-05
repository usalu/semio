# Registered Example Hydration Trace — 2026-11-05

Read-only review; no duplicate native build or production edits. One small registered Node direct production TypeScript parser probe was executed as noted below.

## Synthetic Orbit Exact Route

1. SetActiveExample handler resolves registered example_text, parses it with the native RemodelingSnapshot DSL parser, then calls example_media_content before comparison/publication.
2. For synthetic-orbit only, example_media_content iterates synthetic_orbit::frames(). That iterator flattens the four compiled quarter arrays, each containing nine include_bytes references to the actual committed PNGs. All 36 ordered asset ids match the declared DSL/ground-truth frame ids as the earlier independent asset check verified.
3. Each PNG is decoded for dimensions and placed in ImageAsset with MIME image/png and base64 of the exact compiled bytes. durable_remodeling_asset decodes bytes, enforces its whole-content envelope and creates bounded durable chunks. store_remodeling_asset mints the deterministic composed child handle. The handler inserts artifact under handle.child_id in durable_artifacts and inserts the same handle under original frame asset_id in assets.
4. reset_document_effect encodes the fully hydrated snapshot with ArtifactPack::encode_pack and emits LoadDocument {pack,spr}, with fresh edit-free SPR. The 4382-byte DSL need not embed compressed pixels: the Rust example loader joins those compiled assets before encoding the load pack.
5. External test context dispatch actually handles LoadDocument through the public loader. The picker law selecting_another_example_loads_it_without_an_edit asserts no edit, asset count equals frame count and nonempty streams. This is real registered-action coverage when executed.
6. Cold reconstruction frame_ingestion calls remodeling_asset_chunk_source(scene, frame.asset_id), which looks up the loaded child handle in snapshot-owned durable_artifacts and creates CompressedChunkRope/BoundedStillDecoder. UI image rendering follows the same durable lookup via the facade.

No concrete missing hydration path was found for valid committed Synthetic Orbit. Its loader falls back to dimensions (0,0) on decode failure and skips over-envelope durable asset failures; the current independent PNG check confirms all 36 committed files are valid 320×240 and within the existing finite asset sizes. Stronger native picker acceptance would load through SetActiveExample, pack/cold-reopen and decode all 36 images, rather than only checking asset count. Current numerical reconstruction tests build via imported_app's actual frame/calibration/parameter commands, so they do not by themselves establish picker hydration or cold reopen.

## Concrete Adjacent Demo Session Picker Gap

The same visible registry includes id demo-session with PRIMARY_TEXT pointing to a command-session asset beginning `semio remodeling.remodeling.cmd v1` and `action=demo`. SetActiveExample has no format/replay branch: every registry entry goes directly into snapshot parse_dsl, and any parser failure is returned as successful empty emission. Native snapshot parser is the DSL document surface, not the command decoder.

A bounded direct production TypeScript DSL parser probe against that actual registered asset rejects it with: expected preamble semio remodeling.remodeling.dsl v1, got semio remodeling.remodeling.cmd v1. This corroborates the source-level carrier mismatch; it is not a native execution result. Existing every_registered_example_id_resolves_to_committed_text asserts only text presence, and the one example-selection law exercises Synthetic Orbit, so neither detects the visible Demo Session choice silently doing nothing.

Root was notified promptly. Resolve the registry/selection carrier contract under its actual owner, rather than treating this registered command asset as a parsed document. This lane made no source changes. Browser checklist should use demo for the alternate document switch until that option is repaired or deliberately omitted from the document picker.

## Demo Session Repair Independently Verified

Read surface owner's new shared picker fixture, native dispatch law and real Demo Session DSL binding. PRIMARY_TEXT now points to the snapshot DSL; canonical document id remains remodeling, with two distinct calibrated cameras and two empty capture streams. Dummy command asset is removed. The actual TypeScript parser/JSON codec/family conversion independently accepts all three registered fixture assets, exact projections match all rows, every stream camera resolves, and all three switch pairs describe distinct states. Updated retained example-schema helper and reran exit 0; existing 36-image/cold 170-chunk surrogate still passes. This resolves the documented command/document carrier mismatch in current source.

Native switch source now dispatches every shared pair, checks changed selected snapshot, expected stream ids/camera counts, no Artifact publication lane and clearing old assets/durable content for non-orbit targets. Its historyEdits check currently asserts the fixture value equals zero, rather than comparing actual history snapshots; no-Artifact lane remains the actual no-edit witness. Native dispatch execution and fresh descriptor publication remain separate pending gates. No checked source was edited by this review.

## Latest Native Hydration/History Signature Review

Read the stronger surface-owned picker law after repair. Actual history_snapshot returns HistoryPatch; its edit_count is u32, so u64::from matches the shared JSON expected count, and can_undo/can_redo are booleans. The law now measures real source/selected history, eliminating the earlier fixture-only count limitation.

Loaded durable source exposes MIME/width/height and Vec<Arc<Vec<u8>>> leaves; the flat-map byte collection and exact committed slice comparison are valid against that owner signature. Independent png 0.17 Reader/output_buffer_size/next_frame and RGBA8 expectations agree with the current committed assets already independently decoded in this lane. Owned decode_still_image yields ImageRgba8 with byte data and dimensions for the final comparison. No concrete current API/signature issue was found. This is source review; actual native runtime remains pending root's gate.
