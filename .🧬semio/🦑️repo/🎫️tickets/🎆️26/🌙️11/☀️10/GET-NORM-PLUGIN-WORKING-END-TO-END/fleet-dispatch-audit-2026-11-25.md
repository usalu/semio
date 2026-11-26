# Fleet Dispatch Audit, 2026-11-25

Read-only inspection; no jobs submitted. `repo fleet --help` offers status only, with no dispatch command. `repo remote-execution --help` documents offloading hermetic Bazel actions when `SEMIO_REMOTE_EXECUTION_ENDPOINT` is set, invoked through `repo --tool=bazel run <project:target>`. It is not whole-machine command dispatch. `repo remote-execution status` showed registered workers with network denied and advertised Rust compiler actions and command-1 node/wasm-opt actions. The status was a point-in-time observation.

No supported action for a complete signed Playwright sweep or arbitrary remote repo test was identified. A sweep depending on a live networked Play server is not established as a hermetic eligible action. Remote Bazel actions do not establish that local project graph planning and its memory cost are bypassed. `repo test-file <path>` explicitly avoids reading the project graph according to `repo --help`, but runs locally. No job or SSH was started.

## Cargo rustc unit offload correction

Cargo also has a repository-native rustc unit offload path, separate from Bazel remote execution. The unit-cache wrapper calls `offloadRustcUnitV1`; repository compiler launch configures the wrapper, and the native component driver's compiler chain uses it when present. `distribute` is the default and `--offload=distribute` is accepted before the Cargo `--` delimiter. Each unit is subject to cache, cost, local-slot, portability, worker compatibility and admission decisions; it is not a promise of remote execution. `--offload=pin:<origin>` selects one origin and may refuse. One rustc unit cannot be split over 29 workers.

The EN 1995 artifact root package is `semio-s-artifact-norm-en1995`; its default lib crate type is allowed by offload policy. Workers observed in status advertised `wasm32-wasip2`. Candidate invocation, not executed:

```sh
repo cargo --store=shared --configuration=prototype --offload=distribute -- build --manifest-path '✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/📦️packages/🦀️rust/Cargo.toml' -p semio-s-artifact-norm-en1995 --lib --target wasm32-wasip2
```

Cargo also builds its dependencies. Worker advertisement and crate type do not prove the actual unit's inputs are portable or that it will offload; inspect the unit outcome. Local work and memory admission remain possible. This corrects the earlier statement insofar as it suggested Bazel was the only remote compilation path.

## EN 1995 mutations pin and local fallback

The intended package is `semio-s-artifact-norm-en1995-mutations` (the preceding root artifact candidate is not the requested package). Its default `lib` type is eligible in principle. `--offload=pin:g1-m06` before Cargo's `--` names a replica origin, not a worker process. In pin mode the gate queues eligible units, but the offloader explicitly returns a local outcome for failed attempts; the unit-cache caller falls through to local compilation for those outcomes, exceptions and ineligible source/crate types. Cache hits bypass offload; execution of build scripts remains local. Pin therefore is not a remote-only safety guard.

A status snapshot showed g1-m06 on macOS arm64 with wasm32-wasip2 advertised, headroom true, no running actions, and heartbeats 3–4 seconds old. Actual match still depends on compiler identity and action properties. Local capacity at the same time: 4,861 MB available, 2,048 MB floor, 2,813 MB spare; a separate 4,916 MB request waited. A one-job limit constrains compiler concurrency, not peak memory, and cannot prevent a local fallback. No invocation can be claimed safe for a remote-only peak test on that evidence. No build was launched.

## Observed funded EN 1995 mutations build

The existing single-job `repo cargo` run PID 74638 finished successfully in 15m53s. Its exact target rustc was observed locally as PID 93432; samples at approximately 2.5-second intervals between 21:11:04 and 21:12:06 UTC reached a maximum RSS of 1,262,176 KiB at 21:11:25 UTC. This is the sampled resident set of rustc alone, a lower bound on its true maximum; it is not charged physical footprint, total process-tree memory, or the scheduler's reservation. No target-specific charged-footprint receipt was observed. Local execution was directly observed, rather than inferred from the Cargo `Compiling` line. Raw build output was the existing generated `coord-en1995-wasm-funded.log`.
