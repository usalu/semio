/** 🔬️ What one subject of the matrix shows: `bun at.ts <id|file.json> <scale> <text of the subject id> [--bones a,b] [--all]` evaluates the first subject of the matrix (pose, clip instant or transition) whose id contains the text, prints the extents of the three views with the shape that sets each end, the hard geometry findings of that state and, with `--bones` (or `--all`), the box of every shape of those bones per view.
 * @see ../MR/gate.ts — the gate over every subject */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const args = process.argv.slice(2);
const asset = readDoc(args[0]!).doc.asset;
const scale = Number(args[1]);
const subject = engine.subjects(asset, scale).find((candidate: any) => candidate.id.includes(args[2]!));
if (subject === undefined) throw new Error(`no subject contains "${args[2]}"`);
const state: any = evaluate(asset, subject.query, engine.scaleSelector(scale));
const f = (n: number): string => n.toFixed(4);
const bonesAt = args.indexOf("--bones");
const bones = bonesAt < 0 ? undefined : new Set(args[bonesAt + 1]!.split(","));
console.log(`subject ${subject.id} ${JSON.stringify(subject.query)} tolerance ${f(engine.toleranceM(asset))}`);
for (const view of ["elevation", "front", "plan"]) {
  const boxes = state.shapes[view].filter((s: any) => s.shape.role !== "ground" && s.rings.flat().length > 0).map((s: any) => {
    const points = s.rings.flat();
    const xs = points.map((p: any) => p.x);
    const ys = points.map((p: any) => p.y);
    return { id: s.shape.id, bone: s.shape.bone, solid: s.shape.solid === undefined ? "flat" : `${s.shape.solid.kind}:${s.shape.solid.sweep ?? s.shape.solid.axis ?? ""}`, x0: Math.min(...xs), x1: Math.max(...xs), y0: Math.min(...ys), y1: Math.max(...ys) };
  });
  const end = (key: "x0" | "x1" | "y0" | "y1", low: boolean): string => {
    const best = boxes.reduce((a: any, b: any) => ((low ? b[key] < a[key] : b[key] > a[key]) ? b : a));
    return `${f(best[key])} (${best.id}/${best.bone} ${best.solid})`;
  };
  console.log(`-- ${view}: x ${end("x0", true)} … ${end("x1", false)}   y ${end("y0", true)} … ${end("y1", false)}`);
  if (bones !== undefined || args.includes("--all")) for (const b of boxes) if (bones === undefined || bones.has(b.bone)) console.log(`   ${b.id} [${b.bone}] ${b.solid} x ${f(b.x0)} ${f(b.x1)} y ${f(b.y0)} ${f(b.y1)}`);
}
for (const finding of engine.geometryFindings(asset, state, subject.isDefault)) if (finding.tier === "hard") console.log(`HARD ${finding.check} ${finding.message}`);
if (args.includes("--anchors")) for (const [id, a] of Object.entries(state.anchors) as [string, any][]) console.log(`anchor ${id} [${a.anchor.bone}] ${a.anchor.kind} x ${f(a.at.x)} y ${f(a.at.y)} z ${f(a.z)}`);
