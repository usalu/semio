# Owned Production Plugin Host

## Outcome

The repository-owned instruction-fuelled WebAssembly interpreter is now mounted as the production plugin runtime. The SDK exports a compact owned ABI for allocation, deallocation, describe, reactor polling, job start/step/cancel, logical checkpoint, and logical restore. The native host validates and executes that ABI without generated WIT bindings, preserves interpreter state across fuel/deadline yields, and envelopes exact interpreter state with guest logical state for checkpoint/restore.

Production constructors now select `GuestRuntimes::Owned(OwnedRuntime::new())` in the renderer kernel and scale benchmark, MCP workspace activation, `run`, and shard child. Wasmtime remains only in host/oracle tests and in the describe differential oracle.

## Owned Boundary

The SDK exports exactly these nine functions from each rebuilt plugin artifact:

- `semio_owned_alloc_v1`
- `semio_owned_dealloc_v1`
- `semio_owned_describe_v1`
- `semio_owned_poll_v1`
- `semio_owned_start_job_v1`
- `semio_owned_step_job_v1`
- `semio_owned_cancel_job_v1`
- `semio_owned_checkpoint_v1`
- `semio_owned_restore_v1`

Inputs and outputs use repository-owned serde shapes. The reactor WIT bridge and owned ABI share the same kernel reducer; jobs share the same SDK registry; describe shares the same plugin definition; checkpoint/restore share the same SDK logical state. No third-party implementation source was copied.

`OwnedSemioArtifact` selects exactly one embedded core module, validates memory and all signatures, fingerprints the module, and creates/restores `OwnedSemioInstance`. `OwnedRuntime` stages allocate/call/deallocate as resumable operations. Every interpreter step is fuel-accounted, the host polls its deadline after at most 4,096 guest instructions, and cancellation is injected through `StepControl` before a guest job cancellation call. Host results are capped at 64 MiB.

The `SMOWNH01` host checkpoint contains pending operation metadata, host resource state, optional guest logical checkpoint bytes, and the exact interpreter checkpoint. A checkpoint taken during a fuel-yielded call therefore resumes the same instruction and ABI stage rather than replaying the call.

## Debug Artifact Evidence

Available artifact:

`/Users/ueli/Documents/semio/target/wasm32-wasip2/debug/semio_s_plugin_energy.wasm`

- Last rebuilt: 2026-08-22 02:31:48 Europe/Berlin
- Size: 30,206,964 bytes
- Contains all nine owned ABI exports

Artifact build command:

```text
bun ./📜️script.ts nx exec -- cargo build -p semio-s-plugin-energy --target wasm32-wasip2
```

The relevant build phase finished successfully. The wrapper attempted to repeat the command, so it was interrupted after the completed artifact was verified.

Focused parity command:

```text
SEMIO_OWNED_COMPONENT_FIXTURE='/Users/ueli/Documents/semio/target/wasm32-wasip2/debug/semio_s_plugin_energy.wasm' bun ./📜️script.ts nx exec -- cargo test -p semio-framework-plugin-host configured_component_executes_owned_describe_reactor_jobs_cancel_and_checkpoint_restore -- --nocapture
```

The relevant library test passed: `1 passed; 0 failed; 159 filtered out`, completing in 16.15 seconds. The test proves:

- owned compilation and describe output;
- a reactor turn with non-zero interpreter fuel;
- fuel exhaustion at one instruction, exact mid-call checkpoint/restore, and continuation;
- cancellation while an interpreter operation is instruction-yielded;
- job start, logical checkpoint, cancellation, restore, and step;
- a separate job cancellation followed by step.

The cached test executable was also run repeatedly after the fresh artifact build; the same test passed in 15.96, 19.81, and 20.04 seconds. The Nx exec wrapper was interrupted only after the relevant test had completed because it continued into additional targets/repetitions.

Initial focused failures were resolved rather than waived:

1. A relative fixture path resolved against the crate directory; validation now uses an absolute path.
2. The artifact called `wasi:random/insecure-seed`; the owned host now supplies the bounded host import.
3. The SDK and host initially disagreed on job budget/result JSON; they now share snake-case budget fields and a tagged camel-case `JobStep` representation.

The Energy describe project succeeded under the prior Wasmtime default before the cutover:

```text
bun ./📜️script.ts nx run @semio-tech/energy-plugin:describe
```

The describe tool is now switched to `OwnedRuntime`, but its post-cutover package validation is not claimed: the workspace shared target was occupied by another agent's broad check while free space fell below 10 GiB.

## Warning Gate

Focused strict command:

```text
bun ./📜️script.ts verify rust-warnings --target native -p semio-framework-plugin-host
```

This reached the host after fixing the new `JobBudget` serialization error. It then failed on the existing host warning cohort (119 diagnostics in this focused package), principally `result_large_err`, `unused_qualifications`, and related pre-existing plugin-host diagnostics. No warning gate was weakened. Normal compilation of the owned host and the focused parity test produced no new owned-runtime compile error.

## Dependency Deletion Gate

The direct rows are intentionally retained:

- `wasmtime` and `wasmtime-wasi` in plugin host;
- `wasmtime` and `wasmtime-wasi` in plugin describe;
- `wit-bindgen` in the plugin SDK.

The deletion gate requires all available plugin artifacts in debug and release plus describe, reactor, job, cancellation, and checkpoint parity. Only one debug artifact is currently present; no release artifact is present. The root filesystem had 9.7 GiB free while active P4, Animate, and shared workspace targets were still running. Starting a new cold release/full-suite target would risk active work and cannot produce honest green evidence. Consequently no dependency row or generated-binding oracle was removed.

No target was cleaned. No cache target is presently declared safe because ownership of the large active target directories belongs to other agents. Once the active builds finish or their owners explicitly release a target, the remaining gate is:

1. build or locate every plugin artifact in debug and release;
2. run the owned suite over both profiles;
3. run the Wasmtime differential oracle over the same artifacts;
4. validate the post-cutover describe project and production call sites;
5. clear the strict warning cohort without allowances;
6. only then remove Wasmtime/WASI/WIT generation and their direct Cargo rows.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/👶️child/🦀️main.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/📦️glue.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`

