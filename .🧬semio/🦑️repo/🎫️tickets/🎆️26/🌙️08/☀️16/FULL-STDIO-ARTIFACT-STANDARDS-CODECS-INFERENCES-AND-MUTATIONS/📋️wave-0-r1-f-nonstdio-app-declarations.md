# Wave 0 R1-F Non-stdio Application Declarations

## Scope

Migrated the 11 application-artifact declaration leaves and 10 owning plugin assemblies in the assigned non-stdio trees:

- `📋️forms`: forms
- `📖️playbook`: playbook
- `🗒️note`: note
- `🖍️draw`: draw
- `📐️cad`: cad
- `📏️layout`: layout
- `🪵️sourcing`: curate
- `➗️mathematical`: mathematical
- `📜️imperative`: imperative
- `🏗️fem`: fem2d and fem3d

Each leaf now has `definition() -> Result<ArtifactDefinition, ArtifactDefinitionError>` and a fallible `declaration()` built with `ArtifactDeclaration::builder(definition()?)` and `try_build()`. Each definition owns literal identity, capability kind, descriptor, claim, and English/German localization rows. The assembly helpers only parse, validate, and assemble those rows. Runtime requirements remain supplied only to the declaration (`schema`, `inferences`, `composers`, `languages`, and document codec), which validates them exactly against the definition.

Every plugin root propagates definition construction failures through `PluginAssemblyError::definition`; no callback, alias, placeholder definition, or new production `expect` was introduced.

The curate descriptor and codec rows were checked against their implementations and use the literal values `s.sourcing.curate`, `s.sourcing.curate.inference`, and `sourcing.curate/v1`.

## Checks

- `rg -n -g '🦀️component.rs' 'ArtifactDeclaration::builder\\("' <leased application trees>` produced no raw string-based factory in the completed lease.
- Structural audit found all 11 leaves expose fallible `declaration()` functions and all 10 plugin roots map declaration-definition errors into plugin assembly errors.
- `rustfmt --edition 2021 --check` was run across the 21 edited Rust sources. It parsed them, but exited nonzero because the files have broad existing formatting drift (including regions outside this change). No formatter rewrite was applied, preserving concurrent work.
- Cargo was deliberately not run: R1-G owns active framework-plugin compile fixes and the coordinator requested source/rustfmt/structural checks only until its green signal.

## Gaps and Coordination

- `📖️playbook/🧩️extensions/🌀️procedural/**` is excluded by the coordinator’s ownership correction. It was audited only; no file or hunk in that tree was changed. Its remaining direct document-codec/setup registrar must be migrated by its designated shard.
- Repository MCP tooling was unavailable in this execution environment. This report was placed directly in the coordinator-provided active ticket folder, rather than opening, reopening, or closing a ticket through MCP.
- Cargo/test verification remains pending the coordinator’s explicit signal after the framework-plugin fixes settle.
