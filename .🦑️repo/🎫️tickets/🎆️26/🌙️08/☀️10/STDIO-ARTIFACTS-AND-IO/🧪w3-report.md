# W3 Report — Stdio Artifacts and Io

Ticket: `26/08/10/STDIO-ARTIFACTS-AND-IO`

## Landed

### A. Plugin SDK (`semio-framework-plugin`)

- `ArtifactBuilder`, `ArtifactDecomposer`, `Decomposition<T>`, `Confidence::{High,Medium,Low}`, `DecomposeSource<'a>::{Text,Binary}`
- `PluginBuilder::artifact_kind` → `PluginManifest.artifact_kinds` (works with `.library()`)
- `Plugin::artifact_kind` on the materialized plugin
- `ArtifactKindSpec.export_stdio_kinds` / `import_stdio_kinds: Vec<&'static str>` (additive; `MediaFormat` fields retained)
- Public re-exports from the plugin crate

### B. Registry collapse (os + os/host twins) — confirmed

Both `os/component.rs` and `os/host/component.rs` share the same string-keyed shape:

- `os_media_handler_key(artifact_kind, format_artifact_kind: &str)`
- `register_os_media_export_handler_kind` / `register_os_media_import_handler_kind`
- Thin `MediaFormat` adapters via `MediaFormat::as_str()` (`os_media_export_key` + existing register APIs) so the 54 plugins keep compiling
- `register_artifact_descriptors` also registers `manifest.artifact_kinds`
- Region: `MediaRegistry`

### C. Stub markers removed

- `SRAS` / `sras_codec` → `SMRI` / `raw_rgba_codec`
- `IFCCARTOONMESH` → `IFCPROPERTYSINGLEVALUE('SemioMeshJson', …)`
- `MediaFormat` kept with deprecation comment

### D. Stdio wiring

- `Plugin::builder("stdio")….artifact_kind(...)` for binary/txt/json/xml/csv/md
- Artifact builders/decomposers use SDK traits (local contracts removed)

## Deferred (intentional)

| Item | Why | When |
|---|---|---|
| Full `MediaFormat` enum deletion | Would break 54 domain plugins mid-flight | After W6 |
| Real png/jpg/gif/tiff/ifc codecs | Owned by stdio format crates | W4 |
| Taxonomy / policy / launch.json | W1/W7 ownership | W1/W7 |

## Prove

| Command | Result | Log |
|---|---|---|
| `cargo check -p semio-framework-plugin` | Finished OK | `w3-cargo-plugin.log` |
| `cargo check -p semio-s-plugin-stdio` | Finished OK | `w3-cargo-stdio.log` |
