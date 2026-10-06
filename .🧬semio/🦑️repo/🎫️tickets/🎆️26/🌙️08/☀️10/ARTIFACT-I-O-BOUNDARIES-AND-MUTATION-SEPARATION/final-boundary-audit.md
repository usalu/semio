# Final Boundary Audit

Read-only source audit on 2026-10-06. Concurrent implementation work was active throughout; findings describe the observed files and require rechecking before closure. No tests or native compilation were run by this audit. Only this report was written.

## Confirmed Remaining Violations

All paths below are repository-relative.

1. **Dangling schema SQLite mounts.** `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:32–33` mounts `🪶️sqlite/🦀️.rs`; `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:31–32` does likewise. Both relative targets are absent and both corresponding `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` targets exist. Remove obsolete semantic mounts, retaining canonical I/O assembly. This is a source-level missing-module finding, not a compiler result.
2. **Semantic executor mounts I/O.** `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🦀️.rs:68–69` mounts `../../🚪️io/🦀️.rs` as `pub mod io`. This introduces a schema-to-physical edge regardless of the fact that the target is canonical. Mount it from the artifact assembly rather than the semantic executor.
3. **CAD schema owns SQL relational helpers.** `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📎️references/🚦️frontiers/🦀️.rs:4` imports `store::sqlite_snapshot::SqliteRow`. `keyed` at line 44 validates row IDs, SQL columns and owning parents; `by_identity` at line 62 and `dense` at line 66 also accept physical rows. Move physical row frontier code to SQLite snapshot/reference I/O; neutral literal comparison and generic ordering can remain semantic. Alias verified: CAD artifact root `🦀️.rs:8` declares `extern crate semio_framework_os_kernel as store`; OS package root `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs:236` reexports `semio_framework_io_sqlite_snapshot` as `sqlite_snapshot`. Physical `SqliteRow` is defined in `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🦀️.rs:32`. This is not a replication alias false positive.
4. **Writer schema produces JSON strings.** `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:240,244,251,257` still calls `serde_json::to_string` on completion items. Return semantic completion values from schema and serialize at inference/text I/O. Earlier report inference `to_string` lines no longer reproduce; that file now imports the JSON value macro, which alone is not proof of wire output.
5. **VDI physical native output remains in schema.** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:319` defines `serialize_native_text`; line 515 invokes it. Move native textual serialization and consumers to the proper text snapshot facet.
6. **GIS format-specific JSON output remains semantic.** `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗺️imported-map/🦀️.rs:25` defines `to_json`, rejects JSON-unrepresentable semantic values, and invokes `semio_framework_pack_json::to_string`. Format admissibility and output belong to text snapshot/imported-map I/O.

## Scope And Positive Checks

Targeted production Rust searches across framework product artifact schema and Print schema found no `serde_json::to_/from_`, native serializer, `ArtifactSqlite`, physical `SqliteRow`, or direct I/O mounts in the searched source facets. The seven framework artifact families (Workflow, Run, Space, Collection, Flow, DAG, Playbook) and Print were included through the framework products tree. This is targeted source coverage, not a complete resolved-module proof.

Targeted plugin I/O searches found no `impl ... MutationDiff`, `impl ... DiffAlgebra`, `impl ... Mutation<...>` or `fn apply_to_artifact`. The earlier report’s widespread text-diff semantic application finding therefore no longer reproduces under those patterns. I/O-local SQL row readers and numeric formatter `new` methods are codec machinery, not domain constructors, and were excluded from findings.

Program configuration now contains only one `pub mod io` at line 128; its earlier duplicate-mount finding no longer reproduces. Neutral value conversions and test-only SQLite mounts were excluded. DslDiff metadata and kernel procedural macro façades were not classified as physical codecs.

Layout schema component retirement still referenced `store::ArtifactSqliteSnapshot` in the initial broad schema scan; the runtime agent owns Layout/Drawing extraction, so recheck its final result rather than duplicate that work. WFC/Energy/Drawing/Layout runtime internals were not deeply audited here. All six confirmed findings above are outside that dedicated deep extraction scope.

## Independent TypeScript Follow-Up

7. **ZIP semantic conformance exposes SQLite control.** `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/🟦️.ts:11` imports `artifactSqliteCheckpoint` and `ArtifactSqliteOptions` from framework physical SQLite I/O. The semantic `checkZipIso21320Conformance` API at line 14 accepts that type and checkpoints `projectSnapshot` at lines 15,27,29. Replace this semantic control edge with neutral owned value/decode progress and cancellation.
8. **JSON numeric semantics exposes SQLite phases.** `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔢️number/🟦️.ts:2` imports the same physical controls. `jsonNumberMeaning` at line 6 exposes `ArtifactSqliteOptions` and SQL `projectSnapshot`/`reconstructSnapshot` phases; lines 8,10 checkpoint through SQLite. Its RFC numeric classification is semantic and should retain its location with neutral controls. Both findings were independently resolved against `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts:10`, where `ArtifactSqliteOptions` extends physical `SqliteDatabaseOptions`, rather than a neutral reexport.
9. **Print inference schema performs wire normalization.** `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/✅️validation/🟦️.ts:13` stringify/parses a semantic result before validation, explicitly documenting canonical wire output. `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🧵️worker/🟦️.ts:34` repeats JSON lowering before posting the result. Put JSON normalization in text inference I/O; keep semantic result validation on the owned values. Worker endpoint transport itself also belongs outside semantic ownership.
10. **GIS inference schema measures physical JSON bytes.** `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧬️schema/🟦️.ts:502,509` applies `TextEncoder().encode(JSON.stringify(request)).length` as a route byte ceiling. Keep domain lifetime/hash validation semantic; perform serialized request byte admission in its physical request I/O owner.

ArtifactRef imports under shared framework I/O schema were excluded: they are neutral owned boundary contracts. Drawing geometry imports under framework 2D text helpers were excluded: text layout is not wire serialization. Test/probe JSON calls, diagnostic formatting, generic grouping keys and repo testing infrastructure were excluded. Print inference planner JSON strings embedded as polygon/path table fields remain a separate semantic representation question; no blanket relocation recommendation is made without deciding the authored meaning of those columns.

Root acknowledged assignment of Rust findings 1–6 to existing extraction agents; this report is independent audit evidence and does not claim they remain unresolved after those agents finish.

## Literal Module And Include Existence Follow-Up

Rust Unicode escapes were decoded before existence checks. These are literal relative references in artifact schema and I/O source only; no full Rust nested-module resolution is claimed. Inline test blocks are included. Concurrent edits may have repaired sites during the scan. Missing paths are candidates, not all proven relocation regressions.

| Source Site | Missing Relative Target |
| --- | --- |
| `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs:573` | `🧪️tests/🔬️retained-mounted-laws/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:134` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:153` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:199` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:136` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:15` | `🚦️native/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🧪️tests/💰️backing/🦀️.rs:5` | `../../../../../🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/💰️backing/🔣️.json` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:148` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:15` | `🚦️native/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:206` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🧬️mutations/🦀️.rs:331` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧬️schema/🧬️mutations/🦀️.rs:163` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧬️schema/📸️snapshot/🦀️.rs:14` | `🪶️sqlite/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/🧬️mutations/🦀️.rs:241` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:156` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:214` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:11` | `🚦️native/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🧪️tests/💰️backing/🦀️.rs:5` | `../../../../../🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/💰️backing/🔣️.json` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🧬️schema/🧬️mutations/🦀️.rs:324` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🧬️schema/📸️snapshot/🦀️.rs:264` | `🪶️sqlite/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:561` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:217` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:225` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:234` | `🚦️native/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:236` | `🧪️tests/🪶️sqlite/🧹️lifecycle/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:176` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/🧬️mutations/🦀️.rs:342` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/🧬️mutations/🦀️.rs:258` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/🧬️mutations/🦀️.rs:316` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🧬️mutations/🦀️.rs:534` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:44` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/🧬️mutations/🦀️.rs:249` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/🧬️mutations/🦀️.rs:179` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧬️mutations/🦀️.rs:331` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/🧬️mutations/🦀️.rs:232` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/🧬️mutations/🦀️.rs:218` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🦀️.rs:147` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:85` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:1007` | `🚦️native/🦀️.rs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:71` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/🧬️mutations/🦀️.rs:203` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:190` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/🧬️mutations/🦀️.rs:251` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🦀️.rs:208` | `💾️binary/📡️.protocol.semio` |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📨️response/📤️export/🧪️tests/🦀️.rs:5` | `../🧫️fixtures/🔣️.json` |

## Production Serializer Follow-Up

11. **glTF rename schema retains protocol façades.** `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌳️node/🏷️rename/🦀️.rs:37` starts a production façade block. GraphQL and proto adapters are public at lines 136 and 140; `FacadeProtobufReader` at line 144 owns varint/key/length-delimited reading, and `decode_gltf_change_node_name_protobuf` at line 281 accepts physical octets. No test-only gating surrounds the block. Move format adapters to canonical text/binary mutation facets, retaining domain mutation application in schema.
12. **GIS map schema parses JSON descriptors.** `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:149–150` exposes `gis_map_document_from_descriptor_json` and calls `serde_json::from_str`, silently substituting an empty object on decode failure. The format parse belongs in text snapshot/descriptor I/O; the semantic feature projection can remain typed.
13. **glTF schema retains unconditional typed serializers.** The glTF snapshot file `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🦀️.rs:29–30` imports serde serializer machinery; `GltfJson` implements Serialize at line 73 and ordered attribute map serialization remains at line 249. Unconditional serde derives remain alongside neutral Value derives. The file header explicitly documents unconditional retention. This is a genuine typed format ownership finding rather than a neutral `ToValue` helper.
14. **Semio flattened-scene schema retains cache serializer bridge.** `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/💡️inferences/🎛️flattened-scene/🦀️.rs:48–61` implements unconditional serde Serialize/Deserialize through serde_json::Value. The preceding documentation cites a cache codec trait bound, which should be handled by the cache/I/O owner rather than schema format impls. This audit does not establish whether that documented bound remains current.

Forms dictionary serde bridges and Layout ChangeDataFields serde bridges were independently checked and are cfg(test), so excluded from production findings. glTF double-option Value adapters, DAG Value bindings, Print structural read_path/write_path, finite-element load helpers, and geometry inference result Value projections are semantic rather than physical I/O. Production host searches for old artifact schema text/binary/sqlite codec references returned no matches under the targeted patterns; lifecycle runtime behavior remains untested.

## TypeScript Relative Import Existence Follow-Up

Checked 4887 artifact schema/I/O TypeScript files for literal relative imports; dynamic variable imports and alias imports excluded. Concurrent state applies.

| Source Site | Missing Relative Target |
| --- | --- |
| `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:42` | `../../../../../../../../../../../../../🧰️framework/🔨️modules/📡️replication/🔗️causal/🧬️schema/🔣️history-transition/🔣️.json` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/🧬️schema/🧪️tests/🪶️sqlite/🟦️.ts:7` | `../../🪶️sqlite/🟦️.ts` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🌍️geojson/🧬️schema/🧪️tests/🪶️sqlite/🟦️.ts:5` | `../../🪶️sqlite/🟦️.ts` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:29` | `../../../../🧬️schema/📸️snapshot/🛬️native/🧬️schema/🔣️.json` |

## Additional Schema Physical Callers

15. **Remodeling schema JSON wrappers.** `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:169,180` retains public `apply_remodeling_mutation_json` and `undo_remodeling_mutation_json`, calling physical bridge decode/render helpers. Their helper definitions were absent from the observed file, suggesting concurrent extraction; recheck final assembly instead of treating this snapshot as a final unresolved compiler failure. Keep semantic diff/apply/inverse typed and put external JSON wrappers in I/O.
16. **Jack schema emits fixture JSON.** `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗣️language-service/🦀️.rs:533` defines production `example_graph_snapshot_json` using `semio_framework_pack_json::to_json_string`. Typed example fixture creation is semantic; JSON output is text snapshot I/O.
17. **Jack executor estimates physical JSON byte admission.** `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🪜️execution/🦀️.rs:112,125` defines JSON escaped string byte counting and recursive JSON upper bounds, consumed at lines 454 and 547. This does not serialize directly, but ties semantic execution limits to a concrete physical JSON output representation. Decide whether these represent physical response admission (move to text inference/query output I/O) or neutral semantic ownership bounds (replace JSON-specific escape counting with owned value accounting).

Gisterrain imported-map `from_json` at line 23 parses physical JSON via framework pack JSON and belongs with its already identified `to_json` text codec. Existing runtime extraction ownership applies.

## Jack Output Admission Call-Chain Conclusion

The executor JSON accounting is a confirmed physical dependency, not merely a neutral retained-memory estimate. QueryExecutionPreparation at execution line 187 estimates metadata as `copied_bytes * 6 + 128`; ReturnExecution receives that estimate. It counts column JSON escaping at line 454 and recursive JSON value upper bounds at line 547 before accepting a typed result. The editor run-query job (`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/▶️run-query/🧵️job/🦀️.rs:254–257`) consumes typed `(result, effects)` directly, with no JSON encoding. Thus otherwise identical semantic output admission depends on a physical JSON escape expansion even for typed consumers.

Use the existing `property_owned_bytes`, `node_owned_bytes`, `edge_owned_bytes` and literal owned UTF-8 lengths with a neutral owned-value grant for semantic result retention. Include vector/entity structural costs and maintain work/depth/item limits. Rename the semantic grant to reflect retained ownership rather than physical output. Enforce the separate encoded-byte maximum at the text response owner through existing `semio_framework_pack_json::to_json_string_controlled` (`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:1620`) and `NativeEncodeControl`; its bounded sink at line 1764 refuses append before exceeding `maximum_bytes`. Do not move the whole semantic execution cursor to I/O or remove admission limits.

18. **Jack I/O owns domain query application.** `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs:165` implements `execute` over already typed Graph/Query. Its apply closure at line 171 calls `apply_graph_effects`; it interprets MATCH/WHERE/CREATE/DELETE/SET/MERGE clauses and builds typed returns. `run` at line 221 applies effects after parsing. Restore typed execute/matching/evaluation/result construction to semantic executor; keep text parsing and encoded response orchestration with I/O. Calls from I/O to semantic operations are acceptable; implementing those operations in I/O is not.

## Deep Manual Codecs And Typed Serializer Follow-Up

19. **XML mutation support remains a physical codec owner.** `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🔨️modules/🧬️mutation-support/🦀️.rs:22,40` defines textual encode/decode_snapshot and imports actual I/O text diff codec helpers. Lines 59 and 83 define binary snapshot encoding/decoding; the latter reads `store::ByteReader` flags at lines 90–92. Keep XmlNodePath semantic; extract text and binary helper blocks to corresponding snapshot or mutation-document member I/O facets.
20. **SVG has the parallel deep codec leak.** `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🔨️modules/🧬️mutation-support/🦀️.rs` retains equivalent snapshot text/binary helpers, with physical byte flag reads at lines 96–98. These helpers are live production, not cfg(test).
21. **Puzzle play snapshot manual serde.** `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:997,1003` implements unconditional Serialize/Deserialize for Puzzle3dPlaySnapshot through its cached Value. Physical adapter ownership belongs to I/O; typed semantic projection can remain.
22. **Semio AABB cache serde bridge.** `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/💡️inferences/📦aabb/🦀️.rs:55,61` unconditionally implements serde serialization/deserialization through JSON values. Same physical cache-bridge finding as flattened scene.
23. **Puzzle fill checkpoint binary layout.** `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:805,819` defines FillRunCheckpoint encode/decode methods for a fixed 68-byte little-endian layout. Fields describe semantic resume meaning; layout methods and physical BYTES constant belong to the binary inference checkpoint/host owner. This is separate from WFC snapshot bytes.

Read-only deeper scanning found no other live endian/varint reader implementations beyond these concrete owners and the previously assigned glTF protobuf reader. Lowpoly read_u32/read_u8 closures decode intrinsic Values, not physical byte streams; comments naming moved binary codecs were excluded. Neutral retirement cursor types were not classified as I/O merely because they contain Cursor in their name. No runtime or compiler result is claimed.
