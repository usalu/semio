# Owned Production Plugin Host

## Outcome

The repository-owned instruction-fuelled WebAssembly interpreter is now the production plugin runtime at every production constructor in scope. Renderer WGPU, MCP workspace and its shard child, OS run, the standalone shard child, and the descriptor emitter construct `OwnedRuntime`; remaining `WasmtimeRuntime` constructors are differential/test implementation paths.

The SDK exports a repository-owned ABI for allocation, deallocation, describe, reactor polling, job start/step/cancel, checkpoint, and restore. The native host validates that ABI without generated WIT bindings, preserves exact interpreter state across fuel/deadline yields, and combines it with the guest logical checkpoint. No third-party source was copied and no gate was weakened.

## Owned Boundary

Each rebuilt guest exports:

- `semio_owned_alloc_v1`
- `semio_owned_dealloc_v1`
- `semio_owned_describe_v1`
- `semio_owned_poll_v1`
- `semio_owned_start_job_v1`
- `semio_owned_step_job_v1`
- `semio_owned_cancel_job_v1`
- `semio_owned_checkpoint_v1`
- `semio_owned_restore_v1`

`OwnedSemioArtifact` selects one embedded core module, validates memory and signatures, fingerprints the module, and creates or restores an `OwnedSemioInstance`. `OwnedRuntime` stages allocate/call/deallocate as resumable operations. Every interpreter instruction consumes fuel; the host checks the deadline after at most 4,096 instructions; production slices use an 8 ms deadline. Cancellation enters through `StepControl` before the resumable guest cancellation call and preserves pending state on deadline/fuel yields. Results are capped at 64 MiB.

The `SMOWNH01` host checkpoint contains pending operation metadata, host resource state, optional guest logical checkpoint bytes, and the exact interpreter checkpoint. A checkpoint taken during a yielded call resumes the same instruction and ABI stage rather than replaying the call.

## MCP Request Boundary

`PluginArtifactChannel` owns an `Arc<OwnedRuntime>` and a compiled owned artifact. Its public request surface is synchronous only at the concrete owned turn boundary: every invocation is capped at 2,000,000 fuel and 8 ms, and deadline/fuel exhaustion returns retryable `budget.exceeded` with opening/pending state retained for the next request. Activation completion yields before command work; a completed exchange must contain exactly one command.

No `block_on` compatibility bridge remains in the MCP request/event loop. Pure codecs use the repository's ready resolver. Genuine probe/catalog/host suspension remains async. The probe artifact fixture removes its store from the standard mutex before awaits, then reinserts it. Transport tests use an owned readiness/call driver and add neither `tower` nor `tower-service`. Context truncation is linear rather than repeatedly serializing and removing the entire array.

Pure manifest constructors and accessors were made synchronous, and stale awaits/wrappers were repaired in the shared plugin manifest boundary and its Puzzle consumers. This restored compile stability for downstream renderer/Puzzle work without touching Animate, FEM, or renderer implementation files beyond the authorized runtime-constructor cutover.

## Artifact Inventory and Parity

The complete plugin artifact inventory available under the shared `wasm32-wasip2` target is Energy in both profiles:

| Profile | Artifact | Size | Built |
| --- | --- | ---: | --- |
| debug | `target/wasm32-wasip2/debug/semio_s_plugin_energy.wasm` | 30,206,964 B | 2026-08-22 02:31:48 CEST |
| release | `target/wasm32-wasip2/release/semio_s_plugin_energy.wasm` | 7,392,404 B | 2026-08-22 04:08:18 CEST |

The focused parity test covers owned compile/instantiate/describe, reactor polling, instruction-fuel yield/resume, exact mid-call checkpoint/restore, cancellation of an instruction-yielded operation, job start/step/cancel, logical checkpoint/restore, and post-cancel stepping.

Debug command:

```text
SEMIO_OWNED_COMPONENT_FIXTURE='/Users/ueli/Documents/semio/target/wasm32-wasip2/debug/semio_s_plugin_energy.wasm' bun x nx run @semio-tech/framework-os-host-rs:test-quick -- rust -p semio-framework-plugin-host configured_component_executes_owned_describe_reactor_jobs_cancel_and_checkpoint_restore -- --nocapture
```

Result: PASS, 1/1, 159 filtered; 20.09 s.

Release command was deliberately uncached because the Nx target does not declare `SEMIO_OWNED_COMPONENT_FIXTURE` as an input:

```text
SEMIO_OWNED_COMPONENT_FIXTURE='/Users/ueli/Documents/semio/target/wasm32-wasip2/release/semio_s_plugin_energy.wasm' bun x nx run @semio-tech/framework-os-host-rs:test-quick --skip-nx-cache -- rust -p semio-framework-plugin-host configured_component_executes_owned_describe_reactor_jobs_cancel_and_checkpoint_restore -- --nocapture
```

Final result after the owned warning cleanup: PASS, 1/1, 159 filtered; 5.19 s.

Release artifact build:

```text
bun x nx exec -- cargo build -p semio-s-plugin-energy --target wasm32-wasip2 --release
```

The relevant Cargo build finished successfully in 23m40s. Nx `exec` then began repeating the command for another project; that wrapper repetition was interrupted only after the requested artifact existed and was verified.

Owned describe results:

- debug: role `Plugin`, id `energy`, SHA-256 `e4bc394004883afe213f6ba162dc8f40b0770eb20a2c9b99a9ebb67e236251ea`;
- release: role `Plugin`, id `energy`, SHA-256 `ff1b39c545a0f39cf747b3e14327bffa40301e327b03d12bbac609ab7b439e89`.

Descriptor evidence is stored in `🧪️energy-owned-describe` and `🧪️energy-owned-describe-release` in this ticket directory.

## Gates

- `bun x nx run @semio-tech/framework-os-mcp-rs:check`: PASS.
- `bun x nx run @semio-tech/framework-os-mcp-rs:test-quick`: PASS, 142/142, 27 skipped, 0.338 s.
- `bun x nx run @semio-tech/framework-os-mcp-rs:test-long`: PASS, 169/169, none skipped, 0.604 s.
- `bun x nx run @semio-tech/os-plugin-describe-rs:build`: PASS, release profile, 3m01s, no production Wasmtime-oracle dead-code warning.
- `bun x nx run @semio-tech/os-plugin-describe-rs:test-quick`: PASS, 17/17, 2 skipped, 0.167 s.
- `bun x nx run @semio-tech/framework-os-host-rs:check -- --release`: PASS, 6m16s.
- `bun ./📜️script.ts verify dependencies`: PASS, baseline 238, current 185, 53 removed, zero additions.
- `bun ./📜️script.ts verify interactivity`: PASS in deny mode; one recorded allowlisted blocking bridge and no denied finding.
- `bun ./📜️script.ts verify rust-warnings --target native -p semio-framework-os-mcp`: the parent-repaired store cohort is cleared and the gate now reaches plugin host. Six owned/touched diagnostics were repaired semantically. The remaining gate fails on 89 broader existing plugin-host diagnostics, led by `result_large_err` plus existing await-lock, qualification, map/unwrap, signature-complexity, and related host/shard/effects warnings. The gate remains strict; no allowance or baseline was added.

Notable failures fixed during validation:

1. production cancellation could yield `DeadlineExceeded`; cancellation is now resumable with the same 8 ms slice and the parity driver retries only fuel/deadline yields;
2. MCP fixture code initially used a blocking bridge and later held standard mutex state across genuine awaits; the state now leaves the mutex before suspension;
3. MCP context truncation was O(n²) and exceeded the quick-test budget; byte accounting is now linear;
4. a long MCP propagation test injected directly into an actor mailbox and missed WorkerPool scheduling; it now uses the public host send boundary;
5. release parity initially appeared as an Nx cache hit from the debug fixture; the authoritative release run uses `--skip-nx-cache`.

## Dependency Deletion Gate

The following direct rows and generated oracles are intentionally retained:

- `wasmtime` and `wasmtime-wasi` in plugin host;
- `wasmtime` and `wasmtime-wasi` in plugin describe;
- `wit-bindgen` in the plugin SDK;
- generated Wasmtime actor bindings used only by host/differential tests.

Normal describe builds now `cfg(test)`-exclude the Wasmtime oracle implementation, so production defaults and normal execution do not rely on it. Deleting the rows would still be premature: Energy is the only plugin artifact present under the shared target, while the repository contains a broader plugin suite that has not been rebuilt in both profiles and differentially compared. The deletion gate therefore remains:

1. build every plugin artifact in debug and release;
2. run owned describe/reactor/job/cancel/checkpoint parity for each profile;
3. run the Wasmtime differential oracle over the identical matrix;
4. clear the remaining strict plugin-host warning cohort;
5. only then remove Wasmtime/WASI/WIT generation and direct Cargo rows.

This preserves an honest oracle instead of deleting evidence before suite-wide parity is green.

## Disk and Cache

After release validation:

- filesystem free: 59 GiB;
- `target`: 47 GiB;
- `target/release`: 2.1 GiB;
- `target/wasm32-wasip2`: 14 GiB.

No cache or target was cleaned. The shared target contains active/concurrent work, so no cache target is declared safe for deletion.

## Principal Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⏳️runtime.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/👶️child/🦀️main.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/📦️glue.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🚚️transport/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
- `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`

