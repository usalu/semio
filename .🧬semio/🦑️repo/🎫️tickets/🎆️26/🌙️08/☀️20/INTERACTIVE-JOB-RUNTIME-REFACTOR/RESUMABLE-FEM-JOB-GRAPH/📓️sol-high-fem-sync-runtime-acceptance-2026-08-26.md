# Sol High FEM Sync Runtime Acceptance

Date: 2026-08-26
Owner: `/root/shared_action_cohort`
Scope: `✏️s/🔌️plugins/🏗️fem/**` and included `✏️s/🔨️modules/🏗️fem/**`
Status: BLOCKED OUTSIDE FEM BY OWNED-INTERPRETER DESCRIBE TRAP

## Fresh Baseline

```text
[DEBUG] CARGO_TARGET_DIR='.../🧪️target-sol-fem' cargo check -p semio-s-plugin-fem --lib --message-format=short
[DEBUG] exit=101; rustc errors=135; warnings=137
[DEBUG] plugin-owned diagnostics=108; included FEM-engine diagnostics=27
```

The failures were current sync-trait/model and retained-payload API mismatches, not the stale
141-error count. No non-FEM production source was edited.

## Compiler Repair

The plugin editor/viewer/schema/IO implementations now implement the current synchronous framework
traits. FEM graph, assembly, PCG, and mesh outcomes now publish framework-owned
`RetainedJobPayload` values through the one-opportunity `StepContext` admission route. Current
model collection, fixed-slot, operation identity, fixed UI admission, and mounted session borrow
APIs were repaired without a compatibility path.

```text
[DEBUG] CARGO_TARGET_DIR='.../🧪️target-sol-fem' cargo check -p semio-s-plugin-fem --lib --message-format=json | jq <error-records>
[DEBUG] exit=0; error-records=0
[DEBUG] progression: 135 -> 13 -> 8 -> 0
```

`cargo fmt -p semio-s-plugin-fem` completed successfully after the final FEM source edits.

## Phase 6 Source Gates

```text
[DEBUG] bun ./📜️script.ts verify interactivity tool-jobs --p6h-only --self-test
[DEBUG] exit=0
[DEBUG] [verify interactivity tool-jobs p6h] live-source clean; hostile-mutations=70.
```

```text
[DEBUG] bun ./📜️script.ts verify interactivity tool-jobs --p6i-only --self-test
[DEBUG] exit=0
[DEBUG] [verify interactivity tool-jobs p6i] live-source clean; hostile-mutations=101.
```

The root verifier's sync cutover aligned its extraction with `fn render(` and synchronous
`V::pending_effects(doc, cfg)`. Both FEM live-visual and numerical microcursor source gates are now
green.

## Retained FEM Command Acceptance

Only reducers audited as constant first-step work were admitted:

- FEM2D: `setAnalysisSettings`, `setCamera`, `setResultDisplay`, `setLocale`;
- FEM3D: `setAnalysisSettings`, `setCamera`, `setResultDisplay`.

Both editor owners now register a typed production `ToolJobFactory` and
`ArtifactOwnedToolJobFactory` backed by `ArtifactRetainedCommandJob`. The typed command dispatches
directly through the app command enum; it does not call generic `A::handle`. Wire and checkpoint
ownership are bounded at admission, accepted at the maximum, rejected at maximum plus one, and
restored through `ArtifactRetainedCommandJob::from_wire_with_checkpoint`. Exact source proof,
factory IDs, and manifest dispositions are joined to the language-neutral fixture. The 30 remaining
FEM reducers can traverse, allocate, serialize, or solve and remain fail-closed pending custom
`ArtifactCommandWork` implementations.

```text
[DEBUG] Ajv 2020-12 fixture/oracle
[DEBUG] exit=0; valid=1; duplicate=reject; wrong-owner=reject; maximum+1=reject; routes=2; actions=7

[DEBUG] fixture/source exact join
[DEBUG] exit=0; controllers=2; factory/proof/disposition actions=7; local-unaccepted=0
```

## Fresh Isolated Compiler Gates

```text
[DEBUG] CARGO_TARGET_DIR='.../RESUMABLE-FEM-JOB-GRAPH/🧪️target-sol-fem-native-checkpoint-final' RUSTFLAGS='-Awarnings' cargo check -p semio-s-plugin-fem --lib
[DEBUG] exit=0; Finished dev profile in 4m16s
```

```text
[DEBUG] CARGO_TARGET_DIR='.../RESUMABLE-FEM-JOB-GRAPH/🧪️target-sol-fem-wasip2-final' RUSTFLAGS='-Awarnings' cargo check -p semio-s-plugin-fem --lib --target wasm32-wasip2 --quiet
[DEBUG] first run found six owned stale awaits in the FEM2D/FEM3D Wasm read bridges
[DEBUG] repaired snapshot_json/envelope_json/generation to call the current synchronous API
[DEBUG] rerun exit=0
```

## Full Static Ledger

```text
[DEBUG] bun ./📜️script.ts verify interactivity tool-jobs --format json --output '.../RESUMABLE-FEM-JOB-GRAPH/📊️fem-full-tool-job-verifier-2026-08-26.json'
[DEBUG] exit=1 only for global aggregate failures
[DEBUG] commandRows=774; uniqueCommandRows=772; boundedRows=161; remainingCommands=777
[DEBUG] productionFactories=23; productionRegistrations=166; literalRegistrations=706; selfTests=463
[DEBUG] forged bounded reducer failures=0; FEM failures=0; FEM acceptedCommandRows=7; FEM remainingCommands=30
[DEBUG] global failures: 37 process-global stores; 35 import-media routes; 777 remaining commands
```

The complete machine-readable report is
`RESUMABLE-FEM-JOB-GRAPH/📊️fem-full-tool-job-verifier-2026-08-26.json`. The verifier later advanced
to 464 self-tests due an unrelated shared fixed-operation global-store exemption; this captured run
is the exact live-tree result before that external change.

## Descriptor Generation Blocker

```text
[DEBUG] bun ./📜️script.ts nx run @semio-tech/fem-plugin:describe --skip-nx-cache
[DEBUG] native describe runner build=success; FEM wasm32-wasip2 build=success
[DEBUG] production describe invocation reached owned guest after approximately seven minutes
[DEBUG] exit=1
[DEBUG] semio-framework-plugin-describe describe: calling owned describe() on
[DEBUG] /Users/ueli/Documents/semio/target/wasm32-wasip2/debug/semio_s_plugin_fem.wasm:
[DEBUG] guest trapped: wasm trap: memory.copy destination is out of bounds
```

This failure occurs inside `OwnedRuntime::describe` before pack decoding or descriptor writes. It is
not explained by descriptor size: the committed FEM JSON is 373,210 bytes, while multiple retained
repository descriptors range from 442,281 through 1,763,131 bytes. Native and real Wasm component
type-checks are green. Because the production emitter deliberately calls only the repository-owned
interpreter, descriptor regeneration is not safe and `🛢️descriptor.semio` / `🔣️descriptor.json`
were not modified by the failed command.

Static tracing localizes the emitted text to the owned interpreter's
`CoreInstance::execute_fc`: opcode `0xfc/10` (`memory.copy`) reaches
`checked_end(destination, length, memory.bytes.len(), "memory.copy destination")`. The real-Wasmtime
oracle exists only behind `cfg(test)` and is not run by the production descriptor CLI. The owned
interpreter has memory load/store/grow coverage but no direct `memory.copy` regression or
owned-versus-Wasmtime hostile range differential. Its current error also omits `destination`,
`length`, and the linear-memory bound. A read-only LLDB attempt against the already-built runner
resolved the exact failing branch by symbol offset, but this macOS host left the launched debuggee
suspended without delivering the breakpoint event, so no operand values were recoverable. No
framework source was modified under the FEM-only boundary.

The coordinator subsequently expanded this packet's boundary explicitly to the repository-owned
interpreter. The interpreter now reports exact `start`, `length`, calculated `end`, and `bound` for
every memory range failure. A schema-first JSON fixture covers forward and reverse overlap,
exact-bound, zero-length at the bound, copy after grow, destination and source maximum plus one,
and hostile `u32::MAX` source/destination. Its Rust consumer runs every case through both
`CoreInstance` and Wasmtime and requires identical success/trap classification, exact successful
destination bytes, exact owned diagnostics, and matching grown page counts. The distinct-memory
branch uses disjoint mutable slices after validating both ranges, not a temporary allocation sized
by guest input. The plugin test target now declares its Wasmtime oracle dependency explicitly;
the existing lock already includes that exact dependency.

```text
[DEBUG] bun -e <Ajv2020 memory.copy fixture oracle>
[DEBUG] exit=0; fixture cases=9; ok=5; trap=4
[DEBUG] missing trap diagnostic=reject; forged trap bytes=reject; missing success bytes=reject
```

Strict Ajv 2020-12 validation, `rustfmt`, and `git diff --check` exit zero. The Rust/Wasmtime
differential is source-complete but deliberately not compiled or run yet because the coordinator
reserved the compiler slot for Puzzle. The next permitted build must run this regression first,
then rebuild the production describe runner and re-run FEM describe to capture the now-exact range
if the differential does not already expose the semantic defect.

## Remaining Blocker

The sole owned completion blocker is the repository-owned interpreter's `memory.copy` trap during
FEM `describe()`. No additional Cargo/Nx process was started while the coordinator owns the compiler
slot. A fresh Nx regeneration, descriptor census, and all-app live disposition gate remain required
after that interpreter failure is repaired or proven external.
