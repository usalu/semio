# Repository Test and Cache Script Extraction

This queued Sol slice follows the expanded script audit. Root inspected current definitions, imports and source-as-data consumers on 2026-09-12. Use separate concern owners; preserve concurrent source work and existing native host/cache semantics.

## Repository Test Coordinator

The actual command source is repo/modules/🧪️test/📜️script.ts, approximately 1,662 lines at the original review. It imports the already-neutral test contract/library but still implements source/baseline/coverage reads, case and implementation selection, fixture/coverage selectors, native host materialization for five languages, interpreter provisioning, execution phases, parity policy, metrics/report projection, fixture publication, matrix/gap/schema operations and source inventory.

Move these substantive APIs and class-method bodies into anonymous leaves under the existing test domain's selection, host/materialization, execution/parity, reporting and fixture/schema concerns. The command file retains routing and imported calls; do not move the whole script to one generic runner. Keep actual planned-scenario host implementations distinct from the coordinator that builds and invokes them.

The active manifestless lane has moved Python and .NET host source beside the existing TypeScript host under 🧪️test/🖥️host. It retained the Rust root declaration facade over runner/protocol concerns. Re-read those exact paths, source ownership fixtures, Cargo/csproj selectors and native support before further edits. Do not restore package-owned hosts or duplicate host contracts.

A current source-as-data test at 🧪️test/🧪️tests/🧪️test-platform/🟦️.ts:1145 reads the command file and extracts rustHostExecutableFromCargo and executeOne with installed TypeScript. It proves Cargo's actual executable is resolved under the build budget before the scenario budget begins. Rebase the assertion to the actual owners and preserve that ordering and compiler oracle; do not merely delete the check. The module's exported policy() also contains behavior and needs its true verification owner.

Keep oracle/subject distinction, owner-contributed registries, admitted fixture identity, production-dispatch witnesses, plan/result protocol hashes, native toolchain package boundaries and finite execution/cancellation. Reuse current domain-neutral test interfaces. No registry rewrite or invented missing oracle behavior to force parity green.

## Cache Audit and Artifact Contracts

The source is repo/modules/📚️library/⚡️caching/📜️script.ts, approximately 583 lines at review. Existing owners already include inventory, artifact registry/staging, resource leases and pruning. Extend those actual domains where appropriate.

The exported inventory() implements complete Nx project/command/artifact discovery, explicit outputs and dependency-consumer joins, and policy findings. Its sourceFiles and ticketOutput dependencies are real traversal and ownership behavior. Remaining class bodies implement audit/report/graph/disk/doctor verification, cache-area scans, pruning plans and artifact-package contract capture. Put behavior in the corresponding anonymous owners; do not duplicate the existing readInventoryGraph, artifact registry or prune planner APIs.

SCRIPT_ROOT currently anchors 🔣️policy.json and the 🚀️bootstrap/📜️script.ts exception. Rebase each actual semantic or executable referent explicitly after extraction. Root CleanScript invokes cache-prune as an executable through runCmdStatus; that command reference remains valid and is not an API facade. Root's cache-prune delegation must keep its existing task route.

Cache contract tests at ⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts receive an explicit dependency object and inspect policy-owned source/input sets. Preserve their meaningful boundaries and update exact behavioral source owners. Read-only audit/report is sufficient for live inventory checks; validate destructive pruning against private fixtures and plans, not the shared work fleet's caches. Never reset shared Nx or run broad cleanup merely as a taxonomy test.

## Native Cargo Artifact Implementation

The adjacent ⚡️caching/🦀️cargo/📜️script.ts has reusable startNativeProgress, runOwnedCommand, validateNativeCargoArguments, artifactRustCargoArguments and buildCargoArtifacts. Their implementations and native process/artifact semantics belong to anonymous process/build/publication concerns; command routing remains in the mandatory script. Inspect existing ⚡️caching/🦀️cargo/🟦️.ts and library/🏃️process owners before adding parallel abstractions.

A current symbol screen found fourteen files. Installed TypeScript AST inspection then confirmed twelve direct consumers of the Cargo command API (excluding its own source and a test reference): artifact Rust/TypeScript routers, repo CLI, framework pack/replication, OS MCP/Infinite/scale, server product, Hub, WGPU and plugin descriptor. Exact resolved import clauses are retained at generated/coordinator/cargo-script-api-consumers.json. Rebase the complete live import list. The WGPU/root artifact executor is active on one of those consumers; use its newly moved owner when closing this API, rather than editing a stale package script snapshot.

Preserve build receipt parsing, link-dependency staging, source-project containment, compiler argument validation, stdout progress, bounded process groups, Windows descendant cancellation and final cleanup. The detached throttled cache-prune launcher has a source-relative command path; rebase its actual executable after moving the implementation. Use current process infrastructure behind owned types where semantics match and prove cancellation/runtime behavior with private child fixtures. No new runtime dependency or API exposing third-party types.

## TDD, Runtime Evidence and Registration

For each group, write a portable source/owner/consumer contract before extraction, using installed TypeScript/native host oracles and existing behavior fixtures. Run focused direct and registered Nx tests plus the actual native host/build command appropriate to changed selectors. A compile with zero tests is compile-only evidence. Preserve every ordinary project/launch route and the root Bun-to-Nx bootstrap path; derive launch from its seed.

Retain exact changed paths, complete ancestor registration, meaningful test/native output and current limitations in a ticket Markdown report. Every generated artifact/log/private native project belongs under a lane-specific ticket 🗑️generated directory and should be deleted after retaining evidence. Keep handcrafted inputs and reports. No modifying Git/worktrees/AGENTS or ticket/goal lifecycle changes.

## Adjacent Repository Library Test Commands

A fresh read of repository library 📦️packages/🟦️typescript/📜️script.ts shows that most of its roughly 640 lines are ordinary named test routing, but two concrete domains remain embedded. This is not a claim that every test route must be extracted.

The exported transactionV2BundleRoot allocates and validates a no-follow run/bundle hierarchy under the older normalization ticket. The TestScript transaction-v2 branch then owns input identity receipts, native Bun child bundling, an exact 62-case filter partition, per-shard output retention, detached process groups, PID/boundary registries, descendant retirement, output error handling and completion aggregation. Move that harness to its actual transaction verification/process ownership concern, leaving its command dispatch and raw argument forwarding in the router. Preserve the existing fixture/filter cardinality, separate process cancellation and captured source authority; changing the root source identity from a router to the moved implementation must preserve audit meaning rather than keep an irrelevant router hash. Inspect downstream source-as-data tests before rebasing. This move does not authorize running live transaction apply or deleting retained prior-ticket reports.

The same script's GoTestScript performs canonical module/source-input selection itself: realpath projection, replacement lookup, file/directory inspection, package membership and containment. Move that behavior alongside the existing canonical Go discovery/test-dispatch owner and retain package/input selection and shared native cancellation behavior. The ordinary LintScript and most TestScript branches simply forward literal paths and argument slices to imported owners and are legitimate routing. Preserve them without a whole-file size-based exception.

The repo module planned-scenario coordinator and shared Cargo cache API remain the principal slice above. The library transaction/Go command behaviors may be a subsequent bounded lane if their process/native test scope would overload that extraction. Keep exact ownership, call-site and native validation evidence separate.
