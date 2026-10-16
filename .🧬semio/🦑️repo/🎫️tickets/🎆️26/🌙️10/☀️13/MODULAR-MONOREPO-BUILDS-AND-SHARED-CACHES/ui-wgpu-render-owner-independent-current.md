# Independent WGPU Render Owner Source Review

Read-only review on 2026-10-15 of the actual extraction, ownership handback, new tests and Source closure harness. No tests, Rust/GPU operations, Original, store/image access or Source edits ran. Parent owns the ticket and authorized physical gate. Reported BOTH 8/898 and strict passes are retained handoff evidence, not new execution by this review.

## Material Finding: Three Moved Native Laws Read The Wrong Implementation

`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🧪️tests/🔬️targets-wgpu-gpu-prepared-present/🦀️.rs:351`, `:408`, `:433` still use `include_str!("../../🦀️.rs")`. After extraction this resolves to the child crate root, which contains module/reexport declarations, not the draw implementation. The assertions seek `let world_encoded_format = format.remove_srgb_suffix();`, `world3d_standard_translucent_pipeline` and `world3d_textured_pipeline`, all owned by `../../🔧️pipeline/🦀️.rs`. These deterministic Source laws will fail when executed, independently of GPU availability. The first same-function GPU include was correctly changed to `../../🎮️execution/🦀️.rs`; the draw includes were missed.

Action: handcraft these three include targets to the actual pipeline owner and add a bounded Source law that checks moved inspection assets refer to the intended implementation, rather than only checking that paths exist. No Source was changed here. Root was notified before report completion.

## Actual Cohesive Cut

Let `D` denote `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw`.

`D/🦀️.rs` mounts real private pipeline, execution and shader modules. Those own substantial actual device/pipeline/render work; the new crate is not a forwarding facade over aggregate UI. The child manifest depends on draw-contract/prepared/raster-owner/scene/theme/text and neutral framework owners, with optional canvas scene; no direct aggregate UI dependency is authored. Pure Source graph harness follows authored firstparty normal/target dependency edges and checks independent graphlib SCC/no aggregate owner. That establishes the scoped authored graph, not Cargo feature-resolution or compilation. Dev-dependencies are not traversed by that SCC helper; current child dev dependencies are explicit foundational/test owners, not aggregate UI.

The public seam reexports opaque GpuContext/cursors, bounded table/witness values and owned EngineRasterBuild. Device, Queue, Texture, View, Surface, command buffer and renderer fields remain private in reachable types inspected. Pipeline helper methods carrying raw WGPU types live in private non-reexported types, or are crate-visible on the reexported mesh table. Platform ingress explicitly reexports native Window and transferred browser OffscreenCanvas, with private capture carriers consuming actual surface/canvas before async preparation. This matches the approved ingress exception.

## Session, Failure And Drain Custody

`D/🎮️execution/🦀️.rs:287` owns actual Device/Queue/command Buffer in private GpuSessionRoots. Offscreen shares actual device/queue but calls from_device for its own session roots/commands/tables. `D/🎮️execution/🎬️canvas/🦀️.rs:23,33` compares Arc identity before stepping or closing a build. EngineRasterBuild captures that same Arc and actual raster admission; no numeric session field grants adoption.

Allocation stages validate exact raster admission before texture/view/renderer birth; Render revalidates before execution. At Stage, Returned reinstalls the exact admission/texture/view. Retained leaves these actual resources in pipeline upload_close and returns renderer/session residue in the build. Product EngineCanvas reinstalls the returned opaque build before beginning close in both refusal arms (`prepared-engine-canvas/🦀️.rs:555`). The build’s bounded close retires admission, renderer, view, texture and session; terminal requires actual Option fields empty. Physical table retirement remains independently required after retained staging.

One bounded close risk needs explicit qualification: canvas close line 34 takes admission before invoking fallible `cancel_engine_texture_admission`. Pipeline lines 2185–2198 retain rejected admissions when a close slot exists, but capacity-exhausted branches return Err before retention. Therefore do not claim every cancellation refusal preserves the exact admission root until the invariant excluding simultaneous occupied retirement is established or tested. This is an existing lower API shape carried into a new wrapper, not proof of an exercised loss. The new tests do not cover this slot-exhaustion path. Keep any repair limited to this actual custody handoff.

Infinite world bridge now receives MeshGpuSourceReturn rather than a caller boolean. The child checks exact key/cache epoch plus actual Mesh3dLease equality against in-flight/cache source before retirement; its private witness carries key/resource identity/lease. World bridge calls witness.validates against its retained actual loan before setting gpu_returned and only then returns the loan in bounded steps. The wrong-owner native fixture explicitly uses a same-identity foreign lease and expects rejection. This is materially stronger provenance than equal serialized identity alone. Runtime execution remains unqualified here.

## Tests And Budgets: Scope Of The Evidence

The two new actual engine-raster Rust tests create a real headless context and exercise foreign sibling-session rejection/stale reservation before first texture allocation, verify original roots/admission retained, and drain issuer/table. They return early with a DEBUG unavailable message if headless initialization fails. A passing test count can therefore include no GPU execution; qualification must retain whether those debug-success lines occurred.

Returned/Retained four-vector SQLite checks validate neutral ownership accounting, not actual injected GPU stage faults. Current new native tests do not force Stage Returned after resource creation, Stage Retained table ownership, renderer failure, cancellation at each phase, or retirement-slot exhaustion. Source marker checks likewise establish code presence, not those runtime behaviors. The handback correctly disclaims Rust/GPU/output/throughput until actual gates.

`D/🧪️tests/🟦️.ts` uses real readRustSourceClosureV1 over child roots for five production cfgs and macOS canvas+testkit+test. That reader traverses include_str/include_bytes as embedded inputs, counting full bytes and zero active Rust lines for assets; rust modules stay within child while shared immutable fixture assets may remain outside. Manifest caps are 786432 bytes/64 files/12000 active lines. This is a coherent authored budget approach. The wrong includes above also demonstrate its limitation: a complete existing-path closure can include semantically wrong assets and still pass. After correcting them, rerun the same closure counts; do not preserve old numeric totals by assumption. Linux/Windows/browser test+canvas closures and actual compiler dep-info remain later qualification, not proved by macOS test cfg alone.

The scoped Source API check currently rejects obvious device/queue getters and `pub use wgpu::` in selected files; it is not a complete Rust public API reachability/type-leak checker. The actual inspected reexports appear bounded, but future raw-handle return methods with other names would evade this static regex. No unrelated broad refactor is requested.

## Handback

The extraction is a real cohesive owner cut, with actual session and original mesh provenance retained on Source inspection. Fix the three wrong native Source-test includes before calling moved test qualification complete. Qualify the bounded admission-cancel capacity invariant and actual Stage Returned/Retained handback when executing the existing owner tests. Preserve the distinction between BOTH pure Source 8/898, actual Rust compilation, adapter availability and real GPU execution. Only this Markdown report was authored.
