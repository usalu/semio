# P9-A Rust Browser/Wasm ABI Removal — Sol High Packet Scout

## Scope and decision

This is a read-only, live-source census. It implements the Phase 9/10 plan’s stated direction — replace `wasm-bindgen`/`web-sys` with generated low-level imports/exports plus a small owned JS boundary — but does **not** regard a Cargo-row deletion as removal. A dependency is retired only when its declaration, all non-test source paths, public ABI leakage, generated-bindgen build path, and root lock entry are absent (or a documented platform-only oracle remains outside the product boundary).

The accepted Z0 owner-scope audit is authoritative for the inventory rule: literal external is still deliberately RED (`212`), Rust has `85` direct identities (`65 runtime + 2 build + 3 runner + 20 oracle`), and a mixed owner cannot be hidden by an identity-level exception. No Cargo/Nx/Wasm/browser command was run here.

**Recommendation: start P9-A1, the owned byte/message ABI kernel, not a leaf bridge.** It is the only prerequisite that makes the later packets non-overlapping and gives cancellation, lost-handle, malformed-input, and capacity behavior one contract rather than 60 divergent `JsValue` paths.

## Direct-owner census

| Identity | Direct Cargo owners | Source files mentioning the crate token | Classification |
| --- | ---: | ---: | --- |
| `wasm-bindgen` | 52 | 66 | 1 root workspace declaration; 51 product/runtime owners; several stale owners and macro/build glue remain |
| `wasm-bindgen-futures` | 19 | 22 | runtime scheduling/promise bridge; no dev-only declaration among these 19 |
| `web-sys` | 14 | 39 | browser platform/runtime boundary |
| `js-sys` | 18 | 37 | JS object/promise/typed-array runtime boundary |
| `serde-wasm-bindgen` | 7 | 3 | 3 active codec paths, **4 manifest-only stale rows** |
| `wasm-bindgen-test` | 1 | 0 product paths | test/oracle-only: `compose/client/lib/rs/Cargo.toml:44` |
| `instant` with `wasm-bindgen` feature | 1 | indirect browser timing | `compose/client/lib/rs/Cargo.toml:30`; platform transitive |
| `getrandom` with `wasm_js` | 4 | browser entropy transitive | Compose, CAD, Flow, and OS WGPU; must move behind owned entropy port |
| `console_error_panic_hook` | 2 | debug/panic ABI transitive | Compose-only direct owners |

There are **110 direct core-ABI declaration rows**: root workspace 1; Composition 9; plugins 55; framework 25; OS product 20. The core-row classification is product/runtime except the root workspace declaration and the separate `wasm-bindgen-test` oracle row. The source search finds 60 files with export annotations or browser/JS public types, so a manifest-only pass would falsely claim most of the work complete.

### Exact declaration owners, grouped without overlap

| Area | Direct rows | Cargo owners |
| --- | ---: | --- |
| Root coordination | 1 | `Cargo.toml:163` (`wasm-bindgen`) |
| Composition runtime | 9 | `compose/client/lib/rs/Cargo.toml:47-51`; `compose/client/lib/query/rs/Cargo.toml:34-37` |
| Framework UI/platform | 19 | `framework/editor/.../Cargo.toml:41-44`; `framework/actor/...:34-35`; `framework/machine/...:38-39`; `framework/ui/...:95-98`; `framework/ui/host/...:41-44`; `framework/ui/render/targets/webgpu/...:31-32`; `framework/surface/...:49-53` |
| Framework other/runtime | 6 | `framework/replication/...:31`; `framework/math/...:32`; plus workspace-inherited `wasm-bindgen` owners in plugin/framework component manifests listed below |
| OS product | 20 | `products/os/rust/Cargo.toml:63-67`; `os/infinite/...:58-61`; `os/flow/...:47-51`; `os/renderer/.../wgpu/Cargo.toml:110-113`; `os/host/...:58-59` |
| Plugin application owners | 31 | Writer `:41-42`; Procedural `:68-69`; GIS `:87-88`; Animate `:74-75`; Shooting `:53`; Sequence `:47-50`; FEM `:55-56`; Process `:59`; Layout `:42-47`; CAD `:71`; Imperative `:48`; Trinity `:56-59`; Draw `:40`; Raster `:40`; Note `:37`; Puzzle `:107-110`; Sourcing `:57` |
| Plugin extension/macro owners | 24 | Process concrete/robotic/wood/metal each `:36`; CAD aec-building-structure `:36`, aec-building `:45`, spatial-shape `:36`, aec-building-energy `:36`; Imperative control/text/effect/logic/math each `:46`; Trinity Jack LSP `:28`; Draw command FSM `:34-36`; Sourcing slabs/windows/beams each `:36` |

All unqualified `...:line` entries in the two plugin rows are under `✏️s/🔌️plugins/<plugin>/📦️packages/🦀️rust/Cargo.toml`, except the explicitly named artifact/extension paths. The complete mechanically collected declaration evidence is the 110-row read-only command output; the grouped count is conserved: `1 + 9 + 25 + 20 + 55 = 110`.

### Active import/export ownership

The high-risk owners are not every declaration. Their exact active regions are:

| Owner region | Present external type leakage | Existing owned seam | Packet |
| --- | --- | --- | --- |
| `framework/ui/host/.../window.rs:115-121, 238-273, 740-902` | `HtmlCanvasElement`, `ResizeObserver`, browser clipboard promises, closures | `ClipboardHost`, `WindowDelegate`, `WindowMetrics`, `FrameScheduler` | P9-A2 |
| `framework/ui/.../targets/webgpu/{host,gpu,text,cursor}.rs` and `ui/render/targets/webgpu/{backend,gpu_context}.rs` | canvas, WebGPU/JS values, browser spawn | existing renderer/host contracts | P9-A3 |
| `products/os/flow/🌉️wasm/component.rs:22-585` | `HtmlCanvasElement`, `JsValue`, `Promise`, ~70 bindgen exports | `FlowHost` JSON operations and state | P9-A4 |
| `products/os/store/worker/component.rs:22-92`; `store/sync/component.rs:2774-3033` | `JsValue`, `Reflect`, `Function`, `Uint8Array`, WebSocket callback lifecycle | `backbone_worker_wire::{decode_request,...}` | P9-A5 |
| `products/os/host/component.rs:4531-4555` | `JsValue` and `serde_wasm_bindgen` values | `WorkflowFixture`, `ArtifactPack`, `ArtifactDsl`, `format_accept_filter` | P9-A6 |
| OS renderer WGPU `browser_worker.rs`, `os_host.rs`, `winit_app.rs`, glue; Interpreter/ProgramBridge/Shell | workers, window/document, callback/promise values | renderer frame/job contracts | P9-A7 |
| Composition `client/lib/rs/lib.rs:642,18638-18763`; Query `lib.rs:340-372,1000-1097` | public re-export of all five crates; `Promise`, `Function`, `JsValue`, serde values | `ComposeTransport`, `Transport`, `OpPlan` | P9-A8 |
| Plugin editor bridges under `✏️s/**/✏️editor/🌉️wasm/🦀️component.rs` (Writer, Procedural 2D/3D, GIS, Animate, Shooting, Sequence, FEM 2D/3D, Process3D, Layout, CAD, Imperative, Trinity Jack, Raster, Puzzle 2D/3D/5D) | constructors, exported handles, `Uint8Array`, `Promise`, canvas types | each editor’s existing JSON/byte envelope and job handle | P9-A9, one plugin at a time |
| Cross-cutting non-leaf callers: framework editor/actor/machine/surface; OS Infinite, Flow bridge/VCS/drawing/host/directory/identity; Draw/Puzzle precompute/transfer/macro glue | scheduler calls, time/entropy, JS arrays/values | domain models and artifact/transfer codecs | P9-A10 |

Four `serde-wasm-bindgen` declarations have no source-token user on this snapshot: Layout, Surface, OS root package, and Flow package. They are candidate stale-row removals **only after** an import/path scan for aliases and generated glue in the owning crate; do not combine them with a behavior rewrite.

## Collision-safe packets

### P9-A1 — Owned ABI/data-transfer kernel (first)

**Files:** new owned ABI schema/codec beside `framework/🧬schema` and `framework/⏳async`; no browser owner source changes. Do not touch the 110 rows yet.

**Contract:** schema-first `AbiRequest { operation, request_id, generation, bytes }`, `AbiReply { request_id, generation, status, bytes }`, `AbiEvent`, bounded `AbiPage { handle, index, bytes }`, and `AbiControl::{cancel, close, acknowledge}`. All fields are Rust primitives/owned IDs/`Vec<u8>`; errors are owned codes plus bounded message bytes. The generated import/export shim is the only code allowed to translate linear-memory pointers/lengths to JS `Uint8Array`; no `JsValue`, `Promise`, `Function`, `web_sys::*`, or serde-wasm value crosses into product Rust APIs.

**Fixtures/mutations:** golden empty/single/max page, deterministic request/reply ordering, max and max+1 byte/page/operation counts; malformed tag/length/UTF-8; missing optional field; unknown handle; duplicate acknowledge; callback interruption; cancel before/after seal; drop/lose handle then late reply. Verify codec parity against the present JSON/byte fixtures without using a browser.

**Gates:** focused owned-codec tests, static deny-list outside the shim, schema fixture parser, and source census showing no external ABI type in exported domain signatures. No root Cargo/lock mutation.

### P9-A2 — Framework UI browser-host adapter

**Files:** only `framework/ui/host/.../window.rs`, its Cargo manifest (`:41-44`), and co-located owned host fixtures. Replace direct canvas, resize, clock, clipboard and event callbacks with an owned `BrowserHostPort` implemented by generated ABI imports. Preserve `WindowDelegate`, `ClipboardHost`, frame invalidation and cursor semantics; they are the existing platform-neutral boundary.

**Behavior:** missing window/document/clipboard returns an owned unavailable result; resize and pointer storms coalesce; cancelled clipboard/read callbacks cannot change a later generation; listener/handle loss releases callback resources. Golden fixture: pointer/resize/cursor/clipboard event trace. Hostile: missing optional browser object, rejected promise, callback throws/interruption, close during callback, max/+1 listener queue.

### P9-A3 — Framework browser WebGPU surface

**Files:** only framework UI WGPU target and `ui/render/targets/webgpu` source/manifests. Depend on A1/A2, not on OS renderer. Expose owned `SurfaceId`, `CanvasMetrics`, input envelopes, and `GpuOutcome`; JS owns canvas/WebGPU objects. No public canvas or JS type in Rust.

**Fixtures:** surface create/resize/frame/drop trace, device-lost and stale-generation frame results, max/+1 in-flight frames. Static gate: all `web_sys|js_sys|wasm_bindgen` references are confined to the generated shim, and manifest removals occur only after the owner source paths are gone.

### P9-A4 — Flow editor ABI

**Files:** `products/os/flow/🌉️wasm/component.rs:22-585`, `products/os/flow/.../Cargo.toml:47-51`, and Flow JS host only. Convert every JSON operation to A1 request/reply operations; split canvas attachment into `SurfaceId` admission and an asynchronous owned status event. Preserve existing `FlowHost` domain API.

**Fixtures:** the existing Flow fixture/catalogue/selection JSON becomes byte fixtures. Hostile: malformed JSON bytes, unknown optional selection fields, attach interrupted by close, cancelled GPU create, stale handle, page max/+1.

### P9-A5 — Store worker and sync transport

**Files:** `products/os/store/worker/component.rs:22-92`, `store/sync/component.rs:2774-3033`, owning manifest `products/os/rust/Cargo.toml:63-67`. Reuse `backbone_worker_wire` as the wire schema; replace JS reflection/WebSocket callback values with `AbiEvent` pages and owned connection state. This packet must not edit Flow or renderer.

**Fixtures:** ready/request/reply reconnect trace, cancellation before delivery, duplicate/late messages and generation rejection. Hostile: non-binary message, missing `wire`, malformed length, post callback failure, listener lost, queue max/+1.

### P9-A6 — OS host codec-only cleanup

**Files:** `products/os/host/component.rs:4531-4555`, `products/os/host/.../Cargo.toml:58-59`, and stale serde owner manifests only after A8 confirms no shared re-export. Replace `serde_wasm_bindgen::{to,from}_value` with owned byte codec calls. Exported `decodeWorkflowFixturePack`, parser, accept-filter, and normalizer become operation codes, not bindgen functions.

**Fixtures:** existing pack/DSL/error strings serialized as deterministic owned reply bytes. Hostile: malformed pack/DSL, missing kind array, unknown kind, input max/+1. This is a small, independent implementation packet after A1.

### P9-A7 — OS WGPU renderer browser host

**Files:** only OS renderer WGPU target (`Cargo.toml:109-113`, browser worker/host/winit/glue) and Interpreter/ProgramBridge/Shell bridge regions. Requires A1-A3. The host owns JS Worker/window/canvas objects; Rust receives byte events and returns pages/frame outcomes. Preserve frame budget/cancellation semantics in the existing renderer job contracts.

**Fixtures:** startup, worker-ready, resize/render, device loss/recreate, close during frame, and a 8-ms callback watchdog trace. Hostile: bad worker message, missing canvas, interrupted ready callback, stale worker generation, lost page handle, max/+1 in-flight frames.

### P9-A8 — Composition public API quarantine

**Files:** `compose/client/lib/rs/lib.rs:642,18638-18763`, `compose/client/lib/query/rs/lib.rs:340-372,1000-1097`, and their two manifests. First remove the public re-export at `lib.rs:642`; then make `ComposeTransport` accept an owned async message port, and map `architectCompile`/`architectRun` to byte operations. `wasm-bindgen-test` remains a test oracle until equivalent owned ABI fixtures are stable.

**Fixture:** GraphQL query/subscribe trace with accepted values and callback ordering. Hostile: malformed request JSON, callback throw, stream cancellation, lost subscription handle, max/+1 queued events.

### P9-A9 — Plugin bridge migration, serialized leaf packets

One plugin bridge per packet, in this order: Sequence/Layout/Puzzle (already explicit byte/canvas/Promise patterns), then Writer/GIS/Process/FEM/Procedural/CAD/Imperative/Trinity/Raster/Animate/Shooting/Draw. Each changes exactly one plugin Cargo manifest plus its artifact bridge source and generated host wrapper. Extension declarations with no matching source use are a separate stale-declaration packet, never piggybacked.

Each leaf maps constructor/handle/page/cancel/ack exports to A1. Canonical fixtures replay existing editor commands; hostile variants are malformed page bytes, omitted optional command payload, callback interruption, cancel/close during compute, handle loss, and every configured bound at max/+1. A leaf cannot remove shared root/workspace declarations or touch another plugin manifest.

### P9-A10 — Residual call-site and transitive closure

Only after A2-A9 are green, remove remaining direct source use in framework editor/actor/machine/surface, OS Infinite/directory/identity, and plugin Draw/Puzzle helper paths. Replace time, entropy, spawn-local, JS value/array and console use through owned `ClockPort`, `EntropyPort`, `TaskPort`, and diagnostics event ports. This packet owns `getrandom wasm_js`, `instant wasm-bindgen`, `console_error_panic_hook`, and final direct ABI rows only when their source callers are gone.

## Lock and integration rules

1. **One final root integration owner.** Leaf packets edit no root `Cargo.toml` or `Cargo.lock`; they record the exact removed manifest/source paths. The integrator removes root workspace `wasm-bindgen` only after no `.workspace = true` owner remains.
2. Reconcile `Cargo.lock` once after all direct core rows and browser-adjacent transitives are absent. A lock-only disappearance proves nothing; a manifest-only disappearance is rejected when `rg` still finds token imports/attributes/types.
3. Separate product runtime from oracle/build. `wasm-bindgen-test` and any generated-bindgen tooling stay explicitly classified until their owned equivalent is tested; they cannot satisfy the product-runtime exit gate.
4. Root dependency verifier integration is one isolated packet in `📜️script.ts`: add owner+source-path assertions and hostile self-mutations; do not loosen Z0 literal-external accounting or alter its baseline during ABI work.
5. The final static gate must search both hyphenated manifest names and Rust identifiers/attributes: `wasm-bindgen|wasm_bindgen|wasm-bindgen-futures|wasm_bindgen_futures|web-sys|web_sys|js-sys|js_sys|serde-wasm-bindgen|serde_wasm_bindgen`, including generated glue and public re-exports.

## Ordered handoff

`A1 → A2/A6 → A3 → A4/A5/A8 → A7 → A9(serialized) → A10 → root/lock/verifier integration`.

This ordering prevents the two likely collisions: A7 cannot invent a renderer-local browser protocol before A1-A3, and plugin leaves cannot independently change root Cargo/lock or shared ABI semantics.
