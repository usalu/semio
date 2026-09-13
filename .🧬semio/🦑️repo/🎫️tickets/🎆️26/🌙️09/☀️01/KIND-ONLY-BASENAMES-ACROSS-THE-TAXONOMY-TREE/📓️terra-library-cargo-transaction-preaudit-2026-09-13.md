# Library Cargo and Transaction/Go — Independent Pre-Audit

> **Current status — accepted for the bounded Cargo, transaction-source, and Go-selection extraction.** Historical pre-extraction findings are retained below. The full transaction aggregate remains intentionally unrun because it mutates Git-backed evidence; no global library-router claim is made.

## Scope

The queued slice has three bounded bodies:

- `📚️library/⚡️caching/🦀️cargo/📜️script.ts`, which presently contains generic owned-command execution, Cargo input admission, Cargo artifact capture, component build selection, and the `native` router;
- `📚️library/📦️packages/🟦️typescript/📜️script.ts:11-38` and `:493-703`, which allocate and run the transaction-v2 aggregate harness; and
- `📚️library/📦️packages/🟦️typescript/📜️script.ts:44-63`, which resolves an explicit Go compiler input before delegating to the canonical Go runner.

The package router may retain route registration, literal forwarding, and `runBundleScriptMain`. These three bodies are behavior and must leave it. The ordinary lint route and unrelated named test routes are outside this slice.

## Cargo ownership and live consumers

The present Cargo command source has four separable concerns:

| Current declarations | Semantic concern | Required boundary |
| --- | --- | --- |
| `startNativeProgress`, `runOwnedCommand` | owned-process progress, signal handling, timeout and process-tree retirement | Extend the existing `📚️library/🏃️process` authority. It is generic command behavior, not Cargo-specific API. |
| `validateNativeCargoArguments`, `artifactRustCargoArguments` | Cargo selector admission | Keep the operation/input contract separate from callers' target selection. |
| `buildCargoArtifacts` | private capture target, Cargo JSON receipt parsing, primary-source containment, link-dependency capture and artifact publication | Extend existing Cargo-directory and artifact-staging owners; keep staging and shared compiler intermediates distinct. |
| `NativeScript.run` | command form selection for metadata, component, build, check and test | Retain only routing in `📜️script.ts`; move component manifest validation and operation bodies to their semantic owners. |

`⚡️caching/🦀️cargo/🟦️.ts` already owns Cargo directory interpretation (`cargoDirectories`, target and build roots). It must remain the authority for configured Cargo directories. The new execution API must not duplicate it. The current component branch uses `@iarna/toml` internally; any moved implementation must keep parser values private and expose no third-party parser types.

The historical coordinator inventory recorded twelve direct production consumers. The live import scan now finds fourteen distinct production modules importing `⚡️caching/🦀️cargo/📜️script.ts`:

| Imported operation | Current production consumers |
| --- | --- |
| `buildCargoArtifacts` | Hub Rust command; repo CLI Rust command; cache artifact-Rust router; OS scale fixture Rust command; WGPU native compiler; plugin-descriptor component build; server Rust command; OS MCP Rust command; OS Infinite Rust command; framework replication Rust command; framework pack Rust command. |
| `runOwnedCommand` | cache artifact-TypeScript router; cache artifact-Rust router; UI styling Python builder; UI styling .NET builder. |

Artifact Rust is the one overlap, so this is eleven artifact-build importers plus four owned-process importers across fourteen production modules. The old twelve-file JSON is therefore useful history, not a current complete consumer contract. The two styling builders demonstrate why generic process execution must not stay under a Cargo command API.

There are three separate test relationships:

- `⚡️caching/🧪️tests/📦️publication/🟦️.ts` directly calls `buildCargoArtifacts` for a private Cargo publication fixture;
- `⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` reads and dynamically imports the Cargo source to compare compiler closure, argument admission, progress, target input, and editor-command contracts; after the move it must name the actual owners for behavior while retaining the command router only where it is testing executable command closure;
- `⚡️caching/🧪️tests/🔒️trunk-lockfile/🟦️.ts` invokes the Cargo package command as an actual Trunk hook. This remains an executable router edge, not an implementation API import.

The current retained cache-prune consumer is `🧼️workspace-cleanup/🎮️command/🟦️.ts:66`, which runs the caching router with `cache-prune`. It is a valid executable command relationship. The former concurrent-prune scheduling behavior is absent from the current source and is not part of this extraction.

## Cargo acceptance seams

The focused native control for moved artifact capture is the existing private Cargo publication fixture in `⚡️caching/🧪️tests/📦️publication/🟦️.ts`. It builds an offline temporary Cargo package, compares its executable output with the independently built Cargo output, requires private captured deliverables, and requires shared Cargo intermediates to remain. It is suitable only after owners and source-data consumers are rebound.

The command-source schema/fixture must enumerate every current production importer, the three distinct test relationships, the caching project input, the native route, package forwarding, and launch entries. A post-move native result must distinguish a selected private Cargo receipt/control from a generic Cargo build or a compile-only result. Windows tree cancellation needs a Windows runtime claim only if it is actually run; the current POSIX private descendant control and static `taskkill /t /f` branch are not that claim.

## Transaction-v2 ownership

`transactionV2BundleRoot` allocates an exclusive, no-follow run hierarchy beneath the retained older ticket. The transaction branch then records input identities, builds test and normalization bundles, partitions exactly 62 selected cases, starts detached Bun test shards, captures shard output, tracks PIDs and boundary witnesses, retires descendants, and writes the final receipt. These are transaction verification/process-orchestration behaviors. They require dedicated owner leaves beneath the transaction test concern: one for no-follow run allocation and one for aggregate execution/receipt orchestration. The package router should dispatch `test transaction-v2` to that execution owner.

The current branch hashes the package router as an input (`identityPaths.router`). After extraction, the receipt must instead identify the actual transaction execution owner and any separately owned run-allocation source. It must not preserve a router hash that no longer denotes the behavior under audit. The moved owner, transaction suite, normalization source, taxonomy source, discovery source, and both fixture authorities are all real source/data inputs for the registered target.

### Current blocking source-data defect

The present branch points `identityPaths.ledgerBoundaries` and `identityPaths.harness` at these absent package-local files:

- `📦️packages/🟦️typescript/🧫️fixtures/🔣️transaction-ledger-boundaries.json`
- `📦️packages/🟦️typescript/🧫️fixtures/🔣️transaction-harness-retention.json`

The only current authorities are:

- `📚️library/🧫️fixtures/📒️transaction-ledger-boundaries/🔣️.json`
- `📚️library/🧫️fixtures/🪢️transaction-harness-retention/🔣️.json`

The transaction suite itself reads those two library-level paths. The registered `test-transaction-v2` route cannot be accepted until the harness uses the same actual fixture paths and the source-data contract asserts them. The harness-retention fixture also names `transactionV2BundleRoot`; that source-as-data binding must rebase to the run-allocation owner. `🧪️tests/🛫️preflight-reference-basis/🟦️.ts` separately names the suite path and should stay bound to the suite, not be treated as an execution-owner API.

The existing `@semio-tech/repo-lib:test-transaction-v2` target and its launch entry are registered. They need exact moved-owner and fixture inputs before cacheable acceptance. Do not run it as a read-only probe: it creates retained transaction evidence below the historical ticket root even though it does not authorize a live normalization apply.

## Go input-selection ownership

`GoTestScript.run` resolves an input path, projects a replacement source to its virtual path, derives the selected compiler package, rejects out-of-plan selections, then invokes `runCanonicalGoTests`. The canonical Go implementation already owns recursive replacement discovery, overlay construction, tool invocation and cleanup in `📚️library/🟦️.ts:1270-1316`. The extraction should add a narrow Go input-selection owner for the package-router-specific realpath/lstat/projected-package admission, importing `canonicalGoPlan` and `runCanonicalGoTests` directly. It must not copy the canonical planner or create a second Go overlay implementation.

The correct focused controls are:

- `test-go-input-projection` for portable module/replacement/package selection vectors;
- `test-go-dispatch`, whose current test invokes a selected native Go dispatcher and includes cancellation/spawn-error/temporary-overlay cleanup boundaries; and
- the `go-test` project/package route with its existing literal forwarding, retained as routing after the selection owner moves.

These prove a selected compiler-input path only. They must not be presented as a full Go workspace test or service acceptance.

## Required post-move closure

1. Create schema-first source/owner/consumer fixtures before moving declarations. Use the current fourteen production Cargo importers rather than the historical twelve-file set, and classify generic process imports separately from Cargo build imports.
2. Rebind the three Cargo test relationships by meaning: private artifact publication to artifact capture, cache-contract source-data rows to the actual owners plus router where command closure is tested, and Trunk to the retained router command.
3. Repair transaction fixture paths before any direct or Nx transaction route. Rebind input identities, target inputs, source-as-data fixture fields, and launch entry to actual moved owners.
4. Keep Go selection separate from `canonicalGoPlan` and `runCanonicalGoTests`; verify selected package admission and native dispatcher cancellation through the existing safe fixtures.
5. Preserve project/package/launch routes and add exact source inputs for every behavior or fixture read by those routes. Use ticket-private native artifact roots only for a deliberate Cargo or Go control.

## Owner-plan review

The executor's current nine-owner plan is consistent with this audit. It places generic owned execution under the existing process domain; keeps Cargo admission distinct from receipt capture/build publication; leaves native command selection as a narrow orchestration concern; separates transaction allocation, provenance, shard execution, and result orchestration; and adds one Go input-selection owner that delegates to the existing canonical planner/runner.

The plan carries the current fourteen-module production closure, the publication/cache-contract/Trunk relationships, the absent-fixture repair, and the cache-prune boundary. Its provenance control must identify all four moved transaction owners together with the transaction suite, normalization, taxonomy/discovery, and both actual fixture authorities. A residual package-router hash would be insufficient once the behavior moves.

## Limits

This is source and consumer evidence only. It does not establish a current Cargo receipt, component build, Go command, transaction result, Windows cancellation behavior, or full normalization transaction. The transaction fixture defect is a current route blocker, not evidence of a historical baseline.

## Current source checkpoint — extraction in progress

The nine planned owners now materialize in the working tree. Cargo delegates `native` to `⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts`; for this extracted slice, the library router imports transaction orchestration and Go selection owners. This is not a claim that the whole library router is thin: unrelated Workspaces and ticket-FEM behavior remains in it. Transaction orchestration obtains its ledger and harness through `transactionV2IdentityPaths`, which points to the existing library-level fixture authorities rather than the previously absent package-local paths. The Go owner imports and reuses `canonicalGoPlan` and `runCanonicalGoTests`.

The current portable fixture records all 14 distinct production consumers, including the artifact-Rust overlap and the full-repository Hub consumer. It keeps the former scheduled cache-prune behavior absent. I did not run the new route.

One pre-acceptance repair remains necessary: the 12 `contexts` entries are standalone taxonomy mappings, and the source test validates only those mappings. They are not associated with individual owner paths or checked as a full ancestry sequence. An unrelated set of valid kind mappings could therefore satisfy the evidence. The fixture must bind each owner, or owner family, to the actual directory chain before its context count supports the claimed taxonomy closure.

## Final bounded acceptance

I independently re-read the finalized nine-owner fixture, schema, source test, two package routers, all 18 direct/source-data consumer records, project/package/Nx/launch closure, and current transaction provenance. A separate read-only TypeScript analysis found all nine declared symbol sets at their stated owner paths, all twelve context chains bound to those paths, seven owner-internal import edges, and zero owner-to-command-router edges.

The repair uses the right boundaries: generic cancellation/progress stays under process execution; Cargo admission, build capture, and native operation routing are distinct; all fourteen production consumers are represented, including the full-repository Hub and the artifact-Rust overlap; transaction allocation/provenance/shard/orchestration are separate; and Go input projection reuses the existing canonical planner and runner. The prior package-local transaction fixture references are gone from the execution path in favor of the two library-level fixture authorities.

The final executor report supersedes the earlier 10/248 checkpoint: direct source is 10/270 in 5.32 seconds; the ordinary package route is 10/270 in 3.75 seconds; and isolated cache-skipped `@semio-tech/repo-lib:test-cargo-transaction-command-source` is 10/270 with a 4.27-second Bun body and 4.6-second Nx target. Separate private native publication, native-preparation dependency closure, Go projection/dispatch, and Cargo compiler-closure controls are recorded in `📓️sol-library-cargo-transaction-extraction-2026-09-13.md`; they are not an all-Cargo or all-Go build claim.

I accept the ownership, taxonomy, API-consumer, source-data, cache-input, routing, cancellation-control, and selected Go-admission closure. I do not accept or claim a full transaction result: `test-transaction-v2` deliberately remains unrun because its harness creates Git-backed transaction evidence. Windows process-tree termination remains static-source coverage only; no Windows runtime result was supplied. The whole library router still contains unrelated Workspaces and ticket-FEM responsibilities outside this nine-owner slice.
