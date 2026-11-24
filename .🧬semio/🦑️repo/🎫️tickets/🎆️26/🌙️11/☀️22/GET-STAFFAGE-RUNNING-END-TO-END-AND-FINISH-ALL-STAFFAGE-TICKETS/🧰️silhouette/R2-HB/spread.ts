/** 🧭️ The drawn shapes that make the lateral extent of the front view and of the plan in one pose (the `spread-front-x-plan-y` check of the three-views sweep), optional ops applied first: bun spread.ts <slug|file.json> <scale> <pose> [ops.json|-] [count] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { applyOps } from "./ops.ts";
import { fileOf } from "./people.ts";
const [name, sc, pose, opsArg, count] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
if (opsArg && opsArg !== "-") applyOps(asset, JSON.parse(await Bun.file(opsArg).text()));
const state: any = evaluate(asset, { kind: "pose", pose: pose! }, { kind: "scale", scale: { denominator: Number(sc) } });
for (const [view, axis] of [["front", "x"], ["plan", "y"], ["elevation", "x"], ["plan", "x"], ["elevation", "y"], ["front", "y"]] as const) {
  const rows = (state.shapes[view] as any[]).filter((s) => s.shape.role !== "ground" && s.rings.flat().length > 0).map((s) => { const v = s.rings.flat().map((p: any) => p[axis] as number); return { id: `${s.shape.id}[${s.shape.bone}]`, lo: Math.min(...v), hi: Math.max(...v) }; });
  const n = Number(count ?? 3);
  console.log(`${view} ${axis}: [${Math.min(...rows.map((r) => r.lo)).toFixed(3)}, ${Math.max(...rows.map((r) => r.hi)).toFixed(3)}]  min: ${[...rows].sort((a, b) => a.lo - b.lo).slice(0, n).map((r) => `${r.id} ${r.lo.toFixed(3)}`).join(", ")}  max: ${[...rows].sort((a, b) => b.hi - a.hi).slice(0, n).map((r) => `${r.id} ${r.hi.toFixed(3)}`).join(", ")}`);
}
