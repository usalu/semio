# Final Artifact Package Audit

## Scope And Method

Read-only audit of the in-flight workspace on 2026-09-08. This review inspected package declarations, package-neutral crate roots, direct Cargo metadata, normal dependency declarations, source aliases, and the custom Nx normalizer. It did not change production files, run a compiler gate, alter Git state, or claim a runtime/cache result that was not observed.

## Inventory

The Cargo workspace currently exposes **101** `semio-*-artifact-*` packages:

| Kind | Count | Result |
| --- | ---: | --- |
| Stdio artifacts | 36 | Every stdio artifact has its own Rust package. |
| Other plugin artifacts | 56 | Every non-stdio plugin artifact has its own Rust package. |
| Framework artifacts | 7 | `workflow-run`, `workflow-workflow`, `playbook-playbook`, `flow-flow`, `infinite-dag`, `space-space`, and `space-collection` each have one package. |
| Shared contracts | 2 | `semio-s-artifact-stdio-contract` and `semio-s-artifact-norm-contract`; these are shared contracts, not product artifacts. |

This reconciles the expected **99 product artifacts + 2 shared contracts**. The initial 93 plural-directory owners comprise the 36 stdio, 56 other-plugin, and existing workflow-run artifacts. The remaining six semantic document owners are the framework workflow, playbook, Flow, DAG, space, and collection artifacts.

The source-root scan initially returns 101 immediate `.../🗿️artifacts/*/🦀️.rs` paths. Two are nested glTF serializer/deserializer paths named `.../artifacts/🔣️json`, not top-level product artifact roots; excluding those leaves the expected 99 product roots. They correctly have no product package.

The repository's test probes/oracles and the Store's durable outcome/history records are excluded. They are testing/persistence infrastructure and are neither registered application artifacts nor independent product document owners.

## Package Boundary

All 101 artifact/contract Cargo manifests use `[lib] path = "../../🦀️.rs"`, and every referenced package-neutral source root exists. Every artifact package has both its `📋️project.json` and its local `📜️script.ts`; no duplicate artifact project name was found.

The seven framework project identities exactly follow the required naming rule:

```text
@semio-tech/framework-workflow-run-rs
@semio-tech/framework-workflow-workflow-rs
@semio-tech/framework-playbook-playbook-rs
@semio-tech/framework-flow-flow-rs
@semio-tech/framework-infinite-dag-rs
@semio-tech/framework-space-space-rs
@semio-tech/framework-space-collection-rs
```

The 99 artifact package declaration directories themselves contain manifests, Nx declarations, and routing scripts rather than their artifact Rust implementation. The only Rust implementation files discovered below an artifact `📦️packages` subtree belong to two pre-existing nested drawing command FSM crates:

- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🔄️fsm/📦️packages/🦀️rust/📚️library/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🔄️fsm/✨️macros/📦️packages/🦀️rust/📚️library/🦀️.rs`

They are command/FSM infrastructure, not members of the 99 ArtifactDsl product inventory. If “declaration-only `📦️packages`” is an absolute repository-wide rule rather than the artifact-package rule, these two packages remain actionable exceptions and should be remounted outside their package directories.

## Dependency Direction And Aliases

`cargo metadata --offline --format-version 1 --no-deps` completed successfully with 229 workspace members and all 101 artifact/contract packages. Its direct normal-dependency declarations show four artifact-to-`semio-s-plugin-*` rows:

| Artifact | Dependency | Selection status |
| --- | --- | --- |
| `lowpoly-lowpoly` | `semio-s-plugin-cad` | Optional through `cad-fixtures`; no default feature is declared. |
| `norm-en1992` | `semio-s-plugin-fem` | Optional through `cross-fem`; `default = []`. |
| `norm-en1993` | `semio-s-plugin-fem` | Optional through `cross-fem`; `default = []`. |
| `draw-drawing` | `semio-s-plugin-draw-fsm` | Non-optional, but it is the nested drawing-command FSM package above, not the draw plugin composition package. |

Metadata reports optional normal declarations even when they are not selected. The first three rows are therefore not proof of a selected-default backedge. A feature-isolated `cargo tree`/compile proof is intentionally pending while execution agents continue changing the tree. The direct draw FSM row needs architectural review only if its package identity/location is meant to be a plugin assembly rather than artifact-local command infrastructure.

The package-neutral stdio source audit found no production `crate::registry`, `crate::artifacts`, or `semio_s_plugin_stdio` dependency backedge. Remaining hits are test oracles, generator documentation, and explanatory comments. Every executable `semio_s_artifact_*` identifier matches a declared Cargo artifact/contract package; the only unmatched token is the documentation placeholder `semio_s_artifact_norm_en199x` in EN 1990 commentary. No malformed `$semio_s_artifact_*` rewrite was found.

## Nx Composability And Inputs

Artifact `📋️project.json` files correctly invoke only local `bun ./📜️script.ts` commands. Although their authored build targets name `production`/`^production`, the workspace normalizer expands package delivery scopes to their artifact owner: `📦️packages` is in `testDeliveryScopeDirectoryNames`, and `projectInputs` adds the package-neutral owner tree to `default`.

The direct normalizer result for `@semio-tech/stdio-pdf-rs` is:

```text
build.inputs = ["production", "^production"]
production = ["default", test exclusions]
default includes {workspaceRoot}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/**/*.{rs,toml,json,semio,wit,wgsl,glsl,h,c,cpp}
nativeSources includes the same owner recursive glob
```

Thus an artifact implementation edit is an input to its own package task, while a sibling artifact is outside that owner tree. Cargo targets are cacheable and non-parallel, retaining the shared compiler store while caching only `dist/build` outputs.

The authoritative `bun x nx show project @semio-tech/stdio-pdf-rs --json` did not return within its 30-second bound because concurrent Nx graph generation timed out its `@nx/js` worker. The intended PDF-mutation/JPG-invariance/cache-restoration gate is also pending; earlier restoration attempts were invalidated by concurrently changing shared inputs. These are pending validation, not failures.

## Final Status

The static package inventory, ownership, declaration-only artifact boundary, framework naming, and primary stdio source-direction checks are consistent with the requested topology. Full acceptance remains pending the execution agents’ 17 multi-artifact default/component-facade checks, 24 single-artifact/Nx-neutral-validator checks, and the actual Nx mutation/invariance gate. The optional CAD/FEM declarations and the two nested drawing FSM package implementations are the only architectural follow-ups identified by this audit.

## Coordinator Inventory Reconciliation

A fresh declaration scan found 101 artifact-named Rust crates: 99 production artifact crates plus the Norm and stdio shared contracts. There are also 101 Cargo declarations physically under artifact taxonomy paths: those are the 99 production artifacts plus two pre-existing Draw FSM support crates (runtime and derive macros). The two shared contract packages are outside the artifact directories. Every artifact-path package resolves an existing taxonomy-root `../../🦀️.rs`. These counts describe different selections and must not be added as if they were distinct production artifacts.

## Emitted Declaration Consumer Finding

The coordinator inspected PDF's restored `dist/🟦️.d.ts`. It imports `./🧬️schema/📜️artifact-definition.json`, but the output directory contains only the JavaScript and declaration entry files. The builder emits declarations with `--emitDeclarationOnly`, which does not copy that JSON dependency. The existing consumer probe also uses unconditional `--skipLibCheck`, suppressing the unresolved declaration import, and only references the imported namespace. The Nx executor owns correction of declaration assets and a consumer check that fails on missing emitted dependencies, followed by a refreshed cache restoration proof for the complete required output set. Previous two-file hash restoration results remain valid evidence for caching those files, but do not establish a usable published package by themselves.

A fresh source-file scan excluding generated dist/target outputs found 3 package-local implementation source files under artifact taxonomy package directories. Declared `📜️script.ts` routers are excluded from implementation ownership. This includes the nested Draw FSM support packages.
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/📦️packages/🟨️javascript/🌐️sequence-browser.js`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/📦️packages/🟨️javascript/📜️sequence-browser.d.ts`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/📦️packages/🟨️javascript/🖥️sequence-host.js`
