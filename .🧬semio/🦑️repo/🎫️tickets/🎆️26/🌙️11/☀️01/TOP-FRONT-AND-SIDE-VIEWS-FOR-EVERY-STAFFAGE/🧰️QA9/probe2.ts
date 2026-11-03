import { readFileSync, writeFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [id, scale, spec, only] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(`C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/${id}/🔣️.json`, "utf8")).asset;
const q: any = spec!.includes("@") ? { kind: "clip", clip: spec!.split("@")[0], timeS: Number(spec!.split("@")[1]) } : { kind: "pose", pose: spec };
const st = evaluate(asset, q, { kind: "scale", scale: { denominator: Number(scale) } } as any);
const f = (n: number) => n.toFixed(3);
const out: string[] = [];
for (const view of ["elevation", "front", "plan"] as const) {
  out.push(`-- ${view}`);
  for (const s of st.shapes[view]) {
    if (only && !s.shape.id.match(new RegExp(only)) && s.shape.role !== "ground") continue;
    const p = s.rings.flat();
    if (!p.length) continue;
    out.push(`${s.shape.id.padEnd(22)} ${s.shape.bone.padEnd(14)} ${s.shape.role.padEnd(7)} x[${f(Math.min(...p.map((r: any) => r.x)))},${f(Math.max(...p.map((r: any) => r.x)))}] y[${f(Math.min(...p.map((r: any) => r.y)))},${f(Math.max(...p.map((r: any) => r.y)))}]`);
  }
}
writeFileSync(process.env.QA9_LOG!, out.join("\n"));
