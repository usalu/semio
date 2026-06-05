import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
import { Group } from "three";
import { solidOverlapVolume } from "/Users/ueli/Documents/semio/infinite/world/r3f/index.tsx";
import {
  applyBrushFillPlacementsToFixture,
  brushCollisionGltfRoot,
  brushPreviewWorldMatrix,
  brushCollisionBody,
  buildBrushFillSequence,
  clearBrushCollisionGltfScenes,
  DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET,
  parseFixtureV1,
  registerBrushCollisionGltfScene,
  resolveObjectKindMeshUrl,
} from "/Users/ueli/Documents/semio/puzzle/3d/react/index.tsx";

const repo = "/Users/ueli/Documents/semio";
const loader = new GLTFLoader();
const load = (n: string) =>
  new Promise<Group>((res, rej) =>
    loader.parse(readFileSync(resolve(repo, "semio/fixtures/kit/folder/abbau-aufbau", n)).buffer.slice(0), "", (g) => res(g.scene), rej),
  );

clearBrushCollisionGltfScenes();
registerBrushCollisionGltfScene("/meshes/hexagonal-cut-concrete-forest-left.glb", await load("hexagonal-cut-concrete-forest-left.glb"));
registerBrushCollisionGltfScene("/meshes/hexagonal-cut-concrete-forest-right.glb", await load("hexagonal-cut-concrete-forest-right.glb"));

const fixtureJson = JSON.parse(readFileSync(resolve(repo, "puzzle/3d/fixture/concrete-forest.3d.json"), "utf8"));
const f = parseFixtureV1(fixtureJson)!;
const catalogs = fixtureJson.meta.kindCatalogs;
const compat = fixtureJson.meta.kindCompatibility;
const budget = DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET;

const seq = buildBrushFillSequence({
  baseFixture: f,
  maxCount: 40,
  seed: 42,
  kindCatalogs: catalogs,
  kindCompatibility: compat,
  overlapBudget: budget,
  meshRootForUrl: brushCollisionGltfRoot,
});
const applied = applyBrushFillPlacementsToFixture(f, seq, catalogs);
console.log("[DEBUG] fill overlapBudget", budget, "placed", seq.length, "objects", applied.objects.length);

const entries = applied.objects.map((obj) => {
  const url = resolveObjectKindMeshUrl(obj.objectKind, catalogs, applied)!;
  const body = brushCollisionBody(url)!;
  const world = brushPreviewWorldMatrix({ origin: obj.origin, orientation: obj.orientation, scale: obj.scale });
  return { id: obj.id, body, world };
});

let badPairs = 0;
for (let i = 0; i < entries.length; i++) {
  for (let j = i + 1; j < entries.length; j++) {
    const a = entries[i]!;
    const b = entries[j]!;
    const vol = solidOverlapVolume(a.body, a.world, b.body, b.world, { sampleCount: 1024 });
    if (vol > budget) {
      badPairs += 1;
      if (badPairs <= 8) {
        console.log("[DEBUG] pair", a.id, "x", b.id, "overlap m3", vol.toFixed(3));
      }
    }
  }
}
console.log("[DEBUG] pairs over budget:", badPairs);
