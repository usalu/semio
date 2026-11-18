/** 🧮 Fits the profile of every solid-drawn filled body shape (extruded or revolved plate) of one scale and view to the projected outline of the volume solids of its bone by scaling the profile along and across (lockstep coordinate search on the IoU of the bone).
 *  bun vegetation-fit.ts <asset> <scale> <view> <out.json> [bone,bone,...] */
import { writeFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, boundsOf, referenceOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, scaleArg, view, out, boneArg] = process.argv.slice(2) as string[];
const scale = Number(scaleArg);
const only = boneArg === undefined ? undefined : new Set(boneArg.split(","));
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const feature = featureOf(scale);
const geo = (asset.representations as any)[String(scale)].geometry[view] as any[];
const originals = geo.map((s) => JSON.parse(JSON.stringify(s)));
const scalePath = (d: string, su: number, sv: number): string => {
  if (/[mlcqshvazt]/.test(d)) throw new Error("relative path");
  let k = 0;
  return d.replace(/[MLCQSTHVZ]|-?\d+\.?\d*(?:e-?\d+)?/g, (tok) => {
    if (/[A-Z]/.test(tok)) { k = 0; return tok; }
    const v = Number(tok) * (k % 2 === 0 ? su : sv);
    k++;
    return String(+v.toFixed(5));
  });
};
const shaped = (o: any, su: number, sv: number): any => {
  const c = JSON.parse(JSON.stringify(o));
  if (c.kind === "poly") c.points = c.points.map(([x, y]: number[]) => [x * su, y * sv]);
  else if (c.kind === "path") c.d = scalePath(c.d, su, sv);
  return c;
};
const targets = originals.map((o, i) => ({ o, i })).filter(({ o }) => o.solid !== undefined && o.filled === true && o.role === "body" && (o.kind === "poly" || o.kind === "path") && (only === undefined || only.has(o.bone)));
const state0 = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
const solids = worldSolids(asset, state0.bones as never, scale).filter((s) => s.texture !== true && s.slender !== true);
const cell = feature / 2;
type Ref = { box: { minX: number; minY: number; maxX: number; maxY: number }; v: Mask };
const refs = new Map<string, Ref>();
for (const { o } of targets) {
  if (refs.has(o.bone)) continue;
  const own = solids.filter((s) => s.bone === o.bone);
  if (own.length === 0) continue;
  const groups = referenceOf(view as never, own, undefined, undefined).cut;
  const b = groups.reduce((a, g) => { const c = boundsOf(g); return [Math.min(a[0]!, c[0]), Math.max(a[1]!, c[1]), Math.min(a[2]!, c[2]), Math.max(a[3]!, c[3])]; }, [Infinity, -Infinity, Infinity, -Infinity]);
  const pad = 0.5 + 0 * cell;
  const box = { minX: b[0]! - pad * 0.2 - 2 * cell, minY: b[2]! - pad * 0.2 - 2 * cell, maxX: b[1]! + pad * 0.2 + 2 * cell, maxY: b[3]! + pad * 0.2 + 2 * cell };
  const v = new Mask(box, cell);
  for (const g of groups) v.fill(g, true);
  v.seal();
  refs.set(o.bone, { box, v });
}
const params = targets.map(() => ({ su: 1, sv: 1 }));
const iouOf = (state: any, t: { o: any; i: number }): number => {
  const ref = refs.get(t.o.bone);
  if (ref === undefined) return 1;
  const ev = (state.shapes[view] as any[]).find((s) => s.shape.id === t.o.id && s.shape.bone === t.o.bone);
  if (ev === undefined) return 0;
  const d = new Mask(ref.box, cell);
  d.fill(ev.rings);
  let [both, either] = [0, 0];
  for (let i = 0; i < d.cells.length; i++) { both += d.cells[i]! & ref.v.cells[i]!; either += d.cells[i]! | ref.v.cells[i]!; }
  return either === 0 ? 1 : both / either;
};
const run = (ps: { su: number; sv: number }[]): any => {
  targets.forEach((t, k) => { geo[t.i] = shaped(t.o, ps[k]!.su, ps[k]!.sv); });
  return evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
};
let state = run(params);
const start = targets.map((t) => iouOf(state, t));
let best = [...start];
for (const delta of [0.12, 0.06, 0.03, 0.015, 0.008]) {
  for (const axis of ["su", "sv"] as const) {
    for (const sign of [1, -1]) {
      const trial = params.map((p) => ({ ...p, [axis]: p[axis] * (1 + sign * delta) }));
      state = run(trial);
      targets.forEach((t, k) => {
        const v = iouOf(state, t);
        if (v > best[k]! + 1e-4) { best[k] = v; params[k] = trial[k]!; }
      });
    }
  }
}
targets.forEach((t, k) => { geo[t.i] = originals[t.i]; void k; });
const result = targets.map((t, k) => ({ scale, view, id: t.o.id, bone: t.o.bone, su: +params[k]!.su.toFixed(4), sv: +params[k]!.sv.toFixed(4), iou0: +start[k]!.toFixed(4), iou1: +best[k]!.toFixed(4) }));
writeFileSync(out, JSON.stringify(result));
console.log(`[DEBUG] fit ${name} 1:${scale} ${view}: ${targets.length} shapes, mean iou ${(start.reduce((s, v) => s + v, 0) / Math.max(1, start.length)).toFixed(3)} -> ${(best.reduce((s, v) => s + v, 0) / Math.max(1, best.length)).toFixed(3)}`);
