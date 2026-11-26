# Stdio Build Steward (wasm32-wasip2 stdio + stdio-office)

Status 23:57: BUILD GREEN (rebuild 231237, exit 0). Published, supported, activated; pane journey run (see Journey).

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

## Later errors (all not code, except the last)

- Warden SIGTERMs (hub-stdio, hub-stdio-component, pdf-model, ifc4x3 add2 shards): memory budgets; resolved by the warden agent's budgets plus the coordinator's cheaper compile (`-Z threads=1`, `lto="off"`).
- Two concurrent giant crates: `claude-pane-cargo.sh` builds the two components in two sequential cargo invocations (stdio first, office second), exit non-zero if either fails.
- Transient manifest edits by other agents (brep-import duplicate `step-snapshot` key, bim snapshot missing `sqlite-schema` dep): fixed by their owners during the run; no change by this steward.

## Result (rebuild 231237, 23:12 to 23:48)

- Stripped wasm: stdio 402,046,338 bytes (383 MiB), stdio-office 210,167,265 bytes (200 MiB); both under the 512 MiB cap.
- Warden peaks: semio_hub_stdio_component 9,879 MB at its earlier cancelled run (budget now allows it); this green run logged no cancellations. semio_hub_stdio_office_component final link peaks 4,096 to 4,231 MB (above the unbudgeted 4,096 MB final-link ceiling, reported not cancelled): budget it at about 4,700 MB.
- native/build deleted after publish (disk 31 GiB free).

## Journey (T/🗑️generated/standalone-journey/journey.json)

- en: FAIL, download event timeout 90 s
- de: FAIL, download event timeout 90 s
- open transitional workbook: FAIL, `file-open-failed` notice
- open strict workbook: FAIL, `file-open-failed` notice
- refuse not-a-workbook.xlsx (en, de): PASS
- refuse foreign-package.xlsx (en, de): PASS
