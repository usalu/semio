# Warped Entry-Handle Boundary

The strict fragment kernel correctly rejects entry-handle sourceItem1768 polygon0. Its maximum-anchor-triangle plane leaves another vertex0.843480810mm off plane; this is genuine nonplanarity, not a bucket-normal precision issue. The original physical boundary is:

[
  [
    -0.1799497474683059,
    -0.08500000000000009,
    0.9699497474683058
  ],
  [
    -0.17774562589193463,
    -0.13088156116209823,
    0.9699497474683058
  ],
  [
    -0.17500000000000004,
    -0.1350000000000001,
    0.972
  ],
  [
    -0.17500000000000004,
    -0.08500000000000009,
    0.972
  ]
]

The existing Rhino3dm mesh oracle converts the same quad into triangles `(0,1,3)` and `(1,2,3)`, using the shorter1–3 diagonal (0.04600934309087158m versus 0.050286216157548774m for0–2). Script `audit-warped-quad.py` ran through repository admission and stored its generated result. No model, kernel tolerance or source vertices were changed.

A declared canonical mesh triangulation must precede strict planar Faces; original polygon identity and artist shading should remain separately traceable. It must not pretend this quad has one exact plane. Final capsule fragment production is blocked on this source-contract correction.

The permanent language-neutral fragment-rendering case now contains this exact quad and expected triangle indices. The independent NumPy/Rhino regression observed its first red result: missing `_polygon_facets`, one test, 0.637 s. No production decomposition or precision change has been made at this stage. The next profile input now requests one nonrepeating stack capture, preserving the earlier unexplained interrupted runs as non-passes.

The isolated helper is now green against the exact Rhino/NumPy witness: one test, 0.533 s. It keeps planar boundaries whole and splits only genuinely warped quads on the shorter diagonal (ties use 0–2); nonplanar boundaries with more than four vertices still reject. It is not yet called by production ordering. No source vertices, original polygon IDs, model data or kernel precision have changed.

## Caller Integration

The caller now expands each genuinely warped quad into canonical planar facets before creating strict kernel Faces. Both facets retain the original source and polygon IDs; their geometric normals are recomputed individually. Original Item boundaries remain the previsibility inventory, original lighting remains separate, and only original boundary edges can acquire strokes. No physical model or kernel source changed. The complete six-test renderer fragment suite passes in 0.710 s, and its strict AJV contract rejects five malformed inputs.

The fresh `capsule-fragment-faceted-final` producer completed with unchanged source endpoints: 2,368 original Items and 470 retained. At 256 pixels, full/stable/culled geometric, silhouette and source-ID controls are exactly equal. At 1,024 pixels, geometric and silhouette controls are exactly equal. Normal Cairo material/AA differences remain explicit; they are not accepted through a colour tolerance. The independent nearest-surface audit consumes the original boundaries and separately declared canonical mesh faceting, rather than deriving nearest geometry from paint fragments.

## Pre-SCC Narrow-Order Defect

Parent identified a second, separate plane assumption: a raw nonplanar Item can receive a false uniform order before the strict kernel is reached. A new equal-normal saddle quad crossing a planar patch observed red in 0.003 s: narrow order returned uniform `1` instead of crossing `2`. The same-normal shortcut can also bypass it. The pending correction will derive strict common-plane information once per Item and send actual positive-overlap nonplanar uncertainty into a fragment SCC before those shortcuts. Broad vertex support bounds remain conservative. Current profile and full-suite evidence are preserved before this production change.

## Corrected Common Planes and Singleton Facets

The declared mesh rule now applies before any narrow plane ordering and also to isolated warped Items. `_item_plane` validates a single actual geometric plane against every source vertex at unchanged 64-ULP precision. Actual positive material overlap with noncommon planes enters a fragment SCC before the equal-normal shortcut. Projected overlap tests use the canonical facet material, not an unqualified warped loop. Singleton warped groups also use the kernel, and actual triangle winding determines their per-facet camera visibility independently of artist lighting.

The permanent suite now passes nine tests in 0.617 s. It includes the equal-normal warped/planar crossing (observed red false uniform order), isolated warped faceting (observed red one raw Item), and a projected self-crossing singleton whose raw normal is edge-on while one actual triangle faces the camera. The strict AJV fixture schema accepts the current contract and rejects five malformed variants. No model vertices or kernel tolerances changed.

The final-source full occlusion gate still needs replay: the preceding 23-test run passed 22 tests and found a sentinel test identity lookup error after fragmentation. The corrected isolated sentinel passed in 41.482 s, comparing original-source aliases plus exact fragment geometry, normals and order, rather than pretending every paint Item remains an original Item. That run preceded the common-plane correction and is retained as bounded evidence.
