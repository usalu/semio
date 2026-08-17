# W4a — Mesh Eviction Progress Log

Scope: evict non-mesh material out of `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs`.
File ownership: mesh/component.rs, manifest/component.rs (destination), stdio manifest +
io/component.rs (read-only reference), plus any discovered `mesh::MediaFormat` /
`mesh::STDIO_FORMAT_CATALOG` / `mesh::stdio_format_entry` / `mesh::normalize_stdio_format_kind` /
`mesh::stdio_accept_filter` / `mesh::stdio_mimes_csv` consumer files.

## Step 1 — Keep as-is (MeshData/Primitives/Obj/Glb/Stl codecs)
**Done, untouched.** Confirmed lines 1–805 of the current mesh.rs (MeshData, Primitives,
Obj/Glb/Stl codec functions) are unchanged, and MeshExporter/MeshImporter + Obj/Glb/Stl
exporter/importer unit structs (now ~1143–1216) are unchanged.

## Step 2 — Delete MediaFormat + StdioFormatEntry/STDIO_FORMAT_CATALOG/helpers
**DEFERRED — not deleted. Left fully in place in mesh.rs (region `MediaFormat` ~807–1034,
region `StdioFormatCatalog` ~1036–1140).**

Why I stopped instead of guessing:

- `MediaFormat` cannot be deleted without also touching code the assignment explicitly
  protects. `MeshExporter`/`MeshImporter` and their `ObjExporter`/`GlbExporter`/`StlExporter`
  unit-struct impls (step 1's "keep as-is" range) return/consume `MediaFormat` directly
  (`fn format(&self) -> MediaFormat`). Deleting the enum breaks step 1's own kept code.
- `MediaFormat` also has a genuinely large external fan-out I measured before touching
  anything: **58 files outside mesh.rs** reference `MediaFormat` (grep for the bare
  identifier, excluding mesh.rs, tickets, and the one `semio_framework_3d::mesh` false
  positive which is an unrelated 3d-kernel module of the same name). Most are per-artifact
  plugin engine/serializer files (cad, raster, gis, layout, shooting, fem, puzzle, lowpoly,
  draw, process, …) using `MediaFormat::Xyz` directly as a value, not just importing the
  name — a real type-level dependency, not a mechanical rename.
- `MediaFormat`'s own `stdio_kind_id()` method returns `&'static str` sourced from the
  static `STDIO_FORMAT_CATALOG` table; io's replacement `FormatCatalog` is a **runtime**
  `RwLock<HashMap<String, FormatDescriptor>>` populated via `register_format_descriptors`
  (owned `String` fields). Preserving `stdio_kind_id`'s exact per-variant behavior
  (verified: `Glb` is the one variant NOT in the stdio catalog, so it must keep returning
  `"glb"` not `"stdio.glb"` — everything else is `"stdio." + as_str()`) while switching its
  source to a `String`-keyed runtime registry is not a same-shape swap without either
  leaking memory to keep `&'static str` or changing the fn signature (which then needs its
  own caller audit).
- I traced the *actual* small set of external consumers of `stdio_format_entry` /
  `normalize_stdio_format_kind` / `stdio_accept_filter` (the task's own prediction of "os
  host/core… and possibly a few plugins" was correct: `🧰️framework/🛍️products/💻️os/🦀️component.rs`,
  `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs`, and
  `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎮️commands/🖼️media/🦀️component.rs`). Repointing them to
  io's `format_descriptor`/`normalize_format_kind` is not a drop-in replacement at several
  call sites: e.g. `os_media_export_extension_for_format_kind(...) -> Option<&'static str>`
  returns `row.short_id` — io's `FormatDescriptor.short_id` is an owned `String`, so this
  signature would have to become `Option<String>`, which then needs auditing every caller of
  that fn (not traced further once I hit this).
- Separately discovered (via a `#[path=...]` resolution scan of every `.rs` file in the
  repo) that `🧰️framework/🛍️products/💻️os/🦀️component.rs` — one of the three real call-site
  files above — is **currently not mounted by any crate** (no `#[path]` attribute anywhere
  resolves to it). It's orphaned mid-refactor, consistent with the "concurrent workflow
  module wiring" note in my task briefing. Another reason not to blind-edit it.

Net: kept `MediaFormat`, `StdioFormatEntry`, `STDIO_FORMAT_CATALOG`,
`stdio_format_entry`, `normalize_stdio_format_kind`, `stdio_format_kind_id`,
`stdio_accept_filter`, `stdio_mimes_csv` exactly as they were. No consumer files needed
changes for this step since nothing was deleted.

## Step 3 — Relocate manifest-vocabulary types
**Done and verified.** Moved (verbatim, byte-for-byte body) the contiguous block
`ArtifactKindSpec`/`OsMediaCapability` (region `ArtifactKind`), `MediaClass`/`MediaForm`/
`MediaType`/`MediaWireFormat`/`MediaPortDirection`/`PortMultiplicity`/`MediaPortSpec`/
`MediaCompat`/`media_types_compatible` (region `MediaType`), `ArtifactPresentation`/`AppIo`
(region `AppIo`), `ConfigFieldShape`/`ConfigFieldSpec`/`ConfigSpec` (region `ConfigSpec`),
`CommandFieldSpec`/`CommandVariantSpec`/`CommandGrammar` (region `CommandGrammar`), and
`Media`/`MediaPayload`/`MediaFingerprint`/`MediaError`/`MediaConverter` (region `Media`) —
mesh.rs lines 1144–1571 (428 lines) — into `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`,
wrapped in a new `//#region 🔖️MediaVocabulary` placed right after the existing
`//#region 🔖️Kernel`.

`MediaForm` was **left as the closed enum** (not opened to a `#[serde(transparent)]
pub struct MediaForm(pub String)`), per the task's own explicit permission to stop at that
specific sub-step if the fan-out is too large — I did not attempt to measure that fan-out
separately since the enum moved as-is regardless (moving doesn't require opening it).

Fixed every discovered import fallout:
- `mesh/component.rs`: removed the moved block; removed the now-unused `use dsl::DslValue;`
  (it was only used by `ConfigFieldSpec.default`, which moved).
- `manifest/component.rs`: head import `use crate::mesh::{MediaPortSpec, ArtifactKindSpec,
  ConfigSpec, CommandGrammar, AppIo};` → `use crate::mesh::MediaFormat;` (the 5 named types
  are now local; `MediaFormat` itself stays in mesh and the relocated types still reference
  it — `ArtifactKindSpec.{export,import}_formats`, `MediaWireFormat::Binary`,
  `MediaPayload::Binary`). Fixed the `#[cfg(feature = "typegen")] exports_typescript_bindings`
  smoke test: `crate::mesh::{OsMediaCapability,ArtifactKindSpec,MediaClass,MediaForm,
  MediaType,MediaWireFormat,MediaPortDirection,PortMultiplicity,MediaPortSpec}::export()` →
  `crate::ui::{…}::export()` (matching the existing `crate::ui::kernel::X` style already used
  for the nested kernel module in that same test); left `crate::mesh::MediaFormat::export()`
  as-is since that type didn't move.
- `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs` (mounted as `manifest::kernel`, discovered via
  grep for `mesh::MediaType`): `use crate::mesh::MediaType;` → `use crate::manifest::MediaType;`.
- `🧰️framework/📦️packages/🦀️rust/📦️glue.rs`: removed the 24 relocated names from the
  `pub use mesh::{...}` block (left `MediaFormat` and the still-mesh-owned names in place);
  they're reachable exactly as before via the pre-existing `pub use manifest::*;` glob, so
  external call sites need no changes.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`: `pub use
  semio_framework::mesh::{Media, MediaError, MediaFingerprint, MediaPayload};` →
  `pub use semio_framework::{Media, MediaError, MediaFingerprint, MediaPayload};`. **This one
  was NOT caught by grep** (`mesh::{Media` doesn't match a `mesh::Media\b` substring search
  because of the brace) — only surfaced via `cargo check -p semio-framework-os`, which is
  exactly why I compiled rather than trusting grep alone for the rest of this ticket.
- **Test fallout** (caught by `cargo check -p semio-framework --tests`, which a plain
  non-test `cargo check` does NOT compile — I had to re-run with `--tests` explicitly to
  catch this): mesh.rs's own `#[cfg(test)] mod tests` had 3 tests
  (`media_types_compatible_covers_direct_any_convert_and_reject`,
  `media_fingerprint_structured_hashes_json_binary_reuses_blob_hash`,
  `media_error_messages_are_human_readable`) that referenced the now-relocated
  `MediaType`/`MediaClass`/`MediaForm`/`MediaCompat`/`Media`/`MediaPayload`/
  `MediaFingerprint`/`MediaError` via `use super::*;`. Moved these 3 tests verbatim into a
  new `#[cfg(test)] mod media_vocabulary_tests { use super::*; ... }` in manifest.rs right
  after the new `🔖️MediaVocabulary` region (extending the file that now owns these types,
  per CLAUDE.md — did not create a new test file). Deleted them from mesh.rs's test module.
  The 2 neighboring mesh.rs tests that only touch `MediaFormat` (kept in mesh) or the
  still-in-place codec structs (`os_media_format_str_mime_binary_and_parse_round_trip_all_variants`,
  `document_codecs_round_trip_text_table_raster_archive`) were left untouched — they still
  compile fine since nothing they use moved or was deleted.

Verification:
- `cargo check -p semio-framework` — clean (only pre-existing warnings in unrelated crates).
- `cargo check -p semio-framework-os` — clean (transitively also builds
  `semio-framework-plugin`, `semio-framework-3d`, `semio-framework-ui`,
  `semio-framework-math`, `semio-framework-os-kernel`; this is what caught the
  plugin/component.rs `mesh::{Media, ...}` brace-import miss above).
- `cargo check -p semio-framework --tests` — down from 99 errors (all in mesh.rs's test mod,
  all from the 3 relocated-type tests) to **0 errors in mesh.rs or manifest.rs**. The
  remaining 39 errors are 100% confined to
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs` lines 2361–2580, all
  `cannot find function ... in module store::test_support` — unrelated to anything in this
  ticket, and matches this session's briefing verbatim ("a 🔁️workflow module being wired
  into os-kernel's glue.rs without its Cargo dependency yet"). Per instructions, did not
  touch this — it's a different session's in-flight work.
- `cargo check -p semio-framework-os --tests` — clean, 0 errors.

## Step 4 — Delete stdio-specific codecs (NeutralDocuments, Dwg, TextCsvJson, RasterCodecs,
## PageTableArchive, MeshExtraCodecs)
**DEFERRED — not attempted. All six regions left fully in place in mesh.rs**
(`NeutralDocuments` ~1218–1298, `Dwg` ~1300–2524, `ArtifactCodecs` wrapper ~2526–3139
containing `TextCsvJson`/`RasterCodecs`/`PageTableArchive`/`MeshExtraCodecs`).

Why: a first-pass grep across the workspace for the 19 per-format codec unit structs
(`TxtCodec`/`MdCodec`/`JsonCodec`/`CsvCodec`/`BmpCodec`/`PngCodec`/`JpgCodec`/`GifCodec`/
`TiffCodec`/`PdfCodec`/`DocxCodec`/`PptxCodec`/`XlsxCodec`/`ZipCodec`/`BcfCodec`/`PlyCodec`/
`LasCodec`/`GltfCodec`/`DxfCodec`/`IfcCodec`) and the `NeutralDocuments` payload types
(`RasterImage`/`PageDoc`/`TableDoc`/`TextDoc`/`Archive`/`ArchiveEntry`) showed **zero**
external consumers for most of them. That looked like a green light. But while separately
checking `IoError` (which DID show real hits) I found
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:9470` does
`pub use semio_framework::*;` — a full glob re-export of the entire `semio_framework` crate
root, at the `semio-framework-plugin` crate's OWN root. Through that glob, dozens of
downstream artifact-plugin files (confirmed real, live usage in the `puzzle` and `fem`
plugins' obj/zip/stl/png import/export serializer leaves) reach mesh's `ArtifactCodec`
trait and `JsonCodec` struct as bare `semio_framework_plugin::{ArtifactCodec, IoError,
JsonCodec, MediaFormat}` imports — a consumption path that does **not** show up as a
`mesh::X` (or even an obviously mesh-shaped) substring anywhere, so my grep sweep had
already silently missed it before I happened to check `IoError` by hand.

Given that a second real blind spot in my own grep-based consumer sweep surfaced within the
same session (the first was the `mesh::{Media, ...}` brace-import in step 3, only caught by
`cargo check`), and given the `Dwg` region alone is ~1225 lines with a real, substantial
external fan-out (`DwgDrawing`: 21 files; `dwg_from_bytes`/`DwgColor`/`DwgEntity`/
`DwgGeometry`: 11 files each, outside mesh.rs and glue.rs's re-export list), I don't have
grounds to trust a grep-only sweep here, and fully verifying every real consumer would mean
compiling every downstream plugin crate individually (dozens of crates) rather than the two
(`semio-framework`, `semio-framework-os`) this wave's checkpoints named. That's a
correctness bar I could not respons­ibly clear in this pass without materially raising the
risk of breaking the shared live tree for other concurrent devs. Per the ticket's own
"if a consumer's correct repoint isn't obvious, STOP… leave it… and continue with the
others" instruction, I stopped on all of step 4 rather than guess.

None of step 4's regions, types, or codec impls were touched. No test fallout either
(the DWG and document-codec tests at the tail of mesh.rs's `#[cfg(test)] mod tests` are
all still in place, unmodified, and still compile — confirmed by the `--tests` runs above).

## Step 5 — Final checkpoint
- `cargo check -p semio-framework` — clean.
- `cargo check -p semio-framework-os` — clean.
- `cargo check -p semio-framework --tests` and `-p semio-framework-os --tests` — clean
  except the 39 pre-existing/concurrent-session `store::test_support` errors in
  `🔁️workflow/🦀️component.rs` noted above (not mine, not touched).
- **Final line count of `mesh/🦀️component.rs`: 3582 lines** (was 4064 before this wave).
  It does **not** yet contain only `MeshData`/`Primitives`/generic mesh codecs — it still
  also contains `MediaFormat`/`StdioFormatCatalog` (step 2, deferred) and
  `NeutralDocuments`/`Dwg`/`ArtifactCodecs` (step 4, deferred), exactly as documented above.
  `manifest/🦀️component.rs` grew from 4278 to 4775 lines (the 428-line relocated
  `🔖️MediaVocabulary` region + its 58-line relocated test module).

## Files touched this wave
- `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs` (removed the manifest-vocabulary block + its
  3 tests; everything else unchanged)
- `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` (added `🔖️MediaVocabulary` region + its
  test module; adjusted head import; fixed the typegen smoke test)
- `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs` (one import line)
- `🧰️framework/📦️packages/🦀️rust/📦️glue.rs` (mesh re-export list trimmed)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (one import line)

## Files NOT touched (deferred consumers for steps 2 and 4)
- `🧰️framework/🛍️products/💻️os/🦀️component.rs` (unmounted/orphaned — see step 2)
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs`
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎮️commands/🖼️media/🦀️component.rs`
- every DWG/`NeutralDocuments`/codec consumer implied by step 4 (not enumerated — step 4
  was not attempted, see above)

## Scratch files in this ticket folder (not deleted, per instructions)
`w4a-relocated-manifest-vocab.rs`, `w4a-wrapped-insert.rs`, `w4a-test1.rs`, `w4a-test2.rs`,
`w4a-test3.rs`, `w4a-media-vocab-tests-insert.rs` — intermediate extraction/insertion
scratch used to move the block losslessly; kept for audit trail.
