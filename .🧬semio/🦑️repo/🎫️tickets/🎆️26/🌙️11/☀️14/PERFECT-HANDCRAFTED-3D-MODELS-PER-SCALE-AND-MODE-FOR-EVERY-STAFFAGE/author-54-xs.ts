import { openCtx } from "./kit-lib.ts";
import { meshesAt, meshMask, Mask, distanceField } from "./volumes-lib.ts";
const [spec, sa, sb, view] = [process.argv[2]!, Number(process.argv[3]), Number(process.argv[4]), process.argv[5] as any];
const A = openCtx(spec, sa), B = openCtx(spec, sb);
const ma = meshesAt(A.m, sa, A.vol.body), mb = meshesAt(B.m, sb, B.vol.body);
const box = { minX: -2.2, maxX: 2.2, minY: -2.2, maxY: 2.2 }; const cell = Math.max(0.005, B.feature / 4);
const a = meshMask(view, ma, box, cell), b = meshMask(view, mb, box, cell);
const da = distanceField(a), db = distanceField(b);
const bd = (m: Mask) => { const o: number[] = []; for (let j = 1; j < m.height - 1; j++) for (let i = 1; i < m.width - 1; i++) { const k = j * m.width + i; if (m.cells[k] && (!m.cells[k - 1] || !m.cells[k + 1] || !m.cells[k - m.width] || !m.cells[k + m.width])) o.push(k); } return o; };
const items: { k: number; d: number; from: string }[] = [];
for (const k of bd(a)) items.push({ k, d: db[k]!, from: `${sa}` });
for (const k of bd(b)) items.push({ k, d: da[k]!, from: `${sb}` });
items.sort((x, y) => y.d - x.d);
const p95 = [...items].map((x) => x.d).sort((x, y) => x - y)[Math.floor(items.length * 0.95)];
console.log("p95", p95?.toFixed(3), "n", items.length);
const perSolid = (ctx: typeof A, ms: any[], k: number) => { const x = box.minX + ((k % a.width) + 0.5) * cell, y = box.minY + (Math.floor(k / a.width) + 0.5) * cell; const hit: string[] = []; for (const m of ms) { const mm = meshMask(view, [m], { minX: x - cell, maxX: x + cell, minY: y - cell, maxY: y + cell }, cell / 2); if (mm.cells.some((c) => c)) hit.push(m.id); } return `(${x.toFixed(2)},${y.toFixed(2)}) ${hit.join(",")}`; };
const seen = new Set<string>();
let shown = 0;
for (const it of items) { if (shown >= 10) break; const key = `${Math.round((it.k % a.width) / 12)}-${Math.round(Math.floor(it.k / a.width) / 12)}`; if (seen.has(key)) continue; seen.add(key); shown++; console.log(it.from, "d", it.d.toFixed(3), perSolid(it.from === `${sa}` ? A : B, it.from === `${sa}` ? ma : mb, it.k)); }
if (process.argv[6]) {
  const [x0, x1, y0, y1] = process.argv[6].split(",").map(Number) as [number, number, number, number];
  const st = 0.015;
  for (let y = y1; y >= y0; y -= st) { let row = ""; for (let x = x0; x <= x1; x += st) { const i = Math.floor((x - box.minX) / cell), j = Math.floor((y - box.minY) / cell), k = j * a.width + i; row += a.cells[k] && b.cells[k] ? "#" : a.cells[k] ? "a" : b.cells[k] ? "b" : "."; } console.log(row); }
}
