/** 🔬️ Names the shapes at the extreme edges of the three views of one evaluated state: `bun extremes.ts <category-folder>/<asset> <scale> pose:<id>|clip:<id>@<seconds> [count]` prints, per view and edge, the `count` shapes that reach furthest, with their bone and kind. */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const root = resolve(import.meta.dir, "../../../../../../../..");
const staffage = join(root, "🎓️teaching/🏛️architecture/🧍️staffage");
const { evaluate } = await import(join(staffage, "🔨️modules/🧮️evaluation/🟦️.ts"));
const [path, scale, when, count = "3"] = process.argv.slice(2);
const { asset } = JSON.parse(readFileSync(join(staffage, "🗂️catalog", path!, "🔣️.json"), "utf8"));
const query = when!.startsWith("pose:") ? { kind: "pose", pose: when!.slice(5) } : { kind: "clip", clip: when!.slice(5).split("@")[0], timeS: Number(when!.split("@")[1]) };
const state = evaluate(asset, query, { kind: "scale", scale: { denominator: Number(scale) } });
const spread = Object.entries(state.bones as Record<string, { offset: { abductionDeg: number; twistDeg: number } }>).filter(([, bone]) => Math.abs(bone.offset.abductionDeg) > 1e-9 || Math.abs(bone.offset.twistDeg) > 1e-9).map(([id, bone]) => `${id}(a${bone.offset.abductionDeg.toFixed(1)} t${bone.offset.twistDeg.toFixed(1)})`);
console.log(`${asset.id} 1:${scale} ${when} spread or twisted: ${spread.join(" ") || "none"}`);
for (const view of ["elevation", "front", "plan"] as const) {
  const shapes = (state.shapes[view] as { shape: { id: string; bone: string; kind: string; role: string; solid?: unknown }; rings: { x: number; y: number }[][] }[]).filter((e) => e.shape.role !== "ground" && e.rings.flat().length > 0);
  const rows = shapes.map((e) => {
    const points = e.rings.flat();
    return { id: e.shape.id, bone: e.shape.bone, kind: e.shape.solid === undefined ? e.shape.kind : "solid", minX: Math.min(...points.map((p) => p.x)), maxX: Math.max(...points.map((p) => p.x)), minY: Math.min(...points.map((p) => p.y)), maxY: Math.max(...points.map((p) => p.y)) };
  });
  for (const edge of ["minX", "maxX", "minY", "maxY"] as const) {
    const sorted = [...rows].sort((a, b) => (edge.startsWith("min") ? a[edge] - b[edge] : b[edge] - a[edge])).slice(0, Number(count));
    console.log(`${view} ${edge}: ${sorted.map((r) => `${r.id}[${r.bone},${r.kind}] ${r[edge].toFixed(3)}`).join(" | ")}`);
  }
}
