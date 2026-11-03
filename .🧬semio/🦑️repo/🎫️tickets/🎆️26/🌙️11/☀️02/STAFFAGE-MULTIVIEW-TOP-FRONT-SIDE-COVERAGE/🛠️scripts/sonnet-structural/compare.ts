import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts";
import { catalog } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { solveStructure as solveOld } from "./old/solver.ts";

const only = process.argv.slice(2);
const summary = (r: any) => {
  const w = r.segmentWeights.reduce((s: number, f: any) => s + Math.abs(f.fyN), 0) + 1;
  const kinds = ["did not converge", "slides", "Unbalanced", "lifted"].filter((k) => r.notes.some((n: any) => n.en.includes(k)));
  return { res: Math.hypot(r.residual.fxN, r.residual.fyN, r.residual.fzN ?? 0) / w, active: r.reactions.filter((x: any) => x.active).length, kinds };
};
for (const asset of catalog().filter((a) => only.length === 0 || only.includes(a.id)))
  for (const pose of asset.poses) {
    let state;
    try {
      state = evaluate(asset, { kind: "pose", pose: pose.id, timeS: 0 }, { kind: "scale", scale: { denominator: 50 } });
    } catch {
      continue;
    }
    const a = summary(solveStructure(state));
    const o = summary(solveOld(state));
    if (a.active !== o.active || a.kinds.join() !== o.kinds.join() || a.res > 1e-6 !== o.res > 1e-6) console.log("[DEBUG]", asset.id, pose.id, "new", JSON.stringify(a), "old", JSON.stringify(o));
  }
