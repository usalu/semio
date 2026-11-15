import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, boundaryOf, distanceField, measureView, worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const [spec, scaleS, view, ...win] = process.argv.slice(2) as string[];
const [x0, x1, y0, y1, cellS] = win.map(Number) as [number, number, number, number, number];
const { entry, asset } = entryOf(spec!);
const sc = Number(scaleS);
const m = merged(asset, loadSource(entry)!) as any;
const st = evaluate(m, { kind: "pose", pose: m.presentation.defaultPose }, { kind: "scale", scale: { denominator: sc } });
const solids = worldSolids(m, st.bones as never, sc).filter((w: any) => w.texture !== true);
const r = measureView(m, st, solids, view as any, sc, undefined, 0.15e-3 * sc) as any;
const cell = cellS || 0.004;
const box = { minX: x0, minY: y0, maxX: x1, maxY: y1 };
const a = new Mask(box, cell), b = new Mask(box, cell);
for (const g of r.filled) a.fill(g);
for (const g of r.outlines) a.fill(g);
for (const g of r.reference) b.fill(g);
const W = a.width, H = a.height;
const ba = boundaryOf(a.cells, W, H), bb = boundaryOf(b.cells, W, H);
const da = distanceField(ba, W, H), db = distanceField(bb, W, H);
const lim = (0.015 / cell) ** 2;
let n = 0, far = 0; const grid = new Map<string, number>();
for (let j = 0; j < H; j++) for (let i = 0; i < W; i++) { const k = j * W + i; if (ba[k]) { n++; if (db[k]! > lim) { far++; const key = `${Math.floor((x0 + i * cell) * 2) / 2},${Math.floor((y0 + j * cell) * 2) / 2}`; grid.set(key, (grid.get(key) ?? 0) + 1); } } if (bb[k]) { n++; if (da[k]! > lim) { far++; const key = `${Math.floor((x0 + i * cell) * 2) / 2},${Math.floor((y0 + j * cell) * 2) / 2}`; grid.set(key, (grid.get(key) ?? 0) + 1); } } }
console.log("boundary cells", n, "far(>15mm)", far, (100 * far / n).toFixed(1) + "%", "global p95", r.p95M);
console.log([...grid.entries()].sort((p, q) => q[1] - p[1]).slice(0, 25).map(([k, v]) => `${k}:${v}`).join("  "));
if (process.env.ASCII) {
  const step = Math.max(1, Math.round(0.02 / cell));
  for (let j = H - 1; j >= 0; j -= step) { let line = ""; for (let i = 0; i < W; i += step) { let A = 0, B = 0; for (let jj = j; jj > j - step && jj >= 0; jj--) for (let ii = i; ii < i + step && ii < W; ii++) { A |= a.cells[jj * W + ii]!; B |= b.cells[jj * W + ii]!; } line += A && B ? "#" : A ? "a" : B ? "b" : "."; } console.log(line); }
}
if (process.env.LIST) {
  let c = 0;
  for (let j = 0; j < H && c < 12; j++) for (let i = 0; i < W && c < 12; i++) { const k = j * W + i; if (ba[k] && db[k]! > lim) { c++; console.log("A-far", (x0 + i * cell).toFixed(3), (y0 + j * cell).toFixed(3), "dist", Math.sqrt(db[k]!) * cell, "a", a.cells[k], "b", b.cells[k]); } }
}
