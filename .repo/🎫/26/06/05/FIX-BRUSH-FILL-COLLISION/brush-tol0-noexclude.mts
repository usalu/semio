import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
import {
  brushCompatibleCandidates, brushPreviewCollides, brushPreviewFromCandidate, brushProbeGroupFromPreview,
  brushCollisionGltfRoot, clearBrushCollisionGltfScenes, parseFixtureV1, registerBrushCollisionGltfScene,
  enumerateBrushFillVortexTargets, vortexWorldCadFromObject, type KindCatalogBundle, type KindCompatEntry,
  type BrushSceneCollisionSource,
} from "/Users/ueli/Documents/semio/puzzle/3d/react/index.tsx";
import { Group } from "three";

const repo = "/Users/ueli/Documents/semio";
const loader = new GLTFLoader();
const left = await new Promise<Group>((res,rej)=>loader.parse(readFileSync(resolve(repo,"semio/fixtures/kit/folder/abbau-aufbau/hexagonal-cut-concrete-forest-left.glb")).buffer.slice(0),"",g=>res(g.scene),rej));
const right = await new Promise<Group>((res,rej)=>loader.parse(readFileSync(resolve(repo,"semio/fixtures/kit/folder/abbau-aufbau/hexagonal-cut-concrete-forest-right.glb")).buffer.slice(0),"",g=>res(g.scene),rej));
clearBrushCollisionGltfScenes();
registerBrushCollisionGltfScene("/meshes/hexagonal-cut-concrete-forest-left.glb", left);
registerBrushCollisionGltfScene("/meshes/hexagonal-cut-concrete-forest-right.glb", right);
const fixtureJson = JSON.parse(readFileSync(resolve(repo,"puzzle/3d/fixture/concrete-forest.3d.json"),"utf8"));
const f = parseFixtureV1(fixtureJson)!;
const catalogs = fixtureJson.meta.kindCatalogs as KindCatalogBundle;
const compat = fixtureJson.meta.kindCompatibility as KindCompatEntry[];
const seed = f.objects[0]!;
const seedProbe = brushProbeGroupFromPreview({ origin: seed.origin, orientation: seed.orientation }, left);
const seedGroup = new Group();
seedGroup.userData.puzzle3dObjectId = seed.id;
seedGroup.add(seedProbe.children[0]!.clone(true));
seedGroup.position.copy(seedProbe.position);
seedGroup.quaternion.copy(seedProbe.quaternion);
seedGroup.scale.copy(seedProbe.scale);
const scene: BrushSceneCollisionSource = { collectObjectGroups: () => [seedGroup] };
let withExclude = 0, withoutExclude = 0;
for (const target of enumerateBrushFillVortexTargets(f)) {
  const host = f.objects.find((o) => o.id === target.objectId)!;
  const world = vortexWorldCadFromObject(host, target.vortexIndex)!;
  const targetCtx = { objectId: target.objectId, objectKind: target.objectKind, vortexKind: target.vortexKind };
  for (const candidate of brushCompatibleCandidates(targetCtx, catalogs, compat)) {
    const preview = brushPreviewFromCandidate({ targetVortexFullId: target.fullId, candidate, target: targetCtx, targetWorldPositionCad: world.position, targetWorldDirectionCad: world.direction, referenceOrientationCad: host.orientation, kindCatalogs: catalogs, sceneFixture: f });
    if (!preview) continue;
    const meshRoot = brushCollisionGltfRoot(preview.meshUrl);
    if (!meshRoot) continue;
    const probe = brushProbeGroupFromPreview(preview, meshRoot);
    if (!brushPreviewCollides(scene, probe, seed.id, 0)) withExclude++;
    if (!brushPreviewCollides(scene, probe, undefined, 0)) withoutExclude++;
  }
}
console.log("[DEBUG] free with host excluded:", withExclude, "free without exclude:", withoutExclude);
