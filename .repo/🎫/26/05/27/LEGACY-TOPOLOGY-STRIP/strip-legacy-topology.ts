/** @emoji 🧹 Strip cellComplex/cluster/shell picks; fix cell→solid rename damage in spatial packages. */
import { readFileSync, writeFileSync } from "node:fs";

const core = "c:/git/compose/spatial/js/core/index.ts";
let text = readFileSync(core, "utf8");

text = text.replace(
	/const MODEL_ENTITY_KINDS = new Set<string>\(\[[^\]]+\]\);/,
	'const MODEL_ENTITY_KINDS = new Set<string>(["anchor", "vertex", "edge", "wire", "face", "solid", "object", "geometry", "attribute"]);',
);

text = text.replace(
	/export type CellComplexRef = string & \{ readonly __brand: "CellComplexRef" \};\n  export type ClusterRef = string & \{ readonly __brand: "ClusterRef" \};\n  export type EditableEntityKind =[\s\S]*?export function solidRef\(id: string\): SolidRef \{\n    return id as SolidRef;\n  \}\n  \/\*\* @emoji 🪪 Brep worker cache key[\s\S]*?export type SolidRef = SolidRef;\n\n/,
	`export type GeometryEntityKind =
    | "anchor"
    | "vertex"
    | "edge"
    | "wire"
    | "face"
    | "solid";

  /** @emoji 🪪 Kernel-private brepjs sub-element kind for parametric picks. */
  export type EditableEntityKind = GeometryEntityKind;

  /** @emoji 🪪 Builds a branded \`SolidRef\` from an opaque id string. */
  export function solidRef(id: string): SolidRef {
    return id as SolidRef;
  }

`,
);

text = text.replace(
	/\n    readonly cellComplexes: readonly CellComplexRecord\[\];\n    readonly clusters: readonly ClusterRecord\[\];/,
	"",
);

text = text.replace(
	/type CellComplexRef = kernelGeometry\.CellComplexRef;\ntype ClusterRef = kernelGeometry\.ClusterRef;\ntype EditableEntityKind = kernelGeometry\.EditableEntityKind;\n\nexport const solidRef = kernelGeometry\.solidRef;\nexport const solidRef = kernelGeometry\.solidRef;\n\n\/\*\* @emoji 🧭 Selection kinds[\s\S]*?export type ModelEntityKind = EditableEntityKind \| "object";/,
	`type GeometryEntityKind = kernelGeometry.GeometryEntityKind;
type EditableEntityKind = kernelGeometry.EditableEntityKind;

export const solidRef = kernelGeometry.solidRef;

/** @emoji 🧭 Framework + brepjs sub-element selection kinds. */
export type ModelEntityKind = EditableEntityKind | "object" | "geometry" | "attribute";`,
);

text = text.replace(
	/export type CellComplexRecordDiff[\s\S]*?export type ClusterRecordDiff[\s\S]*?;\n\n/,
	"",
);

text = text.replace(
	/    case "solid":\n      return \(model\.solids\[id\][\s\S]*?return \(hit as unknown as Record<string, unknown>\)\[name\];\n    \}/,
	`    case "solid":
      return (model.solids[id] as unknown as Record<string, unknown> | undefined)?.[name];
    case "object": {
      const hit = opts?.views?.findObject(model, opts.activeViewId ?? null, id);
      if (!hit) return undefined;
      if (name === "id") return id;
      if (name === "label") return hit.label;
      if (name === "typologyId") return hit.typologyId;
      return (hit as unknown as Record<string, unknown>)[name];
    }
    case "geometry":
      return model.solids[id] || model.faces[id] || model.wires[id] || model.edges[id] || model.vertices[id]
        ? id
        : undefined;
    case "attribute":
      return meta?.get(id)?.[name];`,
);

text = text.replace(
	/const geoKeys = \["anchors", "vertices", "edges", "wires", "faces", "shells", "solids", "cellComplexes", "clusters"\] as const;\n  const geometry: Record<string, unknown> = r\.geometry && typeof r\.geometry === "object" \? \{ \.\.\.\(r\.geometry as Record<string, unknown>\) \} : \{\};\n  if \(!Array\.isArray\(geometry\.solids\) && Array\.isArray\(geometry\.solids\)\) geometry\.solids = geometry\.solids;/,
	`const geoKeys = ["anchors", "vertices", "edges", "wires", "faces", "shells", "solids"] as const;
  const geometry: Record<string, unknown> = r.geometry && typeof r.geometry === "object" ? { ...(r.geometry as Record<string, unknown>) } : {};
  if (!Array.isArray(geometry.solids) && Array.isArray((geometry as { cells?: unknown }).cells)) geometry.solids = (geometry as { cells: unknown[] }).cells;`,
);

text = text.replace(
	/    const cell = model\.cells\[row\.geometryRef\];/,
	"    const cell = model.solids[row.geometryRef];",
);

text = text.replace(
	/    \} else if \(kind === "cell"\) \{\n      const c = model\.solids\[id\];\n      if \(c\) for \(const s of c\.shellIds\) walk\("shell", s\);\n    \} else if \(kind === "cellComplex"\) \{\n      const cc = model\.cellComplexes\[id\];\n      if \(cc\) for \(const c of cc\.cellIds\) walk\("solid", c\);\n    \}/,
	`    } else if (kind === "solid") {
      const c = model.solids[id];
      if (c) for (const s of c.shellIds) walk("shell", s);
    } else if (kind === "geometry") {
      const c = model.solids[id];
      if (c) for (const s of c.shellIds) walk("shell", s);
    }`,
);

text = text.replace(
	/export const ALL_MODEL_SELECTION_KINDS: readonly ModelEntityKind\[\] = \[[^\]]+\];/,
	'export const ALL_MODEL_SELECTION_KINDS: readonly ModelEntityKind[] = ["anchor", "vertex", "edge", "wire", "face", "solid", "object", "geometry", "attribute"];',
);

text = text.replace(
	/      case "shell":\n        for \(const id of Object\.keys\(model\.shells\)\) push\(kind, id\);\n        break;\n      case "solid":/,
	`      case "solid":
        for (const id of Object.keys(model.solids)) push(kind, id);
        break;
      case "geometry":
        for (const id of Object.keys(model.solids)) push(kind, id);
        break;
      case "attribute":
        break;
      case "solid_dup_removed":`,
);

// undo botched solid_dup - fix collectGeometrySelectionTargets properly
text = text.replace(
	/      case "solid":\n        for \(const id of Object\.keys\(model\.solids\)\) push\(kind, id\);\n        break;\n      case "geometry":\n        for \(const id of Object\.keys\(model\.solids\)\) push\(kind, id\);\n        break;\n      case "attribute":\n        break;\n      case "solid_dup_removed":\n        for \(const id of Object\.keys\(model\.solids\)\) push\(kind, id\);\n        break;\n      case "cellComplex":[\s\S]*?case "cluster":[\s\S]*?break;\n/,
	`      case "solid":
        for (const id of Object.keys(model.solids)) push(kind, id);
        break;
      case "geometry":
        for (const id of Object.keys(model.solids)) push(kind, id);
        break;
      case "attribute":
        break;
`,
);

text = text.replace(
	/  \{ id: "selection\.selectShells"[\s\S]*?kinds: \["shell"\] \},\n  \{ id: "selection\.selectCells"[\s\S]*?kinds: \["cell"\] \},\n  \{ id: "selection\.selectCellComplexes"[\s\S]*?kinds: \["cellComplex"\] \},\n  \{ id: "selection\.selectClusters"[\s\S]*?kinds: \["cluster"\] \},\n/,
	'  { id: "selection.selectSolids", label: "SelectSolids", key: "xc", operation: "selectKinds", kinds: ["solid"] },\n  { id: "selection.selectGeometries", label: "SelectGeometries", key: "xg", operation: "selectKinds", kinds: ["geometry"] },\n',
);

text = text.replace(/t\.kind === "cell"/g, 't.kind === "solid"');
text = text.replace(/kind === "cell"/g, 'kind === "solid"');
text = text.replace(/kind === "surface" \|\| kind === "part"/g, "false");
text = text.replace(
	/if \(defn\.id === "selection\.selectAnchors" \|\| defn\.id === "selection\.selectCellComplexes" \|\| defn\.id === "selection\.selectClusters"\)/,
	'if (defn.id === "selection.selectAnchors")',
);

writeFileSync(core, text);

const fixtureFiles = [
	"c:/git/compose/spatial/fixtures/simple.spatial.json",
	"c:/git/compose/spatial/fixtures/spatial.spatial.json",
	"c:/git/compose/spatial/fixtures/small-building.model.json",
	"c:/git/compose/spatial/fixtures/tall-building.model.json",
	"c:/git/compose/spatial/fixtures/large-building.model.json",
	"c:/git/compose/spatial/fixtures/geometry.json",
	"c:/git/compose/spatial/fixtures/geometry-loom.json",
	"c:/git/compose/spatial/fixtures/geometry-routes.json",
];
for (const f of fixtureFiles) {
	let j = readFileSync(f, "utf8");
	j = j.split('"cells"').join('"solids"');
	j = j.split("brepjs-cell-").join("brepjs-solid-");
	j = j.split('"cellComplexes"').join('"__removed_cellComplexes"');
	j = j.split('"clusters"').join('"__removed_clusters"');
	writeFileSync(f, j);
}

const schema = "c:/git/compose/spatial/schema/json/model.json";
let schemaText = readFileSync(schema, "utf8");
schemaText = schemaText.replace(
	/"required": \["anchors", "vertices", "edges", "wires", "faces", "shells", "cells", "cellComplexes", "clusters"\]/,
	'"required": ["anchors", "vertices", "edges", "wires", "faces", "shells", "solids"]',
);
schemaText = schemaText.replace(/"cells": \{ "type": "array", "items": \{ "\$ref": "#\/\$defs\/cell" \} \},/g, "");
schemaText = schemaText.replace(
	/"cellComplexes": \{ "type": "array", "items": \{ "\$ref": "#\/\$defs\/cellComplex" \} \},\s*"clusters": \{ "type": "array", "items": \{ "\$ref": "#\/\$defs\/cluster" \} \},/,
	'"solids": { "type": "array", "items": { "$ref": "#/$defs/solid" } },',
);
schemaText = schemaText.replace(/"cell"/g, '"solid"');
schemaText = schemaText.replace(/Cell/g, "Solid");
writeFileSync(schema, schemaText);

const interactionSchema = "c:/git/compose/spatial/schema/json/interaction.json";
let is = readFileSync(interactionSchema, "utf8");
is = is.replace(
	/"enum": \[[\s\S]*?"anchor"\s*\]/,
	`"enum": ["vertex", "edge", "wire", "face", "solid", "anchor"]`,
);
writeFileSync(interactionSchema, is);

console.log("strip-legacy-topology done");
