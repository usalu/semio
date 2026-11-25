import {readFileSync} from "node:fs";
for (const f of process.argv.slice(2)) {
  const p = JSON.parse(readFileSync(f)); const nodes = new Map(p.nodes.map(n=>[n.id,n]));
  const self = new Map(); const dt = p.timeDeltas; let tot=0;
  p.samples.forEach((id,i)=>{const d=dt[i]??0; self.set(id,(self.get(id)??0)+d); tot+=d;});
  const byName = new Map(); for (const [id,t] of self){const n=nodes.get(id); const k=n.callFrame.functionName+" "+n.callFrame.url.slice(-40); byName.set(k,(byName.get(k)??0)+t);}
  const idle=[...byName].filter(([k])=>/\(idle\)/.test(k)).reduce((a,[,t])=>a+t,0);
  console.log(f.slice(-30),"total ms",Math.round(tot/1000),"idle ms",Math.round(idle/1000),"nodes",p.nodes.length);
  [...byName].sort((a,b)=>b[1]-a[1]).slice(0,12).forEach(([k,t])=>console.log(" ",Math.round(t/1000),k));
}
