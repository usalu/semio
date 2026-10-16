# UI WGPU Render Owner Design

The next coherent cut is an actual target execution and resource owner. Moving `draw/🦀️.rs` alone would export backend objects and leave device, queue, surface and presentation custody in aggregate UI. The first implementation must therefore move the draw implementation together with its private GPU execution/session, prepared presentation cursor, readback, shaders and EngineCanvas GPU candidate boundary. This document is a reviewed-scope proposal and an independently executed neutral baseline. No Rust crate, production draw implementation, Cargo manifest, Rust image, or GPU runtime was changed or exercised by this lane.

## Inspected Source

All paths in the following table are relative to `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu`.

| Current owner | Measured bytes / lines | Relevant implementation |
| --- | --- | --- |
| `🖍️draw/🦀️.rs` | 303828 / 5645 | Scene/composite targets 45–282; triangulation 291–357; mesh buffer/table/upload/return 544–857; frame buffers 859–1009; CPU IconAtlas 1014–1042; raster storage/reservation/witness/retirement 1044–2814; pipeline construction/encoders/draw execution 2816–5632 |
| `🎨️shaders/🦀️.rs` | 55910 / 1138 | UI/vector/world/material/celebration/grid/line/texture, blur, blit, postprocess, glass and raster mip shader programs; authored/painted shader constructors |
| `🧊️gpu/🦀️.rs` | 71441 / 1201 | Device/queue/surface 288–319; creation 322–470; actual mesh and raster dispatch 537–708; prepared command/clip/glass/present ladder 708–1021; exposed device/queue and engine texture transfer 1072–1152 |
| `🧊️gpu/📤️readback/🦀️.rs` | 5616 / 136 | Owned async map and padded RGBA/BGRA normalization; bounded allocation, progress, cancellation and buffer destruction |

These four existing files total 436795 bytes before the CPU-only records are reassigned and product EngineCanvas GPU candidate ownership is added. That is a concrete Source scope, not a measured compiler closure or throughput improvement. A 262144-byte owner budget would already be false for the proposed scope; choose an explicit reviewed semantic budget after the real file move, rather than declaring a facade-sized threshold over hidden included Source.

Existing direct neutral owners are already available: `semio-framework-ui-draw-contract`, `semio-framework-ui-prepared` (and its staging child), `semio-framework-ui-raster-owner`, `semio-framework-ui-scene`, and `semio-framework-ui-theme`. Scene math is already `ui_scene::math`, including the genuine native mesh resource owner/loan and lease arena. It must be imported directly, with no second scene-type family and no dependency on `semio-framework-ui`.

## Required Ownership And Dependency Direction

Create the actual crate at `🖍️draw/📦️packages/🦀️rust/Cargo.toml`, proposed package `semio-framework-ui-wgpu-render`. Its private implementation owns device/queue/surface objects, all pipelines, fixed mesh/raster tables, target textures/views/samplers, frame buffers, accepted presentation/upload/readback cursors, and shader programs.

The dependency direction is:

```text
product renderer / aggregate UI orchestration
  -> ui-wgpu-render
       -> ui-draw-contract
       -> ui-prepared -> prepared-staging
       -> ui-scene -> genuine native mesh arena/resource owner
       -> ui-raster-owner
       -> ui-theme
       -> job + value + pixels
       -> optional canvas-scene (canvas-raster feature)
       -> private wgpu + bytemuck backend
```

`canvas-scene` already owns the first-party `Scene` and `CanvasRasterSession`; its inspected manifest depends on canvas assets, value, async, geometry and UI styling, without aggregate UI. The child can privately call its existing backend session while exporting only first-party Canvas scene and owned candidate contracts. Do not add the Canvas facade or whole product renderer as a child dependency. Keep native/browser/wasm32-wasip2 cfg separation explicit: browser DOM constructors do not enter WASI, and neutral declaration components do not acquire WGPU execution by importing a record. Do not weaken the current feature-ownership law merely to make the new manifest compile. The optional Canvas raster feature is enabled by the real EngineCanvas consumer; a neutral `wgpu` declaration feature continues to avoid the execution owner entirely.

Move CPU `IconAtlas` records and triangulation into the existing draw-contract owner (or an already proven geometry owner if the implementation is genuinely the same). They contain no WGPU objects and are consumed by aggregate UI painting and product chrome. Keep one implementation and update all callers at once. Do not retain an aggregate implementation plus child copy.

Aggregate UI's `wgpu-engine` feature consumes the actual child; its `wgpu` declaration feature still consumes only neutral contracts. Root workspace member/profile declarations, lock membership and source-boundary accounting must follow the actual new owner. Remove aggregate dependencies only when its remaining engine/input/host code no longer uses them. Product renderer and Infinite need direct child dependencies when they name the genuine return/operation types; this is not evidence that their aggregate UI edges can all disappear.

## Public API And Backend Privacy

The execution API exposes repository-owned concrete structs with private backend fields, not raw device getters, callback parameters containing WGPU objects, `Any`, downcasts, external error variants, a generic backend type bound requiring WGPU, or integer handles into another registry. Proposed names below are design names, not implemented APIs.

| Repository API | Concrete ownership and input |
| --- | --- |
| `RenderSessionV1` | Actual private device/queue, surface option, tables, targets and cursor admission; no `Clone` of whole session and no public `device()`/`queue()` |
| `NativeSurfaceCaptureV1`, `BrowserSurfaceCaptureV1` | Actual owned native window or transferred OffscreenCanvas retained through surface lifetime; cannot be created from a surface id, token, JSON or numeric handle |
| `PreparedGpuPresentCursor` | Existing command, clip-piece, blur-mip, phase and abandonment owner moved intact; no serialized construction or arbitrary registry enrollment |
| `PreparedGpuReadback` | Existing owned buffer/map/progress/cancel implementation, with only complete first-party RGBA output published |
| `EngineRasterBuildV1` | One private exact admission + actual texture/view + Canvas renderer + actual backend context retention; no public external-type fields |
| `EngineRasterStageFaultV1` | Refusal returns the same owned build, or a retained cleanup loan for the exact resources the table retained; never returns raw texture/view objects or replaces ownership with an error string alone |
| `MeshGpuSourceReturn` | Existing private key + exact resource identity + actual `Mesh3dLease` return evidence; only the real upload/cache owner can construct it |

The schema-first split is explicit: existing scene/draw/prepared/raster schemas remain the input owners; the newly authored JSON schema specifies finite observations only. Actual resource owners are non-serializable Rust types. Before implementation, extend the owned observation contract with the reviewed progress/fault discriminants and keep them separate from constructor admission. The public Rust sketch is:

```rust
impl RenderSessionV1 {
    pub fn begin_prepared(
        &mut self, token: &UiPresentToken, gate: &PreparedRenderGate,
        packet: &PreparedRenderPacket, live_revision: u64, live_generation: u64,
    ) -> Result<PreparedGpuPresentCursor, RenderFailureV1>;
    pub fn prepared_step(
        &mut self, packet: &PreparedRenderPacket, cursor: &mut PreparedGpuPresentCursor,
    ) -> Result<RenderStepV1, RenderFailureV1>;
    pub fn begin_engine_raster(
        &mut self, request: EngineRasterRequestV1<'_>,
    ) -> Result<EngineRasterBuildV1, EngineRasterRejectedV1>;
    pub fn engine_raster_step(
        &mut self, build: &mut EngineRasterBuildV1, scene: &CanvasScene,
    ) -> Result<RenderStepV1, EngineRasterStageFaultV1>;
    pub fn retire_original_mesh_source_step(
        &mut self, key: &str, source: &Mesh3dResourceLoan,
    ) -> Result<Option<MeshGpuSourceReturn>, RenderFailureV1>;
    pub fn close_step(
        &mut self, cx: &mut StepContext<'_>,
    ) -> Result<InteractiveJobCloseStep, OriginalTurnFailure>;
}
```

`RenderStepV1` is a bounded progress/pending/completion observation; no result manufactures authority. `EngineRasterRequestV1` borrows an already owned Canvas scene and existing original raster content/candidate values, with checked finite dimensions. Its child-private constructor validates the actual table reservation and retained original context before the first texture allocation. `EngineRasterRejectedV1` preserves any actual accepted input/build loan needed for paid retirement. Native/offscreen token overloads remain explicit. This sketch is intentionally not a second general renderer abstraction or a generic WGPU provider API.

The session entry points borrow existing `PreparedRenderPacket`, `PreparedRenderGate`, `UiPresentToken` or `OffscreenPresentToken`, `SceneRasterLease`, `RasterContentIdentity` and original mesh resource loan/lease. They do not accept a second copied packet family or derive custody from revision fields. Runtime witness values remain correlation checked against the actual private reservation, candidate and exact session; the neutral observation schema never authorizes work.

A concrete method surface should retain `headless`, `offscreen`, resize/metrics, admitted prepared upload/present steps, readback steps, exact mesh return, exact raster retirement and explicit close/terminal observations. Engine raster entry points become `begin_engine_raster` / `engine_raster_step` / `stage_engine_raster` / `close_engine_raster_step`, each operating on the actual private build and existing reservation tuple. The product passes its first-party Canvas scene borrow, clear color and finite extent; texture/view/renderer creation and staging all happen inside this owner after the corresponding reservation recheck.

Native/browser capture needs an explicit platform seam. Keep actual native window and transferred canvas objects, rather than inventing opaque integer authority. The narrow implementable platform entry module can explicitly reexport the required window/canvas carrier at that platform boundary, wrap the actual owned carrier in `NativeSurfaceCaptureV1`/`BrowserSurfaceCaptureV1`, and expose only those wrappers to the main execution API. This follows the repository's explicit-reexport exception for necessary platform interfaces. It must never reexport WGPU Device/Queue/Surface/Texture/View. If Root requires zero external carrier types even at this platform ingress, the existing framework host must own window/canvas creation and issue its genuine capture directly; that expands the first implementation scope and must be settled before the production cut. A fabricated wrapper around a numeric native handle is not an alternative.

The candidate build may retain the same private backend context that actually owns Device/Queue, so its resources survive a caller cancellation until accepted cleanup settles. It must not use a fresh empty token, copied private struct or numeric context id as authority. Existing `StepContext` and retained grant/retirement APIs remain the sole job work and close admission; do not add an independent issuer or budget namespace.

## Exact Product Callers And Necessary Changes

Paths below are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`.

| Actual consumer | Current boundary and required change |
| --- | --- |
| `🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:944` | Captures the real window before async device setup via `GpuContext::from_window`. Route through genuine native capture; preserve event-thread surface capture before the future is driven. |
| `🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:930` | `from_offscreen_canvas` from the actual transferred canvas; preserve dedicated worker token and browser-only cfg. Lines 759/763 also upload real font/icon atlases. |
| `🎯️targets/🧊️wgpu/🧊️renderer/🧠️app-runtime/🦀️.rs:1382` | Initial font/icon uploads. Pass the existing finite prepared atlas page/view contract instead of making the child depend on aggregate UI typography. Direct text-owner dependency is only justified if this remaining API genuinely needs its owned type. |
| `🎯️targets/🧊️wgpu/🧊️renderer/🖥️frame-presentation/🦀️.rs:1051` | Validates the real prepared gate and native/offscreen token, then ownership/engine/uploads/stage/render/close/acknowledgement phases. Lines 1216/1228 begin and advance the actual GPU cursor. Preserve the whole ladder and accepted callback failure/cleanup behavior. |
| `🎯️targets/🧊️wgpu/🧊️renderer/🎨️presentation-raster/🦀️.rs:95` | `RuntimeRasterOperationAuthority` owns the live operation witness. Its private admission/release remains the product owner; do not replace it with the baseline trace fields. |
| `🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/📦️prepared-engine-canvas/🦀️.rs:609` | Real reserve -> texture -> view -> Canvas renderer -> render -> stage -> retire-renderer phases. Lines 618/632/640/647 directly exchange Device/Queue/Texture/View. Move the actual GPU candidate resource work into child `EngineRasterBuildV1`; keep document/surface freshness, CPU scene and product slot ownership here. |
| `🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🗂️engine-surface-registry/🦀️.rs:1125` | Actual `create_target_texture` moves into the child; no product helper should keep creating textures through an exposed device. |
| `🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🦀️.rs:181` | Returns native world mesh GPU custody through `retire_original_mesh_source_step`. Preserve genuine return evidence and caller's exact loan. |
| `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📤️icon-export/🦀️.rs:207` | Starts actual headless initialization and retains its detached initialization result; cancellation must join/drain that accepted future before session retirement. |
| `🧱️elements/🖼️IconRenderHost/🎯️targets/🧊️wgpu/📤️export/🧊️gpu/🦀️.rs:55` | Offscreen session, accepted packet, mesh/raster upload, prepared cursor, readback and PNG/packet retirement ladder. Keep exact packet return on admission failure and every close phase. |

The remaining production matches were explicitly checked: renderer `🧊️renderer/🦀️.rs:329` imports the context type; app-runtime `:1345` accepts its actual initialization future; native-socket-probe `:332` is a comment identifying GPU-coupled pieces, not an extra device entry; EngineCanvas engine-surface-attach `:1143` is a reservation comment. Product test matches are the renderer async-boundary/retained-IO tests, EngineCanvas standalone and metrics-invalidation-drain tests, and IconRenderHost GPU-export tests; rewrite their ownership assertions around the moved actual implementation rather than preserving obsolete Source-text locations.

Other product occurrences found in scene icon painting, interpreter retained UI comments and chrome are mostly CPU `IconAtlas`/type references, not separate raw WGPU execution authority. Update those references to the neutral contract owner rather than widening the new execution API.

The mesh return has a further actual consumer outside renderer: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🌉️bridge/🦀️.rs:1942` and `:1961`. Both require `MeshGpuSourceReturn`, validate it against the exact original loan, then mark GPU return before the native loan can return. These call sites and Infinite's corresponding resource tests must move to the genuine child type at the same time. A nominal lookalike return type would break custody even if every integer field matched.

## Lifetime Laws To Preserve And Qualify

**Mesh:** the existing upload cursor retains the actual lease, creates bounded vertex/index buffers, advances one scalar at a time and rejects a different lease even with matching cache version. Exact source retirement first closes any partial upload, retires exact cache buffers, and only then issues `MeshGpuSourceReturn`. `Mesh3dResourceLoan::return_step` decrements the real arena loan; the resource owner's close waits for all loans. Preserve private constructors and same-object handback. Existing foreign-same-identity loan tests are required gates.

**Raster:** preserve fixed 256-item and 256 MiB table limits, 64 MiB per item, 256-byte keys, bounded probe capacity, exact content identity and operation tuple, private reservation nonce, original CPU release/GPU resident loans and separately stepped close cursors. Stage failure either returns the same admitted resource bundle or retains it for exact later cleanup. No failure branch drops a texture/view/CPU lease and reports success. Existing candidate/presenting witness retirement must remain admissible after its own first field is consumed.

**Prepared and surface:** retain the exact packet while accepted uploads, presentation and readback are pending. Keep command-page, clip-piece and blur-mip progression and sustained-overrun measurement. Multi-task browser work renders into the prepared composite target; actual surface acquire, final blit/submit and present remain one uninterrupted terminal host opportunity. A copied revision, scene id, witness or surface token does not recreate the required actual source/token/packet owner.

**Cancellation and failure:** stop accepting new operations, drain accepted device initialization/maps/render opportunities, then close cursor view/frame, partial uploads/builds, staged/candidate/live raster ownership, mesh tables and packet/scene loans through paid close demands. Preserve a direct primary fault while retaining secondary cleanup work and its exact owned failure payload. Abandoned cursors continue through the existing real abandonment-owner mechanism; moving the code must not mint a second global registry or convert real retained owners into flags. A cancelled map must close its actual buffer and settle its retained callback ownership.

**Completion scope:** existing `terminal_is_empty` checks prove owned Rust roots/cursors are empty. Do not silently promote that observation to physical GPU/driver quiescence or VRAM accounting. If the new public completion contract promises actual queue completion, add and qualify the real backend completion/fence observation first. The new baseline `step-settle` event describes accepted host-step settlement, not a GPU fence.

## Proposed Source Move Tree

Keep all actual target execution under the new manifest's own closed Source tree:

- `🖍️draw/🦀️.rs`: repository-owned public session, operation and fault contracts; explicit imports of existing neutral owners.
- `🖍️draw/🎮️execution/🦀️.rs`: move real current GPU session and prepared opportunity ladder.
- `🖍️draw/🔧️pipeline/🦀️.rs`: actual pipelines, ABI packing structs, encoders and shader binding; backend types private.
- `🖍️draw/🎯️targets/🦀️.rs`: actual scene/composite/depth/blur targets and framebuffer roots.
- `🖍️draw/🕸️mesh/🦀️.rs`: current mesh table, exact upload/close/return implementation.
- `🖍️draw/🖼️raster/🦀️.rs`: current actual raster registry, admission, upload, staging, witness ledger and retirement.
- `🖍️draw/🎨️shaders/🦀️.rs`: current shader ownership moved once with its matching GPU ABI.
- `🖍️draw/📤️readback/🦀️.rs`: current bounded mapping/readback implementation.
- `🖍️draw/📥️platform/.../🦀️.rs`: actual native/browser surface capture/creation behind the reviewed platform ingress.
- `🖍️draw/🖼️raster/🎬️canvas/🦀️.rs`: real EngineCanvas GPU build/stage/retire operations, optional Canvas scene integration.

Move implementation and tests; remove former aggregate `mod draw`, `mod gpu`, `mod shaders` ownership instead of leaving path-includes pointing outside the child. Aggregate UI may import the child where it is genuinely the orchestrator; the child must not import `crate::wgpu`, `super::*` from aggregate UI, product renderer, Infinite, or the old UI manifest. Preserve external peer renderer Bazel/tooling ownership and coordinate any target registrations with that owner.

## Baseline Added And Actually Executed

Added a closed observation-only schema at `🖍️draw/🧬️schema/🔣️.json`, six finite valid/hostile closure-order traces at `🖍️draw/🧪️tests/🔣️.json`, and a Bun test module at `🖍️draw/🧪️tests/🟦️.ts`. The schema and fixture explicitly say `observation-only-no-resource-authority`; no GPU device, buffer, texture, session or fake admission is created.

The tests independently validate:

1. Seven existing prepared GPU clip cases: logical intersections, fractional DPR physical scissor values, overlapping-piece refusal, full 696-pixel opaque mask output against actual dev-only `@napi-rs/canvas` rasterization. This does not claim shader alpha, material or glass parity.
2. Existing prepared readback RGBA/BGRA row-padding and alpha bytes against independent dev-only Sharp PNG encode/decode. This does not execute a GPU map or prove transfer-function output.
3. Six schema-validated observed closure traces against independent SQLite running retained-root/pending-step sums: early source return, pending accepted work, false terminal and duplicate return are refused. This is a normative observation baseline, not actual GPU resource-lifetime proof.

The existing UI foundation test script imports the new module. Its normal registered Nx headroom routes refused **before tests** with `Original funded graph publication namespace required`; this is an infrastructure refusal, not baseline RED. Root approved a separate ticket `ui-wgpu-render/📜️script.ts` top-level launcher using a preauthored finite 120 s Original and existing owned capture. It refuses an inherited Script invocation before authoring a new root, preserves the exact carried child envelope, and does not enroll a namespace or invoke Nx/Cargo. `test|types prototype|release` commands are registered in both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` beside the existing UI gate family.

| Actual final gate | Result |
| --- | --- |
| Prototype runtime, handle 46127 | Exit 0; 5 tests / 76 expectations; 1.99 s (three new baseline laws plus two existing UI foundation laws) |
| Release runtime, handle 4334 | Exit 0; 5 tests / 76 expectations; 2.01 s |
| Prototype strict, handle 76144 | Exit 0 |
| Release strict, handle 43396 | Exit 0 |

All handles were collected. Generated logs are in ticket `🗑️generated` for Root's inspection and eventual ticket cleanup. No Rust compilation, Rust cache/store opening, WGPU device, GPU render, native/backend qualification, full production extraction or throughput measurement was performed.

## Concrete First Implementation For Root Review

Approve one cohesive execution-owner slice, not a draw-file facade: real draw + shaders + private GPU session/present/readback, exact EngineCanvas build transfer, and the reviewed genuine platform capture. First add neutral API/schema laws for opaque resource return and exact failure retention, then make the actual Source moves and update all listed consumers in one coordinated epoch. Preserve declaration-only feature separation and all existing prepared/mesh/raster/theme contracts.

After Rust storage headroom is restored, the required first native qualification includes direct child-crate test compilation, both actual target configurations available on the host, feature ownership/source-boundary gates, existing draw-unit/raster-residency/witness-lifecycle/prepared-present/readback/foreign-mesh-return tests, actual device startup and cancellation at each resource phase, stale/foreign packet/admission rejection, detached accepted initialization/readback drain, EngineCanvas Returned/Retained stage-failure identity, and real native/headless/browser image checks with the existing independent raster oracle. Platform portability claims require the actual platform gates; this design and CPU baseline establish none of them.
