# 📓️ Why the stub exporters exist — and why they are not 1047 separate jobs

The blocked half of the repository looked like a long list of unwritten exporters. Reading the artifacts
rather than counting the stubs shows something different, and it changes the plan.

## The measurement

Of the **35 owners** carrying stub serializers:

| | subsets | mutations |
| --- | ---: | ---: |
| hold only **child handles** — geometry lives in a child artifact | **15** | **249** |
| hold **embedded geometry** — an exporter could be written directly | 3 | 12 |
| neither / other document kinds | 17 | — |

## What a child-handle artifact actually is

`LowpolyObject` says it outright in its own doc comment: the object stores a two-string handle
(`child_id`/`target`) to an `s.stdio.semio.mesh` document, **"never embedded geometry"**. The same shape
holds across the group:

| composite | child it holds | stub carriers |
| --- | --- | --- |
| `lowpoly` | `SemioMeshSnapshot` | stl, obj, ply, gltf, las, dwg, png |
| `gisterrain` | `SemioMeshSnapshot` | stl, obj, ply, gltf, las, dwg, png |
| `cad` | `SemioDrawingSnapshot`, `SemioModelSnapshot` | stl, obj, gltf, step, ifc, dwg, png |
| `block/3d` | `SemioKitSnapshot` | stl, obj, zip, png |

So the exporters are not unwritten out of neglect. A serializer with the signature
`serialize(&Snapshot) -> Result<TargetSnapshot>` **has no way to reach the child**, and the composite
holds no triangles of its own. Faced with that, the leaves did the two things the gate now catches:
printed the DSL, or reinterpreted the parent's pack bytes as the target type. The second is why
`shooting → png` decodes a `ShootingSnapshot` envelope as a `PngSnapshot`.

## The consequence: one mechanism, not fifteen exporters

These composites should not each grow a geometry writer. Their children **already have real, verified
ones** — `s.stdio.semio.mesh` writes genuine STL, OBJ, PLY and glTF, and `s.stdio.semio.brep` mints a
real AP214 Part-21 graph. Both are now under third-party oracles with reproducible corpora.

What is missing is a **child-resolving export context** so a composite's carrier leaf can resolve its
handles and delegate to the child's serializer. One mechanism unblocks the group, and it inherits the
oracles the leaves already have rather than needing new ones.

That reframes the two pilots. `semio.mesh` (17/17) and `semio.brep` (13/13) are not 2 subsets out of
122 — they are the leaf artifacts that 15 composites, holding 249 mutations, would delegate into.

## What is genuinely a writing job, and what should be withdrawn

Three owners hold embedded geometry (12 mutations) and can have exporters written directly.

The remaining 17 stub owners are document-shaped rather than geometric, and for several the honest
answer is that the export dialect should be **withdrawn, not implemented**. `procedural3d` is a
generation graph; an STL of it is not merely unimplemented, it is undefined. The gate's remediation text
already offers both routes: *implement the serializer, or remove the format from this subset's
exportDialects so the capability is not claimed.* A declared capability that cannot be given a meaning
is a false claim, and deleting it is a fix, not a retreat.

## Ordering that follows

1. **Register the 664 mutations whose subsets already write a real carrier** — no new exporter needed.
2. **Build the child-resolving export context** — unblocks ~249 mutations across 15 composites, reusing
   the mesh and brep oracles as-is.
3. **Write exporters** for the 3 embedded-geometry owners.
4. **Withdraw** export dialects that cannot be given a coherent meaning, rather than faking them.
