import { readFileSync } from "node:fs";
import { parseArgs } from "node:util";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules";
const { evaluate } = await import(`${S}/🧮️evaluation/🟦️.ts`);
const { solveThermal } = await import(`${S}/🌡️thermal/🟦️.ts`);
const { solveStructure } = await import(`${S}/🏗️structure/🟦️.ts`);
const { values } = parseArgs({ options: { file: { type: "string" }, asset: { type: "string" }, q: { type: "string" } } });
const doc = JSON.parse(readFileSync(values.file!, "utf8"));
const asset = (doc.assets ?? [doc]).find((a: any) => a.id === values.asset);
for (const q of values.q!.split(",")) {
  const [kind, id, t] = q.split(":");
  const st = evaluate(asset, kind === "pose" ? { kind: "pose", pose: id } : { kind: "clip", clip: id, timeS: Number(t) });
  const s = solveStructure(st); const th = solveThermal(st);
  console.log(q, "mass", s.totalMassKg.toFixed(1), "com", s.centreOfMass.x.toFixed(3), s.centreOfMass.y.toFixed(3), "bounds", JSON.stringify(Object.fromEntries(Object.entries(st.bounds).map(([k, v]: any) => [k, +v.toFixed(3)]))));
  for (const r of s.reactions) console.log("  R", r.anchor, "at", r.at.x.toFixed(3), r.at.y.toFixed(3), "F", r.fxN.toFixed(0), r.fyN.toFixed(0), "p", r.pressurePa?.toFixed(0), r.active);
  console.log("  res", s.residual.fxN.toExponential(1), s.residual.fyN.toExponential(1), "temps", Object.entries(th.regions).map(([k, v]: any) => `${k}:${v.surfaceTempC.toFixed(0)}`).join(" "));
  console.log("  emit", th.streamlines.map((l: any) => `${l.emitter}:${l.active.toFixed(2)}`).join(" "), "load", st.loads.map((l: any) => `${l.id}:${l.fyN.toFixed(0)}`).join(" "));
}
