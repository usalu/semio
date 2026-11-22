/** 🔎️ Where drawing and volume disagree: cells of one mask farther than `lim` from the other, clustered (bun where.ts slug scale view [lim m]). */
import { loadDoc, measureAsset, featureOf } from "./lib.ts";
import { Mask, distanceField } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
const [slug, sc, view, lim] = process.argv.slice(2);
const scale = Number(sc ?? 50);
const a = loadDoc(slug!).asset;
const m: any = measureAsset(a, scale, undefined, undefined, featureOf(scale as never));
const v = m.views[view ?? "elevation"];
const cell = 0.003;
const box = { minX: v.box.minX - 0.05, minY: v.box.minY - 0.05, maxX: v.box.maxX + 0.05, maxY: v.box.maxY + 0.05 };
const mk = (groups: any[]) => { const k = new Mask(box, cell); for (const g of groups) k.fill(g); return k; };
const D = mk([...v.filled, ...v.outlines]), R = mk(v.reference);
const W = (D as any).width, H = (D as any).height;
const dD = distanceField((D as any).cells, W, H), dR = distanceField((R as any).cells, W, H);
const L = Number(lim ?? 0.013) / cell;
const report = (name: string, mask: any, dist: Float64Array) => {
  const pts: [number, number, number][] = [];
  for (let j = 0; j < H; j++) for (let i = 0; i < W; i++) { const id = j * W + i; if (mask.cells[id] && Math.sqrt(dist[id]!) > L) pts.push([box.minX + (i + 0.5) * cell, box.minY + (j + 0.5) * cell, Math.sqrt(dist[id]!) * cell]); }
  const cl: { x: number; y: number; n: number; d: number }[] = [];
  for (const [x, y, d] of pts) { const c = cl.find((c) => Math.hypot(c.x - x, c.y - y) < 0.05); if (c) { c.n++; c.d = Math.max(c.d, d); } else cl.push({ x, y, n: 1, d }); }
  console.log(name, cl.map((c) => `(${c.x.toFixed(3)},${c.y.toFixed(3)}) n${c.n} ${(c.d * 1000).toFixed(0)}mm`).join("  "));
};
report("drawn-not-volume", D, dR); report("volume-not-drawn", R, dD);
