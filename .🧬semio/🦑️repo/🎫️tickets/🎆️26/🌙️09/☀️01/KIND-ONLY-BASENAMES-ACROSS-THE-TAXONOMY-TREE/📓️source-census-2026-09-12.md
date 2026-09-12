# Cross-Language Source Candidate Census

A separate coordinator census checks filename stems across every unignored first-party Rust, Go, TypeScript, TSX, JavaScript, JSX, Python, C#, C, C++, header, and CSS file. It excludes the opaque research subtree and ticket history. This is deliberately a candidate census, not a policy verdict: exact tool contracts, generated bindings, and scripts must be classified separately.

The initial census finds 913 filenames containing semantic ASCII letters or underscores before their extension: 610 TypeScript, 217 Rust, 27 JavaScript, 26 Go, 20 TSX, 11 CSS, one C#, and one Python. Of these, 453 are the explicitly required `📜️script.ts` filename and 42 are `vitest.config.ts`; neither family is an automatic violation. Compound `.d.ts` stems also need extension-chain-aware classification.

This cross-check reveals additional execution populations beyond the initial Terra packets:

- Repo CLI, MCP, and coordinator Go source files carry semantic basenames. The CLI also has conventional `internal` subpackages. Go compilation and platform constraints must be preserved when making the domain tree anonymous.
- Artifact generators and probes contain Rust `main.rs`, `lib.rs`, named readers, and named generators. These are first-party implementations of test oracles, distinct from generated output. Their Cargo entry paths and helper module declarations need explicit anonymous leaves and semantic owners.
- Framework schema, graph bridges, OS services, flow protocol, CLI binaries, and UI rendering backends have further named Rust leaves.
- Authored Mit-Bestand source is distinct from the captured public web bundle.

The generated candidate list is at `🗑️generated/coordinator/named-source-candidates.json` during execution. Completion requires the production gate and an independent fresh census to classify the full population, not just the two `🦀️component.rs` names called out in the first audit.
