# Kernel scenes: final API (stable)

M = `🧰️framework/🔨️modules/🧊️3d/🗿️model`, package export `@semio-tech/framework-3d-js/model` (viewer: `…/model/viewer`).

## Model decision

Scene membership belongs to the node, never to the solid. A scene is a named set of mesh-carrying nodes. A bone node that must carry different solids per scene gets one identity child node per scene (`scene-specific mesh nodes parented to the bone`), each with its own solids, listed in its scene only. A node listed in several scenes shares its solids (and in GLB its mesh) across them.

## Schema (`semio.model/v1`)

- `ModelDocument.scenes?: readonly Scene[]`; `Scene = { id: Slug; title: Text; nodes: readonly Slug[]; tags?: readonly Slug[] }`.
- Rule: a document without `scenes` (or with `[]`) behaves as one implicit scene holding every node that carries a solid or a mesh.
- Validation ids (errors): `unknown-node` (`/scenes/i/nodes/k`), `duplicate-id` (`/scenes/i/id`, also a repeated node inside one scene), `unscened-node` (node with a solid or mesh in no scene while scenes exist), `empty-scene` (no nodes). `bad-slug`, `bad-type` as elsewhere.

## Functions (all exported from `./🟦️.ts`)

```ts
type SceneOption = { readonly scene?: Slug };            // omitted: whole document (all scenes)
scenesOf(document): readonly Scene[]                      // explicit scenes, or the implicit one { id: document.id, title: document.title, nodes: all carriers }
selectScene(document, sceneId): ModelDocument             // single-scene document: nodes (scene nodes + ancestors + weight and skin joints + their ancestors), their solids, meshes, skins, poses, animation channels, scenes = [that scene]; throws UnknownSceneError
nodesOfScene(document, sceneId): ReadonlySet<Slug>        // the closure above
solidsOfScene(document, sceneId): readonly Slug[]         // provenance: solid ids of the scene (procedural solids, or Primitive.solids ranges of its meshes)
boundsOf(document, selector?, option?): { min, max } | null   // posed bounds of a resolved document (rigid and skinned), optional scene
tessellate(document, quality, operation, option?: SceneOption): Promise<ModelDocument>   // with scene: tessellates the selection only; skinned solids are grouped per scene set: one skinned mesh+node per distinct set (id `skinned`, or `skinned-<scene>-<scene>` when several groups) listed in each of its scenes
poseModel(document, selector?, option?: SceneOption): PoseResult
sampleMaterials(document, animationId, timeS, option?: SceneOption): Record<Slug, …>    // only materials used by the scene
featureEdges(document, angleRad, options: { material?, scene? }, operation): Promise<ModelDocument>   // with scene: the selection only
encodeGlb(document, options: GlbOptions & { scene?: Slug }, operation): Uint8Array
encodeFbx / encodeFbxDetailed(document, options: FbxOptions & { scene?: Slug }, …)
class UnknownSceneError extends Error
```

`GlbOptions.scene` absent: one glTF scene per model scene in document order, `scene: 0`; a node used by several glTF scenes is written once per scene (glTF has one parent per node) and shares meshes, accessors, materials, variants, skin inverse-bind accessors and animation samplers. A single-scene document encodes exactly as before. `FbxOptions.scene` absent: all nodes (documents with several scenes: the first scene).

Additional exports: `materialsOfScene(document, sceneId): ReadonlySet<Slug>` (materials a scene draws incl. variant targets), `sceneIdsOfNodes(document)`, `ModelBounds`. `encodeGlb` with `scene` writes only the meshes, skins and clip channels that scene needs. `FbxOptions.scene` absent: first scene plus notice `scene-defaulted`; unknown: `FbxEncodeError` code `unknown-scene`.

## Viewer (`…/model/viewer`)

```ts
Scene.scenes: readonly { id, title, nodes: ReadonlySet<number> }[]     // sceneOf builds one set of shared buffers for all scenes
FrameInput.scene?: Slug            // active scene, default the first; draw list filtered, buffers untouched; Frame.sceneIndex records it
ScenePrimitive.scenes: readonly number[]
boundsOf(scene, frame)             // bounds of the frame's active scene; pick and followPoint honour it too
ModelViewer: options.scene, scenes, activeScene, setScene(id | null), event `scene`, status kind `scene`; element attribute `scene`
```

Stable from now on; additions only.
