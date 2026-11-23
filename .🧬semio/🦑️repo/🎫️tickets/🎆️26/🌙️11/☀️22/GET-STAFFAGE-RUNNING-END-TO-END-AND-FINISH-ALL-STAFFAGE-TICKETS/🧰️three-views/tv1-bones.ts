/** 🦴️ Ranks the bones of one instant by how far a view leaves the 3D body: `bun tv1-bones.ts <id>|--file <json> <scale> pose:<id>|clip:<id>@<seconds> [min mm]` prints, for every bone that carries body solids and drawn shapes, the largest edge difference (mm) of the side elevation, the front elevation and the plan against the extents of the bone's solids, worst first, bones under the minimum (default 15 mm) left out. */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const staffage = join(resolve(import.meta.dir, "../../../../../../../.."), "🎓️teaching/🏛️architecture/🧍️staffage");
const { catalog } = await import(join(staffage, "🧪️tests/🧊️three-views/🧰️.ts"));
const { evaluate } = await import(join(staffage, "🔨️modules/🧮️evaluation/🟦️.ts"));
const { worldSolids } = await import(join(staffage, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const args = process.argv.slice(2);
const asset = args[0] === "--file" ? JSON.parse(readFileSync(resolve(args[1]!), "utf8")).asset : catalog().find((candidate: { id: string }) => candidate.id === args[0]);
const [scale, when, least = "15"] = args.slice(args[0] === "--file" ? 2 : 1);
const query = when!.startsWith("pose:") ? { kind: "pose", pose: when!.slice(5) } : { kind: "clip", clip: when!.slice(5).split("@")[0], timeS: Number(when!.split("@")[1]) };
const state = evaluate(asset, query, { kind: "scale", scale: { denominator: Number(scale) } });
const solids = (worldSolids(asset, state.bones, Number(scale)) as { bone?: string; positions: Float64Array; texture?: boolean }[]).filter((w) => w.texture !== true);
type Range = [number, number] | undefined;
const range = (values: number[]): Range => (values.length === 0 ? undefined : [Math.min(...values), Math.max(...values)]);
const off = (a: Range, b: Range): number => (a === undefined || b === undefined ? 0 : Math.max(Math.abs(a[0] - b[0]), Math.abs(a[1] - b[1])) * 1000);
const rows: { bone: string; e: number; f: number; p: number; ids: string }[] = [];
for (const bone of asset.rig.bones as { id: string }[]) {
  const body = solids.filter((w) => w.bone === bone.id);
  if (body.length === 0) continue;
  const axis = (k: number): Range => range(body.flatMap((w) => Array.from(w.positions).filter((_, i) => i % 3 === k)));
  const shapes = (view: string) => (state.shapes[view] as { shape: { id: string; bone: string; role: string; solid?: unknown }; rings: { x: number; y: number }[][] }[]).filter((s) => s.shape.role !== "ground" && s.shape.bone === bone.id && s.rings.flat().length > 0);
  const drawn = (view: string, k: "x" | "y"): Range => range(shapes(view).flatMap((s) => s.rings.flat().map((p) => p[k])));
  const e = Math.max(off(axis(0), drawn("elevation", "x")), off(axis(1), drawn("elevation", "y")));
  const f = Math.max(off(axis(2), drawn("front", "x")), off(axis(1), drawn("front", "y")));
  const p = Math.max(off(axis(0), drawn("plan", "x")), off(axis(2), drawn("plan", "y")));
  if (Math.max(e, f, p) >= Number(least)) rows.push({ bone: bone.id, e, f, p, ids: ["front", "plan"].map((view) => `${view}: ${shapes(view).map((s) => `${s.shape.id}${s.shape.solid === undefined ? "" : "*"}`).join(" ")}`).join(" | ") });
}
console.log(`${asset.id} 1:${scale} ${when}: bones off the body by ${least} mm or more (elevation / front / plan, mm; * marks a solid)`);
for (const r of rows.sort((a, b) => Math.max(b.e, b.f, b.p) - Math.max(a.e, a.f, a.p))) console.log(`  ${r.bone}: ${r.e.toFixed(0)} / ${r.f.toFixed(0)} / ${r.p.toFixed(0)}   ${r.ids}`);
