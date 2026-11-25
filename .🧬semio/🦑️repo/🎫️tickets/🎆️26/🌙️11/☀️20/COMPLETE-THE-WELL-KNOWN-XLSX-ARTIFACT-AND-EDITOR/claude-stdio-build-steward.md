# Stdio Build Steward (wasm32-wasip2 stdio + stdio-office)

Status 19:55: build NOT green yet; paused at the coordinator's request (host warden cancels units over memory ceilings).

## Errors and root fixes

| # | Error | Root fix | Files |
|---|-------|----------|-------|
| 1 | `BinaryMutation: ArtifactPersistedOperation` not satisfied (editor bound `ArtifactEditor::Mutation`) | Each stdio mutation vocabulary declares `artifact_persisted_binary_operation!(T)` next to its `original_pack_value_operation_receiving!(T)`, as sibling artifacts (semio graph, space, workflow) do. A first attempt added a second `OpBinaryReceiving` (tagged) and conflicted E0119; reverted, the schema already had the pack-value one. | `💾️binary/.../🧬️schema/🧬️mutations/🦀️.rs` |
| 2 | TxtMutation, DeflateMutation, Idml... same bound | One sweep over all stdio artifacts: 77 mutation vocabularies with `dsl::`-prefixed receiving plus Idml (`protocol::` prefix) now declare the persisted operation. A scan of every stdio `type Mutation = X` finds none missing (only `NoPresenceMutation` and `ZipIso21320Mutation`, an ArtifactBuilder, which do not need it). | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/*/**/🧬️schema/🧬️mutations/🦀️.rs` (bcf, mp4, stl, avi, dxf, ifc, epw, wav, bmp, mp3, svg, jpg, semio-v1 subsets, html, deflate, zip, step, pptx, xml, md, tiff, txt, binary, png, dwg, json, xlsx, ply, docx, pdf, las, gif, gltf, csv, tsv, obj, idml) |
| 3 | DWG `unresolved import semio_s_artifact_stdio_dwg_snapshot_objects_constraints` in dynamic-blocks | Added missing dependency. A `#[path]`-following scan of every dwg crate against its Cargo.toml finds no other gaps. | `🖊️dwg/.../📦️objects/🎛️dynamic-blocks/📦️packages/🦀️rust/Cargo.toml` |
| 4 | `OriginalTurnFailure::plain_refusal` missing | Added by another agent at 19:43 in `🧵️job/⚠️failure/🦀️.rs`; no change by me. | none |
| 5 | SIGTERM of rustc (hub-stdio, hub-stdio-office, pdf-model) | Not code: host warden memory-ceiling cancellations. Not caused by this steward. A separate agent fixes the budgets. | none |

A broader first sweep also touched non-stdio files (plugins and framework tests); those 40 insertions were reverted immediately, out of closure scope.

## Next
Rebuild once the coordinator says the warden budgets are fixed, then bun publish/support/activate/journey steps, then delete native build intermediates first (disk 24 GiB).
