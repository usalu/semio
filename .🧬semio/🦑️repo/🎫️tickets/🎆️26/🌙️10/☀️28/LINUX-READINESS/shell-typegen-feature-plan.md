# Shell Typegen Feature Plan

## Source-Based Recommendation

Do not add an empty Cargo typegen feature. Current shell exporter has no feature-dependent implementation or optional exporter dependency. Its Rust schema registry validates the owned JSON authority and renders authored TypeScript projection strings; its exporter and JSON/registry agreement tests are cfg(test). The owner schema has no cfg(feature=typegen) branch. The crate manifest has no features table. A feature added only to satisfy stale command arguments would be a compatibility placeholder without semantic purpose.

The existing commands helper is shell-owned, despite being shared between its local commands: `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧪️tests/🔬️rust-package-commands/🟦️.ts`. Its typegen launch line26, preview launch line75 and schema-check launch line100 each request the nonexistent typegen feature. The narrow coherent repair is to remove those feature arguments from all three owned request declarations, retaining actual exporter/test filters, Original/control, custody and generation publication. No change to the domain-neutral Cargo execution API is needed.

If command declarations are consolidated schema-first, declare explicit exporter role data (test selector and required feature list, empty for shell) owned by the shell exporter, then render arguments through existing admitted Cargo interfaces. Do not discover arbitrary features by ambient Cargo metadata, inject a generic adapter, relax feature validation, or infer that every typegen-named task needs a typegen feature. The shell exporter does not need a new runtime dependency or cfg gate to generate its current output.

## Peer Evidence

Actor Cargo declares a dedicated [[test]] named typegen with its existing exporter test path. Its authored metadata registry renders its TypeScript mirror; a Cargo feature is not required merely because an exporter is a test.

UI contract is the contrasting purposeful feature: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/Cargo.toml:27` declares typegen=[dep:semio-framework-ui-contract-typegen], enabling an actual optional first-party schema projection crate. This is a dependency exclusion boundary, not an empty naming convention. Async features likewise enable concrete testkit/entrypoint/host implementation behavior rather than accepting unused launch names. No corresponding boundary exists in current shell Source.

## Preserve Real Authority

Shell package glue mounts the owner component, whose public schema module mounts its own test module. The exporter calls schema_registry.validate(), renders the exact registered projection and either writes the requested output or compares the existing mirror. Schema-check additionally compares JSON public exports, Rust registry, TypeScript parser exports and rendered mirror. Keep these checks and actual generated output coordinates; do not create stubs, replace missing types by casts, disable exporter tests or add a feature solely to accept old arguments.

A durable neutral command corpus should distinguish shell zero-feature exporter from the real UI feature-enabled exporter; independent TOML parsing can verify every requested feature exists and exact expected argv, while the existing actual Cargo target confirms runtime production. Such tests should exercise the actual request declaration rather than duplicate a constructed command string. This report proposes no test or code change itself.

Root73042 is still the sole actual producer and must settle before claiming this Source inconsistency was its runtime failure. The earlier live diagnostic proves graph convergence, not Rust exporter success. No tests, Cargo, compiler, Native, process or lease commands were launched during this audit.
