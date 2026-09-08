# Coordinator Integration Review

## Review Findings During Implementation

- TypeScript runtime exports must be package-local generated files because Node forbids `../..` export targets. The implementation source stays at the artifact taxonomy root.
- Bun transpilation alone does not type-check or emit declarations. The existing public `ArtifactDefinition` type must survive package extraction, and a runtime smoke test must resolve the package name itself.
- Actual Cargo dependency edges, including transitive paths, must prove the absence of a dependency back to stdio composition. Schema registry dependencies are a distinct semantic graph and cannot substitute for build-graph validation.
- Nx graph edge presence does not demonstrate precise source hashes or cache invalidation. Validate the artifact ownership boundary and repeated task behavior separately.
- The initial temporary extraction script placed existing Rust inner documentation after imported items. The observed binary source now correctly keeps its inner documentation first. No invalid `$semio_s_artifact_stdio_` macro substitution was found in the source scan performed at this review point.
- Manifest-relative test paths must be adjusted for the new package depth. Compile-time source-relative includes remain attached to their unchanged domain files.
- Cross-artifact dependencies must be inferred from mounted code rather than comments, strings, and unmounted third-party oracle sources. Visibility changes should expose only the helpers required across actual package boundaries.

These are ongoing source review observations, not passing compilation or runtime results. The executing agents own the corresponding fixes and verification.

## Existing Tooling That Must Follow The Split

The old stdio package `HomeIoSurfaceScript` asserted empty Cargo feature sets, artifact modules mounted in the package-local facade, and blanket full-catalog dependencies in consumers. Those structural assertions must change to actual selected package closures while preserving the behavior guarantees for Home I/O and native catalog authority. DWG/source import scans also need the new direct crate names. Existing directory-wiring commands were inspected: they repair directory identities and do not regenerate the full Rust facade.

The new graph and Cargo metadata checks should use a controlled async process lifecycle, rather than opaque multi-minute synchronous subprocess calls. Cancellation, bounded execution, and diagnostic output are required for these expensive operations.

## Other Consumer Guards

Flow editor fixture and note action-cohort source assertions now use direct stdio-semio crate imports. The process STEP export test registers only STEP format descriptors. Hub headless metadata proof has exact source/feature expectations requiring adjustment when composition facade features land; nx executor owns that follow-up. Lowpoly/puzzle wasm-dev optimization profile entries must follow editor code into extracted crates to retain the established interactive budget.

## Generator Prerequisite Failure

The two rejected generator targets currently call domain module scripts directly (`print-design-token-paints/📜️script.ts preview-generated`, `actor-network/📜️script.ts akteursnetz preview`). The existing taxonomy validator insists on the owner package router command `bun ./📜️script.ts preview-generated`. The owner routers currently register tests/fonts only. This is outside artifact extraction, so the package work preserves these concurrent sources and records the failing prerequisite explicitly while diagnosing the bounded package check independently. Final full Nx verification must revisit this prerequisite.
