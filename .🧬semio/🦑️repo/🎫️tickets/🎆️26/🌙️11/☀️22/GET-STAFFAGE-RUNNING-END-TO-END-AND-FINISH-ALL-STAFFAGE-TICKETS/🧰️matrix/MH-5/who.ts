/** 🔦️ Which shapes hold the whole-figure extents: `bun who.ts <id|file.json> <scale> <subject text>…` (from `T`) prints for every subject whose id contains a text, per compared axis and end, the three outermost shapes of both views. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const [id, scale, ...texts] = process.argv.slice(2);
const asset = readDoc(id!).doc.asset;
const AXES = [["height", "elevation", "y", "front", "y"], ["forward", "elevation", "x", "plan", "x"], ["lateral", "front", "x", "plan", "y"]] as const;
for (const subject of engine.subjects(asset, Number(scale))) {
  if (!texts.some((text) => subject.id.includes(text))) continue;
  const state: any = evaluate(asset, subject.query, engine.scaleSelector(Number(scale)));
  console.log(`== ${subject.id}`);
  for (const [name, a, ax, b, bx] of AXES)
    for (const end of ["min", "max"] as const) {
      const line = ([[a, ax], [b, bx]] as const).map(([view, axis]) => {
        const list = state.shapes[view].filter((e: any) => e.shape.role !== "ground").map((e: any) => ({ n: `${e.shape.bone}/${e.shape.id}`, v: Math[end](...e.rings.flat().map((p: any) => p[axis])) })).filter((e: any) => Number.isFinite(e.v));
        list.sort((p: any, q: any) => (end === "min" ? p.v - q.v : q.v - p.v));
        return `${view}: ${list.slice(0, 3).map((e: any) => `${e.n} ${e.v.toFixed(3)}`).join(", ")}`;
      });
      console.log(`  ${name} ${end}  ${line.join("  |  ")}`);
    }
}
