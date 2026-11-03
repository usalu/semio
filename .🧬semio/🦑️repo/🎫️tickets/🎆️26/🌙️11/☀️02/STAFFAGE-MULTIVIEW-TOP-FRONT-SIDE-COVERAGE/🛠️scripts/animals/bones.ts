import { writeFileSync } from "node:fs";
import { catalog, authoredScales, queries, toleranceM } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [, , outFile, factor, ...ids] = process.argv;
const sel = (d: number) => ({ kind: "scale", scale: { denominator: d } }) as const;
type R = [number, number];
function ranges(shapes: any[], axis: "x" | "y", bones: string[]) {
  const m: Record<string, R> = {};
  for (const s of shapes) { if (s.shape.role === "ground" || s.shape.role === "texture") continue; const b = s.shape.bone; for (const r of s.rings) for (const p of r) { const v = p[axis]; const c = (m[b] ??= [Infinity, -Infinity]); if (v < c[0]) c[0] = v; if (v > c[1]) c[1] = v; } }
  return m;
}
const out: string[] = []; let skipped = 0;
for (const a of catalog().filter((x) => ids.includes(x.id))) {
  const base = toleranceM(a) * +factor;
  const bones = a.rig.bones.map((b) => b.id);
  for (const d of authoredScales(a)) {
    const tol = Math.max(base, 0.0008 * d);
    const rest = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, sel(d));
    const rEx = ranges(rest.shapes.elevation, "x", bones), rPx = ranges(rest.shapes.plan, "x", bones), rEy = ranges(rest.shapes.elevation, "y", bones), rFy = ranges(rest.shapes.front, "y", bones);
    const agg = new Map<string, { n: number; worst: number; at: string }>();
    for (const { name, query } of queries(a)) {
      let st: any; try { st = evaluate(a, query, sel(d)); } catch { skipped++; continue; }
      const Ex = ranges(st.shapes.elevation, "x", bones), Px = ranges(st.shapes.plan, "x", bones), Ey = ranges(st.shapes.elevation, "y", bones), Fy = ranges(st.shapes.front, "y", bones);
      for (const b of bones) {
        const tests: [string, R | undefined, R | undefined, R | undefined, R | undefined][] = [["plan-x", Ex[b], Px[b], rEx[b], rPx[b]], ["front-y", Ey[b], Fy[b], rEy[b], rFy[b]]];
        for (const [lab, e, p, re, rp] of tests) {
          if (!e || !p || !re || !rp) continue;
          const dlo = (e[0] - p[0]) - (re[0] - rp[0]), dhi = (e[1] - p[1]) - (re[1] - rp[1]);
          const w = Math.max(Math.abs(dlo), Math.abs(dhi));
          if (w > tol) { const k = `${b} ${lab}`; const c = agg.get(k) ?? { n: 0, worst: 0, at: "" }; c.n++; if (w > c.worst) { c.worst = w; c.at = name; } agg.set(k, c); }
        }
      }
    }
    out.push(`## ${a.id} 1:${d} tol ${tol.toFixed(3)} flagged=${agg.size}`);
    for (const [k, v] of [...agg].sort((x, y) => y[1].worst - x[1].worst)) out.push(`  ${k} n=${v.n} worst=${v.worst.toFixed(3)} at ${v.at}`);
  }
}
writeFileSync(outFile, out.join("\n")); console.log(out.filter((l) => l.startsWith("##")).join("\n"));
