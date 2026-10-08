# General JavaScript and TypeScript Closure Audit — Slot 8

Snapshot: 2026-10-08, current working tree; read-only production audit. Scope excludes both framework product directories, node_modules, target, dist and generated output. TypeScript 5.9.3 AST parsed imports/exports/literal dynamic imports in the actual bodies; esbuild independently bundled selected real entries. No Rust proof is imported into this conclusion.

## Findings

- Parsed 2215 JS/TS/JSON files and 4591 literal module edges. There are **zero direct production module imports into products**, **two executable script imports**, and **one test import**. The kernel TS script imports product-library runVitest at line 3 (two call sites). The Job Rust-package TS script imports product-library runRepositoryTestCommand at line 6 (one call site). Native-build tests import product-owned cargoStreamingStatus at line 8. These are executable/test dependencies, not ordinary application runtime edges.
- Independently refusing all product, s, and hub source loads: the framework TS aggregate router bundles (11 local inputs); the actual kernel and Job routers fail at the product library. This proves a real deletion gap despite the aggregate-router boundary law.
- The General Rust-package Nx project contains **25 command fields invoking product-owned wrappers** (Cargo owner-command and TS owner-command). Additional project inputs reference products in actor (1 string), schema (1) and UI (2); these are build-cache inputs, not runtime imports. Moving Cargo manifests did not close these TS executable paths.
- With import.meta.vitest=false, esbuild bundles framework public runtime (86 inputs), 2D module runtime (8), and 3D public runtime (2), with zero product inputs and no remaining external imports in these selected outputs. The unmodified bundles include test imports/oracles (122 and 4 inputs respectively); their third-party imports cannot all be called production dependencies. These are selected-entry build observations, not universal runtime or all-target JS proof.
- Actual public package manifests still name @semio-tech/s-2d-js and @semio-tech/s-3d-js. Across rg-discovered actual TS/TSX callers, exact AST module specifiers produce **31 declarations across 21 files**, all for s-3d-js: **10 production value declarations**, **8 production type-only declarations**, **13 test/oracle declarations**. There are **21 dependency manifest callers**, all in s. There are zero exact s-2d-js module callers in that discovered set. Descriptions, fixture names, Nx labels, comments and the 2D test-folder import are naming references rather than package API callers.
- The framework TS tsconfig extends repository tsconfig and explicitly maps third-party types under root node_modules. Its include covers package scripts and module source/tests. Framework Vitest config itself uses downward General paths, but runs only six includeSource entries. The aggregate script boundary law examines one router and five registrations; it does not execute all registered child commands, resolve all string-produced paths or inspect every package router/Nx wrapper.
- Product paths occur in source-contract tests, oracle/fixture provenance and JSON schema metadata. Examples include mutation-leaf provenance taxonomyPath, retained scalar ownership schema/JSON, pixels ownership vectors and UI fixture provenance. These need owner-by-owner admission decisions; a refused-owner fixture deliberately naming a product is not a runtime dependency. No unguarded production TS string literal containing a product path was found in the current General AST inventory.

## Ordered Next Execution Slice

1. Rename the actual General package/Nx APIs to neutral names (framework-2d and framework-3d are consistent with existing framework module names); hand-update all 31 AST call declarations, 21 dependency manifests, export metadata, config labels, launch targets, fixture names and lock references in one bounded slice. Resolve exports from manifests: 2D exports ../../🟦️.ts and has no package-local entry file. Do not create aliases.
2. Replace kernel runVitest calls with General Process runVitestV1 plus explicit readVitestPolicyV1. Replace Job runRepositoryTestCommand with the General bounded command/test executor and explicit owned policy. Replace the native-build test's product progress helper with its defining General Process owner or a test-local independent process observation. Preserve progress, cancellation and injected policy.
3. Move the **defining** neutral Cargo/command wrapper into General Process rather than forwarding into product library. Product adapters should supply repository policy. Rewire all 25 General aggregate Cargo Nx commands and the remaining product build-input paths to the defining General owner, keeping native workspace policy and manifests intact.
4. Expand the deletion law from one aggregate router to actual public manifest exports, all General executable routers, resolved Nx command entry paths and material config/resource paths. Use TypeScript AST plus independent esbuild refusal; run commands with product sources unavailable via an interception/closed fixture environment. Keep expected refused-owner strings and oracle provenance separate from executable edges. Include third-party oracle comparison where required by repo law. This audit ran bundling/refusal probes only; it did not run nx test/typecheck or claim runtime command execution.

## Coordinated Resource Work

Root is concurrently moving the complete component-selection-merges and analytic-wire-picking fixtures into General UI Scene and rebinding GeneralMath, Generic3D TS oracle and OSWorld readers. That resource repair is owned by Root; this report makes no remaining-defect assertion against those evolving paths. Root reports whole-General native check3 passed but deletion corpus3 failed on the Specific fixture, so deletion-wide acceptance remains unproven until the repaired corpus is rerun.

## Snapshot Identity

All 8/8 focal files retained the sampled SHA-256 when this report was written. Inventory path/hash digest (sorted path NUL hash lines): 845a312a92c5d02355039ac5b1b191df0374ff7baf788af154056f565a223b47. Detailed machine inventory is temporary ticket-generated output and must be removed at ticket completion. These hashes are actual sampled file bodies, not commit identities; peer work can change them later.

| Source | SHA-256 |
|---|---|
| 🧰️framework/📦️packages/🟦️typescript/🟦️.ts | 55d496d265629efa10a631e4a1f01c5567bc69d30294b804ca55c42494717ec6 |
| 🧰️framework/📦️packages/🟦️typescript/📜️script.ts | ee071a9e86f108bbf40ba913234c7e6e93437ee130fef3623b010ab91b768d7a |
| 🧰️framework/📦️packages/🟦️typescript/tsconfig.json | 49455511bbadff88baf5a0e6c669a7ffb17fb6bda00b466928ecab591a8be977 |
| 🧰️framework/📦️packages/🦀️rust/📋️project.json | 13842967c9cb8e4ef41a4d2b89e8fc1dbf78b359948fb84247eb24a02fe85c31 |
| 🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript/📜️script.ts | da54a3eedf31e097dc97b534ef1235f40c83970ba3c51777c54c8e8d93ab9b16 |
| 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/📜️script.ts | dbf572a74eb95c0ab629cb835b0cc113143d9cb385a44519a63c44eea2a737a3 |
| 🧰️framework/🔨️modules/◻️2d/📦️packages/🟦️typescript/package.json | 8150a9a56862163a24f654c14a4a5529267dcbc3eab670f1320216a8cade89fe |
| 🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/package.json | e949cdc5b60c9c54c30c645f4738f78949bc935a44dd47c93e9f2ea2f1814103 |
