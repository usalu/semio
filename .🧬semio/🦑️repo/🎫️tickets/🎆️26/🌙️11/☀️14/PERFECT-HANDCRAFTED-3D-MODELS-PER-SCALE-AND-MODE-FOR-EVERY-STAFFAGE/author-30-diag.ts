import { readFileSync } from "node:fs";
import { measureAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { Mask, entryOf, featureM, loadSource, merged, poseOf } from "./volumes-lib.ts";

const [spec, scaleArg, file] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const source = file === undefined ? loadSource(entry) : JSON.parse(readFileSync(file, "utf8"));
const m = merged(asset, source);
const scale = Number(scaleArg);
const pm = measureAsset(m as never, scale, poseOf(m), undefined, featureM(scale, m));
for (const view of ["elevation", "front", "plan"] as const) {
  const v = pm.views[view];
  const drawn = new Mask(v.box, v.cell);
  const volume = new Mask(v.box, v.cell);
  for (const g of v.filled) drawn.fill(g);
  for (const g of v.outlines) drawn.fill(g);
  for (const g of v.reference) volume.fill(g);
  console.log(`== ${view} iou ${v.iou.toFixed(3)} recall ${v.recall.toFixed(3)} precision ${v.precision.toFixed(3)} cell ${(v.cell * 1000).toFixed(1)}mm`);
  for (const kind of ["red", "blue"] as const) {
    const w = drawn.width, h = drawn.height;
    const flag = new Uint8Array(w * h);
    for (let i = 0; i < w * h; i++) flag[i] = kind === "red" ? (drawn.cells[i] && !volume.cells[i] ? 1 : 0) : volume.cells[i] && !drawn.cells[i] ? 1 : 0;
    const seen = new Uint8Array(w * h);
    const comps: { n: number; x0: number; x1: number; y0: number; y1: number }[] = [];
    for (let s = 0; s < w * h; s++) {
      if (!flag[s] || seen[s]) continue;
      const stack = [s];
      seen[s] = 1;
      let n = 0, x0 = 1e9, x1 = -1e9, y0 = 1e9, y1 = -1e9;
      while (stack.length) {
        const c = stack.pop()!;
        n++;
        const cx = c % w, cy = Math.floor(c / w);
        x0 = Math.min(x0, cx); x1 = Math.max(x1, cx); y0 = Math.min(y0, cy); y1 = Math.max(y1, cy);
        for (const [dx, dy] of [[1, 0], [-1, 0], [0, 1], [0, -1]] as const) {
          const nx = cx + dx, ny = cy + dy;
          if (nx < 0 || ny < 0 || nx >= w || ny >= h) continue;
          const k = ny * w + nx;
          if (flag[k] && !seen[k]) { seen[k] = 1; stack.push(k); }
        }
      }
      comps.push({ n, x0, x1, y0, y1 });
    }
    comps.sort((a, b) => b.n - a.n);
    const area = (n: number) => (n * v.cell * v.cell).toFixed(4);
    const px = (i: number) => (v.box.minX + i * v.cell).toFixed(3);
    const py = (j: number) => (v.box.minY + j * v.cell).toFixed(3);
    console.log(`  ${kind}:`, comps.slice(0, 16).map((c) => `${area(c.n)}m2 x[${px(c.x0)},${px(c.x1)}] y[${py(c.y0)},${py(c.y1)}]`).join(" | "));
  }
}
