/** 🌗️ Says where the drawing of a document and its 3D volume part most in one view of the default pose: the edge mask of the catalog-silhouette measurement (drawn boundary and volume boundary) is split by the drawn fill, and the boundary cell of each side farthest from the other boundary is printed with the shapes drawn there: bun wc-sil.ts <asset json> <scale> <view> */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { featureOf } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { measureView, worldSolids } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const [file, scaleText, view] = process.argv.slice(2);
const scale = Number(scaleText);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = (worldSolids as any)(asset, state.bones, scale).filter((w: any) => w.texture !== true);
const m = (measureView as any)(asset, state, solids, view, scale, undefined, featureOf(scale), undefined, true);
type Ring = readonly { readonly x: number; readonly y: number }[];
const CELL = 0.002;
const groups = { drawn: m.filled as Ring[][], volume: [...m.reference, ...m.visible, ...m.full] as Ring[][] };
const all = [...groups.drawn, ...groups.volume].flat(2);
const [x0, y0] = [Math.min(...all.map((p) => p.x)) - 2 * CELL, Math.min(...all.map((p) => p.y)) - 2 * CELL];
const [nx, ny] = [Math.ceil((Math.max(...all.map((p) => p.x)) - x0) / CELL) + 3, Math.ceil((Math.max(...all.map((p) => p.y)) - y0) / CELL) + 3];
const inside = (ring: Ring, x: number, y: number): boolean => {
  let hit = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) if (ring[i]!.y > y !== ring[j]!.y > y && x < ((ring[j]!.x - ring[i]!.x) * (y - ring[i]!.y)) / (ring[j]!.y - ring[i]!.y) + ring[i]!.x) hit = !hit;
  return hit;
};
const raster = (list: Ring[][]): Uint8Array => {
  const grid = new Uint8Array(nx * ny);
  for (const group of list)
    for (const ring of group) {
      const [rx0, rx1, ry0, ry1] = [Math.min(...ring.map((p) => p.x)), Math.max(...ring.map((p) => p.x)), Math.min(...ring.map((p) => p.y)), Math.max(...ring.map((p) => p.y))];
      for (let j = Math.max(0, Math.floor((ry0 - y0) / CELL)); j <= Math.min(ny - 1, Math.ceil((ry1 - y0) / CELL)); j++) for (let i = Math.max(0, Math.floor((rx0 - x0) / CELL)); i <= Math.min(nx - 1, Math.ceil((rx1 - x0) / CELL)); i++) if (inside(ring, x0 + i * CELL, y0 + j * CELL)) grid[j * nx + i] = 1;
    }
  return grid;
};
const boundary = (grid: Uint8Array): [number, number][] => {
  const out: [number, number][] = [];
  for (let j = 1; j < ny - 1; j++) for (let i = 1; i < nx - 1; i++) if (grid[j * nx + i] && !(grid[j * nx + i - 1] && grid[j * nx + i + 1] && grid[(j - 1) * nx + i] && grid[(j + 1) * nx + i])) out.push([i, j]);
  return out;
};
const [drawn, volume] = [boundary(raster(groups.drawn)), boundary(raster(groups.volume))];
const shapes = state.shapes[view as "plan"];
const farthest = (from: [number, number][], to: [number, number][], name: string): void => {
  let worst = { d: -1, i: 0, j: 0 };
  for (const [i, j] of from) {
    let best = Infinity;
    for (const [oi, oj] of to) best = Math.min(best, (oi - i) ** 2 + (oj - j) ** 2);
    if (best > worst.d) worst = { d: best, i, j };
  }
  const [x, y] = [x0 + worst.i * CELL, y0 + worst.j * CELL];
  const near = shapes.filter((e) => e.rings.some((ring) => ring.some((p) => Math.hypot(p.x - x, p.y - y) < 0.02) || inside(ring, x, y))).map((e) => e.shape.id);
  console.log(`${name}: ${(Math.sqrt(worst.d) * CELL).toFixed(4)} m at (${x.toFixed(3)}, ${y.toFixed(3)}), shapes near: ${near.join(" ")}`);
};
farthest(drawn, volume, "drawn boundary farthest from the volume boundary");
farthest(volume, drawn, "volume boundary farthest from the drawn boundary");
console.log(`hausdorff ${m.hausdorffM.toFixed(4)} iou ${m.iou.toFixed(4)} edge ${m.edge ? "yes" : "no"} keys ${Object.keys(m).join(",")}`);
