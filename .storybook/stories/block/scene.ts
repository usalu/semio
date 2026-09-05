/// <reference types="vite/client" />
// #region 🧲️Header
// 💻️ .storybook/stories/block/scene.ts
// Specs: The `🧱️block` scope's shared story fixtures, scene-node projections, window-render text and
// story-local plugin-command emulators — everything the `stories/block/**/*.stories.tsx` files need that is
// NOT itself a story.
// Summary: Lives beside the story files rather than inside them because Storybook's CSF indexer treats EVERY
// named export of a `*.stories.*` module as a story, so a shared helper exported from `2d/Board.stories.tsx`
// would be indexed as a broken story. The real shipped example documents (`🗣️.dsl.semio`) are `?raw`-imported
// here and parsed by `./dsl.ts`; the two GLB meshes their `representations` reference are `?url`-imported and
// keyed by the PUBLIC mesh identity the document carries, resolved through the framework mesh catalog
// (`resolveMeshAsset`, `🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🟦️.ts`) — the same authority `World3dHost` funnels
// every mesh url through (`meshAssetTransportUrl`). The `/mesh` transport route those transport urls point at
// is a `mesh-collection` playground asset, and a GENERATED Storybook scope cannot declare `assets`
// (`.storybook/scopes.ts`'s `buildGeneratedScopes` emits id/titlePrefix/sourceRoots/storyGlobs only), so the
// stories hand `World3dHost` the Vite-emitted asset url instead — `meshAssetTransportUrl` passes any non
// `/mesh/` url through untouched, so the host loads the real GLB rather than 404ing.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { UiComponentSceneNode } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";
import { resolveMeshAsset } from "../../../🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🟦️";

import { parseBlock2dDsl, parseBlock3dDsl, parseBlock5dDsl, type Block2dSnapshot, type Block3dSnapshot, type Block5dSnapshot } from "./dsl";

import block2dLeftDsl from "../../../✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🧪️hexagonal-cut-concrete-forest-left/🗣️.dsl.semio?raw";
import block2dRightDsl from "../../../✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/➡️hexagonal-cut-concrete-forest-right/🖼️assets/🧪️hexagonal-cut-concrete-forest-right/🗣️.dsl.semio?raw";
import block3dLeftDsl from "../../../✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🧪️hexagonal-cut-concrete-forest-left/🗣️.dsl.semio?raw";
import block3dNakaginDsl from "../../../✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️nakagin-capsule/🖼️assets/🧪️nakagin-capsule/🗣️.dsl.semio?raw";
import block5dLeftDsl from "../../../✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🧪️hexagonal-cut-concrete-forest-left/🗣️.dsl.semio?raw";
import block5dNakaginDsl from "../../../✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️nakagin-capsule/🖼️assets/🧪️nakagin-capsule/🗣️.dsl.semio?raw";

import hexagonalCutLeftGlbUrl from "../../../♻️mit-bestand/🖼️asset/🏚️abbau-aufbau/◀️hexagonal-cut-concrete-forest-left.glb?url";
import capsuleJGlbUrl from "../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/💊️capsules/🪝️j/🧊️capsule_J.glb?url";

//#region 🔖️ActionArgs
/** @emoji 📨️ `ActionDescriptor.args` is declared `unknown` (`🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest.ts`), so every story reducer narrows it here once instead of casting at each read. A non-object payload becomes an empty bag, exactly as a Rust handler sees no named arguments. */
export function blockStoryActionArgs(args: unknown): Record<string, unknown> {
  return typeof args === "object" && args !== null && !Array.isArray(args) ? (args as Record<string, unknown>) : {};
}
//#endregion 🔖️ActionArgs

//#region 🔖️MeshResolution
/**
 * @emoji 🧊️ Public mesh identity (the `mesh-url` a `representations` row carries) → the Vite-emitted url of the
 * GLB the framework mesh catalog names as that identity's `source`. Built by asking `resolveMeshAsset` for each
 * identity so a catalog rename breaks this map loudly at module load instead of silently serving the wrong mesh
 * — mirroring `../puzzle/3d/World.stories.tsx`'s `STORY_REFERENCE_URL_OVERRIDES` for reference-plane images.
 */
const BLOCK_STORY_MESH_SOURCES: Readonly<Record<string, string>> = {
  "♻️mit-bestand/🖼️asset/🏚️abbau-aufbau/◀️hexagonal-cut-concrete-forest-left.glb": hexagonalCutLeftGlbUrl,
  "🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/💊️capsules/🪝️j/🧊️capsule_J.glb": capsuleJGlbUrl,
};

/** @emoji 🧊️ One representation's mesh url, resolved the way `World3dHost` does (`meshAssetTransportUrl` → `resolveMeshAsset`). `null` for an identity absent from the catalog (`/mesh/capsule_J.1to500.glb`, the nakagin `1:500` representation — no catalog anywhere names it) or present but without a bundled story asset, so callers can drop the representation and SAY so instead of rendering a broken mesh. */
export function resolveBlockStoryMeshUrl(meshUrl: string | undefined): string | null {
  if (meshUrl === undefined || meshUrl === "") return null;
  if (!meshUrl.startsWith("/mesh/")) return meshUrl;
  try {
    return BLOCK_STORY_MESH_SOURCES[resolveMeshAsset(meshUrl).source] ?? null;
  } catch {
    return null;
  }
}
//#endregion 🔖️MeshResolution

//#region 🔖️Block2d
/** @emoji 🎬️ The two `📚️examples/*` units the block2d subset registers (`◻️2d/…/✳️any/🦀️.rs`'s `examples()`) — the exact ids `setActiveExample` switches between. */
export const BLOCK2D_STORY_EXAMPLES: Readonly<Record<string, string>> = {
  "hexagonal-cut-concrete-forest-left": block2dLeftDsl,
  "hexagonal-cut-concrete-forest-right": block2dRightDsl,
};

export const BLOCK2D_STORY_EXAMPLE_IDS: readonly string[] = Object.keys(BLOCK2D_STORY_EXAMPLES);

export type Block2dStoryState = { readonly exampleId: string; readonly snapshot: Block2dSnapshot };

export function block2dStoryStateFor(exampleId: string): Block2dStoryState {
  const dslText = BLOCK2D_STORY_EXAMPLES[exampleId];
  if (dslText === undefined) throw new Error(`[block2d-story] unknown example ${JSON.stringify(exampleId)}`);
  return { exampleId, snapshot: parseBlock2dDsl(dslText) };
}

/** @emoji 🎲️ Mints an id avoiding every id already present — mirrors the collision re-mint in `✏️editor/🎮️commands/🌱️add-handle` / `🔘️add-handle-kind` without the real per-session serial counter. */
function nextBlockId(prefix: string, existing: ReadonlySet<string>): string {
  let index = existing.size;
  let candidate = `${prefix}${index}`;
  while (existing.has(candidate)) {
    index += 1;
    candidate = `${prefix}${index}`;
  }
  return candidate;
}

/** @emoji 🧩️ Story-local mirror of block2d's `command_from_action` → `Block2dCommand::dispatch` path (`◻️2d/…/✏️editor/🦀️.rs`) for the six document-shaping actions the 2D stories exercise. An unknown action is ignored, exactly as the real `command_from_action` returns `None`. */
export function reduceBlock2dStoryAction(state: Block2dStoryState, action: string, rawArgs: unknown): Block2dStoryState {
  const args = blockStoryActionArgs(rawArgs);
  const { snapshot } = state;
  switch (action) {
    case "setActiveExample": {
      const exampleId = args.exampleId;
      return typeof exampleId === "string" && BLOCK2D_STORY_EXAMPLE_IDS.includes(exampleId) ? block2dStoryStateFor(exampleId) : state;
    }
    case "patchNodeKind": {
      const field = typeof args.field === "string" ? args.field : undefined;
      const value = typeof args.value === "string" ? args.value : undefined;
      if (field === undefined || value === undefined || !(field in snapshot.nodeKind)) return state;
      return { ...state, snapshot: { ...snapshot, nodeKind: { ...snapshot.nodeKind, [field]: value } } };
    }
    case "addHandleKind": {
      const ids = new Set(snapshot.handleKinds.map((kind) => kind.id));
      const id = typeof args.id === "string" && !ids.has(args.id) ? args.id : nextBlockId("k", ids);
      return {
        ...state,
        snapshot: {
          ...snapshot,
          handleKinds: [
            ...snapshot.handleKinds,
            { id, name: id, label: typeof args.label === "string" ? args.label : id, color: typeof args.color === "string" ? args.color : "hsl(0 52% 48%)", defaultWireKind: typeof args.defaultWireKind === "string" ? args.defaultWireKind : "cable.link" },
          ],
        },
      };
    }
    case "removeHandleKind": {
      const id = typeof args.id === "string" ? args.id : snapshot.handleKinds.at(-1)?.id;
      if (id === undefined) return state;
      return { ...state, snapshot: { ...snapshot, handleKinds: snapshot.handleKinds.filter((kind) => kind.id !== id), handles: snapshot.handles.filter((handle) => handle.handleKind !== id) } };
    }
    case "addHandle": {
      const ids = new Set(snapshot.handles.map((handle) => handle.id));
      const id = typeof args.id === "string" && !ids.has(args.id) ? args.id : nextBlockId("h", ids);
      return {
        ...state,
        snapshot: {
          ...snapshot,
          handles: [
            ...snapshot.handles,
            {
              id,
              handleKind: typeof args.handleKind === "string" ? args.handleKind : (snapshot.handleKinds[0]?.id ?? ""),
              angle: typeof args.angle === "number" ? args.angle : ((snapshot.handles.length + 1) * Math.PI) / 6,
              radius: typeof args.radius === "number" ? args.radius : (snapshot.handles[0]?.radius ?? 0.36),
            },
          ],
        },
      };
    }
    case "removeHandle": {
      const id = typeof args.id === "string" ? args.id : snapshot.handles.at(-1)?.id;
      if (id === undefined) return state;
      return { ...state, snapshot: { ...snapshot, handles: snapshot.handles.filter((handle) => handle.id !== id) } };
    }
    default:
      return state;
  }
}

/** @emoji 📋️ The exact `ui_text` lines block2d's viewer board window emits (`◻️2d/…/👁️viewer/🎭️modes/👁️view/🪟️windows/📋️board/🦀️.rs`'s `render`). The editor's own board window (`✏️editor/…/🪟️windows/📋️board`) prints the first line plus one `"<n> handle kinds, <m> handles"` count line, so this is a strict superset of both. */
export function block2dBoardRenderLines(snapshot: Block2dSnapshot): readonly string[] {
  const lines = [`Node kind: ${snapshot.nodeKind.label === "" ? "—" : snapshot.nodeKind.label}`, `${snapshot.handleKinds.length} handle kind(s)`];
  for (const kind of snapshot.handleKinds) lines.push(`  ◦ ${kind.label} (${kind.id}) — ${kind.color}`);
  lines.push(`${snapshot.handles.length} handle(s)`);
  for (const handle of snapshot.handles) lines.push(`  ◦ ${handle.id} — kind ${handle.handleKind}, angle ${((handle.angle * 180) / Math.PI).toFixed(1)}°, radius ${handle.radius.toFixed(2)}`);
  return lines;
}

/** @emoji 🖼️ Board-space radius of the single node-kind glyph. A handle template's own `radius` is a NORMALIZED rim offset (0.36 in both examples), never a board-space size, so it scales the handle's placement off this radius instead of being used raw. */
export const BLOCK2D_STORY_NODE_RADIUS = 120;

/** @emoji 🗂️ Document-shaped kind catalogs → the engine-shaped `nodeKinds`/`handleKinds` the board reads — the same translation `../puzzle/2d/Board.stories.tsx`'s `storyBoardKindCatalogsJson` performs (the engine rejects a row still carrying the document's `label`). */
export function block2dGlyphCatalogsJson(snapshot: Block2dSnapshot): string {
  return JSON.stringify({
    handleKinds: snapshot.handleKinds.map((kind) => ({ id: kind.id, name: kind.name, color: kind.color, defaultWireKind: kind.defaultWireKind })),
    nodeKinds: [{ id: snapshot.nodeKind.id, name: snapshot.nodeKind.name, shape: "circle", handles: snapshot.handles.map((handle) => ({ handleKind: handle.handleKind, angle: handle.angle, radius: handle.radius })) }],
  });
}

/** @emoji 📋️ One board node (the node kind itself) carrying one board handle per `Block2dHandleTemplate`, at that template's own angle. */
export function buildBlock2dSceneNode(snapshot: Block2dSnapshot, interactive: boolean, surfaceId = "block2d.play.board", controllerId = "block2d-story"): UiComponentSceneNode {
  const fixture = {
    schema: "block.2d.fixture",
    camera: snapshot.camera2d,
    nodes: [
      {
        id: snapshot.nodeKind.id,
        nodeKind: snapshot.nodeKind.id,
        shape: "circle",
        x: 0,
        y: 0,
        radius: BLOCK2D_STORY_NODE_RADIUS,
        text: snapshot.nodeKind.label,
        handles: snapshot.handles.map((handle) => ({ id: handle.id, handleKind: handle.handleKind, angle: handle.angle, radius: handle.radius * BLOCK2D_STORY_NODE_RADIUS })),
      },
    ],
    edges: [],
  };
  return {
    type: "componentScene",
    surfaceId,
    controllerId,
    componentKind: "board-2d",
    board2d: {
      fixtureJson: JSON.stringify(fixture),
      cameraJson: JSON.stringify(snapshot.camera2d),
      glyphCatalogsJson: block2dGlyphCatalogsJson(snapshot),
      selectionJson: "[]",
      interactive,
      activeUtility: "select",
      selectionMethod: "rectangle",
      gridSnapEnabled: false,
      gridFactor: 1,
      suggestionOffset: 0,
      brushWeightsJson: JSON.stringify({ nodeWeights: {}, handleWeights: {} }),
      placementCompatibilityJson: "[]",
      lodMode: "automatic",
    },
  };
}
//#endregion 🔖️Block2d

//#region 🔖️Block3d
/** @emoji 🎬️ The two `📚️examples/*` units the block3d subset registers. */
export const BLOCK3D_STORY_EXAMPLES: Readonly<Record<string, string>> = {
  "hexagonal-cut-concrete-forest-left": block3dLeftDsl,
  "nakagin-capsule": block3dNakaginDsl,
};

export const BLOCK3D_STORY_EXAMPLE_IDS: readonly string[] = Object.keys(BLOCK3D_STORY_EXAMPLES);

export function block3dStorySnapshotFor(exampleId: string): Block3dSnapshot {
  const dslText = BLOCK3D_STORY_EXAMPLES[exampleId];
  if (dslText === undefined) throw new Error(`[block3d-story] unknown example ${JSON.stringify(exampleId)}`);
  return parseBlock3dDsl(dslText);
}

/** @emoji 🧊️ Per-representation mesh admission: the resolved story url, or `null` with the reason a representation had to be dropped. */
export type BlockStoryMeshAdmission = { readonly id: string; readonly name: string; readonly meshUrl: string | undefined; readonly resolved: string | null };

export function admitBlockStoryRepresentations(representations: readonly { readonly id: string; readonly name: string; readonly meshUrl?: string }[]): readonly BlockStoryMeshAdmission[] {
  return representations.map((representation) => ({ id: representation.id, name: representation.name, meshUrl: representation.meshUrl, resolved: resolveBlockStoryMeshUrl(representation.meshUrl) }));
}

const BLOCK3D_STORY_CONTROLLER_ID = "block3d-story";

/**
 * @emoji 🌐️ The block3d world window's scene, projected the way `🧊️3d/…/👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️world/🦀️.rs`
 * builds it: one mesh + one instance per representation (at the document origin, identity rotation — a viewer
 * has no per-session arrangement offset), every rim vortex at its document position/direction/radius coloured
 * by its `vortex-kind-extra` row. Representations whose mesh identity does not resolve are DROPPED, never
 * rendered url-less: see `resolveBlockStoryMeshUrl`.
 */
export function buildBlock3dWorldSceneNode(snapshot: Block3dSnapshot, admitted: readonly BlockStoryMeshAdmission[], selectedIds: readonly string[], hoveredId: string | null): UiComponentSceneNode {
  const usable = admitted.filter((entry): entry is BlockStoryMeshAdmission & { readonly resolved: string } => entry.resolved !== null);
  const label = snapshot.objectKind.label === "" ? snapshot.objectKind.name : snapshot.objectKind.label;
  const selected = new Set(selectedIds);
  const meshes = usable.map((entry) => ({ id: `block3d-rep-${entry.id}`, url: entry.resolved }));
  const instances = usable.map((entry) => ({
    id: entry.id,
    meshId: `block3d-rep-${entry.id}`,
    position: [0, 0, 0],
    rotation: [0, 0, 0, 1],
    scale: [1, 1, 1],
    label: `${label} — ${entry.name}`,
    objectKind: snapshot.objectKind.id,
    selected: selected.has(entry.id),
    hovered: hoveredId === entry.id,
  }));
  const vortices = snapshot.vortices.map((vortex) => {
    const fullId = `${BLOCK3D_STORY_CONTROLLER_ID}:${vortex.id}`;
    return {
      fullId,
      objectId: snapshot.objectKind.id,
      vortexKind: vortex.vortexKind,
      position: vortex.position,
      direction: vortex.direction,
      radius: vortex.radius,
      color: snapshot.vortexKinds.find((kind) => kind.id === vortex.vortexKind)?.color ?? "#888888",
      selected: selected.has(fullId),
      hovered: hoveredId === fullId,
    };
  });
  return {
    type: "componentScene",
    surfaceId: "block3d.play.world",
    controllerId: BLOCK3D_STORY_CONTROLLER_ID,
    componentKind: "world-3d",
    world3d: {
      cameraJson: JSON.stringify(snapshot.camera3d),
      meshesJson: JSON.stringify(meshes),
      instancesJson: JSON.stringify(instances),
      selectionJson: JSON.stringify({ ids: selectedIds, selectionMode: "object" }),
      vorticesJson: JSON.stringify(vortices),
      interactionJson: JSON.stringify({ activeUtility: "select" }),
    },
  };
}
//#endregion 🔖️Block3d

//#region 🔖️Block5d
/** @emoji 🎬️ The two `📚️examples/*` units the block5d subset registers (`🖐️5d/…/✳️any/🦀️.rs`'s `examples()`). */
export const BLOCK5D_STORY_EXAMPLES: Readonly<Record<string, string>> = {
  "hexagonal-cut-concrete-forest-left": block5dLeftDsl,
  "nakagin-capsule": block5dNakaginDsl,
};

export const BLOCK5D_STORY_EXAMPLE_IDS: readonly string[] = Object.keys(BLOCK5D_STORY_EXAMPLES);

export type Block5dStoryState = { readonly exampleId: string; readonly snapshot: Block5dSnapshot };

export function block5dStoryStateFor(exampleId: string): Block5dStoryState {
  const dslText = BLOCK5D_STORY_EXAMPLES[exampleId];
  if (dslText === undefined) throw new Error(`[block5d-story] unknown example ${JSON.stringify(exampleId)}`);
  return { exampleId, snapshot: parseBlock5dDsl(dslText) };
}

/** @emoji 🧩️ Story-local mirror of block5d's seven-command table (`🖐️5d/…/✏️editor/🦀️.rs`, `BLOCK5D_RETAINED_TOOL_IDS`) for the six document-shaping actions the 5D stories exercise. */
export function reduceBlock5dStoryAction(state: Block5dStoryState, action: string, rawArgs: unknown): Block5dStoryState {
  const args = blockStoryActionArgs(rawArgs);
  const { snapshot } = state;
  switch (action) {
    case "setActiveExample": {
      const exampleId = args.exampleId;
      return typeof exampleId === "string" && BLOCK5D_STORY_EXAMPLE_IDS.includes(exampleId) ? block5dStoryStateFor(exampleId) : state;
    }
    case "patchPartKind": {
      const field = typeof args.field === "string" ? args.field : undefined;
      const value = typeof args.value === "string" ? args.value : undefined;
      if (field === undefined || value === undefined || !(field in snapshot.partKind)) return state;
      return { ...state, snapshot: { ...snapshot, partKind: { ...snapshot.partKind, [field]: value } } };
    }
    case "addGripKind": {
      const ids = new Set(snapshot.gripKinds.map((kind) => kind.id));
      const id = typeof args.id === "string" && !ids.has(args.id) ? args.id : nextBlockId("k", ids);
      return {
        ...state,
        snapshot: {
          ...snapshot,
          gripKinds: [
            ...snapshot.gripKinds,
            { id, name: id, label: typeof args.label === "string" ? args.label : id, color: typeof args.color === "string" ? args.color : "hsl(0 52% 48%)", defaultRopeKind: typeof args.defaultRopeKind === "string" ? args.defaultRopeKind : "rope.link" },
          ],
        },
      };
    }
    case "removeGripKind": {
      const id = typeof args.id === "string" ? args.id : snapshot.gripKinds.at(-1)?.id;
      if (id === undefined) return state;
      return { ...state, snapshot: { ...snapshot, gripKinds: snapshot.gripKinds.filter((kind) => kind.id !== id), grips: snapshot.grips.filter((grip) => grip.gripKind !== id) } };
    }
    case "addGrip": {
      const ids = new Set(snapshot.grips.map((grip) => grip.id));
      const id = typeof args.id === "string" && !ids.has(args.id) ? args.id : nextBlockId("g", ids);
      const seed = snapshot.grips[0];
      const angle = typeof args.angle === "number" ? args.angle : ((snapshot.grips.length + 1) * Math.PI) / 4;
      return {
        ...state,
        snapshot: {
          ...snapshot,
          grips: [
            ...snapshot.grips,
            {
              id,
              gripKind: typeof args.gripKind === "string" ? args.gripKind : (snapshot.gripKinds[0]?.id ?? ""),
              angle,
              radius2d: seed?.radius2d ?? 3,
              position: [Math.cos(angle) * (seed?.radius3d ?? 1) * 4, Math.sin(angle) * (seed?.radius3d ?? 1) * 4, seed?.position[2] ?? 0],
              direction: [Math.cos(angle), Math.sin(angle), 0],
              radius3d: seed?.radius3d ?? 0.3,
            },
          ],
        },
      };
    }
    case "removeGrip": {
      const id = typeof args.id === "string" ? args.id : snapshot.grips.at(-1)?.id;
      if (id === undefined) return state;
      return { ...state, snapshot: { ...snapshot, grips: snapshot.grips.filter((grip) => grip.id !== id) } };
    }
    default:
      return state;
  }
}

/** @emoji 📋️ The `ui_text` lines block5d's board window emits (`🖐️5d/…/✏️editor/🎭️modes/✏️edit/🪟️windows/📋️board/🦀️.rs`'s `render`: part-kind label + `"2d grips: <n>"`), extended with one line per grip kind and per grip so the story's board panel shows the geometry it draws. */
export function block5dBoardRenderLines(snapshot: Block5dSnapshot): readonly string[] {
  const lines = [`Part kind: ${snapshot.partKind.label === "" ? "—" : snapshot.partKind.label}`, `2d grips: ${snapshot.grips.length}`, `${snapshot.gripKinds.length} grip kind(s)`];
  for (const kind of snapshot.gripKinds) lines.push(`  ◦ ${kind.label} (${kind.id}) — ${kind.color}`);
  for (const grip of snapshot.grips) lines.push(`  ◦ ${grip.id} — kind ${grip.gripKind}, angle ${((grip.angle * 180) / Math.PI).toFixed(1)}°, radius ${grip.radius2d.toFixed(2)}`);
  return lines;
}

/** @emoji 🌐️ The `mesh:` line block5d's world window emits (`🖐️5d/…/🪟️windows/🌐️world/🦀️.rs`'s `render`: part-kind label + the FIRST representation's `mesh_url`, `"—"` when there is none). */
export function block5dWorldRenderLines(snapshot: Block5dSnapshot): readonly string[] {
  return [`Part kind: ${snapshot.partKind.label === "" ? "—" : snapshot.partKind.label}`, `mesh: ${snapshot.representations[0]?.meshUrl ?? "—"}`];
}

/** @emoji 📋️ block5d's 2D projection as a board fixture: the part kind as one `part-2d`-shaped glyph, every grip at its own `angle`/`radius-2d`. */
export function buildBlock5dBoardSceneNode(snapshot: Block5dSnapshot, interactive: boolean): UiComponentSceneNode {
  const radius = snapshot.part2d.radius;
  const fixture = {
    schema: "block.5d.fixture",
    camera: snapshot.camera2d,
    nodes: [
      {
        id: snapshot.partKind.id,
        nodeKind: snapshot.partKind.id,
        shape: snapshot.part2d.shape,
        x: 0,
        y: 0,
        radius,
        text: snapshot.partKind.label,
        handles: snapshot.grips.map((grip) => ({ id: grip.id, handleKind: grip.gripKind, angle: grip.angle, radius: grip.radius2d })),
      },
    ],
    edges: [],
  };
  return {
    type: "componentScene",
    surfaceId: "block5d.play.board",
    controllerId: "block5d-story",
    componentKind: "board-2d",
    board2d: {
      fixtureJson: JSON.stringify(fixture),
      cameraJson: JSON.stringify(snapshot.camera2d),
      glyphCatalogsJson: JSON.stringify({
        handleKinds: snapshot.gripKinds.map((kind) => ({ id: kind.id, name: kind.name, color: kind.color, defaultWireKind: kind.defaultRopeKind })),
        nodeKinds: [{ id: snapshot.partKind.id, name: snapshot.partKind.name, shape: snapshot.part2d.shape, handles: snapshot.grips.map((grip) => ({ handleKind: grip.gripKind, angle: grip.angle, radius: grip.radius2d })) }],
      }),
      selectionJson: "[]",
      interactive,
      activeUtility: "select",
      selectionMethod: "rectangle",
      gridSnapEnabled: false,
      gridFactor: 1,
      suggestionOffset: 0,
      brushWeightsJson: JSON.stringify({ nodeWeights: {}, handleWeights: {} }),
      placementCompatibilityJson: "[]",
      lodMode: "automatic",
    },
  };
}

const BLOCK5D_STORY_CONTROLLER_ID = "block5d-story";

/** @emoji 🌐️ block5d's 3D projection as a world scene — same shape as {@link buildBlock3dWorldSceneNode}, with grips standing in for vortices (`🖐️5d/…/👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️world/🦀️.rs` builds its mesh window off the same `representations`). */
export function buildBlock5dWorldSceneNode(snapshot: Block5dSnapshot, admitted: readonly BlockStoryMeshAdmission[], selectedIds: readonly string[], hoveredId: string | null): UiComponentSceneNode {
  const usable = admitted.filter((entry): entry is BlockStoryMeshAdmission & { readonly resolved: string } => entry.resolved !== null);
  const label = snapshot.partKind.label === "" ? snapshot.partKind.name : snapshot.partKind.label;
  const selected = new Set(selectedIds);
  return {
    type: "componentScene",
    surfaceId: "block5d.play.world",
    controllerId: BLOCK5D_STORY_CONTROLLER_ID,
    componentKind: "world-3d",
    world3d: {
      cameraJson: JSON.stringify(snapshot.camera3d),
      meshesJson: JSON.stringify(usable.map((entry) => ({ id: `block5d-rep-${entry.id}`, url: entry.resolved }))),
      instancesJson: JSON.stringify(
        usable.map((entry) => ({
          id: entry.id,
          meshId: `block5d-rep-${entry.id}`,
          position: [0, 0, 0],
          rotation: [0, 0, 0, 1],
          scale: [1, 1, 1],
          label: `${label} — ${entry.name}`,
          objectKind: snapshot.partKind.id,
          selected: selected.has(entry.id),
          hovered: hoveredId === entry.id,
        })),
      ),
      selectionJson: JSON.stringify({ ids: selectedIds, selectionMode: "object" }),
      vorticesJson: JSON.stringify(
        snapshot.grips.map((grip) => {
          const fullId = `${BLOCK5D_STORY_CONTROLLER_ID}:${grip.id}`;
          return {
            fullId,
            objectId: snapshot.partKind.id,
            vortexKind: grip.gripKind,
            position: grip.position,
            direction: grip.direction,
            radius: grip.radius3d,
            color: snapshot.gripKinds.find((kind) => kind.id === grip.gripKind)?.color ?? "#888888",
            selected: selected.has(fullId),
            hovered: hoveredId === fullId,
          };
        }),
      ),
      interactionJson: JSON.stringify({ activeUtility: "select" }),
    },
  };
}
//#endregion 🔖️Block5d
