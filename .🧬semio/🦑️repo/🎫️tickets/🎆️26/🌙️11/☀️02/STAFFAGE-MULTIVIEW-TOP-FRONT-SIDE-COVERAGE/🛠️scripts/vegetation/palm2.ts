import { load, save, stateAt, three, transformShape } from "./lib.ts";
const write = process.argv[2] === "write"; const S = 1.03;
const x = load("date-palm"); const a = x.asset;
for (const den of [50, 100, 200]) { const g = a.representations[String(den)].geometry;
  for (const s of g.front) if (/^fr-[lr]1[ab]$/.test(s.bone)) transformShape(s, { sx: S, tx: 0, sy: 1, ty: 0 });
  for (const s of g.plan) if (/^fr-[lr]1-b$/.test(s.id)) s.r1 = Math.round((s.r1 + 0.06) * 10000) / 10000;
}
{ const g = a.representations["500"].geometry; const S5 = 7 / 6.743;
  for (const s of g.front) if (s.id === "crown") transformShape(s, { sx: S5, tx: 0, sy: 1, ty: 0 });
  for (const s of g.plan) if (s.id === "crown") transformShape(s, { sx: 1, tx: 0, sy: S5, ty: 0 }); }
for (const den of [50, 100, 200, 500]) { const st = stateAt(a, den); const f = three.extentOf(st.shapes.front), p = three.extentOf(st.shapes.plan), e = three.extentOf(st.shapes.elevation);
  console.log(den, "el", (e.maxX - e.minX).toFixed(3), "fr", (f.maxX - f.minX).toFixed(3), "pl", (p.maxX - p.minX).toFixed(3), (p.maxY - p.minY).toFixed(3)); }
if (write) save(x);
