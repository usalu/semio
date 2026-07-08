import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "../../../../../..");
const coreUrl = pathToFileURL(join(root, "forms/core/index.ts")).href;
const proceduralUrl = pathToFileURL(join(root, "procedural/3d/react/index.tsx")).href;
const flowReactUrl = pathToFileURL(join(root, "flow/react/index.tsx")).href;

const { flowFixtureToFormSpec, applyGenerationValuesToFixture } = await import(coreUrl);
const { extractChannelPreviewItems, preferGeometryPreviewItems } = await import(proceduralUrl);
const { FlowOrchestratorClient } = await import(flowReactUrl);

const flowJson = readFileSync(join(root, "procedural/3d/fixture/hexagonal-mushroom-column.procedural.json"), "utf8");
const paramSpec = flowFixtureToFormSpec(flowJson);
console.log("[DEBUG] preview-check labels", paramSpec.steps[0]?.questions.map((question) => `${question.id}:${question.label}`));

const patched = applyGenerationValuesToFixture(flowJson, { height: 6, radius: 0.5, sides: 6 });
const client = new FlowOrchestratorClient();
await client.loadFixtureJson(patched);
const result = await client.evaluate();
const items = preferGeometryPreviewItems(extractChannelPreviewItems(result.outputsJson));
console.log("[DEBUG] preview-check items", items.map((item) => `${item.widgetId}:${item.port}:${item.kind}`));
if (!items.some((item) => item.kind === "geometry")) throw new Error("geometry preview missing");
console.log("[DEBUG] preview-check ok");
