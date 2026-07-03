import {
	OsStore,
	createEmptyOsDocument,
	mergeOsProgramDefinition,
	osOutPort,
	isParameterPortId,
	resolveParameterValuesForInstance,
	applyParameterValuesToProjection,
} from "@semio-tech/framework-os-core";
import { parseSStudioDocument } from "@semio-tech/s-core";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const repoRoot = resolve(import.meta.dir, "../../../../../..");
const demoFixture = JSON.parse(readFileSync(resolve(repoRoot, "s/example/demo.s.json"), "utf8"));

const store = new OsStore(createEmptyOsDocument());
store.dispatch({ kind: "addParameter", type: "numeric", name: "Test Zoom" });
const parameter = store.projection().parameters[0]!;
store.dispatch({ kind: "patchParameter", parameterId: parameter.id, patch: { value: 7 } });

mergeOsProgramDefinition(
	"raster",
	{
		id: "raster",
		name: "Raster",
		apiVersion: "1",
		apps: [{ id: "raster", label: "Raster", controllerId: "raster-play", modes: [{ id: "edit", label: "Edit" }] }],
		createPlatformApi: () => ({}),
	},
	{
		raster: {
			inputs: [],
			outputs: [osOutPort("2d.raster")],
			sourceFormat: "raster.document",
			componentKind: "raster",
			modes: [{ id: "edit", label: "Edit" }],
			parameterFields: [{ fieldPath: "/brushSize", label: "Brush size", type: "numeric" }],
		},
	},
);
store.dispatch({ kind: "spawnAppInstance", programId: "raster", appId: "raster" });
const instance = store.projection().appInstances[0]!;
store.dispatch({
	kind: "bindParameterField",
	instanceId: instance.id,
	fieldPath: "/brushSize",
	parameterId: parameter.id,
});
const node = store.projection().mediaGraph.nodes[0]!;
const demo = parseSStudioDocument(demoFixture);
const demoRasterBinding = demo.vcs.initialProjection.parameterBindings[0];
const demoRasterNode = demo.vcs.initialProjection.mediaGraph.nodes.find((entry) => entry.instanceId === "app-raster-1");

console.log("[DEBUG] parameter value", store.projection().parameters[0]);
console.log("[DEBUG] parameter port present", node.inputs.some((port) => isParameterPortId(port.id)));
console.log("[DEBUG] resolved values", resolveParameterValuesForInstance(store.projection().parameterBindings, store.projection().parameters, instance.id));
console.log(
	"[DEBUG] json pointer override",
	applyParameterValuesToProjection({ brushSize: 1 }, store.projection().parameterBindings, store.projection().parameters, instance.id),
);
console.log("[DEBUG] demo fixture binding", demoRasterBinding);
console.log("[DEBUG] demo raster parameter port", demoRasterNode?.inputs.some((port) => isParameterPortId(port.id)));
