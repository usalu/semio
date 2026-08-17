# Wave 0 R1-H Non-stdio Media Declarations

## Scope

Completed the assigned non-stdio media declaration lane across `🔋️energy`, `🧱️block`, `💠️lowpoly`, `🧩️puzzle`, `🎥️shooting`, `🎪️demonstrator`, `🖨️raster`, `📸️remodel`, `🕸️dag`, and `🔱️trinity`.

The current tree has 15 direct artifact roots in those owners:

- energy model;
- block 2d, 3d, and 5d;
- lowpoly;
- puzzle 2d, 3d, and 5d;
- shooting;
- demonstrator playground;
- raster;
- remodel;
- dag;
- trinity jack and rewrite.

Every root exposes `definition() -> Result<ArtifactDefinition, ArtifactDefinitionError>` and a fallible `declaration()` built only from `ArtifactDeclaration::builder(definition()?)`. Definitions contain literal identity, capability, claim, and English/German localization rows. Declarations bind the concrete typed schema, inference, composer, language, and document-codec facets; no declaration factory receives a raw artifact-kind string.

The 10 owner assemblies propagate any definition validation error through `PluginAssemblyError::definition`. The demonstrator manifest declares its owned playground artifact before registering its six foreign document apps and has no executable registrar callback.

## Cleanup

Removed the 15 dead root-level `io_registry` aliases. Their only imperative behavior had been forwarding to `register_composer_entries`; every declaration already consumes the leaf-owned `standards::v1::subsets::any::io::io_registry::entries()` facet directly. This removes the remaining registrar/callback path without replacing it with a runtime-derived alias or fallback.

## Validation

- Structural scan: 15 direct artifact roots, 15 literal definitions, 15 definition-gated declarations, and 15 typed composer facets.
- Structural scan: zero raw `ArtifactDeclaration::builder("…")` factories, zero root `io_registry` aliases, zero imperative root `register()` functions, and zero executable root `.setup()` callbacks.
- `rustfmt --edition 2021 --check` passed for all 15 artifact roots and 10 owner assembly sources.
- `bun nx run-many -t test-quick --projects=@semio-tech/energy-plugin,@semio-tech/block-plugin,@semio-tech/lowpoly-plugin,@semio-tech/puzzle-plugin,@semio-tech/shooting-plugin,@semio-tech/demonstrator-plugin,@semio-tech/raster-plugin,@semio-tech/remodel-plugin,@semio-tech/dag-plugin,@semio-tech/trinity-plugin --parallel=1` began after the plugin-integration barrier opened. It was stopped by its owner with exit `130` after repeated non-owned shared-build failures, rather than holding the serialized Cargo lock after the blocker was established.
- Observed failures: remodel stopped in framework OS host with `E0425` because `semio_framework::FormatRegistryError` is absent; block, trinity, and demonstrator stopped compiling shared stdio because `📦️glue.rs` references a missing glTF `🧮️geometric-analysis/🦀️component.rs`. Lowpoly was active when cancelled. Energy, puzzle, shooting, raster, and dag did not produce observable results before cancellation, so this report makes no assertion for them.

## Environment Note

Repository MCP ticket tools were unavailable in this execution environment. This required report is therefore written directly into the coordinator-provided active ticket directory.
