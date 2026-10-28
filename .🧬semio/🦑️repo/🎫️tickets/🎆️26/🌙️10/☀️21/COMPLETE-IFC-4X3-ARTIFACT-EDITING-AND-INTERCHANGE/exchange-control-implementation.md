# Spatial And Analytical Original Control Checkpoint

## Implemented Behavior

Both public BIM exchange directions, both FEM directions, and `extractIfc4x3Triangles` now capture the original signal, callback and finite control configuration through the existing shared `captureIfcIoControl` interface before any caller callback. Import and triangle entry checkpoints run before graph index construction or semantic inspection. The FEM import's duplicate initial checkpoint was removed; normal later progress remains. An aborted original operation still throws `io.cancelled` even when its callback replaces the caller's options with a fresh signal and callback. It does not publish an unsupported/partial outcome instead.

The triangle product loop now admits geometry interpretation only for released `IfcProduct` subtypes through the complete IFC4x3 catalog. A non-product with a shape-looking argument six no longer receives geometry. Its actual tessellation remains visible in the existing unbound-item diagnostic. Classification is independent of MVD badges and does not pretend to validate a complete intentionally damaged test entity.

## Test-Driven Evidence

The new closed `exchange-control-cases.json`/schema and feature file cover all five endpoints. Before the production fix, the suite produced 1 pass and 10 failures: every cancellation replacement case resolved instead of rejecting, and every invalid-control case bypassed control admission. After the fix, all 11 tests and 31 assertions pass. The reusable actual drivers live in `exchange-control-fixtures.ts` rather than being copied into the ticket.

The declaration-based extraction regression first reproduced a non-product mesh. Its ten neutral classification cases now pass, including four product families and six non-product/unknown cases. The combined control and glTF suites pass 17 tests and 89 assertions with zero failures. The original official fixture bytes are unchanged; intentionally altered in-memory entities exercise only classification, source preservation and unbound diagnostics. `IFCPROJECTLIBRARY` represents a non-product context; using a second `IFCPROJECT` would instead trigger the independent exactly-one-project unit admission before this test's target.

The new control suite is explicitly registered in the artifact package, including in the retained registration runner's selection. TypeScript checking of the wider BIM/FEM-dependent graph currently stops at the existing missing `@semio-tech/framework-interaction` module in shared replication code. No build repair, missing-module stub or weakened compiler option was added. Bun executes the actual changed graph; the isolated template suite remains strictly checked.

## Independent Implementations

`geometry-product-export.ts` exports the actual TS predicate for every declaration; `geometry-product-oracle.py` compares it and the neutral cases with IfcOpenShell 0.9.0.post1's independent supertype traversal. All 876 declarations, 214 product declarations and ten cases agree with zero mismatches.

`exchange-control-oracle.ts` compares each actual endpoint's normalized cancellation outcome with the original pinned [p-retry implementation](https://github.com/sindresorhus/p-retry). Both retain original cancellation when the caller replaces its signal during operation execution. Five endpoint comparisons agree: cancelled, exactly one original callback, no replacement callback and no published result. This oracle validates the shared cancellation boundary, not IFC/BIM/FEM conversion semantics.

The test-only acquisition script pins p-retry 8.0.1 and is-network-error 1.3.0, verifies official npm registry SHA-512 integrity, retains original package files and MIT licenses under `🗑️generated/ctrlmod`, and records SHA-256 provenance. Package tarball hashes are `e4dda962f036a7ccab86d4bba82f8aad0a0b77d041ec447462815cdee87205eb` and `ab085e1dab3a8d73cc6b17f7f522ced9a11c6e0720ebcdd3a0f6cc6b63a04d35`. No package manifest, runtime dependency or exported production API depends on either oracle package.

## Remaining Boundary

This fixes original control custody and early admission. It does not convert cold graph constructors, repeated whole-population filters or all nested geometry traversal into original-native retained work, enforce every entity/value limit throughout every projection, expand all BIM/FEM mappings, or prove native scheduler/heap behavior. The native exchange worker owns the original retained production routes and tests. Broader mapping, guided forms, complete MVD constraints and authentic native execution remain open goal work.
