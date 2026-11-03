import { readdirSync } from "node:fs";
import { CAT, boxes, load, save, stateAt } from "./lib.ts";
const write = process.argv[2] === "write"; const only = process.argv[3];
for (const d of readdirSync(CAT)) {
  if (["agave", "date-palm", "hosta-clump"].includes(d)) continue;
  if (only && d !== only) continue;
  const x = load(d); const a = x.asset; const st = stateAt(a, 100); const bx = boxes(st);
  let n = 0, planDiff = 0; const rows: string[] = [];
  for (const an of a.anchors) {
    if (an.kind !== "emitter" || an.z !== undefined) continue;
    const f = bx.front[an.bone]; const p = bx.plan[an.bone]; if (!f) continue;
    const lat = Math.round(((f[0] + f[1]) / 2) * 1000) / 1000; if (Math.abs(lat) < 0.05) continue;
    an.z = lat; n++; if (p && Math.abs((p[2] + p[3]) / 2 - lat) > 0.3) planDiff++;
    rows.push(`${an.id}@${an.bone}:${lat}`);
  }
  console.log(d, "emitter anchors with lateral z:", n, "plan centre disagrees:", planDiff, rows.slice(0, 4).join(" "));
  if (write) save(x);
}
