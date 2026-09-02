# 🧪️ Artifact-Child Export Verification

## Compiler Confirmation for Mathematical CSV

A minimal Rust source preserving the relevant shape—an `async fn mathematical_graph`, a synchronous
caller, and `.nodes` on the returned value—was compiled with `rustc`. The compiler produced
`E0609: no field nodes on type impl Future<Output = Graph>` and explicitly suggested awaiting the
future. The reproducer is retained at
`🔬️mathematical-csv-future-repro/🦀️.rs`.

The production fix removes the unnecessary async boundary from the synchronous mathematical scene
accessor family, and `MathematicalIntoCsv` reads `require_mathematical_scene(from)?.graph`
directly.

## Carrier Fixture Engines

Both generators are standalone `[workspace]` crates and use one third-party runtime dependency,
`serde_json`. They mutate and project the complete carrier independently of production plugin code.

- Mathematical generated 10 JSON pairs: `change-coefficient`,
  `change-graph-directed`, `connect-nodes`, `disconnect-nodes`, `insert-point`, `move-point`,
  `remove-point`, `replace-graph`, `replace-points`, and `update-graph-algorithm`.
- Sequence generated 4 JSON pairs: `change-step-collapsed`, `connect-steps`,
  `disconnect-steps`, and `move-step`.
- Every generator executes `assert_ne!` for every pair.
- The separately built reader projected every committed before/after pair through `serde_json`; all
  14 comparisons reported `equal:false`.
- A second generation through the two Nx targets reproduced every mathematical JSON file and every
  sequence JSON/CSV file byte-identically.

## Production Verification Boundary

The requested Nx production test was invoked:

`CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo-target bun nx run '@semio-tech/mathematical-plugin:test-quick'`

Nx stopped before compiling the plugin because the repository's current taxonomy schema references a
missing tracked output:

`jcoprobe/.../📚️library/🦀️.rs`

This is earlier than the already-known `semio-s-plugin-stdio` `E0046` blocker. Consequently no
production exporter has been executed in this worktree, and no claim is made that the plugin crates
compile or that runtime export succeeds.

Per the ticket rule and the explicit task instruction, the prepared mathematical and sequence carrier
oracles and fixture manifests have **not** been merged into the contribution catalog. Registration
must wait until the relevant exporter can be executed and its emitted bytes accepted by the
third-party reader.
