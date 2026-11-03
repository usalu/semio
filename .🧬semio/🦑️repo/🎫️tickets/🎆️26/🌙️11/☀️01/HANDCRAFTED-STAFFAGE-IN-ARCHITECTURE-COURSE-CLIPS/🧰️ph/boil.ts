/** 🫖 ph: kettle water temperature, evaporation and steam over the boil clip: boil.ts [clip] */
import { pathToFileURL } from "node:url";
import { resolve } from "node:path";

const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules";
const load = (path: string) => import(pathToFileURL(resolve(S, path)).href);
const { loadCatalogFromFiles, loadEnvironments } = await load("🗂️catalog/📥️files/🟦️.ts");
const { solveThermalAt } = await load("🌡️thermal/🟦️.ts");

const catalog = await loadCatalogFromFiles();
const presets = new Map(((await loadEnvironments())?.presets ?? []).map((p: any) => [p.id, p.environment]));
const asset = catalog.assets.find((a: any) => a.id === (process.env.PH_ASSET ?? "electric-kettle"));
const environment = presets.get(process.env.PH_ENV ?? "office-neutral");
const times = (process.env.PH_TIMES ?? "0,60,120,160,170,175,180,185,190,195,199,200").split(",").map(Number);
for (const timeS of times) {
  const f = solveThermalAt(asset, { kind: "clip", clip: process.env.PH_CLIP ?? "boil", timeS }, environment);
  const w = f.regions[process.env.PH_REGION ?? "water"];
  console.log(`t=${timeS} T=${w.surfaceTempC.toFixed(2)} steady=${w.steadyTempC.toFixed(1)} phaseW=${w.phaseChangeW.toFixed(1)} latentW=${w.latentW.toFixed(1)} evap=${(w.evaporationKgS * 1e3).toFixed(3)} g/s boiled=${(w.phaseChangedKg * 1e3).toFixed(1)} g`);
}
