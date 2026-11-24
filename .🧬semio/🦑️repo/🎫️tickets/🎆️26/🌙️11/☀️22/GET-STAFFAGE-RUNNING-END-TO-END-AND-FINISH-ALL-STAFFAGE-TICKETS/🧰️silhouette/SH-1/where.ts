/** 📍️ Where the drawn shapes and the volume solids of a person lie in a view (default pose): the extents in millimetres of every shape and solid whose id or bone contains the filter; optional ops applied first.
 *  bun where.ts <slug|file.json> <scale> <view> <filter> [ops.json] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { applyOps } from "./ops.ts";
import { fileOf } from "./people.ts";

const [name, scaleArg, view, filter, opsArg] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
if (opsArg && opsArg !== "-") applyOps(asset, JSON.parse(await Bun.file(opsArg).text()));
const scale = Number(scaleArg);
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const mm = (x: number): string => (x * 1000).toFixed(0);
const hit = (id: string, bone?: string): boolean => filter!.split(",").some((f) => id.includes(f) || (bone ?? "").includes(f));
for (const s of state.shapes[view!] as any[]) if (hit(s.shape.id, s.shape.bone) && s.rings.length > 0) { const p = s.rings.flat(); console.log(`drawn  ${s.shape.id}[${s.shape.bone}] ${s.shape.role}${s.closed ? "" : " open"}${s.shape.filled === false ? " unfilled" : ""}${s.shape.solid ? " " + s.shape.solid.kind : ""}: x ${mm(Math.min(...p.map((q: any) => q.x)))} … ${mm(Math.max(...p.map((q: any) => q.x)))}, y ${mm(Math.min(...p.map((q: any) => q.y)))} … ${mm(Math.max(...p.map((q: any) => q.y)))}`); }
for (const w of worldSolids(asset, state.bones as never, scale)) if (hit(w.id, w.bone)) { let x0 = Infinity, x1 = -Infinity, y0 = Infinity, y1 = -Infinity; for (let i = 0; i < w.positions.length; i += 3) { const p = project(view as never, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); x0 = Math.min(x0, p.x); x1 = Math.max(x1, p.x); y0 = Math.min(y0, p.y); y1 = Math.max(y1, p.y); } console.log(`volume ${w.id}[${w.bone}]${w.slender ? " slender" : ""}${w.texture ? " texture" : ""}: x ${mm(x0)} … ${mm(x1)}, y ${mm(y0)} … ${mm(y1)}`); }
