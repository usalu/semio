import { readFileSync } from "node:fs";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [path, clip, t, scale, view] = process.argv.slice(2);
const a = JSON.parse(readFileSync(path!, "utf8")).asset;
const st: any = evaluate(a, { kind: "clip", clip: clip!, timeS: Number(t) }, { kind: "scale", scale: { denominator: Number(scale) } });
for (const v of ["elevation", view!]) {
  const list = st.shapes[v].filter((e: any) => e.shape.role !== "ground").map((e: any) => { const pts = e.rings.flat(); return { id: e.shape.id, bone: e.shape.bone, kind: e.shape.kind, f: !!e.shape.followTip, maxY: Math.max(...pts.map((p: any) => p.y)) }; }).sort((x: any, y: any) => y.maxY - x.maxY).slice(0, 5);
  console.log(v, "\n  " + list.map((x: any) => x.id + "/" + x.bone + "/" + x.kind + (x.f ? "/tip" : "") + " " + x.maxY.toFixed(3)).join("\n  "));
}
