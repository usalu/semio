# 1112 stdio6
## Done (edited, verification pending)
- stl-inferences / obj-inferences: sqlite snapshot host: deleted `to/from_sqlite_database_receiving` + `validate_*_decoding/encoding` overrides (native decode/encode were already output-slot shaped); removed `🫴️receiving` module + dir + its cfg(test) mirrors (obj 🦀️.rs hoisted mod, stl viewer test mod) + project.json inputs.
- playbook-playbook generation root: `GenerationRootRetirement = ArcLeaseRetirement<GenerationPlayState>`; `RetireOwned` seed form (`RetirementCursor`, `retirement_seed`); `retire_cold` = `Arc::into_inner` + `GenerationPlayState::retire_decoded` (no cold effect exists in production).
- deflate-snapshot: left to stdio4 (already ported on disk).
## Debts
- obj/stl `🧪️tests/🔬️rust-package-commands/🟦️.ts` still reference deleted receiving tests; playbook generation tests (`🧪️tests/🧬️generation`) use old grant drain API.

## Verified
- W-stdio6-1 (hub wasm stl+obj inferences) rc 0; W-stdio6-2 (playbook-playbook) rc 0.
- W-stdio6-3 (-p semio-hub-playbook-component) rc 101: only red crate semio-s-artifact-stdio-semio-image-editor (3x E0201 duplicate natural_file_codec/encode_natural_file/decode_natural_file at 🦀️.rs:185; stdio4 claimed). Not mine.
STATUS: FIXED-verified W-stdio6-2

## Batch 2 (coordinator: gltf-model-snapshot, zip-inferences, pdf-mutations, pdf-mutations-2)
- W-stdio6-6 (hub wasm) rc 0: pdf-mutations, pdf-mutations-2 (removed `original_pack_value_operation_receiving!` from PdfA1Mutation/PdfX1Mutation: no OpBinary impl exists for them), zip-inferences (stdio4 had edited it).
- gltf-model-snapshot: extras receiving rewritten as `MappedReceiving<Json,GltfJsonMap>` (one funded rebuild turn via `TryFrom<Json>`, quoted at entity count; heap not receipted per node = debt); old incremental receiver + its tests deleted; `Json` made `pub` (E0446). W-stdio6-7 had only that error; W-stdio6-8 stops upstream: framework mesh-engine E0119 (MeshData double RetireOwned, cadmodel claim). gltf itself unverified.
- Rerun: `chk.sh W-stdio6-9 "🌎️hub/Cargo.toml" wasm -p semio-s-artifact-stdio-gltf-model-snapshot`
STATUS: BLOCKED gltf W-stdio6-8 stops in upstream mesh-engine E0119 (cadmodel); pdf x2 + zip verified W-stdio6-6
