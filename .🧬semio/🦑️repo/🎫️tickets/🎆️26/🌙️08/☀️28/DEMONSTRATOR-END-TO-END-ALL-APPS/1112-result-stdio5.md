# 1112 stdio5
## Step 1 (edited, verifying W-stdio5-1)
- json-schema: removed hand-written 🫴️receiving module+tests (and geojson mirror test mod), dropped *_receiving/validate_*_decoding/encoding overrides; owned_pack decode/encode on out-slot via store::decode/encode_sqlite_snapshot_record_native.
- xml-snapshot: native decode/encode via settlement::decode_typed/encode_typed; retained PagedList reserve_one -> reserve_one_cold.
- dxf header: decode via decode_sqlite_snapshot_record_native out-slot; post-decode forecast of rows dropped (limits are enforced by the authored backing validation) - debt.
- terminal_frame_bound! removed in stdio editors (wav edit-audio, bmp/jpg x2/tiff/gif x2 paint-region, png x2, docx set-page, semio mesh/brep) with explicit terminal_frame_release_bytes -> Some(size_of::<Self>()).
Rerun: chk.sh W-stdio5-2 "🌎️hub/Cargo.toml" wasm -p semio-s-artifact-stdio-json-schema -p semio-s-artifact-stdio-xml-snapshot -p semio-s-artifact-stdio-dxf-snapshot
## Verified
- W-stdio5-1 (wasm hub): stdio-json-schema, xml-snapshot, dxf-snapshot rc 0.
## Not verified
- W-stdio5-2 (mesh-editor, brep-io, flow-snapshot) started, stuck on "[capacity] waiting for cargo" at hand-off; rerun: chk.sh W-stdio5-3 "🌎️hub/Cargo.toml" wasm -p semio-s-artifact-stdio-semio-mesh-editor -p semio-s-artifact-stdio-semio-brep-io -p semio-s-artifact-stdio-semio-flow-snapshot
- flow-snapshot binary snapshot file already shows effect-shaped close_step (likely fixed by a peer since W-main-187); unconfirmed.
- terminal_frame_bound! removed in wav/bmp/jpg x2/tiff/png x2/gif x2/docx editors (explicit terminal_frame_release_bytes); those crates not compiled by me.
- image-editor natural_file dup left to stdio4; brep-viewer to spatial.
- Debt: dxf decode no longer forecasts rows post-decode; json/receiving cfg(test) tests removed.

## Update
- W-stdio5-2 rc 0 (mesh-editor, brep-io, flow-snapshot). Other editors (wav/bmp/jpg/tiff/png/gif/docx) with removed terminal_frame_bound! not compiled by me.
STATUS: FIXED-verified W-stdio5-2
