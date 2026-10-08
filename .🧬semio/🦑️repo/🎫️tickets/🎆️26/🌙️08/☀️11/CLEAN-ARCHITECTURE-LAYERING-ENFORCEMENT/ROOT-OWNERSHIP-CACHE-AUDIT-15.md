# Root Ownership Cache Audit 15

Read-only audit of production and caches; no tests run, no caches removed. RepoMCP unavailable; no lifecycle claim. Audit used Bun inspection, recursive lstat, retained terminal bodies, current source comparison, du, and current process inventory.

## Exact Cache Retirement Boundary

The eight eligible directories are generated/wgpu-ownership/target-{3,4,5,6,7,8,9}/nx and generated/wgpu-composition/target-2/nx beneath this ticket’s 🗑️generated directory. Other composition cases have no nx directory. Each nx contains only cache and workspace-data; sampled cache root has run.json and terminalOutputs, and workspace-data contains generated repository maps. Recursive inspection found no symlinks (1513–1521 regular files per tree). Producer bodies explicitly direct NX_WORKSPACE_DATA_DIRECTORY and NX_CACHE_DIRECTORY into these case-local nx paths. They are regenerable Nx outputs, not source/input custody. Approximate total measured size is 2.56 GiB (327–329 MiB each).

Every case has a retained terminal receipt: ownership statuses are 3=exit/1, 4=timeout/null, 5=exit/0, 6=exit/1, 7=exit/0, 8=exit/1, 9=exit/0; composition target-2=exit/0. All report sourceExact=true. Closure does not imply passing: failed/timeout predecessor evidence must remain intact. Current process command inventory contains no matching WGPU ownership/composition/Hub renderer command except this audit itself. A concurrent shared Rust OS compile exists and must remain untouched. Root ownership and physical-close assertion come from parent task custody; retained results independently corroborate terminal completion, not exhaustive descendant liveness. No disqualifying symlink/input discovered. Recheck matching process handles immediately before deletion if any new run starts.

Retire only these nx subdirectories. Preserve admission.json, terminal.json, command receipts/stdout/stderr, full-body custody, all source/input folders, shared native target, and peer outputs.

## Specific Hub Composition And Target 9

All nine target-9 selected receipt rows still exactly match current disk: eight mandatory source bodies exist; retired General collaboration harness is absent. The Specific harness exists and current Specific script statically imports ./🤝️collaboration/🟦️.ts. The schema is mandatory in current source and receipt, requires version/commands/collaboration, five commands, seven enumerated journeys, and fifteen fixed bounds with additionalProperties=false.

Current ownership test compiles the same schema with Ajv and compares own SchemaSubset validation; it checks one accepted fixture and three refusal vectors. It imports actual Specific collaboration module, compares journeys/kinds/bounds, asserts old General harness ENOENT, uses TypeScript oracle for command declaration checks, and imports General declaring script. Target-9 retained stdout records portable=5, executableSpecificLiterals=0, schema portable=1/refusalVectors=3/Ajv=1, Specific module imports=1/retainedJourneys=7/bounds=15, declaringTargets=5/retainedNativeLaws=7/TypeScriptOracle=1, General declaring module imports=1/Specific declaring owner main=1/nativeInvocation=0; Nx exits zero with 0/1 cache hit.

This supports the narrow command composition/schema/relocation acceptance. It does not establish live collaboration/native runtime execution, every import closure, all platforms, or atomic snapshot. Receipt explicitly sets nativeRuntimeAccepted=false, allPlatformsAccepted=false, atomicSnapshotClaimed=false. No independent runtime rerun was attempted during disk recovery.
