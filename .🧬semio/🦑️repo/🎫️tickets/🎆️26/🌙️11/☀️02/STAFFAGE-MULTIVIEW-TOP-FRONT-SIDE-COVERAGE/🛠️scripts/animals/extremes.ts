import { catalog, authoredScales, queries, toleranceM } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [, , ...ids] = process.argv;
const sel = (d: number) => ({ kind: "scale", scale: { denominator: d } }) as const;
type Ex = { v: number; id: string };
function ext(shapes: any[], axis: "x" | "y") {
  let lo: Ex = { v: Infinity, id: "" }, hi: Ex = { v: -Infinity, id: "" };
  for (const s of shapes) { if (s.shape.role === "ground") continue; for (const r of s.rings) for (const p of r) { if (p[axis] < lo.v) lo = { v: p[axis], id: s.shape.id }; if (p[axis] > hi.v) hi = { v: p[axis], id: s.shape.id }; } }
  return { lo, hi };
}
for (const a of catalog().filter((x) => ids.includes(x.id))) {
  const tol = toleranceM(a) * 1.5;
  const agg = new Map<string, { n: number; worst: number; note: string }>();
  for (const d of authoredScales(a)) for (const { name, query } of queries(a)) {
    const st = evaluate(a, query, sel(d));
    const el = ext(st.shapes.elevation, "x"), ey = ext(st.shapes.elevation, "y"), fx = ext(st.shapes.front, "x"), fy = ext(st.shapes.front, "y"), px = ext(st.shapes.plan, "x"), py = ext(st.shapes.plan, "y");
    const pairs: [string, Ex, Ex][] = [["X-min el/pl", el.lo, px.lo], ["X-max el/pl", el.hi, px.hi], ["Y-min el/fr", ey.lo, fy.lo], ["Y-max el/fr", ey.hi, fy.hi], ["Z-min fr/pl", fx.lo, py.lo], ["Z-max fr/pl", fx.hi, py.hi]];
    for (const [lab, A, B] of pairs) {
      const dd = Math.abs(A.v - B.v);
      if (dd > tol) {
        const key = `1:${d} ${lab} ${A.id}(${A.v.toFixed(2)}) vs ${B.id}(${B.v.toFixed(2)})`;
        const cur = agg.get(key.replace(/\([-0-9.]+\)/g, "")) ?? { n: 0, worst: 0, note: "" };
        cur.n++; if (dd > cur.worst) { cur.worst = dd; cur.note = `${name} ${key}`; }
        agg.set(key.replace(/\([-0-9.]+\)/g, ""), cur);
      }
    }
  }
  console.log(`## ${a.id} tol ${tol.toFixed(3)} dims ${a.dimensions.widthM}x${a.dimensions.depthM}x${a.dimensions.heightM}`);
  for (const [k, v] of [...agg].sort((x, y) => y[1].worst - x[1].worst)) console.log(`  n=${v.n} worst=${v.worst.toFixed(3)} ${v.note}`);
}
