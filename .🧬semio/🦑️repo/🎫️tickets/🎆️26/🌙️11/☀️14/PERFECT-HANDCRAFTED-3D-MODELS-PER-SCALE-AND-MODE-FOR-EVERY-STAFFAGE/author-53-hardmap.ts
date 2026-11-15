import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const P = "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
const S = (await import(P)) as any;
const { evaluate } = (await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts")) as any;
const [spec, sc, view = "elevation", strokes = "yes", thr = "8"] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!);
const vol = m.volumes[sc!];
const softRegions = new Set<string>(m.thermal.regions.filter((r: any) => ["foliage", "fur", "feather", "hair"].includes(r.kind)).map((r: any) => r.id));
const soft = new Set<string>(vol.thermal.filter((s: any) => s.of && softRegions.has(s.region)).map((s: any) => s.of));
const feature = 0.15e-3 * Number(sc);
const r = S.measureAsset(m, Number(sc), m.presentation.defaultPose, undefined, feature, soft);
const v = r.views[view];
const state = evaluate(m, { kind: "pose", pose: m.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } });
const worlds = S.worldSolids(m, state.bones, Number(sc)).filter((w: any) => w.texture !== true);
const softW = worlds.filter((w: any) => soft.has(w.id));
const box = v.box, cell = v.cell;
const mk = (groups: any[]) => { const k = new S.Mask(box, cell); for (const g of groups) k.fill(g); return k; };
const ref = mk(v.reference);
const softM = mk(S.referenceOf(view, softW, undefined).cut);
const drawn = mk(strokes === "yes" ? [...v.filled, ...v.outlines] : v.filled);
const W = ref.width, H = ref.height;
const ba = S.boundaryOf(S.fillSmallHoles(drawn.cells, W, H, Math.round((4 * feature / cell) ** 2)), W, H);
const bb = S.boundaryOf(S.fillSmallHoles(ref.cells, W, H, Math.round((4 * feature / cell) ** 2)), W, H);
const bins = new Map<string, number>();
let hard = 0, far = 0;
for (const [name, from, to] of [["drawn->3D", ba, bb], ["3D->drawn", bb, ba]] as const) {
  const f = S.distanceField(to, W, H);
  for (let k = 0; k < from.length; k++) if (from[k]) {
    const isSoft = S.nearWithin(softM.cells, W, H, k, 9) !== Infinity;
    if (isSoft) continue;
    hard++;
    const d = Math.sqrt(f[k]);
    if (d > Number(thr)) {
      far++;
      const x = box.minX + ((k % W) + 0.5) * cell, y = box.minY + (Math.floor(k / W) + 0.5) * cell;
      const key = `${name} (${(Math.round(x / 0.5) * 0.5).toFixed(1)}, ${(Math.round(y / 0.5) * 0.5).toFixed(1)})`;
      bins.set(key, (bins.get(key) ?? 0) + 1);
    }
  }
}
console.log(view, "hard cells", hard, "far (>", thr, "cells)", far, ((100 * far) / hard).toFixed(1), "%");
console.log([...bins.entries()].sort((a, b) => b[1] - a[1]).slice(0, 25).map(([k, n]) => `  ${k}: ${n}`).join("\n"));
