import { catalog, authoredScales, queries } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [, , mm, ...ids] = process.argv;
const eps = +mm / 1000;
const sel = (d: number) => ({ kind: "scale", scale: { denominator: d } }) as const;
function box(shapes: any[], bone: string) {
  const pts = shapes.filter((s) => s.shape.bone === bone && s.shape.role !== "ground").flatMap((s) => s.rings.flat());
  if (!pts.length) return undefined;
  const xs = pts.map((p: any) => p.x), ys = pts.map((p: any) => p.y);
  return { cx: (Math.min(...xs) + Math.max(...xs)) / 2, cy: (Math.min(...ys) + Math.max(...ys)) / 2, w: Math.max(...xs) - Math.min(...xs), h: Math.max(...ys) - Math.min(...ys) };
}
for (const a of catalog().filter((x) => ids.includes(x.id))) {
  for (const d of authoredScales(a)) {
    const rest: any = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, sel(d));
    const bad = new Map<string, string>();
    const moved = new Set<string>();
    for (const { name, query } of queries(a)) {
      const st: any = evaluate(a, query, sel(d));
      for (const b of a.rig.bones) {
        const e = box(st.shapes.elevation, b.id), f = box(st.shapes.front, b.id), p = box(st.shapes.plan, b.id);
        const re = box(rest.shapes.elevation, b.id), rf = box(rest.shapes.front, b.id), rp = box(rest.shapes.plan, b.id);
        const missing = [!e && "elevation", !f && "front", !p && "plan"].filter(Boolean);
        if (missing.length && missing.length < 3 && !bad.has(`${b.id} has-no-shape`)) bad.set(`${b.id} has-no-shape`, missing.join("+"));
        if (!e || !f || !p || !re || !rf || !rp) continue;
        const dex = e.cx - re.cx, dey = e.cy - re.cy, dfy = f.cy - rf.cy, dpx = p.cx - rp.cx;
        const dwe = e.w - re.w, dhe = e.h - re.h, dwp = p.w - rp.w, dhf = f.h - rf.h;
        if (Math.abs(dex) > eps || Math.abs(dey) > eps) moved.add(b.id);
        if (Math.abs(dex - dpx) > eps) bad.set(`${b.id} x-travel el/pl`, `${name}: el ${dex.toFixed(3)} pl ${dpx.toFixed(3)}`);
        if (Math.abs(dey - dfy) > eps) bad.set(`${b.id} y-travel el/fr`, `${name}: el ${dey.toFixed(3)} fr ${dfy.toFixed(3)}`);
        if (Math.abs(dwe - dwp) > eps * 2) bad.set(`${b.id} x-extent el/pl`, `${name}: el dw ${dwe.toFixed(3)} pl dw ${dwp.toFixed(3)}`);
        if (Math.abs(dhe - dhf) > eps * 2) bad.set(`${b.id} y-extent el/fr`, `${name}: el dh ${dhe.toFixed(3)} fr dh ${dhf.toFixed(3)}`);
      }
    }
    console.log(`## ${a.id} 1:${d} moving-bones=[${[...moved]}] issues=${bad.size}`);
    for (const [k, v] of bad) console.log(`   ${k}  ${v}`);
  }
}
