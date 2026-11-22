/** 📏️ Lists the drawn shapes that reach furthest along each paper axis in one view, scale and pose (default pose unless POSE): bun extents.ts <slug> <scale> <view> */
import { join } from "node:path";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { HUMANS } from "./lib.ts";
const [slug, scale, view] = process.argv.slice(2) as [string, string, "elevation" | "front" | "plan"];
const asset = (await Bun.file(join(HUMANS, slug, "🔣️.json")).json()).asset;
const state = evaluate(asset, { kind: "pose", pose: process.env.POSE ?? asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scale) } } as never) as any;
const shapes = state.shapes[view] as any[];
const rows = shapes.map((e: any) => {
  const pts = e.rings.flat();
  return { id: e.shape.id, x0: Math.min(...pts.map((p: any) => p.x)), x1: Math.max(...pts.map((p: any) => p.x)), y0: Math.min(...pts.map((p: any) => p.y)), y1: Math.max(...pts.map((p: any) => p.y)) };
});
for (const k of ["x0", "x1", "y0", "y1"] as const) console.log(k, rows.sort((a: any, b: any) => (k.endsWith("0") ? a[k] - b[k] : b[k] - a[k])).slice(0, 4).map((r: any) => `${r.id} ${r[k].toFixed(3)}`).join(" | "));
