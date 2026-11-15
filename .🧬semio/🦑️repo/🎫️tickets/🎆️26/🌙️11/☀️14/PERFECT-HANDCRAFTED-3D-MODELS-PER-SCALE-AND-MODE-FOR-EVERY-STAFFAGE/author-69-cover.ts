/** 🔦️ Which solids of a scale cover a box of a view: bun author-69-cover.ts <asset> <scale> <view> x0 x1 y0 y1 */
import { entryOf, loadSource, merged, scaleMeshes, meshMask } from "./volumes-lib.ts";
const [spec, sc, view, ...r] = process.argv.slice(2);
const [x0, x1, y0, y1] = r.map(Number) as [number, number, number, number];
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!);
const sm = scaleMeshes(m, Number(sc), m.volumes[sc]);
const cell = 0.002;
const box = { minX: x0, minY: y0, maxX: x1, maxY: y1 };
const ids = sm.solids.body;
sm.body.forEach((mesh, i) => {
  const mk = meshMask(view as any, [mesh], box, cell);
  let n = 0; for (const c of mk.cells) if (c) n++;
  if (n > 0) console.log(`${ids[i]?.id ?? i} (${ids[i]?.bone}) covers ${(n * cell * cell * 1e4).toFixed(1)} cm2 of ${((x1 - x0) * (y1 - y0) * 1e4).toFixed(1)}`);
});
