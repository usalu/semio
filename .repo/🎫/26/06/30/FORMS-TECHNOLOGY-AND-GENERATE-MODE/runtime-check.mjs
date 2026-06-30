import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "../../../../../..");
const coreUrl = pathToFileURL(join(root, "forms/core/index.ts")).href;
const {
	formsExtensionHost,
	parseFormSpec,
	flowFixtureToFormSpec,
	applyGenerationValuesToFixture,
	registerFormsFlowFixtureResolver,
	defaultValueForQuestion,
} = await import(coreUrl);

const fixtureJson = readFileSync(join(root, "forms/fixture/building-component.forms.json"), "utf8");
const flowJson = readFileSync(join(root, "procedural/3d/fixture/hexagonal-mushroom-column.procedural.json"), "utf8");

registerFormsFlowFixtureResolver("hexagonal-mushroom-column", () => flowJson);
await formsExtensionHost.activateDefaults();

const spec = parseFormSpec(JSON.parse(fixtureJson));
const geometry = spec.steps[0]?.questions.find((question) => question.kind === "buildingComponent");
if (!geometry) throw new Error("buildingComponent question missing");

const defaults = defaultValueForQuestion(geometry);
console.log("[DEBUG] runtime-check defaults keys", Object.keys(defaults ?? {}));

const paramSpec = flowFixtureToFormSpec(flowJson);
const sliderId = paramSpec.steps[0]?.questions.find((question) => question.kind === "slider")?.id;
if (!sliderId) throw new Error("slider param missing");

const patched = applyGenerationValuesToFixture(flowJson, { ...defaults, [sliderId]: 8 });
const reparsed = JSON.parse(patched);
const slider = reparsed.widgets.find((widget) => widget.id === sliderId);
console.log("[DEBUG] runtime-check slider value", slider?.value);
if (slider?.value !== 8) throw new Error("applyGenerationValuesToFixture failed");

console.log("[DEBUG] runtime-check ok");
