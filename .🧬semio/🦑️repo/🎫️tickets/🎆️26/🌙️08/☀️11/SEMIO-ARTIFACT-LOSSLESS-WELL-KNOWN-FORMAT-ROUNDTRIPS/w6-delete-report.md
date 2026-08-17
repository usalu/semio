# W6 — Framework MediaFormat/ArtifactCodec Deletion — Report

## Scope actually executed

Framework-side `MediaFormat` retirement per the master plan's "V7 deletion" section, steps
1–5, across 14 files:

- `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` — `ArtifactKindSpec.{export,import}_formats`,
  `AppIo.{export,import}_formats`: `Vec<MediaFormat>` → `Vec<String>`. `MediaWireFormat::Binary{format}`
  → `Binary{format_kind: String}`. `MediaPayload::Binary{format}` → `Binary{format_kind: String}`.
  Dropped `use crate::mesh::MediaFormat`, a stale `MediaFormat` round-trip test, and the
  `MediaFormat::export()` ts_rs typegen line.
- `🧰️framework/🔨️modules/🧊️3d/📐️brep/🧰️kernel/🦀️component.rs` — `SolidExporter`/`SolidImporter::format(&self)
  -> MediaFormat` → `format_kind(&self) -> &'static str`; updated all 8 Step/Stl/Obj/Glb impls.
- `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs` (biggest cut, −1105 net lines) — deleted the
  `MediaFormat` enum + impl, `StdioFormatEntry`/`STDIO_FORMAT_CATALOG`/`normalize_stdio_format_kind`/
  `stdio_format_entry`/`stdio_format_kind_id`/`stdio_accept_filter`/`stdio_mimes_csv`, the
  `ArtifactCodec<T>` trait and all 20 concrete impls (Txt/Md/Json/Csv/Bmp/Png/Jpg/Gif/Tiff/Pdf/Zip/
  Bcf/Docx/Pptx/Xlsx/Ply/Las/Gltf/Dxf/Ifc Codec), and the neutral document models
  (`RasterImage`/`PageDoc`/`PageDocPage`/`TableDoc`/`TextDoc`/`Archive`/`ArchiveEntry`).
  `MeshExporter`/`MeshImporter::format(&self) -> MediaFormat` → `format_kind(&self) -> &'static str`
  (6 impls updated: Obj/Glb/Stl Exporter+Importer). Kept `IoError` (changed
  `Unsupported(MediaFormat)` → `Unsupported(String)`). Deleted 2 dead tests
  (`os_media_format_str_mime_binary_and_parse_round_trip_all_variants`,
  `document_codecs_round_trip_text_table_raster_archive`), added one covering
  `MeshExporter`/`Importer::format_kind()`.
- `🧰️framework/🛍️products/💻️os/🦀️component.rs` and `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs`
  (near-duplicate media-registry logic in each, per the master plan's explicit "os + host dupes"
  note) — `registry_export_media`/`registry_import_media` switched from `io_resolve` + manual
  `(entry.compose)(...)` to real `io_dispatch` (subset validation + fallback dispatcher, master
  plan step 1). Deleted `export_handlers`/`import_handlers`'s MediaFormat-typed entry points
  (`register_os_media_export_handler`/`register_os_media_import_handler`,
  `export_os_app_instance_media`/`import_os_app_instance_media`,
  `os_media_export_extension_for_format`, `os_media_export_key`, `media_accept_filter`,
  `required_media_formats`, `assert_os_media_{export,import}_coverage`, `MediaDirection`) — kept
  only the `_kind` (string) forms per the master plan's "keep only the string-kind (_kind) forms".
  `register_2d_export_handlers`/`register_mesh_{exporter,importer}`/`register_mesh_dwg_{export,
  import}_handler`/`SolidMediaExport` (`register_solid_exporter/importer`, `solid_exporter_for`,
  `export_registered_solid`, `import_registered_solid`) rewired onto `format_kind()`/string kind
  ids, deriving ext/mime/binary from the io module's `format_descriptor`. `normalize_stdio_format_kind`/
  `stdio_format_entry`/`stdio_accept_filter` call sites (including the two WASM bridge fns
  `wasm_media_accept_filter_kinds`/`wasm_normalize_stdio_format_kind`) switched to
  `format_descriptor`/`format_accept_filter`/`normalize_format_kind` (step 2).
  `negotiate_wire_format`/`registry_shared_stdio_dialect` (used `MediaFormat::parse`) rewritten to
  return/build plain `String` format kinds via `format_descriptor(..).short_id`.
  `OsArtifactDescriptor.{export,import}_formats: Vec<MediaFormat>` → `Vec<String>`. Removed 2 dead
  tests per file that only exercised the deleted coverage-assertion functions; fixed remaining
  tests to key on `os_media_handler_key(kind, "dwg"/"glb"/"obj"/"step")` instead of
  `os_media_export_key(kind, &MediaFormat::X)`.
  Note: `🧰️framework/🛍️products/💻️os/🦀️component.rs` is not mounted by any crate (`#[path]`
  search across the repo found zero mounters) — edited anyway to satisfy the grep-zero exit gate,
  but not verifiable by `cargo check`; `🖥️host/🦀️component.rs` (mounted as `host_core` in the
  `semio-framework-os` crate) is the one actually compiled and iterated against.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — dropped the `MediaFormat`
  re-export, `MediaArtifactError::NoImporter(MediaFormat)` → `NoImporter(String)`, fixed the
  `MediaWireFormat::Binary{format}` match arm.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs` — `MediaContract`'s hand-rolled
  DSL encode/decode (`media_contract_to_record`/`media_contract_from_record`) updated for
  `MediaWireFormat::Binary{format_kind}`; dropped `MediaFormat::parse` (no longer needed — the wire
  word already is the format-kind string); fixed the one literal test.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs` — `media_to_artifact`/`media_from_artifact`
  updated for `MediaPayload::Binary{format_kind}`/`MediaWireFormat::Binary{format_kind}`; mime
  lookup for blob storage now via `format_descriptor(&format_kind).mime`.
- `🧰️framework/📦️packages/🦀️rust/📦️glue.rs` — pruned the dead mesh re-export list (MediaFormat,
  StdioFormatEntry/STDIO_FORMAT_CATALOG + its 4 fns, ArtifactCodec + 20 concrete codecs, the 7
  neutral document types); kept `IoError`/`MeshExporter`/`MeshImporter`/Obj/Glb/Stl
  Exporter/Importer/DWG family/`mesh_to_obj` family — all real, still-alive code (see "Deliberate
  deviation" below). Added `FormatDescriptor`, `register_format_descriptors`, `format_descriptor`,
  `normalize_format_kind`, `format_accept_filter`, `formats_csv` to the `io::` re-export block (the
  step-2 replacement APIs, needed by the os files and one plugin fix below).
- `✏️s/🔌️plugins/🪐️space/…/🔗️connections/🦀️component.rs` and `…/🖼️media/🦀️component.rs` — the one
  genuine plugin-side fallout: `ArtifactKindSpec` test fixtures (`vec![MediaFormat::Svg]` →
  `vec!["svg".into()]`, same for Glb), and `normalize_stdio_format_kind` → `format_descriptor(..)
  .short_id` at 3 call sites (media export/import command handlers), preserving the pre-existing
  short-id semantics the code and its test (`export_media_emits_download_effect_and_import_requests_file_open`)
  depend on.
- `✏️s/🔌️plugins/🏭️process/…/⚙️engine/🦀️component.rs` — the one plugin call site that invoked
  `SolidExporter::format()` directly (`export_process3d_model`); switched to `.format_kind()` +
  `format_descriptor` for ext/mime/binary.

## Deliberate deviation from the master plan's literal step-4 wording — flagged, not silent

Step 4 lists, in one breath: "the `MediaFormat` enum itself, `ArtifactCodec<T>` trait + impls,
`MeshExporter`/`MeshImporter` impl families, `mesh_to_obj`/`mesh_to_glb`/`mesh_to_stl` + `_from_*`
counterparts … the hand-rolled DWG codec (~1000 LOC) … StdioFormatEntry/STDIO_FORMAT_CATALOG."
Before cutting anything I ran real external-consumer greps (not just the `MediaFormat` census) for
every symbol in that list, because the plan's own prose only justifies the `mesh_to_obj`-family
deletion with "safe to delete now that nothing calls them" — it does **not** make that claim for
`MeshExporter`/`MeshImporter` or the DWG codec, and the evidence shows why:

- **`MeshExporter`/`MeshImporter` + `ObjExporter`/`GlbExporter`/`StlExporter`/`ObjImporter`/
  `GlbImporter`/`StlImporter`**: real, non-`MediaFormat`-text consumers in 9 plugin files across 7
  plugins never touched by W5 for this mechanism (remodel, process, cad, demonstrator ×3,
  procedural, puzzle ×2, lowpoly) plus the OS product's `register_mesh_exporter`/`register_mesh_importer`.
  None of these call sites spell `MediaFormat` (they just pass `Box::new(ObjExporter)`), so W5's
  MediaFormat-grep-based migration never saw them — this is a real gap in the W0/W6 census's
  coverage, not a false alarm. Deleting the trait would have broken 9+ plugins outside this wave's
  write scope. **Fix applied**: kept the traits/structs, renamed `format() -> MediaFormat` to
  `format_kind() -> &'static str` (zero-touch for all 9 plugin call sites — none of them call
  `.format()` themselves, only `process3d` did and is fixed above).
- **The hand-rolled DWG codec** (`DwgDrawing`/`DwgEntity`/`DwgLayer`/`DwgColor`/`DwgGeometry`/
  `DwgBitWriter`/`DwgBitReader`/`dwg_to_bytes`/`dwg_from_bytes`/`mesh_to_dwg_drawing`/
  `dwg_drawing_to_mesh`/`paths_to_dwg_drawing`/`dwg_drawing_to_paths`/`DwgPathSegment`, ~1226 LOC):
  grep-verified **zero** `MediaFormat` references anywhere inside it (the exit gate never required
  its removal), and 19 real external consumer files spanning the OS product itself
  (`register_2d_export_handlers`, `dwg_drawing_to_svg`, `svg_to_dwg_bytes` — all load-bearing 2D
  export infrastructure) plus 8 plugins, including stdio's own `semio/cad` and `semio/drawing`
  subset snapshots. Deleting it would be a large, uncontrolled, multi-plugin migration completely
  outside "framework MediaFormat deletion" scope and with zero benefit to the actual acceptance
  bar. **Left fully intact.**
- **`RasterImage`/`PageDoc`/`TableDoc`/`TextDoc`/`Archive`/`ArtifactCodec<T>` + its 20 concrete
  codecs**: verified **zero** real external consumers (every apparent "hit" outside `mesh/component.rs`
  was either a same-named-but-unrelated local type in another plugin, e.g. stdio's own `pdf`
  artifact's `PageDoc`, or an OS-canvas `RasterImage` renderer type) — deleted outright as the plan
  intended.

## Exit checklist

```
grep -rn "MediaFormat" --include="*.rs" ✏️s 🧰️framework | grep -v "🎫️tickets" | wc -l
```
→ **0**

```
cargo check -p semio-framework
```
→ clean (0 errors; only pre-existing `semio-framework-os-kernel` warnings, e.g. unused
`extern crate` aliases, unrelated to this ticket).

```
cargo check -p semio-s-plugin-stdio --lib
```
→ clean (0 errors; 493 pre-existing warnings, unrelated).

```
cargo test -p semio-s-plugin-stdio --lib
```
→ **1930 passed; 0 failed; 3 ignored** (W0 baseline was 1075/0 — monotonically grown, consistent
with W2–W5 landing since).

```
cargo check --workspace --keep-going
```
→ 118 `error[...]` lines total, **zero** mention `MediaFormat` (verified via `grep -i mediaformat`
on the full log). Classified:

- **57** in `semio-framework-os-kernel-db` (`db_storage`/`db_state`/`DbError`/… unresolved module
  cascade) — unrelated db crate, foreign, pre-existing (git status showed no MediaFormat-related
  file involvement).
- **22** in `semio-compose-rs` — unrelated crate, foreign.
- **14** in `semio-framework-os` (`--features os-host-full`) — `OsAppRegistration`/`AppDefinition`
  both declare `pub label` twice (`LocalizedLabel` then `Vec<String>`, a literal duplicate-field
  merge artifact) plus the cascading `document` field and `ArtifactEnvelope` `dialect`/
  `migrated_from` errors this triggers. Confirmed pre-existing at `git show HEAD` (file was clean
  before I touched it, and my diff never touches `OsAppRegistration`/`AppDefinition`/
  `ArtifactEnvelope`) — matches this ticket's own `📌️important.md` "os-run blocker" note. Default
  features (no `os-host-full`) compile **clean**.
- **~10** "couldn't read `…/📌️panels/📄️document/🦀️component.rs`" across `block`/`dag`/`imperative`/
  `forms`/`reasoning-mindmap`/`sequence`/`vcs`/`flow`/`mathematical`/`sourcing` — stale `#[path]`
  entries in each plugin's own `glue.rs` pointing at a `📄️document` dir renamed to `📄️artifact` in
  a prior commit; same class of issue the W6 lowpoly migration report already flagged and spawned a
  follow-up task for. Foreign, not `MediaFormat`.
- **3** in `semio-s-plugin-process` (`JsonValue`/`Value` mismatch in the stdio JSON deserializer) —
  already documented by the process W6-migrate report as live concurrent `🗄️stdio` churn at the
  time.
- Remainder are duplicate/cascading reports of the above (e.g. `semio-s-plugin-*` "due to N
  previous errors" summary lines).

## Files touched
- `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`
- `🧰️framework/🔨️modules/🧊️3d/📐️brep/🧰️kernel/🦀️component.rs`
- `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs`
- `🧰️framework/🔨️modules/🚪️io/🦀️component.rs` (comment only)
- `🧰️framework/🛍️products/💻️os/🦀️component.rs` (orphaned/unmounted — see note above)
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs`
- `🧰️framework/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎮️commands/🔗️connections/🦀️component.rs`
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎮️commands/🖼️media/🦀️component.rs`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️component.rs` (comment only)

Not touched (confirmed dead/out of scope, or foreign in-flight): `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`
(staged by another session, zero `MediaFormat`), any `db`/`compose-rs`/panel-path breakage above.
