/** 🧭️ Which shapes reach farthest in a view: `bun reach.ts <id|file.json> <scale> <pose> <view> <left|right|top|bottom> [count, default 6]` evaluates the pose and prints the shapes whose points lie farthest to that side, with bone, kind, the point and the box of the shape, so a spike on a sheet can be traced to the shape that draws it.
 * @see ./segments.ts — the same for straight runs */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { S, fileOf } from "../MR/lib.ts";

const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const [id, scale, pose, view, side, count] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(fileOf(id!), "utf8")).asset;
const state = evaluate(asset, { kind: "pose", pose }, engine.scaleSelector(Number(scale)));
const far = (p: { x: number; y: number }): number => (side === "left" ? -p.x : side === "right" ? p.x : side === "top" ? p.y : -p.y);
const found = state.shapes[view!].filter((entry: any) => entry.rings.flat().length > 0).map((entry: any) => { const points = entry.rings.flat(); const tip = points.reduce((best: any, p: any) => (far(p) > far(best) ? p : best)); return { tip, line: `${entry.shape.id} on ${entry.shape.bone} (${entry.shape.kind}, ${entry.shape.filled === true ? "filled" : "open"}): tip ${tip.x.toFixed(3)},${tip.y.toFixed(3)}; box x ${Math.min(...points.map((p: any) => p.x)).toFixed(3)} … ${Math.max(...points.map((p: any) => p.x)).toFixed(3)}, y ${Math.min(...points.map((p: any) => p.y)).toFixed(3)} … ${Math.max(...points.map((p: any) => p.y)).toFixed(3)}` }; });
for (const entry of found.sort((a: any, b: any) => far(b.tip) - far(a.tip)).slice(0, Number(count ?? 6))) console.log(entry.line);
