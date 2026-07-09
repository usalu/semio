#!/usr/bin/env node
/** 🧪 Load rebuilt forms plugins and assert manifest + initial document + module preview wiring. */

import { join } from "node:path";
import { pathToFileURL } from "node:url";

const repoRoot = join(import.meta.dirname, "../../../../../..");
const pluginRoot = join(repoRoot, "framework/product/os/dev/plugin-modules");

async function loadPlugin(pluginId, jsName) {
  const moduleUrl = pathToFileURL(join(pluginRoot, pluginId, `${jsName}.js`)).href;
  const mod = await import(moduleUrl);
  const api = await mod.createPluginApi();
  const manifest = JSON.parse(await api.manifest());
  return { api, manifest };
}

function buildContributionsJson(manifest, pluginId) {
  return JSON.stringify((manifest.contributions ?? []).map((contribution) => ({ pluginId, contribution })));
}

const procedural = await loadPlugin("forms-module-procedural", "forms_module_procedural");
const proceduralApps = procedural.manifest.apps ?? [];
console.log("[DEBUG] forms-module-procedural apps", proceduralApps.map((app) => app.id));
if (proceduralApps.length !== 1) throw new Error("expected one forms-module-procedural app");
if (!proceduralApps[0].windowKinds?.length) throw new Error("forms-module-procedural app must declare window kinds");
console.log("[DEBUG] forms-module-procedural windowKinds", proceduralApps[0].windowKinds.map((kind) => kind.id));

const contributions = procedural.manifest.contributions ?? [];
const building = contributions.find((entry) => entry.question_kind === "buildingComponent" || entry.questionKind === "buildingComponent");
if (!building) throw new Error("buildingComponent contribution missing");
console.log("[DEBUG] forms-module-procedural contribution ok");

const instanceId = await procedural.api.createApp("forms-module-procedural");
const payload = JSON.stringify({
  fixtureSlug: "hexagonal-mushroom-column",
  params: { height: 6, radius: 0.5, sides: 6 },
  questionId: "geometry",
  controllerId: "forms-play",
  surface: "try",
  interactive: true,
});
const preview = JSON.parse(await procedural.api.renderWithDocument(instanceId, "preview", "{}", payload));
if (!preview || preview.type !== "componentScene") throw new Error(`preview render expected componentScene got ${preview?.type}`);
console.log("[DEBUG] forms-module-procedural preview render ok", preview.componentKind);

const params = JSON.parse(await procedural.api.renderWithDocument(instanceId, "params", "{}", payload));
if (!params || params.type !== "stack") throw new Error(`params render expected stack got ${params?.type}`);
console.log("[DEBUG] forms-module-procedural params render ok children", params.children?.length ?? 0);

const forms = await loadPlugin("forms", "forms_plugin");
const formsApps = forms.manifest.apps ?? [];
const formsPlay = formsApps.find((app) => app.id === "forms-play");
if (!formsPlay) throw new Error("forms-play app missing");
const formsInstance = await forms.api.createApp("forms-play");
const contributionsJson = buildContributionsJson(procedural.manifest, "forms-module-procedural");
const viewState = { contributionsJson };
const context = JSON.stringify({ viewState, actor: "local" });
const blueprint = JSON.parse(await forms.api.render(formsInstance, "forms.play.blueprint", "{}"));
if (!JSON.stringify(blueprint).includes("table") && blueprint.type !== "componentScene") {
  throw new Error("forms-play blueprint must render edit table");
}
const tryStep1 = JSON.parse(await forms.api.render(formsInstance, "forms.play.try", JSON.stringify(viewState)));
if (!JSON.stringify(tryStep1).includes("Component Name")) {
  throw new Error("forms-play fresh document must seed Building Component step 1");
}
await forms.api.handleCommand(formsInstance, JSON.stringify({ command: "nextStep" }), context);
const tryStep2 = JSON.parse(await forms.api.render(formsInstance, "forms.play.try", JSON.stringify(viewState)));
const tryJson = JSON.stringify(tryStep2);
if (!tryJson.includes("externalSlot") || !tryJson.includes("forms-module-procedural")) {
  throw new Error("forms-play step 2 must emit forms-module-procedural external slots");
}
console.log("[DEBUG] forms-play seeded fixture + extension slots ok");

async function verifyGenerate(appId, pluginId, jsName, previewBody) {
  const loaded = await loadPlugin(pluginId, jsName);
  const app = loaded.manifest.apps.find((entry) => entry.id === appId);
  if (!app?.modes?.some((mode) => mode.id === "generate")) throw new Error(`${appId} missing generate mode`);
  const fi = await loaded.api.createApp(appId);
  const ctx = JSON.stringify({ viewState: {}, actor: "local" });
  const add = JSON.parse(await loaded.api.handleCommand(fi, JSON.stringify({ command: "addGeneration" }), ctx));
  if (!add.operations?.length) throw new Error(`${appId} addGeneration must emit patch ops`);
  const rename = JSON.parse(
    await loaded.api.handleCommand(fi, JSON.stringify({ command: "renameGeneration", args: { name: "Verify Gen" } }), ctx),
  );
  if (!rename.operations?.length) throw new Error(`${appId} renameGeneration must emit patch ops`);
  const preview = JSON.parse(await loaded.api.render(fi, previewBody, "{}"));
  if (preview.type !== "componentScene" && preview.type !== "stack") {
    throw new Error(`${appId} generate preview expected componentScene/stack got ${preview.type}`);
  }
  const remove = JSON.parse(await loaded.api.handleCommand(fi, JSON.stringify({ command: "removeGeneration" }), ctx));
  if (!remove.operations?.length) throw new Error(`${appId} removeGeneration must emit patch ops`);
  console.log("[DEBUG] generate mode ok", appId);
}

await verifyGenerate("flow-play", "flow", "flow_plugin", "flow.play.generate-preview");
await verifyGenerate("procedural2d-play", "procedural", "procedural_plugin", "procedural2d.play.generate-preview");
await verifyGenerate("procedural3d-play", "procedural", "procedural_plugin", "procedural.play.generate-preview");

console.log("[DEBUG] plugin-runtime-check ok");
