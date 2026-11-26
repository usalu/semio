/** 🧭️ Where the silhouette guard sees its distance: `bun guard-where.ts <json> <scale> <view> [<cell m, default 0.02>]` prints the guard numbers and, from a raster (even-odd) of the drawn filled shapes (`filled`) and of the reference (`reference`) of `measureView`, the connected regions where exactly one of the two covers a cell: size, box, and whether the region touches the outside of the union (a channel) */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const sil = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
const [file, scaleArg, view, cellArg] = process.argv.slice(2);
const scale = Number(scaleArg), cell = Number(cellArg ?? 0.02);
const asset = readDoc(file!).doc.asset;
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = sil.worldSolids(asset, state.bones, scale).filter((w: any) => w.texture !== true);
const m = sil.measureView(asset, state, solids, view, scale, undefined, featureOf(scale), undefined, true);
console.log(`iou ${m.iou.toFixed(4)} hausdorff ${(m.hausdorffM * 1000).toFixed(1)} mm; drawn ${m.counts.drawn} volume ${m.counts.volume} cells; filled groups ${m.filled.length}, reference groups ${m.reference.length}`);
type R = { x: number; y: number }[];
const all = [...m.filled, ...m.reference].flat().flat() as { x: number; y: number }[];
const [x0, y0] = [Math.min(...all.map((p) => p.x)), Math.min(...all.map((p) => p.y))];
const [w, h] = [Math.ceil((Math.max(...all.map((p) => p.x)) - x0) / cell) + 2, Math.ceil((Math.max(...all.map((p) => p.y)) - y0) / cell) + 2];
const raster = (groups: readonly (readonly R[])[]): Uint8Array => {
  const out = new Uint8Array(w * h);
  for (const group of groups) {
    const g = new Uint8Array(w * h);
    for (let j = 0; j < h; j++) {
      const y = y0 + (j + 0.5) * cell;
      const xs: number[] = [];
      for (const ring of group) for (let i = 0; i < ring.length; i++) { const a = ring[i]!, b = ring[(i + 1) % ring.length]!; if (a.y > y !== b.y > y) xs.push(a.x + ((y - a.y) / (b.y - a.y)) * (b.x - a.x)); }
      xs.sort((p, q) => p - q);
      for (let k = 0; k + 1 < xs.length; k += 2) for (let i = Math.max(0, Math.ceil((xs[k]! - x0) / cell - 0.5)); i <= Math.min(w - 1, Math.floor((xs[k + 1]! - x0) / cell - 0.5)); i++) g[j * w + i] = 1;
    }
    for (let k = 0; k < g.length; k++) out[k] |= g[k]!;
  }
  return out;
};
const [drawn, ref] = [raster(m.filled as never), raster(m.reference as never)];
const only = new Uint8Array(w * h);
let nd = 0, nr = 0;
for (let k = 0; k < only.length; k++) { if (drawn[k] && !ref[k]) { only[k] = 1; nd++; } else if (ref[k] && !drawn[k]) { only[k] = 2; nr++; } }
console.log(`raster ${w}x${h} cells of ${cell} m: drawn only ${nd}, reference only ${nr}`);
const seen = new Uint8Array(w * h);
const regions: { kind: number; n: number; box: number[] }[] = [];
for (let s = 0; s < only.length; s++) {
  if (!only[s] || seen[s]) continue;
  const kind = only[s]!, stack = [s]; seen[s] = 1; let n = 0; const box = [Infinity, Infinity, -Infinity, -Infinity];
  while (stack.length) { const k = stack.pop()!; n++; const [i, j] = [k % w, Math.floor(k / w)]; box[0] = Math.min(box[0]!, i); box[1] = Math.min(box[1]!, j); box[2] = Math.max(box[2]!, i); box[3] = Math.max(box[3]!, j); for (const [di, dj] of [[1, 0], [-1, 0], [0, 1], [0, -1]] as const) { const [ii, jj] = [i + di, j + dj]; if (ii < 0 || jj < 0 || ii >= w || jj >= h) continue; const q = jj * w + ii; if (only[q] === kind && !seen[q]) { seen[q] = 1; stack.push(q); } } }
  regions.push({ kind, n, box: box as number[] });
}
regions.sort((a, b) => b.n - a.n);
for (const r of regions.slice(0, 8)) console.log(`${r.kind === 1 ? "drawn only" : "reference only"} ${r.n} cells (${(r.n * cell * cell).toFixed(3)} m2) x ${(x0 + r.box[0]! * cell).toFixed(2)}..${(x0 + (r.box[2]! + 1) * cell).toFixed(2)} y ${(y0 + r.box[1]! * cell).toFixed(2)}..${(y0 + (r.box[3]! + 1) * cell).toFixed(2)}`);
const holes = (mask: Uint8Array): { n: number; box: number[] }[] => {
  const seen = new Uint8Array(w * h);
  const out: { n: number; box: number[] }[] = [];
  for (let s = 0; s < mask.length; s++) {
    if (mask[s] || seen[s]) continue;
    const stack = [s]; seen[s] = 1; let n = 0, outside = false; const box = [Infinity, Infinity, -Infinity, -Infinity];
    while (stack.length) { const k = stack.pop()!; n++; const [i, j] = [k % w, Math.floor(k / w)]; if (i === 0 || j === 0 || i === w - 1 || j === h - 1) outside = true; box[0] = Math.min(box[0]!, i); box[1] = Math.min(box[1]!, j); box[2] = Math.max(box[2]!, i); box[3] = Math.max(box[3]!, j); for (const [di, dj] of [[1, 0], [-1, 0], [0, 1], [0, -1]] as const) { const [ii, jj] = [i + di, j + dj]; if (ii < 0 || jj < 0 || ii >= w || jj >= h) continue; const q = jj * w + ii; if (!mask[q] && !seen[q]) { seen[q] = 1; stack.push(q); } } }
    if (!outside && n >= Number(process.env.MINCELLS ?? 3)) out.push({ n, box });
  }
  return out.sort((a, b) => b.n - a.n);
};
for (const [name, mask] of [["drawn", drawn], ["reference", ref]] as const) {
  const list = holes(mask);
  console.log(`${name}: ${list.length} holes of at least 3 cells; largest ${list.slice(0, 6).map((r) => `${(r.n * cell * cell).toFixed(3)} m2 at x ${(x0 + r.box[0]! * cell).toFixed(2)}..${(x0 + (r.box[2]! + 1) * cell).toFixed(2)} y ${(y0 + r.box[1]! * cell).toFixed(2)}..${(y0 + (r.box[3]! + 1) * cell).toFixed(2)}`).join("; ")}`);
}

const pin = holes(drawn).filter((r) => { let cov = 0, tot = 0; for (let j = r.box[1]!; j <= r.box[3]!; j++) for (let i = r.box[0]!; i <= r.box[2]!; i++) if (!drawn[j * w + i]) { tot++; cov += ref[j * w + i]!; } return tot > 0 && cov / tot > 0.5; });
console.log(`holes of the drawn mask that stand inside the reference body (the reference is covered there): ${pin.length}; ${pin.slice(0, 10).map((r) => `${r.n} cells at x ${(x0 + r.box[0]! * cell).toFixed(2)}..${(x0 + (r.box[2]! + 1) * cell).toFixed(2)} y ${(y0 + r.box[1]! * cell).toFixed(2)}..${(y0 + (r.box[3]! + 1) * cell).toFixed(2)}`).join("; ")}`);
const back = holes(ref).filter((r) => { let cov = 0, tot = 0; for (let j = r.box[1]!; j <= r.box[3]!; j++) for (let i = r.box[0]!; i <= r.box[2]!; i++) if (!ref[j * w + i]) { tot++; cov += drawn[j * w + i]!; } return tot > 0 && cov / tot > 0.5; });
console.log(`holes of the reference that stand inside the drawn body (the drawing is covered there): ${back.length}; ${back.slice(0, 10).map((r) => `${r.n} cells at x ${(x0 + r.box[0]! * cell).toFixed(2)}..${(x0 + (r.box[2]! + 1) * cell).toFixed(2)} y ${(y0 + r.box[1]! * cell).toFixed(2)}..${(y0 + (r.box[3]! + 1) * cell).toFixed(2)}`).join("; ")}`);
