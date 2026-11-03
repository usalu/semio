import { load, save, stateAt, three } from "./lib.ts";
const id = process.argv[2]!; const write = process.argv[3] === "write"; const x = load(id); const a = x.asset;
const m = (den: number) => { const st = stateAt(a, den); const f = three.extentOf(st.shapes.front), p = three.extentOf(st.shapes.plan); return `fr ${(f.maxX - f.minX).toFixed(3)} pl ${(p.maxX - p.minX).toFixed(3)}x${(p.maxY - p.minY).toFixed(3)}`; };
if (id === "date-palm") {
  const pose = a.poses.find((q: any) => q.id === "calm");
  const T = Number(process.argv[4] ?? 78);
  for (const b of ["fr-l1a", "fr-r1a"]) pose.bones[b].abductionDeg = Math.sign(pose.bones[b].abductionDeg) * T;
}
if (id === "hosta-clump") {
  for (const den of [50, 100]) for (const s of a.representations[String(den)].geometry.plan) if (/^blade-[LR]2$/.test(s.id)) { s.r0 = Math.round((s.r0 - 0.012) * 10000) / 10000; s.r1 = Math.round((s.r1 - 0.012) * 10000) / 10000; }
  for (const s of a.representations["100"].geometry.plan) if (/^(blade|midrib)-[LR]1$/.test(s.id)) { const sg = Math.sign(s.x1); s.x1 = Math.round((s.x1 + sg * 0.011) * 10000) / 10000; }
}
for (const den of [50, 100, 200, 500]) if (a.representations[String(den)]?.status === "authored") console.log(id, den, m(den));
if (write) save(x);
