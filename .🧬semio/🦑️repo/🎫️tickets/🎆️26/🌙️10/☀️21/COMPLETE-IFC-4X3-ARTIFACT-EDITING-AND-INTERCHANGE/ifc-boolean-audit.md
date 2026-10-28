# Closed-Solid Boolean Audit and Oracle Plan

Read-only review; no production edits. Shared first-party contract should expose source-owned mesh/status/issue/control types. Existing manifold-3d 3.5.1 is a test-only independent oracle, never a runtime dependency or leaked public type. High reports its actual WASM initialization rejected by the Node REPL embedder (Wasm code generation disallowed); do not count those cases as executed. Installed Three ExtrudeGeometry can independently construct known difference shapes with holes for a smaller executed comparison, but is not a general CSG oracle.

## Acceptance invariants

Validate operands and final output: all coordinates finite, indices in range, no zero-area triangles; every welded undirected edge has exactly two opposite directed uses; each vertex link forms one cycle, excluding pinch/bowtie vertices; no edge-interior vertex/T junction; no nonincident triangle crossing or coincident doubled faces; consistent shell orientation and nesting (cavity normals inward). Edge counts and positive signed volume alone can accept self-intersecting, pinched, disconnected-overlapping or incompletely cut surfaces. A global weld must not erase thin walls; use a scale-aware tolerance and reject unresolved ambiguity rather than marking complete. Account work/control throughout splitting, conformance, pair checks and final acceptance; publish no partial host on limited/cancel/invalid. Zero polygons can be a valid empty difference; do not conflate with a failed triangulation.

## Neutral failure matrix

For A=[0,2]^3, volume8:

| Operand B | Expected A minus B | Main trap |
|---|---|---|
| disjoint [3,4]^3 | volume8, same boundary | unnecessary plane splits/tolerance drift |
| exactly A | empty valid solid | duplicate opposite coplanar sheets |
| enclosing [-1,3]^3 | empty valid solid | treating no polygons as failure |
| contained [.5,1.5]^3 | volume7, two nested shells | inner shell wrong orientation or ignored |
| crossing [1,3]x[0,2]x[0,2] | volume4 | coplanar side fragmentation/T junction |
| outside sharing one full face [2,3]x[0,2]^2 | volume8 | removing A contact face |
| outside sharing edge or vertex | volume8 | nonmanifold contact fragments |
| thin through slot [.9,1.1]x[-1,3]x[-1,3] | volume7.2, two components | accidentally weld gap or discard component |
| internal tunnel [.5,1.5]^2x[-1,3] | volume6, through-hole topology | missing tunnel walls despite positive volume |
| duplicate opening repeated twice | same as once | double subtraction/coplanar instability |
| overlapping openings | subtract their union | double-counted removed volume |

Repeat reflection, rotated placements, reversed triangle order, equivalent alternate face diagonals, very small/large world scales and large common translation. Add open/nonmanifold operands, NaN, invalid indices, cancel during final validation and exact fuel exhaustion. Invalid input must never yield a complete manufactured host. Do not rely only on triangle count: legitimate triangulations differ.

## Installed public oracle API

Package exports allow ESM import from manifold-3d or manifold-3d/manifold.js; CommonJS require.resolve(package root) fails because only import conditions are exported. Public types in installed manifold.d.ts / manifold-encapsulated-types.d.ts define Module default initialization, Manifold.cube(size,center), new Mesh({numProp:3,vertProperties:Float32Array,triVerts:Uint32Array}), new Manifold(mesh), a.subtract(b), result.status(), result.volume(), result.getMesh(), and delete() ownership cleanup. Initialize the public module and setup according to its bundled declaration/example; do not call internal WASM exports. Check operand/result status before comparing output. Differential comparison should include topology, occupied-point probes, volume and bounds; Float32 mesh upload needs coordinates rebased/scaled so the oracle does not itself lose tiny details at huge world offsets.

## Exact Official Slab Openings

Retained unchanged input official-slab-openings.ifc; pinned official source: https://raw.githubusercontent.com/buildingSMART/Standards-builds/a8664407b63d42651e48a78a252c76d09dd6274c/IFC/RELEASE/IFC4/ADD2_TC1/HTML/annex/annex-e/slab-openings.ifc . Provenance/hash in ifc-opening-subtraction-plan.md. #102 sets millimetres. Host #303 has global z[-200,0]mm. Its width1000 rectangle with y[0,4000] has two bulging circular sides: each radius5200mm, sweep0.7895822393995231rad. #308 uses two IFCLINEINDEX and two IFCARCINDEX segments. Host placement contributes z−200 once; solid Position is unset. Circle #325 is independently placed at center(100,300),r50,z[-200,0]; recess #336 centered(500,1000),size1000x500,z[-50,0]. Opening placements are not implicitly relative to host: use their explicit own world placement chains, then common-coordinate subtraction.

Analytic perimeter area6150303.753363103mm²; gross volume1230060750.6726205mm³. Through circle removes1570796.3267948965mm³; recess removes25000000mm³. Net analytic volume1203489954.3458257mm³ =1.2034899543458257m³. Both openings touch the host top plane exactly; the circle also touches bottom exactly. Thus this fixture tests coplanar subtraction, not only overlapping cutters extending beyond faces. Recess is inside host horizontal outline and disjoint from circle. Actual tessellated-volume oracle must use identical independently reproduced angular sampling; analytic value is a limiting reference, not an exact finite-mesh assertion. Preserve circle and perimeter arc approximation separately, and test no opening product appears as visible solid.

No native or full Boolean oracle execution claimed by this audit.

## Executed Independent Finite-Mesh Volume Oracle

Input harness ifc-slab-three-oracle.mjs independently samples public Three EllipseCurve from exact official slab dimensions/analytic circle centers, without reading first-party projection vertices. At72fullcircle segments, each perimeter half-arc uses5steps; gross1225625254.9381237mm³, circle removal1568803.3694578472mm³, recess25000000mm³; net1199056451.5686657mm³. High reports first-party Boolean volume1199056451.5685766mm³, difference0.0000891mm³ (~7.4e−14relative). This supports finite-mesh volume, not by itself topology acceptance or full Boolean correctness.
