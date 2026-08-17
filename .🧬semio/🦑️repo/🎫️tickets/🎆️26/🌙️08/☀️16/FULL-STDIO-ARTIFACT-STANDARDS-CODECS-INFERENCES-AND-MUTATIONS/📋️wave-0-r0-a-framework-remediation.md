# Wave 0 R0-A Framework Remediation

## Scope

Framework plugin declaration, transactional assembly, native/WIT inference transport, schema/language registry preflight seams, plugin macro runtime transport, and the non-stdio plugin-root startup signature migration for `FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS`.

## Changed Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️world.wit`
- `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs`
- 32 non-stdio plugin roots: `🔋️energy`, `🌿️vcs`, `🌊️flow`, `📏️layout`, `🖍️draw`, `➗️mathematical`, `💠️lowpoly`, `🖨️raster`, `🏗️fem`, `📖️playbook`, `🎥️shooting`, `📸️remodel`, `🪐️space`, `🧱️block`, `🌀️procedural`, `🏛️architect`, `🧩️puzzle`, `🎞️animate`, `🔱️trinity`, `📋️forms`, `📕️norm`, `✒️writer`, `🎪️demonstrator`, `🎬️sequence`, `🗒️note`, `📐️cad`, `💡️reasoning`, `🏭️process`, `🪵️sourcing`, `🕸️dag`, `📜️imperative`, and `🌍️gis`.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🦀️component.rs`

## Decisions

- `ArtifactDefinition` is the only artifact authority. `ArtifactDeclaration::builder(definition)` derives its kind from that definition; no raw-kind or post-build definition pairing remains.
- `ArtifactDeclarationBuilder::try_build() -> Result<ArtifactDeclaration, ArtifactDefinitionError>` replaces its infallible terminal. `PluginBuilder::{try_build,try_library}` return `Result<Plugin, PluginAssemblyError>`; public `build`, `library`, `setup`, and both builder and post-build `register_document_app` APIs are removed.
- Canonical stdio constructors require strict lower-case segmented singular identities. A representation takes `Option<ArtifactMime>`; MIME is not fabricated when absent. Codec identity remains `s.stdio.<artifact>.standard.<revision>.codec.<codec>.vN`.
- `PluginRegistration` replaces arbitrary startup callbacks with stable `s.<plugin>.registration.<name>` identities, descriptor bytes, preflight function identity, commit function identity, deterministic order, idempotence only for complete identity equality, and typed conflicts/unavailable registry errors.
- `PluginBuilder::try_build` holds one process-wide assembly mutex over definition validation, complete plan preflight, and batch commit. The plan preflights schema descriptors, inference descriptors and services, formats, subset validators, composer refs, languages, document codecs, dialect migrations, app schemas, and typed plugin registrations before the first registry commit.
- Schema, native inference-service, app-schema, and language registry paths now expose typed preflight/batch registration. Poisoned assembly, registration, and inference registry locks become typed errors instead of recovered locks. Plugin registration callbacks run after their registry write lock is released.
- Native/WIT inference contracts now carry revision, generation, source dialect, policy, finite budgets, cancellation identity, previous state, cache mode, dependencies, diagnostics, provenance, and canonical payload without exposing third-party public types.
- Plugin macro/runtime bundle installation transports `PluginAssemblyError` through the owned startup fault boundary. The production checkpoint-composition-pin caller now propagates R0-B's fallible store result.

## Verification

- `cargo check --manifest-path 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml`
  - Passed on 2026-08-16. The workspace emits existing warnings, but the plugin library compiled successfully.
- `cargo test --manifest-path 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml artifact_definition_contract_tests --lib`
  - Passed on 2026-08-16: 4 passed, 0 failed. Covers plural definition conflict/identity behavior plus full-descriptor `PluginRegistration` idempotence/conflict.
- `cargo fmt --manifest-path 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml --check`
  - Failed because the shared source has unrelated formatting differences. No formatting write was made.
- `rg --files` found no Nx project target for the Rust plugin package. The nearby TypeScript registry project is unrelated, so no `bun nx` target applies to this Rust framework verification.

## Exact Remaining Integration Blockers

This lane stops at the clean framework boundary. These are deliberate compile-time migration failures, not compatibility paths.

- 46 non-stdio semantic artifact declaration factories still invoke the removed raw `ArtifactDeclaration::builder("…")` shape. They are in 46 artifact `🦀️component.rs` files: energy (1), forms (1), block (3), lowpoly (1), puzzle (3), shooting (1), procedural (2), playbook (1), note (1), demonstrator (1), draw (1), cad (1), layout (1), sourcing (1), norm (15), mathematical (1), imperative (1), fem (2), space (1), gis (2), raster (1), remodel (1), dag (1), and trinity (2). Each must author the one authoritative definition and propagate its fallible declaration terminal.
- 13 non-stdio plugin roots still use the removed imperative `.setup(...)`: `💡️reasoning`, `🎬️sequence`, `✒️writer`, `🏭️process`, `🌀️procedural`, `🌊️flow`, `🪐️space`, `🎞️animate`, `🌍️gis`, `🌿️vcs`, `🧩️puzzle`, `📐️cad`, and `🏛️architect`. Each registrar needs an explicit `PluginRegistration` with a real preflight proof before it can assemble.
- 81 non-stdio `.build()` terminals remain across the semantic declarations and adjacent builders; 30 root terminals already use `.try_build()`, and all 32 non-stdio root `plugin()` functions now return `Result<Plugin, semio_framework_plugin::PluginAssemblyError>`. The declaration-specific terminals must be changed to `.try_build()` as part of the 46-factory migration.
- No executable non-stdio `Plugin::register_document_app`/builder alias callers remain. One stale puzzle documentation reference was changed to `.document_app()`.
- A combined non-stdio workspace build is expected to fail until those two migration families are assigned. R0-C owns the stdio declaration/root migration and has the exact declaration terminal and definition API.

No ticket was closed.
