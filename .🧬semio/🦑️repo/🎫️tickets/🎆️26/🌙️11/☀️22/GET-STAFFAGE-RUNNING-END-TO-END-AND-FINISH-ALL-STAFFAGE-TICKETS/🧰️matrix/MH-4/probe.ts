/** 🔦️ One subject of the matrix under the lamp: `bun probe.ts <id|file.json> --scale 50 --subject "<subject id>" [--bones a,b]` evaluates the subject (a pose, clip instant or transition, by the id the gate prints) and prints the extent of each view with the shape that reaches each of its four ends, the findings of the matrix for that state, and for every bone named (default: the bones of the hard findings) each shape with its construction and its ranges in the three views. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const asset = readDoc(args[0]!).doc.asset;
const scale = Number(flag("scale"));
const subject = engine.subjects(asset, scale).find((candidate: any) => candidate.id === flag("subject"));
if (subject === undefined) throw new Error(`no subject "${flag("subject")}"`);
const state = evaluate(asset, subject.query, engine.scaleSelector(scale));
const VIEWS = ["elevation", "front", "plan"] as const;
const kind = (shape: any): string => (shape.solid === undefined ? (shape.followTip === true ? "tip" : "flat") : shape.solid.kind === "extruded" ? `sw${shape.solid.sweep}` : "rev");
const range = (rings: any[], axis: "x" | "y"): [number, number] => {
  const values = rings.flat().map((p: any) => p[axis] as number);
  return [Math.min(...values), Math.max(...values)];
};
for (const view of VIEWS) {
  const shapes = state.shapes[view].filter((shape: any) => shape.shape.role !== "ground" && shape.rings.flat().length > 0);
  const ends = (["x", "y"] as const).map((axis) => {
    const sorted = shapes.map((shape: any) => ({ name: `${shape.shape.bone}/${shape.shape.id}(${kind(shape.shape)})`, r: range(shape.rings, axis) }));
    const low = sorted.reduce((a: any, b: any) => (b.r[0] < a.r[0] ? b : a));
    const high = sorted.reduce((a: any, b: any) => (b.r[1] > a.r[1] ? b : a));
    return `${axis} [${low.r[0].toFixed(3)} ${low.name}, ${high.r[1].toFixed(3)} ${high.name}]`;
  });
  console.log(`${view}: ${ends.join("  ")}`);
}
const findings = engine.geometryFindings(asset, state, subject.isDefault);
for (const finding of findings.filter((candidate: any) => candidate.tier === "hard")) console.log(`HARD ${finding.check}: ${finding.message}`);
const bones = flag("bones")?.split(",") ?? [...new Set(findings.filter((finding: any) => finding.tier === "hard" && finding.bone !== undefined).map((finding: any) => finding.bone as string))];
for (const bone of bones) {
  console.log(`bone ${bone}`);
  const ids = new Set<string>(VIEWS.flatMap((view) => state.shapes[view].filter((shape: any) => shape.shape.bone === bone).map((shape: any) => shape.shape.id as string)));
  for (const id of ids) {
    const cells = VIEWS.map((view) => {
      const shape = state.shapes[view].find((candidate: any) => candidate.shape.bone === bone && candidate.shape.id === id);
      if (shape === undefined || shape.rings.flat().length === 0) return `${view[0]} -`;
      const [x, y] = [range(shape.rings, "x"), range(shape.rings, "y")];
      return `${view[0]}(${kind(shape.shape)}) x[${x.map((v) => v.toFixed(3))}] y[${y.map((v) => v.toFixed(3))}]`;
    });
    console.log(`   ${id}: ${cells.join("  ")}`);
  }
  for (const [id, anchor] of Object.entries(state.anchors) as [string, any][]) if (anchor.anchor.bone === bone) console.log(`   anchor ${id}: x ${anchor.at.x.toFixed(3)} y ${anchor.at.y.toFixed(3)} z ${anchor.z.toFixed(3)}`);
}
