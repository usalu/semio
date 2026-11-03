import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules";
const V = "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json";
const load = (path: string) => import(pathToFileURL(resolve(S, path)).href);
const { loadCatalogFromFiles, loadEnvironments } = await load("🗂️catalog/📥️files/🟦️.ts");
const { solveThermalAt, thermalSession } = await load("🌡️thermal/🟦️.ts");
const { evaluate } = await load("🧮️evaluation/🟦️.ts");
const { solveTransport } = await load("💨️transport/🟦️.ts");
const casts = JSON.parse(readFileSync(V, "utf8")).casts;
const catalog = await loadCatalogFromFiles();
const presets = new Map(((await loadEnvironments())?.presets ?? []).map((p: any) => [p.id, p.environment]));
const want = process.argv.slice(2);
const f1 = (x: number) => x.toFixed(1);
for (const [id, cast] of Object.entries<any>(casts)) {
  if (cast.view !== "thermal" || (want.length && !want.some((w) => id.includes(w)))) continue;
  const asset = catalog.assets.find((a: any) => a.id === cast.asset);
  const env = presets.get(cast.environment);
  const session = thermalSession();
  let times = [0];
  let mk = (t: number) => ({ kind: "pose", pose: cast.pose, timeS: t });
  if (cast.clip) {
    const clip = asset.clips.find((c: any) => c.id === cast.clip);
    const d = clip.durationS;
    times = [0, 0.1, 0.25, 0.5, 0.75, 1].map((f) => f * d);
    mk = (t: number) => ({ kind: "clip", clip: cast.clip, timeS: t });
  }
  console.log(`\n## ${id} (${cast.asset} ${cast.clip ?? cast.pose}) air=${env?.airTempC}`);
  for (const t of times) {
    const q = mk(t);
    const field = solveThermalAt(asset, q, env, { session });
    const rs = Object.values<any>(field.regions).map((r) => `${r.region}:${f1(r.surfaceTempC)}`);
    const b = field.body ? ` body core ${f1(field.body.coreTempC)} skin ${f1(field.body.meanSkinTempC)} met ${f1(field.body.metabolicW)}W resp ${f1(field.body.respiratorySensibleW)}+${f1(field.body.respiratoryLatentW)}W` : "";
    const ph = Object.values<any>(field.regions).filter((r) => r.phaseChangeW > 0).map((r) => `${r.region} phase ${f1(r.phaseChangeW)}W`);
    console.log(` t=${t.toFixed(1)} tot sens ${f1(field.totals.sensibleW)} lat ${f1(field.totals.latentW)} em ${f1(field.totals.emitterSensibleW)}${b} | ${rs.join(" ")} ${ph.join(" ")}`);
  }
}
