import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { BoxGeometry, Group, Mesh } from "three";
import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
import {
  applyBrushFillPlacementsToFixture,
  boxesPenetrationExceeds,
  brushPreviewCollisionBox,
  brushProbeGroupFromPreview,
  buildBrushFillSequence,
  parseFixtureV1,
  resolveObjectKindMeshUrl,
  updateWorldMatrixChain,
  type KindCatalogBundle,
  type KindCompatEntry,
} from "/Users/ueli/Documents/compose/puzzle/3d/react/index.tsx";

const fixtureJson = JSON.parse(readFileSync("/Users/ueli/Documents/compose/puzzle/3d/fixture/concrete-forest.3d.json", "utf8"));
const f = parseFixtureV1(fixtureJson)!;
const catalogs = fixtureJson.meta.kindCatalogs as KindCatalogBundle;
const compat = fixtureJson.meta.kindCompatibility as KindCompatEntry[];
const loader = new GLTFLoader();
const meshDir = "/Users/ueli/Documents/compose/compose/fixtures/kit/folder/abbau-aufbau";
function loadGlb(name: string): Promise<Group> {
  const bytes = readFileSync(resolve(meshDir, name));
  const buf = bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength);
  return new Promise((res, rej) => loader.parse(buf, "", (g) => res(g.scene), rej));
}
const realMeshes = new Map<string, Group>();
realMeshes.set("hexagonal-cut-concrete-forest-left.glb", await loadGlb("hexagonal-cut-concrete-forest-left.glb"));
realMeshes.set("hexagonal-cut-concrete-forest-right.glb", await loadGlb("hexagonal-cut-concrete-forest-right.glb"));

const unitBox = new Mesh(new BoxGeometry(1, 1, 1));
const seq = buildBrushFillSequence({
  baseFixture: f, maxCount: 100, seed: 42, kindCatalogs: catalogs, kindCompatibility: compat,
  collisionTolerance: 0, meshRootForUrl: () => unitBox,
});
const applied = applyBrushFillPlacementsToFixture(f, seq, catalogs);
console.log("[DEBUG] 1x1 probe seq", seq.length);
function realBox(obj: typeof applied.objects[0]) {
  const url = resolveObjectKindMeshUrl(obj.objectKind, catalogs, applied)!;
  const meshRoot = realMeshes.get(url.replace("/meshes/", ""))!;
  const probe = brushProbeGroupFromPreview({ origin: obj.origin, orientation: obj.orientation, scale: obj.scale }, meshRoot);
  updateWorldMatrixChain(probe);
  return brushPreviewCollisionBox(probe, 0);
}
let pairs = 0; let overlapVol = 0;
const meshVol = 9.8 * 4.5 * 3;
const boxes = applied.objects.map((o) => realBox(o));
for (let i = 0; i < boxes.length; i++) for (let j = i + 1; j < boxes.length; j++) {
  const a = boxes[i]!; const b = boxes[j]!;
  if (boxesPenetrationExceeds(a, b, 0, 0)) pairs++;
  const ix = Math.max(0, Math.min(a.max.x, b.max.x) - Math.max(a.min.x, b.min.x));
  const iy = Math.max(0, Math.min(a.max.y, b.max.y) - Math.max(a.min.y, b.min.y));
  const iz = Math.max(0, Math.min(a.max.z, b.max.z) - Math.max(a.min.z, b.min.z));
  overlapVol += ix * iy * iz;
}
console.log(`[DEBUG] pairs=${pairs} overlapVolRatio=${(overlapVol/(meshVol*applied.objects.length)*100).toFixed(1)}%`);
