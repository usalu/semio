import { entryOf, loadSource, merged, scaleMeshes, meshArea } from "./volumes-lib.ts";
const { entry, asset } = entryOf(process.argv[2]!); const scale = Number(process.argv[3] ?? 50);
const source = loadSource(entry)!; const a = merged(asset, source);
const sm = scaleMeshes(a, scale, source.volumes[String(scale)]);
sm.body.forEach((m, i) => console.log("body", m.id.padEnd(14), (meshArea(m)).toFixed(4)));
const byRegion: Record<string, number> = {};
sm.thermal.forEach((m, i) => { const s = sm.solids.thermal[i]!; const k = `${s.region}/${s.layer ?? "-"}`; byRegion[k] = (byRegion[k] ?? 0) + meshArea(m); });
const target: Record<string, number> = Object.fromEntries((a.thermal.regions as any[]).map((r) => [r.id, r.areaM2]));
for (const [k, v] of Object.entries(byRegion)) console.log("thermal", k.padEnd(24), v.toFixed(4), "target", target[k.split("/")[0]!]);
