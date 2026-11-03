/** 🔬️ Which shape sets each extreme edge per view. Usage: bun diagnose.ts slug scale query-json */
import { readFileSync, readdirSync, existsSync } from "node:fs";
import { join } from "node:path";
import type { Asset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const ROOT = join(import.meta.dirname, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const [slug, scale, q] = process.argv.slice(2);
let asset!: Asset;
for (const g of readdirSync(ROOT, { withFileTypes: true }).filter((e) => e.isDirectory())) {
  const f = join(ROOT, g.name, slug!, "🔣️.json");
  if (existsSync(f)) asset = (JSON.parse(readFileSync(f, "utf8")) as { asset: Asset }).asset;
}
const state = evaluate(asset, JSON.parse(q!), { kind: "scale", scale: { denominator: Number(scale) } });
for (const view of ["elevation", "front", "plan"] as const) {
  const edges = { minX: [Infinity, ""], maxX: [-Infinity, ""], minY: [Infinity, ""], maxY: [-Infinity, ""] } as Record<string, [number, string]>;
  for (const s of state.shapes[view]) {
    if (s.shape.role === "ground") continue;
    for (const p of s.rings.flat()) {
      const label = `${s.shape.id}@${s.shape.bone}${s.shape.solid ? ":" + s.shape.solid.kind : ""}${(s.shape as { followTip?: boolean }).followTip ? ":tip" : ""}`;
      if (p.x < edges.minX![0]) edges.minX = [p.x, label];
      if (p.x > edges.maxX![0]) edges.maxX = [p.x, label];
      if (p.y < edges.minY![0]) edges.minY = [p.y, label];
      if (p.y > edges.maxY![0]) edges.maxY = [p.y, label];
    }
  }
  console.log(view, Object.entries(edges).map(([k, [v, l]]) => `${k}=${v.toFixed(3)} (${l})`).join("  "));
}
