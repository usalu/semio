/** 🔎️ The shapes of one bone in one evaluated subject: `bun bone.ts <id|file.json> <scale> <bone> <subject id>` prints per view every shape of the bone with its id, role, fill, solid and extent, to see which shape sets an extent. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const [arg, key, bone, subjectId] = process.argv.slice(2);
const asset = readDoc(arg!).doc.asset;
const subject = engine.subjects(asset, Number(key)).find((s: any) => s.id === subjectId);
const state = evaluate(asset, subject.query, engine.scaleSelector(Number(key)));
for (const view of ["elevation", "front", "plan"] as const)
  for (const shape of state.shapes[view]) {
    if (shape.shape.bone !== bone) continue;
    const xs = shape.rings.flat().map((p: any) => p.x), ys = shape.rings.flat().map((p: any) => p.y);
    console.log(`${view} ${shape.shape.id} ${shape.shape.role} ${shape.shape.filled ? "filled" : "open"} ${shape.shape.kind}${shape.shape.solid ? ` ${shape.shape.solid.kind}` : ""} z ${shape.shape.z} x [${Math.min(...xs).toFixed(3)}, ${Math.max(...xs).toFixed(3)}] y [${Math.min(...ys).toFixed(3)}, ${Math.max(...ys).toFixed(3)}] points ${xs.length}`);
  }
