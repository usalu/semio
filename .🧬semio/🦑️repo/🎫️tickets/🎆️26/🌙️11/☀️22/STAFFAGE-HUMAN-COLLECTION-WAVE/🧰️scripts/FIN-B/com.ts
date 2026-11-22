/** 🎯️ Centre of mass and root of a clip over a time window, to find kinks of the motion: bun com.ts <slug|file.json> <clip> <from> <to> <step> */
import { join, resolve } from "node:path";
import { evaluate, solveStructure } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const [arg, clip, from, to, step] = process.argv.slice(2);
const a = (await Bun.file(arg!.endsWith(".json") ? arg! : join(catalog, arg!, "🔣️.json")).json()).asset;
for (let t = Number(from); t <= Number(to) + 1e-9; t += Number(step)) {
  const state: any = evaluate(a, { kind: "clip", clip: clip!, timeS: t });
  const s: any = solveStructure(state);
  const c = s.centreOfMass;
  console.log(t.toFixed(4), "com", [c.x, c.y, c.z].map((v: number) => v.toFixed(5)).join(" "), "root", JSON.stringify(state.root), "contacts", (state.contacts ?? []).map((x: any) => x.anchor ?? x).join(","));
}
