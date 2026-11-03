import { load } from "../humans/lib.ts";
import { bandOf } from "../humans/bands.ts";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const doc = load("skateboarder"); const a = doc.asset;
const mf = Number(process.argv[2]);
const rep = bandOf(a, { slug: "skateboarder", den: 500, drop: /^(eye|hood|hood-seam|deck-grip|ear-[lr]|hair|knee-[lr]|elbow-[lr]|shoulder-[lr]|truck-front|cuff-[lr]|sole.*|toe.*)$/, minFeatureM: mf, epsM: 0.012, pens: { body: 0.12, detail: null, texture: null, glazing: null, ground: 0.18 }, range: [350, 900], en: { silhouette: "x" }, de: { silhouette: "x" } } as any);
a.representations["500"] = rep;
const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: 500 } });
for (const v of ["plan", "front"]) {
  const rows = st.shapes[v].filter((s: any) => s.shape.role !== "ground").map((s: any) => { const p = s.rings.flat(); const xs = p.map((q: any) => q.x), ys = p.map((q: any) => q.y); return { id: s.shape.id, x: [Math.min(...xs), Math.max(...xs)], y: [Math.min(...ys), Math.max(...ys)] }; });
  const lo = (k: "x" | "y") => rows.reduce((m, r) => Math.min(m, r[k][0]), 9), hi = (k: "x" | "y") => rows.reduce((m, r) => Math.max(m, r[k][1]), -9);
  console.log(v, "x", lo("x").toFixed(3), hi("x").toFixed(3), "y", lo("y").toFixed(3), hi("y").toFixed(3));
  for (const k of ["x", "y"] as const) { console.log("  ", k, "lo", rows.filter((r) => Math.abs(r[k][0] - lo(k)) < 0.004).map((r) => r.id).join(","), "hi", rows.filter((r) => Math.abs(r[k][1] - hi(k)) < 0.004).map((r) => r.id).join(",")); }
}
