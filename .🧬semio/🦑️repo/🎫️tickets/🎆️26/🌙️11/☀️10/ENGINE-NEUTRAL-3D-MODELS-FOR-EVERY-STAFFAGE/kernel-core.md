# Kernel core: `🗿️model` (semio.model/v1)

Root: `🧰️framework/🔨️modules/🧊️3d/🗿️model/` (abbreviated `M`). Zero runtime dependencies. Part of the existing `@semio-tech/framework-3d-js` project (no new workspace member, no lock change).

## Import specifier for clients

- From `@teaching/staffage` (relative path, same as `🔨️modules/📤️export/🎥️video`): from a module at `🔨️modules/<x>/🟦️.ts` use
  `../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts`; from `🔨️modules/<x>/<y>/🟦️.ts` (one level deeper) use six `../`.
- Package subpaths (for workspace members, in the package `exports`): `@semio-tech/framework-3d-js/model` (entry `📦️packages/🟦️typescript/🗿️model/🟦️.ts`, the kernel barrel including both encoders) and `@semio-tech/framework-3d-js/model/viewer` (entry `📦️packages/🟦️typescript/🗿️model/🖥️viewer/🟦️.ts`, the browser viewer; separate so clients without a DOM never load it). Relative paths into `🗿️model/🟦️.ts` and the deep module paths (`📤️export/🧊️glb/🟦️.ts`, `📤️export/🎬️fbx/🟦️.ts`, `🖥️viewer/...`) stay valid.
- Types: `import type { ModelDocument, Solid, Operation, ... } from ".../🗿️model/🟦️.ts"` (barrel re-exports the schema twin; `export type *`).

## Public API (barrel `M/🟦️.ts`)

- `validateModel(document, operation?) -> Issue[]` (sync; the operation is polled per solid, per primitive array and inside the outline pair loops).
- `tessellate(document, quality, operation?) -> Promise<ModelDocument>` (async; rejects with `CancelledError`, or `ModelInvalidError` when validation has errors).
- `featureEdges(document, angleRad, { material? }?, operation?) -> Promise<ModelDocument>` (async).
- `poseModel(document, selector?) -> { world: Record<node, Mat4>, local: Record<node, {translation, rotation, scale}>, issues: Issue[] }` (sync). `selector` = pose id | `{ animation, timeS }` | absent (rest). Throws on unknown ids.
- `sampleMaterials(document, animationId, timeS)` -> animated `baseColor`/`emissive` per material.
- Encoders: `encodeGlb(document, { pose?, animations, variants }, operation) -> Uint8Array`, `encodeFbx` / `encodeFbxDetailed(document, { animation?, variant?, format: "binary" }, operation, compressor?)`, with `GlbOptions`, `FbxOptions`, `FbxResult`, `Notice`, `Compressor`, `ownedCompressor`, `FbxEncodeError`, `FbxCancelledError` (a `CancelledError`).
- Helpers: `tessellateSolid(solid, quality, extra?, operation?)`, `restWorldMatrices`, `silentOperation`, `Pace`, `CancelledError`, `ModelInvalidError`, and math (`compose`, `multiply4`, `invert4`, `qMul`, `qSlerp`, `qRotate`, `transformPoint`).
- Progress and cancellation (audit fixes, `kernel-audit-fixes.md`): every function above takes the `Operation` (`validateModel(document, operation?)` too) and polls `cancelled()` through `Pace` (module `⏱️operation`, `CancelledError` and `silentOperation` live there) every ~2048 work units inside its hot loops; a cancellation throws/rejects with `CancelledError` and produces no result. Synchronous functions see a cancellation only if `cancelled()` can change while they run (deadline, shared flag); `tessellate`, `featureEdges` and `sceneOfAsync` also yield to the event loop.
- Schema `$defs`: ModelDocument, Node, Joint, Solid (BoxSolid, EllipsoidSolid, CapsuleSolid, RevolveSolid, ExtrudeSolid, LoftSolid), Repeat, Mesh, Primitive, Skin, Material, Variant, Pose, Animation, Channel, Text, Slug, Quality, Issue (+ helpers Vec2/3/4, Mat4, TransformPatch). `kind` discriminates solids.

## Semantics fixed by this implementation (encoders rely on them)

- Resolved document: `solids: []`. Rigid solids (no `weights`): one mesh per node (mesh id = node id, primitives merged per material, node-local coordinates, node `mesh` set). Weighted solids: model-space positions at the rest pose, joints/weights 4 per vertex (uniform per solid, at most 4, normalised), one skin `skinned` (joints in first-use order, inverse bind = inverse rest world matrix) on one mesh `skinned` carried by a new identity root node `skinned` (`mesh`+`skin`). Ids get `-2`, `-3` suffixes on collision.
- Triangles CCW outward; mirrored transforms (negative determinant) are re-oriented. Normals are unit; smooth on curved surfaces, hard on creases above 35 degrees and on flat caps.
- Order per solid: geometry, then `transform` (scale, rotate, translate), then repeat copy k = step^k, step = translate(`translation`) after rotation about `origin` by `rotationRad` around `rotationAxis` (default Y).
- Capsule: hull of two spheres; `lateralScale` scales the radius along the node +X axis projected perpendicular to the capsule axis (fallback Z). Revolve: same lateral rule; profile `[axial, radial]`, open ends are closed to the axis (flat discs), profiles whose first equals last point are closed loops (torus); partial `sweepRad` gets flat cap faces. Extrude taper scales about the outer ring's area centroid. Loft (A2): a section with `rotation` lies in the x, y plane of that rotation (absolute, unchanged); a section without `rotation` lies perpendicular to the path tangent at that section (central difference of the neighbouring `at`, one-sided at the first and last section; degenerate tangents fall back to the overall travel, then Z), its x axis being the node X axis projected perpendicular to the tangent (fallback Z when parallel) and y = tangent x x, produced by the very same helper (`revolveFrame` in `🔺️tessellation/🌀️revolved`) that orients `lateralScale`. Sections with and without rotation may be mixed (ring direction follows the first section's frame against the overall travel). Exponent n gives `|x/rx|^n + |y/ry|^n = 1`, caps are flat fans.
- Solid provenance (A5): every primitive that `tessellate` writes carries `solids: { id, first, count }[]`, index ranges into `indices`, contiguous from 0, in solid order, covering all indices (multiple of 3 for triangles, 2 for lines); a solid that produced no triangle has no range. `featureEdges` copies it onto the `lines` primitive of a source primitive that has it: an edge belongs to the solid of the first triangle that uses it (an edge shared by two solids is kept once, under the earlier solid), solids without edges get no range; a source without `solids` gives lines without `solids`. `validateModel` errors `bad-solids` for ranges that are not contiguous/ordered/whole/covering or have a non-slug id.
- Profile validation: extrude outlines and holes and revolve profiles must be simple polygons. `self-intersection` (error) = two non-adjacent edges touch or cross, adjacent edges fold back, or a closed ring has no area; `bad-holes` (error) = a hole touches or crosses the outline or another hole, lies outside the outline or inside another hole. A first point equal to the last closes the loop (extrude and revolve); a revolve profile is walked as the tessellator does (radial clamped at zero, end points projected onto the axis for open profiles) and contacts at radial 0 (touching the axis, the bicone) are allowed.
- Cap triangulation: ear clipping keeps collinear and near-duplicate vertices as vertices (no T-junctions, watertight with the side walls): an ear must be convex beyond a relative flatness tolerance (1e-14 of the squared extent) and no other vertex may lie in it or on its boundary within that tolerance; a second exact pass takes what the tolerant pass rejects. Verified on exactly the same output as before for the 3514 solids of the 122 catalog volumes.
- Joints (`poseModel`): `q = rest * q_joint`; revolute clamps twist about `axis` to `[min,max]` and drops swing (`joint-swing`); prismatic clamps along axis, drops the rest (`joint-swing`); spherical clamps twist to `[twistMin,twistMax]` and swing half-angle to `max`; fixed ignores the pose (`joint-fixed`); clamps report `joint-limit` (warning, path `/nodes/<i>/joint`). Animations wrap when `loop`, otherwise hold the last key; rotation interpolation is shortest-path slerp, `step` holds.
- Node-targeted channels apply in `poseModel`; material-targeted channels only through `sampleMaterials`.

## Files

Kernel: `M/🧬️schema/{🔣️.json,🟦️.ts}` (schema generated by `T/generate-model-schema.ts`), `M/🟦️.ts`, `M/🧮️math/🟦️.ts`, `M/✅️validation/🟦️.ts`, `M/🔺️tessellation/🟦️.ts` plus `🧱️builder`, `📏️segments`, `🔺️polygon`, `📦️box`, `🌀️revolved` (ellipsoid, capsule, revolve), `🏗️extrude`, `🧵️loft`, `M/🦴️mechanism/🟦️.ts`, `M/📏️edges/🟦️.ts`; `🔺️polygon` also exports `cleanRing`, `contact`, `ringFault`, `holesFault`, `🌀️revolved` also `revolveOutline` (used by the validator).
Wiring (existing 3d project): `🧊️3d/🧪️tests/🎚️config/🟦️.ts` (include globs for `🗿️model/🧪️tests/*`, `🗿️model/*/🧪️tests/*`, `🗿️model/*/*/🧪️tests/*`, excludes `🧰️support`), `🧊️3d/📦️packages/🟦️typescript/package.json` (`./model` export), `🧊️3d/📦️packages/🟦️typescript/🗿️model/🟦️.ts`.
Tests (`M/🧪️tests/<case>/🟦️.ts`): 📦️box, 🥚️ellipsoid, 💊️capsule, 🌀️revolve, 🏗️extrude, 🧵️loft, 🔁️repeat, 🦴️skinning, 🎞️pose-animation, ✅️validation, 📏️feature-edges, ⏱️operation, 🧬️provenance, 🔺️polygon; support `🧰️support/🟦️.ts`, `🧰️support/🧊️solids/🟦️.ts`.
Fixtures (language-agnostic JSON, `M/🧫️fixtures/<case>/🔣️.json`): same cases except ⏱️operation (generated by `T/generate-model-fixtures.py`, schema by `T/generate-model-schema.ts`).

## Test commands

- Authoritative: `repo run @semio-tech/framework-3d-js:test` (attempted once at the end, result below).
- Fast direct run used for iteration (from repo root): `bun node_modules/vitest/vitest.mjs run --config .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE/vitest.model.config.ts` (bun from `~/Library/Caches/semio/runtimes/bun/*/bun`).
- Typecheck: `bun node_modules/typescript/bin/tsc -p T/tsconfig.model.json`.
- Result of the direct run after the maintenance pass (`T/kernel-maintenance.md`): 14 test files, 319 tests, 319 passed, 0 failed. Typecheck of the whole kernel including encoders and viewer (`bun node_modules/typescript/bin/tsc -p T/tsconfig.kernel-maintenance.json`): no errors.
- Fixtures: `python3 T/generate-model-fixtures.py <dir>` writes all kernel fixtures (copy the changed ones into `M/🧫️fixtures/`), `bun T/generate-model-schema.ts <file>` the schema.

## Oracles (test-only, three 0.182 and ajv from the root node_modules)

| Feature | Oracle |
|---|---|
| All solids: volume, bounds | analytic formulas in the fixtures plus signed-volume and `Box3` measured with three `Vector3`/`Box3` |
| Watertight and consistent orientation | three `BufferGeometryUtils.mergeVertices` weld, then every directed edge used once with its reverse |
| Normals | three `Triangle.getNormal` against vertex normals, unit length |
| Schema twin | `ajv` validates procedural and resolved documents and each validation mutation against `🧬️schema/🔣️.json` |
| box | three `BoxGeometry`, `RoundedBoxGeometry` |
| ellipsoid | three `SphereGeometry` scaled |
| capsule | three `CapsuleGeometry` |
| revolve | three `LatheGeometry` surface area; Pappus/frustum volumes |
| extrude | three `ExtrudeGeometry` (volume), `ShapeUtils.area` (profile with holes) |
| loft | three `CylinderGeometry`; superellipse area via gamma function |
| repeat | connected components of the welded three geometry give copy centres |
| skinning | three `Skeleton` (inverse binds) and `SkinnedMesh.applyBoneTransform` against the hand-computed linear blend with kernel world matrices |
| pose/animation | three `QuaternionKeyframeTrack`/`VectorKeyframeTrack` interpolants (slerp, linear, discrete); three `Quaternion` for joint-limit twist/swing |
| feature edges | three `EdgesGeometry` segment counts |

## Deviations from design.md

- `tessellate` and `featureEdges` return a `Promise` (async); `validateModel` and `poseModel` are sync. `poseModel` returns an object (matrices plus local transforms plus issues), not a bare matrix list.
- Skin weights are uniform per solid (every vertex gets the solid's blend); the design gives only a per-solid weight list.
- Fields `kind` (solid discriminator) and helper `$defs` (Vec2/3/4, Mat4, TransformPatch, solid kind defs) were added; no listed `$defs` name changed.
- `featureEdges` has optional extra parameters (`{ material }`, `operation`).

## Open issues

- Skin weights per vertex (gradients along a limb) are not expressible yet; needs a design decision (for example weight ramps along the solid axis).
- Loft with very large exponent concentrates samples near the corners; normals at sharp superellipse corners are smooth-averaged.
- Extrude and revolve cap triangulation is ear clipping (O(n^2)) and validation is O(n^2) edge pairs; fine for authored profiles, not for thousands of points.
- Loft sections without rotation have a sign/axis flip where the path tangent passes parallel to X (the fallback to Z jumps the x axis); a path that turns through X-parallel needs explicit `rotation` on the sections around it.
- Direct vitest run and `tsc` are green; see below for the `repo run` attempt.

## `repo run` attempt

`repo run @semio-tech/framework-3d-js:test` did NOT complete: after about 750 s of reading the project graph the plan was cancelled with "the host has had only 65 MB of free memory (floor 256 MB) for 30 s". No test result came from it. The direct vitest run (205/205 passed) used a ticket-local config (`vitest.model.config.ts`) that targets the same test files; the project's own config (`🧊️3d/🧪️tests/🎚️config/🟦️.ts`) was edited but has not been exercised through `repo` and fails outside it with `Explicit Vitest policy required` when started with bare vitest.
