import{readFileSync}from"node:fs";
const g=await import("/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs");
const m=JSON.parse(readFileSync("manifest.json","utf8")),v=g.unpackFactIndexV1(readFileSync("index.facts.br"),4e9);
const names=v.owners;let total=0;const per=new Map();
for(const c of v.classes){const r=c.r.length;for(const o of c.o){per.set(o,(per.get(o)??0)+r);total+=r;}}
console.log("owners in index",names.length,"rows",v.rows.length,"classes",v.classes.length,"incidences",total);
const kind=new Map();
for(const [i,n] of names.entries()){const row=m.owners[n];const k=row?(row.tag??row.kind):"pseudo";const e=kind.get(k)??{n:0,sum:0,max:0};e.n++;e.sum+=per.get(i)??0;e.max=Math.max(e.max,per.get(i)??0);kind.set(k,e);}
for(const [k,e] of kind)console.log(k,"owners",e.n,"rows total",e.sum,"avg",Math.round(e.sum/e.n),"max",e.max);
const top=[...per].sort((a,b)=>b[1]-a[1]).slice(0,8).map(([i,c])=>[names[i].slice(0,60),c]);console.log(top);
