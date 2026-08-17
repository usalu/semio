# Wave 4a verification (orchestrator — the dispatched verify agent hit a
connection error mid-run and returned nothing, so this replaces it)

## Direct verification performed
- `cargo check -p semio-framework` — **clean** (`Finished`).
- `cargo check -p semio-framework-schema` — **clean** (`Finished`).
- `cargo check -p semio-framework-os` — fails, but on `semio-framework-math`
  (`E0004` non-exhaustive `TokenKind` match) — a subsystem NEITHER agent
  touched, absent from `📸️baseline-cargo-check.txt`, and with no git-status
  changes visible from this ticket. Yet another concurrent session mid-edit
  on the math tokenizer. Not investigated further, not fixed.
- `wc -l`: mesh 4064→3582 (−482), manifest 4278→4777 (+499 — the relocated
  vocabulary, roughly matches), schema 1566→656 (−910, the closed catalog +
  catalog-integration regions + parity test).

## Schema deletion (w4a-schema-deletion) — COMPLETE, verified
`register_all_app_schema_descriptors()` (~665 lines) and both
`catalog-integration`-gated dead regions deleted; feature removed from
Cargo.toml; framework's 39-owner parity test deleted (roster-completeness
now belongs to registry codegen validation, not framework). Open API
(`register_app_schema_descriptor`, `AppSchemaRegistry`,
`validate_registered_app_descriptor`, `AppSchemaDescriptor`, `FacetLeaves`)
confirmed still `pub`. Spot-checked cad/flow/procedural still call their own
`register_app_schema()` correctly — blocked only by the known concurrent
"document" churn, not this change.

## Mesh eviction (w4a-mesh-eviction) — PARTIAL, well-reasoned deferrals
- Step 1 (keep MeshData/Primitives/generic codecs): done, untouched.
- Step 2 (delete `MediaFormat`/`STDIO_FORMAT_CATALOG`): **deferred** —
  correctly found `MediaFormat` is a hard dependency of the step-1-protected
  `MeshExporter`/`MeshImporter` trait signatures, with a 58-file real-value
  fan-out (not just imports) and non-mechanical `&'static str`→`String`
  signature ripples. Also found one supposed consumer
  (`💻️os/🦀️component.rs`, the "os core" file) is orphaned — not mounted
  into any crate right now, consistent with prior waves' findings about
  concurrent churn in that exact file. Good catch, not a regression to force.
- Step 3 (relocate manifest-vocabulary types): **done and verified** — ~22
  types (`ArtifactKindSpec`, `MediaClass`/`MediaForm`/`MediaType`, `AppIo`,
  `ConfigSpec`, `CommandGrammar`, `Media`/`MediaPayload`, etc.) moved from
  mesh into `🛂️manifest`, with fallout fixed in kernel, framework glue's
  re-export list, and a `pub use semio_framework::*`-obscured import in
  `🔌️plugin/🦀️component.rs` that only `cargo check` (not grep) caught.
  Verified with both plain and `--tests` checks.
- Step 4 (delete DWG/codec regions): **deferred** — found real external
  consumers of mesh's `ArtifactCodec`/`JsonCodec` reachable only through a
  glob re-export (`pub use semio_framework::*` in the plugin crate),
  invisible to a `mesh::X`-shaped grep; `DwgDrawing` alone has 21 external
  references. Judged full verification would require compiling dozens of
  downstream plugin crates individually — correctly deferred rather than
  risk an unverifiable deletion mid-wave.

## Disposition
No regressions from either agent's actual changes. Two genuinely hard sub-
steps (MediaFormat opening, DWG/codec deletion) remain — both correctly
identified as needing a dedicated, carefully-scoped follow-up pass rather
than being rushed. Recording as a follow-up task rather than re-attempting
immediately, to keep this wave's verified, compiling state intact.
