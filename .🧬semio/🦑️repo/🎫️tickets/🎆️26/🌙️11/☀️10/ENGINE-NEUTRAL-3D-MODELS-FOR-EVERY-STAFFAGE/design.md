# Engine-Neutral 3D Models for Every Staffage — Design

Normative for every agent of this ticket. Change it only through the coordinator.

## Layers

| Layer | Place | Content |
|---|---|---|
| Neutral model kernel (domain-neutral) | `🧰️framework/🔨️modules/🧊️3d/🗿️model/` | schema `semio.model/v1`, typed twins, solid tessellation, mechanism (joints), skinning, animation sampling, GLB encoder, FBX encoder, validation |
| Staffage volume (domain-specific) | `🎓️teaching/🏛️architecture/🧍️staffage/` | `asset.volume` (declarative solids bound to the rig), `🔨️modules/🧊️volume` (asset + evaluated state → `ModelDocument`), `📤️export/🧊️glb`, `📤️export/🎬️fbx`, `📤️export/🗿️model`, 3D views of the seven representations, viewer |

No runtime dependency on any engine or library (three, Blender, FBX SDK, glTF libs). Third-party libraries are test oracles only.

## Frames and units

- Staffage world (unchanged): x ahead, y up, z left, left-handed, metres, degrees.
- Neutral model and both file formats: right-handed, +Y up, +Z ahead (front of the asset), +X the asset's left, metres, radians, quaternions `[x, y, z, w]`, column-major 4×4 matrices. Mapping from staffage world: `X = z`, `Y = y`, `Z = x` (one reflection, so triangle winding is re-oriented to CCW outward).
- Staffage bone-local frame for solids: `u` along the bone, `v` its normal in the sagittal plane, `w` lateral (left is +) — the same frame the existing `solid` hint uses.

## `semio.model/v1` (`ModelDocument`)

Schema-first JSON Schema draft-07 at `🗿️model/🧬️schema/🔣️.json` with typed twins. All text is `{ en, de }`.

```
ModelDocument { $schema, version: "semio.model/v1", id: Slug, title: Text, provenance?,
  nodes:      Node[]       { id, parent: Slug|null, translation[3], rotation[4], scale[3], joint?: Joint, mesh?: Slug, skin?: Slug, tags?: Slug[] }
  solids:     Solid[]      procedural declarative geometry (below)
  meshes:     Mesh[]       { id, primitives: Primitive[] }
  skins:      Skin[]       { id, joints: Slug[], inverseBind: number[16][] }
  materials:  Material[]   { id, title, baseColor[4], metallic, roughness, emissive[3], alphaMode: opaque|blend, doubleSided, unlit }
  variants:   Variant[]    { id, title, bindings: { [materialId]: materialId } }   named looks of one geometry
  poses:      Pose[]       { id, title, nodes: { [nodeId]: { translation?, rotation?, scale? } } }
  animations: Animation[]  { id, title, durationS, loop, channels: Channel[], events: { t, id }[] }
}
Joint     { type: fixed|revolute|prismatic|spherical|free, axis[3], min?, max?, twistMin?, twistMax? }   the mechanism: limits in rad or m, node-local axis
Primitive { mode: triangles|lines, material: Slug, positions: number[], normals?: number[], colors?: number[] (rgba), joints?: number[] (4 per vertex, skin joint index), weights?: number[] (4 per vertex), indices: number[] }
Channel   { target: { node: Slug, path: translation|rotation|scale } | { material: Slug, path: baseColor|emissive }, interpolation: step|linear, times: number[], values: number[] }
Solid     { id, node: Slug, material: Slug, tags?: Slug[], transform?: { translation?, rotation?, scale? }, repeat?: Repeat, weights?: { node: Slug, weight: number }[] } &
          box { size[3], cornerRadius? } | ellipsoid { radii[3] } | capsule { p0[3], p1[3], r0, r1, lateralScale? } |
          revolve { profile: [axial, radial][], axis[3], origin[3], sweepRad?, lateralScale? } |
          extrude { profile: [a, b][], holes?: [a, b][][], plane: xy|yz|zx, from, to, taperTo?: [fromScale, toScale] } |
          loft { sections: { at[3], rx, ry, exponent? (superellipse, 2 = ellipse, large = rectangle), rotation? }[], capStart, capEnd }
Repeat    { count, translation?[3], rotationAxis?[3], rotationRad?, origin?[3] }   k-th copy applies the step k times
```

A document is *procedural* when it carries `solids` and no meshes, *resolved* after `tessellate(document, quality)` turned every solid into primitives skinned to its node (`weights` absent = rigid to `node`). Both validate against the same schema. `quality` is `{ chordM, minSegments, maxSegments }`, deterministic; identical input gives identical bytes.

Kernel API (names fixed, all pure, all with progress + cancellation through one `Operation { report(fraction), cancelled() }` parameter):

```
validateModel(document) → Issue[]
tessellate(document, quality, operation) → ModelDocument (resolved)
poseModel(document, poseId | { animation, timeS }) → node world matrices
featureEdges(document, angleRad) → ModelDocument with lines primitives added
encodeGlb(document, { pose?, animations: boolean, variants: boolean }, operation) → Uint8Array
encodeFbx(document, { animation?: Slug, variant?: Slug, format: "binary" }, operation) → Uint8Array
```

- GLB: glTF 2.0, one buffer, skins, animations (node TRS; material colour channels through `KHR_animation_pointer`), `KHR_materials_variants` for `variants`, `KHR_materials_unlit` for `unlit`, joints as `extras.joint`.
- FBX: binary 7.4 (version 7400, zlib-deflated arrays; inflate/deflate owned or from the platform's system library, never an npm runtime dependency), `Model` LimbNode skeleton, `Geometry` with normals and materials by polygon, `Deformer` Skin/Cluster, bind `Pose`, `AnimationStack`/`Layer`/`CurveNode`/`Curve` for node TRS and for material `DiffuseColor`/`EmissiveColor`, joint limits as rotation limit properties, `GlobalSettings` Y-up right-handed metres.

## Staffage

- `ExportFormat` gains `glb`, `fbx`, `model`. `Asset` gains the required key `volume`:

```
Volume { status: "authored", solids: VolumeSolid[], decisions: Text[] }
VolumeSolid = the Solid kinds above with: `bone` instead of `node` (coordinates in the bone-local u, v, w frame at rest, metres), `material` (asset material id), `role` (ShapeRole), `tone` (0..1),
              `region?`/`regions?` (thermal region ids), `maxDenominator?` (coarsest preset scale that still shows it), `weights?` over bones
```

- `🔨️modules/🧊️volume`: `modelOf(asset, { view, scale, environment? })` → procedural `ModelDocument`: nodes = rig bones (rest pose, joints mapped to the mechanism), solids = volume solids, poses = asset poses (via `evaluate`), animations = asset clips sampled with `evaluate` at the clip fps (root motion included), variants = the seven views.
- The seven views as variants of one geometry: `dark-solid`/`light-solid` tone materials; `dark-line`/`light-line` paper-coloured unlit surface plus feature-edge `lines`; `thermal` one material per thermal region coloured by the solved `ThermalField` with the 2D colour ramp (animated per clip as material colour channels); `forces`/`moments` one material per bone coloured by N or M with the 2D ramps, plus glyph solids (arrows for loads and reactions, arcs for moments) tagged `glyph`.
- Exports: one GLB per static pose (all seven variants inside), one FBX per clip (one view, default `light-solid`), `model` = the neutral JSON document. CLI `--format glb|fbx|model`, site downloads, zip bundles. Files are produced on demand like every other staffage export and are not checked in.
- A fourth viewpoint `axonometric` depicts the same model through the existing `DrawingScene`, so SVG/PDF/DXF get the 3D view; the React target gets an owned WebGL2 orbit viewer.

## Tests

- Language-agnostic vectors under `🧫️fixtures/` (kernel: per solid kind, skinning, animation sampling, GLB and FBX structure digests; staffage: sample figure volume).
- Third-party oracles (test-only): Khronos `gltf-validator` and three `GLTFLoader` for GLB, three `FBXLoader` (and Blender/assimp/ufbx where installed) for FBX, an independent mesh library (e.g. trimesh/shapely) for volumes, bounds and watertightness.
- Catalog-wide guard: every asset has an authored volume; its bounding box matches `dimensions` within tolerance; every pose exports a valid GLB and every clip a loadable FBX; each solid's silhouette agrees with the authored elevation/front/plan shapes within tolerance.

## Rules for agents

- Read the root `AGENTS.md`. No git commands that modify anything. Re-read a file right before editing it; others edit this tree live.
- Use the `repo` binary for every build/test command; never `repo dashboard`.
- Temporary files, logs and scripts go into this ticket folder (tool output under `🗑️generated/`); write your report as a markdown file here and return only its path plus a five-line summary.
- No comments inside definitions; docstrings start with a unique fitting emoji; English first, German second, no default language.

## Amendments (coordinator, after the first authoring reports)

- **A1 — volume frame.** A `VolumeSolid` is handed to the kernel with `(x, y, z) := (u, v, w)` of its bone; `volume-check` already does exactly this and is the reference. `modelOf` realises it with one node per bone whose local axes are `(u, v, −w)` (right-handed) and mirrors every solid across `w` (positions, quaternions conjugated accordingly, winding re-oriented by the kernel). Kernel rules that name an axis therefore read in bone terms: X = u, Y = v, Z = w. This replaces the earlier wording "converted from the bone-local frame".
- **A2 — loft sections.** A section with `rotation` lies in the x, y plane of that rotation (unchanged, absolute). A section without `rotation` lies perpendicular to the loft path tangent at that section (central difference of neighbouring `at`), its x axis being the node X axis projected perpendicular to the tangent (fallback Z), y = tangent × x — the same rule the capsule uses for `lateralScale`. Kernel change; the staffage schema text states the same.
- **A3 — `lateralScale`.** Kernel semantics are normative: the radius is scaled along X projected perpendicular to the solid axis, fallback Z (in bone terms: along `u`, and along `w` for a solid along the bone). The staffage schema text is corrected.
- **A4 — closed revolve profiles.** A revolve profile whose first point equals its last is a closed loop (torus-like body) and is valid; the staffage validator accepts it and checks self-intersection on the loop.
- **A5 — solid provenance.** Resolved primitives keep which solid produced which triangles: `Primitive.solids?: { id: Slug, first: number, count: number }[]` (index ranges), so viewers can pick solids and exporters can name parts.
- **A6 — bounds.** `dimensions` describe the presentation state (`presentation.defaultPose`), not the rig rest pose; every bounds check evaluates that pose.
