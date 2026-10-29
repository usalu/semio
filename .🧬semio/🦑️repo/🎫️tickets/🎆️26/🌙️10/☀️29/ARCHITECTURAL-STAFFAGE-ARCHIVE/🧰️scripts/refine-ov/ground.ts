import { readFileSync } from "node:fs";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [, , file, only] = process.argv;
const doc = JSON.parse(readFileSync(file!, "utf8"));
for (const asset of doc.assets) {
  if (only && asset.id !== only) continue;
  const worst: Record<string, { y: number; where: string }> = {};
  const scan = (q: any, tag: string) => {
    const st: any = evaluate(asset, q);
    for (const s of st.shapes.elevation) {
      if (s.shape.role === "ground") continue;
      let m = Infinity; for (const ring of s.rings) for (const p of ring) m = Math.min(m, p.y);
      if (m < -0.005 && (!worst[s.shape.id] || m < worst[s.shape.id]!.y)) worst[s.shape.id] = { y: m, where: tag };
    }
  };
  for (const p of asset.poses) scan({ kind: "pose", pose: p.id }, p.id);
  for (const c of asset.clips) for (let i = 0; i <= 12; i++) scan({ kind: "clip", clip: c.id, timeS: (c.durationS * i) / 12 }, `${c.id}@${i}`);
  const list = Object.entries(worst).sort((a, b) => a[1].y - b[1].y);
  console.log(asset.id, list.length ? list.slice(0, 6).map(([k, v]) => `${k}=${(v.y * 1000).toFixed(0)}mm@${v.where}`).join(" ") : "ok");
}
