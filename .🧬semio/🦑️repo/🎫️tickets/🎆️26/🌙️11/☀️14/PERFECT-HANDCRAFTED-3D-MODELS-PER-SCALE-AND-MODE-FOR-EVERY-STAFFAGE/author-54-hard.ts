import { openCtx, drawn } from "./kit-lib.ts";
import { meshesAt, meshMask, Mask, distanceField } from "./volumes-lib.ts";
const [spec, sc, view] = [process.argv[2]!, Number(process.argv[3]), process.argv[4] as any];
const c = openCtx(spec, sc);
const softRe = new RegExp(process.argv[5] ?? "^blade");
const ms = meshesAt(c.m, sc, c.vol.body);
const box = { minX: -1.2, maxX: 1.2, minY: -0.2, maxY: 1.2 }; const cell = c.feature / 2;
const bx = view === "plan" ? { minX: -1.2, maxX: 1.2, minY: -1.2, maxY: 1.2 } : box;
const a = meshMask(view, ms, bx, cell);
const soft = meshMask(view, ms.filter((m) => softRe.test(m.id)), bx, cell);
const b = new Mask(bx, cell); for (const d of drawn(c, view)) b.fill(d.rings);
const da = distanceField(a), db = distanceField(b), ds = distanceField(soft);
const out: { x: number; y: number; d: number; src: string }[] = [];
for (let j = 1; j < a.height - 1; j++) for (let i = 1; i < a.width - 1; i++) { const k = j * a.width + i; const edgeA = a.cells[k] && (!a.cells[k - 1] || !a.cells[k + 1] || !a.cells[k - a.width] || !a.cells[k + a.width]); const edgeB = b.cells[k] && (!b.cells[k - 1] || !b.cells[k + 1] || !b.cells[k - b.width] || !b.cells[k + b.width]); const d = edgeA ? db[k]! : edgeB ? da[k]! : 0; if (d > 0.012 && ds[k]! > 3 * cell) out.push({ x: bx.minX + (i + 0.5) * cell, y: bx.minY + (j + 0.5) * cell, d, src: edgeA ? "3D" : "2D" }); }
out.sort((p, q) => q.d - p.d);
console.log("hard cells with d>12mm and >3 cells from soft:", out.length);
for (const o of out.slice(0, 12)) console.log(o.src, o.d.toFixed(3), o.x.toFixed(3), o.y.toFixed(3));
