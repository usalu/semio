# Wave 0 R1-E Norm Declarations

## Result

All fifteen `📕️norm` semantic artifact declaration factories now start from their one authoritative, leaf-owned `ArtifactDefinition`. Each leaf supplies immutable literal capability rows for its `v1`/`any` taxonomy, schema, inference, composer dialect, five grammars, document codec, and explicit English and German localized descriptors. The shared artifacts component only parses, validates, and assembles those supplied rows; it does not inspect or derive capability identities or claims from runtime descriptor tables.

Each declaration consumes its definition, binds the pre-existing schema/inference/composer/language runtime data plus its typed `ArtifactApp` document codec, and terminates with `try_build()`. The norm plugin root maps definition failures to `PluginAssemblyError` before adding the fifteen typed declarations to `Plugin::builder`.

## Changed Paths

- `✏️s/🔌️plugins/📕️norm/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🦀️component.rs` through `📘️en1999/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/🦀️component.rs`

## Verification

- `rustfmt --edition 2021` completed for every changed Rust source file.
- Static counts after formatting:
  - raw `ArtifactDeclaration::builder("…")` calls: `0`;
  - leaf `definition()` factories: `15`;
  - fallible `declaration(definition)` factories: `15`;
  - typed `.document_codec::<ArtifactApp>()` bindings: `15`;
  - explicit EN/DE localization capability identities: `30`;
  - root definition/declaration bindings: `15`.
- No Cargo command ran. R1-C/R1-D concurrency and R1-G framework remediation still reserve the integration build lane.

## Gaps And Handoff

- Rust compilation and runtime registration remain unverified until the coordinator authorizes a serial Cargo gate after R1-G lands.
- The mutable owner-mutation roster is registered by the existing typed `document_app::<ArtifactApp>()` path. It is intentionally not duplicated as a declaration callback or synthetic capability table.
- Existing historical documentation mentions the removed `.setup()` fan-out only as migration context; no executable norm `.setup(...)` call remains.
