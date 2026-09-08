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

The corrected complete leaf gate passes with both FEM integrations enabled:

```text
CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo cargo check \
  -p 'semio-s-artifact-norm-din*' \
  -p 'semio-s-artifact-norm-en*' \
  -p semio-s-artifact-norm-iso16757 \
  -p semio-s-artifact-norm-vdi3805 \
  --features semio-s-artifact-norm-en1992/cross-fem,semio-s-artifact-norm-en1993/cross-fem \
  --lib --keep-going --message-format short
```

Result: all 15 norm leaves pass. EN 1992 and EN 1993 compile against `semio-s-artifact-fem-2d`; ISO 16757 compiles with its single canonical snapshot export. Existing lint warnings remain outside the extraction boundary.

The final thin-plugin runtime gate was started once with incremental output disabled:

```text
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo cargo test -p semio-s-plugin-norm --lib --message-format short
```

The original attached session was `67307` with Cargo PID `22032`. After the execution turn finalized, the session became unavailable (`Unknown process id`) and the PID disappeared without a durable raw log. No pass/fail result is claimed from that run. The explicit config and surface integration targets registered afterward are the authoritative replacement gates.

## Integration handoff

Nx declarations are owned by the Nx execution packet. Native source inputs must explicitly include the lower contract taxonomy sources because they are outside the contract package directory. VDI 3805 must conservatively include its complete artifact owner glob because Rust source discovery cannot see every macro-generated module.

## Related finding

Before the stdio owner fix, `semio-s-artifact-stdio-semio` unconditionally depended on all other 35 stdio leaves, causing `en1990` and its downstream norm chain to pull the full stdio catalog. This was reported to the coordinator and reassigned to the stdio owner because it directly defeats the selective-compilation goal.

## Coordinator Integration Target Audit

The final read-only audit found two missing explicit Cargo integration targets still referenced by persistent script routes. The coordinator mounted the existing config mutation test under the shared Norm contract package and routed that command directly to the contract. The full thirty-app surface-render integration test remains a parent plugin composition test and is now explicitly mounted from its taxonomy source, with its existing third-party oracle dependencies declared only for tests. Both target paths resolve on disk. Native execution is pending. The remaining seven-line parent package facade was removed; Cargo now mounts the taxonomy root directly, where its unchanged plugin export macro resides.

The two independent source/oracle commands completed with exit 0: config checked 5 cases, 5 hostile payloads, 4 undeclared forms, 13 text vectors and 25 binary vectors; surface checked 15 variants, 30 apps, 120 bodies and 5 hostile vectors with AJV. These are source/oracle checks; the new native integration targets still require execution.

Additional coordinator-owned files:
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/📦️packages/🦀️rust/Cargo.toml`

A focused scan of literal Rust include paths in the shared Norm config and app-surface taxonomy found 0 unresolved references.

Before native execution, the mounted integration surfaces were audited against the current graph. The config test imports only `semio-s-artifact-norm-contract` plus the framework kernel traits, and its semantic kind, text opcode, and binary tag match the owned change-selected-check-index descriptor. The surface test imports the public framework testkit and the fifteen artifact crates directly; every editor/viewer type path and plugin app/artifact identity resolves from the current package roots. No Rust source or manifest below Norm refers to the removed parent artifact/config/app-surface API or to `semio-s-plugin-fem`. Both integration sources pass `rustfmt --check`; the surface test import layout was normalized during this audit.

The three authoritative native commands are staged sequentially with `CARGO_INCREMENTAL=0` and the one ticket Cargo target: contract `config-mutation-test`, plugin `test -- --lib`, then plugin `surface-render-test`. They remain intentionally unstarted while the coordinator's host Cargo process owns the shared build epoch.
