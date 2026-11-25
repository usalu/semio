# Operation wire receiving and conflict path at the instance ingress

## Root cause
The instance ingress (`plugin/instance/staging/ingress`) installed only `ArtifactOperationWire::semantic::<Snapshot>().with_batch_digest()` on the document store. Persisted operations (history reload, folder reload, replay) need two more capabilities on that wire:
- `original_receiving` (`with_original_*_receiving`), which wraps the vocabulary's own `OpBinaryReceiving` cursor.
- `original_target` (`with_original_target`), the authored conflict path issuer.
Without them the replay stopped with "original operation has no authored conflict path issuer" and then "installed operation wire has no original receiving capability". The generic app mutation was not bounded by any trait that says how its persisted form is received, so the ingress could not pick the right receiving. The `NativeReceive<DslValue>` route (`with_original_receiving`) does not apply to binary vocabularies. The vocabulary-owned `ArtifactPersistedOperation` trait (already present in the store) is the right owner of that choice.

## Fix
- `ArtifactApp::Mutation`, `ArtifactEditor::Mutation` and `ArtifactViewer::Mutation` now require `store::ArtifactPersistedOperation` (artifact-app-trait, artifact-editor-trait, artifact-viewer-trait).
- The ingress installs `<A::Mutation as ArtifactPersistedOperation>::install_persisted_receiving(wire.with_batch_digest()).with_original_target(whole_document_target::<A::Mutation>)`.
- `store` re-exports `ArtifactPersistedOperation` and the new macro `artifact_persisted_encoded_binary_operation!`: persisted bytes are exactly `OpBinary::encode_op`, and those bytes are also the original encoding source (`with_original_encoded_binary_receiving`). It is the right declaration for vocabularies whose only wire decoder is `OpBinary` (paired with `original_decoded_operation_receiving!`).
- Vocabularies declared (each next to its `OpBinary` impl): `TestMutation` (test fixture), `SequenceMutation`, `WriterMutation`, `VcsDemoMutation`, `ProcedureMutation`, `ModelMutation`, `ProgramMutation` with encoded declaration; `Block2d/3d/5dMutation`, `FormMutation`, `EquationMutation`, `LowpolyMutation` were declared concurrently by another agent with the plain binary macro. `Process3dMutation` was declared by the coordinator. All other editor/app/viewer vocabularies already had the declaration.

## Results
See the bottom of this file.

## Results
- `cargo check --tests` of `semio-plugin-tests-07-3`: green (after declaring `TestMutation`).
- `cargo test -j 2 folder_reload_route` in `semio-plugin-tests-07-3`: 6 of 8 pass. The two target laws (`a_folder_bound_document_reloads_...`, `two_programs_on_one_folder_converge_...`) no longer stop at "no authored conflict path issuer" or "no original receiving capability". They now fail one step later, in `time-travel` `replay completes` (tests/time-travel line 296), with `operation output payload extent exhausted` (`ownershipLimit`) raised by `ArtifactPreparedOperationOutput::append` (`store/operation-wire/output`). The payload limit comes from a history batch-message owner (`replay/publication`: `DOCUMENT_BACKBONE_BATCH_MAXIMUM_PAYLOAD_BYTES`). Switching `TestMutation` from the plain binary to the encoded declaration did not change it, so the cause is the payload quote for the replayed batch, not the receiving capability. Not diagnosed further.
- Not compiled: the plugin crates under `✏️s` whose declarations were added (Sequence, Writer, VcsDemo, Procedure, Model, Program) and the three trait bounds against every app crate. Only the tests-07-3 dependency chain was compiled. The hub wasm build is the check for those.

## Changed files
- `framework/products/os/modules/plugin/declaration/app/artifact-app-trait`, `declaration/surface/artifact-editor-trait`, `declaration/surface/artifact-viewer-trait`: `Mutation: store::ArtifactPersistedOperation`.
- `framework/products/os/modules/plugin/instance/staging/ingress`: installs persisted receiving and whole-document target.
- `framework/products/os/modules/store/🦀️.rs`, `store/operation-wire/🦀️.rs`, `store/one-item/publication/lifted/operation-wire/receiving/🦀️.rs`: export `ArtifactPersistedOperation`, new `artifact_persisted_encoded_binary_operation!`.
- `framework/products/os/modules/plugin/testing/test-app-mutations/document/mutations/🦀️.rs`: `TestMutation` declaration.
- `✏️s/plugins/{sequence,writer,vcs,procedure,bim? model,architect program}/.../io/binary/mutations/🦀️.rs`: `original_decoded_operation_receiving!` plus the encoded declaration (Sequence, Writer, VcsDemo, Procedure, Model, Program).
