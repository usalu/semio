# Fragment Kernel Design And Integration Audit

Read-only audit while execution authors the shared standard-library BSP kernel and GH integrates it only for genuine remaining source SCCs. No physical model edit, role mask, duplicate ray process or production change by this audit. Saved implementation review is pending its ready signal.

## Actual Input Boundary

The current renderer Item.verts is a concatenation of one or more coplanar Item.polygons. Those polygons can be disconnected or surround an actual void. The kernel must receive one Face per original polygon with stable(sourceItemID,polygonID), never triangulate the concatenated Item.verts as one boundary. Doing the latter would recreate filled annular holes/disconnected gaps. This concrete current API hazard was sent directly to execution/GH.

Face normals and vertices must retain their source winding/plane. Coplanar equal/opposite normals do not authorize dropping physical faces. Unit-normal plane distances, signed classification and numerical tolerance must remain in consistent physical units; a pixel tolerance cannot classify the current millimetre seam/panel crossing as coplanar.

## Material And Provenance Proof Required

Splits must preserve each original polygon's union/area, source identity and actual holes, while generating only convex pieces or valid ear-triangulated simple regions. Concave input, disconnected polygons, collinear/repeated boundary vertices, tiny finite thickness, coplanar/reversed faces and planes through original vertices/edges need portable cases and independent NumPy/Pillow oracles. Invalid/nonplanar/zero-normal inputs should have an explicit contract rather than silent face deletion.

The real capital seam/panel crossing has no single whole-face order. Geometry fragmentation can repair that painter limitation without modifying physical M. Genuine SCC scope is important to avoid splitting unrelated whole-building geometry by every infinite source plane. Finite input/source counters and area checks should remain distinguishable from sampled point-nearest raster evidence.

## Renderer Integration Hazards

A fragment draw must use its actual new vertices with current source style/semantic objects; reusing the original draw closure unchanged would paint the whole original face again. world_vertices/depth must derive the actual fragment, with the original plane normal retained. Cache only pure geometry recipes keyed by actual source polygons/planes, then bind current palette/semantic objects.

New interior split boundaries must not gain artificial outline strokes. Original physical boundary segments and real source seams remain visible according to actual occlusion. Underpaint must use fragment unions/source holes, not a hull or invented closure cap. Point-nearest/source-ID tests must identify the original source and the actual fragment covering that point; a style change cannot substitute for geometric depth.

## Expensive Operation Contract

The iterative kernel must report progress and check cancellation during growth/clipping, not only after all fragmentation. Cancellation must not publish a partial successful recipe/cache. Worst-case BSP fragment growth remains potentially large even with an iterative stack; no unmeasured linear-time or universal fragment-count guarantee is assumed. Current source/permanent tests and fresh capital rays remain required before acceptance.

## Initial Saved Kernel Review And Actual Input Boundary

The saved kernel owns immutable Face/source/polygon tuples and uses only standard-library code. It validates finite simple planar inputs, prepares concave boundaries by ears, splits convex material against real source planes, retains original normal/IDs, deterministically sorts coplanar ties and traverses iteratively far-to-near. Portable cases cover crossing/cyclic faces, concave U, disconnected same-source polygon IDs, opposed coplanar winding, rotated views, independent area/nearest-ray/Pillow union, progress/cancellation and1050iterative planes. This is substantive kernel evidence, not yet proof of actual capital integration.

A cheap preparation-only audit of the immutable2602capital export found389 rejected polygons when fed the renderer's approximate Item bucket/global-Newell normals. Example index46 capital-panel-east-2-1-0-0 has supplied-plane distance residual1.362e−11m versus64-ULP4.547e−13m, despite actual planar thin-quad vertices. This is a concrete caller/API conditioning boundary; it must not be hidden with a pixel/physical tolerance waiver. Stable local maximum-area anchor-triangle normals, normalized and aligned to original lighting orientation, pass2602/2602 under the unchanged strict policy. Original Item lighting metadata can remain separate from the actual per-original-polygon Face plane.

Execution briefly considered coordinate-conditioned angular acceptance, then root selected the strict Face.normal-as-source-plane contract. That intermediate policy/proof is superseded and must not be cited as final. Caller-normal preparation and final strict implementation evidence are pending their save; no final integration acceptance is claimed here.

## Input And Cancellation Checks

Actual collapsed-boundary probes with three identical input vertices or three collapsing to two both raise the declared ValueError for zero material area; root's suspected IndexError was not reproduced and no unnecessary fix is requested. A genuine tiny-scale positive triangle is not the same as a collapsed boundary under scale-aware ulps.

The first source version lacked cancellation checks inside quadratic simple-boundary pair validation. The latest reread now passes check into _simple and checks both pair loops, local plane search and ear-selection loops. This resolves the observed cancellation gap in saved source; final cancellation test evidence remains execution-owned. Worst-case BSP growth and physical/raster correctness remain separate from iteration/no-recursion guarantees.

## Final Strict Kernel Reread

Independently reread the final saved implementation after execution's7-test/actual-input proof. _plane recovers a maximum-area local geometric plane with cancellation checks and aligns its sign to the supplied source normal. _prepare separately requires both supplied-normal point distances and geometric planarity to satisfy the original64-ULP policy. No coordinate-conditioned angular allowance or geometric/pixel epsilon increase remains. Face.normal stays verbatim for source provenance; geometric splitting/projection uses the stable actual plane.

The actual raw bucket-normal389 rejects versus caller-local2602/2602 acceptance is now an explicit portable/API boundary, including the real index46 witness. Inner pair and local-plane searches are cancellable; invalid/nonplanar/collapsed inputs have declared rejection. Source/polygon identities, winding, deterministic coplanar handling, concave/disconnected coverage and standard-library-only ownership are preserved in the reviewed code. No additional concrete kernel source blocker was found in this final reread.

This accepts the kernel contract/source review with execution's recorded final7testsPASS7.773s and exact preparation inputs, not final renderer integration. GH still must bind every original Item.polygons boundary to correct Face plane geometry, rebind current style/semantic objects, avoid cut strokes and prove current capital point-nearest/material coverage and performance. No native/gallery acceptance follows yet.

## Initial Renderer Integration Source Review

Read the saved renderer integration after GH's two initial portable integration tests. Each original Item.polygons boundary becomes a separate Face with its polygon index and maximum-local-cross geometric normal aligned to the source orientation. Disconnected source polygons are not concatenated into a fictitious boundary. Original lighting_normal remains separately captured by the source draw closure; fragment geometric metadata therefore does not change source lighting.

The bounded order cache stores indices and immutable coordinate/normal recipes only. Restore binds the current input Item, palette, semantic owner and draw callback afresh. origin pointers exist on transient Items only. Returned Mobjects receive copied world geometry and primitive source_polygon metadata; no Item or draw closure is assigned to them. Existing source draw closures register the same shape before fragment geometry replaces its points, preserving current semantic ownership. The reviewed source callbacks contain no nested marks that would be lost by the fragment's submobject reset.

Fragment strokes are clipped from original source boundary edges only; triangulation/partition edges receive no stroke. Consecutive recipes coalesce only for matching source, polygon and actual normal, without crossing intervening paint geometry. This is source-level evidence, not yet acceptance of all transparent runs, current capital ray ordering, raster coverage, performance, native scenes or final galleries. GH still owns those pending integrated controls.

Root reports the two repaired orthographic public-interface tests now pass347.231s; this closes their formerly pending execution result without changing physical source.

### Expanded Integration Test Source

The next saved test revision adds independent Pillow alpha compositing for a real three-surface cyclic transparency case, exact full-image comparison of adjacent transparent triangles coalesced into one compound source polygon, and an intervening-source noncoalescing assertion. A weak-reference lifetime test keeps rendered shapes alive while requiring owner groups captured by transient draw callbacks to be collectible after the input Items leave scope. These address the requested transparency and rich-cache retention boundaries substantively. This paragraph records inspected test intent/source only; execution results and actual capital export/nearest-surface proof remain pending GH's durable result.

### Observed Integration Result And Manifest Follow-Up

GH records five renderer-fragment tests PASS0.173s, including source area, cyclic Pillow alpha, exact compound transparency, current styles/owners and weakref release. Capital/capsule actual-source producers remain pending, so this is not final surface-order acceptance.

Direct GH/Excel public source_files currently list the shared renderer file without propagating its source_files result. The new kernel implementation and schema therefore need explicit manifest propagation in both consumers; relying on loaded Python/AST closure would not establish schema inclusion. Sent this concrete source-provenance follow-up to GH, who owns the corresponding save.

The subsequent saved GH/Excel manifest reread resolves the preceding follow-up: both explicitly propagate *_nakagin.source_files(), and _nakagin is their imported shared renderer module. That renderer returns its own file plus the kernel implementation/schema manifest. This closes the direct source-manifest omission at source level; final recorder endpoint maps remain run-specific evidence.

### Actual Producer Boundary

GH reports capital-fragment-final completed with unchanged endpoints. Its independent capital query exited1 after200/938 samples, so no completed nearest-surface acceptance exists yet. Current capsule and plane producers rejected an actual entry-handle nonplanar quad with0.84348mm residual. Strict rejection correctly enforces the reviewed planar Face contract; the renderer must use an explicitly selected canonical source triangulation rather than enlarging kernel tolerances. Root owns that decision. No final material/order/native/gallery acceptance is asserted.

## Final Ordering Checkpoint Delta

Read the released progress/cancellation delta. _order_checkpoint checks cancellation before progress and again after the callback. Input hashing, actual common-plane preparation, polygon projection and overlap outer sweep report truthful counts at0/128/end, with callback-absent fast paths. Geometry values and cache-key inputs remain unchanged. GH records initial2reds then12fragmenttests PASS0.763s, including257Items and cancellation before dependency ordering. The128 bound is on outer Item work, not wall-clock latency or inner candidate/polygon count; strict kernel loops retain their own finer cancellation. No additional source blocker was found in this bounded reread. Fresh actual exports/profile/native/final galleries remain pending.
