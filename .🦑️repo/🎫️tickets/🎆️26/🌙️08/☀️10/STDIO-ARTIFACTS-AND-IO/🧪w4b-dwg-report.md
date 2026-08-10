# W4b Report — Stdio DWG Leaf

## Scope

Final missing stdio roster leaf: `🖊️dwg` under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg`.

Facet tree cloned from `🖼️bmp` (binary-shaped, 82 files). Generator: `generators/w4b_scaffold_dwg.py`.

## Codec (`stdio.dwg`)

Lossless DWG container with validated **AC10xx** file header sentinel:

| Field | Role |
|-------|------|
| `schema` | `stdio.dwg` envelope id |
| `version` | Six-byte ASCII sentinel (e.g. `AC1018`) |
| `bytes` | Full raw DWG file octets |
| `section_names` | Best-effort: known `AcDb:*` substring hits + optional R2004+ header table at `0x80` |

`decode_dwg` / `encode_dwg` in `🧬️schema/📸️snapshot/🦀️component.rs`:

- Rejects buffers without `AC` + four version digits (`AC10xx` family).
- `DocumentDsl`: hex body → raw DWG → snapshot.
- `DocumentPack`: `wrap_binary` / `unwrap_binary` around raw DWG payload.
- `DwgMutation`: `OpBinary` via JSON serde (same as bmp/png siblings).
- `DwgDecomposer`: `DecomposeSource::Text` + `DecomposeSource::Binary` (semio pack).

Pre-R13 DWG and encrypted section bodies are **not** parsed; round-trip is byte-identical for validated AC10xx headers.

## Wiring

- `📦️glue.rs`: `artifacts::dwg` with `#[path = "."]` (inserted after `bmp`).
- `🔌️plugin/🦀️component.rs`: `dwg::engine::register()` + `artifact_kind()`.
- `📦️packages/🟦️typescript/📦️index.ts`: `export * as dwg`.

## Verification

```text
cargo check -p semio-s-plugin-stdio
→ Finished `dev` profile (green)
```

## Examples

- `📚️examples/🎬️demo/🖼️assets/example.dwg` — minimal `AC1018` stub (16 bytes).
- `🗣️example.dsl.semio` — hex of same stub for DSL demo.
