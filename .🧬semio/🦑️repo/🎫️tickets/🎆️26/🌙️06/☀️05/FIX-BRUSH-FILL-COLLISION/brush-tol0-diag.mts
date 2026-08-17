import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { Box3, Vector3 } from "three";
import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
import {
  brushCompatibleCandidates,
  brushPlacementCollisionExcludeObjectIds,
  brushPreviewCollisionBox,
  brushPreviewCollides,
  brushPreviewFromCandidate,
  brushProbeGroupFromPreview,
  boxesPenetrationExceeds,
  brushCollisionContactEpsilon,
  brushCollisionGltfRoot,
  clearBrushCollisionGltfScenes,
  parseFixtureV1,
  registerBrushCollisionGltfScene,
  enumerateBrushFillVortexTargets,
  vortexWorldCadFromObject,
  type KindCatalogBundle,
  type KindCompatEntry,
  type BrushSceneCollisionSource,
} from "/Users/ueli/Documents/compose/puzzle/3d/react/index.tsx";
import { Group, Mesh, BoxGeometry } from "three";

const repo = "/Users/ueli/Documents/compose";
const loader = new GLTFLoader();
function loadGlb(name: string) {
  const bytes = readFileSync(resolve(repo, "compose/fixtures/kit/folder/abbau-aufbau", name));
  const buf = bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength);
  return new Promise<Group>((res, rej) => loader.parse(buf, "", (g) => res(g.scene), rej));
}

clearBrushCollisionGltfScenes();
const left = await loadGlb("hexagonal-cut-concrete-forest-left.glb");
const right = await loadGlb("hexagonal-cut-concrete-forest-right.glb");
registerBrushCollisionGltfScene("/meshes/hexagonal-cut-concrete-forest-left.glb", left);
registerBrushCollisionGltfScene("/meshes/hexagonal-cut-concrete-forest-right.glb", right);

const fixtureJson = JSON.parse(readFileSync(resolve(repo, "puzzle/3d/fixture/concrete-forest.3d.json"), "utf8"));
const f = parseFixtureV1(fixtureJson)!;
const catalogs = fixtureJson.meta.kindCatalogs as KindCatalogBundle;
const compat = fixtureJson.meta.kindCompatibility as KindCompatEntry[];
const seed = f.objects[0]!;

// mock scene with seed object as probe group
const seedProbe = brushProbeGroupFromPreview({ origin: seed.origin, orientation: seed.orientation, scale: seed.scale }, left);
const seedGroup = new Group();
seedGroup.userData.puzzle3dObjectId = seed.id;
seedGroup.add(seedProbe.children[0]!.clone(true));
seedGroup.position.copy(seedProbe.position);
seedGroup.quaternion.copy(seedProbe.quaternion);
seedGroup.scale.copy(seedProbe.scale);

const scene: BrushSceneCollisionSource = { collectObjectGroups: () => [seedGroup] };
const tol = 0;
let free = 0,
  hostOverlap = 0,
  otherOverlap = 0,
  proposed = 0;

for (const target of enumerateBrushFillVortexTargets(f)) {
  const host = f.objects.find((o) => o.id === target.objectId);
  if (!host) continue;
  const world = vortexWorldCadFromObject(host, target.vortexIndex);
  if (!world) continue;
  const targetCtx = { objectId: target.objectId, objectKind: target.objectKind, vortexKind: target.vortexKind };
  const candidates = brushCompatibleCandidates(targetCtx, catalogs, compat);
  const exclude = brushPlacementCollisionExcludeObjectIds(target.objectId, f.attractions);
  for (const candidate of candidates) {
    const preview = brushPreviewFromCandidate({
      targetVortexFullId: target.fullId,
      candidate,
      target: targetCtx,
      targetWorldPositionCad: world.position,
      targetWorldDirectionCad: world.direction,
      referenceOrientationCad: host.orientation,
      kindCatalogs: catalogs,
      sceneFixture: f,
    });
    if (!preview) continue;
    const meshRoot = brushCollisionGltfRoot(preview.meshUrl);
    if (!meshRoot) continue;
    const probe = brushProbeGroupFromPreview(preview, meshRoot);
    const collides = brushPreviewCollides(scene, probe, exclude, tol);
    if (!collides) {
      free++;
      proposed++;
      const previewBox = brushPreviewCollisionBox(probe, tol);
      const hostBox = brushPreviewCollisionBox(seedGroup, tol);
      const ce = brushCollisionContactEpsilon(tol);
      if (boxesPenetrationExceeds(previewBox, hostBox, tol, ce)) hostOverlap++;
    }
  }
}
console.log("[DEBUG] tol=0 free placements:", free, "proposed:", proposed, "of which overlap host (excluded from collides):", hostOverlap);
