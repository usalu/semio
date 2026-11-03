import { L, load } from "./lib.ts";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { queries } = await import(`${L}/🧪️tests/🧊️three-views/🧰️.ts`);
const [slug, den, filter] = process.argv.slice(2);
const a = load(slug!).asset;
const edges = (shapes: any[], key: "x" | "y") => {
  const rows = shapes.filter((s) => s.shape.role !== "ground").map((s) => { const v = s.rings.flat().map((q: any) => q[key]); return { id: s.shape.id, lo: Math.min(...v), hi: Math.max(...v) }; });
  const lo = Math.min(...rows.map((r) => r.lo)), hi = Math.max(...rows.map((r) => r.hi));
  return { lo, hi, loIds: rows.filter((r) => r.lo - lo < 0.004).map((r) => r.id).slice(0, 3).join(","), hiIds: rows.filter((r) => hi - r.hi < 0.004).map((r) => r.id).slice(0, 3).join(",") };
};
for (const { name, query } of queries(a)) {
  if (filter && !name.includes(filter)) continue;
  const st = evaluate(a, query, { kind: "scale", scale: { denominator: +den! } });
  const ex = edges(st.shapes.elevation, "x"), px = edges(st.shapes.plan, "x");
  const ey = edges(st.shapes.elevation, "y"), fy = edges(st.shapes.front, "y");
  const fx = edges(st.shapes.front, "x"), py = edges(st.shapes.plan, "y");
  const f = (n: number) => n.toFixed(3);
  const bad: string[] = [];
  const tol = 0.05;
  if (Math.abs(ex.lo - px.lo) > tol) bad.push(`x.lo e${f(ex.lo)}[${ex.loIds}] p${f(px.lo)}[${px.loIds}]`);
  if (Math.abs(ex.hi - px.hi) > tol) bad.push(`x.hi e${f(ex.hi)}[${ex.hiIds}] p${f(px.hi)}[${px.hiIds}]`);
  if (Math.abs(ey.hi - fy.hi) > tol) bad.push(`y.hi e${f(ey.hi)}[${ey.hiIds}] f${f(fy.hi)}[${fy.hiIds}]`);
  if (Math.abs(ey.lo - fy.lo) > tol) bad.push(`y.lo e${f(ey.lo)} f${f(fy.lo)}`);
  if (Math.abs(fx.lo - py.lo) > tol) bad.push(`z.lo f${f(fx.lo)}[${fx.loIds}] p${f(py.lo)}[${py.loIds}]`);
  if (Math.abs(fx.hi - py.hi) > tol) bad.push(`z.hi f${f(fx.hi)}[${fx.hiIds}] p${f(py.hi)}[${py.hiIds}]`);
  if (bad.length) console.log(name, "|", bad.join(" | "));
}
