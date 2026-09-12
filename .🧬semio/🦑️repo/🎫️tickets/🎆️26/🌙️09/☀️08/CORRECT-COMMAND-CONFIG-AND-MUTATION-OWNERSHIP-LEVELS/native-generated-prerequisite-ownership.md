# Native Generated Prerequisite Ownership

The native SDK/editor checks stopped before tests because the canonical generated IconName source was absent. Its existing `@semio-tech/assets:build` Nx target passed and wrote 286 deterministic owned outputs. The current WGPU include resolves correctly to the shared framework assets owner; no WGPU path replacement or duplicate generated asset was added.

The CAD agent then reproduced `graph output path is not an exact safe identity` in the existing framework graph producer. The source catalog, neutral fixture and JSON Schema already declare `📇️registry/🦀️.rs` and `🔠️types/🟦️.ts`. Two TypeScript producer checks still required flat `.rs`/`.ts` filenames. Root aligned those two checks with the declared nested language owners; all path identity, Unicode, traversal, duplicate and manifest-bijection checks remain. The existing registered graph generation and independent Ajv fixture checks will supply runtime evidence.

The nested Rust registry also emitted module paths as though the registry were at the generated root. The producer now computes each include relative to the declared registry path. `graph-generate-native-prerequisite-2.log` passed and wrote nine manifests. The actual generated Rust and TypeScript reference-loading test then passed.

The first complete freshness suite reached the independent Ajv test and exposed a stale duplicate-output fixture: its duplicate value used the old flat TypeScript type path, which the current schema correctly rejects before uniqueness validation. The fixture now duplicates the existing nested Clock output, retaining its intended schema-valid but producer-invalid case. `graph-check-native-prerequisite-3.log` passed all three tests and 70 expectations and confirmed all nine generated manifests are fresh.

The taxonomy playground preview contract now declares the exact existing script invocation `generate playground-session preview`. Its project target already used that route; no validator bypass or alternate generator was added.

Changed files: framework graph Rust package `📜️script.ts`; graph neutral `🧫️fixtures/🔣️outputs.json`; graph suite `🧪️tests/🧩️suite/🟦️.ts`; repo library taxonomy `🔣️taxonomy.json` (one exact preview argument). Generated outputs remain with their registered canonical owners.
