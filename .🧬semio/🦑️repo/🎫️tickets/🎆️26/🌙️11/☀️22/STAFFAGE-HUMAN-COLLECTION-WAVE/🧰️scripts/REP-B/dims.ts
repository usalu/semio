/** 📐️ Declares the dimensions the body has: an axis that `volumes-bounds` reports at a scale becomes the extent of the body of 1:50 in the default pose (to the millimetre), after checking that every other authored scale then lies within its own limit. Keeps one `dims` op at the head of the repair file of the person.
 *  bun dims.ts <slug> [--file <asset.json>] [--dry] [--axes widthM,heightM,depthM] [--fit]
 *  `--fit`: where the drawings of some scale still stand further from that extent than `validateAsset` allows (2 % or 15 mm), the axis is declared at the value nearest to the body that every drawn view of every scale accepts, and what the body then misses is printed. */
import { parseArgs } from "node:util";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { viewExtent } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { bodyDimensions, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { applyOps, readOps, writeOps, type Op } from "./ops.ts";
import { catalogFile, opsFile } from "./people.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { file: { type: "string" }, dry: { type: "boolean" }, axes: { type: "string" }, fit: { type: "boolean" } } });
const slug = positionals[0]!;
const asset = (await Bun.file(values.file ?? catalogFile(slug)).json()).asset;
const existing = await readOps(opsFile(slug));
applyOps(asset, existing);
const AXES = ["widthM", "heightM", "depthM"] as const;
const scales = [50, 100, 200, 500].filter((d) => asset.volumes[String(d)]?.status === "authored");
const boxes = Object.fromEntries(scales.map((d) => {
  const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: d } });
  return [d, bodyDimensions(worldSolids(asset, state.bones as never, d))];
})) as Record<number, readonly number[]>;
const limit = (d: number, declared: number): number => Math.max(featureOf(d), 0.02 * declared);
const drawn: Record<string, number[]> = { widthM: [], heightM: [], depthM: [] };
if (values.fit) for (const d of scales) {
  const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: d } });
  const [e, f, p] = (["elevation", "front", "plan"] as const).map((v) => viewExtent(state, v));
  if (e) { drawn.widthM!.push(e.horizontal); drawn.heightM!.push(e.vertical); }
  if (f) { drawn.depthM!.push(f.horizontal); drawn.heightM!.push(f.vertical); }
  if (p) { drawn.widthM!.push(p.horizontal); drawn.depthM!.push(p.vertical); }
}
const set: Record<string, number> = {};
AXES.forEach((axis, k) => {
  const declared = asset.dimensions[axis] as number;
  const failing = scales.some((d) => Math.abs(boxes[d]![k]! - declared) > featureOf(d));
  if (values.axes !== undefined ? !values.axes.split(",").includes(axis) : !failing && !values.fit) return;
  let next = Number(boxes[scales[0]!]![k]!.toFixed(3));
  if (values.fit && drawn[axis]!.length > 0) {
    const tolerance = Math.max(0.02 * next, 0.015) - 0.0006;
    const [low, high] = [Math.max(...drawn[axis]!) - tolerance, Math.min(...drawn[axis]!) + tolerance];
    if (low > high) console.log(`${slug} ${axis}: the drawings disagree among themselves (${Math.min(...drawn[axis]!).toFixed(3)} to ${Math.max(...drawn[axis]!).toFixed(3)} m): no declared value satisfies all of them`);
    const fitted = Number(Math.min(Math.max(next, Math.min(low, high)), Math.max(low, high)).toFixed(3));
    if (fitted !== next) console.log(`${slug} ${axis}: body ${next} m, drawings ${Math.min(...drawn[axis]!).toFixed(3)} to ${Math.max(...drawn[axis]!).toFixed(3)} m: declared ${fitted} m, the body of 1:${scales[0]} misses it by ${(Math.abs(fitted - next) * 1000).toFixed(1)} mm (one smallest feature is ${(featureOf(scales[0]!) * 1000).toFixed(1)} mm)`);
    next = fitted;
  }
  const off = scales.filter((d) => Math.abs(boxes[d]![k]! - next) > featureOf(d));
  console.log(`${slug} ${axis}: ${declared} -> ${next} (bodies ${scales.map((d) => `1:${d} ${boxes[d]![k]!.toFixed(3)}`).join(", ")})${off.length > 0 ? ` STILL OFF at ${off.map((d) => `1:${d}`).join(", ")}` : ""}`);
  if (next !== declared) set[axis] = next;
  void limit;
});
const ops: Op[] = Object.keys(set).length > 0 ? [{ op: "dims", set }] : [];
if (!values.dry && ops.length > 0) await writeOps(opsFile(slug), [{ op: "dims", set: { ...Object.assign({}, ...existing.filter((o) => o.op === "dims").map((o) => o.set)), ...set } }, ...existing.filter((o) => o.op !== "dims")]);
console.log(`${slug}: ${Object.keys(set).length} dimensions redeclared${values.dry ? " (dry)" : ""}`);
