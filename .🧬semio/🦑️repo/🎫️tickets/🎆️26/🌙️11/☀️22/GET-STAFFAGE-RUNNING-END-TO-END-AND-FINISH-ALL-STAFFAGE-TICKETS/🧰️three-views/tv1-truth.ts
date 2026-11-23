/** 🧊️ Says which view is wrong at one instant: the extents of the 3D body of the scale (the `volumes` solids placed by the same evaluated bones) against the extents of the side elevation, the front elevation and the plan: `bun tv1-truth.ts <id>|--file <json> <scale> pose:<id>|clip:<id>@<seconds> [bone…]`. Without bones the whole asset; with bones one line per bone (its solids against its shapes). Columns: ahead x, up y, left z. Each drawn range is followed by the ids of the two shapes that set it. */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const staffage = join(resolve(import.meta.dir, "../../../../../../../.."), "🎓️teaching/🏛️architecture/🧍️staffage");
const { catalog } = await import(join(staffage, "🧪️tests/🧊️three-views/🧰️.ts"));
const { evaluate } = await import(join(staffage, "🔨️modules/🧮️evaluation/🟦️.ts"));
const { worldSolids } = await import(join(staffage, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const args = process.argv.slice(2);
const asset = args[0] === "--file" ? JSON.parse(readFileSync(resolve(args[1]!), "utf8")).asset : catalog().find((candidate: { id: string }) => candidate.id === args[0]);
const [scale, when, ...bones] = args.slice(args[0] === "--file" ? 2 : 1);
const query = when!.startsWith("pose:") ? { kind: "pose", pose: when!.slice(5) } : { kind: "clip", clip: when!.slice(5).split("@")[0], timeS: Number(when!.split("@")[1]) };
const state = evaluate(asset, query, { kind: "scale", scale: { denominator: Number(scale) } });
const solids = (worldSolids(asset, state.bones, Number(scale)) as { id: string; bone?: string; positions: Float64Array; texture?: boolean }[]).filter((w) => w.texture !== true);
type Range = [number, number];
const range = (values: number[]): Range | undefined => (values.length === 0 ? undefined : [Math.min(...values), Math.max(...values)]);
const show = (r: Range | undefined): string => (r === undefined ? "       –        " : `[${r[0].toFixed(3)}, ${r[1].toFixed(3)}]`);
const axis = (list: typeof solids, k: number): Range | undefined => range(list.flatMap((w) => Array.from(w.positions).filter((_, i) => i % 3 === k)));
const drawn = (view: string, bone: string | undefined, k: "x" | "y"): string => {
  const rows = (state.shapes[view] as { shape: { id: string; bone: string; role: string }; rings: { x: number; y: number }[][] }[]).filter((e) => e.shape.role !== "ground" && e.rings.flat().length > 0 && (bone === undefined || e.shape.bone === bone)).map((e) => ({ id: e.shape.id, lo: Math.min(...e.rings.flat().map((p) => p[k])), hi: Math.max(...e.rings.flat().map((p) => p[k])) }));
  if (rows.length === 0) return show(undefined);
  const lo = rows.reduce((a, b) => (b.lo < a.lo ? b : a));
  const hi = rows.reduce((a, b) => (b.hi > a.hi ? b : a));
  return `${show([lo.lo, hi.hi])} (${lo.id}, ${hi.id})`;
};
for (const bone of bones.length === 0 ? [undefined] : bones) {
  const body = solids.filter((w) => bone === undefined || w.bone === bone);
  console.log(`${bone ?? asset.id} 1:${scale} ${when}`);
  console.log(`  x ahead: body ${show(axis(body, 0))} elevation ${drawn("elevation", bone, "x")} plan ${drawn("plan", bone, "x")}`);
  console.log(`  y up:    body ${show(axis(body, 1))} elevation ${drawn("elevation", bone, "y")} front ${drawn("front", bone, "y")}`);
  console.log(`  z left:  body ${show(axis(body, 2))} front     ${drawn("front", bone, "x")} plan ${drawn("plan", bone, "y")}`);
}
