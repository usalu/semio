/** @emoji 🔄 Bulk rename topology vocabulary to model across spatial/js. */
import { readdirSync, readFileSync, writeFileSync, statSync } from "node:fs";
import { join } from "node:path";

const root = "c:/git/semio/spatial/js";
const pairs: [string, string][] = [
	["TopologyGraphJson", "ModelJson"],
	["TopologyGraph", "Model"],
	["TopologyDiff", "ModelDiff"],
	["parseTopologyGraphJson", "parseModelJson"],
	["applyTopologyDiff", "applyModelDiff"],
	["isEmptyTopologyDiff", "isEmptyModelDiff"],
	["EMPTY_TOPOLOGY_DIFF", "EMPTY_MODEL_DIFF"],
	["EntityMetadataStore", "AttributeStore"],
	["readTopologyEntityProperty", "readModelEntityProperty"],
	["TopologyEntityKind", "ModelEntityKind"],
	["TopologyEntityRef", "ModelEntityRef"],
	["spatial.topology/v1", "spatial.model/v1"],
	["small-building.topology.json", "small-building.model.json"],
	["tall-building.topology.json", "tall-building.model.json"],
	["large-building.topology.json", "large-building.model.json"],
	["../../assets/interaction/", "../../assets/extension/builtin/interaction/"],
	["topologyCellAabb", "modelObjectAabb"],
	["boxTopologyDiff", "boxModelDiff"],
	["meshFaceTopologyDiff", "meshFaceModelDiff"],
	["selfMergeTopologyDiff", "selfMergeModelDiff"],
	["computePartViewsFromTopology", "computePartViewsFromModel"],
	["computeSurfaceViewsFromTopology", "computeSurfaceViewsFromModel"],
	["computeVolumeViewsFromTopology", "computeVolumeViewsFromModel"],
	["parseTopologyEntityKinds", "parseModelEntityKinds"],
	["topology:", "model:"],
	["doc.topology", "doc.model"],
	["opts.document.topology", "opts.document.model"],
	["readonly topology:", "readonly model:"],
	["topology,", "model,"],
	["(topo:", "(model:"],
	[" topo:", " model:"],
	[" topo.", " model."],
	[" topo)", " model)"],
	[" topo,", " model,"],
	[" topo;", " model;"],
	[" topo\n", " model\n"],
	[" topo]", " model]"],
	["const topo =", "const model ="],
	["let topo =", "let model ="],
	[" topo =", " model ="],
	["(topo)", "(model)"],
	[" topo:", " model:"],
];

function walk(dir: string, out: string[] = []): string[] {
	for (const name of readdirSync(dir)) {
		const p = join(dir, name);
		if (statSync(p).isDirectory()) {
			if (name === "node_modules") continue;
			walk(p, out);
		} else if (/\.(ts|tsx|json)$/.test(name)) out.push(p);
	}
	return out;
}

for (const file of walk(root)) {
	let text = readFileSync(file, "utf8");
	let changed = false;
	for (const [from, to] of pairs) {
		if (text.includes(from)) {
			text = text.split(from).join(to);
			changed = true;
		}
	}
	if (changed) writeFileSync(file, text);
}
