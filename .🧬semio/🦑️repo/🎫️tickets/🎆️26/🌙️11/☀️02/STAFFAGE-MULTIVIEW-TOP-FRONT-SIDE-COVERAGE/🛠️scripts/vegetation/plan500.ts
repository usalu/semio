import { load, save, stateAt, three } from "./lib.ts";
const id = process.argv[2]!; const write = process.argv[3] === "write"; const n0 = Number(process.argv[4] ?? 11);
const x = load(id); const a = x.asset; const D = a.dimensions; const r4 = (v: number) => Math.round(v * 10000) / 10000;
const g = a.representations["500"].geometry;
const st = stateAt(a, 500); const E = three.extentOf(st.shapes.elevation), F = three.extentOf(st.shapes.front), P = three.extentOf(st.shapes.plan);
const x0 = (E.minX + E.maxX) / 2 - D.widthM / 2, x1 = x0 + D.widthM, z0 = (F.minX + F.maxX) / 2 - D.depthM / 2, z1 = z0 + D.depthM;
const crown = g.plan.filter((s: any) => s.kind === "poly" && s.role === "body" && !/trunk|stem|dot/.test(s.id)).sort((p: any, q: any) => q.points.length - p.points.length)[0];
const pts: [number, number][] = [];
for (let i = 0; i < 180; i++) { const t = (i / 180) * 2 * Math.PI; const r = 0.9 + 0.1 * Math.pow(Math.abs(Math.sin((n0 * t) / 2 + 0.6)), 0.7) + 0.02 * Math.sin(3 * t); pts.push([Math.cos(t) * r, Math.sin(t) * r]); }
const xs = pts.map((p) => p[0]), ys = pts.map((p) => p[1]); const ax = Math.min(...xs), bx = Math.max(...xs), ay = Math.min(...ys), by = Math.max(...ys);
crown.points = pts.map(([px, py]) => [r4(x0 + ((px - ax) / (bx - ax)) * (x1 - x0)), r4(z0 + ((py - ay) / (by - ay)) * (z1 - z0))]);
const after = three.extentOf(stateAt(a, 500).shapes.plan);
console.log(id, "plan 500 before", (P.maxX - P.minX).toFixed(2), (P.maxY - P.minY).toFixed(2), "after", (after.maxX - after.minX).toFixed(2), (after.maxY - after.minY).toFixed(2), "dims", D.widthM, D.depthM, "shape", crown.id, crown.bone);
if (write) save(x);
