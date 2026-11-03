import { load, save, stateAt, three, transformShape } from "./lib.ts";
const write = process.argv[2] === "write"; const x = load("miscanthus-grass"); const a = x.asset;
for (const den of [50, 100, 200]) { for (const s of a.representations[String(den)].geometry.plan) if (/^plume-\d$/.test(s.bone) && s.solid && Math.max(...s.points.map((p: number[]) => p[0])) < 0.35) transformShape(s, { sx: 1.3, tx: 0, sy: 1, ty: 0 });
  const st = stateAt(a, den); const f = three.extentOf(st.shapes.front), p = three.extentOf(st.shapes.plan), e = three.extentOf(st.shapes.elevation); console.log(den, "el", (e.maxX - e.minX).toFixed(3), (e.maxY - e.minY).toFixed(3), "fr", (f.maxX - f.minX).toFixed(3), (f.maxY - f.minY).toFixed(3), "pl", (p.maxX - p.minX).toFixed(3), (p.maxY - p.minY).toFixed(3)); }
if (write) save(x);
