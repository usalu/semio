import {readFileSync} from "node:fs";
const snap=JSON.parse(readFileSync(process.argv[2],"utf8")),{node_fields:nf,node_types:nt}=snap.snapshot.meta,N=nf.length,nodes=snap.nodes,strings=snap.strings;
const iType=nf.indexOf("type"),iName=nf.indexOf("name"),iSize=nf.indexOf("self_size"),types=nt[0];
const byPrefix=new Map(),dup=new Map();let total=0,count=0;
for(let n=0;n<nodes.length;n+=N){if(types[nodes[n+iType]]!=="string")continue;const s=strings[nodes[n+iName]],size=nodes[n+iSize];total+=size;count++;
 const key=s.slice(0,48).replace(/[0-9a-f]{16,}/g,"#").replace(/\d+/g,"9");const r=byPrefix.get(key)??[0,0];r[0]++;r[1]+=size;byPrefix.set(key,r);
 const d=dup.get(s)??[0,0];d[0]++;d[1]=size;dup.set(s,d);}
const mb=x=>(x/1048576).toFixed(0).padStart(5);
console.log("strings",count,mb(total)+"MB");
console.log("== by prefix\n"+[...byPrefix].sort((a,b)=>b[1][1]-a[1][1]).slice(0,25).map(([k,[c,s]])=>`${mb(s)}MB ${String(c).padStart(8)} ${JSON.stringify(k)}`).join("\n"));
let dupBytes=0,dupN=0;for(const [,[c,s]] of dup)if(c>1){dupBytes+=s*(c-1);dupN+=c-1;}
console.log("== duplicates: redundant copies",dupN,mb(dupBytes)+"MB; distinct",dup.size);
console.log([...dup].filter(([,[c]])=>c>1).sort((a,b)=>b[1][0]*b[1][1]-a[1][0]*a[1][1]).slice(0,12).map(([k,[c,s]])=>`${mb(c*s)}MB x${c} ${JSON.stringify(k.slice(0,100))}`).join("\n"));
