# acceptance lane (11/12)
## State: semio-plugin-testing-acceptance green
Edits (under 🔌️plugin/🧪️tests/):
- 🧪️history-edit-acceptance/🎛️acceptance-policy-pump-and-drive:37 input.step -> close_original_effect_step(.., |effect| input.step(body, effect))
- 🧪️history-edit-acceptance/📚️acceptance-fixtures-cases-and-change-buckets:14 same port for owner.step
- 🧪️history-edit-acceptance/💾️reload-and-child-history-scenarios:22 begin_document_archive_load(&mut archive, &mut OriginalDocumentArchiveAdmission::empty(), cx) in an admission loop (yield -> error)
- 🧪️node-drag-history: Mu gains protocol::OpBinaryReceiving (StoreMutationFor); the 3 E0308 were cascades
- 🧪️history-edit-acceptance Cargo.toml: dep semio-plugin-document-media (LOCK NEEDED, hub check passed anyway)
## Verified
- W-acceptance-3 wasm hub -p semio-plugin-testing-acceptance rc0
- W-acceptance-6 native Cargo.toml --tests --features artifact-app-testing rc0 (native without the feature is red by design: lib is cfg-gated on artifact-app-testing; W-acceptance-4)
## Full hub map W-acceptance-hub-1: rc101, 11 crates fail (none acceptance)
semio-os-flow-extensions (flowext), semio-s-artifact-stdio-contract-editing-patch (borrowed codec), semio-s-artifact-fem-2d-editor-transient (121; BTreeMap<String,FemGumballGesture>: RetainedClone / RetainedPatchEmpty derive), process3d-presence (4), puzzle-3d precompute-geometry (2), mutations-pose/catalog/volume/reference/vortex (NativeReceive/DslOwnedReceivingField derive; factorysweep), hub-inference-binding (3). Not edited by acceptance.
Rerun: "$C/1105b-tools/hub.sh" W-acceptance-hub-2
STATUS: FIXED-verified W-acceptance-3

## Round 2: fem 2d
- Root cause editor-transient: BTreeMap is no longer RetainedClone (design: native sorted maps instead). 2d transient `gestures` is now `Vec<FemGumballWindowGesture{window,gesture}>` sorted by window (+ `gesture(window)`, `windows()`); schema json, fixture, 2d/3d gumball tests ported. 3d transient dir is empty (2d holds both), so no sibling to mirror.
- Hand `SnapshotProjection` impl replaced by `#[derive(SnapshotProjection)]`.
- Framework macro bug `retained_root_record!` (replication retained set-field): struct literal via path metavar (now via type alias) and missing `retained_empty_option_patch!` for the diff (RetainedPatchEmpty). fem 2d was its only user.
- model-window: `original_ordinal_operation_receiving!(Fem2dModelWindowConfigMutation)` + `DslReceiving` derives on Mutation/Update/Patch.
- Verified: W-acceptance-8 (transient wasm) rc0, W-acceptance-15 (model-window wasm) rc0. Fem hub component W-acceptance-16 rc101 but only stdio-* crates red (mesh-io, deflate-snapshot, drawing-editor, las-mutations, txt-tests): none fem.
- Not verified: native fem2d --tests (W-acceptance-11 blocked by stdio-binary-mutations / stdio-semio-base upstream).
STATUS: FIXED-verified W-acceptance-15
