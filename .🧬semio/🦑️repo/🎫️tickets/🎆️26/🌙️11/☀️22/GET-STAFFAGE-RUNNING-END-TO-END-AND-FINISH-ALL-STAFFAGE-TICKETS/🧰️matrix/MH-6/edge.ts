/** 🧱️ Which shapes carry the extent of each view in one subject of the matrix: `bun edge.ts <id|file.json> <scale> <subject id or part of it>` evaluates the subject (a pose, clip instant or transition as `gate.ts` names it) and prints per view and side (low and high end of the two paper axes) the three shapes that reach farthest, so that a `sync-*` finding of the whole figure can be laid to the part that causes it. Run it from `T`. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const [arg, scale, wanted] = process.argv.slice(2);
const asset = readDoc(arg!).doc.asset;
const all = engine.subjects(asset, Number(scale)) as { id: string; query: unknown }[];
const subject = all.find((candidate) => candidate.id === wanted) ?? all.find((candidate) => candidate.id.includes(wanted!));
if (subject === undefined) throw new Error(`no subject "${wanted}"`);
const state = evaluate(asset, subject.query, engine.scaleSelector(Number(scale)));
console.log(`${asset.id} 1:${scale} ${subject.id}`);
for (const view of ["elevation", "front", "plan"] as const) {
  const shapes = (state.shapes[view] as { shape: any; rings: { x: number; y: number }[][] }[]).filter((entry) => entry.shape.role !== "ground" && entry.rings.flat().length > 0);
  for (const axis of ["x", "y"] as const)
    for (const side of [-1, 1]) {
      const ranked = shapes.map((entry) => ({ id: `${entry.shape.bone}/${entry.shape.id}`, at: side * Math.max(...entry.rings.flat().map((p) => side * p[axis])) })).sort((a, b) => side * (b.at - a.at));
      console.log(`  ${view.slice(0, 2)} ${axis} ${side < 0 ? "low " : "high"}: ${ranked.slice(0, 3).map((e) => `${e.id} ${e.at.toFixed(3)}`).join(", ")}`);
    }
}
