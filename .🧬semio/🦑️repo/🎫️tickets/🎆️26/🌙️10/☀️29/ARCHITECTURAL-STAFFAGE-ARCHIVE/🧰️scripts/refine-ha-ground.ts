import { readFileSync } from "node:fs";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [file, only] = process.argv.slice(2);
const doc = JSON.parse(readFileSync(file!, "utf8"));
for (const a of doc.assets) {
  if (only && a.id !== only) continue;
  const rows: string[] = [];
  const probe = (label: string, q: any) => {
    const s = evaluate(a, q);
    let worst = 1e9, who = "";
    for (const e of s.shapes.elevation) {
      if (e.shape.kind === "line" && false) continue;
      for (const r of e.rings) for (const p of r) if (p.y < worst) { worst = p.y; who = e.shape.id; }
    }
    if (worst < -0.005) rows.push(`${label}: ${(worst * 1000).toFixed(1)} mm ${who} phase=${s.phase}`);
  };
  for (const p of a.poses) probe("pose " + p.id + (p.key ? "*" : ""), { kind: "pose", pose: p.id });
  for (const c of a.clips) { const n = 40; for (let i = 0; i < n; i++) probe(`clip ${c.id}@${(c.durationS * i / n).toFixed(3)}`, { kind: "clip", clip: c.id, timeS: c.durationS * i / n }); }
  console.log(a.id, rows.length ? "\n  " + rows.join("\n  ") : "ok");
}
