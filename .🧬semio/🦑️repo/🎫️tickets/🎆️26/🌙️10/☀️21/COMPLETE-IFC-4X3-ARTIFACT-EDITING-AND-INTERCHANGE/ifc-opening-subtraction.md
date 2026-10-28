# Closed-Solid IFC Opening Subtraction

The shared IFC geometry engine now subtracts actual world-space IfcRelVoidsElement operands through a first-party closed-triangle-solid interface. Every host is published only after all of its opening operations succeed. Opening volumes remain excluded from display with their original source IDs. Invalid, unsupported, nonmanifold or incomplete operands/results do not publish an uncut host.

## Owned Interface and Algorithms

IFC/📐️geometry/🧊️boolean contains the strict neutral schema, owned Rust/TypeScript SolidMesh contracts, finite original caller controls, native/TypeScript implementations and permanent neutral/source tests. No external runtime dependency was introduced. The kernel uses plane-partitioned polygon clipping, normalized source coordinates, spatially indexed vertex welding, conforming edge splitting and convex polygon triangulation. Numeric weld/classification tolerance is 1e-9 of the combined operand extent; ill-conditioned/thin geometry and invalid topology are refused rather than replaced by bounds.

Admission and result acceptance independently require finite nondegenerate triangles, two oppositely directed uses of every welded edge, a single-cycle link at every vertex, no edge-interior vertices, no nonincident triangle intersections or coincident sheets, and consistent solid-angle shell nesting/winding, including inward cavity boundaries. Empty results and disconnected valid solids are permitted. A cut whose void boundaries only meet along an edge can produce a pinched nonmanifold boundary; that result is explicitly refused. This bounded floating-point implementation is not a claim of arbitrary-solid conformance, exact arithmetic, halfspace support or full IFC CSG.

The shared projector reads opening product/representation graphs through their own standard readers, composes source placements and mapped transforms before subtraction, preserves source product/representation and interaction IDs, and atomically publishes only complete products. Both IFC4 and IFC2x3 native entry points use the same first-party operation. Existing semantic/details mutation routing remains central.

## Canonical Controls

The schema-first shared geometry limits are 📐️geometry/📜️limits.json: work8,000,000, vertices200,000, triangles400,000 and depth64. Rust and TypeScript defaults consume the same tested values. The standalone Boolean schema declares work2,000,000, polygons200,000 and partition depth256 defaults; the projector supplies remaining original caller work and its topology caps. Every plane/edge/face split, tree/spatial traversal, validation and emission phase is controlled, including empty/disjoint copying and final completion. Native request rendering continues to cap these defaults by StepContext fuel, cancellation and yield. There is no resume cursor; a one-shot limited render does not demonstrate completed interactive work.

## Executed Evidence

- Permanent shared Boolean TypeScript suite: 17 groups passed, zero strict diagnostics. It covers disjoint, contained cavity, through tunnel, face/edge contact, coplanar half/notch, identical/covering empty results, translated/reflected solids, malformed/open/intersecting/pinched/wrongly nested operands, strict schema and all controls.
- Independent existing Three BoxGeometry and ExtrudeGeometry reconstruct matching solid, cavity, hole and notch output. Volume and closed oriented incidence agree.
- Permanent opening source suite: 7 groups passed, zero strict diagnostics. It covers native source placements, multiple valid openings, translated world geometry, IFC2x3 native input, invalid cutter refusal and original cancellation with no uncut-host publication.
- The unchanged official IFC4 slab-openings source is retained under CC BY-ND4.0 with original FILE_NAME metadata, pinned provenance and SHA-256 736a4aad0f03ac6512d486e7d1e5e94e68cfe8b5916c7e9e6d14b0215e26ac62. Output is host303/representation314, 1,342 vertices and2,684 triangles; opening325 and recess336 remain void diagnostics. The strict result is accepted at4,263,443 workticks under the8m default. One instrumented actual-source projection took1,200.16ms in Node REPL; this timing excludes the preceding source compilation/typecheck and is not a native/UI benchmark.
- Official sampled solid volume:1,199,056,451.5685766mm³. Independent Three EllipseCurve/ShapeUtils sampling from official analytic arc centers gives1,199,056,451.5686657mm³ (absolute difference0.0000891mm³, approximately7.4e-14relative). The fixture and permanent test compare this sampled geometry; analytic curved volume is different and is not misrepresented as the tessellated result.
- The previous IFC4 geometry source suite continued to pass45 groups at the first integration check; the root owns later growing geometry fixture checks.
- Eight owned native kernel/projector/facade/test sources pass the independent existing Rust grammar with no syntax errors. Native neutral, translated/multiple/official IFC4 opening and IFC2x3 source regressions are authored. Native compilation and execution remain unverified: repo-routed tests stop before execution at the Windows native-build cache per-directory case-sensitivity access denial.

## Independent Gate

Existing manifold-3d3.5.1 was imported and its original WASM initialization attempted. The Node REPL embedder rejects WebAssembly.instantiate with “Wasm code generation disallowed by embedder”. No bypass was attempted. Permanent manifold.test.ts is registered for actual repo-routed execution once that route works; it compares all neutral operations and independently admits the first-party output mesh. Its source typechecks with zero diagnostics, but its WASM assertions have not executed. No claim of native IfcOpenShell or Manifold oracle execution is made.

Inputs retained in this ticket: ifc-boolean-check.mjs, ifc-opening-source-check.mjs and ifc-opening-check.mjs. The root owns suite registration and ticket closure. Generated own-prefix outputs are cleaned after final checks.
