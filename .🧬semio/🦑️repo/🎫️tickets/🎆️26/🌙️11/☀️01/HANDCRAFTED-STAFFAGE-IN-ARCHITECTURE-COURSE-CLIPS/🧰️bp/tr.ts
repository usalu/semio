/** 💨️ bp: transport numbers of manifest casts: tr.ts <cast> ... (env BP_TIMES="1,3,5") */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules";
const V = "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json";
const load = (path: string) => import(pathToFileURL(resolve(S, path)).href);
const { loadCatalogFromFiles, loadEnvironments } = await load("🗂️catalog/📥️files/🟦️.ts");
const { solveTransport } = await load("💨️transport/🟦️.ts");

const manifest = JSON.parse(readFileSync(V, "utf8")).casts;
const catalog = await loadCatalogFromFiles();
const presets = new Map(((await loadEnvironments())?.presets ?? []).map((p: any) => [p.id, p.environment]));
const times = (process.env.BP_TIMES ?? "1,3,5").split(",").map(Number);

for (const id of process.argv.slice(2)) {
  const cast = manifest[id];
  const asset = catalog.assets.find((a: any) => a.id === cast.asset);
  const environment = presets.get(process.env.BP_ENV ?? cast.environment);
  const clip = cast.clip ? asset.clips.find((c: any) => c.id === cast.clip) : undefined;
  console.log(`== ${id} ${cast.asset} ${cast.clip ?? cast.pose} env=${cast.environment} airC=${environment?.airTempC} rh=${environment?.relativeHumidity}`);
  for (const t of times) {
    const query = clip ? { kind: "clip", clip: clip.id, timeS: t } : { kind: "pose", pose: cast.pose, timeS: t };
    const f = solveTransport(asset, query, environment);
    const by = new Map<string, any[]>();
    for (const p of f.parcels) (by.get(p.emitter) ?? by.set(p.emitter, []).get(p.emitter)!).push(p);
    console.log(`  t=${t} parcels=${f.parcels.length} pathlines=${f.pathlines.length} cond=${f.condensation.length} resp=${f.respiration ? JSON.stringify({ flowLs: (f.respiration.flowM3S * 1000).toFixed(3), vol: f.respiration.lungVolumeL ?? f.respiration.lungVolumeM3, rate: f.respiration.rateBpm }) : null}`);
    for (const e of f.emitters) {
      const ps = by.get(e.emitter) ?? [];
      const up = ps.length ? ps.map((p) => p.velocity.y) : [];
      const sp = ps.map((p) => Math.hypot(p.velocity.x, p.velocity.y));
      const ys = ps.map((p) => p.at.y), xs = ps.map((p) => p.at.x);
      const rng = (a: number[]) => (a.length ? `${Math.min(...a).toFixed(2)}..${Math.max(...a).toFixed(2)}` : "-");
      const maxEx = ps.length ? Math.max(...ps.map((p) => p.excessK)).toFixed(2) : "-";
      console.log(`    ${e.emitter} kind=${e.kind} at=(${e.at.x.toFixed(2)},${e.at.y.toFixed(2)}) dir=${((e.directionRad * 180) / Math.PI).toFixed(0)} flowLs=${(e.volumeFlowM3S * 1000).toFixed(3)} exit=${e.exitVelocityMS.toFixed(2)} T=${e.temperatureC.toFixed(1)} active=${e.active} n=${ps.length} speed=${rng(sp)} vy=${rng(up)} x=${rng(xs)} y=${rng(ys)} maxExcess=${maxEx}`);
    }
  }
}
