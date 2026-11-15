import { entryOf, loadSource, merged, scaleMeshes } from "./volumes-lib.ts";
const [spec, sc, id] = [process.argv[2]!, Number(process.argv[3]), process.argv[4]!];
const { entry, asset } = entryOf(spec);
const m = merged(asset, loadSource(entry)!);
const sm = scaleMeshes(m, sc, m.volumes[String(sc)]);
const i = sm.solids.body.findIndex((s: any) => s.id === id);
const mesh = sm.body[i]!;
const rows = new Map<number, number[][]>();
for (let k = 0; k < mesh.world.length; k += 3) { const y = Math.round(mesh.world[k + 1]! * 100) / 100; (rows.get(y) ?? rows.set(y, []).get(y)!).push([mesh.world[k]!, mesh.world[k + 2]!]); }
for (const [y, pts] of [...rows.entries()].sort((a, b) => a[0] - b[0]).slice(0, 14)) console.log(y, "n", pts.length, "x", Math.min(...pts.map((p) => p[0]!)).toFixed(2), Math.max(...pts.map((p) => p[0]!)).toFixed(2), "z", Math.min(...pts.map((p) => p[1]!)).toFixed(2), Math.max(...pts.map((p) => p[1]!)).toFixed(2));
console.log(JSON.stringify(sm.solids.body[i].sections.slice(0, 3)));
