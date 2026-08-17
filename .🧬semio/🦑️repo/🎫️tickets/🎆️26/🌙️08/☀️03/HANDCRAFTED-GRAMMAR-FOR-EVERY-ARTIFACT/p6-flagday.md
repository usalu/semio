# P6 Flag Day — Generic Codec Derive Removal

Ticket: `HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT` (2026/08/03)

## Verdict

P6 flag day complete for the generic codec derive ban:

- `#[derive(dsl::DslDocument)]` / `#[derive(dsl::DslOps)]` no longer emit `DocumentDsl` / `DocumentPack` / `OpText` / `OpBinary`.
- `dsl::__rt` codec wrappers (`parse_document_record` / `print_*` / `parse_inline_record`) and the old `op_rt` derive path are gone.
- `POLICY_GENERIC_CODEC_DERIVE_EXEMPTIONS` is empty (`new Set([])`).
- All 93 formerly exempt artifact files: derives swapped to `DslRecord` / `DslEnum`, with handcrafted trait impls.

## Derive surface (kept)

| Derive | Emits after P6 |
| --- | --- |
| `DslRecord` | `__dsl_spec` / `__dsl_to_record` / `__dsl_from_record` + `DslField` only |
| `DslDocument` | Same field helpers + envelope constants + `DslField` (no codec traits). Prefer `DslRecord` in plugins. |
| `DslEnum` | `DslVariants` only |
| `DslOps` | `DslVariants` only (same as `DslEnum`; no OpText/OpBinary) |
| `DslDiff` / `DslScalar` | unchanged |

`DslRecord` stays: it only emits field helpers, never the four codec traits.

## Runtime

- `__rt` retains `field_error`, `unit_for_derive`, newtype variant helpers for derive bodies.
- New `dsl::variants_binary::{encode_op,decode_op}` for handcrafted `OpBinary` (explicit call sites only).
- Document codecs use public `dsl::parse` / `dsl::print` + `store::pack_rt` + semio envelopes.

## Artifact migration (93 paths)

For each former exemption path:

1. `dsl::DslDocument` → `dsl::DslRecord`
2. `dsl::DslOps` → `dsl::DslEnum`
3. Inject `//#region 🔖️HandcraftedDocumentCodecs` and/or `//#region 🔖️HandcraftedOpCodecs` with thin `store::DocumentDsl` / `DocumentPack` / `protocol::OpText` / `OpBinary` impls.

Tooling in this ticket folder: `🔧️p6-handcraft-all.mjs`, `🧪p6-handcraft-log.json`, `🧪p6-exempt-paths.json`.

## Policy

- `POLICY_GENERIC_CODEC_DERIVE_EXEMPTIONS = new Set([])`
- Other P6 allowlists already empty (wiring / empty / distinctness / generic / declared-use)
- Gate: `bun ./📜️script.ts policy` — `handcrafted-grammar/generic-codec-derive` must be green

## Notes

- Facet `parse_dsl` / `encode_op` free functions still forward through the traits; traits now own the RecordSpec-backed transitional codecs. Domain-true codecs against `.grammar.semio` / `.protocol.semio` remain follow-on work.
- App `🎚️config` crates outside `🗿️artifacts/` are outside this policy scanner; migrate similarly if they still derive `DslDocument`/`DslOps` for helpers only.
