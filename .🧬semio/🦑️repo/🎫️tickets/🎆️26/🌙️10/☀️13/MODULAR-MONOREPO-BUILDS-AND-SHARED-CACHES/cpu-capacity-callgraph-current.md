# Compiler Capacity Call Graph And Declaration Thread Cut

2026-10-15. Bounded read-only Source audit and proposed next scope. No implementation, capacity acquisition, compiler, storage mount, Nx publication or Core/Native lease edit was performed for this audit. The 88-law direct-child custodian foundation is unchanged. One read-only repository scan session, 32946, completed and was collected; no terminal handles remain.

## Authority Consumers

There are two production decisions that turn the inherited marker into unpaid admission:

1. `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts:59-61`: `acquireCompilerCapacityLeaseV1` returns a no-op lease from the boolean/count instead of acquiring gate/cores.
2. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts:133`: the launcher skips its entire capacity/scheduler/jobserver/broker branch when the boolean is true.

`compilerCapacityWorkerCountV1` at capacity `:43-48` and its browser materialization consumers use marker/count as worker sizing. `withWholeHostCompilerCapacity` at library `⚡️caching/📦️bundle/🟦️.ts:44-47` additionally refuses a marked partial pool for an unbounded compiler, then calls the same vulnerable acquisition API. Sizing and a partial-pool guard do not establish paid custody. No production Rust reader of either capacity marker was found; Cargo's actual jobserver/broker protocols are separate from this TypeScript admission bypass.

The package projections reexport the same Source functions. A projection import is not a cross-process capacity handoff and cannot make an environment marker authentic.

## Concrete In-Process Nesting

| Outer paid owner | Nested Source call | Boundary and consequence |
| --- | --- | --- |
| `✏️s/🧑‍💻dev/🗄️stdio/🏘️composition/🏗️build/🟦️.ts:31-36`, local `compile` closure | `bundleJavaScript` in library `⚡️caching/📦️bundle/🟦️.ts:35` | The function call is in the same process; the nested esbuild admission sees the authored environment marker. Only the final esbuild command is a child. This is a real duplicate reservation seam that private local custody can replace. The outer directory is `.cache/framework/leases`, while the bundle helper names `.cache/b/l/m/data`; do not silently assume they are one physical guard. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🌐️wasm/📜️script.ts:44-45` | Local `compileTrunkRenderer` calls `buildCargoArtifacts` at `:84`, whose own acquisition is process native-build `🟦️.ts:250` | This is same-process nested admission using the same renderer policy lease directory. The outer reservation remains live through Trunk, the later Cargo observation query and publication. Simply paying again would self-block when the outer owner has all cores. |
| Process native-build component driver `🧩️component-pass/🟦️.ts:377-380` | `buildSchedulerV1` at native-build `🎟️admission/🟦️.ts:307-333` | Scheduler construction is an in-process function call followed by a real Cargo child; it does not call the capacity API again. Its compiler work is currently covered by the driver's live pool, then later work shares actual jobserver tokens. Adding an independent acquisition here would introduce nesting that does not currently exist. |
| Repository Cargo launcher `:134-136` | The same `buildSchedulerV1` | Same pattern. The scheduler is compiled while the launcher holds its real reservation on an honest unmarked path. |
| Next development admission `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🏗️builder/🎟️admission/🟦️.ts:6-36` | Its in-process participant group | The existing private group shares one acquisition among overlapping rebuild participants and drains when the group is idle. It is an example of local grouping, but its original acquisition still accepts environment markers. It also constructs a structural resource-control wrapper, so it cannot be assumed to carry a new private Original control loan without explicit repair. |

Private in-process borrowing can replace the first two duplicate admission seams. It must start from a real privately issued paid reservation and retain actual accepted operations, not infer authority from the environment passed to a callback. It must account for simultaneous children and the parent's own runnable allocation; asynchronous context alone is not a worker-count ledger. It cannot be serialized into a new process.

## Concrete Cross-Process Consumers And Deadlock Edges

| Paid parent | Child admission | Evidence |
| --- | --- | --- |
| Private `withBunBundleFiles` in library `⚡️caching/📦️bundle/🟦️.ts:92-96` holds `withWholeHostCompilerCapacity` | `⚡️caching/📦️bundle/🟦️bun/📜️script.ts:100` calls that helper again around `compileBunBundle` | Parent launches `process.execPath` with the worker script and marked environment. Parent and child both naming real whole-host acquisition would deadlock. The child performs actual `Bun.build`, not just transport. |
| TypeScript package main, `⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts:32-34,56` holds one core in `compile` | Its executable `emit-declarations` branch at `:99-106` acquires one core again | `runRepositoryCommand` at `:56` is a real process spawn; its environment carries the parent marker. This is not an in-process call despite sharing one Source file. |
| Stdio composition local `compile`, `✏️s/🧑‍💻dev/🗄️stdio/🏘️composition/🏗️build/🟦️.ts:32,38` | The same executable declaration branch | A second real subprocess caller of `emit-declarations`. The old marker suppresses its independent one-core request. |
| Native component driver `🧩️component-pass/🟦️.ts:377-380,600-603` retains the paid pool | Arbitrary declared pipeline jobs are real spawned commands and can enter first-party compiler wrappers | Its closed plan passes each job's assigned count and actual jobserver/broker endpoints, but the boolean/count still cross a process boundary. Current `🏢️semio-tech/🎡️play/📋️project.json:200-382` declares actual Rust compiler jobs, WASM jobs and the Vite bundle job under `jobserver`, `reserved` or `exclusive` worker protocols. This is a concrete supported ancestor, not a hypothetical use of the schema. |
| `buildViteArtifact`, library `⚡️caching/🌐️vite/🟦️.ts:51-58`, holds `withCompilerState` capacity | The Vite CLI runs configuration and plugin code in a new process | Browser compilation can invoke the Bun bundle worker from plugin code. A private scope in the Vite launcher does not cross that CLI boundary. Development Vite also holds capacity around `createServer` at `:97-111` and around transform/warmup callbacks at `:115-124`; a plugin-triggered compiler can therefore have a live outer compiler phase. |
| `runRepositoryCargoTestExecutableV1`, library `🏃️process/🦀️cargo/🧪️testing/📦️executable/🟦️.ts:38-45` | The compiled test executable inherits the marker and can call first-party commands | Capacity is retained through both compilation and execution, rather than released before the test body. A descendant repository build can therefore legitimately encounter the old inherited marker while this parent still owns all cores. Releasing the parent reservation early would change the accounting of actual test compute. |

Storybook's build owner also spawns its CLI under a marked `withCompilerState` reservation (`🏗️build/📖️storybook/🏃️execution/🟦️.ts:22-23`). Next's production builder does the same for its Node CLI (`🖥️server/🎛️coordinator/🏗️builder/🏃️execution/📜️script.ts:30-35`). These parent scopes and plugin callbacks must be considered when moving downstream admission.

## Other Acquisition Boundaries Audited

- `withCompilerState`, process `🏗️build/🗃️state/🟦️.ts:39-44`, acquires and marks the environment passed to its callback. Callers include TypeScript checks (`🏗️build/🟦️typescript/🟦️.ts:26-29`), Vite, Storybook and Next. `capacity:false` paths are explicitly state/publication work and must not be interpreted as CPU custody.
- `buildGoArtifactsV1`, process `📦️artifacts/🐹️go-build/🟦️.ts:29-32`, acquires immediately before a real `go build` child. Its production library wrapper is `runCanonicalGoBuild`, library `🟦️.ts:1113-1123`, called by Go package scripts. It has no observed second local capacity call. That does not prove it is safe to ignore an enclosing cross-process pipeline/test reservation.
- `buildWasmWebV1`, process `📦️artifacts/🕸️wasm-build/🟦️.ts:111-138`, owns capacity across Cargo/wasm-pack/bindgen commands and propagates marked sizing. Actual release pipeline WASM jobs can already have a paid driver parent.
- Native artifact build, `📦️artifacts/🏗️native-build/🟦️.ts:250-256`, owns capacity immediately before `captureCargoArtifacts`, which actually spawns Cargo at `:775-781`; Trunk supplies the concrete same-process outer owner described above.
- `runCargoRelayV1`, process native-build `🌊️relay/🟦️.ts:29-30`, acquires and marks a real Cargo relay child. Exact Cargo testing, process `🧪️testing/🦀️cargo/🎯️exact/🟦️.ts:280`, acquires around its build probe. These remain marker consumers through the shared acquisition API.
- Managed Rust unit compilation, library `⚡️caching/🦀️cargo/📦️units/🏗️compiler/🦀️rustc/🟦️.ts:27`, explicitly copies execution-environment markers into a one-core capacity request before the real rustc child. Its unit script passes the retained compiler-store environment at unit `📜️script.ts:36-37`; the generated Nx unit target is a real `bun ... unit compile` command (`units/🟦️.ts:36`). No local outer CPU acquisition is present in that unit script, but that alone does not establish the entire process ancestry is unpaid.
- Sealed Rust compiler-code construction, library `⚡️caching/🦀️cargo/📦️units/🏗️compiler/🔏️code/🟦️.ts:40-41`, acquires one core around in-process `compileNxAuthorizerSnapshotV1`; that function calls `Bun.build` directly (authorizer snapshot `🟦️.ts:22-26`). It releases capacity before invoking its later code-reader callback. There is no local nested capacity call in that compiler function.
- Python styling (`🖱️ui/🎨️styling/🏗️builder/🐍️python/📜️script.ts:18-21`), .NET styling (`🔷️dotnet/📜️script.ts:22`) and repository CMake (`library/🏗️build/🛠️cmake/📜️script.ts:57-61`) acquire around actual external tool phases; Python and CMake propagate marker/count. Their public helpers do not make the incoming environment authentic.

## Conclusion About A Global Flip

A global removal of inherited handling is not justified by private local borrowing alone. At least the Bun worker, declaration worker, native-driver jobs, Vite CLI/plugin chain and compiled test executable cross real process boundaries while a parent reservation can remain held. Making both sides pay without changing those ownership boundaries introduces actual self-blocking.

The multiple hard-coded lease directories also require an explicit physical-identity decision. This audit did not resolve `.cache/framework/leases`, process-owner configured directories and `.cache/b/l/m/data` to one shared inode or mutate any namespace. Storage is qualifying its current epoch; this lane must not rewrite those paths or Core/Native lease Source during that qualification.

## Proposed Narrow Next Scope: Paid Declaration Worker

The declaration emitter is the strongest bounded cut found. Its real computation is `emitTypeScriptDeclarations` in library `⚡️caching/📦️artifacts/🟦️typescript/🧬️declarations/🟦️.ts:7-48`: synchronous TypeScript compiler API calls and filesystem emission, returning only a declaration count. It contains no process spawn. The exact production subprocess callers are package-main `:56` and stdio composition `:38`, plus the public Nx `repo:emit-typescript-declarations` command in caching `📋️project.json:47-58`. Existing web-build testing directly invokes the pure emitter as an oracle; it is not another production child admission route.

Moving the emitter synchronously into the caller's event loop is rejected because it would prevent original cancellation/deadline/progress from running during expensive compilation.

The proposed schema-first worker contract is closed and domain-specific: version, declaration intent, absolute entry/workspace/output paths bounded in both portable length measures, fixed one-compiler-worker count, finite input/source/output/file/diagnostic budgets and one immutable request identity. Exact serialized UTF-8 request/result bounds are required in addition to per-field limits. Worker data contains work only: no Original object, grant, capacity marker, PID, resource name presented as authority, callback or arbitrary executable/module selector. The supported operation always uses the fixed declaration implementation.

The parent API must require the exact live issued Original control, register accepted work before reservation or worker birth, and acquire an actual one-core capacity lease with marker inheritance disabled for that boundary. A private in-process paid owner can lend that one actual core when the caller already owns it, retaining the actual grant through the worker's terminal join. Worker messages are bounded diagnostics/results and do not authorize local parent compute or manufacture a subgrant.

On cancellation/expiry/startup error/output refusal, stop births, request worker termination and await actual worker death before releasing the real core. A result message is not death; callback return is not death. Preserve caught Error and thrown `undefined` with explicit primary-failure state, and retain accepted detached failures in the canonical ledger. Parent process death ends its threads; this operation must be restricted to the fixed declaration compiler with no native child creation, rather than generalized to arbitrary JS/plugin work.

The two production parent declarations wrappers should be replaced together so they do not retain a duplicate reservation around an independently paying declaration operation. The public command must use the same paid owner. A private worker entry may reject direct main-thread/CLI invocation before loading the compiler; there must be no public flag or alternate CLI that performs unpaid declaration work. A parentPort/isMainThread check is an execution-shape guard, never proof of capacity. The root owner holding real custody is the authorization boundary. All remaining old cross-process marker ancestry must be explicitly handled before claiming this entry supports inherited cross-process CPU loans.

### Runtime Join Qualification Required Before Implementation

Node documents its Worker `exit` event as occurring after the worker stops and `terminate()` as a promise fulfilled by that event. This is a suitable contract candidate for an owned-thread interface, subject to actual runtime tests. [Node Worker Threads](https://nodejs.org/api/worker_threads.html#workerterminate)

Bun's current Worker docs explicitly describe its close event as termination being marked while full exit may still take time, and call worker termination experimental. Bun 1.3.14's `node:worker_threads` implementation resolves `terminate()` from the underlying WebWorker close event and emits Node-style `exit` from that same close event. Therefore Node API naming under Bun is insufficient evidence of a true join. [Bun Workers](https://bun.sh/docs/runtime/workers#terminating-a-worker), [Bun 1.3.14 Worker Threads implementation](https://raw.githubusercontent.com/oven-sh/bun/bun-v1.3.14/src/js/node/worker_threads.ts)

Do not implement an assumed Bun join. First qualify a provider with actual startup/cancel/terminal-work proof. A possible fallback is a real Node declaration host that owns the actual SQLite grant and creates only owned declaration worker threads. Its parent EOF/cancel path would retain that real host/grant until actual Node Worker join, using the existing exact carried Original rather than a copied control. Such a host must not create an additional paid parent in front of an already paid child or revive a deadline after readiness. Its real runtime/version/source entry and SQLite support must be validated before any production route changes. This is a proposed alternative, not an executed runtime claim.

### Required Tests For The Proposed Cut

1. Closed neutral declaration corpus: repository validator and Ajv agree on requests/results, exact UTF-8 and path bounds; existing TypeScript API emission supplies the independent output oracle.
2. Actual one-core grant precedes worker birth. Independent Python SQLite sees the real core locked during startup and held work. Forged environment markers and forged result messages cannot enter an unpaid supported compiler callback.
3. Both existing production callers and the public command route execute the same owner once, including under an actual parent local reservation, without a duplicate acquisition or enlarged count.
4. Cancellation, absolute deadline including preparation/startup, malformed response, startup failure and detached failure retain the real grant until actual termination/join. Assert final Original success/failure outside its receiver and preserve `undefined` primary failure.
5. Kill the actual paid parent process while held declaration-thread work exists; verify competing actual capacity cannot overlap that work. Use actual handles/terminal ordering and SQLite rather than PID liveness as custody.
6. Direct invocation of the private worker entry refuses before the emitter starts. No CLI marker or work-data field can opt into an unpaid path.
7. Strict Source and actual runtime proofs precede integration; no full SDK or normal Nx publication is needed for the private proof cohort. The 88-law custodian suite remains unchanged.

No implementation is authorized by this report alone. The root agent requested this design for review before the next Source change. General Cargo/native marker removal and durable arbitrary-descendant ownership remain separate work.
