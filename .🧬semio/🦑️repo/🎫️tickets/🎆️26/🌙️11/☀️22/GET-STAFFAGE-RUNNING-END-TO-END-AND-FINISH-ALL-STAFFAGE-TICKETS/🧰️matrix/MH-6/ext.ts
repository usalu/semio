/** 🔦️ The extents of every drawn shape of chosen bones in one subject of the matrix: `bun ext.ts <id|file.json> <scale> <subject id or part of it> <bone>[,<bone>…] [--anchors]` evaluates the subject (a pose, clip instant or transition as `gate.ts` names it) and prints per bone and view each shape with its construction and its ranges on the two paper axes (elevation: ahead, up; front: lateral, up; plan: ahead, lateral), then the extent of each whole view; `--anchors` adds the three coordinates of every anchor of those bones. Run it from `T`. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const [arg, scale, wanted, boneList] = process.argv.slice(2);
const asset = readDoc(arg!).doc.asset;
const bones = boneList === undefined || boneList === "*" ? undefined : new Set(boneList.split(","));
const all = engine.subjects(asset, Number(scale)) as { id: string; query: unknown }[];
const subject = all.find((candidate) => candidate.id === wanted) ?? all.find((candidate) => candidate.id.includes(wanted!));
if (subject === undefined) throw new Error(`no subject "${wanted}"; known: ${all.slice(0, 40).map((candidate) => candidate.id).join(" | ")}`);
const state = evaluate(asset, subject.query, engine.scaleSelector(Number(scale)));
const range = (rings: any[], axis: "x" | "y"): string => {
  const values = rings.flat().map((p: any) => p[axis] as number);
  return values.length === 0 ? "[]" : `[${Math.min(...values).toFixed(3)}, ${Math.max(...values).toFixed(3)}]`;
};
const kind = (shape: any): string => (shape.solid === undefined ? (shape.followTip ? "tip" : "flat") : shape.solid.kind === "extruded" ? `ext-${shape.solid.sweep}` : "rev");
console.log(`${asset.id} 1:${scale} ${subject.id}`);
for (const view of ["elevation", "front", "plan"] as const) {
  const shapes = state.shapes[view].filter((entry: any) => entry.shape.role !== "ground");
  console.log(`${view} whole ${range(shapes.map((entry: any) => entry.rings).flat(), "x")} ${range(shapes.map((entry: any) => entry.rings).flat(), "y")}`);
  for (const entry of shapes) if (bones === undefined ? false : bones.has(entry.shape.bone)) console.log(`  ${view.slice(0, 2)} ${entry.shape.bone}/${entry.shape.id} ${kind(entry.shape)} ${entry.shape.role ?? ""} x ${range(entry.rings, "x")} y ${range(entry.rings, "y")}`);
}
if (process.argv.includes("--anchors")) for (const [id, { anchor, at, z }] of Object.entries(state.anchors) as [string, any][]) if (bones === undefined || bones.has(anchor.bone)) console.log(`  anchor ${id} ${anchor.kind} on ${anchor.bone}: ahead ${at.x.toFixed(3)} up ${at.y.toFixed(3)} lateral ${z.toFixed(3)}`);
