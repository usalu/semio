# Normative Spec — Stdio Artifacts and Io

Ticket: `26/08/10/STDIO-ARTIFACTS-AND-IO`
Goal: `AI-OPTIMIZED-REPO`
Due: `2026-08-17`

Exact emoji tokens: see `🧪tokens.json`.

## 1. Vocabulary (taxonomy.json SSOT)

### Completeness (`artifactComponentDirs`)

- `🧬️schema`
- `⚙️engine`
- `🚪️io`
- `🏗️builder`
- `🪓️decomposer`

REMOVED from root: `🗣️dsl`, `🔧️op`, `📡️spr`, `🔺️diff`, `📸️snapshot`, root-level `🧬️mutations`.

Structural (`artifactChildDirs`) = completeness + `📚️examples`.

### Schema tree

```
🧬️schema/
  # five schemaFormats leaves (artifact-level) — unchanged
  📸️snapshot/
    📝️text/      # was 🗣️dsl
    💾️binary/    # was 📸️snapshot/🎒️pack
    # five schemaFormats leaves (was 📸️snapshot/🧬️schema)
  🔺️diff/
    📝️text/      # was 🔺️diff grammar+rs+ts
    💾️binary/    # NEW
    # five schemaFormats leaves (was 🔺️diff/🧬️schema)
  🧬️mutations/
    📝️text/      # was 🔧️op
    💾️binary/    # was 📡️spr
    <mutation>/{🦠️mutation,🔺️diff,↩️inverse}/
```

### Taxonomy keys

- `schemaChildDirs`: `📸️snapshot`, `🔺️diff`, `🧬️mutations`
- `representationDirs`: `📝️text`, `💾️binary`
- `mutationChildDirs`: `🦠️mutation`, `🔺️diff`, `↩️inverse` (unchanged)
- `ioDirectionDirs`: `📥️import`, `📤️export`
- `ioDirectionChildDirs`: `📥️import`→`🧩️deserializers`, `📤️export`→`🧵️serializers`
- After deserializers/serializers: mandatory `🗿️artifacts/<stdio-artifact>/` with rs+ts leaves

Deleted keys: `mediaFormatDirs`, `ioFormatChildDirs`, `snapshotChildDirs`, `diffChildDirs`.

### Text spec leaves (8)

`📖️component.grammar.semio`, `🔤️component.ebnf`, `🅰️component.g4`,
`🔗️component.graphql`, `🔣️component.json`, `🛰️component.proto`,
`🦀️component.rs`, `🟦️component.ts`

### Binary spec leaves (6)

`📡️component.protocol.semio`, `🔠️component.abnf`, `🥋️component.ksy`,
`🌶️component.spicy`, `🦀️component.rs`, `🟦️component.ts`

(Exact filenames with VS16: see `🧪tokens.json` — `ebnf`, `g4`, `abnf`, `ksy`, `spicy`.)

### IO shape

```
🚪️io/
  🦀️component.rs
  🟦️component.ts
  📥️import/
    🧩️deserializers/
      🗿️artifacts/<stdio-artifact>/{🦀️component.rs,🟦️component.ts}
  📤️export/
    🧵️serializers/
      🗿️artifacts/<stdio-artifact>/{🦀️component.rs,🟦️component.ts}
```

### Wildcard walker

`artifactFacetPathIsDeclared` must support `*` = any emoji-prefixed slug:

1. `🧬️schema` → `schemaChildDirs` → (`representationDirs` OR mutation slug → `mutationChildDirs`) → leaf parents
2. `🚪️io` → `ioDirectionDirs` → deserializers|serializers → `🗿️artifacts` → `*` → leaf

`taxonomyLeafParentDirs` must include: `📝️text`, `💾️binary`, `🧩️deserializers`, `🧵️serializers`, `🏗️builder`, `🪓️decomposer`, and keep `🧬️schema`, `🚪️io`, `⚙️engine`, mutation triad dirs.

### Example asset prefixes

Re-key `exampleAssetKindPrefixes`:

- `dsl` → snapshot text
- `pack` → snapshot binary
- `op` → mutations text
- `spr` → mutations binary
- `diff` → diff text

## 2. Stdio plugin

Path: `✏️s/🔌️plugins/🗄️stdio/`

Zero apps. Pattern: `Plugin::builder("stdio").label("Stdio").version("0.1.0").artifact_kind(...).library()`.

29 artifacts — roster in `🧪owner-table.json` → `stdio_roster`.
DAG edges in `stdio_dag_edges`. Every path terminates at `binary`.
Acyclicity enforced by `policyIoTerminalityBreaches`.

Crate name: `semio-s-plugin-stdio`. Workspace member required.
No `🎛️apps/` at plugin root. Stub `🔌️plugin/🎛️apps/🦀️component.rs` required.

## 3. Contracts (plugin SDK)

```rust
pub trait ArtifactBuilder: Sized {
    type Snapshot; type Mutation; type Diff;
    fn empty() -> Self;
    fn from_snapshot(snapshot: Self::Snapshot) -> Self;
    fn from_text(text: &str) -> Result<Self, TextError>;
    fn from_binary(bytes: &[u8]) -> Result<Self, PackError>;
    fn mutate(self, mutation: Self::Mutation) -> Self;
    fn absorb(self, diff: Self::Diff) -> Self;
    fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>>;
}

pub trait ArtifactDecomposer: Sized {
    type Snapshot; type Parts;
    fn decompose(sources: &[DecomposeSource]) -> Decomposition<Self::Parts>;
}

pub struct Decomposition<T> {
    pub parts: T,
    pub confidence: Confidence,
    pub diagnostics: Vec<Diagnostic>,
}
```

Delegate to `DocumentDsl` / `DocumentPack` / `Mutation` / `MutationDiff` / `ArtifactEngine`.
Soft findings use `Diagnostic`+`Severity`, never `Fault` for partial success.
Re-export from every plugin Rust crate and TS barrel.

`PluginBuilder::artifact_kind` moves kinds onto `PluginManifest` (required for zero-app plugins).

## 4. Framework deletions

Delete: `MediaFormat`, all stub `*Codec`, `IoFormatSpec`, `ArtifactIo`,
`ArtifactImport`, `ArtifactExport`, `required_media_formats`,
`assert_os_media_*_coverage`, `register_2d_export_handlers`,
`register_mesh_*`, `register_solid_*`, `register_dwg_import_handler`,
SRAS / IFCCARTOONMESH / "minimal" stubs.

Collapse os/host twin media registry → single module keyed
`(artifact_kind, format_artifact_kind)`.

`ArtifactKindSpec.export_formats` / `import_formats` → `Vec<&'static str>`
of stdio kind ids, derived from the io facet.

## 5. Old → new path map (all 54 domain artifacts)

| Old | New |
|---|---|
| `🗣️dsl/` | `🧬️schema/📸️snapshot/📝️text/` |
| `📸️snapshot/🎒️pack/` | `🧬️schema/📸️snapshot/💾️binary/` |
| `📸️snapshot/🧬️schema/` | `🧬️schema/📸️snapshot/` (5 leaves) |
| `🔺️diff/` (grammar+rs+ts) | `🧬️schema/🔺️diff/📝️text/` |
| `🔺️diff/🧬️schema/` | `🧬️schema/🔺️diff/` (5 leaves) |
| (new) | `🧬️schema/🔺️diff/💾️binary/` |
| `🔧️op/` | `🧬️schema/🧬️mutations/📝️text/` |
| `📡️spr/` | `🧬️schema/🧬️mutations/💾️binary/` |
| `🧬️mutations/<m>/…` | `🧬️schema/🧬️mutations/<m>/…` |
| `🚪️io/<format>/{import,export}/` | `🚪️io/{import/deserializers,export/serializers}/artifacts/<stdio>/` |
| (new) | `🏗️builder/`, `🪓️decomposer/` |

## 6. Curated IO matrix

See `🧪owner-table.json`. 54 domain artifacts × curated stdio targets (285 pairs).
Each pair gets both import deserializer and export serializer leaves.

## 7. Policy rules

### New

- `policyStdioCatalogBreaches` — 29 stdio artifacts exist with required facets
- `policyArtifactBuilderBreaches` — every artifact has `🏗️builder` with rs+ts
- `policyArtifactDecomposerBreaches` — every artifact has `🪓️decomposer` with rs+ts
- `policySchemaRepresentationBreaches` — schema tree + all text/binary spec leaves
- `policyIoSerializerMatrixBreaches` — curated matrix leaf parity
- `policyIoTerminalityBreaches` — stdio DAG acyclic, every path → binary
- `policyCodecFidelityBreaches` — no SRAS / IFCCARTOONMESH / stub markers

### Deleted

- `policyMediaFormatCatalogBreaches`
- `policyArtifactIoFacetCompletenessBreaches`
- `policyArtifactIoLeafParityBreaches`
- `policyArtifactIoNoEngineIoBreaches`
- `policyArtifactSchemaPackRelocationBreaches`

## 8. Gates

- `bun ./📜️script.ts policy` — 0 breaches
- `bun nx run @semio-tech/plugin-registry:check` and `:generate`
- `cargo test -p semio-s-plugin-stdio`
- `cargo check` per touched plugin crate
- launch.json entries for the seven new gates

## 9. Concurrency ownership

- W1 / W3 / W7 only: `taxonomy.json`, root `📜️script.ts`, root `Cargo.toml`, `launch.seed.jsonc`, framework `os` / `os/host`
- W4 / W6: own plugin subtree + its `📦️glue.rs` only
- All temps/logs/scripts live in this ticket folder and are kept
