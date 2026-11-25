import {readFileSync} from "node:fs";
const p = JSON.parse(readFileSync(process.argv[2])); const nodes = new Map(p.nodes.map(n=>[n.id,n]));
const parent = new Map(); for (const n of p.nodes) for (const c of n.children??[]) parent.set(c,n.id);
const self = new Map(); let tot=0; p.samples.forEach((id,i)=>{const d=p.timeDeltas[i]??0; self.set(id,(self.get(id)??0)+d); tot+=d;});
const nm = (id)=>{const c=nodes.get(id).callFrame; return (c.functionName||"(anon)")+(c.url?"@"+c.url.split("/").pop().slice(0,22):"");};
// inclusive by function name (count once per stack)
const incl=new Map(), selfF=new Map();
for (const [id,t] of self){ const seen=new Set(); let x=id; while(x!==undefined){ const k=nm(x).replace(/\?v=\d+/,""); if(!seen.has(k)){seen.add(k); incl.set(k,(incl.get(k)??0)+t);} x=parent.get(x);} const k=nm(id).replace(/\?v=\d+/,""); selfF.set(k,(selfF.get(k)??0)+t);}
console.log("total ms",Math.round(tot/1000));
console.log("== inclusive top"); [...incl].sort((a,b)=>b[1]-a[1]).slice(0,+process.argv[3]||40).forEach(([k,t])=>console.log((t/tot*100).toFixed(1).padStart(5)+"% incl "+(( selfF.get(k)??0)/tot*100).toFixed(1).padStart(5)+"% self  "+k));
