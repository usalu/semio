/** 📏 Reading aid: evaluates the candidate in a pose at a scale and prints the shapes whose rings reach furthest out per view (`bun author-extents.ts <scale> [pose]`). */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const doc = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
const scale = Number(process.argv[2] ?? 50);
const pose = process.argv[3] ?? "stored";
const state = evaluate(doc.asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: scale } });
for (const view of ["elevation", "front", "plan"] as const) {
  const rows = state.shapes[view].map((s) => {
    const p = s.rings.flat();
    return { id: s.shape.id, minX: Math.min(...p.map((q) => q.x)), maxX: Math.max(...p.map((q) => q.x)), minY: Math.min(...p.map((q) => q.y)), maxY: Math.max(...p.map((q) => q.y)) };
  });
  const all = { minX: Math.min(...rows.map((r) => r.minX)), maxX: Math.max(...rows.map((r) => r.maxX)), minY: Math.min(...rows.map((r) => r.minY)), maxY: Math.max(...rows.map((r) => r.maxY)) };
  console.log(`[DEBUG] ${view}: x ${all.minX.toFixed(4)}..${all.maxX.toFixed(4)} y ${all.minY.toFixed(4)}..${all.maxY.toFixed(4)}`);
  for (const key of ["minX", "maxX", "minY", "maxY"] as const) {
    const sorted = [...rows].sort((a, b) => (key.startsWith("min") ? a[key] - b[key] : b[key] - a[key])).slice(0, 2);
    console.log(`[DEBUG]   ${key}: ${sorted.map((r) => `${r.id} ${r[key].toFixed(4)}`).join(", ")}`);
  }
}
