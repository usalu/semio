import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules";
const V = "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json";
const load = (path: string) => import(pathToFileURL(resolve(S, path)).href);
const { loadCatalogFromFiles, loadEnvironments } = await load("🗂️catalog/📥️files/🟦️.ts");
const { solveTransport } = await load("💨️transport/🟦️.ts");
const casts = JSON.parse(readFileSync(V, "utf8")).casts;
const catalog = await loadCatalogFromFiles();
const presets = new Map(((await loadEnvironments())?.presets ?? []).map((p: any) => [p.id, p.environment]));
for (const [id, cast] of Object.entries<any>(casts)) {
  if (!cast.transport) continue;
  const asset = catalog.assets.find((a: any) => a.id === cast.asset);
  const env = presets.get(cast.environment);
  const clip = asset.clips.find((c: any) => c.id === cast.clip);
  console.log(`\n## ${id} age=${cast.transportAgeS} trail=${cast.transportTrailS} stroke=${cast.transportStrokeMm} clipDur=${clip.durationS}`);
  for (const f of [0, 0.25, 0.5, 0.75]) {
    const t = (cast.transportAgeS ?? 0) + f * clip.durationS;
    const field = solveTransport(asset, { kind: "clip", clip: cast.clip, timeS: t }, env, { maxAgeS: cast.transportAgeS, periodS: clip.loop ? clip.durationS : undefined });
    const sp = field.parcels.map((p: any) => Math.hypot(p.velocity.x, p.velocity.y));
    const ys = field.parcels.map((p: any) => p.at.y);
    const byEm: Record<string, number[]> = {};
    for (const p of field.parcels) (byEm[p.emitter] ??= []).push(Math.hypot(p.velocity.x, p.velocity.y));
    const em = field.emitters.map((e: any) => `${e.emitter}:v${e.exitVelocityMS.toFixed(2)} T${e.temperatureC.toFixed(1)} Q${(e.volumeFlowM3S * 1000).toFixed(2)}L/s`).join(" ");
    console.log(` t=${t.toFixed(1)} parcels ${field.parcels.length} vmax ${Math.max(0, ...sp).toFixed(2)} ymax ${Math.max(-9, ...ys).toFixed(2)} byEmitter ` + Object.entries(byEm).map(([k, v]) => `${k}:${Math.max(...v).toFixed(2)}`).join(",") + ` | ${em}`);
  }
}
