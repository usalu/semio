import { readFile } from "node:fs/promises";
import { resolve } from "node:path";

const path = resolve(process.argv[2] ?? "🌎️hub/🧩️compositions/🌍️gis/🧩️component/🔣️.json");
const { packageId, manifest } = JSON.parse(await readFile(path, "utf8"));
const terrainKind = "s.gis.gisterrain";
const errors = [];
const check = (condition, failure) => { if (!condition) errors.push(failure); };
const editor = manifest?.apps?.find((app) => app.id === `${terrainKind}@1/*#editor`);
const viewer = manifest?.apps?.find((app) => app.id === `${terrainKind}@1/*#viewer`);
check(packageId === "semio:gis" && manifest?.pluginId === "gis", "The descriptor belongs to the GIS plugin");
check(editor?.role === "editor" && editor.dialect?.artifactKind === terrainKind, "A GIS Terrain editor is registered");
check(viewer?.role === "viewer" && viewer.dialect?.artifactKind === terrainKind, "A GIS Terrain viewer is registered");

const actions = new Map((editor?.actions ?? []).map((action) => [action.id, action]));
const checkLocalized = (binding, name) => {
  for (const context of ["native", "reuse"]) {
    for (const language of ["en", "de"]) {
      check(Boolean(binding?.label?.[context]?.[language]?.trim()), `${name} has a ${language} ${context} label`);
    }
  }
};
for (const id of ["createElevationGrid", "editElevation", "gradeTerrainRegion", "generateTerrainContours",
  "addOverlayPosition", "moveOverlayPosition", "editOverlayShape", "removeOverlayFeature", "clearOverlays"]) {
  check(actions.get(id)?.kind === "mutation", `The editor declares the ${id} document mutation`);
  checkLocalized(actions.get(id), id);
}
for (const id of ["importElevationGrid", "exportElevationGrid", "importGeoJson", "exportGeoJson"]) {
  check(actions.get(id)?.kind === "shell", `The editor declares the ${id} file action`);
  checkLocalized(actions.get(id), id);
}
const settings = actions.get("configureTerrainBrush");
check(settings?.kind === "view", "The editor offers window-scoped brush settings");
for (const id of ["radius", "smoothStrength"]) {
  check(settings?.args?.some((argument) => argument.id === id && argument.schema?.integer === true),
    `Brush settings declare an integer ${id}`);
}
const elevationStep = settings?.args?.find((argument) => argument.id === "elevationStep");
check(elevationStep?.schema?.step === 0.001 && elevationStep.schema?.min === 0.001 && !elevationStep.schema?.integer,
  "Brush settings accept exact millimetre steps");
const fillMissing = actions.get("editElevation")?.args?.find((argument) => argument.id === "operation")
  ?.schema?.options?.find((option) => option.value === "fill-missing");
check(Boolean(fillMissing), "The editor declares Fill Missing Samples in Edit Elevation");
checkLocalized(fillMissing, "Fill Missing Samples");
for (const id of ["column", "row", "radius"]) {
  check(actions.get("editElevation")?.args?.some((argument) => argument.id === id && argument.schema?.integer && argument.schema?.max === 64),
    `Elevation editing admits the full supported ${id} range`);
}
for (const id of ["columns", "rows"]) {
  check(actions.get("createElevationGrid")?.args?.some((argument) => argument.id === id && argument.schema?.min === 2 && argument.schema?.max === 65),
    `Grid creation declares independent ${id}`);
}

const terrainWindow = editor?.windowKinds?.find((window) => window.id === "gis3d-main");
check(terrainWindow?.surfaceKind === "world-3d", "The terrain window declares the World3d surface");
checkLocalized(terrainWindow, "Terrain viewport");
for (const id of ["terrainRaise", "terrainLower", "terrainSmooth", "terrainMovePosition"]) {
  const utility = editor?.utilities?.find((utility) => utility.id === id);
  check(Boolean(utility), `The editor publishes the ${id} utility`);
  checkLocalized(utility, id);
}
check(!viewer?.utilities?.some((utility) => utility.id === "terrainMovePosition"), "The read-only Viewer does not publish Move Position");
const terrainData = editor?.panelTabs?.find((panel) => panel.bodyKey === "gis3d.terrain.data");
check(Boolean(terrainData), "The editor publishes the Terrain Data panel");
checkLocalized(terrainData, "Terrain Data panel");

const ports = new Map((editor?.io?.ports ?? []).map((port) => [port.id, port]));
for (const [id, direction, type, form] of [
  ["map:in", "in", "twoD", "vector"],
  ["elevation:in", "in", "data", "value"],
  ["elevation:out", "out", "data", "value"],
  ["scene:out", "out", "threeD", "mesh"],
]) {
  const port = ports.get(id);
  check(port?.direction === direction && port.mediaType?.class === type && port.mediaType?.form === form,
    `The editor declares the typed ${id} port`);
}

const demo = manifest?.examples?.find((example) => example.id === "demo" && example.dialect?.artifactKind === terrainKind);
const artifact = demo?.artifactJson ?? "";
check(/^semio gis\.gisterrain\.dsl v1\r?\n/u.test(artifact), "The example carries the Terrain artifact DSL");
const elevation = artifact.match(/(?:^|\s)elevation-source=bounds=([^\s]+)\s+rows-millimetres=\s*\[((?:\s*\[[^\[\]]*\]){33})\s*\]/mu);
const samples = elevation && [...elevation[2].matchAll(/\[([^\[\]]*)\]/gu)]
  .map(([, row]) => row.trim().split(/\s+/u).map(Number));
const asset = JSON.parse(await readFile(new URL("../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🏔️elevation.json", import.meta.url), "utf8"));
check(JSON.stringify(samples) === JSON.stringify(asset.rowsMillimetres), "The example preserves every signed Liège sample in canonical order");
check(JSON.stringify(elevation?.[1].split(",").map(Number)) === JSON.stringify(asset.bounds),
  "The example preserves the Liège WGS84 sample extent");
for (const id of ["p_institut_de_botanique_ulg_liege", "p_lycee_block_3000", "r_sart_tilman", "g_sart_tilman"]) {
  check(artifact.includes(id), `The example contains ${id}`);
}

if (errors.length) {
  console.error(`[DEBUG] ${path} failed ${errors.length} Terrain descriptor checks`);
  for (const error of errors) console.error(`[DEBUG] ${error}`);
  process.exitCode = 1;
} else {
  console.log(JSON.stringify({ path, editor: editor.id, viewer: viewer.id, demo: demo.id, ports: [...ports.keys()] }));
}
