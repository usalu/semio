# 📓️ What the reference kernel actually does

Findings from building the BRep corpus against `brepjs` / OpenCASCADE 8.0. Every one was MEASURED
while generating a fixture, not read in documentation. They matter twice over: each is a behaviour the
corpus now pins, and each is a decision a new BRep kernel will have to make deliberately rather than
inherit by accident.

## 🔴️ `thicken()` returns a NEGATIVE-volume solid, and `fuse()` then behaves like `cut()`

The sharpest finding in the whole corpus. `thicken()` applied to a cylinder's curved face returns a
solid of the **correct magnitude but inverted orientation** — a negative volume. Fusing that inverted
solid wholly inside a block does not add material; it **removes** it:

```
block 108 000 mm³  ⊕  thickened tube 6 031.86 mm³   →   101 968.14 mm³, solids: 2
                                                        (= 108 000 − 6 031.86)
```

`fuse` silently subtracted, and split the result into two disjoint bodies. Nothing in the API surface
suggested a subtraction was happening. Pinned as `fuse-thickened-shell-into-block`, declared `disjoint`.

**For a kernel author:** face orientation is not cosmetic bookkeeping — it is the sign of every
subsequent boolean. A kernel that does not validate orientation on the output of an offset-style
operation will produce this class of defect, and it will look like a boolean bug rather than an offset bug.

## 🟠️ `fuseAll` over three or more shapes silently fails to merge

Shapes that pairwise `fuse` into one manifold solid do **not** merge when handed to `fuseAll` together.
No error is raised; the result is simply not one solid. Two mechanical recipes were rewritten to use
sequential `fuse` instead.

**For a kernel author:** an n-ary boolean is not a fold of the binary one unless you make it so.

## 🟠️ `fillet(shape, radius)` over all edges can return an invalid zero-volume solid, silently

When the radius exceeds what the local material thickness supports, `fillet` applied to every edge
returns a solid that is invalid and has zero volume — **and throws nothing**. Only measuring the result
reveals it. Fixed in the corpus by filleting selected edges.

**For a kernel author:** a blend that cannot be constructed must fail loudly. A zero-volume "success"
propagates into everything downstream.

## 🟡️ `chamfer` throws on complex topology

`CHAMFER_FAILED` on an 88-edge topology. Recorded and the offending step dropped rather than hidden —
an honest refusal, unlike the two above.

## 🟡️ `fuse` leaves redundant un-merged coplanar faces; `intersect` does not

Three independent constructions — `fuse-face-touching-boxes`, `fuse-coincident-faces`,
`fuse-nearly-identical-operands` — all show `fuse` leaving **10–14 faces** on results that are
analytically plain boxes with a minimal topology of 6. `intersect` on equivalent geometry reaches the
minimal topology every time.

**For a kernel author:** matching volume, and even matching mesh, does not imply matching `fuse`
topology. This is exactly why the comparison pipeline asserts the COMPONENT count and the genus but
deliberately does **not** assert face and edge counts.

## 🟡️ Exact contact imprints without removing volume

`cut-tangent-cylinder-exact`: a cutter exactly tangent to a face removes **zero** volume
(8000 → 8000 to within 1.8e-12) while taking the shape from 6 faces / 12 edges to **7 / 15**. The
tangent line is imprinted. One epsilon below, nothing changes at all (6 / 12); one epsilon above, a
sliver of 8.44e-08 is removed and the shape reaches 9 / 21.

**For a kernel author:** the three rungs are semantically distinct and a volume-only comparison cannot
tell the first two apart. This is the corpus's canonical contact bracket.

## ⚪️ STEP export is not byte-self-deterministic

Two `exportSTEP` calls on the SAME shape in the SAME process differ: OCCT stamps an incrementing
translator counter into `PRODUCT` and a wall-clock timestamp into `FILE_NAME`. Every generated fixture
therefore records `reproducible: false`, and raw STEP byte equality is not a gate in either direction.

## ⚪️ `importSTL` rejects `exportSTL`'s own output

Round-tripping a box through the library's own STL writer and reader fails with `STL_IMPORT_FAILED`,
for both the ASCII output it produces and a hand-written binary STL. Not on the critical path — the
mesh gate uses `manifold-3d` and never needs STL — but it rules STL out as a transport between probes.

## ⚪️ Tessellation emits one vertex per FACE CORNER

`mesh()` returns unwelded vertices, so no two triangles share an index and a mesh kernel correctly
refuses the result as non-manifold. The mesh probe welds on a fixed 1e-7 grid before handing it over.

**For a kernel author:** decide explicitly whether your tessellator emits welded or per-corner
vertices, and say so — every downstream consumer has to know.

## 📐️ Conventions, measured

| Call | Convention |
| --- | --- |
| `box(dx, dy, dz)` | CORNER at origin |
| `cylinder(r, h)` | AXIS at origin, extending +z |
| `rotate(shape, angle, {at, axis})` | angle in **degrees**, ONE options object |
| `exportSTEP` / `exportSTL` | return a `Blob`, not a string |
| Boolean results | a COMPOUND even when holding exactly one solid, so `isSolid` is false and the solid COUNT is the assertion |
