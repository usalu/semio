/** 🔦️ Names the solids the evaluator cannot draw: `bun which.ts <id|file.json>` evaluates every subject of the matrix at every authored scale and, for a subject that fails, evaluates it again with one front or plan solid at a time taken out; prints one JSON line `{ scale, view, id, bone, subject }` per shape whose removal lets the subject evaluate. Run it from `T`. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
const asset = readDoc(process.argv[2]!).doc.asset;
const seen = new Set<string>();
for (const scale of Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined)) {
  const fails = (query: unknown): boolean => {
    try {
      evaluate(asset, query, engine.scaleSelector(Number(scale)));
      return false;
    } catch {
      return true;
    }
  };
  for (const subject of engine.subjects(asset, Number(scale))) {
    if (!fails(subject.query)) continue;
    let named = false;
    for (const view of ["front", "plan", "elevation"]) {
      const list = asset.representations[scale].geometry[view] as any[];
      for (let i = 0; i < list.length; i++) {
        if (list[i].solid === undefined) continue;
        const [shape] = list.splice(i, 1);
        const cured = !fails(subject.query);
        list.splice(i, 0, shape);
        if (!cured) continue;
        named = true;
        const key = `${scale} ${view} ${shape.id} ${shape.bone}`;
        if (!seen.has(key)) console.log(JSON.stringify({ scale, view, id: shape.id, bone: shape.bone, subject: subject.id }));
        seen.add(key);
      }
    }
    if (!named && !seen.has(`${scale} ?`)) console.log(JSON.stringify({ scale, view: "?", id: "?", subject: subject.id })), seen.add(`${scale} ?`);
  }
}
