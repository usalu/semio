import { readFileSync } from "node:fs";
const M = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules";
const { gapsBetween, featureWidthMm, penOf } = await import(`${M}/🖨️representation/🟦️.ts`);
const { evaluate } = await import(`${M}/🧮️evaluation/🟦️.ts`);
const [file, scale] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
const d = Number(scale);
const rep = asset.representations[scale];
const k = 1000 / d;
for (const pose of asset.poses.filter((p: any) => !p.key)) {
  const st = evaluate(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: d } });
  const paper: any[] = [];
  for (const e of st.shapes.elevation) {
    const pen = penOf(rep.pens, e.shape);
    if (pen === null || e.shape.role === "ground") continue;
    const rings = e.rings.map((r: any) => r.map((p: any) => ({ x: p.x * k, y: -p.y * k })));
    paper.push({ id: e.shape.id, rings, closed: e.closed, penMm: pen });
    if (e.closed && e.shape.filled) { const w = featureWidthMm(rings, 0.2); if (w < 0.2) console.log(`${pose.id} FEATURE ${e.shape.id} ${w.toFixed(3)}`); }
  }
  for (const g of gapsBetween(paper, 1)) if (g.gapMm < 0.15) console.log(`${pose.id} GAP ${g.a} | ${g.b} ${g.gapMm.toFixed(3)} at ${(g.at.x / k).toFixed(3)},${(-g.at.y / k).toFixed(3)} m`);
}
