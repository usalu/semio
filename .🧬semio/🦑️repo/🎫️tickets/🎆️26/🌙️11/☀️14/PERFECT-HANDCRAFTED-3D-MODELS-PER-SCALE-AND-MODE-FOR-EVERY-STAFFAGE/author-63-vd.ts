/** 📐️ Extents of `volumes-views-disagree` per view with the pen widening, and which shape sets each end: bun author-63-vd.ts <asset> <scale> */
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [spec, sc] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const a: any = merged(asset, loadSource(entry)!);
const d = Number(sc);
const st: any = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: d } });
const axes: any = { elevation: ["x", "y"], front: ["z", "y"], plan: ["x", "z"] };
const ext: Record<string, { lo: number; hi: number; slo: string; shi: string }> = {};
for (const view of ["elevation", "front", "plan"]) for (const item of st.shapes[view]) {
  if (!["body", "detail", "glazing"].includes(item.shape.role)) continue;
  const pts = item.rings.flat(); if (!pts.length) continue;
  const stroke = !item.closed || item.shape.filled === false;
  const pen = stroke ? ((item.shape.penMm ?? a.representations[`${d}`].pens?.[item.shape.role] ?? 0) * d) / 2000 : 0;
  axes[view].forEach((ax: string, i: number) => {
    const vs = pts.flatMap((q: any) => (i === 0 ? [q.x - pen, q.x + pen] : [q.y - pen, q.y + pen]));
    const k = `${view}/${ax}`; const e = ext[k] ?? { lo: Infinity, hi: -Infinity, slo: "", shi: "" };
    if (Math.min(...vs) < e.lo) { e.lo = Math.min(...vs); e.slo = item.shape.id; }
    if (Math.max(...vs) > e.hi) { e.hi = Math.max(...vs); e.shi = item.shape.id; }
    ext[k] = e;
  });
}
for (const [k, e] of Object.entries(ext)) console.log(k.padEnd(12), e.lo.toFixed(4), e.hi.toFixed(4), (e.hi - e.lo).toFixed(4), e.slo, e.shi);
