# Fixed Filename Contract Review

The coordinator read the current catalog after the latest source extractions: 553 fixed-filename contracts, including 252 Bytecode Alliance JCO, 65 JCO interface-emitter, 20 preview2-shim, 12 wasm-bindgen and two wasm-pack contracts. These counts are catalog declarations, not 553 existing files or defects. The exact narrowed snapshot is 🗑️generated/coordinator/fixed-filename-contract-review.json.

Several config contracts identify their own verification through an explicit config selector while declaring the name unconfigurable: Vitest, Tailwind, PostCSS, ESLint and dependency-cruiser. TypeScript also has package, root, Nx-owned and window-kit tsconfig entries. These are candidates for native-consumer review, not permission to rename all tool configuration. A compiler flag alone does not prove every ordinary editor, task-runner, discovery and package-consumer route can find the replacement automatically. Preserve required zero-touch native discovery and distinguish actual executable behavior from declarative tool metadata.

The Cargo native build-entry probe is stronger concrete evidence: the manifest itself selects the anonymous source and ordinary cargo check executes it. Follow 📓️cargo-build-entry-constraint-probe-2026-09-12.md for that bounded correction. The existing package metadata and tooling packets cover JCO producer configurability, persistent generated bindings and root configurations; inspect installed emitter/native options, current consumers and source bodies before changing their precise contracts.

The first inline catalog query had a syntax error and produced no result; the corrected read produced the counts above. No product code, configuration, artifact or passing validation gate changed in this review.
