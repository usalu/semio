import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { Box3, GLTFLoader } from "three";
import { GLTFLoader as L } from "three/addons/loaders/GLTFLoader.js";
import {
  brushCompatibleCandidates,
  brushPreviewCollisionBox,
  brushPreviewFromCandidate,
  brushProbeGroupFromPreview,
  brushCollisionGltfRoot,
  clearBrushCollisionGltfScenes,
  parseFixtureV1,
  registerBrushCollisionGltfScene,
  enumerateBrushFillVortexTargets,
  vortexWorldCadFromObject,
  type KindCatalogBundle,
  type KindCompatEntry,
} from "/Users/ueli/Documents/compose/puzzle/3d/react/index.tsx";
import { Group } from "three";

const repo = "/Users/ueli/Documents/compose";
const loader = new L();
const left = await new Promise<Group>((res, rej) => loader.parse(readFileSync(resolve(repo, "compose/fixtures/kit/folder/abbau-aufbau/hexagonal-cut-concrete-forest-left.glb")).buffer.slice(0), "", (g) => res(g.scene), rej));
clearBrushCollisionGltfScenes();
registerBrushCollisionGltfScene("/meshes/hexagonal-cut-concrete-forest-left.glb", left);
registerBrushCollisionGltfScene(
  "/meshes/hexagonal-cut-concrete-forest-right.glb",
  await new Promise<Group>((res, rej) => loader.parse(readFileSync(resolve(repo, "compose/fixtures/kit/folder/abbau-aufbau/hexagonal-cut-concrete-forest-right.glb")).buffer.slice(0), "", (g) => res(g.scene), rej)),
);
const f = parseFixtureV1(JSON.parse(readFileSync(resolve(repo, "puzzle/3d/fixture/concrete-forest.3d.json"), "utf8")))!;
const catalogs = JSON.parse(readFileSync(resolve(repo, "puzzle/3d/fixture/concrete-forest.3d.json"), "utf8")).meta.kindCatalogs;
const compat = JSON.parse(readFileSync(resolve(repo, "puzzle/3d/fixture/concrete-forest.3d.json"), "utf8")).meta.kindCompatibility;
const seed = f.objects[0]!;
const seedBox = brushPreviewCollisionBox(brushProbeGroupFromPreview({ origin: seed.origin, orientation: seed.orientation }, left), 0);
const target = enumerateBrushFillVortexTargets(f)[0]!;
const host = seed;
const world = vortexWorldCadFromObject(host, target.vortexIndex)!;
const targetCtx = { objectId: target.objectId, objectKind: target.objectKind, vortexKind: target.vortexKind };
const candidate = brushCompatibleCandidates(targetCtx, catalogs, compat)[0]!;
const preview = brushPreviewFromCandidate({
  targetVortexFullId: target.fullId,
  candidate,
  target: targetCtx,
  targetWorldPositionCad: world.position,
  targetWorldDirectionCad: world.direction,
  referenceOrientationCad: host.orientation,
  kindCatalogs: catalogs,
  sceneFixture: f,
})!;
const probe = brushProbeGroupFromPreview(preview, brushCollisionGltfRoot(preview.meshUrl)!);
const prevBox = brushPreviewCollisionBox(probe, 0);
const ox = Math.min(prevBox.max.x, seedBox.max.x) - Math.max(prevBox.min.x, seedBox.min.x);
const oy = Math.min(prevBox.max.y, seedBox.max.y) - Math.max(prevBox.min.y, seedBox.min.y);
const oz = Math.min(prevBox.max.z, seedBox.max.z) - Math.max(prevBox.min.z, seedBox.min.z);
console.log("[DEBUG] first candidate", candidate.objectKindId, "overlap axes", { ox, oy, oz }, "min pen", Math.min(ox, oy, oz));
console.log("[DEBUG] seed box size", seedBox.getSize({} as any));
console.log("[DEBUG] preview box size", prevBox.getSize({} as any));
