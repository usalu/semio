/** @emoji 🔄 Bulk cell→solid rename for core + kernel-brepjs. */
import { readFileSync, writeFileSync } from "node:fs";

const files = ["c:/git/compose/spatial/js/core/index.ts", "c:/git/compose/spatial/js/kernel-brepjs/index.ts"];

const pairs: [string, string][] = [
  ["CellRecordDiff", "SolidRecordDiff"],
  ["CellSolid", "SolidPrimitive"],
  ["CellRecord", "SolidRecord"],
  ["CellRef", "SolidRef"],
  ["cellRef", "solidRef"],
  ["cellVolume", "solidVolume"],
  ["adjacentCells", "adjacentSolids"],
  ["cellSolidAabb", "solidPrimitiveAabb"],
  ["solidFromCellSolid", "solidFromSolidPrimitive"],
  ["solidForCell", "solidForSolidRecord"],
  ["derivedCellPoints", "derivedSolidPoints"],
  ["pointOnCellAt", "pointOnSolidAt"],
  ["cellPlacement", "solidPlacement"],
  ["disposeCell", "disposeSolid"],
  ["MutableCellRecord", "MutableSolidRecord"],
  ["model.cells", "model.solids"],
  ["diff.cells", "diff.solids"],
  ["inv.cells", "inv.solids"],
  ["geom(model).cells", "geom(model).solids"],
  ["g.cells", "g.solids"],
  ["geo.cells", "geo.solids"],
  ["this.cells", "this.solids"],
  ["applyEntityDiff(model.cells", "applyEntityDiff(model.solids"],
  ["isEntityDiffEmpty(d.cells)", "isEntityDiffEmpty(d.solids)"],
  ["readonly cell:", "readonly solid:"],
  ["r.cell", "r.solid"],
  ["wire.extrudeToCell", "wire.extrudeToSolid"],
  ["cell.createBox", "solid.createBox"],
  ["brepjs-cell-", "brepjs-solid-"],
  ['kind: "cell"', 'kind: "solid"'],
  ['case "cell":', 'case "solid":'],
  ["cells:", "solids:"],
  ["cells?", "solids?"],
  ["cells[", "solids["],
  ["cells.", "solids."],
  ["cells ", "solids "],
  ["cells)", "solids)"],
  ["cells,", "solids,"],
  ["cells]", "solids]"],
  ["cells`", "solids`"],
  ["const cells =", "const solids ="],
  ["res.diff.cells", "res.diff.solids"],
  ["CellSolid is stale", "SolidPrimitive is stale"],
  ["stores CellSolid", "stores SolidPrimitive"],
  ["for that cell", "for that solid"],
  ["same cell and", "same solid and"],
  ["other cells", "other solids"],
  ["one cell", "one solid"],
  ["per cell", "per solid"],
  ["cell minus", "solid minus"],
  ["cell ∩", "solid ∩"],
  ["cell \\\\", "solid \\\\"],
  ["authoritative brep for a cell", "authoritative brep for a solid"],
  ["WASM-side engine: exact solids keyed by `CellRef`", "WASM-side engine: exact solids keyed by `SolidRef`"],
  ["topologic-style `CellSolid`", "brepjs `SolidPrimitive`"],
  ["from shell vertices when present, else analytic `CellSolid`", "from shell vertices when present, else analytic `SolidPrimitive`"],
  ["Full axis-aligned box model: 8 vertices, 12 edges, 6 wires, 6 faces, one shell, one cell.", "Full axis-aligned box model: 8 vertices, 12 edges, 6 wires, 6 faces, one shell, one solid."],
];

for (const file of files) {
  let text = readFileSync(file, "utf8");
  for (const [from, to] of pairs) {
    if (text.includes(from)) text = text.split(from).join(to);
  }
  // kernel-brepjs: drop cellComplexes/clusters from geom buckets
  if (file.includes("kernel-brepjs")) {
    text = text.replace(/\r?\n\tcellComplexes: Record<string, kernelGeometry\.CellComplexRecord>;\r?\n\tclusters: Record<string, kernelGeometry\.ClusterRecord>;\r?\n/, "\n");
  }
  writeFileSync(file, text);
}
