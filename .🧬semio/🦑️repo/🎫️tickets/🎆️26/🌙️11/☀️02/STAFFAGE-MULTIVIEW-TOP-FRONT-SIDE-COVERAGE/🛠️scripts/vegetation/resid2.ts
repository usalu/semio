import { load, save, stateAt, three, transformShape } from "./lib.ts";
const id = process.argv[2]!; const write = process.argv[3] === "write"; const x = load(id); const a = x.asset;
const rep = (den: number) => a.representations[String(den)].geometry;
const meas = (den: number) => { const st = stateAt(a, den); const e = three.extentOf(st.shapes.elevation), f = three.extentOf(st.shapes.front), p = three.extentOf(st.shapes.plan); return `el ${(e.maxX - e.minX).toFixed(3)}x${(e.maxY - e.minY).toFixed(3)} fr ${(f.maxX - f.minX).toFixed(3)}x${(f.maxY - f.minY).toFixed(3)} pl ${(p.maxX - p.minX).toFixed(3)}x${(p.maxY - p.minY).toFixed(3)}`; };
if (id === "hornbeam-hedge") for (const s of rep(500).front) if (s.id === "hedge") transformShape(s, { sx: 0.35 / 0.37, tx: 0, sy: 1, ty: 0 });
if (id === "multi-stem-birch") for (const s of rep(500).plan) if (s.id === "crown") { const st = stateAt(a, 500); const p = three.extentOf(st.shapes.plan); const c = (p.minY + p.maxY) / 2; const sc = 7 / (p.maxY - p.minY); transformShape(s, { sx: 1, tx: 0, sy: sc, ty: c * (1 - sc) }); }
if (id === "miscanthus-grass") {
  const S = Number(process.argv[4] ?? 1.3);
  for (const den of [50, 100, 200]) for (const s of rep(den).front) if (/^plume-\d$/.test(s.bone)) transformShape(s, { sx: S, tx: 0, sy: 1, ty: 0 });
}
for (const den of [50, 100, 200, 500]) console.log(id, den, meas(den));
if (write) save(x);
