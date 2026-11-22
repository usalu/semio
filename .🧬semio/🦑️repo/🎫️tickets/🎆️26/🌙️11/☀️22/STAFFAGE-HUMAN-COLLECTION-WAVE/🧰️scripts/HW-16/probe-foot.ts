/** 🔬️ [DEBUG] world extents of the solids of some bones in the default pose against their drawings. */
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { readDoc } from "./lib.ts";
const [slug, sc, ...bones] = process.argv.slice(2);
const a = (await readDoc(slug!)).asset;
const st: any = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } } as never);
for (const w of worldSolids(a, st.bones, Number(sc), "body")) {
  if (!bones.includes(w.bone!)) continue;
  const ext = [0, 1, 2].map((k) => { let lo = Infinity, hi = -Infinity; for (let i = k; i < w.positions.length; i += 3) (lo = Math.min(lo, w.positions[i]!)), (hi = Math.max(hi, w.positions[i]!)); return `${lo.toFixed(3)}..${hi.toFixed(3)}`; });
  console.log("solid", w.id, w.bone, ext.join(" "));
}
for (const v of ["elevation", "front", "plan"]) for (const e of st.shapes[v]) if (bones.includes(e.shape.bone)) { const p = e.rings.flat(); console.log(v, e.shape.id, [Math.min(...p.map((q: any) => q.x)), Math.max(...p.map((q: any) => q.x)), Math.min(...p.map((q: any) => q.y)), Math.max(...p.map((q: any) => q.y))].map((x) => x.toFixed(3)).join(" ")); }
