import {readFileSync} from "node:fs";
import {join} from "node:path";
const G="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs";
const {openPackedFactIndexV1}=await import(G);
const [dir,digest]=process.argv.slice(2);
const index=openPackedFactIndexV1(readFileSync(join(dir,"x",digest+".facts.br")),1<<30);
const c=new Map();const ex=[];
for(let i=0;i<index.rows.length;i++){const r=index.rows[i];
 if(r.kind==="resolve"){const [from,request]=JSON.parse(r.path);const k=(request.startsWith(".")?"relative":request.startsWith("/")?"absolute":"bare")+" -> "+(typeof r.fact==="string"&&r.fact.startsWith("!")?"unresolved":/\/(node_modules|\.cache)\//.test(r.fact)?"into node_modules/.cache":"inside");c.set(k,(c.get(k)??0)+1);if(k.startsWith("bare")&&ex.length<3)ex.push([request,r.fact].join(" => "));}
 else if(r.kind==="registry"||r.kind==="closure"||r.kind==="derived"){const k=r.kind;c.set(k,(c.get(k)??0)+1);}
}
console.log([...c].sort((a,b)=>b[1]-a[1]));console.log(ex);
