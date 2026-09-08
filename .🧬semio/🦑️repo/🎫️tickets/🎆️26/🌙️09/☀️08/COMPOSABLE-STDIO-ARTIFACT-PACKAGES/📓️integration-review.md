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

## Generated Evidence Loss During Active Checks

The shared ticket generated directory was cleared while root and all three execution agents had active commands. Only its Cargo directory remained; source files and durable Markdown reports were intact. Every fleet executor confirmed they had not removed the common generated directory. The coordinator's earlier intentional cache cleanup removed only Cargo's incremental subdirectory while holding the Cargo build lock. The cause of this later deletion is unconfirmed.

Raw logs, isolated Nx workspace data and proof caches were lost. Existing documented completed outcomes remain recorded; pending commands must not be reported as passing without their exits and results. Future root validation streams through both the tool and ticket logs so that deleting a log path does not erase the only copy of diagnostics. Warm reruns are justified where the lost output prevents assessing a result.

## Compiler Lock Recapture Follow-Up

Read-only lsof showed that the generated-output removal also replaced Cargo’s `.cargo-build-lock` inode (old pending commands held 164256927; the new recapture held 165889246). The coordinator cancelled only its newly started recapture Cargo PID 52621 to let the old queue finish before starting additional native gates. No compiler or lock files were removed. Agents were asked to hold new native starts until the old queue drains. This interrupted recapture has no runtime test result.

## Queued Native Verification Restart

The coordinator inspected the exact process tree for its seven-framework Nx run: PID 93572 owned Bun 96706 and queued Cargo 96708, with no compiler children. Sending SIGINT only to Nx 93572 terminated all three; the tool returned exit 1 for the interrupted run. No new runtime result is attributed to that invocation. The artifact executor was asked to similarly cancel its queued old-inode check. The actively compiling host check PID 87785 remains running; once it exits, fresh native acceptance gates can share the current lock epoch and capture their diagnostics.

Full `cargo metadata --offline --format-version 1` completed with exit 0 after the latest target/dependency edits, resolving 230 workspace members. This was graph/lock validation, not a native compile. The two explicit Norm integration targets are present in the resolved target graph.

## Shared Source Prerequisite Parser Findings

The Space source-only Nx run reached its four targets but failed while parsing the shared fresh-component test source, before any Space oracle result. A targeted scan found nine malformed `import.source.directory`/`import.source.url` expressions across three plugin test files (runner-self-tests, browser-bundle and describe/fresh-component). The Nx executor owns narrowly repairing these expressions against the moved source-context contract and rerunning the affected prerequisites; no source/runtime pass is inferred from the blocked invocation.

## Embedded Rust Asset Inputs

The coordinator inspected the current native source collector for fixture assets such as TSV, GraphQL, protocol and TypeScript declarations embedded with `include_str!`. It lexes include/module references, resolves literals and `concat!(env!("CARGO_MANIFEST_DIR"), ...)`, and falls back to the complete owner tree for dynamic/generated module cases. Native cache inputs therefore do not depend solely on a narrow file-extension glob. This is a source inspection, not an additional runtime/cache mutation gate.

## Conservative Native Feature Closure

The current Nx native prerequisite/input graph includes all declared optional local dependencies, while Cargo itself selects features for the actual compile. This avoids stale task outputs when a wrapper enables an optional feature, but means default Semio Nx tasks can be invalidated by inactive optional-format sources. The proven PDF/JPG isolation is valid because PDF’s declared dependency closure is narrow. No claim of exact inactive-feature cache exclusion is made.

## Space Directory Source Oracle Diagnosis

The Directory schema declares draft-07; its two-item tuple array with additionalItems=false is valid for that draft. The Home source oracle incorrectly copied its definitions into an Ajv2020 validator, which rejected the draft-07 tuple declaration before running the selected fixture. The artifact executor owns correcting the caller to the existing draft-07 Ajv validator, registering receipt custom keywords before compilation, and replacing a remaining deleted parent crate-root read with the Home artifact root. No shared schema conversion or compatibility layer is needed.

## Current Native Queue Resume Plan

The sole remaining owned old-lock-epoch Cargo command is host check PID 87785 (tool session 44071), checking OS, Flow, Infinite and Space with the OS host feature. Its redirected raw log was unlinked during the generated-output incident, so only a successful exit can establish this gate without recapture; a failing exit requires a warm diagnostic rerun with `pipefail` and `tee`.

After that process exits, all owners may use the current shared ticket target and lock epoch. The coordinator will run the two GIS defaults, their component features, Block/GIS composition library tests and GIS native-codec integration, then current canonical framework artifact Nx targets. Artifact execution owns the multi-artifact and stdio feature matrix; Nx execution owns the 24 single-artifact native targets and normal native prerequisite gate; Registry execution owns Norm integration. All new Cargo checks disable incremental compilation for this bounded acceptance run. No agent should finish its task while holding an unresolved native tool session.

The GIS `component_cold_map_patch` integration target additionally consumes a receipt-bound compiled WASI component through environment variables. Its include and Cargo target paths were repaired; the native-codec integration and parent/library tests are the direct packaging acceptance gates. No standalone cold-component runtime result is implied by those native gates.

## Artifact Router Argument Review

A focused read of the shared native builder found that the new artifact routers forwarded build/check/test arguments directly, while the generic native router called `validateNativeCargoArguments`. Workspace/package/manifest selection could therefore escape a leaf task's declared source contract. The Nx execution owner is applying the same validator to all three artifact operations (after stripping the test-level selector) and recording a focused regression. The same review requested build progress during silent Cargo lock waits, matching the existing check/test ten-second progress and process-tree cancellation behavior.

The artifact-router review is resolved: all three operations apply the argument validator, test-level stripping precedes validation, and Cargo build reports periodic progress. The execution owner recorded failing-before/passing-after receipts and a language-neutral schema fixture checked by the independent `jsonschema` implementation. The focused native-preparation contract passed without invoking a compiler.

The old host check (session 44071 / PID 87785) exited 101. Because its raw diagnostics had been unlinked, this establishes failure only. The coordinator immediately started `framework-host-integration-recapture-6.txt` with the same package/feature selection, current ticket target, `CARGO_INCREMENTAL=0`, `pipefail` and `tee` (session 30088), before releasing other native commands. This recapture retains both tool output and its ticket log.

After confirming the old lock epoch was gone and recapture session 30088 held the current lock first, the coordinator released the agent native queues. Norm, artifact and Nx native gates may wait on that current lock and proceed automatically. The root GIS default runtime recapture (session 11127, `gis-default-runtime-recapture-6.txt`) selects only the map/terrain libraries with no default features and no fail-fast; its result is pending. Native PDF cache isolation is being checked separately from the completed TypeScript proof.

The artifact execution owner completed the final stdio composition mount cleanup after recapture30088 started: the Cargo library now points to taxonomy `../../🦀️.rs`, which mounts intact assembly source at `🔌️plugin/🦀️.rs`; the package-local wrapper is removed. This preserves public module identities, but a Cargo graph resolved before that move can retain the old library source path. Any such failure requires a fresh graph and is not proof of a current source defect. Owners were instructed to finish manifest/mount moves before final native acceptance and rerun the repository package contract on the final path.

The coordinator started the seven current canonical framework artifact Nx test targets in session 36514 (`framework-seven-final-nx-7.txt`), with installed `bun x nx`, `NX_DAEMON=false`, unique `nx-root-framework-final-7` workspace data, serial targets and the current ticket Cargo target. `--excludeTaskDependencies` intentionally selects these already independently exercised artifact tests; the separate normal Forms Nx path provides native prerequisite-chain acceptance. Initial inspection found the owned repository graph worker actively computing, so it was left running. No runtime result is claimed yet.
