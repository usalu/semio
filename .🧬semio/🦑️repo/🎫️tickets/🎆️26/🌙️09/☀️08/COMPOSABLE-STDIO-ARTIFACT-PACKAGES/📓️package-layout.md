# Composable Artifact Packages

## Layout

All 99 production artifact owners have dedicated Rust declarations. The two shared registry contracts are separate packages. TypeScript implementations have 40 artifact package declarations, including the newly separated Sequence browser API. The independent static audit reconciles the inventory; compiler, runtime and cache acceptance are still in progress.

For PDF, the artifact implementation remains at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🦀️.rs` and its standards, subsets, schema, operations and tests stay in their domain taxonomy. `📦️packages/🦀️rust/Cargo.toml` points to that source using `[lib] path = "../../🦀️.rs"`. Its neighboring Nx declaration and `📜️script.ts` only declare and route package tasks. TypeScript similarly builds the taxonomy-root source into package-local JavaScript and declaration outputs.

## Composition

A Rust consumer selects the artifact it uses:

```toml
[dependencies]
semio-s-artifact-stdio-pdf = { workspace = true }
```

PDF's selected artifact dependencies are Binary, Deflate and the stdio shared contract. It does not depend on the stdio plugin or other catalog formats. Artifact app assembly is selected explicitly through `component-app-assembly`. The stdio plugin composes the full catalog, while `home-io` selects its smaller input/output surface. Semio conversion bridges have explicit conversion features, allowing its schema and model consumers to remain independent of unrelated media formats.

Plugin and framework hosts refer directly to artifact package APIs. Artifact implementations do not depend back on their composition parent. The seven framework document packages own Workflow, Run, Playbook, Flow, DAG, Space and Collection data; host orchestration stays at the existing host layer.

## Development

The launch configuration provides an artifact package project selector and build/check/test selector. For PDF, use `@semio-tech/stdio-pdf-rs` for Rust or `@semio-tech/stdio-pdf` for TypeScript. A separate launch configuration runs the repository artifact package contract.

The corresponding Nx targets are `@semio-tech/stdio-pdf-rs:build`, `:check` and `:test`; each invokes its local Bun script router. Nx inputs include the artifact's taxonomy source and declared dependency inputs. TypeScript package exports resolve built files within the package's `dist` directory, including `.d.ts` declarations.

## Validation Status

See the live execution and integration reports for exact passing, failing and pending gates. No final compile-time speedup is claimed while the current runtime and cache checks remain incomplete.
