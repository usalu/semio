/** @emoji 🔄️ cells→solids and cellRef→solidRef in spatial siblings. */
import { readFileSync, writeFileSync } from "node:fs";

const files = ["c:/git/compose/spatial/js/query/index.ts", "c:/git/compose/spatial/js/renderer-r3f/index.tsx", "c:/git/compose/spatial/js/renderer-r3f/play/main.tsx", "c:/git/compose/spatial/js/machine-stately/index.ts"];

const pairs: [string, string][] = [
  ["cellRef", "solidRef"],
  ["CellRef", "SolidRef"],
  ["model.cells", "model.solids"],
  ["graph.cells", "graph.solids"],
  ["listModelCellRefs", "listModelSolidRefs"],
  ["readonly cell:", "readonly solid:"],
  ['kind: "cell"', 'kind: "solid"'],
  ['case "cell":', 'case "solid":'],
  ['"cell",', '"solid",'],
  ['| "cell"', '| "solid"'],
  ['add("cell"', 'add("solid"'],
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
  ["const cells =", "const solids ="],
  ["(cells)", "(solids)"],
  ["cells.has", "solids.has"],
  ["visitCell", "visitSolid"],
  ["cellComplexes", "__cellComplexesRemoved"],
  ["cellComplex", "__cellComplexRemoved"],
  ["clusters", "__clustersRemoved"],
  ["cluster", "__clusterRemoved"],
  ["builtin.kernel.__removedCellComplex", "__removed"],
];

for (const file of files) {
  let text = readFileSync(file, "utf8");
  for (const [from, to] of pairs) {
    if (text.includes(from)) text = text.split(from).join(to);
  }
  writeFileSync(file, text);
}

console.log("patch-siblings done");
