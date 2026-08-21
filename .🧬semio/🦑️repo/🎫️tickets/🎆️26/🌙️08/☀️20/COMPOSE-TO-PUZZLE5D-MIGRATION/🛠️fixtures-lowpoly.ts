#!/usr/bin/env bun
/** 💠️ Handcrafted mutation fixtures for the `lowpoly` artifact (17 leaves).
 *
 * `LowpolyDiff` is a REAL sparse delta: `objects: {added, removed, patched, reordered}`, with each
 * `patched` entry carrying an object-level `LowpolyObjectPatch` and, separately, an index-keyed
 * `paintLayers: {added, removed, patched, strokes}` sub-delta. Which of those four object-level
 * buckets and which of those four paint-layer buckets a leaf writes is transcribed below from that
 * leaf's own `🔺️diff/🦀️component.rs` and the `diff_objects_*` / `diff_*_paint_layer` constructor it
 * calls — never from the leaf's name.
 *
 * Pixel buffers are deliberately 8 bytes, not the 1024×1024 RGBA the runtime allocates: the
 * committed JSON has to stay readable and the stroke/run arithmetic is identical at any length.
 */
import { emitAll, f, type Case, type Tree } from "./🛠️emit-fixture.ts";

const tree: Tree = {
  mutationsRoot: "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
  glue: "✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/📦️glue.rs",
  gluePrefix: "../../🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
  diffPath: "crate::artifacts::lowpoly",
  diffName: "LowpolyDiff",
  snapshotPath: "crate::artifacts::lowpoly",
  snapshotName: "LowpolySnapshot",
  mutationsPath: "crate::artifacts::lowpoly",
  mutationName: "LowpolyMutation",
  entry: "kernel",
};

//#region 🔖️Vocabulary
const b64 = (bytes: readonly number[]) => Buffer.from(Uint8Array.from(bytes)).toString("base64");
const OPAQUE = [255, 255, 255, 255, 255, 255, 255, 255];
const RED_TAIL = [255, 255, 255, 255, 255, 0, 0, 255];
const CLEAR = [0, 0, 0, 0, 0, 0, 0, 0];

const transform = (position: unknown[], rotation: unknown[], scale: unknown[]) => ({ position, rotation, scale });
const rest = transform([f(0), f(0), f(0)], [f(0), f(0), f(0)], [f(1), f(1), f(1)]);

const meshChild = (childId: string, artifactId: string) => ({
  childId,
  target: { artifactId, dialect: { artifactKind: "s.stdio.semio", standard: "v1", subset: "mesh" } },
});
const hullMesh = meshChild("mesh-hull-01", "obj-hull-mesh");
const finMesh = meshChild("mesh-fin-01", "obj-fin-mesh");

const layer = (name: string, visible: boolean, opacity: number, blendMode: string, pixels: readonly number[]) => ({
  name,
  visible,
  opacity: f(opacity),
  blendMode,
  pixels: b64(pixels),
});
const baseLayer = layer("Base", true, 1, "normal", OPAQUE);
const detailLayer = layer("Detail", true, 1, "normal", CLEAR);

type ObjectOver = { name?: string; transform?: unknown; smoothShading?: boolean; mesh?: unknown; paintLayers?: unknown[] };
const hull = (over: ObjectOver = {}) => ({
  id: "obj-hull",
  name: over.name ?? "Hull",
  transform: over.transform ?? rest,
  smoothShading: over.smoothShading ?? false,
  mesh: over.mesh === undefined ? hullMesh : over.mesh,
  paintLayers: over.paintLayers ?? [baseLayer],
});
const fin = (over: ObjectOver = {}) => ({
  id: "obj-fin",
  name: over.name ?? "Fin",
  transform: over.transform ?? rest,
  smoothShading: over.smoothShading ?? true,
  mesh: over.mesh === undefined ? null : over.mesh,
  paintLayers: over.paintLayers ?? [],
});
const mast = {
  id: "obj-mast",
  name: "Mast",
  transform: rest,
  smoothShading: false,
  mesh: null,
  paintLayers: [],
};

const snapshot = (objects: unknown[]) => ({ schema: "lowpoly.document", objects });

type ObjectsDelta = { added?: unknown[]; removed?: string[]; patched?: unknown[]; reordered?: string[] | null };
const objects = (over: ObjectsDelta) => ({
  added: over.added ?? [],
  removed: over.removed ?? [],
  patched: over.patched ?? [],
  reordered: over.reordered ?? null,
});

/** 🩹 `LowpolyObjectPatch` has four always-serialized fields (no `skip_serializing_if`). */
type Patch = { name?: string | null; smoothShading?: boolean | null; transform?: unknown; mesh?: unknown };
const patch = (over: Patch = {}) => ({
  name: over.name ?? null,
  smoothShading: over.smoothShading ?? null,
  transform: over.transform ?? null,
  mesh: over.mesh ?? null,
});

type LayersDelta = { added?: unknown[]; removed?: number[]; patched?: unknown[]; strokes?: unknown[] };
const paintLayers = (over: LayersDelta) => ({
  added: over.added ?? [],
  removed: over.removed ?? [],
  patched: over.patched ?? [],
  strokes: over.strokes ?? [],
});

type LayerPatch = { name?: string | null; visible?: boolean | null; opacity?: unknown; blendMode?: string | null };
const layerPatch = (over: LayerPatch) => ({
  name: over.name ?? null,
  visible: over.visible ?? null,
  opacity: over.opacity ?? null,
  blendMode: over.blendMode ?? null,
});

const entry = (id: string, objectPatch: unknown, layers: unknown = null) => ({ id, patch: objectPatch, paintLayers: layers });

/** 🔺️ `LowpolyDiff` carries `default` on the container and no `skip_serializing_if` anywhere, so
 * every one of its 38 fields is emitted — `null` for the ones a leaf never touches. */
const delta = (objectsDelta: unknown | null) => ({
  artifact: null,
  schema: null,
  objects: objectsDelta,
  activeObjectId: null,
  selection: null,
  selectedObjectIds: null,
  paintUtility: null,
  activePaintLayer: null,
  activeUtilityId: null,
  showEdges: null,
  sunEnabled: null,
  sunAzimuth: null,
  sunElevation: null,
  sunIntensity: null,
  sunColor: null,
  worldCameraPositionX: null,
  worldCameraPositionY: null,
  worldCameraPositionZ: null,
  worldCameraTargetX: null,
  worldCameraTargetY: null,
  worldCameraTargetZ: null,
  worldCameraFov: null,
  utilityParamsJson: null,
  paintColorR: null,
  paintColorG: null,
  paintColorB: null,
  paintColorA: null,
  selectionMethod: null,
  selectionModeDefault: null,
  engagementInput: null,
  locale: null,
  hoveredObjectId: null,
  hoveredTargetObjectId: null,
  hoveredTargetMode: null,
  hoveredTargetId: null,
  strokeDragActive: null,
  transformDragActive: null,
  previewSeq: null,
});

const applied = { status: "applied" };
const twoObjects = snapshot([hull(), fin()]);
//#endregion 🔖️Vocabulary

//#region 🔖️ObjectLane
const objectCases: readonly Case[] = [
  {
    leafDir: "🌱️create-object",
    leafSlug: "create-object",
    caseName: "inserts-obj-mast-between-hull-and-fin",
    headline: "`create-object` routes through `diff_objects_add`, which does NOT carry an insertion index on the added entry: it appends to `added` and additionally publishes a FULL `reordered` id permutation that places the new object at the payload index.",
    before: twoObjects,
    after: snapshot([hull(), mast, fin()]),
    mutation: { CreateObject: { index: 1, object: mast } },
    diff: delta(objects({ added: [mast], reordered: ["obj-hull", "obj-mast", "obj-fin"] })),
    outcome: applied,
  },
  {
    leafDir: "💀️delete-object",
    leafSlug: "delete-object",
    caseName: "removes-obj-fin-without-touching-the-order",
    headline: "`delete-object` routes through `diff_objects_remove`, the one objects-delta constructor that leaves `reordered` as `None` — removal alone re-derives the order, so no permutation is published.",
    before: twoObjects,
    after: snapshot([hull()]),
    mutation: { DeleteObject: { id: "obj-fin" } },
    diff: delta(objects({ removed: ["obj-fin"] })),
    outcome: applied,
  },
  {
    leafDir: "🔀️reorder-objects",
    leafSlug: "reorder-objects",
    caseName: "moves-obj-fin-in-front-of-obj-hull",
    headline: "`reorder-objects` writes ONLY `reordered`, and its diff builder pre-computes the permutation to no-op when the clamped move would leave the order unchanged; nothing is added, removed or patched.",
    before: twoObjects,
    after: snapshot([fin(), hull()]),
    mutation: { ReorderObjects: { id: "obj-fin", toIndex: 0 } },
    diff: delta(objects({ reordered: ["obj-fin", "obj-hull"] })),
    outcome: applied,
  },
  {
    leafDir: "🏷️rename-object",
    leafSlug: "rename-object",
    caseName: "retitles-obj-hull",
    headline: "`rename-object` writes ONE `patched` entry whose `LowpolyObjectPatch` sets `name` and leaves `smoothShading`/`transform`/`mesh` null, and whose `paintLayers` sub-delta is absent entirely.",
    before: twoObjects,
    after: snapshot([hull({ name: "Hull, revised" }), fin()]),
    mutation: { RenameObject: { id: "obj-hull", newName: "Hull, revised" } },
    diff: delta(objects({ patched: [entry("obj-hull", patch({ name: "Hull, revised" }))] })),
    outcome: applied,
  },
  {
    leafDir: "🔘️change-object-smooth-shading",
    leafSlug: "change-object-smooth-shading",
    caseName: "turns-on-smooth-shading-for-obj-hull",
    headline: "`change-object-smooth-shading` no-ops on an unchanged flag; otherwise it patches the single `smoothShading` boolean — the object's transform, mesh handle and paint layers are all left null in the patch.",
    before: twoObjects,
    after: snapshot([hull({ smoothShading: true }), fin()]),
    mutation: { ChangeObjectSmoothShading: { id: "obj-hull", newSmoothShading: true } },
    diff: delta(objects({ patched: [entry("obj-hull", patch({ smoothShading: true }))] })),
    outcome: applied,
  },
  {
    leafDir: "↗️move-object",
    leafSlug: "move-object",
    caseName: "translates-obj-hull-along-x-and-z",
    headline: "`move-object` rejects non-finite coordinates and no-ops on an unchanged position; the patch it writes is a WHOLE `LowpolyTransform` rebuilt from the base's own rotation and scale with only `position` replaced.",
    before: twoObjects,
    after: snapshot([hull({ transform: transform([f(2.5), f(0), f(-1.5)], [f(0), f(0), f(0)], [f(1), f(1), f(1)]) }), fin()]),
    mutation: { MoveObject: { id: "obj-hull", newPosition: [f(2.5), f(0), f(-1.5)] } },
    diff: delta(
      objects({
        patched: [entry("obj-hull", patch({ transform: transform([f(2.5), f(0), f(-1.5)], [f(0), f(0), f(0)], [f(1), f(1), f(1)]) }))],
      }),
    ),
    outcome: applied,
  },
  {
    leafDir: "🔄️rotate-object",
    leafSlug: "rotate-object",
    caseName: "yaws-obj-hull-about-the-y-axis",
    headline: "`rotate-object` guards only on finiteness (no range clamp) and republishes the whole transform with the base's own position and scale preserved and `rotation` replaced.",
    before: twoObjects,
    after: snapshot([hull({ transform: transform([f(0), f(0), f(0)], [f(0), f(1.5), f(0)], [f(1), f(1), f(1)]) }), fin()]),
    mutation: { RotateObject: { id: "obj-hull", newRotation: [f(0), f(1.5), f(0)] } },
    diff: delta(
      objects({
        patched: [entry("obj-hull", patch({ transform: transform([f(0), f(0), f(0)], [f(0), f(1.5), f(0)], [f(1), f(1), f(1)]) }))],
      }),
    ),
    outcome: applied,
  },
  {
    leafDir: "📐️scale-object",
    leafSlug: "scale-object",
    caseName: "halves-obj-hull-uniformly",
    headline: "`scale-object` carries a stricter guard than its move/rotate siblings — a component that is non-finite OR `<= 0.0` is fatal — and then republishes the whole transform with only `scale` replaced.",
    before: twoObjects,
    after: snapshot([hull({ transform: transform([f(0), f(0), f(0)], [f(0), f(0), f(0)], [f(0.5), f(0.5), f(0.5)]) }), fin()]),
    mutation: { ScaleObject: { id: "obj-hull", newScale: [f(0.5), f(0.5), f(0.5)] } },
    diff: delta(
      objects({
        patched: [entry("obj-hull", patch({ transform: transform([f(0), f(0), f(0)], [f(0), f(0), f(0)], [f(0.5), f(0.5), f(0.5)]) }))],
      }),
    ),
    outcome: applied,
  },
];
//#endregion 🔖️ObjectLane

//#region 🔖️MeshLane
const meshCases: readonly Case[] = [
  {
    leafDir: "🕸️create-mesh",
    leafSlug: "create-mesh",
    caseName: "attaches-a-mesh-child-handle-to-obj-fin",
    headline: "`create-mesh` writes ONLY the two-string child HANDLE into the patch's double-`Option` `mesh` slot — its `meshWorkspace` payload field carries the live geometry and is deliberately absent from the diff, because the parent snapshot never stores mesh content.",
    before: twoObjects,
    after: snapshot([hull(), fin({ mesh: finMesh })]),
    mutation: {
      CreateMesh: {
        id: "obj-fin",
        childId: "mesh-fin-01",
        target: { artifactId: "obj-fin-mesh", dialect: { artifactKind: "s.stdio.semio", standard: "v1", subset: "mesh" } },
        meshWorkspace: "{\"vertices\":[],\"faces\":[]}",
      },
    },
    diff: delta(objects({ patched: [entry("obj-fin", patch({ mesh: finMesh }))] })),
    outcome: applied,
  },
  {
    leafDir: "🧨delete-mesh",
    leafSlug: "delete-mesh",
    caseName: "detaches-the-mesh-child-handle-from-obj-hull",
    headline: "`delete-mesh` no-ops when the object already has no mesh; otherwise it sets the patch's `mesh` slot to `Some(None)` — outer present, inner cleared — which serialises to the same bare `null` a `None` does (see this ticket's census note on the missing `double_option`).",
    before: twoObjects,
    after: snapshot([hull({ mesh: null }), fin()]),
    mutation: { DeleteMesh: { id: "obj-hull" } },
    diff: delta(objects({ patched: [entry("obj-hull", patch({}))] })),
    outcome: applied,
  },
];
//#endregion 🔖️MeshLane

//#region 🔖️PaintLane
const painted = (layers: unknown[]) => snapshot([hull({ paintLayers: layers }), fin()]);

const paintCases: readonly Case[] = [
  {
    leafDir: "➕️insert-paint-layer",
    leafSlug: "insert-paint-layer",
    caseName: "stacks-a-detail-layer-above-the-base-layer",
    headline: "`insert-paint-layer` clamps the requested index to the layer count and raises `mutation.clamped` when it had to; at index 1 of a one-layer stack no clamping happens, and the diff carries a single `paintLayers.added` entry with an object-level patch that touches nothing.",
    before: twoObjects,
    after: painted([baseLayer, detailLayer]),
    mutation: { InsertPaintLayer: { objectId: "obj-hull", index: 1, layer: detailLayer } },
    diff: delta(objects({ patched: [entry("obj-hull", patch({}), paintLayers({ added: [{ index: 1, layer: detailLayer }] }))] })),
    outcome: applied,
  },
  {
    leafDir: "➖️remove-paint-layer",
    leafSlug: "remove-paint-layer",
    caseName: "drops-the-detail-layer-at-index-1",
    headline: "`remove-paint-layer` rejects an out-of-range index outright (no clamping, unlike its insert sibling) and writes the bare layer INDEX into `paintLayers.removed` — paint layers are positional, not id-keyed.",
    before: painted([baseLayer, detailLayer]),
    after: painted([baseLayer]),
    mutation: { RemovePaintLayer: { objectId: "obj-hull", index: 1 } },
    diff: delta(objects({ patched: [entry("obj-hull", patch({}), paintLayers({ removed: [1] }))] })),
    outcome: applied,
  },
  {
    leafDir: "🔖️rename-paint-layer",
    leafSlug: "rename-paint-layer",
    caseName: "retitles-the-base-layer-to-undercoat",
    headline: "`rename-paint-layer` writes an indexed `LowpolyPaintLayerPatch` with only `name` set — the layer's visibility, opacity, blend mode and pixel buffer are all untouched, and the object-level patch stays entirely null.",
    before: twoObjects,
    after: painted([layer("Undercoat", true, 1, "normal", OPAQUE)]),
    mutation: { RenamePaintLayer: { objectId: "obj-hull", index: 0, newName: "Undercoat" } },
    diff: delta(objects({ patched: [entry("obj-hull", patch({}), paintLayers({ patched: [{ index: 0, patch: layerPatch({ name: "Undercoat" }) }] }))] })),
    outcome: applied,
  },
  {
    leafDir: "👁️change-paint-layer-visible",
    leafSlug: "change-paint-layer-visible",
    caseName: "hides-the-base-layer",
    headline: "`change-paint-layer-visible` no-ops when the flag already matches; otherwise it writes an indexed layer patch with only `visible` set — hiding a layer never touches its pixels.",
    before: twoObjects,
    after: painted([layer("Base", false, 1, "normal", OPAQUE)]),
    mutation: { ChangePaintLayerVisible: { objectId: "obj-hull", index: 0, newVisible: false } },
    diff: delta(objects({ patched: [entry("obj-hull", patch({}), paintLayers({ patched: [{ index: 0, patch: layerPatch({ visible: false }) }] }))] })),
    outcome: applied,
  },
  {
    leafDir: "🌫️change-paint-layer-opacity",
    leafSlug: "change-paint-layer-opacity",
    caseName: "fades-the-base-layer-to-half",
    headline: "`change-paint-layer-opacity` compares the stored `f32` for exact equality before writing, and its indexed layer patch sets only `opacity` — the layer stays visible and keeps its blend mode.",
    before: twoObjects,
    after: painted([layer("Base", true, 0.5, "normal", OPAQUE)]),
    mutation: { ChangePaintLayerOpacity: { objectId: "obj-hull", index: 0, newOpacity: f(0.5) } },
    diff: delta(objects({ patched: [entry("obj-hull", patch({}), paintLayers({ patched: [{ index: 0, patch: layerPatch({ opacity: f(0.5) }) }] }))] })),
    outcome: applied,
  },
  {
    leafDir: "🎛️change-paint-layer-blend-mode",
    leafSlug: "change-paint-layer-blend-mode",
    caseName: "switches-the-base-layer-to-multiply",
    headline: "`change-paint-layer-blend-mode` treats the blend mode as an opaque string (no vocabulary check) and writes an indexed layer patch with only `blendMode` set.",
    before: twoObjects,
    after: painted([layer("Base", true, 1, "multiply", OPAQUE)]),
    mutation: { ChangePaintLayerBlendMode: { objectId: "obj-hull", index: 0, newBlendMode: "multiply" } },
    diff: delta(objects({ patched: [entry("obj-hull", patch({}), paintLayers({ patched: [{ index: 0, patch: layerPatch({ blendMode: "multiply" }) }] }))] })),
    outcome: applied,
  },
  {
    leafDir: "🎨️edit-paint-layer",
    leafSlug: "edit-paint-layer",
    caseName: "paints-red-over-the-second-half-of-the-base-layer",
    headline: "`edit-paint-layer` no-ops on an empty run list; otherwise it writes `paintLayers.strokes` — the only bucket that carries raw pixel RUNS rather than metadata — and the runs overwrite bytes in place, so the buffer length never changes.",
    before: twoObjects,
    after: painted([layer("Base", true, 1, "normal", RED_TAIL)]),
    mutation: { EditPaintLayer: { objectId: "obj-hull", layerIndex: 0, runs: [{ offset: 4, bytes: b64([255, 0, 0, 255]) }] } },
    diff: delta(
      objects({
        patched: [entry("obj-hull", patch({}), paintLayers({ strokes: [{ layerIndex: 0, runs: [{ offset: 4, bytes: b64([255, 0, 0, 255]) }] }] }))],
      }),
    ),
    outcome: applied,
  },
];
//#endregion 🔖️PaintLane

emitAll(tree, [...objectCases, ...meshCases, ...paintCases]);
