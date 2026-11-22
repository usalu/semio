/** 🎨️ The paint order of one view of a pose (farthest first) with each shape's authored z, depth now and at rest: bun order.ts <slug|file.json> <scale> <view> <pose> [filter] */
import { join, resolve } from "node:path";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { prepare, sourceOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts";
import { evaluateDrawing, restShapeDepth, shapeDepth } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const [arg, scale, view, pose, filter] = process.argv.slice(2);
const asset = (await Bun.file(arg!.endsWith(".json") ? arg! : join(catalog, arg!, "🔣️.json")).json()).asset;
const query = pose!.includes("@") ? { kind: "clip", clip: pose!.split("@")[0], timeS: Number(pose!.split("@")[1]) } : { kind: "pose", pose };
const state: any = (evaluateDrawing as any)(asset, query, { kind: "scale", scale: { denominator: Number(scale) } });
const prepared = prepare(state, view as never, sourceOf(state), 1000 / Number(scale));
void evaluate;
for (const p of prepared) {
  if (!p.shape.filled || (filter && !new RegExp(filter).test(`${p.shape.id} ${p.shape.bone}`))) continue;
  console.log(String(p.order).padStart(3), p.shape.id.padEnd(22), p.shape.bone.padEnd(14), "z", String(p.shape.z).padEnd(5), "now", shapeDepth(state, p.shape, view as never).toFixed(3), "rest", restShapeDepth(asset, p.shape, view as never).toFixed(3), "box", [p.box.minX, p.box.minY, p.box.maxX, p.box.maxY].map((v) => v.toFixed(2)).join(","));
}
