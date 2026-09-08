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

## Continuation Verification

The previous goal turn made source progress: Block/GIS parent compositions, a typed DSL floating-value round-trip correction, and the independent final static audit. Automatic continuation ended the old process handles; an authoritative process listing confirmed that those owned jobs were no longer running. Their output did not prove completion. The first Block/GIS and corrected Playbook attempts rejected a concurrently stale Cargo.lock. Full offline Cargo metadata has now refreshed the current graph successfully, and the two gates are being rerun.

Current root gates: Nx Flow/Playbook tests (`framework-flow-playbook-final-nx-test.txt`) and Block/GIS component compiler (`block-gis-composition-check-2.txt`). Framework host/Space integration remains required. DAG retirement integration and norm followups are assigned to the Sol registry executor; 17 multi-artifact leaves/remaining five parent facades to the Sol artifact executor; 24 single-artifact leaves, Nx cache/runtime contract and nested Draw package ownership to the Sol Nx executor.

The Flow/Playbook Nx test run reached its task wrappers but both were cancelled by a newly introduced 180-second command deadline while waiting for the shared Cargo target. These are orchestration failures, not test results. The Nx executor restored the existing buildBudgetMs semantics (default unlimited, opt-in SEMIO_BUILD_BUDGET_MS) while keeping progress and cancellation. A fresh runtime gate is required.

## Current Runtime Integration

The host check compiled the cold WGPU/Wasmtime dependency stack, Flow data, DAG data, and both Space data packages, then failed solely at a newly introduced schema-registry import whose dependency had been added after Cargo loaded its graph. Current manifest declares that edge. Retry is `framework-host-integration-check-2.txt`. Block/GIS check 4 hit an empty JSON read in mutation source-authority macros during concurrent taxonomy edits; both authority documents currently validate. Keep validation enabled and retry current graph. Root process handles now: runtime four-package diagnostic 92905, host check 44429, Block/GIS check 32492.

Two narrow shared compiler corrections preserve concurrent work: the new attach_hot_backbone body belonged to ArtifactStore but had been inserted into SpaceHost; it was moved unchanged next to attach_backbone. In the new plugin backbone binding, dsl already reexports the derive macros, so the duplicate macro import was removed, and FromValue receives the owned decoded value. Files: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` and `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🦀️.rs`. The subsequent UI-contract schema-registry failure was another graph captured before its declared dependency; current manifests already contain it. Current combined host/Block/GIS check is `framework-block-gis-host-check-3.txt` (34741); current four-artifact runtime check is `framework-four-artifacts-runtime-test-3.txt` (41171).

## Current Integration Gates

Host integration check 4 compiled the Flow host and reached the Home artifact, reporting 18 missing direct component dependencies. The artifact executor repaired those Home dependency selections after that run; a fresh host integration command is required.

Block/GIS parent runtime compilation was deliberately cancelled with exit 130 when free disk fell to 2 GiB. This is not a test result. Only that owned Cargo command was interrupted. A cleanup process acquired Cargo's exclusive build-directory lock and reclaimed the ticket's incremental cache; compiled dependency outputs are retained. Subsequent commands disable incremental compilation for this validation target to prevent repeated source variants from exhausting local disk.

The owned incremental cleanup completed under Cargo's exclusive build-directory lock, increasing measured free space from 2,067,554,304 to 38,844,026,880 bytes. Parent runtime retry 2, Home/OS host integration retry 5, and the seven current framework Nx test identities are now queued/running with incremental compilation disabled. GIS default leaf tests are queued separately.

A fresh read-only repo MCP tools listing confirms that ticket association can be changed only through `ticket_reopen`; `ticket_close` accepts summary/files but no goal. The completion sequence must therefore close the completed work, reopen with the verified full repository-project goal id to correct the initial shortened association, and close that metadata correction through MCP. Do not modify the ticket JSON directly or close the ticket before validation completes.
