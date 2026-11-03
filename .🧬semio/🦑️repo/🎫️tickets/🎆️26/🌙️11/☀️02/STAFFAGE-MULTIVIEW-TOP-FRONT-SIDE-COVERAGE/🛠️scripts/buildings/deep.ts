import { readFileSync, existsSync } from "node:fs";
const R = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const T = await import(`${R}/🧪️tests/🧊️three-views/🧰️.ts`);
const { evaluate } = await import(`${R}/🔨️modules/🧮️evaluation/🟦️.ts`);
const ids = process.argv.slice(2);
const N = 11;
for (const id of ids) {
  const f = ["🧱️components","🔌️appliances","👤️humans"].map(g=>`${R}/🗂️catalog/${g}/${id}/🔣️.json`).find(existsSync)!;
  const a = JSON.parse(readFileSync(f, "utf8")).asset;
  const tol = T.toleranceM(a);
  const issues: string[] = [];
  for (const s of T.authoredScales(a)) {
    const qs: any[] = [...a.poses.map((p:any)=>({n:`pose ${p.id}`,q:{kind:"pose",pose:p.id}}))];
    for (const c of a.clips) for (let i=0;i<N;i++) qs.push({n:`clip ${c.id}@${(c.durationS*i/N).toFixed(2)}`,q:{kind:"clip",clip:c.id,timeS:c.durationS*i/N}});
    for (const {n,q} of qs) {
      let st; try { st = evaluate(a,q,{kind:"scale",scale:{denominator:s}}); } catch(e:any){ issues.push(`${s} ${n} eval ${e.message}`); continue; }
      const e = T.extentOf(st.shapes.elevation), fr = T.extentOf(st.shapes.front), p = T.extentOf(st.shapes.plan);
      if(!e||!fr||!p){ issues.push(`${s} ${n} empty e=${!!e} f=${!!fr} p=${!!p}`); continue; }
      const d = (x:number,y:number)=>Math.abs(x-y);
      const chk=(l:string,A:number[],B:number[])=>{ const m=Math.max(d(A[0],B[0]),d(A[1],B[1])); if(m>tol) issues.push(`${s} ${n} ${l} [${A.map(v=>v.toFixed(3))}] vs [${B.map(v=>v.toFixed(3))}]`); };
      chk("ex/px",[e.minX,e.maxX],[p.minX,p.maxX]);
      chk("ey/fy",[e.minY,e.maxY],[fr.minY,fr.maxY]);
      chk("fx/py",[fr.minX,fr.maxX],[p.minY,p.maxY]);
    }
  }
  console.log(`## ${id} tol=${tol.toFixed(3)} issues=${issues.length}`);
  const seen = new Set<string>();
  for (const i of issues) { const k=i.replace(/@[\d.]+/,"").replace(/\[.*\]/g,""); if(seen.has(k)&&seen.size>40) continue; seen.add(k); console.log("  "+i); }
}
