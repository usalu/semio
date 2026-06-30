import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "../../../../../..");
const coreUrl = pathToFileURL(join(root, "draw/core/index.ts")).href;
const {
	parseDrawDocument,
	flattenDrawDocumentToSceneNodes,
	drawPlayLayersTreeRowId,
	drawPlayLayerIdFromTreeRowId,
	findDrawLayer,
	applyDrawEditOp,
} = await import(coreUrl);

const fixtureJson = readFileSync(join(root, "draw/fixture/semio.draw.json"), "utf8");
const doc = parseDrawDocument(JSON.parse(fixtureJson));
console.log("[DEBUG] runtime-check fixture layers", doc.layers.length);
if (doc.layers.length < 3) throw new Error("semio fixture missing layers");

const nodes = flattenDrawDocumentToSceneNodes(doc);
const booleanNode = nodes.find((node) => node.needsKernel && node.kernelKind === "boolean");
const traceNode = nodes.find((node) => node.needsKernel && node.kernelKind === "trace");
console.log("[DEBUG] runtime-check kernel nodes", { boolean: Boolean(booleanNode), trace: Boolean(traceNode) });
if (!booleanNode || !traceNode) throw new Error("semio fixture missing boolean or trace showcase layers");

const orange = findDrawLayer(doc, "emblem-orange");
if (!orange) throw new Error("emblem-orange layer missing");
const treeId = drawPlayLayersTreeRowId(orange);
const layerId = drawPlayLayerIdFromTreeRowId(treeId);
console.log("[DEBUG] runtime-check tree row mapping", { treeId, layerId });
if (layerId !== "emblem-orange") throw new Error("tree row mapping failed");

const toggled = applyDrawEditOp(doc, { op: "setLayerVisible", layerId: "emblem-orange", visible: false });
const group = toggled.layers.find((layer) => layer.kind === "group");
const hidden = group?.kind === "group" ? group.children.find((layer) => layer.id === "emblem-orange") : undefined;
console.log("[DEBUG] runtime-check visibility toggle", hidden?.visible);
if (hidden?.visible !== false) throw new Error("toggleLayerVisible failed");

console.log("[DEBUG] runtime-check ok");
