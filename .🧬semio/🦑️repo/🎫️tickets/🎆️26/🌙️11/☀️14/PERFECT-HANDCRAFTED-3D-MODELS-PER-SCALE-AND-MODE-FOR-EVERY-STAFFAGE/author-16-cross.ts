import { entryOf, loadSource, scaleMeshes, projectedPoints, gridFor, meshMask, merged, type View } from "./volumes-lib.ts";
const [spec, a, b, view = "elevation", cols = "70"] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const src = loadSource(entry)!;
const A = scaleMeshes(asset, Number(a), src.volumes[a!]);
const B = scaleMeshes(asset, Number(b), src.volumes[b!]);
const pts = [...projectedPoints(view as View, A.body), ...projectedPoints(view as View, B.body)];
const bb = { minX: 1e9, minY: 1e9, maxX: -1e9, maxY: -1e9 };
for (const p of pts) { bb.minX = Math.min(bb.minX, p.x); bb.maxX = Math.max(bb.maxX, p.x); bb.minY = Math.min(bb.minY, p.y); bb.maxY = Math.max(bb.maxY, p.y); }
const g = gridFor(bb, Math.max(bb.maxX - bb.minX, bb.maxY - bb.minY), Number(cols));
const ma = meshMask(view as View, A.body, g.box, g.cell);
const mb = meshMask(view as View, B.body, g.box, g.cell);
const w = ma.width;
const h = ma.height;
let both = 0, ao = 0, bo = 0;
for (let y = h - 1; y >= 0; y--) {
  let row = "";
  for (let x = 0; x < w; x++) {
    const i = y * w + x;
    const pa = ma.cells[i], pb = mb.cells[i];
    row += pa && pb ? "#" : pa ? "a" : pb ? "b" : ".";
    if (pa && pb) both++; else if (pa) ao++; else if (pb) bo++;
  }
  console.log(row);
}
console.log({ both, aOnly: ao, bOnly: bo, iou: both / (both + ao + bo) });
