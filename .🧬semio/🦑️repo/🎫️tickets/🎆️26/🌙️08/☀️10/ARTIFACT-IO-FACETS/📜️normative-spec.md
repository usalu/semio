# 📜️ Normative Spec — Artifact Io Facets

Ticket `26/08/10/ARTIFACT-IO-FACETS`. Every agent on this ticket reads **only this document** plus
`🧪owner-table.json` and `📋️fanout-brief.md`. Do not invent shapes.

## 1. What is being added

Every artifact gets a required **`🚪️io`** facet:

```
artifacts/<artifact>/
  io/
    component.rs          ArtifactIo: formats() + register()
    component.ts
    <format-dir>/           e.g. svg, glb, json
      import/{component.rs, component.ts}
      export/{component.rs, component.ts}
```

(Emoji prefixes are required on disk: `🚪️io`, `🎨️svg`, `📥️import`, `📤️export`, etc. — see owner table `format_dirs`.)

Format dirs are grouping folders. Leaf parents are `📥️import` and `📤️export`.

## 2. Closed format catalog

`mimes.csv` is the single source of truth. Trimmed to open/documented formats (26 entries).
`OsMediaFormat` is renamed `MediaFormat` and must stay in parity with the CSV (policy
`artifact-io/catalog-parity`).

| Format | Dir | Neutral model |
| --- | --- | --- |
| glb | glb | MeshData |
| gltf | gltf | MeshData |
| obj | obj | MeshData |
| stl | stl | MeshData |
| ply | ply | MeshData |
| las | las | MeshData |
| step | step | Brep |
| ifc | ifc | Brep |
| dwg | dwg | DwgDrawing |
| dxf | dxf | DwgDrawing |
| svg | svg | DwgDrawing |
| png | png | RasterImage |
| jpg | jpg | RasterImage |
| gif | gif | RasterImage |
| bmp | bmp | RasterImage |
| tiff | tiff | RasterImage |
| pdf | pdf | PageDoc |
| docx | docx | PageDoc |
| pptx | pptx | PageDoc |
| csv | csv | TableDoc |
| xlsx | xlsx | TableDoc |
| md | md | TextDoc |
| txt | txt | TextDoc |
| zip | zip | Archive |
| bcf | bcf | Archive |
| json | json | Value |

Removed from the prior CSV: blend, rvt, 3dm, speckle, doc, xls, ppt, mp4, mp3.
Added: step, ply, las, md.

Exact emoji folder names live in `🧪owner-table.json` → `catalog_formats`.

## 3. Neutral models and codec ownership

Byte codecs live **once** in the framework. Artifact leaves only map Snapshot to Neutral:

```
Snapshot --(artifact leaf)--> Neutral --(framework codec)--> bytes
```

Neutral models:

1. MeshData (existing)
2. DwgDrawing (existing)
3. RasterImage (new)
4. PageDoc (new)
5. TableDoc (new)
6. TextDoc (new)
7. Archive (new)
8. Value / serde_json::Value for json

Each format codec has a round-trip unit test in the framework module that owns it.

## 4. Coverage derivation

```rust
pub fn required_media_formats(media_type: MediaType, direction: MediaDirection) -> Vec<MediaFormat>
```

Keyed on MediaClass x MediaForm (and OsMediaCapability / MediaForm::Brep where relevant).
Authoritative per-artifact lists are in `🧪owner-table.json`. Every artifact also always includes `json`.

Artifacts that currently lack media_type / dimension **must** be filled in (W4): architect program,
all 15 norm artifacts, energy model, trinity rewrite, space home.

## 5. SDK traits

```rust
pub trait ArtifactImport {
    type Snapshot;
    const FORMAT: MediaFormat;
    fn import(bytes: &[u8]) -> Result<Self::Snapshot, IoError>;
}
pub trait ArtifactExport {
    type Snapshot;
    const FORMAT: MediaFormat;
    fn export(snapshot: &Self::Snapshot) -> Result<Vec<u8>, IoError>;
}
pub trait ArtifactIo {
    fn formats() -> &'static [IoFormatSpec];
    fn register();
}
```

`engine::register()` calls `io::register()` and **must not** call `register_2d_export_handlers`,
`register_mesh_*`, `register_solid_*`, or `register_dwg_*` directly (policy `artifact-io/no-engine-io`).

`ArtifactKindSpec.export_formats` / `import_formats` are derived from the facet, not hand-listed.

OS keeps one handler map keyed `"{artifact_kind}:{format}"`.

## 6. Taxonomy twins

Add `🚪️io` to `artifactChildDirs` and `artifactComponentDirs`.

```json
"ioFormatChildDirs": ["📥️import", "📤️export"],
"mediaFormatDirs": { "glb": "🧊️glb", "...": "..." }
```

Add `📥️import` and `📤️export` to `taxonomyLeafParentDirs`.

Keep the four twins in sync:

1. validateTaxonomy (discovery)
2. validateTaxonomyTree (registry)
3. assert_taxonomy_components (Rust)
4. policyTaxonomyDirsBreaches + new policyArtifactIoBreaches

Rule kinds: artifact-io/facet-completeness, format-coverage, leaf-parity, registration,
no-engine-io, catalog-parity, roundtrip-test.

## 7. Glue wiring

Rust glue mounts `artifacts.<name>.io` via `#[path]` to emoji folders, with per-format import/export leaves.

TypeScript barrel re-exports relative emoji paths as today.

## 8. Pilots (verbatim reference)

W5 pilots are note (TwoD Document) and cad (ThreeD Brep). Their leaves are the shape every
W6 agent copies. After W5, quote the note SVG export leaf and the cad STEP import leaf in section 13.

## 9. Gates

Scoped (never full policy):

```bash
bun -e 'const m = await import("./📜️script.ts");
const b = m.policyArtifactIoBreaches(process.cwd()).filter(x => x.scope.includes("PLUGIN_DIR_FRAGMENT"));
console.log(b.length); for (const x of b) console.log(x.kind, "|", x.summary);'
```

Plus cargo check/test on the crate (macOS: DEVELOPER_DIR=/Library/Developer/CommandLineTools).

Final: policyArtifactIoBreaches 0, taxonomy dirs 0, validateTaxonomy 0, media coverage asserts green
for all 54, registry generate pass.

## 10. Waves

| Wave | Agents | Model | Work |
| --- | --- | --- | --- |
| W0 | 1 | Grok 4.5 | ticket + this spec + owner table |
| W1 | 1 | Grok 4.5 | catalog + MediaFormat rename |
| W2 | 7 | Composer 2.5 | neutral models + codecs |
| W3 | 1 | Grok 4.5 | taxonomy + policy |
| W4 | 1 | Grok 4.5 | SDK + OS registry + media_type fills |
| W5 | 2 | Grok 4.5 | note + cad pilots |
| W6 | 32 | Composer 2.5 | per-plugin fan-out |
| W7 | 1 | Composer 2.5 | TS/WASM + UI accept menus |
| W8 | 1 | Grok 4.5 | aggregate gate + ticket_close |

## 13. Pilot leaf reference

Pilots: `🗒️note` (2D vector) and `📐️cad` (3D brep). Both compile with `io::register()` replacing engine-side `register_*` media calls.

### note `🚪️io` root

Declares `Dwg, Dxf, Json, Pdf, Png, Svg` via `IoFormatSpec` and registers each leaf. SVG/DWG leaves call `engine::note_document_to_svg` / `dwg_from_bytes` + `note_document_json_from_dwg`; other formats round-trip through `JsonCodec` against `NoteSnapshot`.

### cad `🚪️io` root

Declares `Dwg, Glb, Gltf, Ifc, Json, Obj, Png, Step, Stl`. Leaves register unified `register_os_media_*_handler` entries keyed `3d.cad:<format>`.

### Shape (verbatim contract for W6)

```
artifact/🚪️io/🦀️component.rs          // ArtifactIo + format_specs + register()
artifact/🚪️io/<format-dir>/📥️import/🦀️component.rs
artifact/🚪️io/<format-dir>/📤️export/🦀️component.rs
```

Glue wires `pub mod io` as a **sibling** of `engine` (never nested inside it). Engine `register()` calls `crate::artifacts::<ascii>::io::register()` only.

