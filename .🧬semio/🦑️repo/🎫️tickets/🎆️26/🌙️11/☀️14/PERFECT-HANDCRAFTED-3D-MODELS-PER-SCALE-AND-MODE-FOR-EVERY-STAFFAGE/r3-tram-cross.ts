/** 🔀️ Round 3 tram: where the bodies of two scales disagree in one view (`bun r3-tram-cross.ts 50 100 elevation [minMm]`), binned by 0.25 m; owned raster, slender and texture solids left out. */
import { distanceField, entryOf, gridFor, loadSource, meshMask, merged, projectedPoints, scaleMeshes } from "./volumes-lib.ts";

const [fine, coarse, view, minMm] = [Number(process.argv[2]), Number(process.argv[3]), process.argv[4] ?? "elevation", Number(process.argv[5] ?? 30)];
const { entry, asset } = entryOf("vehicles/tram");
const a = merged(asset, loadSource(entry)!);
const bodyOf = (scale: number) => {
  const sm = scaleMeshes(a, scale, a.volumes[String(scale)]);
  return sm.body.filter((_, i) => !sm.solids.body[i]?.slender && !sm.solids.body[i]?.texture);
};
const [f, c] = [bodyOf(fine), bodyOf(coarse)];
const pts = [...projectedPoints(view as never, f), ...projectedPoints(view as never, c)];
const b = { minX: Math.min(...pts.map((p) => p.x)), minY: Math.min(...pts.map((p) => p.y)), maxX: Math.max(...pts.map((p) => p.x)), maxY: Math.max(...pts.map((p) => p.y)) };
const g = gridFor(b, Math.max(b.maxX - b.minX, b.maxY - b.minY), 1600);
const [mf, mc] = [meshMask(view as never, f, g.box, g.cell), meshMask(view as never, c, g.box, g.cell)];
const [df, dc] = [distanceField(mf), distanceField(mc)];
const bins = new Map<string, { n: number; max: number }>();
for (let i = 0; i < mf.cells.length; i++) {
  if (mf.cells[i] === mc.cells[i]) continue;
  const d = mf.cells[i] ? dc[i]! : df[i]!;
  if (d * 1000 < minMm) continue;
  const x = g.box.minX + ((i % mf.width) + 0.5) * g.cell;
  const y = g.box.minY + (Math.floor(i / mf.width) + 0.5) * g.cell;
  const k = `${mf.cells[i] ? `only-1:${fine}` : `only-1:${coarse}`} x=${(Math.round(x * 4) / 4).toFixed(2)} y=${(Math.round(y * 4) / 4).toFixed(2)}`;
  const e = bins.get(k) ?? { n: 0, max: 0 };
  e.n++;
  e.max = Math.max(e.max, d);
  bins.set(k, e);
}
console.log(`[DEBUG] cell ${(g.cell * 1000).toFixed(1)} mm`);
for (const [k, e] of [...bins].sort((p, q) => q[1].n - p[1].n).slice(0, 30)) console.log(`[DEBUG] ${k} cells ${e.n} max ${(e.max * 1000).toFixed(0)} mm`);
