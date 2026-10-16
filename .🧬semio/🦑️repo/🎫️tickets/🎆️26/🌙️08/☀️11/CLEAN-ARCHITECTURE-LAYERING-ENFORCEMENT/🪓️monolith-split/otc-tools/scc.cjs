const fs=require("fs");
const [,,file,prefix]=process.argv;
const j=JSON.parse(fs.readFileSync(file,"utf8"));
const adj=new Map();
for(const m of j.modules){ if(!adj.has(m.source))adj.set(m.source,[]);
  for(const d of m.dependencies){ if(!d.resolved||d.coreModule||d.couldNotResolve)continue; adj.get(m.source).push({to:d.resolved,types:d.dependencyTypes,circ:d.circular}); } }
for(const m of j.modules) for(const d of m.dependencies) if(d.resolved&&!adj.has(d.resolved))adj.set(d.resolved,[]);
let idx=0;const I=new Map(),L=new Map(),on=new Set(),st=[],sccs=[];
function strong(v){ const stack=[[v,0]]; I.set(v,idx);L.set(v,idx);idx++;st.push(v);on.add(v);
  while(stack.length){const top=stack[stack.length-1];const [u,i]=top;const e=adj.get(u);
    if(i<e.length){top[1]++;const w=e[i].to; if(!I.has(w)){I.set(w,idx);L.set(w,idx);idx++;st.push(w);on.add(w);stack.push([w,0]);} else if(on.has(w)) L.set(u,Math.min(L.get(u),I.get(w)));}
    else{ if(L.get(u)===I.get(u)){const c=[];let w;do{w=st.pop();on.delete(w);c.push(w);}while(w!==u);sccs.push(c);} stack.pop(); if(stack.length){const p=stack[stack.length-1][0];L.set(p,Math.min(L.get(p),L.get(u)));}}}}
for(const v of adj.keys()) if(!I.has(v)) strong(v);
const big=sccs.filter(c=>c.length>1).sort((a,b)=>b.length-a.length);
const out={sccs:big.map(c=>{const s=new Set(c);let edges=0;for(const v of c)for(const e of adj.get(v)) if(s.has(e.to))edges++;return {size:c.length,edges,members:c.sort()}})};
fs.writeFileSync(file.replace(/\.json$/,".scc.json"),JSON.stringify(out,null,1));
console.log("sccs>1:",big.length,"sizes",big.map(c=>c.length).join(","));
const viol=j.summary.violations.filter(v=>v.rule.name==="no-circular");
console.log("no-circular violations total:",viol.length);
if(prefix){const ps=prefix.split("|");console.log("violations with from under any of",ps,":",viol.filter(v=>ps.some(p=>v.from.startsWith(p))).length);}
