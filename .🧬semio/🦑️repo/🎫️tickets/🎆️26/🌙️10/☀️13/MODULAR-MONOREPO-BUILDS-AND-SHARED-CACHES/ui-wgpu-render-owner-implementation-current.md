# UI WGPU Render Owner Source Handback

This is a Source-complete extraction handback, pending the explicitly held Rust validation window. It claims no Rust compile, native GPU, browser GPU, raster output from our GPU implementation, throughput gain, or full SDK publication. The earlier design-only qualification remains in `ui-wgpu-render-owner-design-current.md`.

## Actual Owner and Contracts

`semio-framework-ui-wgpu-render` now owns private draw pipeline, shader text, GPU session, readback and EngineCanvas execution under `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw`. Its root is `🦀️.rs`, with actual implementation in `🔧️pipeline/🦀️.rs`, `🎨️shaders/🦀️.rs`, `🎮️execution/🦀️.rs`, `🎮️execution/📤️readback/🦀️.rs` and optional `🎮️execution/🎬️canvas/🦀️.rs`. Old aggregate GPU/shader implementation mounts were removed. The aggregate UI no longer publicly supplies backend draw/GPU modules. Neutral polygon triangulation and IconAtlas values moved into the existing draw-contract owner; mesh/scene/prepared/raster/theme contracts remain their existing firstparty owners.

The public execution seam exposes opaque `GpuContext`, present/readback cursors, existing bounded mesh/raster custody values, and owned `EngineRasterBuild` with `Returned`/`Retained` refusal carrying the original residual build. Backend Device/Queue/Surface/Texture/View objects are private. A real private `GpuSessionRoots` owns Device, Queue and command Buffer; an EngineRasterBuild retains that same allocation and is checked against the actual session before allocation/render/close. No integer or serialized marker grants execution. Each texture/view/renderer birth checks the existing exact raster admission. Render rechecks the exact admission too. Stage Returned reinstalls original admission/texture/view; Stage Retained leaves those actual resources in the existing table close cursor while returning actual renderer/session residuals. Close retires admission, renderer, view, texture and session reference in bounded steps. Terminal requires physical owned fields empty. Existing table admission/retirement Source was preserved.

`📥️platform/🦀️.rs` captures the actual native surface before delegating device preparation and consumes the actual transferred browser canvas before its preparation. Only the necessary native Window/browser OffscreenCanvas carrier is explicitly reexported at ingress, as approved. Raw WGPU objects do not leave the child. Offscreen contexts share the actual device and queue but receive their own command buffer/session/table, so they cannot adopt one another's pending admissions.

## Concrete Consumers and Registration

Product renderer's native winit app, browser worker, renderer runtime, Shell icon export and IconRenderHost export now consume `ui_gpu` directly. EngineCanvas retains product surface/document/scene/metrics freshness and owns only the opaque child build. Its exact refusal arm reinstalls the original child owner before beginning close. Its old product-local texture creator and raw renderer/texture/view fields were removed. Infinite aggregate, world bridge and world host consume the child directly; native resource tests observe actual retained mesh lease/count/byte metrics without accessing GPU buffers.

Framework workspace member, root workspace dependency, child manifest/project, aggregate UI feature edge and direct consumer manifests are authored. Child prepared dependency explicitly keeps the existing browser-presenter guard. The `canvas-raster` API is excluded on WASI p2 where the actual Canvas native renderer is absent. Wasm prototype opt-level 1 matches the former aggregate UI's inherited policy; no throughput tuning is claimed. Child metadata bounds the actual production+native-test closure at 768 KiB/64 files/12000 active lines. Pure Bun TOML inspection verified actual child dependency paths/package identities, member and lib mount.

The actual `@semio-tech/ui-wgpu-render-rs` row is present exactly once in `📋️build-owners.json` (it appeared through concurrent catalog work before this lane's guarded insertion, so no duplicate was added). Six child build/check/test launch rows and the four existing ticket test/types rows appear in both launch and seed. All ten rows in each catalog were checked for both required cache-disabling environment entries. Unrelated launch/catalog/profile rows were preserved. Root owns lock refresh; this lane invoked no Cargo/store/compiler operation.

## TDD and Source Evidence

The neutral schema/corpus first gained four exact original-root accounting vectors for Returned/Retained, including missing/duplicate ownership refusals. Independent SQLite validates these expected outcomes; it creates no GPU authority. The existing Canvas clip/DPR oracle (7 cases/696 pixels), Sharp RGBA/BGRA padded readback oracle and six finite ownership traces remain. New actual Source closure checks caught and corrected the initially missed moved readback test mount. A Source marker diagnostic confirmed existing retained raster contract markers and all 39 mutation targets remain active after moving the implementation; this diagnostic is not a Rust test result.

Real child Rust unit tests were authored for foreign actual sibling session rejection and stale exact reservation refusal before first texture allocation, retaining original session/admission through issuer close. Existing real draw/raster/present/readback tests moved into the child with exact shared fixture paths. They have not executed while the Rust hold applies. Existing native surface ordering/type laws and product raster Source laws were updated for the actual capture/build API. The theme feature law hash changes only to reflect its authored new pipeline Source path.

Final finite 120-second Source-owned harness runs, with unchanged outer Original budget, all terminal and collected:

| Gate | Handle | Outcome |
| --- | --- | --- |
| test prototype | 57648 | exit 0, 8 tests, 898 expectations, 5.16s |
| test release | 31818 | exit 0, 8 tests, 898 expectations, 5.10s |
| strict types prototype | 60838 | exit 0 |
| strict types release | 7702 | exit 0 |

Logs are `🗑️generated/ui-wgpu-extraction-qualified-{test,types}-{prototype,release}.log`. Preceding Source REDs are retained: missed mount, complete-test-closure byte cap, and the default five-second Bun per-test timeout under parallel load. The grouped Source audit now has an explicit finite 20-second per-test allowance inside the original unchanged 120-second envelope. No Actor/Cargo timing policy changed. Other terminal handles collected: 73986, 95701, 59867, 64350, 62962, 71223, 99462.

Production native Source closure is 6 files/436473 bytes/7062 active lines on macOS/Linux/Windows; browser is 7079 active lines and WASI p2 7047. Actual macOS canvas+test closure is 25 files/625256 bytes/10279 active lines, including immutable shared JSON fixtures and all moved native tests. Every Rust implementation Source mount stays within the child. The independent graphlib SCC oracle checked 125 real firstparty dependency owners: no cycle and no aggregate UI dependency from this child. Source target selection is not a target compilation result.

## Pending Required Validation

After Root authorizes the compiler window: qualify the real new child native library/tests with canvas-raster and testkit, aggregate UI engine compilation/tests, product renderer and Infinite direct consumer closure, and supported browser compilation. GPU tests that report unavailable adapters must be distinguished from executed GPU proofs. Run prototype and release where the managed runtime policy permits. Compiler dep-info must qualify the actual ownership and generated inputs; Source analysis alone does not establish that result. Native/browser presentation and raster output remain unclaimed until their real runtime gates execute. No full SDK or normal Nx publication was attempted.
