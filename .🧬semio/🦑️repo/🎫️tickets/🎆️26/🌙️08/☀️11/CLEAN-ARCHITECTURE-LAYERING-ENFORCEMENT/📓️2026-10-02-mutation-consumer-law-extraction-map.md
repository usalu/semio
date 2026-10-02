# Mutation Consumer Law Extraction Map

Read-only source inspection; no jobs. Selected-law timeout before results is parent-reported; this report does not infer an import-stage timing measurement.

Owner base `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`. Current four laws reside `🧪️tests/🔬️workspace-contract/🟦️.ts`: scaffold :7411, inventory :7528, reachability :7596, wrapped type origin :7631. They share describe helpers :6985–7000, not unrelated workspace contracts.

## Minimal dependency closure

All four require bun:test describe/test/expect, Ajv, filesystem/path functions and node:child_process spawnSync; scaffold/inventory use fast-glob. Source providers: scaffold `🏗️authoring/🧬️mutation-tree/🟦️.ts`; discovery `🔍️discovery/🟦️.ts` exports inspectRustMutationAggregateSpan, inspectRustModuleGraphFacts, inspectRustStructure, canonicalPrimaryFilenameForKind; inventory `🧹️normalization/🧬️mutation/🧾️evidence/🟦️.ts`; reachability/type-origin `🧹️normalization/🧬️mutation/📐️structural-reachability/🟦️.ts` exports policyMutationStructuralBreaches/inspectMutationRootReachability. Taxonomy/root/git environment are currently imported from broad package barrel :18; use their actual narrow owning APIs where available, preserving the same environment and workspace authority semantics.

Three shared helpers :6985–7000: mutationRoot canonical probe taxonomy path; prepareMutationFixtureRoot initializes isolated fixture Git with --template= and copies taxonomy+taxonomy schema; mutationFixtureRoot requires caller SEMIO_TEST_ARTIFACT_DIR and creates isolated root. Keep these helpers adjacent to extracted tests, with repository library root resolved explicitly. Inventory also independently git init/copies same files (:7530–7534); preserve behavior initially and only deduplicate once fixture setup equivalence is proved. No live Git mutation is involved in these original isolated fixture tests.

No four-law body uses root script policy imports, styling builder, general routing/playground/browser APIs, broad normalization apply/plan APIs, or imported node-native/ticket/source-roster/path-statute test collections. Moving just these laws avoids loading that unrelated workspace prelude (:15–45). Importing workspace-contract from a focused file would restore the same cost and is wrong direction.

## Owners and preserved oracles

- scaffold fixture `🧫️fixtures/🏗️mutation-scaffolding/🔣️.json`, schema `🧬️schema/🏗️mutation-scaffolding/🔣️.json`: byte snapshots, dry-run/cancel/failure atomicity and native rustc AST-tree parse :7462 remain.
- inventory fixture `🧫️fixtures/📋️mutation-inventory/🧪️consumers/🔣️.json`, inline INPUT schema :7535, OUTPUT schema `🧬️schema/📋️mutation-inventory/🔣️.json`: source roster fast-glob parity, consumer/helper edges, assignments and byte-change digest remain.
- reachability fixture `🧫️fixtures/📡️mutation-reachability/🔣️.json` and its `🛂️schema/🔣️.json`: all original cases, nativeAccepted or selected original positive rustc compiles :7616–7623 remain.
- type origin fixture `🧫️fixtures/🧬️mutation-type-origin/🔣️.json` and its `🛂️schema/🔣️.json`: virtual/filesystem/Windows link/root ancestor cases, compileAccepted compiler calls, exact origin and wrapped assertions remain.

Current timeouts are scaffold default Bun law timeout, inventory30s, reachability60s, type origin original explicit timeout (retain current source value). The focused owner command budget must retain existing declared command budget; do not replace an early import timeout with a larger deadline. Native compile subcall30s is already authored independently.

## Safe placement and collection

Existing taxonomy convention is domain test collection under `🧪️tests/<specific-emoji-case>/🟦️.ts`, with corresponding fixtures/schema. Correction: `🧹️normalization/🧬️mutation/📇️direct-owner-index/🟦️.ts` is a production provider, not a nested test owner; there is no mutation subtree 🧪️tests collection. Existing library `🧪️tests/🧬️mutation-case-pair` establishes focused domain placement. Recommended: one specifically named mutation scope/consumer test module under the existing library 🧪️tests collection, with the four original names retained. Register its focused execution through existing package 📜️script.ts and Nx/launch ownership, using a domain-specific test target.

Full workspace-contract should import the extracted owner module once; remove the four duplicate bodies there. Other tests currently relying on the three local helpers retain their own shared owner import or helper definition—do not delete helpers from the broad describe unless all their uses move. Extracted module can register `describe("direct mutation ownership",...)` so the original suite naming is retained. Correct import direction is workspace collection -> focused owner; focused owner never imports workspace collection. Keep fixtures in their current canonical locations, adapting path resolution through an explicit library root rather than assuming the old import.meta.dir depth.
