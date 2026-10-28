// [gram-g] heap snapshot census: self size by constructor/type, string duplication. usage: bun snap.mjs <snapshot.json> [top]
import{readFileSync}from"node:fs";
const file=process.argv[2],top=Number(process.argv[3]??"40");
const snap=JSON.parse(readFileSync(file,"utf8")),{node_fields:nf,node_types:nt}=snap.snapshot.meta,N=nf.length,nodes=snap.nodes,strings=snap.strings;
const iType=nf.indexOf("type"),iName=nf.indexOf("name"),iSize=nf.indexOf("self_size"),types=nt[0];
const by=new Map(),str=new Map();let total=0,stringBytes=0,stringCount=0;
for(let i=0;i<nodes.length;i+=N){
 const type=types[nodes[i+iType]],name=strings[nodes[i+iName]],size=nodes[i+iSize];total+=size;
 const key=type+":"+(type==="string"||type==="concatenated string"||type==="sliced string"?"":name.slice(0,60));
 const e=by.get(key)??{n:0,size:0};e.n++;e.size+=size;by.set(key,e);
 if(type==="string"||type==="concatenated string"){stringBytes+=size;stringCount++;const t=name.length>200?name.slice(0,200)+"#"+name.length:name;const s=str.get(t)??{n:0,size:0};s.n++;s.size+=size;str.set(t,s);}
}
const mb=n=>(n/1048576).toFixed(1);
console.log("nodes",nodes.length/N,"total MB",mb(total),"strings",stringCount,"MB",mb(stringBytes));
for(const [k,v] of [...by].sort((a,b)=>b[1].size-a[1].size).slice(0,top))console.log(mb(v.size).padStart(8),String(v.n).padStart(9),k);
let dupBytes=0,dupN=0;for(const [,v] of str)if(v.n>1){dupBytes+=v.size-v.size/v.n;dupN+=v.n-1;}
console.log("string duplicates: extra instances",dupN,"reclaimable MB",mb(dupBytes),"distinct",str.size);
console.log("top duplicated strings:");
for(const [t,v] of [...str].filter(([,v])=>v.n>1).sort((a,b)=>(b[1].size-b[1].size/b[1].n)-(a[1].size-a[1].size/a[1].n)).slice(0,15))console.log(mb(v.size-v.size/v.n).padStart(8),String(v.n).padStart(8),JSON.stringify(t.slice(0,110)));
{const cls=t=>t.length<=12?"short<=12":/^[0-9a-f]{64}$/.test(t)?"hex64":/^"[0-9a-f]{64}"$/.test(t)?"qhex64":t.startsWith("/Users/")?"abs-path":t.startsWith("{\"provider")?"json-provider":t.startsWith("{")||t.startsWith("[")?"json-other":t.includes("\0")?"key-nul":/^[^\s/]*[\u{1F300}-\u{1FAFF}]/u.test(t)?"rel-emoji-path":t.includes("/")?"path-other":"other";
 const agg=new Map();for(const [t,v] of str){if(v.n<2)continue;const c=cls(t),e=agg.get(c)??{distinct:0,extra:0,mb:0};e.distinct++;e.extra+=v.n-1;e.mb+=v.size-v.size/v.n;agg.set(c,e);}
 console.log("reclaimable by class");for(const [c,e] of [...agg].sort((a,b)=>b[1].mb-a[1].mb))console.log(mb(e.mb*1).padStart(8),String(e.extra).padStart(9),String(e.distinct).padStart(8),c);}
