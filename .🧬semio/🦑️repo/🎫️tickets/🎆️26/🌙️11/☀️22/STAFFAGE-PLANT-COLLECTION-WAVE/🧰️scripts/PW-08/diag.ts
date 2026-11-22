/** 🔎️ PW-08 diagnosis: `bun diag.ts <asset-id> <scale> <view>` rasterises the candidate's own 3D body and its filled 2D shapes as the product places them, and prints the boundary-distance percentiles and where the largest distances lie (paper metres of the view).
 *
 * @see ./build.ts */
import { readFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Grid, edt, paper, type P2, type View } from "./lib.ts";

const HERE = dirname(fileURLToPath(import.meta.url));
const [id, scaleS, view] = process.argv.slice(2) as [string, string, View];
const doc = JSON.parse(readFileSync(join(HERE, "../../🗑️generated/PW-08", id, "🔣️.json"), "utf8"));
const asset = doc.asset;
const scale = Number(scaleS);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
const frames: Record<string, { o: number[]; u: number[]; v: number[]; w: number[] }> = {};
for (const [k, b] of Object.entries(state.bones as Record<string, any>)) frames[k] = { o: [b.frame.o.x, b.frame.o.y, b.frame.o.z], u: [b.frame.u.x, b.frame.u.y, b.frame.u.z], v: [b.frame.v.x, b.frame.v.y, b.frame.v.z], w: [b.frame.w.x, b.frame.w.y, b.frame.w.z] };
const cell = 0.01 * (scale / 100);
const solids = asset.volumes[String(scale)].body;
const pts: P2[] = [];
const tris: [P2, P2, P2][] = [];
for (const s of solids) {
  const f = frames[s.bone]!;
  const w = (s.vertices as number[][]).map((q) => [f.o[0]! + q[0]! * f.u[0]! + q[1]! * f.v[0]! + q[2]! * f.w[0]!, f.o[1]! + q[0]! * f.u[1]! + q[1]! * f.v[1]! + q[2]! * f.w[1]!, f.o[2]! + q[0]! * f.u[2]! + q[1]! * f.v[2]! + q[2]! * f.w[2]!] as [number, number, number]);
  const p = w.map((q) => paper(view, q));
  pts.push(...p);
  for (const [a, b, c] of s.triangles) tris.push([p[a]!, p[b]!, p[c]!]);
}
const g3 = Grid.around(pts, cell, 0.5);
for (const [a, b, c] of tris) g3.tri(a, b, c);
const g2 = g3.like();
for (const e of state.shapes[view]) if (e.shape.filled && e.shape.role !== "texture") g2.ring(e.rings[0].map((q: any) => [q.x, q.y] as P2));
const boundary = (g: Grid) => { const b = g.like(); for (let j = 0; j < g.h; j++) for (let i = 0; i < g.w; i++) { const k = j * g.w + i; if (!g.d[k]) continue; if (i === 0 || j === 0 || i === g.w - 1 || j === g.h - 1 || !g.d[k - 1] || !g.d[k + 1] || !g.d[k - g.w] || !g.d[k + g.w]) b.d[k] = 1; } return b; };
const b3 = boundary(g3), b2 = boundary(g2);
const d3 = edt(b3, (v) => v === 0), d2 = edt(b2, (v) => v === 0);
const samples: { d: number; x: number; y: number; side: string }[] = [];
for (let k = 0; k < b2.d.length; k++) { if (b2.d[k]) samples.push({ d: Math.sqrt(d3[k]!) * cell, x: g3.x0 + (k % g3.w) * cell, y: g3.y0 + Math.floor(k / g3.w) * cell, side: "2d" }); if (b3.d[k]) samples.push({ d: Math.sqrt(d2[k]!) * cell, x: g3.x0 + (k % g3.w) * cell, y: g3.y0 + Math.floor(k / g3.w) * cell, side: "3d" }); }
samples.sort((a, b) => a.d - b.d);
const pc = (q: number) => samples[Math.floor(q * (samples.length - 1))]!.d;
let both = 0, any = 0; for (let k = 0; k < g2.d.length; k++) { if (g2.d[k] || g3.d[k]) any++; if (g2.d[k] && g3.d[k]) both++; }
console.log(`1:${scale} ${view}: iou ${(both / any).toFixed(4)} p50 ${(pc(0.5) * 1000).toFixed(1)} p95 ${(pc(0.95) * 1000).toFixed(1)} max ${(pc(1) * 1000).toFixed(1)} mm`);
for (const s of samples.slice(-12)) console.log(`  ${s.side} ${(s.d * 1000).toFixed(0)} mm at ${s.x.toFixed(2)}, ${s.y.toFixed(2)}`);
const far = samples.filter((s) => s.d > 0.06);
const hist = new Map<number, number>(); for (const s of far) { const k = Math.round(s.y); hist.set(k, (hist.get(k) ?? 0) + 1); }
console.log(`  > 60 mm by height (m: count): ${[...hist.entries()].sort((a, b) => a[0] - b[0]).map(([k, v]) => `${k}:${v}`).join(" ")}`);
