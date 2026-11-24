/** 🧭️ What bounds a view of a person: the drawn shapes and the volume solids that reach furthest left, right, down and up in the default pose, with their extents in millimetres.
 *  bun extreme.ts <slug|file.json> <scale> <view> [count] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { fileOf } from "./people.ts";

const [name, scaleArg, view, countArg] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const scale = Number(scaleArg), count = Number(countArg ?? 4);
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
type Row = { id: string; x0: number; x1: number; y0: number; y1: number };
const drawn: Row[] = (state.shapes[view!] as any[]).filter((s) => s.shape.role !== "ground" && s.rings.length > 0).map((s) => { const p = s.rings.flat(); return { id: `${s.shape.id}[${s.shape.bone}${s.shape.role === "body" ? "" : ":" + s.shape.role}${s.shape.filled === false ? ":open" : ""}]`, x0: Math.min(...p.map((q: any) => q.x)), x1: Math.max(...p.map((q: any) => q.x)), y0: Math.min(...p.map((q: any) => q.y)), y1: Math.max(...p.map((q: any) => q.y)) }; });
const solids: Row[] = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true).map((w) => { let x0 = Infinity, x1 = -Infinity, y0 = Infinity, y1 = -Infinity; for (let i = 0; i < w.positions.length; i += 3) { const p = project(view as never, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); x0 = Math.min(x0, p.x); x1 = Math.max(x1, p.x); y0 = Math.min(y0, p.y); y1 = Math.max(y1, p.y); } return { id: `${w.id}[${w.bone}]`, x0, x1, y0, y1 }; });
const mm = (x: number): string => (x * 1000).toFixed(1);
for (const [what, rows] of [["drawn", drawn], ["volume", solids]] as const) {
  console.log(`${what} 1:${scale} ${view}: x ${mm(Math.min(...rows.map((r) => r.x0)))} … ${mm(Math.max(...rows.map((r) => r.x1)))}, y ${mm(Math.min(...rows.map((r) => r.y0)))} … ${mm(Math.max(...rows.map((r) => r.y1)))}`);
  console.log(`  left:  ${rows.slice().sort((a, b) => a.x0 - b.x0).slice(0, count).map((r) => `${r.id} ${mm(r.x0)}`).join("  ")}`);
  console.log(`  right: ${rows.slice().sort((a, b) => b.x1 - a.x1).slice(0, count).map((r) => `${r.id} ${mm(r.x1)}`).join("  ")}`);
  console.log(`  low:   ${rows.slice().sort((a, b) => a.y0 - b.y0).slice(0, count).map((r) => `${r.id} ${mm(r.y0)}`).join("  ")}`);
  console.log(`  high:  ${rows.slice().sort((a, b) => b.y1 - a.y1).slice(0, count).map((r) => `${r.id} ${mm(r.y1)}`).join("  ")}`);
}
