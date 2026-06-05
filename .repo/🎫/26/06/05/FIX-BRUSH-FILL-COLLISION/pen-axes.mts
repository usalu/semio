import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
import {
  brushCompatibleCandidates, brushPreviewCollisionBox, brushPreviewFromCandidate, brushProbeGroupFromPreview,
  brushCollisionGltfRoot, clearBrushCollisionGltfScenes, parseFixtureV1, registerBrushCollisionGltfScene,
  enumerateBrushFillVortexTargets, vortexWorldCadFromObject,
} from "/Users/ueli/Documents/semio/puzzle/3d/react/index.tsx";
import { Group, Vector3 } from "three";

const repo = "/Users/ueli/Documents/semio";
const loader = new GLTFLoader();
const left = await new Promise<Group>((res,rej)=>loader.parse(readFileSync(resolve(repo,"semio/fixtures/kit/folder/abbau-aufbau/hexagonal-cut-concrete-forest-left.glb")).buffer.slice(0),"",g=>res(g.scene),rej));
const right = await new Promise<Group>((res,rej)=>loader.parse(readFileSync(resolve(repo,"semio/fixtures/kit/folder/abbau-aufbau/hexagonal-cut-concrete-forest-right.glb")).buffer.slice(0),"",g=>res(g.scene),rej));
clearBrushCollisionGltfScenes();
registerBrushCollisionGltfScene("/meshes/hexagonal-cut-concrete-forest-left.glb", left);
registerBrushCollisionGltfScene("/meshes/hexagonal-cut-concrete-forest-right.glb", right);
const fixtureJson = JSON.parse(readFileSync(resolve(repo,"puzzle/3d/fixture/concrete-forest.3d.json"),"utf8"));
const f = parseFixtureV1(fixtureJson)!;
const catalogs = fixtureJson.meta.kindCatalogs;
const compat = fixtureJson.meta.kindCompatibility;
const seed = f.objects[0]!;
const seedBox = brushPreviewCollisionBox(brushProbeGroupFromPreview({origin:seed.origin,orientation:seed.orientation}, left), 0);
let faceOnly = 0, volumePen = 0;
const eps = 1e-4;
for (const target of enumerateBrushFillVortexTargets(f)) {
  const host = f.objects.find((o) => o.id === target.objectId)!;
  const world = vortexWorldCadFromObject(host, target.vortexIndex)!;
  const targetCtx = { objectId: target.objectId, objectKind: target.objectKind, vortexKind: target.vortexKind };
  for (const candidate of brushCompatibleCandidates(targetCtx, catalogs, compat)) {
    const preview = brushPreviewFromCandidate({ targetVortexFullId: target.fullId, candidate, target: targetCtx, targetWorldPositionCad: world.position, targetWorldDirectionCad: world.direction, referenceOrientationCad: host.orientation, kindCatalogs: catalogs, sceneFixture: f });
    if (!preview) continue;
    const probe = brushProbeGroupFromPreview(preview, brushCollisionGltfRoot(preview.meshUrl)!);
    const b = brushPreviewCollisionBox(probe, 0);
    const ox = Math.min(b.max.x, seedBox.max.x) - Math.max(b.min.x, seedBox.min.x);
    const oy = Math.min(b.max.y, seedBox.max.y) - Math.max(b.min.y, seedBox.min.y);
    const oz = Math.min(b.max.z, seedBox.max.z) - Math.max(b.min.z, seedBox.min.z);
    if (ox > eps && oy > eps && oz > eps) volumePen++;
    else faceOnly++;
  }
}
console.log("[DEBUG] vs host: face-contact only", faceOnly, "volume penetration (all axes)", volumePen);
