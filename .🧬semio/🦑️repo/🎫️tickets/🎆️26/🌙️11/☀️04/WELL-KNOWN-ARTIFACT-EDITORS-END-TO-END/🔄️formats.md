# Formats: import/export contract and codec status (T04)

## 1. How the OS app reaches a codec

Open and Save of a real file go through exactly one route per editor, the natural-file route:

1. `ArtifactEditor::natural_file_codec() -> Option<NaturalFileCodec { format_kind, extension, media_type, binary }>`.
2. `encode_natural_file(&Snapshot) -> Result<Vec<u8>, MediaError>` (Save) and `decode_natural_file(&[u8]) -> Result<Snapshot, MediaError>` (Open).
3. `PluginBuilder::editor` mounts the paired `Save File` / `Open File...` actions (`mount_natural_file_actions`) only when `natural_file_codec()` is `Some`. Without it the editor has no file import/export at all.
4. Renderer: `ShellHelpers/transfer-checkin-tutorial` (`naturalFileFormatV1`, `naturalFileNameV1`, `openNaturalFileOwnerV1`) reads the codec only from the Open/Save action defaults (`formatKind`, `extension` matching `^\.[A-Za-z0-9]+$`, `mediaType`, `binary`).
5. Export media: `produce_media(NATURAL_FILE_PORT)` yields `MediaArtifact { kind_id = format_kind, wire = Binary{format_kind}, data = bytes }`. Import: `consume_media` requires `wire.format_kind == codec.format_kind`, then `import_media` -> `decode_natural_file` -> `document_reference_load_emit` (requires an explicit `reference_snapshot_codec`).

Rules for every batch editor:

- `format_kind` is `s.stdio.<artifact>@<standard>` (one value for all subsets of a standard, e.g. `s.stdio.svg@1.1`).
- `extension`, `media_type`, `binary` come verbatim from the artifact's `📜️artifact-definition.json` `representations[0]` (`extensions[0]`, `mimes[0]`, `is_binary`). Never invent them.
- `decode_natural_file` / `encode_natural_file` delegate to the artifact's real byte codec (`decode_x` / `encode_x`). No byte-replay side channel (see `logical-materialization-contract.md` of the earlier ticket).
- Add the paired roster row to `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧫️fixtures/📄️natural-file-lifecycle/🔣️.json` (the Rust test asserts the row count) and one editor unit test following `🪟️bmp/.../✏️editor/🧪️tests/🔬️unit/🦀️.rs::natural_file_route_preserves_bmp_v3_bytes_and_opens_a_fresh_snapshot_event`: decode fixture, encode, decode again equal, plus an independent third-party parse of the exported bytes.
- The native codec factory (`stdio.native.<x>.v1`, extension `semio`/format ext) is a different layer: the artifact's snapshot pack for the document store. It is not the OS file import/export route. Do not confuse the two.

## 2. Export file name, MIME, encoding (fixed in this ticket)

- File stem = sanitized document name (`sanitize_export_file_stem(name, fallback)` in `🖥️host/🔀️workflow`): reserved characters `<>:"/\|?*` and controls become `_`, trailing dots/spaces removed, Windows device names (`CON`, `PRN`, `AUX`, `NUL`, `COM1-9`, `LPT1-9`) prefixed with `_`, at most 100 characters, fallback artifact kind/id, final fallback `document`.
- Extension and MIME come from the format descriptor (`FormatDescriptor.extensions[0]`, `mimes[0]`); binary data is base64 with `encoding: "base64"`, text data is UTF-8.
- Workflow node exports use `node.label` as the document name (was the artifact kind, e.g. `2d.note.svg`).
- Space pack/DSL exports return `BackboneExport { name, files }`; hub commands name downloads `<stem>.pack`, `<stem>.ops`, `<stem>.os`.

## 3. Codec status of the 38 stdio artifact definitions (reconciled with `📊️coverage-matrix.md`, update 2026-11-04)

Authority for per-row status is `📊️coverage-matrix.md`; this table only restates the format-routing columns. NF = natural-file route (Open/Save of a real file). Native = `stdio.native.*` receipt in `🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/📜️native-codec-factories.json`. "uncompiled" means written in this ticket and not yet built.

| Artifact | NF | Native receipt |
| --- | --- | --- |
| txt, md, csv, tsv, json, xml, html | yes | yes |
| png, jpg, svg | yes | yes |
| bmp | yes | yes (uncompiled, generators pending) |
| gif (87a, 89a), tiff | yes (uncompiled) | yes (uncompiled) / yes |
| wav | yes | yes (uncompiled, placeholder `pack_schema_hash` must be refreshed) |
| mp3, mp4, avi | yes | yes (extension `semio`) |
| docx, xlsx, pptx | yes | yes |
| pdf (1.7 base and subsets a, e, h, ua, vt, x; 1.4 base, a, x) | yes (uncompiled; 1.4x no) | yes (two factories) |
| dxf, dwg (ac1018, ac1024) | yes (uncompiled) | yes |
| obj, stl, ply, gltf, las | yes (uncompiled; gltf saves `.gltf` only) | yes |
| step (cc1 to cc6) | yes (uncompiled) | yes |
| zip, deflate | yes (uncompiled; zip iso21320 surface has no codec) | yes |
| binary, epw | yes (uncompiled) | no |
| ifc (4, 2x3 base, cobie, cv20, sav, 4x3) | yes (uncompiled) | no (none declared) |
| bcf | yes (uncompiled) | yes (extension `semio`) |
| idf | yes, through the energy model `#editor` (uncompiled) | no (definition-only artifact) |
| idml | yes (uncompiled, new surface crate) | no; not in the hub roster, needs `composition-prepare` |
| semio (19 subsets) | yes (this section 5, uncompiled) | none declared |
| inventory | library, no definition | no |

The earlier count of 21 artifacts without a natural-file route is stale: batches C, D, E, F and H wired gif, tiff, dxf, dwg, pdf subsets, zip, deflate, binary, epw, bcf, ifc and idf. The only family that lacked one at my last report was semio, which this ticket now wires.

Open registration gap: `avi`, `bcf`, `mp3` and `mp4` declare native factories with extension `semio`, while the others use their file extension.

## 4. Shared defects fixed (tests added; none compiled yet at the time of writing)

| Defect | Fix | Test |
| --- | --- | --- |
| Export names used the kind id | stem from the document name, sanitized | `workflow-unit::export_file_stems_come_from_the_document_name_and_are_safe_on_every_platform` against the Python-generated fixture `🧫️fixtures/📄️export-file-stems` |
| Pack and DSL export dropped `BackboneDocument.name` | `BackboneExport { name, files }`; hub export commands name downloads from it | host-unit `space_pack_export_keeps_the_document_name_and_import_never_overwrites` |
| Envelope registry ignored version | structured key (envelope id, component, version) | semio-unit `registry_resolves_envelopes_by_identity_and_version` |
| `wrap_text` trimmed BOM and leading whitespace | exact `preamble\n` + body; split removes one line break and a file-level BOM; a body that looks like a preamble is never interpreted | semio-unit `text_body_round_trips_exactly_including_bom_and_leading_whitespace` against the Python `codecs` fixture `📜️text-framing`, `wrap_text_never_interprets_a_body_that_looks_like_a_preamble` |
| Two envelope id parsers disagreed on dotted ids | both split at the last dot; `envelope_from_filename` deleted | `one_parser_reads_every_envelope_identity_the_same_way` |
| Registry registration replaced silently, poisoned lock panicked | duplicate refused unless the identical handler; poison returns `RegistryUnavailable` | `registry_resolves_envelopes_by_identity_and_version` |
| Space import overwrote by id | refused | same host-unit test as above |
| File space wrote `<stem>.<ext>.*.semio` companions | the chosen file holds the whole payload, atomic temp and rename, never emptied | host-unit `file_space_port_persists_exactly_the_chosen_file_with_no_companions` |
| Catalog uris lived in a process-global map keyed by pointer | `OsBackbonePorts` is a struct (`OsBackbonePorts::store(..)`, `::space(..)`) holding its own uri set; the global map is removed; the 10 construction sites are updated | host-unit `catalog_state_does_not_outlive_its_port` |
| Failed port read hid spaces | the error is returned | covered by the same listing path |
| Saved-at sorted as text | typed instant (`saved_at_nanoseconds`), unknown formats refused | host-unit `saved_at_stamps_sort_as_instants_and_never_as_text` against `🕰️saved-at-stamps` generated with Python `datetime` |
| Sanitizer capped by characters and missed `COM¹²³`/`LPT¹²³` | capped at 100 bytes on a character boundary; superscript device names reserved | the stem test above |

## 5. The `.semio` natural-file dispatcher

- Naming rule: artifact KIND ids and dialect `artifact_kind` stay `s.`-prefixed (`s.stdio.semio`, including kit child `#[child(kind = "s.stdio.semio")]`). Only DOCUMENT schema ids are `stdio.semio.<subset>` (`stdio.semio` for the base subset), the stdio `artifact_schema` convention such as `stdio.csv`. Capability ids (`s.stdio.semio.text.inference`) and test-local tags (`.../v1`) are different namespaces and were not touched.
- Generated component captures (`🧩️component/🔣️.json`) and descriptors (`🛂️.descriptor.semio`) still carry the old length-prefixed `s.stdio.semio.<subset>` strings and must be regenerated by the repo tooling (T06).
- `🧬️semio/🦀️.rs`: `dispatch(bytes)` sniffs the versioned envelope (text preamble or binary header, file-level BOM tolerated) and returns it with the handler registered for exactly that identity and version; `registered_envelope_keys()` lists the roster; `NATURAL_FILE_EXTENSION` is `.semio`.
- `✉️base/🦀️.rs`: `semio_natural_file!(Snapshot, "stdio.semio.<subset>")` declares `natural_file_codec` (`stdio.semio.<subset>@v1`, `.semio`, `application/vnd.semio`, binary), `encode_natural_file` (the subset pack, which already carries its envelope) and `decode_natural_file` (routes through `dispatch`, then decodes with the subset's own identity check, so another subset's file is refused). The envelope is registered once per subset, when the codec is first mounted (a `OnceLock`); a refused registration leaves the codec unmounted instead of panicking. All 19 subset editors invoke the macro right after `const DOCUMENT_SCHEMA`.
- Dispatcher law: `🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/🧿️semio-natural-dispatch/🦀️.rs` (`semio_natural_dispatch`): 19 subsets, each registered exactly once, each reopens its own export, foreign subset files are refused.

## 6. Typed localized `MediaError` payload: plan and blockers (not started in code)

Measured scope (Grep, 2026-11-04): `MediaError::Payload` is constructed at about 222 sites in 91 files (stdio editors, gis, sequence, layout, shooting, sourcing, forms, trinity, raster, procedural, process, remodel, energy, norm, window-kit contract), and matched exhaustively in the media owner (`🧰️framework/🔨️modules/🛂️manifest/🎞️media/🦀️.rs` Display), the original-completion metadata metering (`🔌️plugin/🎞️media/⚠️turn`, `📜️declaration/🎞️media`), their pointer-identity ownership tests, and the wire projection `MediaArtifactError`.

Design (schema-first):
1. A schema document `MediaRefusal { code: string, labels: map<locale, string> }` next to the media manifest schema, generated into Rust (`RetireOwned`, `ToValue`/`FromValue`) and TS.
2. `MediaError::Payload(port, MediaRefusal)`; constructors `MediaRefusal::new(code).label("en", ..).label("de", ..)` and a `LocalizedRefusal` trait that typed errors (`EpwDecodeError`, `IdfImportError`, the semio mount refusal) implement, replacing every `From<..> for String` bridge.
3. The wire projection carries `code` and the label map; the shell renders it with `localizedLabelTextV1` using primary-subtag matching and the code as fallback.
4. All producers migrate in one pass, and every refusal carries `en` AND `de` at migration time; there are no `en`-only labels. Generic codec failures share one localized generic label (code `media.payload.codec-failure`, `en` and `de`) with the verbatim technical detail as a parameter (`detail`), rendered into the label by the shell. Typed errors (`EpwDecodeError`, `IdfImportError`, the semio mount refusal) supply their own code and both labels through `LocalizedRefusal`.

Sequencing (coordinator): the migration runs right after the first compile wave of the batch crates, together with the multi-format migration of the registry, on the coordinator's go.

Blocker: the ownership metering and pointer-identity tests make this a compile-driven change, and no cargo run has been granted capacity in this session (the semio crate run is still queued). Doing a 222-site edit blind would break the shared tree for every worker, so it is held until a run can verify it.
