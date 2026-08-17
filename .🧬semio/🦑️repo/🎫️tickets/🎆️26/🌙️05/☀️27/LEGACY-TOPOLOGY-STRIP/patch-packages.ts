/** @emoji 🔄️ Patch query/renderer/machine for solids + drop cellComplex/cluster/surface/part. */
import { readFileSync, writeFileSync } from "node:fs";

const files = ["c:/git/compose/spatial/js/query/index.ts", "c:/git/compose/spatial/js/renderer-r3f/index.tsx", "c:/git/compose/spatial/js/renderer-r3f/play/main.tsx", "c:/git/compose/spatial/js/machine-stately/index.ts"];

const pairs: [string, string][] = [
  ["model.cells", "model.solids"],
  ["this.model.cells", "this.model.solids"],
  ["geom(model).cells", "geom(model).solids"],
  [".cells[", ".solids["],
  ['kind: "cell"', 'kind: "solid"'],
  ['case "cell":', 'case "solid":'],
  ['"cell",', '"solid",'],
  ['| "cell"', '| "solid"'],
  ['add("cell"', 'add("solid"'],
  ['walk("cell"', 'walk("solid"'],
  ['geometryEntityWireSegments(buckets, "cell"', 'geometryEntityWireSegments(buckets, "solid"'],
  ["cellIds", "solidIds"],
  ["cellId", "solidId"],
  ["cells:", "solids:"],
  ["cells,", "solids,"],
  ["cells ", "solids "],
  ["cells)", "solids)"],
  ["cells]", "solids]"],
  ["cells.", "solids."],
  ["cells?", "solids?"],
  ["cells[", "solids["],
  ["builtin.kernel.cellComplex", "builtin.kernel.__removed"],
  ["cellComplexes", "__removedCellComplexes"],
  ["cellComplex", "__removedCellComplex"],
  ["clusters", "__removedClusters"],
  ["cluster", "__removedCluster"],
  ['"surface"', '"__removed_surface"'],
  ['"part"', '"__removed_part"'],
  ['"volume"', '"__removed_volume"'],
  ["view.surfaces", "view.__removed_surfaces"],
  ["view.parts", "view.__removed_parts"],
  ["view.volumes", "view.__removed_volumes"],
  ["Surface", "__RemovedSurface"],
  ["Part", "__RemovedPart"],
  ["Volume", "__RemovedVolume"],
  ["CellComplexRecord", "never"],
];

for (const file of files) {
  let text = readFileSync(file, "utf8");
  for (const [from, to] of pairs) {
    if (text.includes(from)) text = text.split(from).join(to);
  }
  writeFileSync(file, text);
}

const fixtureGlob = [
  "c:/git/compose/spatial/fixtures/simple.spatial.json",
  "c:/git/compose/spatial/fixtures/spatial.spatial.json",
  "c:/git/compose/spatial/fixtures/small-building.model.json",
  "c:/git/compose/spatial/fixtures/tall-building.model.json",
  "c:/git/compose/spatial/fixtures/large-building.model.json",
  "c:/git/compose/spatial/fixtures/geometry.json",
  "c:/git/compose/spatial/fixtures/geometry-loom.json",
  "c:/git/compose/spatial/fixtures/geometry-routes.json",
];
for (const f of fixtureGlob) {
  let j = readFileSync(f, "utf8");
  j = j.replace(/\s*,?\s*"__removed_cellComplexes":\s*\[[^\]]*\]/g, "");
  j = j.replace(/\s*,?\s*"__removed_clusters":\s*\[[^\]]*\]/g, "");
  writeFileSync(f, j);
}

console.log("patch-packages done");
