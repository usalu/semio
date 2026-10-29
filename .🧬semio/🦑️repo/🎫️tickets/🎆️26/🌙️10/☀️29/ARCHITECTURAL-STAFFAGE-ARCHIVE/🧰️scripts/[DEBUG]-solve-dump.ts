import { readFileSync } from "node:fs";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { solveStructure } = await import(`${S}/🔨️modules/🏗️structure/🟦️.ts`);
const doc = JSON.parse(readFileSync(`${S}/🗂️catalog/🧱️components/🔣️.json`, "utf8"));
const only = process.argv[2];
const f = (v: number) => (Math.abs(v) < 1e-6 ? "0" : v.toFixed(1));
for (const asset of doc.assets) {
  if (only && asset.id !== only) continue;
  console.log(`== ${asset.id}`);
  const queries = [...asset.poses.map((p: any) => ({ kind: "pose", pose: p.id })), ...asset.clips.map((c: any) => ({ kind: "clip", clip: c.id, timeS: c.durationS / 2 }))];
  for (const q of queries) {
    const st = evaluate(asset, q);
    const r = solveStructure(st);
    const label = q.kind === "pose" ? q.pose : `${q.clip}@${q.timeS}`;
    console.log(`  [${label}] W=${f(r.totalMassKg * r.gravityMS2)} resid(Fx,Fy,M)=${r.residual.fxN.toExponential(1)},${r.residual.fyN.toExponential(1)},${r.residual.momentNm.toExponential(1)}`);
    for (const rx of r.reactions) console.log(`    R ${rx.anchor} act=${rx.active} Fx=${f(rx.fxN)} Fy=${f(rx.fyN)} M=${f(rx.momentNm)}`);
    for (const [id, d] of Object.entries(r.members) as any) {
      const s0 = d.stations[0], sEnd = d.stations[d.stations.length - 1];
      const mx = d.stations.reduce((a: any, b: any) => (Math.abs(b.momentNm) > Math.abs(a.momentNm) ? b : a));
      console.log(`    M[${id}] s0 N=${f(s0.axialN)} V=${f(s0.shearN)} M=${f(s0.momentNm)} | end M=${f(sEnd.momentNm)} | max|M|=${f(mx.momentNm)} at s=${mx.s.toFixed(3)}${d.deflection ? ` | wmax=${Math.max(...d.deflection.map((p: any) => Math.abs(p.y))).toExponential(3)}` : ""}`);
    }
    for (const n of r.notes) if (n.en.includes("Unbalanced") || n.en.includes("slack") || n.en.includes("Model")) console.log(`    note: ${n.en.slice(0, 120)}`);
  }
}
