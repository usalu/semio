import { catalog, authoredScales, extentOf, queries, toleranceM } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const ids = (process.env.IDS ?? "").split(",").filter(Boolean);
const only = (process.env.SCALES ?? "").split(",").filter(Boolean).map(Number);
const TOL = Number(process.env.TOL ?? 0.05);
const FR = Number(process.env.FRACTIONS ?? 0);
const ctr = (shapes:any[], bone:string, axis:"x"|"y", wh?:string) => { const e = extentOf(shapes.filter((s:any)=>s.shape.bone===bone), true); return e ? (axis==="x" ? (e.minX+e.maxX)/2 : (e.minY+e.maxY)/2) : undefined; };
const rng = (shapes:any[], bone:string) => extentOf(shapes.filter((s:any)=>s.shape.bone===bone), true);
for (const a of catalog()) {
  if (!["machinery","vehicles"].includes(a.category)) continue;
  if (ids.length && !ids.includes(a.id)) continue;
  for (const s of authoredScales(a)) {
    if (only.length && !only.includes(s)) continue;
    const sel = {kind:"scale",scale:{denominator:s}} as any;
    const rest:any = evaluate(a, {kind:"pose",pose:a.presentation.defaultPose}, sel);
    const bad = new Map<string, {err:number; at:string; kind:string}>();
    const note = (k:string, err:number, at:string, kind:string)=>{ const o=bad.get(k); if(!o||err>o.err) bad.set(k,{err,at,kind}); };
    for (const {name, query} of queries(a)) {
      let st:any; try { st = evaluate(a, query, sel); } catch { continue; }
      const bones = new Set<string>([...st.shapes.plan, ...st.shapes.front, ...st.shapes.elevation].map((x:any)=>x.shape.bone));
      for (const b of bones) {
        const eC = ctr(st.shapes.elevation,b,"x"), eR = ctr(rest.shapes.elevation,b,"x");
        const eCy = ctr(st.shapes.elevation,b,"y"), eRy = ctr(rest.shapes.elevation,b,"y");
        const pC = ctr(st.shapes.plan,b,"x"), pR = ctr(rest.shapes.plan,b,"x");
        const fCy = ctr(st.shapes.front,b,"y"), fRy = ctr(rest.shapes.front,b,"y");
        const ex = eC!==undefined&&eR!==undefined ? eC-eR : undefined;
        const ey = eCy!==undefined&&eRy!==undefined ? eCy-eRy : undefined;
        const px = pC!==undefined&&pR!==undefined ? pC-pR : undefined;
        const fy = fCy!==undefined&&fRy!==undefined ? fCy-fRy : undefined;
        if (ex!==undefined&&px!==undefined) note(`${b} plan-x`, Math.abs(ex-px), name, `E dx=${ex.toFixed(2)} P dx=${px.toFixed(2)}`);
        if (ey!==undefined&&fy!==undefined) note(`${b} front-y`, Math.abs(ey-fy), name, `E dy=${ey.toFixed(2)} F dy=${fy.toFixed(2)}`);
      }
    }
    const out=[...bad.entries()].filter(([,v])=>v.err>TOL).sort((x,y)=>y[1].err-x[1].err);
    console.log(`## ${a.id} 1:${s} offenders ${out.length}`);
    for (const [k,v] of out.slice(0,12)) console.log(`   ${k} err=${v.err.toFixed(2)} @${v.at} ${v.kind}`);
  }
}
