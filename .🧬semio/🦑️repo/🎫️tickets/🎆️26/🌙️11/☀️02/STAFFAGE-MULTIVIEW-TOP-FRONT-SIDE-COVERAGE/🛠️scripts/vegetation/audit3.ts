import { readdirSync, readFileSync, writeFileSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { toleranceM, authoredScales } = await import(`${L}/🧪️tests/🧊️three-views/🧰️.ts`);
const root = `${L}/🗂️catalog/🌳️vegetation`;
const only = process.argv[2];
const f = (n: number) => n.toFixed(1);
const out: string[] = [];
for (const d of readdirSync(root)) {
  if (only && only !== "all" && d !== only) continue;
  const a = JSON.parse(readFileSync(`${root}/${d}/🔣️.json`, "utf8")).asset;
  out.push(`## ${a.id}`);
  for (const den of authoredScales(a)) {
    const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: den } });
    const box: Record<string, Record<string, number[]>> = { elevation: {}, front: {}, plan: {} };
    for (const v of ["elevation", "front", "plan"]) for (const s of st.shapes[v]) {
      if (s.shape.role === "ground") continue;
      const pts = s.rings.flat(); if (!pts.length) continue;
      const b = box[v][s.shape.bone] ??= [1e9, -1e9, 1e9, -1e9];
      for (const p of pts) { b[0] = Math.min(b[0], p.x); b[1] = Math.max(b[1], p.x); b[2] = Math.min(b[2], p.y); b[3] = Math.max(b[3], p.y); }
    }
    const tol = toleranceM(a) * 2;
    const bad: string[] = [];
    for (const bone of Object.keys(box.elevation)) {
      const e = box.elevation[bone], fr = box.front[bone], p = box.plan[bone];
      const c = (b: number[], i: number) => (b[i] + b[i + 1]) / 2;
      if (e && p && Math.abs(c(e, 0) - c(p, 0)) > tol) bad.push(`${bone}:x el ${f(c(e, 0))} pl ${f(c(p, 0))}`);
      if (e && fr && Math.abs(c(e, 2) - c(fr, 2)) > tol) bad.push(`${bone}:y el ${f(c(e, 2))} fr ${f(c(fr, 2))}`);
      if (fr && p && Math.abs(c(fr, 0) - c(p, 2)) > tol) bad.push(`${bone}:lat fr ${f(c(fr, 0))} pl ${f(c(p, 2))}`);
    }
    out.push(`  1:${den} ${bad.length} mismatches (tol ${f(tol)}): ${bad.slice(0, 14).join("; ")}`);
  }
}
writeFileSync(process.argv[3], out.join("\n"));
