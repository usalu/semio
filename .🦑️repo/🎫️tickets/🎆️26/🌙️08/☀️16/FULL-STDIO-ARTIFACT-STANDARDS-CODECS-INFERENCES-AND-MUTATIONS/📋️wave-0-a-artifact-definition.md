# Wave 0-A Artifact Definition Contract

## Scope

Lease W0-A changed only:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- this report

No GLTF, stdio catalog, IO, store, scripts, launch configuration, AGENTS instructions, Git state, or worktree was changed.

## Delivered Contract

`ArtifactDefinition` is an owned, schema-first declaration for an artifact family or a specific standard/profile. `ArtifactIdentity` gives canonical dot-delimited hierarchy and `ArtifactCapability` supports an open-ended `ArtifactCapabilityKind`, so adding a capability category never requires a central dispatch edit.

The supplied category constructors cover schemas, standards, profiles, source dialects, representations, codecs, mutations, inferences, resources, localizations, conformance suites, and extensions. Capabilities may claim open-ended identity namespaces; supplied namespaces include artifact, schema, dialect, codec, MIME, file extension, and extension implementation.

`ArtifactDefinitionRegistry` uses deterministic ordered maps and rejects duplicate artifact identities and external claims atomically. It never replaces a prior entry. Tests cover schema, dialect, codec, MIME, and extension conflicts. Locale is represented by `ArtifactLocale`, has no `Default` implementation, and is required by `ArtifactLocalization`; duplicate locales fail validation.

`ArtifactDeclaration` now carries plural definitions and exposes `definitions()` plus the cross-declaration preflight seam `register_definitions(&mut ArtifactDefinitionRegistry)`. Its runtime-facing schemas, document codecs, composers, subset validators, and grammar specifications are append-only: `.schemas(...)`, repeated codec calls, `.composers(...)`, `.subset_validators(...)`, and `.languages(...)` all retain prior entries instead of overwriting. Attached definitions are validated before this declaration writes its legacy runtime registries.

## Integration Handoff

The existing `PluginBuilder` loop is in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs`, outside W0-A’s write lease. It currently consumes and registers one declaration at a time, so it cannot reject a schema/dialect/codec/MIME/extension conflict split across two declarations before the first legacy registration runs.

The clean seam is already public to the crate: create one `ArtifactDefinitionRegistry`, call `declaration.register_definitions(&mut registry)` for every declaration in a read-only preflight pass, then perform the existing consuming registration pass only if the preflight succeeds. This is data-driven and needs no central capability dispatch. The builder owner/coordinator should make that isolated integration edit at the barrier.

## Remaining Contract Gaps

### Cross-declaration atomicity — integration required

`ArtifactDeclaration::register_all` currently creates a fresh registry for its own definitions and converts a validation failure into a panic. It therefore detects conflicts only inside one declaration. It cannot atomically reject a duplicate split between two `PluginBuilder.artifact(...)` entries.

`PluginBuilder::build` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` also invokes its legacy `setup` callbacks before it iterates declarations. A setup callback can mutate an old global registry before cross-declaration validation occurs. Both facts are outside W0-A's lease.

Required builder-owner change:

1. Add a no-side-effect preflight over `self.artifacts.iter()` before every setup or runtime registration, with one `ArtifactDefinitionRegistry` shared for the whole plugin.
2. Change the runtime path to a typed result, for example `try_build(self) -> Result<Plugin, PluginAssemblyError>` and `try_register_all(...) -> Result<Plugin, PluginAssemblyError>`. `PluginAssemblyError` must remain codebase-owned, with owned `code: String` and `message: String`, and map `ArtifactDefinitionError` and IO conflicts without exposing third-party types.
3. Retire or data-model remaining imperative setup registrations so plugin construction has no side effects prior to a successful preflight. Until that wave lands, `build()` cannot promise an atomic plugin-wide registration transaction.

### Legacy singular entry point — intentionally retained, not a singular field

There is no remaining single `schema: Option<_>` or `document_codec: Option<_>` field in W0-A: both are plural vectors. `.schema(...)` remains the typestate-required *first* schema call so a declaration cannot be schema-less; `.schemas(...)` appends every further version/profile descriptor. `DocumentCodecSpec` models one codec implementation, while `ArtifactDeclaration.document_codecs` is plural and both codec builder methods append.

The old global schema/format/language/codec registries are still owned outside W0-A and may have their own overwrite semantics. W0-A prevents duplicate claims in declarative preflight; replacing those global registry contracts remains the W0-B/coordinator integration concern.

## Verification

Final focused gate passed:

```text
cargo test --manifest-path '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml' --lib artifact_definition_contract_tests
```

Result: 3 passed, 0 failed, 171 filtered. The test set verifies plural open capability assembly, deterministic identity ordering, explicit locale rejection, duplicate-locale rejection, and atomic schema/dialect/codec/MIME/extension collision rejection.

The last invocation waited briefly for the shared Cargo build-directory lock, then completed successfully in 16.03 seconds. No broad Nx/Cargo workspace gate was started.

`cargo fmt --check` was also run. It reports pre-existing/concurrent formatting differences outside this lane; no formatter write was performed, preserving concurrent edits.

One intermediate repeat was temporarily blocked upstream by in-flight W0-B edits in `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`:

- `ArtifactDialect` lacks `Ord` while a concurrent registry uses `BTreeMap` (`E0277`, around lines 799, 803, and 839).
- An iterator chain combines `&&FormatDescriptor` and `&FormatDescriptor` (`E0271`, around line 1141).

No W0-A file was modified to mask or repair those out-of-lease failures. W0-B subsequently resolved them and the final focused gate above passed.
