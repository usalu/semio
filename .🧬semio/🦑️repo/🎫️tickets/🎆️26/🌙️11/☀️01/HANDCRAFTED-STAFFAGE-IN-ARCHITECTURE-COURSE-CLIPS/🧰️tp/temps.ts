/** 🌡️ tp: print region surface temperatures of manifest casts over their timeline: temps.ts <cast> [<cast> ...]  (env TP_FRACTIONS="0,0.1,0.25,0.5,1") */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules";
const V = "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json";
const load = (path: string) => import(pathToFileURL(resolve(S, path)).href);
const { loadCatalogFromFiles, loadEnvironments } = await load("🗂️catalog/📥️files/🟦️.ts");
const { solveThermalAt } = await load("🌡️thermal/🟦️.ts");
const { frameSchedule } = await load("📤️export/🎞️frames/🟦️.ts");

const manifest = JSON.parse(readFileSync(V, "utf8")).casts;
const catalog = await loadCatalogFromFiles();
const presets = new Map(((await loadEnvironments())?.presets ?? []).map((p: any) => [p.id, p.environment]));
const fractions = (process.env.TP_FRACTIONS ?? "0,0.05,0.1,0.25,0.5,1").split(",").map(Number);

for (const id of process.argv.slice(2)) {
  const cast = manifest[id];
  const asset = catalog.assets.find((a: any) => a.id === cast.asset);
  const environment = presets.get(cast.environment);
  const clip = cast.clip ? asset.clips.find((c: any) => c.id === cast.clip) : undefined;
  const duration = clip ? clip.durationS : 0;
  console.log(`== ${id} ${cast.asset} ${cast.clip ?? cast.pose} env=${cast.environment} airC=${environment?.airTempC} dur=${duration}`);
  for (const f of clip ? fractions : [0]) {
    const timeS = Math.min(duration * f, duration - 1e-6);
    const field = solveThermalAt(asset, clip ? { kind: "clip", clip: clip.id, timeS } : { kind: "pose", pose: cast.pose, timeS: 0 }, environment);
    const parts = Object.values(field.regions as Record<string, any>).map((r) => `${r.region}=${r.surfaceTempC.toFixed(1)}${Math.abs(r.steadyTempC - r.surfaceTempC) > 0.5 ? `(->${r.steadyTempC.toFixed(1)})` : ""}`);
    const body = field.body ? ` core=${field.body.coreTempC.toFixed(2)} skin=${field.body.meanSkinTempC.toFixed(1)} met=${field.body.metabolicW.toFixed(0)}W resp=${(field.body.respiratorySensibleW + field.body.respiratoryLatentW).toFixed(0)}W` : "";
    const t = field.totals;
    console.log(`  t=${timeS.toFixed(1)}s ${parts.join(" ")}${body} sens=${t.sensibleW.toFixed(0)} lat=${t.latentW.toFixed(0)} em=${t.emitterSensibleW.toFixed(0)}`);
  }
}
