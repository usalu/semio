# stdio g4 — dxf dwg ply stl obj gltf las ifc step

## Converted (8/9): dxf, ply, stl, gltf, las, obj, step, dwg
Each gained `pub fn declaration() -> ArtifactDeclaration` at its artifact root
(`🗿️artifacts/<x>/🦀️component.rs`), covering schema/inferences/composers/languages/document_codec_bare
(+ subset_validators for step). Plugin root: `engine::register()` line removed, `.artifact(declaration())`
added right after each `.artifact_kind(...)`. `.composers(...)` uses the fully-qualified ENGINE shim
(`crate::artifacts::<x>::engine::io_registry::entries()`, owned `&[ComposerEntry]`), never the artifact
root's own shadowing `io_registry` module (`&[&ComposerEntry]`, the SILENT REBIND trap) — verified zero
bare `io_registry::entries()` calls across all 9 files.

**dwg** (2 standards, ac1018+ac1024): `engine::register()` shim is ac1024-only (plain glob re-export);
repo-wide grep proved ac1018's own `register()` free fn has zero callers — dead code, nothing dropped.
Composers ARE both-live (root's own `io_registry::register()` unions them), so I built
`dwg_combined_composer_entries()`: re-materializes both engines' owned `io_registry::entries()` into one
new `&'static [ComposerEntry]` (field-copy, `ComposerEntry` has no `Clone` but every field is `Copy`).

**las, obj, dwg**: `register_schema_specs()` (`dsl::registry::register_schema_spec`) has no
`ArtifactDeclaration` field (same gap `🗜️deflate`'s W6-g2 exemplar documents) — not invented, not
dropped, survives via `.setup(crate::artifacts::<x>::engine::register_schema_specs)` added alongside
each `.artifact(...)`.

## Left imperative (1/9): ifc — genuine structural gap, reported not invented
`ifc`'s two standards are BOTH live (glue.rs shim locally overrides `register()` to call
`v4::engine::register()` AND `v2x3::engine::register()` explicitly — unlike dwg's dead ac1018).
v2x3 registers a SECOND independent `ArtifactSchemaDescriptor` (`"s.stdio.ifc.2x3"` vs v4's
`"s.stdio.ifc"`) and a SECOND independent document codec (`Ifc2x3Snapshot`/`Mutation`, different
document schema string) plus its own languages + 3 subset validators (cv20/sav/cobie).
`ArtifactDeclaration` has exactly one `.schema()`/`.document_codec()` slot (mandatory, non-accumulating)
— no field or combination covers two independent schemas/codecs; converting v4-only would silently
drop v2x3's live registrations (a real regression, not a dead-code no-op like dwg's ac1018). Left
`crate::artifacts::ifc::engine::register()` untouched in the plugin root; documented the gap in both
`ifc::component.rs` and the plugin root's comment. Field that would cover it: an accumulating
multi-schema/multi-codec declaration shape, or two `.artifact()` calls with different kinds (unverified
whether v2x3's Dialect.artifact_kind actually varies — out of this pass's budget to confirm safely).

## Verify
- `grep -rn "fn declaration"` present at all 8 converted artifact roots; absent (by design) at ifc.
- Plugin root: 8 `engine::register()` lines removed, `.artifact(...)` added; `ifc::engine::register()`
  untouched; 3 `.setup(register_schema_specs)` survivors added (las, obj, dwg).
- `io_registry::entries` fully qualified everywhere in my 9 files — zero bare calls (grep-verified).
- `cargo check -p semio-s-plugin-stdio --all-targets`, twice (logs `scratch-stdio-g4-check-1.txt`,
  `-2.txt`): **NOT green** — 1 lib error + 9 lib-test errors, ALL in
  `🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/...` (unresolved imports/functions
  `demo_mesh_snapshot`/`print_mesh_dsl`/`parse_mesh_dsl`/`encode_mesh_pack` missing from
  `standards::v1::engine`). **Proved upstream, not mine**: `🧿️semio` is not one of my 9 artifacts, I
  never opened any file under it; the erroring snapshot file's mtime is `Aug 13 00:59:01`, ~30s before
  my first check ran and identical on retry — active concurrent edit by another session, not stale
  breakage I caused. Zero error lines mention dxf/dwg/ply/stl/obj/gltf/las/ifc/step anywhere in either
  log. Per ticket protocol (retry-and-wait, don't patch upstream/other-artifact files) I did not touch
  `🧿️semio`. Could not obtain a `Finished`/exit-0 line for the whole crate as a result — my own 9
  artifacts' code is implicated in zero of the 9 errors, but I cannot claim a full-crate green.

Files touched: 9 artifact roots (declaration() added to 8; doc-only gap note added to ifc) +
`✏️s/🔌️plugins/🗄️stdio/🦀️component.rs` (plugin root, my 9 artifacts' lines only).
