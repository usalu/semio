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
for (const id of ["radius", "elevationStep", "smoothStrength"]) {
  check(settings?.args?.some((argument) => argument.id === id && argument.schema?.integer === true),
    `Brush settings declare an integer ${id}`);
}

const terrainWindow = editor?.windowKinds?.find((window) => window.id === "gis3d-main");
check(terrainWindow?.surfaceKind === "world-3d", "The terrain window declares the World3d surface");
checkLocalized(terrainWindow, "Terrain viewport");
for (const id of ["terrainRaise", "terrainLower", "terrainSmooth"]) {
  const utility = editor?.utilities?.find((utility) => utility.id === id);
  check(Boolean(utility), `The editor publishes the ${id} utility`);
  checkLocalized(utility, id);
}
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
const elevation = artifact.match(/(?:^|\s)elevation-source=bounds=([^\s]+)\s+rows=\s*\[((?:\s*\[[^\[\]]*\]){33})\s*\]/mu);
const samples = elevation && [...elevation[2].matchAll(/\[([^\[\]]*)\]/gu)]
  .map(([, row]) => row.trim().split(/\s+/u).map(Number));
check(samples?.every((row) => row.length === 33 && row.every(Number.isInteger)), "The example carries a 33 by 33 authored elevation grid");
const heights = samples?.flat() ?? [];
check(Math.min(...heights) === 47 && Math.max(...heights) === 252, "The example preserves the Liège elevation range");
check(JSON.stringify(elevation?.[1].split(",").map(Number)) === JSON.stringify([5.555, 50.585, 5.608333333333333, 50.638333333333335]),
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
