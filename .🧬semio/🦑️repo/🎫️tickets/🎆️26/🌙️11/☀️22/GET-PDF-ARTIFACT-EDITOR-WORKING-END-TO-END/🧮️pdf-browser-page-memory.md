# 🧮️ PDF editor page memory in the browser (2026-11-25)

Follow-up to "End-to-end verification, round 5", section 5 of `🌐️pdf-browser-journey.md`: one headless browser running one PDF editor page passed the host warden's 4,096 MB `browser` cap during New Document → Save and during Open File and was SIGKILLed.

No cargo, no rustc, no wasm rebuild: the staged component `7653cb9f…` (285,191,177 bytes) was transpiled again from the stripped component the round-5 stage left in `🗑️generated/wasm-transpile/component.wasm`. One headless Chrome at a time, closed after each measurement. The warden was not changed.

## 1. How it was measured

* `probe-memory.ts`: one page on the native host (port 6260). It boots, edits the title, opens `2️⃣two-pages.pdf` `PROBE_OPENS` times (default 2) and waits 20 s. At each stage it records:
  * the **warden's own measure**: `ps rss` summed over the runner and every process below it, broken down by Chrome process type;
  * the **physical footprint**: `top` MEM over the same tree, plus the renderer's footprint and its kernel-tracked lifetime peak from `vmmap -summary`;
  * the in-page and per-worker **wasm inventory**. A temporary `[DEBUG]` prelude is prepended to the staged `🟨️shard-worker.js` (and added as a page init script), and the original file is restored after every run. It counts `WebAssembly.compile`/`compileStreaming`/`instantiate`, tracks every `WebAssembly.Module` and `WebAssembly.Memory` through `WeakRef`s after a forced `gc()`, and records message volumes between the page and its workers;
  * optionally: Chrome memory-infra allocator dumps, a dump at a threshold, `vmmap` at a threshold, a `sample` of the renderer's threads, V8 and Chrome flags, and the shard pool size.
* `probe-compile-sharing.ts`: what one compilation of the 285 MB core module costs in a blank page, and what a second worker pays when it compiles the module again versus when it receives worker 1's module by structured clone.
* `wasm-memory-layout.ts`: the core module's sections, memory limits, data segments and function size distribution.
* `pdf-memory-budget.ts` + `pdf-memory-budget.json`: the journey-level budget (section 5).

The probe closes its own browser at `PROBE_ABORT_MB`, before the warden's SIGKILL. A run that ends there is counted as "crossed the cap" below.

## 2. Where the memory goes

### The component

| | |
| --- | --- |
| Core module | 271.9 MB: **code 258.9 MB in 573,605 functions** (median 121 B, largest 379 KB, 15 above 100 KB), data 11.6 MB in 2 segments at 8 MB (the 8 MB shadow stack is below them) |
| Linear memory | declared initial 344 pages = 21.5 MB, maximum 8,192 pages = 512 MB. A booted instance holds **104 MB**: 106 MB after the title edit, 114–116 MB after an Open |
| JS glue | 757 KB (automatic) / 798 KB (factory) of transpiled jco JavaScript |

### The browser tree (baseline, the original glue)

| Stage | Tree RSS (warden) | Renderer RSS | Notes |
| --- | --- | --- | --- |
| Boot | 2,548 MB | 1,992 MB | browser 88, GPU 152, network 122, runner 194 |
| Title edited | 2,530 MB | 2,016 MB | |
| Open File #1 | 2,709 MB (peak during the open: 3,691) | 2,273 MB | |
| Open File #2 | 2,743 MB | 2,362 MB | |
| Settled (+20 s) | 2,816 MB | 2,429 MB | |

In other baseline runs the first Open reached 3,733, 3,924, 3,963, 4,023, 4,385 and 4,418 MB. Five of seven runs crossed about 3.9 GB, and most of those crossed the cap.

In-page inventory at "Settled", baseline. **One** program was open, yet:

* **3 compilations** of the 285 MB core module: one per program instance (boot instance in worker 1, opened instance in worker 2, re-opened instance in worker 1 again), plus the two tiny trampoline modules per instance.
* **3 live linear memories, 329 MB** (116 + 104 in worker 1, 109 in worker 2). The retired predecessors' instances are never released.
* 4 shard workers plus one other worker. Each activation imported its own copy of the 757 KB glue.

### What a compilation costs (`probe-compile-sharing.ts`, blank page)

| Step | Peak | Settled |
| --- | --- | --- |
| Blank page | 396 MB | 396 MB |
| Worker 1 compiles the core module (streaming) | 1,032 MB | 768 MB (+372: the wire bytes V8 keeps, plus metadata) |
| Worker 2 compiles the same bytes again | 1,083 MB (**+306 transient**) | 777 MB (+9: V8's native module cache shares the code) |
| Worker 2 receives worker 1's `WebAssembly.Module` by structured clone | 779 MB (+1) | 779 MB |

### Allocator attribution at the first-Open peak (memory-infra dump at 4,149 MB)

* Renderer: `malloc` 1,999 MB, of which PartitionAlloc's malloc partitions hold 702.
* `v8` 1,428 MB in total. Of that, `v8/workers/malloc` is about 210–230 MB per isolate. It is the same for idle workers, so it is the shared wasm engine's allocator reported once per isolate, not 5 × 220 MB.
* JS heaps are small (main 90 MB, all workers together 42 MB), Blink GC 66 MB, compositor tiles 49 MB.
* Message volumes are small: each `activate` carries 8.4 MB of assets, and the 3,006 turn receipts total under 1 MB.
* Linear memories stay at 100–116 MB per instance.
* `vmmap`: the PartitionAlloc region (tag 253) is 1.4 GB resident at boot and 2.3 GB at the peak, but only **0.58 GB dirty in both cases**. The renderer's physical footprint at that moment was 1.6 GB, against 3.4 GB RSS.

### V8 flag diagnostics (measurement only, not a product setting)

| Flags | Renderer peak footprint during Open #1 | Tree RSS peak |
| --- | --- | --- |
| default | 2,765–3,072 MB | 3,461–5,051 MB |
| `--no-wasm-tier-up` (Liftoff only) | 2,867 MB | crossed the cap |
| `--no-liftoff` (TurboFan for lazy compilation) | **2,048 MB** (2,458 later) | **3,043 MB**, settled 1,960 MB |
| `--no-wasm-lazy-compilation` (all of Liftoff at boot) | cap crossed **during boot**: footprint 2,458 MB, peak 3,482 MB | 4,155 MB |
| one shard worker (`hardwareConcurrency` = 2) | 2,765 MB | 3,821 MB / crossed the cap |

## 3. Causes

1. **Every program instance was a module-scope singleton that could never be released.** `pluginComponentBridgeSource` imported the jco module (automatic instantiation) under a per-activation URL (`?actor=…&activation=…`). The browser keeps every imported ES module for the page's lifetime. The glue keeps `exports0`/`memory0`/… in module scope, so each activation added its own 757 KB glue module, and the instance and its linear memory stayed reachable forever.
2. **Each activation compiled the core module again.** The glue's `fetchCompile` streamed the 285 MB file for every instance. A worker with an earlier compile still got a new `WebAssembly.Module` (V8's native module cache later shares the code), and each compile paid about 300 MB of transient memory and about 0.5 s.
3. **A disposed activation was kept whenever the guest did not close its original source.** On `dispose` the worker deleted the actor only when `closeOriginal()` answered `sourceClosed: true`. For a retired PDF program the guest's `closeActorContext` answers `complete=false, blocked=false` with zero spend, observed for every retirement in this journey. So the worker kept the whole instance although nothing would ever call it again. No path retries a close, and `disposeRequested` blocks a second dispose.
4. **The first Open lazily compiles a large share of a 259 MB code section with Liftoff.** This is the remaining peak:
   * Boot compiles the boot path. The first real Open runs the PDF import paths for the first time and adds about 600 MB of permanent renderer footprint (1,024 → 1,638 MB), with a transient peak of 2.8–3.1 GB.
   * Eager Liftoff of the whole module costs at least 2.4 GB. TurboFan-only lazy compilation cuts the Open peak by about 0.8 GB.
   * It is neither TurboFan tier-up, nor the module compile (now done once), nor concurrency across workers, nor messages, JS heaps or linear memory.
5. **The warden's measure counts memory that is already freed.** On macOS, PartitionAlloc returns free pages with `MADV_FREE_REUSABLE`. They stay in RSS until the kernel reclaims them, but they are not in the physical footprint: 2.3 GB resident against 0.58 GB dirty. The same Open therefore reads anywhere from 2.98 to 5.05 GB on the warden's `ps rss` sum, with a peak footprint of 2.8–3.1 GB.

## 4. Fixes (product code)

1. **The component is a jco instantiation factory** (`--instantiation async`, `nativeTranspileArgsV1`). There is one module per rebuild version per worker, and each activation instantiates it with its own host and core modules. Nothing about an instance lives in module scope, so an instance is collectable once its API is dropped.
   * `rewriteJcoComponentFactory` (replacing the asset-URL and host-URL rewrites) adds the WASI interfaces the factory names as static preview2-shim imports and exports them as `wasiImports`. They are relocated to the vendor directory as before, so the shard worker's diagnostics switch still reaches the same `cli.js` instance.
   * The host shim is now a factory as well (`createActorHost(actorId, activationGeneration)`), so its clock binding, checked-out turn and pending effects die with the activation.
   * The bridge refuses a module that is not a factory with a typed message.
2. **Each core module is compiled once per page and shared by structured clone.** `ShardCoreModules` (`🎭️actor/📮️shard-client/🧊️core-modules`, schema-first, embedded into the generated worker by source text) keys modules by the URL they are served from (URL plus rebuild version: the identity the module map already gives the glue that asks for them) and holds them per activation.
   * A worker compiles a URL once, from the streamed response (no extra `ArrayBuffer` copy). It reports every hold to the page as `core-module`.
   * The page keeps the module as a seed for every later activation on any worker (`activate.coreModules`).
   * Both realms release a URL with its last holding activation (worker: on dispose; page: `disposeActivation`, and `failShard` for a lost worker).
   * A URL key rather than a content hash: a hash would have to be computed at staging time and could go stale against a later `wasm-opt` rewrite of the same file. The served URL with its rebuild version cannot.
3. **A disposed activation is always dropped.** `closeOriginalActor` deletes the actor and releases its core-module holds whether or not the guest closed its source. An unclosed source stays held in the kernel's stock, as the `sourceClosed: false` report says, but its instance and memory go. This also lets the same actor id activate again on that worker.
4. **Every consumer is on the new shapes:**
   * the descriptor probe instantiates the factory in Node;
   * `assertPluginOutputChildren` admits every `*.core<N>.wasm`;
   * the test kits' fake guests and hosts are factories (`guestFactorySource`, `hostFactorySource`, `noCoreModule`);
   * the worker inbox inventory fixture lists the new message kind and field;
   * the ticket's `stage-pdf-component.ts` stages every core module and lists them in `.semio-artifact.json`.

### Result (same probe, final code)

| Stage | Tree RSS (warden) | Tree footprint | Renderer footprint (peak) |
| --- | --- | --- | --- |
| Boot | 2,578 MB | 1,273 MB | 998 MB (1,536) |
| Title edited | 2,551 MB | 1,301 MB | 1,024 MB |
| Open #1 | 3,308 MB | 1,962 MB | 1,638 MB (**2,867**) |
| Open #2 | 3,674 MB | 2,092 MB | 1,741 MB |
| Open #3 | 3,608 MB | 2,104 MB | 1,741 MB (growth 0) |
| Settled | **3,064 MB** | 1,920 MB | 1,638 MB |

Before → after:

| Measure | Before | After |
| --- | --- | --- |
| Compilations of the 285 MB module | 3 | **1** (worker 2 adopted the seed: 0 compiles, instantiation in under a millisecond) |
| Live linear memories with one program open | 3 (329 MB) | **1 (104 MB)** |
| Renderer footprint after Open #2 | 1,946 MB | **1,536–1,741 MB** |
| Renderer footprint growth per further Open | +205 MB | **0 to −100 MB** |
| Glue copies | one per activation | one per worker |

The run without the dispose fix (factory and cache only, probe label `fp-default`) settled at a renderer footprint of 1,843 MB with 3 live memories. With the dispose fix the same journey releases instance 1 at Open #2 and instance 2 at "settled".

**Still open:** the first Open's lazy compilation. The renderer's peak footprint (2.8–3.1 GB) did not change, because it is code, not modules or instances. The warden's RSS during Open #1 still crosses 4,096 MB in roughly half the runs: final-code first-Open peaks were 3,461, 3,847 and 3,869 MB, and 4,054, 4,095, 4,116, 4,373, 4,661, 4,829 and 5,051 MB. Only a smaller code section removes it (section 6).

## 5. Tests

* **`🎭️actor/📮️shard-client/🧊️core-modules/🧪️tests/🟦️.ts`** (vitest, registered in the actor config), driven by the language-agnostic fixture `🧪️tests/🧫️fixtures/🔣️.json` under `🧬️schema/🔣️.json`, and validated with Ajv (the third-party check). Cases:
  * one compilation for many activations;
  * a seed adopted without compiling;
  * the last holder releases the entry;
  * a closed instance releases every core of its component;
  * a released URL compiles again;
  * an unknown holder drops nothing.

  V8's own `WebAssembly` proves that one compiled module serves N instances with independent memories, and that closing them releases the entry. A failed compilation leaves no entry behind. The class runs unchanged from its embedded source text. **10/10 pass**, and the whole actor suite passes: 119/119.
* **Plugin actor transport suite** (os vitest, 183 tests: worker inbox inventory, generated-worker traffic, codec-reply bridges, segmented download through the generated bridge): **183/183**.
* **Bun:** `🧬️wit-lowering` 6/6, `🔁️succession` + `🚪️lifetime` 5/5.
* **Dev staging suite** (`🧬️jco-rewrites`, `🛂️routes-and-bridge`, test kit): updated and syntax-checked. It could not be collected in this session because another session's `🧑‍💻dev/🚚️distribution/🏗️compiler/🟦️.ts` currently fails to transform (`Unexpected ")"` at line 45).
* **Type check** (`tsc --strict`) of the shard client, core modules and every changed materialization source: clean in the touched areas.
* **Journey budget:** `bun pdf-memory-budget.ts` runs the probe with 3 opens and asserts `pdf-memory-budget.json`:

  | Budget | Limit |
  | --- | --- |
  | Core module compilations | = 1 |
  | Live linear memories after settling | = 1 |
  | Renderer growth per further Open | ≤ 128 MB |
  | Settled tree RSS | ≤ 3,584 MB (512 MB below the cap) |
  | Renderer peak footprint | ≤ 3,328 MB (768 MB below the cap) |
  | The journey ran to the end below the warden cap | — |

  `budget-1` passed all six (1 / [104] / 0 / 3,064 / 2,867, sampler peak 3,847 MB). `budget-2` was killed by the warden during Open #1, and `budget-3` stopped at 4,116 MB. The first verdict is the open guest-side issue below, and is deliberately not relaxed.

## 6. What a wasm rebuild still has to change (guest side)

1. **Shrink the code section; it is what the renderer pays for.**
   * The browser component is the 20-app `semio_hub_stdio_pdf` composition at the prototype profile: 259 MB of code in 573,605 functions.
   * Liftoff's machine code is a multiple of that: at least 2.4 GB for the whole module, about 600 MB for what the first Open runs.
   * Build the browser deliverable with `opt-level = "s"` (or `"z"`), `codegen-units = 1`, `lto = "fat"` (or `"thin"`), and run `wasm-opt -Oz` on it in the prototype profile as well.
   * Compose only the apps the page serves, either as a PDF-editor-only component or as one component per app, instead of shipping the hub monomorphization.
   * Expect the peak to scale with the code: TurboFan-only lazy compilation, whose code is denser, already cut the first-Open peak by about 0.8 GB.
2. **Compilation hints:** mark the hot start-up and Open paths with wasm compilation-priority hints (`metadata.code.compilation_priority`) where the engine supports them, so they compile with the denser top tier instead of Liftoff.
3. **Stack:** `-zstack-size=8388608` (8 MB) was sized for opt-level-0 frames, where `UiPatchApplyArena::default` alone spent about 654 KiB. Once the browser build is optimized, measure the deepest frame chain and reduce it: every instance commits and touches its stack.
4. **Boot heap:**
   * An instance starts at 21.5 MB and grows to 104 MB during `instance-open`, before any document is open. Find out what the first open allocates (UI arenas, descriptor, fonts) and keep it lazy or shared.
   * The declared 512 MB maximum is right. Keep it: it bounds the guard reservation per memory.
5. **Static data** (11.6 MB) is fine. It is copied into every instance's memory, so large tables that never change belong in it only if they are needed at boot.
6. **Context close:** a retired program's `closeActorContext` answers neither complete nor blocked with zero spend, so its original source is never returned to the kernel's stock. The worker now drops the instance anyway, but the stock keeps that source held (funds forfeited). This belongs to the original-work-funds owner.

Not changed and only recommended: the host warden measures `ps rss`. On macOS that counts PartitionAlloc's already-freed `MADV_FREE_REUSABLE` pages, which explains the 2.98–5.05 GB spread for a 2.8–3.1 GB peak footprint. Measuring `phys_footprint` would make the cap track memory the browser really holds.

Evidence: the probe rows (`🗑️generated/memory-<label>.jsonl`), dumps, `vmmap` and `sample` outputs of this measurement were deleted after the numbers above were taken from them. The cleanup glob `probe-*.log` also removed round 5's `🗑️generated/probe-open.log` (and its host log), which `🌐️pdf-browser-journey.md` round 5 cites. Rerun `run-probe-open.sh` to recreate it.

## 7. Changed files

Product:
- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧊️core-modules/🟦️.ts`, `🧬️schema/🔣️.json`, `🧪️tests/🟦️.ts`, `🧪️tests/🧫️fixtures/🔣️.json` (new)
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📮️shard-client/🧊️core-modules/🟦️.ts` (new facet), `📦️packages/🟦️typescript/package.json` (export `./shard-client/core-modules`)
- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🗃️state/🟦️.ts`, `📨️messages/🟦️.ts`, `🪪️exact-activation/🟦️.ts`, `📬️request-transport/🟦️.ts`
- `🧰️framework/🔨️modules/🎭️actor/🧪️tests/🎚️config/🟦️.ts` (suite registered)
- `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/📥️inbox/🧪️tests/🧫️fixtures/🔣️.json`
- `…/🔌️plugin/🌐️browser-bundle/🏗️materialization/`:
  - `🌉️component-bridge-source/🟦️.ts`
  - `🪢️host-shim-source/🟦️.ts`
  - `🧬️jco-module-rewrites/🟦️.ts`
  - `🦀️native-transpile/🟦️.ts`
  - `🧾️contract/🟦️.ts` (`PLUGIN_HOST_IMPORT`)
  - `🧵️shard-worker-source/🔌️actor-loading-and-replies/🟦️.ts`
  - `🧵️shard-worker-source/📨️message-handler/🟦️.ts`
  - `🟦️.ts`
  - `🚀️commands/🟦️.ts`
- `…/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts` (factory probe), `…/🔌️plugin/🏗️build/🛂️descriptor/🟦️.ts` (core files, unused import removed)
- `…/🧑‍💻dev/♻️activation/📥️installation/🟦️.ts`, `🏃️execution/🟦️.ts`, `🧰️preparation/🟦️.ts`, `✏️s/🧑‍💻dev/🧩️catalog/🌐️browser-host/🏗️staging/🟦️.ts` (unused import removed)

Tests:
- `…/🔌️plugin/🧪️tests/✂️worker-vm-source/🟦️.ts` (factory helpers)
- `🧬️component-codec-reply/🟦️.ts`
- `🧪️actorreturnresponseframing-uses-canonical-vectors-with-no-payload-copies/🟦️.ts`
- `🧪️shardclient-reserved-response-settlement/📥️segmented-download-and-multiplexing/🟦️.ts`
- `…/🏗️materialization/🧪️tests/🧬️wit-lowering/🟦️.ts`, `🔁️succession/🟦️.ts`
- `…/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🧰️test-kit/🟦️.ts`, `🧬️jco-rewrites/🟦️.ts`, `🛂️routes-and-bridge/🟦️.ts`

Ticket:
- New: `probe-memory.ts`, `probe-compile-sharing.ts`, `wasm-memory-layout.ts`, `pdf-memory-budget.ts`, `pdf-memory-budget.json`, `refresh-staged-glue.ts`, `run-actor-vitest.sh`, `run-os-vitest.sh`, `run-dev-vitest.sh`, this file
- Changed: `stage-pdf-component.ts`
- Other tickets' scripts renamed to `rewriteJcoComponentFactoryAt`: `GET-FEM-PLUGIN-WORKING-END-TO-END/native-fem-materialize.ts`, `GIS-MAP-END-TO-END/prepare-gis-descriptor.ts`, `REMODEL-PLUGIN-END-TO-END/🧪️browser-wasm-descriptor-probe.ts`

Staged products image: `📘️stdio-pdf` restaged as a factory (now `core.wasm`, `core2.wasm`, `core3.wasm`), with the new bridge, host shim and shard worker. Other staged modules (for example `🗒️note`) keep their old bridge and glue until they are materialized again. The new shard worker only adds a trailing argument they ignore. A bridge regenerated over old automatic glue fails with `actor-component.not-a-factory`.
