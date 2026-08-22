# P9aa Owned WASM Interpreter and Semio Describe Boundary

## Decision

This packet implements a repository-owned, instruction-fuelled WebAssembly core interpreter and an ABI-specific Semio component/describe boundary. It proves exact descriptor parity against retained Wasmtime for every component artifact that could be exercised without another multi-gigabyte build: the real energy plugin and the scale fixture.

It does **not** prove full plugin-suite parity. The 433,894,912-byte debug stdio component cleanly executes for 100,000,000 owned instruction-fuel units but does not reach its describe task return, while the retained Wasmtime oracle cannot compile/execute either the raw or normalized artifact inside the 600,000 ms router budget. Other production plugin components were not available as built artifacts during the disk-constrained gate.

Therefore:

- Wasmtime remains the native default and comparison oracle;
- the direct `wasmtime`, `wasmtime-wasi`, and `wit-bindgen` rows and sources remain;
- the browser continues to use the platform WebAssembly boundary unchanged;
- dependency deletion and owned-default switching remain correctly ratcheted behind later full-suite parity.

## Owned Files and Routing

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🦀️component.rs` owns component/core parsing, core execution, checkpoints, the Semio actor adapter, and the pure describe host/session.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/📦️glue.rs` retains the Wasmtime oracle, normalizes execution binaries, and owns the multi-fixture differential gate.
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️scale/📦️packages/🦀️rust/📜️script.ts` and `📋️project.json` expose the fixture's real WASI-P2 component build through Nx.
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts` and `📋️project.json` expose an explicit root-cdylib release WASI-P2 build for the later release-parity wave. The build was stopped before linking when free-disk capacity fell to 14 GiB; no release stdio artifact is claimed.

No external interpreter source or runtime implementation detail was copied. Runtime execution uses repository-owned Rust and the standard library.

## Instruction-Granular Runtime

`CoreInstance::step(fuel, StepControl)` is the only instruction driver. Every decoded core instruction consumes one fuel unit and returns an explicit outcome:

- `Yield { fuel_used }` when the grant ends;
- `HostCall { fuel_used, call }` when an imported function suspends;
- `Complete { fuel_used, values }` on return;
- `Cancelled { fuel_used }` before another instruction executes after cancellation;
- `Fault { fuel_used, error }` for decode, validation, trap, host, or state failure.

Imports suspend into typed `HostCall` state and continue only through `resume_host(call_id, result)`. No host import runs through an opaque one-shot compute closure. Function bodies and structured-control maps are shared through `Arc`, avoiding a per-instruction copy of large guest functions.

Core checkpoints include:

- module fingerprint and format version;
- memories, tables, globals, passive data and element liveness;
- operand stack, locals, call frames, program counters, and control frames;
- pending host-call identity, arguments, and expected results.

Checkpoint bytes are deterministic for identical state and rejected against a different module. The every-instruction loop test restores at every instruction and completes byte-identically.

## Supported Core Surface

- Core version-1 sections: custom, type, import, function, table, memory, global, export, start, element, data-count, code, and data.
- Function, table, memory, and global imports; explicit host suspension and typed reply validation.
- `i32`, `i64`, `f32`, `f64`, nullable `funcref`, and nullable `externref`.
- Multi-value function/block signatures.
- Blocks, loops, if/else, branch, branch-if, branch-table, return, direct calls, and indirect calls.
- Locals, globals, tables, select, and typed select.
- Scalar loads/stores, memory size/grow, bounds checks, and memory64 limits/address values.
- Scalar numeric opcodes through sign extension, saturating conversions, checked traps, reinterpretation, deterministic canonical NaNs, signed-zero min/max, and ties-to-even rounding.
- Active/passive data and element segments.
- Bulk memory/table initialization, drop, copy, fill, grow, and size operations.
- Start functions during instantiation.

## Unsupported Core Surface

- SIMD (`0xfd`).
- Threads/atomics (`0xfe`) and shared memories.
- Exception tags.
- GC/reference types beyond nullable `funcref` and `externref`.
- Tail-call proposal instructions.
- Multi-memory indexed scalar load/store encodings; scalar loads/stores target memory zero. Indexed bulk operations and memory size/grow are supported.
- General-purpose proposal coverage beyond the Semio guest subset.

## Component and Semio ABI Surface

The owned structural boundary supports component header 13.1, bounded byte size/nesting/core-module counts, every current component section identifier, recursive nested components, full owned parsing of embedded core modules, artifact discrimination, and stable component fingerprints.

`SemioActorArtifact` selects exactly one embedded core module implementing all seven generated async-lift exports with the observed Semio ABI signatures:

- checkpoint and restore;
- describe;
- cancel-job, start-job, and step-job;
- reactor poll.

It additionally validates canonical memory, `cabi_realloc`, and the describe task-return import. `SemioActorInstance` exposes bounded begin/step/resume, canonical memory, bounded descriptor extraction, and component-fingerprint-bound checkpoints.

The executable canonical adapter is deliberately ABI-specific. It invokes the generated actor core export directly rather than implementing a general component instantiation graph. End-to-end lifted value handling is currently implemented only for `describe`, whose canonical result is the task-return `(pointer, length)` byte slice.

## Pure Describe Host

`SemioDescribeSession` owns actor state, deterministic host state, descriptor state, bounded stepping, cancellation, and checkpoint/restore. Its host implements the import subset reached by the available Semio describe artifacts:

- async root context get/set, waitable allocation/poll/join/drop, and task cancellation;
- Semio pure `now-ms` as zero;
- WASI monotonic/wall clocks as zero;
- deterministic insecure seed as zeroes;
- empty environment;
- terminal absence;
- deterministic stdin/stdout/stderr resources;
- poll and pollable block;
- stdout/stderr sink semantics with 65,536-byte write capacity, write success, and flush success;
- resource drops;
- bounded describe task-return extraction.

Unavailable imports fault explicitly. Filesystem, sockets, network, real entropy, real time, terminal behavior, and general host-async effects are not silently emulated.

## Unsupported Component/Canonical Surface

- General core-instance construction, aliases, component instantiation, and canonical lift/lower interpretation.
- General strings, lists, records, variants, results, options, tuples, resources, futures, streams, and post-return wiring.
- Typed repository-owned invocation/results for reactor, jobs, and component checkpoint exports. Their exact core signatures are validated and raw core invocation is possible, but their canonical values and host protocols are not implemented end to end.
- The full WASI Preview-2 surface.
- Full production host replacement. Owned execution is mounted and differential-tested in describe, while the retained Wasmtime production path remains default.

## Execution-Binary Normalization

`wasm_execution_binary` recursively removes only custom sections from the root component, embedded core modules, and nested components, then re-encodes section lengths. Original bytes remain the artifact identity/input for descriptor hashes and reports.

For stdio this reduces the execution input from 433,894,912 to 191,435,298 bytes. Both forms remain valid Semio actor artifacts. The normalization focused gate passed in 15.864 seconds.

Evidence: `📝️p9-owned-stdio-execution-normalization.log`.

## Differential Evidence

### Available two-artifact suite

The final suite variable contained:

1. `target/wasm32-wasip2/debug/semio_s_plugin_energy.wasm` — real energy plugin, 29,091,073 bytes;
2. `🧪️target-stdio-compression/wasm32-wasip2/debug/semio_framework_os_scale_fixture.wasm` — real WASI-P2 scale fixture, 841,642 bytes.

For each artifact the retained Wasmtime path and owned path produced byte-identical descriptor results. The owned path also proved byte-identical checkpoint/restore during execution and cancellation before its first instruction.

Result: 1/1 suite test passed in 51.786 seconds.

Evidence: `📝️p9-owned-wasmtime-available-suite-differential.log`. Earlier single-artifact evidence is retained in `📝️p9-owned-wasmtime-energy-describe-differential.log` and `📝️p9-owned-wasmtime-scale-describe-differential.log`.

### Exact stdio limitation

The 433,894,912-byte debug stdio component passes owned structural parsing and exact seven-export actor ABI validation. Its owned describe invocation then:

- consumed exactly 100,000,000 instruction-fuel units;
- serviced at least one real host call;
- cleanly yielded without cancellation or fault;
- did not reach the describe task-return within that fuel grant.

Result: 1/1 bounded limitation test passed in 54.685 seconds.

Evidence: `📝️p9-owned-stdio-actor-abi-validation.log` and `📝️p9-owned-stdio-100m-bounded-execution.log`.

The retained Wasmtime oracle was attempted against both the raw artifact and the recursively normalized execution bytes. Both stayed CPU-bound and were killed at exactly 600,000 ms without producing a descriptor. Repeating the same debug oracle is not useful evidence.

Evidence: `📝️p9-wasmtime-stdio-describe-oracle.log` and `📝️p9-wasmtime-stdio-describe-normalized-oracle.log`.

This is a performance/available-oracle blocker, not a claimed runtime fault and not parity.

## Regression Gates

- Final describe quick suite: 17/17 passed, 2 long tests skipped, 0.191 seconds. Evidence: `📝️p9-owned-describe-quick-final.log`.
- Exact stdio 100M-fuel bounded execution: 1/1 passed, 54.685 seconds. Evidence: `📝️p9-owned-stdio-100m-bounded-execution.log`.
- Energy + scale owned/Wasmtime differential: 1/1 passed, 51.786 seconds. Evidence: `📝️p9-owned-wasmtime-available-suite-differential.log`.
- Shared native plugin host check: passed in 1m01s. Evidence: `📝️p9-owned-host-check-final.log`.
- Earlier native release describe build after the core validation additions: passed in 2m34s. Evidence: `📝️p9-owned-describe-release-build.log`. A new large release build was intentionally not repeated under the final 14 GiB free-disk constraint.
- Scoped Nx format write/check for the changed stdio router files: passed. Evidence: `📝️p9-owned-scoped-format-write.log` and `📝️p9-owned-scoped-format-check.log`.
- Temporary source debug output census: no `[DEBUG]`, `dbg!`, or temporary interpreter prints remain. Existing describe CLI/host logging is production behavior.

## Dependency Ratchet

The retained direct declarations are deliberate:

- plugin SDK: one direct `wit-bindgen` row;
- native describe tool: direct `wasmtime` and `wasmtime-wasi` rows;
- native plugin host: direct `wasmtime` and `wasmtime-wasi` rows.

No dependency row or generated binding source was deleted. Removal is permitted only after later-wave native debug/release full-plugin-suite parity, including stdio, proves the owned path and after the production host defaults to it.

## Phase 1 Closure at This Boundary

Owned guest execution is driven as explicit bounded fuel steps. The two production plugin-host paths in `⏳️imports.rs` and `⚡️effects/🦀️component.rs` were migrated to `InteractiveJob`/`ComputePool::run_job`; one bounded job step is submitted per worker closure with explicit cancellation, deadline, and terminal propagation. Blocking operating-system I/O remains separately named `ComputePool::run_io`. No production `ComputePool::run_blocking` call remains at the plugin boundary.

## Final Status

- Repository-owned core execution: implemented and mounted.
- Instruction fuel, deterministic checkpoints, host suspension/resume, and cancellation: green.
- ABI-specific Semio actor validation and owned describe execution: implemented.
- Exact available-artifact differential: green for energy and scale.
- Exact stdio execution: cleanly resumable but incomplete at 100M fuel; oracle unavailable inside 600 seconds.
- Full plugin suite and release stdio parity: deferred to the next P9 wave.
- Owned native default: not switched.
- Wasmtime/WASI/wit-bindgen deletion: not performed.
