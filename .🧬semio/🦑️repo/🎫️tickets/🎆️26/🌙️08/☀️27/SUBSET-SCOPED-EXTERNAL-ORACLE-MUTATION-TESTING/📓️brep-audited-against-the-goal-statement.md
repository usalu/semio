# 🧊️ brep, audited clause by clause against the goal statement

The goal's worked example is brep, so it is worth checking the built corpus against that sentence
literally rather than against the coverage number. Audited today; every row is a file on disk.

| the goal says | what exists |
|---|---|
| "use andymai/brepjs in typescript" | `brepjs-occt`, `third-party-library`, brepjs **18.119.8** — the named oracle for all **13/13** kernel kinds |
| "for creating STEP files (both breps and the resulting meshes)" | **155 STEP files**; every fixture carries `operand-a.step` (+`operand-b.step` for booleans, 90 operand files), `expected.step`, `expected.mesh.json`, `expected.metrics.json` |
| "every mutation must produce the same STEP file" | pipeline stage `step-external-canonicalizer` → `canonicalBytesEqual: true` |
| "and a similar mesh (it can use different tesselation…)" | stage `manifold-mesh-compare` (manifold-3d 3.5.1, capabilities `mesh.hausdorff`/`mesh.volume`/`mesh.area`/`mesh.genus`/`mesh.components`) |
| "…but must have similar hausdorf distance" | `hausdorffInTessellationTolerancesMax: 3` — expressed in units of the fixture's own tessellation tolerance, which is what makes "different tesselation is allowed" enforceable rather than rhetorical |
| "volume, etc" | `normalizedSymmetricDifferenceVolumeMax: 0.01`, plus `relativeVolumeErrorMax: 1e-8`, `relativeAreaErrorMax: 1e-7`, `normalizedCentroidDistanceMax: 1e-8`, `normalizedBoundingBoxDiagonalErrorMax: 1e-8`, `connectedComponentsEqual`, `genusEqual` |
| "test complicated boolean operations" | **18 boolean fixtures**, and they are the hard cases on purpose: tangent spheres (point contact) and tangent cylinders (line contact), non-manifold corner-touching boxes, single and double nested voids at two scales, coincident face stacks, fuse→cut→intersect three-step chains, a multistep case scaled to 1e4, fully-engulfed cuts and non-overlapping intersects |

## The corpus is complete, and its gaps are declared

72 fixture directories. Seven carry no `expected.step`/`expected.mesh.json`, and all seven are supposed
to: two are empty boolean results (`booleans-cut-fully-engulfed-empty`,
`booleans-intersect-non-overlap-empty`) and five are rejected edits (non-planar `move-vertex` ×2,
`delete-edge` on a boundary edge, `delete-shell` on a solid's only shell, `delete-vertex` corner
cascade). Each declares it rather than merely lacking the file:

```json
{ "declaredOutcome": "empty",    "hasExpected": false, "kind": "delete-solid", "solids": 0, "volume": 0 }
{ "declaredOutcome": "rejected", "hasExpected": false, "kind": "delete-edge",  "solids": 0, "volume": 0 }
```

A mutation with no resulting shape has nothing to export; asserting bytes against a file that should
not exist would be the defect.

## Distribution across the thirteen kinds

`create-solid` 19, `move-vertex` 10, `replace-curve` 7, `replace-surface` 7, `delete-solid` 5, and 3
each for `create-edge`/`create-face`/`create-shell`/`create-vertex`/`delete-edge`/`delete-face`/
`delete-shell`/`delete-vertex` — 72 in total. The boolean corpus sits under `create-solid`, which is
why that kind carries 19: booleans are the INPUT that produces the solid whose STEP and mesh are then
compared.

## Standing caveat, unchanged

`brepjs`/OpenCASCADE already ships inside the `cad` plugin, so it is not an independent oracle THERE.
In this subset it is, because the subject is semio's own b-rep kernel and brepjs is the second party.
OCCT's STEP export is also not byte-deterministic, which is why the byte equality runs through
`step-external-canonicalizer` rather than over raw exporter output.
