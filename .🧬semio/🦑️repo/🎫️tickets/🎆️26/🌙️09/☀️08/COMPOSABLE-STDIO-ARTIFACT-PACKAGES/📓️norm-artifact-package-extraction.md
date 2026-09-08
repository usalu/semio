# Norm Artifact Package Extraction

## Scope

This packet extracts the 15 norm artifacts into independently addressable Rust crates while keeping implementation sources in the domain taxonomy. Framework workflow/run is explicitly outside this packet and remains owned by the coordinator.

## Architecture

The package-neutral lower boundary is `semio-s-artifact-norm-contract` at `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/📦️packages/🦀️rust`. Its `lib.path` mounts `📇️registry/🧬️contract/🦀️.rs`, which owns:

- common norm document/compliance APIs and `impl_norm_artifact_record!`;
- config schema and mutations;
- common editor/viewer app-surface APIs and `norm_owned_tool_job_factory!`;
- artifact-definition assembly;
- a language-neutral package declaration parser and semantic validator.

The contract has framework/pack dependencies only. It has no norm artifact, norm plugin, or stdio dependency.

Each artifact owns its existing taxonomy source root, its editor/viewer implementation modules, its artifact-local `🧬️schema/📜️artifact-definition.json`, and a Cargo declaration under its local `📦️packages/🦀️rust`. Package names are `semio-s-artifact-norm-<id>`. Artifact definitions validate their local package schema before assembly.

The norm plugin package is now a composition facade: its package root mounts `norm/🦀️.rs`, while the plugin source imports the 15 leaf crates directly to assemble the closed `NormApps` enum, definitions, editors, viewers, activations, and capability requests. It does not expose a public `artifacts::<id>` compatibility namespace.

## Dependency graph

- `din4108`, `din16798`, and `vdi3805` depend only on the shared contract and framework support.
- `din18599` depends on `din16798`, `din4108`, and stdio `semio`.
- `en1990` depends on stdio `semio`.
- `en1991` depends on `en1990`.
- `en1992` has an optional direct dependency on the `semio-s-artifact-fem-2d` leaf.
- `en1993` depends on `en1992` and has an optional direct dependency on the `semio-s-artifact-fem-2d` leaf.
- `en1994` depends on `en1993`.
- `en1995` through `en1999` form the observed standard chain and also use `en1990` where their sources require it.
- `iso16757` depends on `en1999`.

The plugin `cross-fem` feature propagates into the two FEM-owning leaves.

The two cross-domain helpers now import the 2D artifact's public `elements2d` and `model` APIs directly. Their optional feature graph contains no dependency on the owning FEM plugin and constructs the leaf's closed `Elements::BeamEb2` variant explicitly.

## Schema test

The contract has positive and negative language-neutral JSON fixtures. Its Rust test parses the valid fixture with the first-party pack codec and independently with `serde_json`, compares canonical identity fields, and verifies that the first-party semantic validator rejects a syntactically valid invalid fixture.

All 15 artifact schemas were parsed independently with Bun and verified against the identity rules:

- `id = s.norm.<artifact>`;
- `rust_package = semio-s-artifact-norm-<artifact>`;
- `nx_project = @semio-tech/norm-<artifact>-rs`.

## External consumer audit

No Rust source outside the norm plugin imports the removed `semio_s_plugin_norm::{artifacts,editor,viewer,document,config,app_surface}` APIs. The framework DSL fixture-sweep package still declares a norm plugin dependency for integration coverage; it does not use the removed umbrella source API.

## Validation

Passing:

```text
CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo cargo test -p semio-s-artifact-norm-contract --lib
```

Result: 18 passed, 0 failed. This covers the package schema oracle and the mounted shared compliance, config, and app-surface behavior.

```text
cargo metadata --no-deps --format-version 1
```

This resolves all 16 norm package manifests and the intended leaf-to-leaf dependency graph.

```text
bun -e <artifact-schema identity audit> <each of 15 artifact schemas>
```

This validates all 15 language-neutral package declarations.

```text
cargo tree -p semio-s-artifact-norm-contract --prefix none
```

This confirms the lower contract has no norm plugin, norm leaf, or stdio back-edge.

```text
CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo cargo check -p semio-s-artifact-norm-din16798 --lib --message-format short
CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo cargo check -p semio-s-artifact-norm-vdi3805 --lib --message-format short
```

Both independently compiled leaves pass. DIN 16798 exercises a standalone package boundary; VDI 3805 exercises the macro-heavy source tree that requires conservative Nx inputs.

`cargo fmt --package ... -- --check` parsed the contract and representative small/composed/macro-heavy leaves, then reported pre-existing repository formatting differences. It was used as a parse audit rather than recorded as a passing format gate.

The stdio owner subsequently repaired that boundary. A fresh `cargo tree -p semio-s-artifact-norm-en1990 --prefix none` now shows only `semio-s-artifact-stdio-semio` and `semio-s-artifact-stdio-contract`, with zero unrelated stdio artifact leaves. The final full norm wildcard and plugin test commands are queued after the shared OS-host build and listed in `📓️framework-space-artifact-extraction.md`.

Both optional FEM dependency trees resolve directly to `semio-s-artifact-fem-2d`, and static source/manifests contain no `semio-s-plugin-fem`, `fem::core`, or `dep:fem` reference:

```text
cargo tree -p semio-s-artifact-norm-en1992 --features cross-fem --prefix none
cargo tree -p semio-s-artifact-norm-en1993 --features cross-fem --prefix none
```

The first complete leaf compile also found that the extracted ISO 16757 root re-exported `Iso16757Snapshot` through both its canonical document-schema route and its generated standards route. The redundant standards-route re-export was removed; the canonical root API remains unchanged.

## Integration handoff

Nx declarations are owned by the Nx execution packet. Native source inputs must explicitly include the lower contract taxonomy sources because they are outside the contract package directory. VDI 3805 must conservatively include its complete artifact owner glob because Rust source discovery cannot see every macro-generated module.

## Related finding

Before the stdio owner fix, `semio-s-artifact-stdio-semio` unconditionally depended on all other 35 stdio leaves, causing `en1990` and its downstream norm chain to pull the full stdio catalog. This was reported to the coordinator and reassigned to the stdio owner because it directly defeats the selective-compilation goal.
