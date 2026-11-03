import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules";
const load = (path: string) => import(pathToFileURL(resolve(S, path)).href);
const { loadCatalogFromFiles, loadEnvironments } = await load("🗂️catalog/📥️files/🟦️.ts");
const { solveThermalAt, thermalSession } = await load("🌡️thermal/🟦️.ts");
const jobs = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const catalog = await loadCatalogFromFiles();
const presets = new Map(((await loadEnvironments())?.presets ?? []).map((p: any) => [p.id, p.environment]));
const f1 = (x: number) => x.toFixed(1);
for (const job of jobs) {
  const asset = structuredClone(catalog.assets.find((a: any) => a.id === job.asset));
  for (const patch of job.patch ?? []) {
    const region = asset.thermal.regions.find((r: any) => r.id === patch.region);
    Object.assign(region.operating ?? region, patch.set);
    if (patch.area) region.areaM2 = patch.area;
  }
  const env = presets.get(job.env);
  const session = thermalSession();
  console.log(`\n## ${job.label ?? job.asset} ${job.clip ?? job.pose} env=${job.env}`);
  for (const t of job.times ?? [0]) {
    const q = job.clip ? { kind: "clip", clip: job.clip, timeS: t } : { kind: "pose", pose: job.pose, timeS: t };
    const field = solveThermalAt(asset, q, env, { session });
    console.log(` t=${t} sens ${f1(field.totals.sensibleW)} | ` + Object.values<any>(field.regions).map((r) => `${r.region}:${f1(r.surfaceTempC)}(${f1(r.steadyTempC)})`).join(" "));
  }
}
