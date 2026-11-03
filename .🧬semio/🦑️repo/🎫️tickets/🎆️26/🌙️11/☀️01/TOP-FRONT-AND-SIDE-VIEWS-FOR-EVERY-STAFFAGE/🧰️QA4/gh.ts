import { readFileSync, writeFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const a = JSON.parse(readFileSync("C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/gas-hob/🔣️.json", "utf8")).asset;
const out: string[] = [];
for (const p of a.poses.filter((q: any) => !q.key)) {
  const st: any = evaluate(a, { kind: "pose", pose: p.id } as any, { kind: "scale", scale: { denominator: 200 } } as any);
  for (const v of ["elevation", "front", "plan"]) for (const s of st.shapes[v]) {
    const q = s.rings.flat(); const f = (k: string) => [Math.min(...q.map((r: any) => r[k])), Math.max(...q.map((r: any) => r[k]))].map((n) => n.toFixed(3)).join("..");
    out.push(`${p.id} ${v} ${s.shape.id} z${s.shape.z} x[${f("x")}] y[${f("y")}]`);
  }
}
writeFileSync(process.env.QA4_LOG!, out.join("\n"));
