/** @emoji 🔄️ cell→solid rename for spatial/js/core/index.ts only. */
import { readFileSync, writeFileSync } from "node:fs";

const file = "c:/git/compose/spatial/js/core/index.ts";
let text = readFileSync(file, "utf8");

const pairs: [string, string][] = [
  ["CellRecordDiff", "SolidRecordDiff"],
  ["CellSolid", "SolidPrimitive"],
  ["CellRecord", "SolidRecord"],
  ["CellRef", "SolidRef"],
  ["cellRef", "solidRef"],
  ["cellVolume", "solidVolume"],
  ["adjacentCells", "adjacentSolids"],
  ["cellSolidAabb", "solidPrimitiveAabb"],
  ["model.cells", "model.solids"],
  ["diff.cells", "diff.solids"],
  ["inv.cells", "inv.solids"],
  ["g.cells", "g.solids"],
  ["geo.cells", "geo.solids"],
  ["this.cells", "this.solids"],
  ["applyEntityDiff(model.cells", "applyEntityDiff(model.solids"],
  ["isEntityDiffEmpty(d.cells)", "isEntityDiffEmpty(d.solids)"],
  ["readonly cell:", "readonly solid:"],
  ["wire.extrudeToCell", "wire.extrudeToSolid"],
  ["cell.createBox", "solid.createBox"],
  ['case "cell":', 'case "solid":'],
  ['"cell",', '"solid",'],
  ['| "cell"', '| "solid"'],
  ['kind: "cell"', 'kind: "solid"'],
  ['"cells",', '"solids",'],
  ["cells:", "solids:"],
  ["cells?", "solids?"],
  ["cells[", "solids["],
  ["cells ", "solids "],
  ["cells)", "solids)"],
  ["cells,", "solids,"],
  ["cells]", "solids]"],
  ["cells`", "solids`"],
  ["cells.", "solids."],
];

for (const [from, to] of pairs) {
  if (text.includes(from)) text = text.split(from).join(to);
}

// Remove cellComplex / cluster blocks from Model class and JSON
text = text.replace(/\n\s*cellComplexes: Record<string, CellComplexRecord> = \{\};\n\s*clusters: Record<string, ClusterRecord> = \{\};/g, "");
text = text.replace(/\n\s*cellComplexes: sortedRecordValues\(this\.cellComplexes\),\n\s*clusters: sortedRecordValues\(this\.clusters\),/g, "");
text = text.replace(/\n\s*g\.cellComplexes = recordsById\(geo\.cellComplexes \?\? \[\]\);\n\s*g\.clusters = recordsById\(geo\.clusters \?\? \[\]\);/g, "");
text = text.replace(/\n\s*case "cellComplex":[\s\S]*?case "cluster":[\s\S]*?return \(model\.clusters\[id\][\s\S]*?\);\n/g, "\n");
text = text.replace(/const geoKeys = \["anchors", "vertices", "edges", "wires", "faces", "shells", "cells", "cellComplexes", "clusters"\] as const;/, 'const geoKeys = ["anchors", "vertices", "edges", "wires", "faces", "shells", "solids"] as const;');
text = text.replace(/\n\s*readonly cellComplexes\?: EntityDiff<CellComplexRecord, CellComplexRecordDiff, CellComplexRef>;\n\s*readonly clusters\?: EntityDiff<ClusterRecord, ClusterRecordDiff, ClusterRef>;/g, "");
text = text.replace(/\n\s*isEntityDiffEmpty\(d\.cellComplexes\) &&\n\s*isEntityDiffEmpty\(d\.clusters\)/g, "");
text = text.replace(/\n\s*const ccInv: EntityDiff<CellComplexRecord, CellComplexRecordDiff, CellComplexRef> = \{\};\n\s*const clInv: EntityDiff<ClusterRecord, ClusterRecordDiff, ClusterRef> = \{\};/g, "");
text = text.replace(/\n\s*applyEntityDiff\(model\.cellComplexes as Record<string, CellComplexRecord>, diff\.cellComplexes, ccInv\);\n\s*applyEntityDiff\(model\.clusters as Record<string, ClusterRecord>, diff\.clusters, clInv\);/g, "");
text = text.replace(/\n\s*if \(!isEntityDiffEmpty\(ccInv\)\) inv\.cellComplexes = ccInv;\n\s*if \(!isEntityDiffEmpty\(clInv\)\) inv\.clusters = clInv;/g, "");

// Remove CellComplexRecord / ClusterRecord type blocks in namespace
text = text.replace(/\n\s*\/\*\* @emoji 🧱️ Cell complex payload[\s\S]*?export interface ClusterRecord \{[\s\S]*?\}\n/g, "\n");

// First namespace: SolidRef only
text = text.replace(
  /export type ShellRef = string & \{ readonly __brand: "ShellRef" \};\n  export type CellRef = string & \{ readonly __brand: "CellRef" \};\n  export type CellComplexRef = string & \{ readonly __brand: "CellComplexRef" \};\n  export type ClusterRef = string & \{ readonly __brand: "ClusterRef" \};\n\n  \/\*\* @emoji 🧱️ Kernel-private geometry entity kinds[\s\S]*?export function cellRef\(id: string\): CellRef \{\n    return id as CellRef;\n  \}\n\}/,
  `export type ShellRef = string & { readonly __brand: "ShellRef" };
  export type SolidRef = string & { readonly __brand: "SolidRef" };

  /** @emoji 🧱️ Kernel-private geometry entity kinds for selection and query adapters. */
  export type EditableEntityKind = "anchor" | "vertex" | "edge" | "wire" | "face" | "shell" | "solid";

  /** @emoji 🪪️ Builds a branded \`SolidRef\` from an opaque id string. */
  export function solidRef(id: string): SolidRef {
    return id as SolidRef;
  }
}`,
);

text = text.replace(
  /type ShellRef = kernelGeometry\.ShellRef;\ntype CellRef = kernelGeometry\.CellRef;\ntype CellComplexRef = kernelGeometry\.CellComplexRef;\ntype ClusterRef = kernelGeometry\.ClusterRef;\ntype EditableEntityKind = kernelGeometry\.EditableEntityKind;\n\nexport const cellRef = kernelGeometry\.cellRef;\n/,
  "type ShellRef = kernelGeometry.ShellRef;\ntype SolidRef = kernelGeometry.SolidRef;\ntype EditableEntityKind = kernelGeometry.EditableEntityKind;\n",
);

text = text.replace(
  /const MODEL_ENTITY_KINDS = new Set<string>\(\["anchor", "vertex", "edge", "wire", "face", "shell", "cell", "cellComplex", "cluster", "object"\]\);/,
  'const MODEL_ENTITY_KINDS = new Set<string>(["anchor", "vertex", "edge", "wire", "face", "shell", "solid", "object"]);',
);

// Second namespace header: drop CellRef/cellRef duplicates
text = text.replace(
  /export type CellRef = string & \{ readonly __brand: "CellRef" \};\n  export type SolidRef = string & \{ readonly __brand: "SolidRef" \};\n  export type CellComplexRef = string & \{ readonly __brand: "CellComplexRef" \};\n  export type ClusterRef = string & \{ readonly __brand: "ClusterRef" \};\n  export type EditableEntityKind =[\s\S]*?export function cellRef\(id: string\): CellRef \{\n    return id as CellRef;\n  \}\n  export function solidRef/,
  `export type SolidRef = string & { readonly __brand: "SolidRef" };
  export type EditableEntityKind =
    | "anchor"
    | "vertex"
    | "edge"
    | "wire"
    | "face"
    | "shell"
    | "solid";
  export function solidRef`,
);

text = text.replace(
  /type CellSolid = kernelGeometry\.CellSolid;\ntype CellRecord = kernelGeometry\.CellRecord;\ntype CellComplexRecord = kernelGeometry\.CellComplexRecord;\ntype ClusterRecord = kernelGeometry\.ClusterRecord;/,
  "type SolidPrimitive = kernelGeometry.SolidPrimitive;\ntype SolidRecord = kernelGeometry.SolidRecord;",
);

// CellSolid → SolidPrimitive in second namespace if still present
text = text.replace(
  /\/\*\* @emoji 🧊️ Analytic cell solid[\s\S]*?export interface CellRecord \{[\s\S]*?readonly solid\?: CellSolid;\n  \}/,
  `/** @emoji 🧊️ Analytic solid primitive (\`BRepPrimAPI\` / \`Geom\` brepjs payload). */
  export type SolidPrimitive =
    | { readonly kind: "box"; readonly cornerA: Vec3; readonly cornerB: Vec3; readonly height: number }
    | { readonly kind: "sphere"; readonly center: Vec3; readonly radius: number }
    | { readonly kind: "cylinder"; readonly base: Vec3; readonly axis: Vec3; readonly radius: number; readonly height: number }
    | { readonly kind: "cone"; readonly base: Vec3; readonly axis: Vec3; readonly radius: number; readonly height: number; readonly radiusTop?: number };

  /** @emoji 🧱️ Solid payload: bounded volume via closed shells and/or analytic primitive. */
  export interface SolidRecord {
    readonly id: SolidRef;
    readonly shellIds: readonly ShellRef[];
    readonly solid?: SolidPrimitive;
  }`,
);

text = text.replace(/readonly cells: readonly CellRecord\[\];\n    readonly cellComplexes: readonly CellComplexRecord\[\];\n    readonly clusters: readonly ClusterRecord\[\];/, "readonly solids: readonly SolidRecord[];");

writeFileSync(file, text);
