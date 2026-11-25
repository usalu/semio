/** 🕵️ Which added plate the silhouette guard objects to: `bun culprit.ts <file.json> <scale> <view>` measures the view in the default pose as `🧊️catalog-silhouette` does (`measureView`, largest boundary distance against twice the smallest feature), then halves the list of plates (`mvc-…`) of that view until one plate is left with which the view fails, and prints it (id, bone, solid, points, its drawn box and area) with the distance with only that plate and with every plate but it. Assumes one culprit; says so when the halves disagree. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const { measureView, worldSolids } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const [arg, key, view] = process.argv.slice(2);
const scale = Number(key);
const asset = readDoc(arg!).doc.asset;
const feature: number = featureOf(scale);
const list: any[] = asset.representations[key!].geometry[view!];
const plates = list.filter((shape) => String(shape.id).startsWith("mvc-"));
const measure = (kept: Set<any>): { m: number; state: any } => {
  const copy = structuredClone(asset);
  copy.representations[key!].geometry[view!] = list.filter((shape) => !String(shape.id).startsWith("mvc-") || kept.has(shape));
  const state: any = evaluate(copy, { kind: "pose", pose: copy.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
  const solids = (worldSolids(copy, state.bones, scale) as any[]).filter((w) => w.texture !== true);
  return { m: measureView(copy, state, solids, view, scale, undefined, feature).hausdorffM as number, state };
};
const limit = 2 * feature;
const [none, all] = [measure(new Set()).m, measure(new Set(plates)).m];
console.log(`${asset.id} 1:${key} ${view}: limit ${(limit * 1000).toFixed(1)} mm, without plates ${(none * 1000).toFixed(1)} mm, with all ${plates.length} plates ${(all * 1000).toFixed(1)} mm`);
if (all > limit && none <= limit) {
  let keep = plates.slice();
  while (keep.length > 1) {
    const half = keep.slice(0, Math.ceil(keep.length / 2));
    const first = measure(new Set(half)).m > limit;
    if (!first && measure(new Set(keep.slice(half.length))).m <= limit) { console.log(`neither half of ${keep.length} plates fails alone: more than one plate takes part (${keep.map((shape) => shape.id).join(", ").slice(0, 400)})`); break; }
    keep = first ? half : keep.slice(half.length);
    console.log(`[DEBUG] ${keep.length} left`);
  }
  if (keep.length === 1) {
    const shape = keep[0];
    const { m, state } = measure(new Set(keep));
    const drawn = (state.shapes[view!] as any[]).find((s) => s.shape.id === shape.id && s.shape.bone === shape.bone);
    const points = (drawn.rings as { x: number; y: number }[][]).flat();
    console.log(`culprit "${shape.id}" on ${shape.bone}: ${JSON.stringify(shape.solid)}, ${shape.points.length} points, ${drawn.rings.length} drawn rings of ${drawn.rings.map((ring: any[]) => ring.length).join("+")} points, closed ${drawn.closed}, x [${Math.min(...points.map((p) => p.x)).toFixed(3)}, ${Math.max(...points.map((p) => p.x)).toFixed(3)}] y [${Math.min(...points.map((p) => p.y)).toFixed(3)}, ${Math.max(...points.map((p) => p.y)).toFixed(3)}]`);
    console.log(`points ${JSON.stringify(shape.points)}`);
    console.log(`rings ${JSON.stringify(drawn.rings.map((ring: any[]) => ring.map((p) => [Number(p.x.toFixed(4)), Number(p.y.toFixed(4))])))}`.slice(0, 1500));
    console.log(`only this plate ${(m * 1000).toFixed(1)} mm, every plate but it ${(measure(new Set(plates.filter((other) => other !== shape))).m * 1000).toFixed(1)} mm`);
  }
}
