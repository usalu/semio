import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, boundaryOf, distanceField, measureView, worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const [spec, scaleS, view, limS] = process.argv.slice(2) as string[];
const { entry, asset } = entryOf(spec!);
const sc = Number(scaleS);
const m = merged(asset, loadSource(entry)!) as any;
const st = evaluate(m, { kind: "pose", pose: m.presentation.defaultPose }, { kind: "scale", scale: { denominator: sc } });
const solids = worldSolids(m, st.bones as never, sc).filter((w: any) => w.texture !== true);
const r = measureView(m, st, solids, view as any, sc, undefined, 0.15e-3 * sc) as any;
const bx = r.box; const cell = 0.5 * (0.15e-3 * sc) / 2 || 0.004; const lim2 = (Number(limS ?? 0.015) / cell) ** 2;
const grid = new Map<string, number>(); let n = 0, far = 0;
const WX = 10, WY = 8;
for (let x0 = bx.minX; x0 < bx.maxX; x0 += WX) for (let y0 = bx.minY; y0 < bx.maxY; y0 += WY) {
  const box = { minX: x0, minY: y0, maxX: Math.min(x0 + WX, bx.maxX), maxY: Math.min(y0 + WY, bx.maxY) };
  const a = new Mask(box, cell), b = new Mask(box, cell);
  for (const g of r.filled) a.fill(g); for (const g of r.outlines) a.fill(g); for (const g of r.reference) b.fill(g);
  let any = false; for (let i = 0; i < a.cells.length; i++) if (a.cells[i] || b.cells[i]) { any = true; break; } if (!any) continue;
  const W = a.width, H = a.height;
  const ba = boundaryOf(a.cells, W, H), bb = boundaryOf(b.cells, W, H); const da = distanceField(ba, W, H), db = distanceField(bb, W, H);
  for (let j = 1; j < H - 1; j++) for (let i = 1; i < W - 1; i++) { const k = j * W + i; for (const [bd, df] of [[ba, db], [bb, da]] as const) if (bd[k]) { n++; if (df[k]! > lim2) { far++; const key = `${(Math.floor((x0 + i * cell) * 2) / 2).toFixed(1)},${(Math.floor((y0 + j * cell) * 2) / 2).toFixed(1)}`; grid.set(key, (grid.get(key) ?? 0) + 1); } } }
}
console.log("cell", cell, "boundary cells", n, "far", far, (100 * far / n).toFixed(2) + "%", "p95", r.p95M);
console.log([...grid.entries()].sort((p, q) => q[1] - p[1]).slice(0, 40).map(([k, v]) => `${k}:${v}`).join("  "));
