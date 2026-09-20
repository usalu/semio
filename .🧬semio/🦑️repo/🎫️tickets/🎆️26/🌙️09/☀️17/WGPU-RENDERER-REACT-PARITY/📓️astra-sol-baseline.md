# Astra Sol Baseline

## Scope

This pass established the current React and native UI baselines, repaired the Overlay/Absolute layout contradiction, and repaired the WGPU browser bundle ownership manifest. Final integrated native renderer and wasm coverage remains with the parent pass.

## Results

| Gate | Result | Evidence |
| --- | --- | --- |
| `@semio-tech/framework-renderer-react:test-quick` | PASS | 1 file, 5 tests passed. `🗑️generated/astra-baseline/react-test-quick.txt` |
| Focused React Overlay law | PASS | 1 file, 2 tests passed, 114 unrelated tests skipped. `🗑️generated/astra-baseline/react-overlay-flow.txt` |
| `@semio-tech/ui-rs:test-wgpu-engine` | NO RESULT | Nx dependencies and UI generation completed, then Cargo remained queued for more than 20 minutes with no rustc child or diagnostic. The agent interrupted its outer exec, but parent inspection found the Nx/Bun/nextest/Cargo child tree survived. The parent retained that queue position and monitors `🗑️generated/astra-baseline/ui-test-wgpu-engine.txt`; no duplicate full suite was started. |
| React package typecheck | BLOCKED BY EXISTING WORKSPACE ERRORS | The target reached `tsc` and reported the existing broad workspace corpus. It included concurrent ShellHost/session migration errors and many unrelated packages; the new Overlay test module itself emitted no diagnostic. `🗑️generated/astra-baseline/react-typecheck.txt` |
| WGPU generator inputs + browser boot + frame worker | PASS | Parent rerun passed all 5 tasks in 42.7 seconds after the ownership repair. `🗑️generated/astra-runtime/browser-regenerate.log` |
| WGPU browser worker check | PASS | Parent rerun passed the target and its 4 dependencies in 12.6 seconds. `🗑️generated/astra-runtime/browser-worker-recheck.log` |
| Fresh React runtime | PASS WITH UNRELATED WARNING | Parent observed both Puzzle windows boot with no captured errors. The only observation was the already unrelated staged-plugin registry warning. |

The first focused React Overlay attempt collected no tests because ShellHost still referenced the removed `session-refresh/🌐️broker-port` module at transform time. A concurrent peer then moved ShellHost to the owned replacement `session-refresh/🪪️session-port/🟦️.ts`, after which the focused law passed. The older `space-artifact-creation-owner` suite still names the retired broker client and retired worker seams; that is a broader concurrent session migration rather than a local import-only repair.

## Ownership Repair

The WGPU browser profile now owns these direct/transitive modules in both `sourceModulePaths` and `inputPatterns`:

- `🧰️framework/🔨️modules/🕹️interaction/👆️gesture/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🏘️spaces/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔐️sign-in/🟦️.ts`

The arrays were checked as byte-ordered and unique before regeneration. The strict ownership resolver was not relaxed.

## Commands

```text
NX_DAEMON=false bun nx run @semio-tech/framework-renderer-react:test-quick --skip-nx-cache
CARGO_INCREMENTAL=0 NX_DAEMON=false bun nx run @semio-tech/ui-rs:test-wgpu-engine --skip-nx-cache
NX_FORCE_REUSE_CACHED_GRAPH=true NX_DAEMON=false bun nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache -- --run /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx --testNamePattern='Overlay layout flow'
NX_FORCE_REUSE_CACHED_GRAPH=true NX_DAEMON=false bun nx run @semio-tech/framework-renderer-react:typecheck --skip-nx-cache
```

Parent verification:

```text
bun nx run-many --projects=repo,@semio-tech/framework-renderer-wgpu --targets=generator-inputs,generate-browser-boot,generate-frame-worker --parallel=1
bun nx run @semio-tech/framework-renderer-wgpu:check-browser-worker --skip-nx-cache
```

## Remaining Integrated Gates

- `@semio-tech/framework-renderer-wgpu:test-wgpu-unit`
- `@semio-tech/ui-rs:test-wgpu-engine`
- `@semio-tech/ui-rs:check-wgpu-engine-wasm`

These were handed to the parent because Cargo never admitted the first native run during this slot.
