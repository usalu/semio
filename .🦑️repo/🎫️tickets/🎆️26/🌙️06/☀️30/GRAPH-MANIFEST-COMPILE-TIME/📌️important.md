# Compile-Time Graph Manifest Refactor

Unified `manifest/v1` kernel at `mathematical/graph/manifest` with codegen to Rust enums + TypeScript literal unions.

## Manifests

- `trinity/manifest/nakagin.manifest.json` — Trinity + Puzzle Nakagin (properties + visual catalogs)
- `puzzle/2d/manifest/default.manifest.json`, `puzzle/3d/manifest/default.manifest.json`
- `flow/manifest/dag.manifest.json`, `draw/manifest/layers.manifest.json`
- `writer/manifest/languages.manifest.json`, `framework/product/platform/manifest/builtin.manifest.json`
- `reasoning/mindmap/wires/manifest/wires.manifest.json`

## Validation

- Rust: `ManifestValidator`, `BoardHost::validate_against_manifest_id`, Flow DAG fixture load
- Jack: unknown node/edge kinds are errors
- TS: draw/writer/platform document + plugin validation

## Runtime check

`bun .repo/🎫️/26/06/30/GRAPH-MANIFEST-COMPILE-TIME/manifest-check.mjs`
