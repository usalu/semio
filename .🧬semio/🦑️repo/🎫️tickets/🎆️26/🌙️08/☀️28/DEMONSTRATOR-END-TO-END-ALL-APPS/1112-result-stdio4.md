# 1112 stdio4
Scope: stdio las-snapshot, jpg-inferences, gif-inferences, bmp-snapshot, deflate-zlib; then UNCLAIMED stdio reds.
## Round 1 (EDITED, verification W-stdio4-2 wasm pending)
- bmp-snapshot: host impl on output-slot natives; native decode via `decode_sqlite_snapshot_record_receiving` (admit inside closure with control), encode admit -> NotStarted; backing `allocate` passes the loan straight into `NativeDecodeControl::new_forwarded`.
- gif-inferences: same shape; preflight/row forecast moved ahead of encode (NotStarted) and into decode closure.
- las-snapshot: pack native decode/encode on output-slot shape; host drops to/from_receiving + validate_*_decoding/encoding; removed sqlite 🫴️receiving module+dir and its test mount.
- jpg-inferences: host on output-slot shape; removed 🫴️receiving dir, host mount, crate-root hoisted test mount.
- deflate-zlib: retained-encoder close_step `RetainedWorkOutcome::<_,ValueError>` annotation.
Debt: project.json `test-original-receiving-*` scripts of jpg still name the removed receiving tests.
- W-stdio4-2 wasm rc 0: all five crates.
## Round 2 (EDITED; verification W-stdio4-5 hub stdio component pending)
- OpBinaryReceiving: `dsl::original_pack_value_operation_receiving!(X)` appended after every stdio `*Mutation` enum lacking it (~65 files).
- gif 989a snapshot, deflate-snapshot (decode_typed/encode_typed, authored validate_sqlite_snapshot_subset), csv-snapshot (TSV-shaped bind/natives, receiving removed), md-snapshot (owned_pack decode/encode), ply-inferences decode, wav edit-audio close_step (close_original_retained_effect_step), semio flow SemioFlowSnapshotDecode::close_step (effect-shaped).
- Removed: las/jpg/deflate/csv sqlite 🫴️receiving dirs and their hoisted test mounts.
Debt: project.json test-original-receiving-* scripts; ifc-4 owned-graph/owned natives (RetirementCursor shape) NOT ported; ifc-geometry prepare/mesh not ported; image-editor natural_file duplicates (stdio5).
## Verified
- W-stdio4-2 rc0: bmp-snapshot gif-inferences las-snapshot jpg-inferences deflate-zlib.
- W-stdio4-9 rc0: zip-opc svg-snapshot csv-snapshot md-snapshot ply-inferences gif-snapshot(989a) zip-inferences.
- W-stdio4-7 (semio-hub-stdio-component, before zip-opc/svg fixes): red = ifc-4-model 12, ifc-geometry 6, zip-opc 3 (fixed), gltf-model-snapshot 1 (stdio6), semio-image-editor 3 (stdio5), semio-cad-export 6 (missing step_snapshot dependency edge, Cargo).
- W-stdio4-8: only semio-framework-mesh-engine red (peer: derive RetireOwned + artifact_retire_struct! on MeshData), masks the rest.
## Open (stdio4)
- ifc-4-model: native/owned.rs + owned-graph.rs custom RetireOwned/RetirementCursor impls (retirement/close_step shapes) + snapshot host natives; ifc-geometry prepare/mesh: reserve_one_funded now effect-shaped, needs cx.with_original_effect.
STATUS: BLOCKED ifc-4-model + ifc-geometry unported; hub stdio component not yet rc 0
