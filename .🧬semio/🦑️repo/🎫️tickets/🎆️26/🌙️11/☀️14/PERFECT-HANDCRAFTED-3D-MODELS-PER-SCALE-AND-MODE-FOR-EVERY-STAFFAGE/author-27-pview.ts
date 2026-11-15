/** Product silhouette overlay of one view: green both, red drawn only, blue 3D only (product `measureAsset` masks). Usage: bun author-27-pview.ts <asset> --scale N --view elevation --source file [--crop x0,y0,x1,y1] */
import { readFileSync, writeFileSync } from "node:fs";
import { Canvas } from "./volumes-raster.ts";
import { entryOf, merged } from "./volumes-lib.ts";
import { measureAsset, Mask } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

const a = process.argv.slice(2);
const f = (n: string) => (a.indexOf(n) < 0 ? undefined : a[a.indexOf(n) + 1]);
const { asset } = entryOf(a[0]!);
const src = JSON.parse(readFileSync(f("--source")!, "utf8"));
const m = merged(asset, src);
const scale = Number(f("--scale"));
const view = (f("--view") ?? "elevation") as "elevation";
const r = measureAsset(m as never, scale, undefined, undefined, undefined);
const v = r.views[view];
const vol = new Mask(v.box, v.cell);
for (const g of v.reference) vol.fill(g);
const drawn = new Mask(v.box, v.cell);
for (const g of v.filled) drawn.fill(g);
for (const g of v.outlines) drawn.fill(g);
const W = vol.width, H = vol.height;
const step = Math.max(1, Math.round(W / 900));
const cw = Math.ceil(W / step), ch = Math.ceil(H / step);
const c = new Canvas(cw, ch, [247, 247, 244]);
for (let j = 0; j < ch; j++) for (let i = 0; i < cw; i++) {
  const x = i * step, y = j * step;
  const idx = (H - 1 - y) * W + x;
  const d = drawn.cells[idx] === 1, s = vol.cells[idx] === 1;
  if (d && s) c.set(i, j, [147, 207, 156]);
  else if (d) c.set(i, j, [225, 60, 60]);
  else if (s) c.set(i, j, [70, 110, 235]);
}
c.text(4, 4, `${view} 1:${scale} iou ${v.iou.toFixed(3)} cell ${(v.cell * 1000).toFixed(2)}mm both ${v.counts.both} either ${v.counts.either}`, [0, 0, 0], 1);
const out = f("--out") ?? `🗑️generated/author-27/pview-${scale}-${view}.png`;
writeFileSync(out, c.png());
process.stdout.write(`${out} iou ${v.iou} filled ${v.filled.length} outlines ${v.outlines.length}\n`);
