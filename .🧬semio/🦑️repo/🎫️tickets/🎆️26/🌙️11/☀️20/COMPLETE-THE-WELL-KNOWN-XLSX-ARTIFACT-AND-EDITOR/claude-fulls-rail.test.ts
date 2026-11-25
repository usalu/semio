import {expect, test} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";

type Action = {id: string; args?: {id: string; default?: unknown}[]};
type App = {id: string; actions?: Action[]; windowKinds?: {actions?: Action[]}[]};
const root = "/Users/ueli/Documents/semio";
const component = JSON.parse(readFileSync(join(root, "🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/💼️office/🧩️component/🔣️.json"), "utf8")) as {manifest: {apps: App[]}};
const declared = (app: App): string[] => [...(app.actions ?? []), ...(app.windowKinds ?? []).flatMap(kind => kind.actions ?? [])].map(action => action.id);

/** 📂️ Open File is declared per editor app, so a host with no focused editor shows no `openArtifactFile` rail row: the journey spawns an editor first. */
test("every XLSX editor declares paired Open and Save, no XLSX viewer declares Open", () => {
  const xlsx = component.manifest.apps.filter(app => app.id.startsWith("s.stdio.xlsx@"));
  const editors = xlsx.filter(app => app.id.endsWith("#editor"));
  expect(editors.map(app => app.id).sort()).toEqual(["s.stdio.xlsx@ecma-376/*#editor", "s.stdio.xlsx@ecma-376/strict#editor", "s.stdio.xlsx@ecma-376/transitional#editor"]);
  for (const app of editors) expect(declared(app)).toEqual(expect.arrayContaining(["openArtifactFile", "saveArtifactFile"]));
  for (const app of xlsx.filter(candidate => candidate.id.endsWith("#viewer"))) expect(declared(app)).not.toContain("openArtifactFile");
});

test("the journey driver bootstraps an editor through the palette when the rail has no Open File row", () => {
  const driver = readFileSync(join(import.meta.dir, "claude-fulls-journey.ts"), "utf8");
  expect(driver).toContain("spawn.${program.pluginId}.${program.appId}");
  expect(driver).toMatch(/railHas\("openArtifactFile"\)[\s\S]{0,200}spawnEditor\(editorAppId\)/u);
});
