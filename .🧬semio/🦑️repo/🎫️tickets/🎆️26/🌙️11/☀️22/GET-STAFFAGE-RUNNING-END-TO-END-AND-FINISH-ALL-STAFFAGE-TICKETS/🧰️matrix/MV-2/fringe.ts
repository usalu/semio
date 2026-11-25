/** 🍂️ Makes the plan of a foliage bone reach as far forward as its side elevation at rest: `bun fringe.ts <id> <scale> <bone> <plan path id> [--in file]` measures the largest forward position of every elevation shape of the bone in the rest frame and of the plan path (a texture of closed leaf outlines, one subpath per leaf), and when the plan stops short it moves the foremost leaf forward by the difference, so the plan shows the leaf fringe the elevation draws beyond the foliage mass (cause G of `multiview-diagnosis.md`: the views draw different extents for one bone at rest). The result is a `set` op with the absolute path in `ops/<id>.json` (it replaces an earlier op for the same shape; a run on a file that already reaches leaves the ops file as it is), which `apply.ts` writes.
 * @see ./apply.ts */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const { flattenGeometry } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const args = process.argv.slice(2);
const [id, scale, bone, shapeId] = args as [string, string, string, string];
const asset = readDoc(args.includes("--in") ? args[args.indexOf("--in") + 1]! : id).doc.asset;
const geometry = asset.representations[scale].geometry;
const { o, u, v } = restFrames(asset.rig)[bone].frame;
const reach = Math.max(...geometry.elevation.filter((shape: any) => shape.bone === bone && shape.role !== "ground").flatMap((shape: any) => flattenGeometry(shape).rings.flat().map((p: any) => o[0] + p.x * u[0] + p.y * v[0])));
const shape = geometry.plan.find((candidate: any) => candidate.id === shapeId && candidate.bone === bone);
const leaves = (shape.d as string).split(/(?=M)/).map((part) => part.trim()).filter((part) => part.length > 0);
const xs = (leaf: string): number[] => [...leaf.matchAll(/(-?\d*\.?\d+(?:e-?\d+)?)\s+(-?\d*\.?\d+(?:e-?\d+)?)/g)].map((m) => Number(m[1]));
const tips = leaves.map((leaf) => Math.max(...xs(leaf)));
const foremost = tips.indexOf(Math.max(...tips));
const shift = reach - tips[foremost]!;
console.log(`${id} 1:${scale} ${bone}: elevation reaches ${reach.toFixed(4)} m, plan "${shapeId}" ${tips[foremost]!.toFixed(4)} m (leaf ${foremost + 1} of ${leaves.length}), difference ${(shift * 1000).toFixed(1)} mm`);
if (shift > 0.0005) {
  leaves[foremost] = leaves[foremost]!.replace(/(-?\d*\.?\d+(?:e-?\d+)?)(\s+)(-?\d*\.?\d+(?:e-?\d+)?)/g, (_, x, gap, y) => `${Number((Number(x) + shift).toFixed(4))}${gap}${y}`);
  const file = join(import.meta.dir, "ops", `${id}.json`);
  const ops = (JSON.parse(readFileSync(file, "utf8")) as any[]).filter((op) => !(op.op === "set" && op.scale === scale && op.view === "plan" && op.id === shapeId && op.bone === bone));
  ops.push({ op: "set", scale, view: "plan", id: shapeId, bone, set: { d: leaves.join(" ") } });
  writeFileSync(file, "[\n" + ops.map((op) => "  " + JSON.stringify(op)).join(",\n") + "\n]\n");
  console.log(`${id}: op written, ${ops.length} ops`);
} else console.log(`${id}: nothing to do`);
