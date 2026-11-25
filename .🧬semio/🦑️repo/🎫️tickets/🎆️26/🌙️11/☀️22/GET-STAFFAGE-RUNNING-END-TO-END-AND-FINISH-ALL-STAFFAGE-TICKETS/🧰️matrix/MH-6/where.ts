/** 🎯️ Where the silhouette guard of one view finds its largest distance: `bun where.ts <id|file.json> <scale> <view> [<pose>] [--cell 0.004] [--dump x0,y0,x1,y1]` evaluates the pose (default: the default pose), rasters what the view draws and what the volume projects (the reference of the guard, slender solids as strokes) on a coarse grid and prints the bounding boxes of both and, as the guard measures it (enclosed holes of up to four smallest features squared filled, then outline against outline, the outlines of larger holes included), the ten places where the outline of one lies farthest from the outline of the other, with their place on the sheet (ahead or lateral, up or lateral) and the shapes drawn nearest; `--dump` prints the two rasters of a window of the sheet as text. Run it from `S/📦️packages/🟦️typescript`. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { measureView, worldSolids } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const args = process.argv.slice(2);
const cellAt = args.indexOf("--cell");
const cell = cellAt < 0 ? 0.004 : Number(args.splice(cellAt, 2)[1]);
const dumpAt = args.indexOf("--dump");
const dump = dumpAt < 0 ? undefined : args.splice(dumpAt, 2)[1]!.split(",").map(Number);
const [arg, scaleText, view, poseArg] = args;
const asset = readDoc(arg!).doc.asset;
const scale = Number(scaleText);
const pose = poseArg ?? asset.presentation.defaultPose;
const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w: any) => w.texture !== true);
const m = measureView(asset, state, solids, view, scale, undefined, featureOf(scale));
type P = { x: number; y: number };
const drawnGroups: P[][][] = [...m.filled, ...m.outlines];
const referenceGroups: P[][][] = m.reference;
const all = [...drawnGroups, ...referenceGroups].flat(2);
const [minX, minY] = [Math.min(...all.map((p) => p.x)) - 4 * cell, Math.min(...all.map((p) => p.y)) - 4 * cell];
const [width, height] = [Math.ceil((Math.max(...all.map((p) => p.x)) - minX) / cell) + 4, Math.ceil((Math.max(...all.map((p) => p.y)) - minY) / cell) + 4];
const inside = (group: P[][], x: number, y: number): boolean => {
  let odd = false;
  for (const ring of group) for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) if (ring[i]!.y > y !== ring[j]!.y > y && x < ((ring[j]!.x - ring[i]!.x) * (y - ring[i]!.y)) / (ring[j]!.y - ring[i]!.y) + ring[i]!.x) odd = !odd;
  return odd;
};
const raster = (groups: P[][][]): Uint8Array => {
  const mask = new Uint8Array(width * height);
  for (const group of groups) {
    const points = group.flat();
    const [i0, i1] = [Math.floor((Math.min(...points.map((p) => p.x)) - minX) / cell), Math.ceil((Math.max(...points.map((p) => p.x)) - minX) / cell)];
    const [j0, j1] = [Math.floor((Math.min(...points.map((p) => p.y)) - minY) / cell), Math.ceil((Math.max(...points.map((p) => p.y)) - minY) / cell)];
    for (let j = Math.max(0, j0); j <= Math.min(height - 1, j1); j++) for (let i = Math.max(0, i0); i <= Math.min(width - 1, i1); i++) if (mask[j * width + i] === 0 && inside(group, minX + (i + 0.5) * cell, minY + (j + 0.5) * cell)) mask[j * width + i] = 1;
  }
  return mask;
};
const [drawn, reference] = [raster(drawnGroups), raster(referenceGroups)];
const box = (mask: Uint8Array): string => {
  let [a, b, c, d] = [Infinity, -Infinity, Infinity, -Infinity];
  for (let j = 0; j < height; j++) for (let i = 0; i < width; i++) if (mask[j * width + i]) [a, b, c, d] = [Math.min(a, i), Math.max(b, i), Math.min(c, j), Math.max(d, j)];
  return `x [${(minX + a * cell).toFixed(3)}, ${(minX + (b + 1) * cell).toFixed(3)}] y [${(minY + c * cell).toFixed(3)}, ${(minY + (d + 1) * cell).toFixed(3)}]`;
};
console.log(`${asset.id} 1:${scale} ${view} pose ${pose}: guard iou ${m.iou.toFixed(4)} hausdorff ${(m.hausdorffM * 1000).toFixed(1)} mm; grid ${width}×${height} of ${cell * 1000} mm`);
console.log(`  drawn     ${box(drawn)}`);
console.log(`  reference ${box(reference)}`);
const { fillSmallHoles, boundaryOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const holes = Math.round((4 * featureOf(scale)) ** 2 / cell ** 2);
const [drawnEdge, referenceEdge]: Uint8Array[] = [boundaryOf(fillSmallHoles(drawn, width, height, holes), width, height), boundaryOf(fillSmallHoles(reference, width, height, holes), width, height)];
const far = (own: Uint8Array, other: Uint8Array, name: string): { name: string; x: number; y: number; d: number }[] => {
  const out: { name: string; x: number; y: number; d: number }[] = [];
  const reach = Math.ceil(0.4 / cell);
  for (let j = 0; j < height; j++) for (let i = 0; i < width; i++) {
    if (!own[j * width + i] || other[j * width + i]) continue;
    let best = Infinity;
    for (let r = 1; r <= reach && r < best; r++) for (let dj = -r; dj <= r; dj++) for (let di = -r; di <= r; di++) {
      if (Math.max(Math.abs(di), Math.abs(dj)) !== r) continue;
      const [ii, jj] = [i + di, j + dj];
      if (ii < 0 || jj < 0 || ii >= width || jj >= height || !other[jj * width + ii]) continue;
      best = Math.min(best, Math.hypot(di, dj));
    }
    if (best * cell > 2 * cell) out.push({ name, x: minX + (i + 0.5) * cell, y: minY + (j + 0.5) * cell, d: best * cell });
  }
  return out;
};
const worst = [...far(drawnEdge, referenceEdge, "drawn edge"), ...far(referenceEdge, drawnEdge, "volume edge")].sort((a, b) => b.d - a.d);
const shapes = state.shapes[view] as { shape: any; rings: P[][] }[];
const nearest = (x: number, y: number): string => shapes.map((entry) => ({ id: `${entry.shape.bone}/${entry.shape.id}`, d: Math.min(...entry.rings.flat().map((p) => Math.hypot(p.x - x, p.y - y))) })).sort((a, b) => a.d - b.d).slice(0, 3).map((e) => `${e.id} ${(e.d * 1000).toFixed(0)}`).join(", ");
const picked: typeof worst = [];
for (const w of worst) if (picked.every((p) => Math.hypot(p.x - w.x, p.y - w.y) > 0.05) && picked.push(w) >= 10) break;
for (const w of picked) console.log(`  ${w.name} ${(w.d * 1000).toFixed(0)} mm at (${w.x.toFixed(3)}, ${w.y.toFixed(3)}); nearest drawn: ${nearest(w.x, w.y)}`);
if (dump !== undefined) {
  const [x0, y0, x1, y1] = dump as [number, number, number, number];
  console.log(`  window x ${x0} to ${x1} (left to right), y ${y1} down to ${y0}: # both, D drawn only, V volume only`);
  for (let j = Math.floor((y1 - minY) / cell); j >= Math.floor((y0 - minY) / cell); j--) {
    let row = "";
    for (let i = Math.floor((x0 - minX) / cell); i <= Math.floor((x1 - minX) / cell); i++) {
      const k = j * width + i;
      row += i < 0 || j < 0 || i >= width || j >= height ? " " : drawn[k] && reference[k] ? "#" : drawn[k] ? "D" : reference[k] ? "V" : ".";
    }
    console.log(`  ${row}`);
  }
}
