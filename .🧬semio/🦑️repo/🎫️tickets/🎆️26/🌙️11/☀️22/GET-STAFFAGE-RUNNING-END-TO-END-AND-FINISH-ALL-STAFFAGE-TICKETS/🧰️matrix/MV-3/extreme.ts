/** 🧭️ Which shape sets each end of the whole-figure extents of one subject: `bun extreme.ts <id|file.json> <scale> <subject id>` prints per view and axis the shape (id, bone, kind, solid) at the low and the high end, the facts behind a `sync-forward`, `sync-height` or `sync-lateral` finding. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const [arg, key, subjectId] = process.argv.slice(2);
const asset = readDoc(arg!).doc.asset;
const subject = engine.subjects(asset, Number(key)).find((s: any) => s.id === subjectId);
const state = evaluate(asset, subject.query, engine.scaleSelector(Number(key)));
for (const view of ["elevation", "front", "plan"] as const)
  for (const axis of ["x", "y"] as const) {
    let low: any, high: any;
    for (const shape of state.shapes[view]) {
      if (shape.shape.role === "ground") continue;
      for (const p of shape.rings.flat()) {
        if (low === undefined || p[axis] < low.value) low = { value: p[axis], shape: shape.shape };
        if (high === undefined || p[axis] > high.value) high = { value: p[axis], shape: shape.shape };
      }
    }
    const name = (e: any): string => `${e.value.toFixed(3)} by "${e.shape.id}" on ${e.shape.bone} (${e.shape.kind}${e.shape.solid ? ` ${e.shape.solid.kind}` : ""}, ${e.shape.filled ? "filled" : "open"} ${e.shape.role})`;
    console.log(`${view} ${axis}: ${name(low)} … ${name(high)}`);
  }
