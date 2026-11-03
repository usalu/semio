import { readFileSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const [slug, scaleS, poseArg] = process.argv.slice(2);
const a = JSON.parse(readFileSync(`${L}/🗂️catalog/👤️humans/${slug}/🔣️.json`, "utf8")).asset;
const pose = poseArg || a.presentation.defaultPose;
const st = evaluate(a, { kind: "pose", pose }, { kind: "scale", scale: { denominator: +scaleS } });
for (const v of ["elevation", "front", "plan"]) {
  console.log("== " + v);
  const rows = st.shapes[v].filter((s: any) => s.shape.role !== "ground").map((s: any) => { const p = s.rings.flat(); const xs = p.map((q: any) => q.x), ys = p.map((q: any) => q.y); return { id: s.shape.id, bone: s.shape.bone, role: s.shape.role, kind: s.shape.kind, x: [Math.min(...xs), Math.max(...xs)].map((n) => +n.toFixed(3)), y: [Math.min(...ys), Math.max(...ys)].map((n) => +n.toFixed(3)) }; });
  for (const r of rows) console.log(`${r.id.padEnd(22)} ${r.bone.padEnd(14)} ${r.role.padEnd(7)} ${r.kind.padEnd(8)} x${JSON.stringify(r.x)} y${JSON.stringify(r.y)}`);
}
