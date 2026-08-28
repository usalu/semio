# 📓️ The mesh pilot — and the assumption it broke

A second subset-scoped external-oracle pilot, on `s.stdio.semio@v1/✳️mesh`: **17 mutations, all 17
externally oracled, 65 third-party-generated fixtures.** It is the first owner in the repository where
every mutation of the subset is covered, and building it overturned a tolerance assumption carried over
from the BRep pilot.

## The finding: tessellation freedom is a BRep property, not a mesh property

The BRep pilot gates in *tessellation tolerances*, because a solid can legitimately be tessellated
many ways. I began by assuming the same here and tested it directly:

| comparison | relativeVolumeError |
| --- | --- |
| same sphere at 8192 tris vs 128 tris — *legitimate* | **8.43e-2** |
| bore r=5 vs r=6, identical tessellation — *a real defect* | **1.07e-1** |

**The two overlap.** Any threshold loose enough to accept the first accepts the second. A gate built on
that assumption would have passed a wrong solid, which is worse than no gate because it reports green.

The reason is a real distinction: this subset stores explicit vertex and index buffers, so a mutation
transforms them deterministically. `move-vertex` does not re-tessellate anything — it moves one vertex.
Expected and actual must therefore agree EXACTLY, and the measurements say they can:

| comparison | relVol | relArea | Hausdorff |
| --- | --- | --- | --- |
| same mesh, STL vs OBJ | 0.000e+00 | 0.000e+00 | 0.000e+00 |
| same mesh, STL vs PLY | 0.000e+00 | 0.000e+00 | 0.000e+00 |
| same mesh, STL vs glTF | 0.000e+00 | 0.000e+00 | 0.000e+00 |
| same mesh, PLY vs glTF | 0.000e+00 | 0.000e+00 | 0.000e+00 |
| **bore r=5 vs r=6** | **1.07e-01** | 1.97e-02 | **9.99e-01** |

One mesh through four independent format readers agrees to the bit; a one-millimetre geometric error
separates by five orders of magnitude. That is the gate.

## Why glTF carries the pilot

Research read every `serialize_bytes` body: **all four carrier serializers here are real**, none are
`print_dsl` stubs. Of the four, only glTF encodes PBR metallic-roughness materials and texture images,
so it is the only carrier that can witness 10 of the 17 mutations. STL, OBJ and PLY carry triangles, so
a roughness change is invisible in them by construction — the `gltf-materials` and `material-compare`
probes therefore return `unsupported` rather than an empty `ok` when handed one. An empty material list
reported as ok would let a roughness mutation pass against a carrier that never encoded roughness: a
green result standing on the absence of the evidence.

Verified round-trip: a mesh exported with `roughness=0.4, metalness=0.1` reads back through three's
`GLTFLoader` as exactly `rough=0.4, metal=0.1`.

## Two engine families, on purpose

`three` 0.182.0 parses the carrier; `manifold-3d` 3.5.1 measures volume, area, genus and components;
`three-mesh-bvh` answers the closest-point queries behind the Hausdorff bound. Because the reader and
the measurer are independent implementations, the measurement checks the parse rather than confirming
it. manifold refuses a triangle soup outright, so welding is a precondition, not tidying — on a grid
keyed to the bounding-box diagonal (1e-7 relative). A 20 mm bored cube re-imported from STL welds 6336
soup corners back to exactly the 1056 shared vertices it was built with, 0 degenerate triangles,
**genus 1** — the bore survives as a topological fact, not merely as a triangle count.

## Three bugs found by building it

* **Fixture paths resolved against the wrong directory.** `verifyFixture` joins onto the manifest's
  `manifestDir`, which the loader sets to the *oracle* directory, not the fixture directory. Bare
  `<recipe>/<file>` paths resolved to a non-existent `🧪️oracle/<recipe>/…` and read as 369 mismatches.
  Fixed in the generator so regeneration stays correct, not just in the emitted data.
* **Two browser APIs fail silently under Bun.** `GLTFExporter` assigns `onloadend`, not `onload`, and
  `GLTFLoader` dispatches a `ProgressEvent`. A shim missing either does not throw — the completion
  callback simply never fires and the run hangs. The probe suite now carries a watchdog, because in a
  test harness a hang reports nothing at all, which is strictly worse than a failure.
* **One fixture legitimately cannot be built.** `degenerate-microscopic-cube` (a 1e-9 mm cube) collapses
  all eight corners onto one lattice point at the weld floor. Kept in the corpus with the cause
  recorded, rather than dropped to make the corpus look clean.

## Corpus

65 of 66 recipes across `primitives` (16), `booleans` (20), `topology` (12), `scale` (8),
`degenerate` (10) — built with manifold-3d, exported to all four carriers, then **re-imported and
measured from the written artifact** rather than from the in-memory shape. Byte-identical across two
runs, so unlike the BRep corpus this one is reproducible: repository-wide fixture reproducibility went
1.65% → 36.02%. The `scale` family holds one shape at 1e-3, 1, 1e3 and 1e6; welded vertex count stays
exactly 1056 at every scale while volume and area scale by exactly factor³ and factor² — direct
evidence the scale-relative tolerance works across nine orders of magnitude.
