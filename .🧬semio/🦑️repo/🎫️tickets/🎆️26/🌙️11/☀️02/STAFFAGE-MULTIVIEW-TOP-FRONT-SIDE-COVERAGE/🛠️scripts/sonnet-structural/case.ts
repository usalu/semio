import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts";
import { catalog } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { solveStructure as solveOld } from "./old/solver.ts";
const [id, pose] = process.argv.slice(2);
const asset = catalog().find((a) => a.id === id)!;
const state = evaluate(asset, { kind: "pose", pose: pose!, timeS: 0 }, { kind: "scale", scale: { denominator: 50 } });
for (const [name, r] of [["new", solveStructure(state)], ["old", solveOld(state)]] as const) {
  console.log("[DEBUG]", name, "com", JSON.stringify((r as any).centreOfMass), "residual", JSON.stringify(r.residual));
  for (const x of r.reactions as any[]) console.log("[DEBUG]  ", x.anchor, x.active, "at", x.at.x.toFixed(3), x.at.y.toFixed(3), (x.z ?? 0).toFixed(3), "f", x.fxN.toFixed(1), x.fyN.toFixed(1), (x.fzN ?? 0).toFixed(1), "m", (x.mzNm ?? 0).toFixed(1));
  console.log("[DEBUG]  notes", r.notes.map((n) => n.en).filter((n) => /slides|lifted|Unbalanced|converge|clamped|Collinear/.test(n)).map((n) => n.slice(0, 110)));
}
