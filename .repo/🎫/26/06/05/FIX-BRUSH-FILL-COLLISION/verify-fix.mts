import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { BoxGeometry, Group, Mesh } from "three";
import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
import {
  applyBrushFillPlacementsToFixture,
  boxesPenetrationExceeds,
  brushCollisionContactEpsilon,
  brushCollisionGltfRoot,
  brushPreviewCollisionBox,
  brushProbeGroupFromPreview,
  buildBrushFillSequence,
  clearBrushCollisionGltfScenes,
  parseFixtureV1,
  registerBrushCollisionGltfScene,
  resolveObjectKindMeshUrl,
  type KindCatalogBundle,
  type KindCompatEntry,
} from "/Users/ueli/Documents/compose/puzzle/3d/react/index.tsx";

const repo = "/Users/ueli/Documents/compose";
const loader = new GLTFLoader();
function loadGlb(name: string): Promise<Group> {
  const bytes = readFileSync(resolve(repo, "compose/fixtures/kit/folder/abbau-aufbau", name));
  const buf = bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength);
  return new Promise((res, rej) => loader.parse(buf, "", (g) => res(g.scene), rej));
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

const stub = new Mesh(new BoxGeometry(1, 1, 1));
const stubSeq = buildBrushFillSequence({ baseFixture: f, maxCount: 100, seed: 42, kindCatalogs: catalogs, kindCompatibility: compat, collisionTolerance: 1, meshRootForUrl: () => stub });
const realSeq = buildBrushFillSequence({ baseFixture: f, maxCount: 100, seed: 42, kindCatalogs: catalogs, kindCompatibility: compat, collisionTolerance: 1, meshRootForUrl: brushCollisionGltfRoot });
console.log("[DEBUG] stub seq", stubSeq.length, "real seq", realSeq.length);

const applied = applyBrushFillPlacementsToFixture(f, realSeq, catalogs);
let pairs = 0;
const boxes = applied.objects.map((obj) => {
  const url = resolveObjectKindMeshUrl(obj.objectKind, catalogs, applied)!;
  const root = brushCollisionGltfRoot(url)!;
  const probe = brushProbeGroupFromPreview({ origin: obj.origin, orientation: obj.orientation, scale: obj.scale }, root);
  return brushPreviewCollisionBox(probe, 0);
});
for (let i = 0; i < boxes.length; i++)
  for (let j = i + 1; j < boxes.length; j++) {
    if (boxesPenetrationExceeds(boxes[i]!, boxes[j]!, 0, brushCollisionContactEpsilon(0))) pairs++;
  }
console.log("[DEBUG] real fill pairs at tol=0:", pairs);
