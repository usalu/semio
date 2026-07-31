import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
import {
  brushCompatibleCandidates,
  brushPreviewCollides,
  brushPreviewFromCandidate,
  brushProbeGroupFromPreview,
  brushPreviewCollisionBox,
  boxesPenetrationExceeds,
  brushCollisionContactEpsilon,
  brushCollisionGltfRoot,
  clearBrushCollisionGltfScenes,
  parseFixtureV1,
  registerBrushCollisionGltfScene,
  enumerateBrushFillVortexTargets,
  vortexWorldCadFromObject,
  type BrushSceneCollisionSource,
} from "/Users/ueli/Documents/compose/puzzle/3d/react/index.tsx";
import { Group } from "three";

const repo = "/Users/ueli/Documents/compose";
const loader = new GLTFLoader();
const left = await new Promise<Group>((res, rej) => loader.parse(readFileSync(resolve(repo, "compose/fixtures/kit/folder/abbau-aufbau/hexagonal-cut-concrete-forest-left.glb")).buffer.slice(0), "", (g) => res(g.scene), rej));
clearBrushCollisionGltfScenes();
registerBrushCollisionGltfScene("/meshes/hexagonal-cut-concrete-forest-left.glb", left);
registerBrushCollisionGltfScene(
  "/meshes/hexagonal-cut-concrete-forest-right.glb",
  await new Promise<Group>((res, rej) => loader.parse(readFileSync(resolve(repo, "compose/fixtures/kit/folder/abbau-aufbau/hexagonal-cut-concrete-forest-right.glb")).buffer.slice(0), "", (g) => res(g.scene), rej)),
);
const f = parseFixtureV1(JSON.parse(readFileSync(resolve(repo, "puzzle/3d/fixture/concrete-forest.3d.json"), "utf8")))!;
const catalogs = JSON.parse(readFileSync(resolve(repo, "puzzle/3d/fixture/concrete-forest.3d.json"), "utf8")).meta;
const seed = f.objects[0]!;
const seedProbe = brushProbeGroupFromPreview({ origin: seed.origin, orientation: seed.orientation }, left);
const seedGroup = new Group();
seedGroup.userData.puzzle3dObjectId = seed.id;
seedGroup.add(seedProbe.children[0]!.clone(true));
seedGroup.position.copy(seedProbe.position);
seedGroup.quaternion.copy(seedProbe.quaternion);
seedGroup.scale.copy(seedProbe.scale);
const scene: BrushSceneCollisionSource = { collectObjectGroups: () => [seedGroup] };
const target = enumerateBrushFillVortexTargets(f)[0]!;
const host = seed;
const world = vortexWorldCadFromObject(host, target.vortexIndex)!;
const targetCtx = { objectId: target.objectId, objectKind: target.objectKind, vortexKind: target.vortexKind };
const candidate = brushCompatibleCandidates(targetCtx, catalogs.kindCatalogs, catalogs.kindCompatibility)[0]!;
const preview = brushPreviewFromCandidate({
  targetVortexFullId: target.fullId,
  candidate,
  target: targetCtx,
  targetWorldPositionCad: world.position,
  targetWorldDirectionCad: world.direction,
  referenceOrientationCad: host.orientation,
  kindCatalogs: catalogs.kindCatalogs,
  sceneFixture: f,
})!;
const probe = brushProbeGroupFromPreview(preview, brushCollisionGltfRoot(preview.meshUrl)!);
const pb = brushPreviewCollisionBox(probe, 0);
const sb = brushPreviewCollisionBox(seedGroup, 0);
const ox = Math.min(pb.max.x, sb.max.x) - Math.max(pb.min.x, sb.min.x);
const oy = Math.min(pb.max.y, sb.max.y) - Math.max(pb.min.y, sb.min.y);
const oz = Math.min(pb.max.z, sb.max.z) - Math.max(pb.min.z, sb.min.z);
console.log("axes", { ox, oy, oz });
console.log("boxesPenetrationExceeds", boxesPenetrationExceeds(pb, sb, 0, 0));
console.log("brushPreviewCollides no exclude", brushPreviewCollides(scene, probe, undefined, 0));
console.log("brushPreviewCollides exclude host", brushPreviewCollides(scene, probe, seed.id, 0));
