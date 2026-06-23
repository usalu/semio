import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { Box3, Group, Vector3 } from "three";
import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
import {
  applyBrushFillPlacementsToFixture,
  boxesPenetrationExceeds,
  brushCollisionContactEpsilon,
  brushPreviewCollisionBox,
  brushPreviewMeshFrameGroup,
  brushProbeGroupFromPreview,
  buildBrushFillSequence,
  parseFixtureV1,
  resolveObjectKindMeshUrl,
  type KindCatalogBundle,
  type KindCompatEntry,
  updateWorldMatrixChain,
} from "/Users/ueli/Documents/compose/puzzle/3d/react/index.tsx";

const repoRoot = "/Users/ueli/Documents/compose";

function parseKindCatalogs(meta: Record<string, unknown> | undefined): KindCatalogBundle | undefined {
  const raw = meta?.kindCatalogs;
  if (!raw || typeof raw !== "object") return undefined;
  return raw as KindCatalogBundle;
}

function parseKindCompatibility(meta: Record<string, unknown> | undefined): readonly KindCompatEntry[] {
  const raw = meta?.kindCompatibility;
  return Array.isArray(raw) ? (raw as KindCompatEntry[]) : [];
}
const meshDir = resolve(repoRoot, "compose/fixtures/kit/folder/abbau-aufbau");
const fixtureJson = JSON.parse(
  readFileSync(resolve(repoRoot, "puzzle/3d/fixture/concrete-forest.3d.json"), "utf8"),
);

const loader = new GLTFLoader();
function loadMesh(name: string): Promise<Group> {
  const bytes = readFileSync(resolve(meshDir, name));
  const buf = bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength);
  return new Promise((resolveP, reject) => {
    loader.parse(buf, "", (gltf) => resolveP(gltf.scene), reject);
  });
}

const meshes = new Map<string, Group>();
const meshRootForUrl = (url: string) => {
  const name = url.replace("/meshes/", "");
  return meshes.get(name) ?? null;
};

function collisionBoxForObject(
  obj: { objectKind?: string; origin: number[]; orientation: number[]; scale?: number },
  catalogs: ReturnType<typeof parseKindCatalogs>,
  fixture: NonNullable<ReturnType<typeof parseFixtureV1>>,
) {
  const url = resolveObjectKindMeshUrl(obj.objectKind, catalogs, fixture);
  if (!url) return null;
  const meshRoot = meshRootForUrl(url);
  if (!meshRoot) return null;
  const probe = brushProbeGroupFromPreview(
    { origin: obj.origin as [number, number, number], orientation: obj.orientation as [number, number, number, number], scale: obj.scale },
    meshRoot,
  );
  updateWorldMatrixChain(probe);
  const box = brushPreviewCollisionBox(probe, 0);
  return Number.isFinite(box.min.x) && !box.isEmpty() ? box : null;
}

function maxPenetration(a: Box3, b: Box3): number {
  const ox = Math.min(a.max.x, b.max.x) - Math.max(a.min.x, b.min.x);
  const oy = Math.min(a.max.y, b.max.y) - Math.max(a.min.y, b.min.y);
  const oz = Math.min(a.max.z, b.max.z) - Math.max(a.min.z, b.min.z);
  if (ox <= 0 || oy <= 0 || oz <= 0) return 0;
  return Math.min(ox, oy, oz);
}

function countPairCollisions(
  objects: readonly { id: string; objectKind?: string; origin: number[]; orientation: number[]; scale?: number }[],
  catalogs: ReturnType<typeof parseKindCatalogs>,
  fixture: NonNullable<ReturnType<typeof parseFixtureV1>>,
) {
  const boxes: { id: string; box: Box3 }[] = [];
  for (const obj of objects) {
    const box = collisionBoxForObject(obj, catalogs, fixture);
    if (box) boxes.push({ id: obj.id, box });
  }
  let pairs = 0;
  let maxPen = 0;
  const examples: string[] = [];
  for (let i = 0; i < boxes.length; i++) {
    for (let j = i + 1; j < boxes.length; j++) {
      const a = boxes[i]!;
      const b = boxes[j]!;
      const pen = maxPenetration(a.box, b.box);
      if (pen > maxPen) maxPen = pen;
      if (boxesPenetrationExceeds(a.box, b.box, 0, brushCollisionContactEpsilon(0))) {
        pairs++;
        if (examples.length < 10) {
          examples.push(`${a.id} vs ${b.id} pen=${pen.toFixed(3)}`);
        }
      }
    }
  }
  return { pairs, boxes: boxes.length, examples, maxPen };
}

const f = parseFixtureV1(fixtureJson);
const catalogs = parseKindCatalogs(f?.meta);
const compat = parseKindCompatibility(f?.meta);

const left = await loadMesh("hexagonal-cut-concrete-forest-left.glb");
const right = await loadMesh("hexagonal-cut-concrete-forest-right.glb");
meshes.set("hexagonal-cut-concrete-forest-left.glb", left);
meshes.set("hexagonal-cut-concrete-forest-right.glb", right);

for (const [name, scene] of meshes) {
  const frame = brushPreviewMeshFrameGroup(scene);
  const g = new Group();
  g.add(frame);
  updateWorldMatrixChain(g);
  const raw = new Box3().setFromObject(g, true);
  console.log(`[DEBUG] mesh ${name} AABB size`, raw.getSize(new Vector3()).toArray());
}

function runFill(meshFn: (url: string) => Group | null, label: string, tolerance: number) {
  const sequence = buildBrushFillSequence({
    baseFixture: f!,
    maxCount: 100,
    seed: 42,
    kindCatalogs: catalogs,
    kindCompatibility: compat,
    collisionTolerance: tolerance,
    meshRootForUrl: meshFn,
  });
  const applied = applyBrushFillPlacementsToFixture(f!, sequence, catalogs);
  const result = countPairCollisions(applied.objects, catalogs, applied);
  console.log(`[DEBUG] ${label} tol=${tolerance} seq=${sequence.length} pairs=${result.pairs} maxPen=${result.maxPen.toFixed(3)}`);
}

const sequence = buildBrushFillSequence({
  baseFixture: f!,
  maxCount: 100,
  seed: 42,
  kindCatalogs: catalogs,
  kindCompatibility: compat,
  collisionTolerance: 1,
  meshRootForUrl,
});
const applied = applyBrushFillPlacementsToFixture(f!, sequence, catalogs);
const origins = applied.objects.map((o) => `${o.id} @ [${o.origin.map((v) => v.toFixed(2)).join(",")}]`);
const originGroups = new Map<string, string[]>();
for (const o of applied.objects) {
  const key = o.origin.map((v) => v.toFixed(1)).join(",");
  const list = originGroups.get(key) ?? [];
  list.push(o.id);
  originGroups.set(key, list);
}
const stacked = [...originGroups.entries()].filter(([, ids]) => ids.length > 1);
console.log(`[DEBUG] stacked origins: ${stacked.length}`, stacked.slice(0, 5));
const meshVol = 9.8 * 4.5 * 3;
let overlapVol = 0;
const boxes: { id: string; box: Box3 }[] = [];
for (const obj of applied.objects) {
  const box = collisionBoxForObject(obj, catalogs, applied);
  if (box) boxes.push({ id: obj.id, box });
}
for (let i = 0; i < boxes.length; i++) {
  for (let j = i + 1; j < boxes.length; j++) {
    const a = boxes[i]!;
    const b = boxes[j]!;
    const ix = Math.max(0, Math.min(a.box.max.x, b.box.max.x) - Math.max(a.box.min.x, b.box.min.x));
    const iy = Math.max(0, Math.min(a.box.max.y, b.box.max.y) - Math.max(a.box.min.y, b.box.min.y));
    const iz = Math.max(0, Math.min(a.box.max.z, b.box.max.z) - Math.max(a.box.min.z, b.box.min.z));
    overlapVol += ix * iy * iz;
  }
}
console.log(`[DEBUG] total overlap volume: ${overlapVol.toFixed(1)} (meshVol~${meshVol.toFixed(0)}) ratio~${(overlapVol / (meshVol * applied.objects.length) * 100).toFixed(1)}%`);

