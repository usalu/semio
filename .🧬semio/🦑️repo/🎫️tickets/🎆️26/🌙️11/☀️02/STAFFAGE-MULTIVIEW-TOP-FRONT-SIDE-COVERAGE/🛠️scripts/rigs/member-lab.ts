import { readFileSync } from "node:fs";
import { join } from "node:path";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
const root = join(import.meta.dirname, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/labrador-dog/🔣️.json");
const a = JSON.parse(readFileSync(root, "utf8")).asset;
const member = { id: "head", kind: "member", bone: "head-core", role: "body", z: 4, filled: true, tone: 0.72, region: "coat-head", from: { x: 0, y: 0.02, z: 0 }, to: { x: 0.25, y: 0.03, z: 0 }, section: { kind: "rect", w: 0.1, h: 0.08 } };
for (const v of ["elevation", "front", "plan"]) a.representations["50"].geometry[v] = a.representations["50"].geometry[v].map((s: any) => (s.id === "head" ? member : s));
const query = { kind: "pose", pose: a.presentation.defaultPose } as const;
const state = evaluate(a, query, { kind: "scale", scale: { denominator: 50 } });
for (const v of ["elevation", "front", "plan"] as const) {
  const s = state.shapes[v].find((x: any) => x.shape.id === "head")!;
  console.log(v, s.rings.map((r: any) => r.length));
  const o = defaultDepictionOptions("dark-line", "en", 50, v);
  const d = depict(state, o);
  console.log(v, JSON.stringify(d).includes("shape:head") || JSON.stringify(d).includes("outline:head"));
}
