import { openCtx } from "./kit-lib.ts";
import { localGeometry, qualityOfAsset } from "./volumes-lib.ts";
const [spec, sc, id] = [process.argv[2]!, Number(process.argv[3]), process.argv[4]!];
const c = openCtx(spec, sc);
const s = c.vol.body.find((x: any) => x.id === id);
const g = localGeometry(s, qualityOfAsset(c.m));
const mn = [1e9, 1e9, 1e9], mx = [-1e9, -1e9, -1e9];
for (let i = 0; i < g.positions.length; i += 3) for (let k = 0; k < 3; k++) { mn[k] = Math.min(mn[k]!, g.positions[i + k]!); mx[k] = Math.max(mx[k]!, g.positions[i + k]!); }
console.log("min", mn.map((x) => x.toFixed(3)), "max", mx.map((x) => x.toFixed(3)));
const rows: Record<string, number[]> = {};
for (let i = 0; i < g.positions.length; i += 3) { const k = g.positions[i]!.toFixed(2); (rows[k] ??= []).push(g.positions[i + 2]!, g.positions[i + 1]!); }
for (const [k, v] of Object.entries(rows).slice(0, 12)) console.log(k, "n", v.length / 2, "w", Math.min(...v.filter((_, i) => i % 2 === 0)).toFixed(3), Math.max(...v.filter((_, i) => i % 2 === 0)).toFixed(3));
for (const u0 of [0.2, 0.35, 0.5, 0.6, 0.72]) {
  const vs: number[] = [], ws: number[] = [];
  for (let i = 0; i < g.positions.length; i += 3) if (Math.abs(g.positions[i]! - u0) < 0.02) { vs.push(g.positions[i + 1]!); ws.push(g.positions[i + 2]!); }
  console.log("u", u0, "n", vs.length, "v", Math.min(...vs).toFixed(3), Math.max(...vs).toFixed(3), "w", Math.min(...ws).toFixed(3), Math.max(...ws).toFixed(3));
}
