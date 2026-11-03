import { readFileSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const [id, den] = [process.argv[2], Number(process.argv[3])];
const a = JSON.parse(readFileSync(`${L}/🗂️catalog/🌳️vegetation/${id}/🔣️.json`, "utf8")).asset;
const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: den } });
const f = (n: number) => n.toFixed(1).padStart(5);
const box: any = { elevation: {}, front: {}, plan: {} };
for (const v of ["elevation", "front", "plan"]) for (const s of st.shapes[v]) {
  if (s.shape.role === "ground") continue;
  const k = s.shape.bone; const b = box[v][k] ??= [1e9, -1e9, 1e9, -1e9, ""];
  for (const p of s.rings.flat()) { b[0] = Math.min(b[0], p.x); b[1] = Math.max(b[1], p.x); b[2] = Math.min(b[2], p.y); b[3] = Math.max(b[3], p.y); }
  b[4] += s.shape.region ? "" : "";
}
const show = (b: number[] | undefined) => b ? `[${f(b[0])},${f(b[1])}]x[${f(b[2])},${f(b[3])}]` : "      -      ";
for (const bone of a.rig.bones) console.log(bone.id.padEnd(14), "el", show(box.elevation[bone.id]), "fr", show(box.front[bone.id]), "pl", show(box.plan[bone.id]));
