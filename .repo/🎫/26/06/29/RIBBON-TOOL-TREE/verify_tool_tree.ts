import {
	hasInteractiveToolNodes,
	mergeAppTools,
	resolveDefaultToolPath,
	toolCollection,
	type AppTools,
} from "@semio-tech/framework-core";

const puzzle2dLike: AppTools = [
	toolCollection("selection", "mouse-pointer-2", [
		toolCollection("methods", "square", [{ id: "rect", kind: "toggle", iconId: "square", pressed: true }]),
		toolCollection("targets", "layers", [{ id: "nodes", kind: "toggle", iconId: "circle-dot", pressed: true }]),
	]),
	toolCollection("view", "layout-grid", [{ id: "grid", kind: "toggle", iconId: "grid-3x3", pressed: false }]),
];

const path = resolveDefaultToolPath(puzzle2dLike);
console.log("[DEBUG] default path:", path);
if (path.join("/") !== "selection") {
	throw new Error(`expected selection, got ${path.join("/")}`);
}

const merged = mergeAppTools(
	[toolCollection("selection", "mouse-pointer-2", [{ id: "a", kind: "button", iconId: "x" }])],
	[toolCollection("selection", "mouse-pointer-2", [{ id: "b", kind: "button", iconId: "y" }]), toolCollection("filter", "filter", [{ id: "c", kind: "toggle", iconId: "z", pressed: false }])],
);
const selection = merged?.find((node) => node.id === "selection");
if (!selection || selection.kind !== "collection" || selection.children.length !== 2) {
	throw new Error("merge failed for collection children");
}

if (!hasInteractiveToolNodes(puzzle2dLike)) {
	throw new Error("expected interactive tools");
}

console.log("[DEBUG] ribbon tool tree verification passed");
