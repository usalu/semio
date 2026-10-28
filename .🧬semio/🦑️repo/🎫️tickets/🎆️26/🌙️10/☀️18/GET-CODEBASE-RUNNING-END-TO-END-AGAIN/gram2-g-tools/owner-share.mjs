// [gram2-g] how the rows of a few owners are shared in a sealed generation index: per owner the rows by number of consumers (read only). usage: bun owner-share.mjs <gen-dir> <owner>...
import{readFileSync}from"node:fs";import{join}from"node:path";
const D=process.argv[2],WANT=process.argv.slice(3);
const g=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs");
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
console.log("generation",pointer.generation,JSON.stringify(Object.keys(pointer)).slice(0,120));
const index=g.unpackFactIndexV1(readFileSync(join(D,"x",manifest.factIndex+".facts.br")),g.GENERATION_POLICY_V1.maximumIndexBytes);
const buckets=[1,2,3,10,100,1000,4000,1e9];
for(const name of WANT.length?WANT:[index.owners.find(n=>!n.startsWith("@")&&n.length<30)]){const at=index.owners.indexOf(name);if(at<0){console.log(name,"?");continue;}
 const hist=new Map(),kinds=new Map();let total=0;
 for(const c of index.classes){if(!c.o.includes(at))continue;const b=buckets.find(x=>c.o.length<=x);hist.set(b,(hist.get(b)??0)+c.r.length);total+=c.r.length;for(const id of c.r){const k=index.rows[id].kind+(c.o.length>=1000?"*":"");kinds.set(k,(kinds.get(k)??0)+1);}}
 console.log(name,"rows",total,"by consumers<=",JSON.stringify([...hist]),"kinds",[...kinds].sort((a,b)=>b[1]-a[1]).slice(0,10).map(([k,v])=>k+":"+v).join(" "));}
