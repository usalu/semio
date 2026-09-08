# Registry And Contract Implementation

## Ownership Boundary

The shared Rust contract is declared as `semio-s-artifact-stdio-contract` at `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/Cargo.toml`. Its library path is `../../🦀️.rs`, so implementation remains in the package-neutral taxonomy at `📇️registry/🧬️contract/🦀️.rs`.

The contract owns local artifact-definition parsing and validation, definition construction, artifact-local format derivation, executable identity binding, native codec factory and receipt verification, capability ledger derivation, Base64 encoding, semantic fingerprints, and the mutation wire-codec macro. It depends only on first-party framework/kernel crates at runtime. The third-party `base64` crate is test-only.

Each artifact contributes function pointers and immutable local schema through `ArtifactContribution`. Runtime executable identity cannot be inferred from schema text, so `definition_from_schema_with_executables` requires exact artifact-owned `ArtifactExecutable` values. `native_codec_executables` derives the codec subset from artifact-owned factories; glTF additionally supplies inference-service and mutation function identities.

The component registry now consumes selected package contributions. It contains no `crate::artifacts` references and no artifact-schema `include_str!`. Its remaining responsibilities are the 36-artifact/full or eight-artifact/home selection, cross-artifact identity/claim/dependency validation, assembly, native codec aggregation, and the existing signed catalog projection checks.

## Verification

The repository target directory was already locked by unrelated Cargo work. The coordinator directed the fleet to use the single ticket target `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES/🗑️generated/cargo`; no shared process or cache was stopped.

`CARGO_TARGET_DIR='<ticket>/🗑️generated/cargo' cargo check -p semio-s-artifact-stdio-contract` exited 0. The final warm check completed in 1.63 seconds with no warning output.

`CARGO_TARGET_DIR='<ticket>/🗑️generated/cargo' cargo test -p semio-s-artifact-stdio-contract --lib` exited 0. The test result was `1 passed; 0 failed`; the contract's padded RFC 4648 output matched the third-party `base64` implementation across empty, remainder-one, remainder-two, ordinary ASCII, and non-ASCII byte cases.

`cargo tree -p semio-s-artifact-stdio-contract --depth 1` showed only `semio-framework-hash`, `semio-framework-os-kernel`, `semio-framework-pack`, `semio-framework-plugin`, and `semio-framework-value-derive` as runtime dependencies. No stdio component or artifact package occurs at depth one.

A first independent `cargo check -p semio-s-artifact-stdio-binary` reached the new package boundary and failed on nine unresolved `schema::ArtifactSchemaDescriptor`/`ArtifactInferenceDescriptor` references. The extracted crate has a root `schema` module, so its former component-facade extern alias cannot keep that name. Artifact execution was given the exact correction: retain the `framework_schema` alias and mechanically update framework descriptor references in affected leaves. This is an extraction integration failure, not a shared-contract failure; final selective artifact and component results will be recorded after that owner lands the fix and dependency features.
