import {readFileSync} from "node:fs";
const p = JSON.parse(readFileSync(process.argv[2])); const nodes = new Map(p.nodes.map(n=>[n.id,n]));
const self = new Map(); let tot=0; p.samples.forEach((id,i)=>{const d=p.timeDeltas[i]??0; self.set(id,(self.get(id)??0)+d); tot+=d;});
const nm = (id)=>{const c=nodes.get(id).callFrame; return (c.functionName||"(anon)")+(c.url?"@"+c.url.split("/").pop().slice(0,16):"")+(c.lineNumber>=0&&!/wasm/.test(c.functionName)?":"+c.lineNumber:"");};
const incl = (id)=>{const n=nodes.get(id); if(n.__i!==undefined) return n.__i; let s=self.get(id)??0; for(const c of n.children??[]) s+=incl(c); return n.__i=s;};
const root=p.nodes[0].id; incl(root);
const pat=new RegExp(process.argv[3]); const depth=+process.argv[4]||6; const minPct=+process.argv[5]||1;
function show(id,d,ind){ const n=nodes.get(id); console.log(" ".repeat(ind)+(n.__i/tot*100).toFixed(1)+"% "+nm(id)); if(d===0)return; const kids=(n.children??[]).map(c=>nodes.get(c)).filter(c=>c.__i/tot*100>=minPct).sort((a,b)=>b.__i-a.__i); // merge same-name
  for(const k of kids) show(k.id,d-1,ind+1);}
const seen=new Set();
for(const n of p.nodes){ if(pat.test(nm(n.id)) && n.__i/tot>0.03 && !seen.size){ show(n.id,depth,0); seen.add(1);} }
