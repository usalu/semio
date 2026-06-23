/** @emoji ⏱ Compares O(1) fill prefix composition vs replaying placements. */
import { BoxGeometry, Mesh } from "three";
import {
  applyBrushFillPlacementsToFixture,
  buildBrushFillSequence,
  createBrushFillSequenceStepper,
  type FixtureV1,
  type KindCatalogBundle,
  type KindCompatEntry,
} from "/Users/ueli/Documents/compose/puzzle/3d/react/index.tsx";

const brushCatalogs: KindCatalogBundle = {
  objects: [
    {
      id: "Capsule L",
      meshUrl: "/meshes/capsule_L.glb",
      vortices: [{ vortexKind: "door capsule left", position: [1.3, -1.25, 0], direction: [1, 0, 0], radius: 0.36 }],
    },
    {
      id: "Tambour",
      meshUrl: "/meshes/tambour.glb",
      vortices: [{ vortexKind: "door tambour left", position: [0.9, 2.75, 0.2], direction: [0, 1, 0], radius: 0.36 }],
    },
  ],
};
const brushCompat: readonly KindCompatEntry[] = [{ bidirectional: true, specificity: "vortex", source: "door capsule left", target: "door tambour left" }];
const meshRoot = new Mesh(new BoxGeometry(2, 2, 2));
const fixture: FixtureV1 = {
  version: 1,
  objects: [
    {
      id: "host",
      objectKind: "Tambour",
      meshUrl: "/meshes/tambour.glb",
      origin: [0, 0, 0],
      orientation: [0, 0, 0, 1],
      vortices: [{ id: "host:v0", vortexKind: "door tambour left", label: "door tambour left", position: [0.9, 2.75, 0.2], direction: [0, 1, 0] }],
    },
  ],
  attractions: [],
};
const args = {
  baseFixture: fixture,
  maxCount: 64,
  seed: 42,
  kindCatalogs: brushCatalogs,
  kindCompatibility: brushCompat,
  meshRootForUrl: () => meshRoot,
};
const stepper = createBrushFillSequenceStepper(args);
let built = stepper.step(64);
while (!built.done) {
  built = stepper.step(64);
}
const n = built.sequence.length;
const composePrefix = (): FixtureV1 => ({
  ...fixture,
  objects: [...fixture.objects, ...built.appendedObjects],
  attractions: [...fixture.attractions, ...built.appendedAttractions],
});
const replayPrefix = (): FixtureV1 => applyBrushFillPlacementsToFixture(fixture, built.sequence, brushCatalogs);
const time = (label: string, fn: () => void): void => {
  const start = performance.now();
  fn();
  console.log(`[DEBUG] ${label} ms=${(performance.now() - start).toFixed(3)}`);
};
console.log(`[DEBUG] fill prefix timing n=${n}`);
time("compose prefix (O(1))", composePrefix);
time("replay prefix (O(n))", replayPrefix);
time("buildBrushFillSequence sync", () => {
  buildBrushFillSequence(args);
});
