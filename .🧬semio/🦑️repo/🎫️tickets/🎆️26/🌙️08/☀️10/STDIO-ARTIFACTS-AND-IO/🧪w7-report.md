# W7 Report — Stdio Artifacts and Io

Ticket: `26/08/10/STDIO-ARTIFACTS-AND-IO`

## Verdict

W7 complete: Space Studio + os/host media paths resolve via stdio format kind string keys; canonical `mimes.csv` derived from the 29-entry stdio catalog; UI duplicate deleted; cargo green on touched crates.

## Before / After Paths

| Role | Before | After |
|---|---|---|
| Canonical mimes | framework assets `mimes.csv` (26 rows, no Kind) | `🧰️framework/🔨️modules/🖼️assets/📃️list/📋️mimes.csv` (**29 rows**) |
| Stale UI duplicate | framework UI assets `mimes.csv` (legacy `representation/*`) | **deleted** |
| Ticket derived copy | — | `generators/w7-mimes.csv` |

Exact disk paths:

- Canonical (exists=True): `🧰️framework/🔨️modules/🖼️assets/📃️list/📋️mimes.csv`
- Deleted UI duplicate (exists=False): `🧰️framework/🔨️modules/🖱️ui/🖼️assets/📃️list/📋️mimes.csv`
- Header: `MIME,Extension,Name,FullName,Neutral,Dir,Kind`
- Rows: 29
- SHA-256: `a423c3451ea092d393fa7fbbeb42f624aeb227bd142853e35da7ff209286ebb0`

### Deleted duplicate proof

- `Path.exists(🧰️framework/🔨️modules/🖱️ui/🖼️assets/📃️list/📋️mimes.csv)` => **False**
- Framework `*mimes.csv` hits excluding ticket/node_modules/target/dist/fixture: `['🧰️framework/🔨️modules/🖼️assets/📃️list/📋️mimes.csv']`
- Machine proof JSON: `generators/w7-final-proof.json`

## Stdio catalog SSOT

- Rust SSOT: `STDIO_FORMAT_CATALOG` in framework mesh (`StdioFormatCatalog` region)
- Helpers: `normalize_stdio_format_kind`, `stdio_format_entry`, `stdio_format_kind_id`, `stdio_accept_filter`, `stdio_mimes_csv`
- Re-exported from `semio-framework` package glue

## Media rewire (artifact-kind + stdio format kinds)

### os / os-host

- `media_accept_filter_kinds([&str])` → `stdio_accept_filter`
- `export_os_app_instance_media_kind` / `import_os_app_instance_media_kind`
- `OsMediaExportResult::from_format_kind_bytes`
- `OsArtifactDescriptor.export_stdio_kinds` / `import_stdio_kinds` populated from `ArtifactKindSpec`
- `negotiate_wire_format` prefers intersecting stdio kind ids, then legacy `MediaFormat` lists
- Thin `MediaFormat` adapters retained for mid-migration plugins

### Space Studio media commands

- Space app `export-media` / `import-media` / `import-media-payload` rewired to kind-string APIs
- Uses `normalize_stdio_format_kind` + `media_accept_filter_kinds`
- Accepts `dwg` and `stdio.dwg`; file-picker accept resolves to `.dwg`
- Host effects: `DownloadMediaExport` / `RequestFileOpen` unchanged in shape; accept filter now stdio-derived

### WASM bridge

- `mediaAcceptFilterKinds` / `normalizeStdioFormatKind` wasm exports on os + host twins

### TS barrels

- OS TS component: `normalizeStdioFormatKind`, `mediaAcceptFilterKinds`
- Generated manifest TS: `exportStdioKinds` / `importStdioKinds` on `ArtifactKindSpec`

## Cargo proof

| Command | Result | Log |
|---|---|---|
| `cargo check -p semio-framework` | OK | `generators/w7-cargo-check.log` |
| `cargo check -p semio-framework-plugin` | OK | `generators/w7-cargo-check.log` |
| `cargo check -p semio-framework-os` | OK | `generators/w7-cargo-recheck.log` |
| `cargo check -p semio-framework-os --features os-host-full` | OK | `generators/w7-cargo-recheck.log` |
| `cargo check -p semio-s-plugin-space` | OK | `generators/w7-space-check.log` |

## Ownership notes

- Touched W7 surfaces only: framework mesh catalog, os/host twins, UI assets (delete duplicate), Space Studio media commands (app commands, not artifact facets), TS barrels/manifest types.
- No plugin artifact facet churn.
- Temps/logs kept under this ticket folder.
