# Kernel scenes (`semio.model/v1`) — report

Signatures: `kernel-api.md`. Code: M = `🧰️framework/🔨️modules/🧊️3d/🗿️model`. Scripts: `kernel-fixture.ts` (writes the vector), `kernel-test.sh bun|node NAME [filter]`.

## Model decision (justification)

A scene is a named set of mesh-carrying **nodes**; solids belong to a scene only through their node. A bone that needs different solids per scene gets one identity child node per scene (`hull-body`, `frame-body`, ... under `spine`).
- A solid-level `scene` field would make tessellation emit several meshes on one node, which neither glTF (one mesh per node) nor FBX can hold, and it would duplicate the resolved/procedural split. Node membership is the same rule in procedural and resolved documents.
- It reuses what every consumer already has (node ids, tags, parents), keeps `Solid` unchanged, and sharing is explicit: a node listed in several scenes shares its solids, its mesh and (GLB) its accessors.
- Rule: no `scenes` (or `[]`) = one implicit scene of every node that carries a solid or a mesh (`scenesOf` returns it with `id = document.id`).

## What changed

- Schema + twin: `Scene` def (32 `$defs`), optional `scenes`. Validation: `unknown-node`, `duplicate-id` (scene ids, repeated node), `unscened-node`, `empty-scene`, `bad-slug`.
- `🎞️scenes`: `scenesOf`, `selectScene` (single-scene, self-contained cut: nodes + ancestors + weight/skin joints, own solids/meshes/skins, clip channels, poses), `nodesOfScene`, `solidsOfScene` (provenance), `materialsOfScene`, `UnknownSceneError`. `📐️bounds`: `boundsOf` (rigid + linear-blend skinning, rest/pose/clip).
- `tessellate(.., { scene })`; weighted solids are grouped by scene set: one skinned node + mesh per distinct set (`skinned`, or `skinned-<scene>-<scene>`), one shared skin; the node joins each of its scenes. `poseModel`, `sampleMaterials`, `featureEdges` take the scene.
- GLB: one glTF scene per model scene, default first, scene extras = title/tags/nodes. glTF has one parent per node, so the nodes a scene needs are written once per scene (same name); meshes, accessors, materials, variants, skin IBM accessors, animation samplers are shared; channels repeat per node copy. `scene` option writes one scene with only what it needs. A document without scenes encodes byte-identically to before (all pre-existing digests unchanged).
- FBX: option `scene` (absent: first + notice `scene-defaulted`; unknown: `unknown-scene`).
- Viewer: `Scene.scenes`, `ScenePrimitive.scenes`, `FrameInput.scene`/`Frame.sceneIndex` (draw list filter; buffers untouched), `boundsOf`, `pick`, `followPoint`, software rasteriser, `ModelViewer.setScene/scenes/activeScene`, event + status `scene`, element attribute `scene`.
- README updated.

## Tests (all green: 40 files, 673 tests, under bun and under node 24; `repo` not used, direct vitest)

- Vector `🧫️fixtures/🎞️scenes` (language-agnostic): document with 3 scenes over one skeleton (rigid per-scene nodes on the same bones, a skinned solid shared by two scenes, a thermal-only skinned solid), invalid cases with issue paths, hand-derived node closures, solid ids, bounds (rest and bent), digests.
- `🧪️tests/🎞️scenes`: schema (Ajv), validation rules, implicit scene, cut, tessellation, pose, sampling, bounds, edges, progress/cancellation, and a guard that no source outside portable math and tests calls an engine-dependent `Math` function.
- GLB `🧪️tests/🎞️scenes`: Khronos gltf-validator (0 errors, 0 warnings; all scenes, each alone, baked pose and clip time), glTF-Transform (scene count, default, per-scene node sets, sharing of meshes/IBM/samplers, repeated channels), three GLTFLoader (per-scene node and mesh sets, no shared objects, `Box3` precise bounds with skinning at rest and bent against the vector), cut-equals-select, digests.
- FBX `🧪️tests/🎞️scenes`: assimp (node tree, world matrices, triangle multiset, bones, clip), three FBXLoader (meshes, triangle count), notice, rejection, digests.
- Viewer `🧪️tests/🎞️scenes`, `🧪️tests/🎞️scene-controller`: shared buffers, draw lists, bounds vs vector and kernel, picking, follow, software raster, controller/element.
- Byte identity: digests recorded under bun reproduce under node (GLB all/per scene/posed, FBX per scene). Everything pinned already goes through `🧮️math/🎯️portable`; no new transcendental calls.

## Notes for consumers (`modelOf`/exports)

- Build one procedural document with all scenes (`s50-body`, `s50-thermal`, ...), tessellate once, `encodeGlb` once (all scenes), `encodeFbx({scene})` per scale/view.
- Per-scene geometry on a shared bone: add identity child nodes per scene; list shared nodes in several scenes to avoid duplicates.
- Not done: viewer UI chrome for the scene switch (the controller API exists); `repo run` target not executed (whole-suite runs belong to the health agent).
