# Coordinator P6i Pre-acceptance Counterexamples

Date: 2026-08-24  
Verdict: **RED — P6i is not source-acceptable.**

## 1. Mounted Render Still Clones Complete Visual Buffers

Both FEM3D `render_with_progress` functions obtain `&str` from `Fem3dMountedVisualLease` and call
`to_string()` for meshes and instances before constructing the World3d node. This clones both
complete JSON buffers on the render/UI call path. The P6i contract explicitly forbids whole visual
clone/decode/sort/scene encoding on that thread and requires immutable page/packet lease consumption.

## 2. The FEM3D Viewer Is Not Mounted

The editor body dispatch reaches `with_live_visual`, but the FEM3D viewer `render` function still
calls its local whole `fem3d_scene_parts`, which builds dynamic mesh/instance vectors and serializes
both complete JSON values. The contract names the exact FEM editor/viewer surface, so mounting only
the editor leaves a live bypass.

## 3. Output And Order Storage Is Monolithic, Not Fixed-page

`BoundedText::reserve` calls `try_reserve_exact(OUTPUT_BYTES)` in one grant, and the 3D job similarly
calls `try_reserve_exact(doc.solids.len())` / `try_reserve_exact(doc.elements.len())` for the complete
order arrays. Credit is recorded only after the complete allocation. This violates pre-allocation
page/item/byte admission and can itself exceed the 8 ms ceiling.

The lease close cursor repeatedly truncates `instances_json`, but truncation does not return String
capacity. It ultimately replaces the entire String/Vec with an empty owner in one grant, releasing
the complete backing rather than one actual page. The 2D job/lease must be checked and repaired for
the same representation, admission, and retirement defect.

## 4. FEM3D Numerical Fields Are Placeholder Zeros

The displacement, residual, reaction, contour, and mode stages construct a fresh field per node with
all vectors/scalars set to zero. No immutable solver-result view feeds those values. Consequently the
required numerical-to-visual correspondence, convergence tiers, and result/mode presentation cannot
pass, even if stage tokens and schemas exist.

## 5. Verifier False-green

The 21 mutations establish stage/token and selected call-site shapes, but do not reject complete
`to_string()` cloning on mounted render, a viewer whole-scene bypass, monolithic `try_reserve_exact`,
capacity-preserving truncate close, or zero solver fields. Baseline success is therefore not evidence
for the live packet.

## Required Closure

- Carry actual immutable fixed page leases through the World3d prepared-frame boundary without
  complete render-thread cloning.
- Mount the FEM3D viewer and every editor results mode on the same lease authority; remove live
  whole-solve/scene bypasses.
- Replace monolithic String/Vec output and order storage with real pre-admitted fixed pages/index
  slots and release one actual page/backing per close grant.
- Feed generation-tagged solver displacement, residual, reaction, contour, and mode/eigen views into
  3D visual construction, with correspondence laws using non-zero fixtures.
- Add faithful call-graph, backing-allocation/retirement, viewer, and numerical mutations.

The isolated P6i and P6h structural gates passed before this read-through. No Cargo, Nx, Wasm,
browser, or timing gate was run while overlapping Rust source packets were active.
