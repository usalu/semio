import { catalog } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { VIEWS } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔗️contracts/🟦️.ts";
import { depict, defaultDepictionOptions } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const all = catalog();
console.log("assets", all.length);
const a = all.find((x) => x.id === (process.argv[2] ?? "standing-adult")) ?? all[0]!;
for (const d of [200, 50]) {
  const t0 = performance.now();
  const state = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: d } });
  const t1 = performance.now();
  for (const vp of ["elevation", "front", "plan"] as const)
    for (const v of VIEWS) {
      const s = performance.now();
      try { depict(state, { ...defaultDepictionOptions(v, "en", d, vp) }); } catch (e) { console.log(vp, v, "ERR", (e as Error).message.slice(0, 100)); }
      console.log(d, vp, v, (performance.now() - s).toFixed(0), "ms");
    }
  console.log("eval", (t1 - t0).toFixed(0));
}
