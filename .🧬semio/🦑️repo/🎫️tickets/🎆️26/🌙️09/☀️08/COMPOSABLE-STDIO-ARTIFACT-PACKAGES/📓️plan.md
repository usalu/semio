# Composable Artifact Packages

## Objective

Give every production artifact an independent package boundary and Nx build identity, starting with stdio. Package declarations belong under each artifact’s `📦️packages` language directory; implementation stays in the domain taxonomy. Preserve existing behavior and concurrently edited source.

## Fleet

The main coordinator runs GPT 6 Astra at Extra High. Read-only exploration uses multiple GPT 5.6 Terra Extra High agents. Execution will use multiple GPT 5.6 Sol Extra High agents after exploration, followed by independent Terra audits. The session permits four total concurrent agents including the coordinator.

## Work Sequence

1. Read `repo://goals`, inspect existing tickets, open the shared repo MCP ticket (done).
2. Map source ownership, cross-artifact dependencies, Cargo/Nx graph, launch routing and existing tests.
3. Define a language-neutral package contract and regression tests before changing build ownership.
4. Extract artifact crate ownership into taxonomy roots, retaining declaration-only package directories and acyclic local dependencies.
5. Integrate Cargo workspace, Nx cache inputs/outputs and dependencies, Bun command routing and launch configurations.
6. Run package contract tests, selective compilation, representative existing behavioral tests, and independent audits. Verify cache behavior and actual runtime logs where supported.
7. Record validation and remaining limits, remove generated ticket outputs, close the repo ticket and complete the task goal only when all required work is done.

## Initial Evidence

The stdio package currently mounts artifact implementation into one large Rust crate. The repo’s custom Nx plugin already discovers Cargo declarations and domain ownership outside package directories. Further findings are recorded in the individual audit files.

## Infrastructure Notes

The repo MCP server was accessed over its existing stdio binary because this session did not expose repo MCP tools directly. Its model catalog rejects `gpt-6-astra`; the generic `codex` metadata value was used and the actual requested models are recorded in the ticket prompt. No external management issue was created.

## Settled Package Boundaries

- Artifact Rust packages: `semio-s-artifact-stdio-<artifact>`, Nx `@semio-tech/stdio-<artifact>-rs`.
- Artifact TypeScript packages: `@semio-tech/stdio-<artifact>`. Runtime exports must resolve within generated package output; Node does not allow an export target to escape the package root. Build sources remain at artifact taxonomy roots.
- Shared lower contract: `semio-s-artifact-stdio-contract`, sourced from `📇️registry/🧬️contract`, with no artifact or component dependency.
- Stdio retains component aggregation, selected catalog validation, and native receipt authority. Artifact APIs move to actual artifact crates; external consumers will use direct package dependencies.

## Execution Ownership

`artifact_execution` owns Rust extraction, artifact manifests, workspace Cargo registration, and thin component assembly. `registry_execution` owns the shared contract and registry split. `nx_execution` owns schema fixtures, package verification, Nx declarations, Bun/TypeScript packages, scripts, and launch integration. All three run GPT 5.6 Sol Extra High concurrently. The coordinator inventories consumers, reviews integration contracts, and coordinates the follow-up migration and validation wave.

## Repository-Wide Completion Scope

“Every artifact” applies to all first-party artifact owners. Stdio is the first execution pass because it motivates the compile cost and exercises the reusable packaging mechanism. The current source inventory contains 93 artifact roots (36 stdio and 57 elsewhere), recorded in `📓️repository-scope.md`. After stdio boundaries and direct consumer dependencies work, continue across the remaining artifact owners in parallel waves and extend generic package validation. Do not mark the user’s goal complete after the stdio pass alone.

## Remaining Rust Extraction Assignments

After their bounded stdio validations, the three Sol Extra High execution slots continue with explicit Cargo/source/Nx ownership:

- `artifact_execution`: procedural (3), GIS (2), FEM (2), trinity (2), puzzle (3), block (3), space (2): 17 artifact roots.
- `nx_execution`: writer, mathematical, flow, VCS, animate, shooting, demonstrator, sequence, architect, process, lowpoly, reasoning, forms, layout, CAD, playbook, imperative, remodel, energy, DAG, draw, raster, note, sourcing: 24 artifact roots. This transfers Rust/Cargo ownership as well as declarations; the earlier Nx-only boundary applied to stdio. Also owns neutral package script/contract/launch integration.
- `registry_execution`: norm (15) and framework workflow/run (1): 16 artifact roots.

Root coordinates and reviews, updates external consumers once crate APIs settle, validates final integration, and preserves concurrent unrelated edits. All root Cargo mutations are small, current-state edits coordinated among executors; no generated whole-file overwrite from stale snapshots.

## Schema Ownership Completion Pass

The initial 93 roots were a directory inventory, not a complete schema audit. Root additionally extracted the standalone framework `os.workflow` document and shared PlaybookSpec/generation document; workflow/run was part of the initial inventory and is now independently compiled and tested. Registry execution owns subsequent `os.space` and `os.collection` extraction after its norm gates. Framework Flow and Dag serializers must be classified in the final audit: they already reside in dedicated framework Flow and Infinite packages, respectively, and their reusable engine/core types are distinct from the plugin artifact composition wrappers. Test-only DslArtifact types and Store's private durable decision records are protocol/test support and do not automatically define separately installable artifacts.

Current execution ownership: artifact_execution finishes stdio default/selected/full conversion gates and the 17 multi-artifact roots, including Block TypeScript packages and a lower Trinity engine API; nx_execution finishes its 24 artifact compile refinement, VCS/writer plugin dependency removal, package contract, Nx cache restoration/invalidation and dynamic launch inventory; registry_execution finishes norm15 then builtin space/collection. Root owns direct consumer imports/manifests, precise Semio conversion feature declarations, workflow/run, workflow/workflow, shared framework Playbook and integration validation.

All Cargo/compilation results in reports refer to actual executed commands. Concurrent edits require a final fresh resolution and audit before completion. Shared Nx graph waits are isolated for verification using the installed Nx binary via `bun x nx` and ticket-owned workspace data; shared worker processes are preserved.

## Final Inventory and Current Ownership

The complete audited scope is 99 production artifacts plus 2 shared contracts, totaling 101 Rust declarations; 39 existing TypeScript entry points are declared separately. The initial 93-root count above was superseded by schema ownership exploration. Root now owns both GIS leaves and the GIS/Block parent composition boundaries alongside all framework artifacts except Space/Collection (registry executor). The artifact executor owns the remaining 15 multi-artifact leaves.

PDF TypeScript cache acceptance passed: baseline task hash `2961489084152580833`, deleted-output restoration at 1/1 local cache hit, relevant PDF source mutation at a cache miss with hash `6201406382484965991`, and unrelated JPG mutation retaining the baseline hit and identical JS/declaration outputs. Temporary source markers were removed. Broader compiler/runtime, Nx aggregate/launch/prerequisite, and final independent audit gates remain open.
