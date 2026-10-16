# Independent Frozen WGPU Subdivision Audit

Read-only Source review on 2026-10-15. No Rust/TS tests, builds, compiler/store/image/Original operations ran. Parent reports BOTH 16/1375 and strict; those are retained pure Source evidence, not Rust compilation. Reviewed current nine-owner cut, design, device/resource/command descriptors, mesh/raster/draw/upper code, native tests and feature manifests. The earlier single-owner implementation report describes the prior epoch; its old file paths, 768 KiB cap and 8/898 totals must not describe this frozen subdivision.

Let `D` mean `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw`.

## Actionable: Submitted Work Does Not Retain Session Through Callback When Witness Is Dropped

`D/🎮️execution/🔌️device/🖌️commands/🦀️.rs:13–18`: submit consumes actual CommandBuffer, checks exact session and registers real queue.on_submitted_work_done. Its callback captures only the AtomicBool witness. Session custody is solely in returned Submission; that type has no required settlement/drop discipline. Upper execution calls `queue.submit(...).expect(...)` as a statement throughout `D/🎮️execution/🦀️.rs` (for example lines643,651,700 and later), immediately dropping the witness. Readback does the same at `📤️readback/🦀️.rs:64`.

Consequently the new firstparty completion owner does not retain its actual session through accepted completion if the caller drops the witness, and the upper path does not drain that witness. Backend internal object retention may keep driver resources safe, but it is not the authored explicit accepted-owner/drain contract. The design specifically requires actual retained roots through settlement. The native completion law keeps Submission alive and polls it, so it does not exercise the production discard path or prove original session release ordering after early witness drop.

Minimal repair/review should bind real retained session/resource custody to completion and explicitly account for upper accepted submissions until completion/close. Simply requiring callers to inspect a boolean is insufficient. Same audit applies to device `BufferSlice::map_async`: callback captures only completion, not session/buffer owner; current readback keeps its buffer, but the public backend seam itself permits dropping the owner before completion. Keep repair within the concrete device/upper accepted work boundary. Root was notified promptly; no Source changed here.

## Actionable: Public External Trait Bound Survives The New Crate Seam

`D/🔧️pipeline/🎬️execution/🦀️.rs:31` exposes public GrowBuffer::upload<T: Pod> after privately importing bytemuck::Pod. This directly requires an external trait at a new public owner seam without an explicit reexport/interface. No raw WGPU type was found escaping the reviewed public surface; this is a separate external-type contract issue under repository rules. Prefer a firstparty owned upload contract or restrict the generic helper to its actual crate consumers; if the external bound is intentionally public, explicit approved reexport is required. Do not claim the public API is entirely repository-owned on the strength of a raw-wgpu regex alone.

## Real Cut And API Seams

The new neutral `🖌️render/🔌️device` physically owns firstparty resource/descriptor/closed fault vocabulary. Concrete device owns actual creation, binding, copy, encode, queue submission, map and platform preparation, not a facade into upper render. Shader package owns actual programs; pipeline factory owns resources/construction; mesh owns source leases/cache/retirement; raster owns exact admission/table/retirement; draw execution owns real encoding/upload; upper render owns assembly/presentation/readback/canvas state. Their normal metadata is 262144 bytes/32files/4000active lines rather than a special giant render exemption.

The actual manifests under D include device, shaders, draw-contract, render, draw execution, pipeline factory, mesh and raster, with the ninth neutral device owner outside D. This review found no direct aggregate UI edge in the inspected new owner manifests. The actual Source graph/census is distinct from Cargo’s resolved feature/target graph and compiler dep-info. CPU’s ongoing 97-owner census is not independently rerun or expanded here.

Raw WGPU resources remain private inside device wrapper fields; descriptors specialize only firstparty opaque types. Device-issued SessionRoots contains real Device/Queue, and resources privately capture the same Arc. Foreign binding, copy, pass attachment and submission checks compare Arc identity before backend work. Queue::submit returns the exact consumed foreign command in OwnedFailure::Returned. Constructors for Session/Device-issued resources are private; no numeric identity or serialized record creates them. Approved native Window/browser OffscreenCanvas carriers are explicit platform ingress reexports.

RenderPass<'a> borrows its mutable actual encoder and holds session; public descriptors borrow firstparty attachments, and commands validate foreign resources before accessing raw handles. Public pass setter lifetimes are deliberately broad wrappers over the pinned backend API; only actual Rust compilation can establish their coherence. No source-only test is a substitute for checking these borrowed signatures against pinned WGPU29. No definite borrowed-lifetime compile defect is asserted from inspection.

## Raster And Mesh Custody Preserved

Upper EngineRasterBuild continues to retain actual GpuSessionRoots Arc and admission/renderer/texture/view residue, compare exact upper session before step/close, and restore original build before product close. Its cancellation now explicitly handles RasterTextureCancelFault::Returned by reinstalling admission and Retained only when table retirement owns it. This resolves the earlier audit’s concrete consume-before-fallible-cancel risk at the Source seam. Stage Returned similarly restores admission/texture/view; Stage Retained remains bound to table close ownership. Runtime forced faults remain separate proof.

MeshGpuSourceReturn remains private-constructed and validates actual lease plus key/resource identity. Split mesh cache/upload stores original source lease and checks same-identity foreign lease before retirement. Infinite consumes the witness before returning its retained loan. No serialized same-identity boolean replaced that proof.

Canvas native interop moved inside concrete device platform CanvasRasterizer. Its actual raw session/render calls check device, queue and view against the same private session before passing private WGPU objects to CanvasScene. Upper only sees firstparty object. Feature canvas-raster forwards from upper to device, and both native canvas API mounts retain the WASI-p2 exclusion. Device manifest retains explicit native/backend/browser target dependencies and optional CanvasScene. No specific missing dependency/feature was found in inspected paths, but neither feature resolution nor browser compilation was run.

## Moved Tests And Evidence Limits

Earlier three wrong whole-pipeline include_str laws have been replaced/moved away from the upper native test body; native pixel/clip traversal and independent tiny-skia oracle remain. The design’s Rust token and independent tree-sitter Source descriptor checks are appropriate for descriptor text policy, not executed GPU output. New device native tests cover actual foreign binding, foreign command returned to issuer then real completion callback, and foreign render attachment before pass birth. They obtain real headless devices but skip with DEBUG unavailable if acquisition fails. A native pass count must distinguish adapter unavailability from exercised GPU proof.

The completion test holds the returned witness until callback; it does not cover abandoned Submission/map callback custody, upper cancellation/close while commands are pending, or forced Returned/Retained raster staging with live GPU resources. The new opaque descriptor conversions are substantial actual Rust code: TS 16/1375 establishes authored bounds/Source/AST laws, not their compile correctness or exact driver semantics. Native tests, supported browser check, compiler complete closure/dep-info and genuine pixel/output laws remain required after a separately authorized gate.

Complete owner-cut tests must count all included fixtures and feature/test source. Moving whole-file inspection out of Rust reduces duplicated embedded production bytes legitimately; it must not delete native behavioral assertions. The bounded files inspected reflect that distinction. CPU’s full census limitation at ui-contract/value-derive remains its reported separate scope, not a reason to reopen this bounded cut or silently inflate caps.

## Frozen Bytes Observed

- Device root: c60e6a0b1f7ad9342933f0b2fb8f00bf592c4187d42ff1d3e00a23770341f6a8
- Device commands: 0e469a89cf2acf388c06463d473bd37f77f7329d452ef2c06c8037f2ab1cd729
- Draw execution: 5fca67a8f1ae1415f82a85cfa455cdbe3004a9039aa4bb9443b666ed0ece2b4a
- Canvas upper: f11a5a55c6d2d7904237c941c06c6a5a5f7cc1a01f3165f5408b0616fac351d8

These identify read Source, not compiled artifacts. Only this Markdown audit was authored.
