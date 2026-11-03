import { readFileSync, writeFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const id = process.env.QA4_ASSET!, clip = process.env.QA4_CLIP!, scale = Number(process.env.QA4_SCALE ?? 100);
const times = (process.env.QA4_TIMES ?? "").split(",").map(Number);
const asset = JSON.parse(readFileSync(`C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/${id}/🔣️.json`, "utf8")).asset;
const f = (n: number) => (Math.round(n * 1000) / 1000).toFixed(3);
const out: string[] = [];
for (const t of times) {
  const st = evaluate(asset, { kind: "clip", clip, timeS: t } as any, { kind: "scale", scale: { denominator: scale } } as any);
  out.push(`### ${clip} @${t}`);
  for (const view of ["elevation", "front", "plan"] as const) {
    const rows = st.shapes[view].filter((s: any) => s.shape.role !== "ground").map((s: any) => {
      const p = s.rings.flat();
      return { id: s.shape.id, bone: s.shape.bone, x0: Math.min(...p.map((q: any) => q.x)), x1: Math.max(...p.map((q: any) => q.x)), y0: Math.min(...p.map((q: any) => q.y)), y1: Math.max(...p.map((q: any) => q.y)) };
    });
    out.push(`-- ${view}`);
    for (const r of rows) out.push(`${r.id.padEnd(20)} ${r.bone.padEnd(12)} x[${f(r.x0)},${f(r.x1)}] y[${f(r.y0)},${f(r.y1)}]`);
  }
}
writeFileSync(process.env.QA4_LOG!, out.join("\n"));
