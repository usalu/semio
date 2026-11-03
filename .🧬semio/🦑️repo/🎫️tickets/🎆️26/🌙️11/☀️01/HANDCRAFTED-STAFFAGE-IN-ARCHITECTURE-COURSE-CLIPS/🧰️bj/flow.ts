import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules";
const V = "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json";
const load = (path: string) => import(pathToFileURL(resolve(S, path)).href);
const { loadCatalogFromFiles, loadEnvironments } = await load("🗂️catalog/📥️files/🟦️.ts");
const { solveTransport } = await load("💨️transport/🟦️.ts");
const id = process.argv[2] ?? "shared-person-pathlines";
const cast = JSON.parse(readFileSync(V, "utf8")).casts[id];
const catalog = await loadCatalogFromFiles();
const presets = new Map(((await loadEnvironments())?.presets ?? []).map((p: any) => [p.id, p.environment]));
const asset = catalog.assets.find((a: any) => a.id === cast.asset);
const env = presets.get(cast.environment);
for (let t = 3.5; t < 8.6; t += 0.25) {
  const f = solveTransport(asset, { kind: "clip", clip: cast.clip, timeS: t }, env, { maxAgeS: cast.transportAgeS, periodS: 5 });
  const n = f.parcels.filter((p: any) => p.emitter.includes("nose")).length;
  const xs = f.parcels.filter((p: any) => p.emitter.includes("nose")).map((p: any) => p.at.x);
  console.log(t.toFixed(2), "flow", (f.respiration?.flowM3S * 1000).toFixed(3), "exit", f.emitters[0].exitVelocityMS.toFixed(2), "noseParcels", n, "maxX", xs.length ? Math.max(...xs).toFixed(2) : "-", "pathlines", f.pathlines.length);
}
for (const t of [3.5, 8.5, 13.5]) {
  const f = solveTransport(asset, { kind: "clip", clip: cast.clip, timeS: t }, env, { maxAgeS: cast.transportAgeS, periodS: 5 });
  const nose = f.pathlines.filter((l: any) => l.emitter.includes("nose"));
  console.log("t", t, nose.filter((_: any, i: number) => i % 8 === 0).map((l: any) => `${l.ages.at(-1).toFixed(2)}:${(l.points.at(-1).x - l.points[0].x).toFixed(2)},${(l.points.at(-1).y - l.points[0].y).toFixed(2)}`).join(" "));
}
