/** 🪓️ Finds the bone whose drawing and body disagree in one view of `humans/kneeling-gardener`: the product's own silhouette measure of the candidate document with only one bone kept at a time (body solids and drawn shapes of that bone).
 * Usage: `bun r3-kneeling-gardener-bisect.ts <scale> <elevation|front|plan>` */
import { readFileSync } from "node:fs";
import { measureAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { CANDIDATE } from "./r3-kneeling-gardener-figure.ts";
const asset = JSON.parse(readFileSync(CANDIDATE, "utf8")).asset;
const scale = Number(process.argv[2]);
const view = process.argv[3] as "front";
const feature = 0.00015 * scale;
const only = (keep: (bone: string, id: string) => boolean): unknown => {
  const a = JSON.parse(JSON.stringify(asset));
  const v = a.volumes[String(scale)];
  v.body = v.body.filter((s: { bone: string; id: string }) => keep(s.bone, s.id));
  const g = a.representations[String(scale)].geometry;
  for (const k of Object.keys(g)) g[k] = g[k].filter((s: { bone: string; id: string }) => keep(s.bone, s.id));
  return a;
};
const all = measureAsset(asset, scale, undefined, undefined, feature).views[view] as { iou: number; p95M: number; hausdorffM: number };
console.log("all".padEnd(14), all.iou.toFixed(4), all.p95M.toFixed(4), all.hausdorffM.toFixed(4));
const ids = process.argv[4];
const names: string[] = ids === undefined ? asset.rig.bones.map((b: { id: string }) => b.id) : asset.volumes[String(scale)].body.filter((s: { bone: string }) => s.bone === ids).map((s: { id: string }) => s.id);
for (const name of names) {
  try {
    const m = measureAsset(only((bone, id) => (ids === undefined ? bone === name : bone === ids && (id === name || id.startsWith(name)))) as never, scale, undefined, undefined, feature).views[view] as { iou: number; p95M: number; hausdorffM: number };
    if (m.hausdorffM > 0.02) console.log(name.padEnd(14), m.iou.toFixed(4), m.p95M.toFixed(4), m.hausdorffM.toFixed(4));
  } catch (e) {
    console.log(name, "error", (e as Error).message.slice(0, 80));
  }
}
if (ids === undefined)
  for (const name of names) {
    const m = measureAsset(only((bone) => bone !== name) as never, scale, undefined, undefined, feature).views[view] as { iou: number; p95M: number; hausdorffM: number };
    if (Math.abs(m.p95M - all.p95M) > 0.002 || Math.abs(m.hausdorffM - all.hausdorffM) > 0.002) console.log("without", name.padEnd(14), m.iou.toFixed(4), m.p95M.toFixed(4), m.hausdorffM.toFixed(4));
  }
