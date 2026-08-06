# 📋️ Registrar Handoff — Framework Math Family Crate Consolidation

The 51 math-family crates under `🧰️framework/🔨️modules/🧮️math/**` are gone. One Shape V2 crate replaces
them all:

| | |
| --- | --- |
| crate name | `semio-framework-math` |
| lib name | `semio_framework_math` |
| path | `🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust` |
| role | `framework` (`[package.metadata.semio] role = "framework"`) |
| nx project (rust) | `@semio-tech/framework-math` |
| nx project (ts) | `@semio-tech/framework-math-js` |

Nothing outside `🧰️framework/🔨️modules/🧮️math/**` was touched. Every edit below is the registrar's.

## 1. Root `Cargo.toml` — `[workspace] members`

Delete all **51** lines matching `"🧰️framework/🔨️modules/🧮️math/…/⚡️implementations/🦀️rust"`
(they are the only members under that prefix; see `🧪️root-math-lines.txt` for the captured lines and their
line numbers at the time of writing). Replace them with the single entry:

```toml
    "🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust",
```

Do **not** touch `"✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/📦️packages/🦀️rust"` — different owner.

## 2. Root `Cargo.toml` — `[workspace.dependencies]`

Delete the 13 `semio-framework-os-kernel-math-*` convenience entries (root lines 295–307 at time of
writing) and add:

```toml
semio-framework-math = { path = "🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust" }
```

## 3. Consumer `Cargo.toml` rewrites (24 manifests)

All external consumers use explicit `path = …, package = …` dependencies — none use
`.workspace = true` — so each one needs its math dependency block collapsed into a single line:

```toml
math = { path = "<relative>/🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust", package = "semio-framework-math" }
```

The full list is in `🧪️external-consumers.txt`. Their Rust sources then need the alias-to-module rewrite
from the table in §4 (e.g. `mathematical_graph::DirectedGraph` → `math::graph::DirectedGraph`).

## 4. Old crate → new module path

| old crate | old alias | new path |
| --- | --- | --- |
| `…-math-algebra` | `mathematical_algebra` | `semio_framework_math::algebra` |
| `…-math-cas` | `mathematical_cas` | `…::cas` |
| `…-math-causal` | `mathematical_causal` | `…::causal` |
| `…-math-entropy` | `mathematical_entropy` | `…::entropy` |
| `…-math-fuzzy` | `mathematical_fuzzy` | `…::fuzzy` |
| `…-math-geometry` | `mathematical_geometry` | `…::geometry` |
| `…-math-lie` | `mathematical_lie` | `…::lie` |
| `…-math-number` | `mathematical_number` | `…::number` |
| `…-math-optimize` | `mathematical_optimize` | `…::optimize` |
| `…-math-polynomial` | `mathematical_polynomial` | `…::polynomial` |
| `…-math-probability` | `mathematical_probability` | `…::probability` |
| `…-math-random` | `mathematical_random` | `…::random` |
| `…-math-sampling` | `mathematical_sampling` | `…::sampling` |
| `…-math-signal` | `mathematical_signal` | `…::signal` |
| `…-math-spatial` | `mathematical_spatial` | `…::spatial` |
| `…-math-statistics` | `mathematical_statistics` | `…::statistics` |
| `…-math-tabular` | `mathematical_tabular` | `…::tabular` |
| `…-math-wfc` | `mathematical_wfc` | `…::wfc` |
| `…-math-graph` | `mathematical_graph` | `…::graph` |
| `…-math-graph-drawing` | `mathematical_graph_drawing` | `…::graph::drawing` |
| `…-math-graph-dsl` | `mathematical_graph_dsl` | `…::graph::dsl` |
| `…-math-graph-manifest` | `mathematical_graph_manifest` | `…::graph::manifest` |
| `…-math-graph-operators` | `mathematical_graph_operators` | `…::graph::operators` |
| `…-math-graph-traversal` | `mathematical_graph_traversal` | `…::graph::traversal` |
| `…-math-graph-normal-directed` | `mathematical_graph_normal_directed` | `…::graph::normal::directed` |
| `…-math-graph-normal-undirected` | `mathematical_graph_normal_undirected` | `…::graph::normal::undirected` |
| `…-math-graph-port-directed-normal` | `mathematical_graph_port_directed_normal` | `…::graph::ports::directed::normal` |
| `…-math-graph-port-undirected` | `mathematical_graph_port_undirected` | `…::graph::ports::undirected` |

The 23 NetworkX-parity graph stubs below had empty `📦️lib.rs` bodies and no code consumers; they are
deleted with no replacement module. Any consumer manifest still naming them just drops the line:

`approximation`, `bipartite`, `centrality`, `cliques`, `clustering`, `coloring`, `community`,
`components`, `connectivity`, `cycles`, `dag`, `flow`, `generate`, `generate-random`, `io`,
`isomorphism`, `matching`, `paths`, `planarity`, `similarity`, `spectral`, `structure`, `trees`.

## 5. npm packages

`@semio-tech/graph-dsl-core` and `@semio-tech/graph-manifest` are replaced by
`@semio-tech/framework-math-js` (`🧰️framework/🔨️modules/🧮️math/📦️packages/🟦️typescript`). Consumers to
repoint: `🧰️framework/🛍️products/💻️os/⚡️implementations/🟦️typescript/package.json`,
`🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-assets.ts`, plus the root
`📜️script.ts` and `bun.lock` regeneration.

The manifest schema export moved: `@semio-tech/graph-manifest/🔣️manifest.schema.json` →
`@semio-tech/framework-math-js/🔣️manifest.schema.json`.

## 6. Codegen relocation

Manifest codegen now lives in `📦️packages/🦀️rust/📜️script.ts` and writes to
`🧰️framework/🔨️modules/🧮️math/🤖️generated/` (owner root, per Shape V2). `bun nx run
@semio-tech/framework-math:generate` is a `dependsOn` of every test/lint target, and `build.rs`
bootstraps it when the output is missing.

## 7. Verification status

The crate is not a workspace member yet, so it was verified with a temporary standalone workspace
(`🔧️cargo.sh` appends `[workspace]` + the repo's workspace lints, runs cargo, then restores the
manifest). Results: `cargo check` clean, `cargo clippy --all-targets` clean for
`semio-framework-math`, `cargo test --no-fail-fast` = **1740 passed / 13 failed**, byte-identical to
the pre-consolidation baseline in `🧪️baseline-tests.txt` (all 13 are pre-existing `cas` and
`polynomial` failures). Once §1 lands, drop `🔧️cargo.sh` and use `bun nx run
@semio-tech/framework-math:test`.
