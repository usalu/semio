/** 🔎️ Prints which shapes set the extremes of the default pose per scale and view: bun extent-probe.ts <category/slug> */
import { join, resolve } from "node:path";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const asset = (await Bun.file(join(catalog, process.argv[2]!, "🔣️.json")).json()).asset;
console.log(JSON.stringify(asset.dimensions), asset.presentation.defaultPose);
for (const d of (process.argv[3] ?? "50,100,200,500").split(",").map(Number)) {
  const st = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: d } });
  console.log(`== 1:${d} bounds x ${st.bounds.minX.toFixed(3)}..${st.bounds.maxX.toFixed(3)} y ${st.bounds.minY.toFixed(3)}..${st.bounds.maxY.toFixed(3)}`);
  for (const vp of ["elevation", "front", "plan"] as const) {
    const items = st.shapes[vp].filter((s: any) => s.shape.role !== "ground").map((s: any) => {
      const p = s.rings.flat(); if (!p.length) return null;
      const xs = p.map((q: any) => q.x), ys = p.map((q: any) => q.y);
      return { id: s.shape.id, bone: s.shape.bone, x0: Math.min(...xs), x1: Math.max(...xs), y0: Math.min(...ys), y1: Math.max(...ys) };
    }).filter(Boolean) as any[];
    if (!items.length) continue;
    const f = (k: string, sign: number) => items.slice().sort((a, b) => sign * (b[k] - a[k]))[0];
    const mx = f("x1", 1), mn = f("x0", -1), my = f("y1", 1), ny = f("y0", -1);
    console.log(` ${vp}: x[${mn.x0.toFixed(3)} ${mn.id}/${mn.bone} .. ${mx.x1.toFixed(3)} ${mx.id}/${mx.bone}] y[${ny.y0.toFixed(3)} ${ny.id}/${ny.bone} .. ${my.y1.toFixed(3)} ${my.id}/${my.bone}]`);
  }
}
