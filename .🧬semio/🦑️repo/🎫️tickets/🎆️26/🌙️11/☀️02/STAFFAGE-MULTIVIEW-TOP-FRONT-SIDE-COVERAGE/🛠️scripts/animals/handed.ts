import { catalog, authoredScales, queries } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [, , all, ...ids] = process.argv;
const near = /(^|-)(n|hn|fn|near)$|^near-|^n-/, far = /(^|-)(f|hf|ff|far)$|^far-|^f-/;
for (const a of catalog().filter((x) => ids.includes(x.id))) {
  const boneSide = (b: string) => (near.test(b) ? -1 : far.test(b) ? 1 : 0);
  const named = a.rig.bones.filter((b) => boneSide(b.id) !== 0).length;
  const bad = new Map<string, number>();
  for (const d of authoredScales(a)) {
    const qs = all === "all" ? queries(a) : [{ name: "default", query: { kind: "pose", pose: a.presentation.defaultPose } as any }];
    for (const { name, query } of qs) {
      const st: any = evaluate(a, query, { kind: "scale", scale: { denominator: d } } as any);
      for (const [v, ax] of [["plan", "y"], ["front", "x"]] as const)
        for (const s of st.shapes[v]) {
          const side = boneSide(s.shape.bone); if (!side || s.shape.role === "ground") continue;
          const vs = s.rings.flat().map((p: any) => p[ax]); const c = (Math.min(...vs) + Math.max(...vs)) / 2;
          if (Math.abs(c) > 0.01 && Math.sign(c) !== side) { const k = `1:${d} ${name} ${v} ${s.shape.id}/${s.shape.bone} c=${c.toFixed(3)} expects ${side < 0 ? "-" : "+"}`; bad.set(k, (bad.get(k) ?? 0) + 1); }
        }
    }
  }
  console.log(a.id, "sided bones", named, "wrong-side shapes", bad.size, [...bad].slice(0, 6).map(([k, n]) => `${k} x${n}`).join(" | "));
}
