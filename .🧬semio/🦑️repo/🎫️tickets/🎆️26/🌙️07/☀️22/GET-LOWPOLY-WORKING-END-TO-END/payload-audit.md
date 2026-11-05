# Lowpoly Payload Audit

Read-only audit, 2026-11-05. No native/server run or limits changed.

## Picker Lane

Lowpoly editor /🎮️commands/📤️media/🦀️.rs:97 emits RequestFileOpen accepting .obj,.ply,.stl, read_as dataUrl, import_action importMeshFile. React ShellHost /⚡️host-effects/📥️document-load-effects/🟦️.ts:59 forwards picked files through importOpenedFilesV1. ShellHelpers /🎞️media-import/🟦️.ts:40 dispatches inline importChunkArguments per importPayloadChunks chunk. Native WGPU shell /📂️doc-open/🎨️theme-file-import-and-accessibility/🦀️.rs:17-24 builds the same inline payload/name/chunk/chunkCount action envelopes.

Kernel /🎞️media-jobs/🦀️.rs:577 chunk bytes are half guest contiguous ceiling; :613 chunks UTF-8 extents; :647+ staging maximum128chunks,4MiB. These are transport staging counters, not a separate file-reference payload lane.

Plugin /🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs:205-208 admits staged chunks and moves Whole into source.assembled. :126-131 selects that assembled argument for typed production. Lowpoly args bridge :164 maps filename/contents aliases to name/payload. ImportMeshFile is two owned String fields, no blob/reference handle. Media handler decodes data URL and imports by extension, producing LoadDocument outside mutation history.

## Limit Contradiction

Lowpoly media :25 states 2MiB largest admitted payload string. Retained command :133 checks that same string length, including data-url overhead; decoded raw bytes are a different count. But retained factory :9/:104/:459 and play-app :197 declare max_raw_wire_bytes16,384 for importMeshFile too.

Framework OriginalCommandAdmission in plugin /🎮️command/🎟️typed/📥️admission/🦀️.rs rejects input.declared_bytes above row.contract.max_raw_wire_bytes. Original wire writer /🎮️command/🎟️typed/📨️wire/🦀️.rs:33-35 measures full encoded command and rejects measured bytes above maximum. Hence chunk staging alone does not bypass final whole typed-command wire admission. No separate out-of-wire file lane was found in this route. Files whose encoded typed command exceeds16KiB cannot be justified by the media2MiB predicate; encoded field/framing overhead makes the practical payload ceiling smaller. This is a source contradiction requiring a public-dispatch boundary regression and deliberate payload design, not an automatic limit increase.

## Test Coverage

Media unit import_mesh_file_emits_a_load_document_effect invokes media::import_mesh_file::handle directly on a tiny quad. opened_file_bytes test directly checks raw/dataUrl parsing. Export roundtrip helper tests deserialize formats directly; only export command uses context::dispatch. Mounted root has imported import_mesh_file symbol but no importMeshFile journey invocation in the inspected mounted tree. Thus those tests do not establish picker chunks -> staging -> public typed admission at the advertised2MiB cap. UI picker and media helper both claim OBJ/PLY/STL; no extra format promise is needed to expose this byte-limit mismatch.

## Separate Focused Test Outcome

At root-authorized scope update, inspected runtime's already-applied admission Python registry/import edit and ran exact public repo tools exec --light bun -- test admission/🟦️.ts. Exit0,4pass,0fail,30expect calls, including actual registered Python oracle and neutral child projection. No duplicate source edits, environment/PATH changes or setup workaround.
