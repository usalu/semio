import { readFileSync, existsSync } from "node:fs";
const R = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const T = await import(`${R}/🧪️tests/🧊️three-views/🧰️.ts`);
const { evaluate } = await import(`${R}/🔨️modules/🧮️evaluation/🟦️.ts`);
const N = 5;
const scalesArg = Number(process.env.SC ?? 100);
for (const id of process.argv.slice(2)) {
  const f = ["🧱️components","🔌️appliances"].map(g=>`${R}/🗂️catalog/${g}/${id}/🔣️.json`).find(existsSync)!;
  const a = JSON.parse(readFileSync(f, "utf8")).asset;
  const tol = Math.max(0.015, T.toleranceM(a) * 0.5);
  const qs: any[] = [...a.poses.map((p:any)=>({n:`pose ${p.id}`,q:{kind:"pose",pose:p.id}}))];
  for (const c of a.clips) for (let i=0;i<N;i++) qs.push({n:`clip ${c.id}@${(c.durationS*i/N).toFixed(2)}`,q:{kind:"clip",clip:c.id,timeS:c.durationS*i/N}});
  const out = new Map<string,string>();
  for (const {n,q} of qs) {
    const st = evaluate(a,q,{kind:"scale",scale:{denominator:scalesArg}});
    const by: Record<string, Record<string, any[]>> = {};
    for (const v of ["elevation","front","plan"]) for (const e of st.shapes[v]) { const s=e.shape; if (s.role==="ground"||s.role==="texture"||s.role==="detail") continue; ((by[s.bone] ??= {})[v] ??= []).push(e); }
    for (const [bone,views] of Object.entries(by)) {
      if (!views.elevation||!views.front||!views.plan) { const miss=["elevation","front","plan"].filter(v=>!views[v]); const k=`${bone} missing ${miss}`; if(!out.has(k)) out.set(k,n); continue; }
      const e=T.extentOf(views.elevation,true), fr=T.extentOf(views.front,true), p=T.extentOf(views.plan,true);
      const chk=(l:string,A:number[],B:number[])=>{ const m=Math.max(Math.abs(A[0]-B[0]),Math.abs(A[1]-B[1])); if(m>tol){ const k=`${bone} ${l}`; if(!out.has(k)) out.set(k,`${n} [${A.map(v=>v.toFixed(3))}] vs [${B.map(v=>v.toFixed(3))}]`);} };
      chk("ex/px",[e.minX,e.maxX],[p.minX,p.maxX]); chk("ey/fy",[e.minY,e.maxY],[fr.minY,fr.maxY]); chk("fx/py",[fr.minX,fr.maxX],[p.minY,p.maxY]);
    }
  }
  console.log(`## ${id} tol=${tol.toFixed(3)} bone-mismatches=${out.size}`);
  for (const [k,v] of [...out].slice(0,25)) console.log("  "+k+" :: "+v);
}
