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
	createDrawShapeLayer,
	createDrawTextLayer,
	createDrawImageLayer,
	defaultDrawDocument,
} = await import(coreUrl);

function selectionMergeIds(mode, current, incoming) {
	const currentSet = new Set(current);
	const incomingSet = new Set(incoming);
	if (mode === "default") return [...incomingSet];
	if (mode === "additive") {
		for (const id of incomingSet) currentSet.add(id);
		return [...currentSet];
	}
	if (mode === "subtractive") {
		for (const id of incomingSet) currentSet.delete(id);
		return [...currentSet];
	}
	for (const id of incomingSet) {
		if (currentSet.has(id)) currentSet.delete(id);
		else currentSet.add(id);
	}
	return [...currentSet];
}

function marqueeModeFromModifiers(modifiers) {
	const shift = modifiers.shiftKey === true;
	const ctrl = modifiers.ctrlKey === true || modifiers.metaKey === true;
	if (shift && ctrl) return "invertive";
	if (shift) return "additive";
	if (ctrl) return "subtractive";
	return "default";
}

const fixtureJson = readFileSync(join(root, "draw/fixture/semio.draw.json"), "utf8");
const doc = parseDrawDocument(JSON.parse(fixtureJson));
console.log("[DEBUG] runtime-check fixture layers", doc.layers.length);
if (doc.layers.length !== 1 || doc.layers[0]?.id !== "emblem-group") throw new Error("semio fixture must contain only the authored emblem group");

const nodes = flattenDrawDocumentToSceneNodes(doc);
console.log("[DEBUG] runtime-check emblem nodes", nodes.map((node) => node.id));
if (nodes.some((node) => node.needsKernel) || nodes.map((node) => node.id).join() !== "emblem-orange,emblem-red,emblem-teal") {
	throw new Error("semio fixture must render only the three authored emblem paths");
}

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

const shape = createDrawShapeLayer("R", { shapeKind: "rect", rect: { x: 0, y: 0, width: 10, height: 10 } });
const text = createDrawTextLayer();
const image = createDrawImageLayer("I", "key", 100, 80);
const richDoc = applyDrawEditOp(
	applyDrawEditOp(
		applyDrawEditOp(defaultDrawDocument("rich"), { op: "addShapeLayer", layer: shape }),
		{ op: "addTextLayer", layer: text },
	),
	{ op: "addImageLayer", layer: image },
);
const richNodes = flattenDrawDocumentToSceneNodes(richDoc);
if (!richNodes.some((n) => n.text) || !richNodes.some((n) => n.image)) throw new Error("text/image flatten failed");

const mergedDefault = selectionMergeIds("default", ["a"], ["b"]);
const mergedAdd = selectionMergeIds("additive", ["a"], ["b"]);
const mergedSubtract = selectionMergeIds("subtractive", ["a", "b"], ["b"]);
const mergedInvert = selectionMergeIds("invertive", ["a"], ["b"]);
if (mergedDefault.join() !== "b" || !mergedAdd.includes("b") || mergedSubtract.join() !== "a" || mergedInvert.join() !== "a,b") {
	throw new Error("selectionMergeIds modes failed");
}
if (marqueeModeFromModifiers({ shiftKey: true, ctrlKey: true }) !== "invertive") throw new Error("marqueeModeFromModifiers failed");

const committed = applyDrawEditOp(defaultDrawDocument("commit"), {
	op: "addShapeLayer",
	layer: createDrawShapeLayer("Drawn", { shapeKind: "rect", rect: { x: 1, y: 2, width: 3, height: 4 } }),
});
if (committed.layers.length < 2) throw new Error("commit-style shape add failed");

console.log("[DEBUG] runtime-check ok");
