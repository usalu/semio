/** 🔬 Patched variant (env PATCH=ops.json) of silhouette-diag.ts. Where the drawing and the volume of one asset disagree: worst mismatch clusters per view at one scale, in the world frame of the view (paper-view x, y in metres), with the drawn shape or volume solid nearest.
 *  bun silhouette-diag.ts <asset> <scale> [view] */
import { readFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { distanceToTriangle, project } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { GATE, Mask, VIEWPOINTS, boundaryOf, distanceField, fillSmallHoles, extentOf, measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

void readFileSync;
const [name, scaleArg, only, px, py, pr] = process.argv.slice(2) as [string, string, string | undefined, string | undefined, string | undefined, string | undefined];
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
if (process.env.PATCH) {
  const find = (list: any[], id: string) => list.find((x) => x.id === id);
  for (const o of JSON.parse(await Bun.file(process.env.PATCH).text())) {
    const shapes = (): any[] => (asset as any).representations[o.scale].geometry[o.view];
    const solids = (): any[] => (asset as any).volumes[o.scale].body;
    const [list, key] = o.op.startsWith("shape") ? [shapes(), "shape"] : [solids(), "solid"];
    if (o.op.endsWith("-add")) list.splice(list.indexOf(find(list, o.after)) + 1, 0, o[key]);
    else if (o.op.endsWith("-set")) Object.assign(find(list, o.id), o.set);
    else list[list.indexOf(find(list, o.id))] = o[key];
  }
}
const feature = featureOf(scale);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const e = extentOf(solids);
console.log("extent min", e.min.map((x) => +x.toFixed(3)), "max", e.max.map((x) => +x.toFixed(3)), "dims", asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM, "feature", feature);
for (const view of VIEWPOINTS) {
  if (only !== undefined && only !== view) continue;
  const m = measureView(asset, state, solids, view, scale, undefined, feature, Infinity, false);
  const cell = m.cell;
  const box = m.box;
  const drawn = new Mask(box, cell);
  for (const g of m.filled) drawn.fill(g);
  for (const g of m.outlines) drawn.fill(g);
  const vol = new Mask(box, cell);
  for (const g of m.reference) vol.fill(g, true);
  if (m.section) for (const g of m.visible) vol.fill(g, true);
  vol.seal();
  const [w, h] = [vol.width, vol.height];
  const holes = (4 * feature / cell) ** 2;
  const dF = fillSmallHoles(drawn.cells, w, h, Math.round(holes));
  const vF = fillSmallHoles(vol.cells, w, h, Math.round(holes));
  const bd = boundaryOf(dF, w, h);
  const bv = boundaryOf(vF, w, h);
  const toVol = distanceField(bv, w, h);
  const toDrawn = distanceField(bd, w, h);
  const limit = 2 * feature;
  const clusters: { side: string; x: number; y: number; d: number }[] = [];
  const seen = new Set<number>();
  for (const [side, from, field] of [["drawn-only", bd, toVol], ["volume-only", bv, toDrawn]] as const) {
    const list: { i: number; j: number; d: number }[] = [];
    for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (from[j * w + i] && Math.sqrt(field[j * w + i]!) * cell > limit) list.push({ i, j, d: Math.sqrt(field[j * w + i]!) * cell });
    list.sort((a, b) => b.d - a.d);
    seen.clear();
    for (const c of list) {
      const key = Math.round(c.i / (0.15 / cell)) * 100000 + Math.round(c.j / (0.15 / cell));
      if (seen.has(key)) continue;
      seen.add(key);
      clusters.push({ side, x: +(box.minX + (c.i + 0.5) * cell).toFixed(3), y: +(box.minY + (c.j + 0.5) * cell).toFixed(3), d: +c.d.toFixed(3) });
      if (clusters.length > 400) break;
    }
  }
  if (px !== undefined && py !== undefined) {
    const r = Number(pr ?? 0.1);
    const step = Math.max(1, Math.round((2 * r) / cell / 70));
    console.log(`x from ${(Number(px) - r).toFixed(3)} step ${(step * cell).toFixed(4)}  y step ${(step * cell * 2).toFixed(4)}`);
    for (let y = Number(py) + r; y > Number(py) - r; y -= step * cell * 2) {
      let line = "";
      for (let x = Number(px) - r; x < Number(px) + r; x += step * cell) {
        const i = Math.floor((x - box.minX) / cell);
        const j = Math.floor((y - box.minY) / cell);
        const k = j * w + i;
        if (process.env.FILLED) {
          const [fa, fb, ba, bb] = [dF[k], vF[k], bd[k], bv[k]];
          line += ba && bb ? "*" : ba ? "d" : bb ? "v" : fa && fb ? "#" : fa ? "D" : fb ? "V" : ".";
        } else {
          const [a, b] = [drawn.cells[k], vol.cells[k]];
          line += a && b ? "#" : a ? "D" : b ? "V" : ".";
        }
      }
      console.log(`${y.toFixed(3)} ${line}`);
    }
  }
  console.log(`== ${view} 1:${scale} iou ${m.iou.toFixed(3)} hausdorff ${m.hausdorffM.toFixed(4)} limit ${limit} section ${m.section} drawn=${m.counts.drawn} vol=${m.counts.volume}`);
  const out = clusters.sort((a, b) => b.d - a.d).slice(0, Number(process.env.NC ?? 8));
  for (const c of out) {
    const P = { x: c.x, y: c.y };
    const dseg = (a: any, b: any) => { const dx = b.x - a.x, dy = b.y - a.y, l = dx * dx + dy * dy; let t = l === 0 ? 0 : ((P.x - a.x) * dx + (P.y - a.y) * dy) / l; t = Math.max(0, Math.min(1, t)); return Math.hypot(P.x - a.x - t * dx, P.y - a.y - t * dy); };
    const ds: string[] = [];
    for (const s of (state.shapes as any)[view]) { let d = Infinity; for (const ring of s.rings) for (let i = 0; i < ring.length - 1 + (s.closed ? 1 : 0); i++) d = Math.min(d, dseg(ring[i], ring[(i + 1) % ring.length])); if (d < 0.05 && s.shape.role !== "texture") ds.push(`${s.shape.id}:${d.toFixed(3)}`); }
    const vs: string[] = [];
    for (const w of solids) { let d = Infinity; for (let k = 0; k < w.indices.length; k += 3) { const t = [0, 1, 2].map((j) => { const i = w.indices[k + j]! * 3; return project(view as never, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); }) as [any, any, any]; d = Math.min(d, distanceToTriangle(P, t)); } if (d < 0.05) vs.push(`${w.id}:${d.toFixed(3)}`); }
    console.log(`  ${c.side} at (${c.x}, ${c.y}) distance ${c.d}  D[${ds.join(" ")}] V[${vs.join(" ")}]`);
  }
}
void GATE;
