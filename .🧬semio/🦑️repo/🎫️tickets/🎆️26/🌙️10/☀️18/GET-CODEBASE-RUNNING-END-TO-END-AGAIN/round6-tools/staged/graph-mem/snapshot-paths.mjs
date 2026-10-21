// 🧠️ Names what retains the biggest objects of a V8-format heap snapshot (Bun.generateHeapSnapshot): self sizes by type and by constructor, then the shortest reference path from the root to each of the biggest nodes and to a sample of the biggest string groups.
import {readFileSync} from "node:fs";
const [path,topArg="25"]=process.argv.slice(2),top=Number(topArg);
const d=JSON.parse(readFileSync(path,"utf8")),meta=d.snapshot.meta,NF=meta.node_fields.length,EF=meta.edge_fields.length,ntypes=meta.node_types[0],etypes=meta.edge_types[0];
const nodes=d.nodes,edges=d.edges,strings=d.strings,count=d.snapshot.node_count;
const type=i=>ntypes[nodes[i*NF]],name=i=>strings[nodes[i*NF+1]],self=i=>nodes[i*NF+3],ecount=i=>nodes[i*NF+4];
const first=new Int32Array(count+1);for(let i=0;i<count;i++)first[i+1]=first[i]+ecount(i);
const parent=new Int32Array(count).fill(-1),via=new Int32Array(count).fill(-1),seen=new Uint8Array(count),queue=new Int32Array(count);let head=0,tail=0;queue[tail++]=0;seen[0]=1;
while(head<tail){const v=queue[head++];for(let e=first[v];e<first[v+1];e++){const et=etypes[edges[e*EF]];if(et==="weak")continue;const w=edges[e*EF+2]/NF;if(seen[w])continue;seen[w]=1;parent[w]=v;via[w]=e;queue[tail++]=w;}}
const byType=new Map(),byName=new Map();let total=0;
for(let i=0;i<count;i++){const t=type(i),s=self(i);total+=s;byType.set(t,(byType.get(t)??0)+s);if(t==="object"){const n=name(i);const r=byName.get(n)??[0,0];r[0]+=s;r[1]++;byName.set(n,r);}}
const mb=n=>(n/1e6).toFixed(1);
console.log("total self MB",mb(total),"nodes",count);
for(const [k,v] of [...byType].sort((a,b)=>b[1]-a[1]).slice(0,6))console.log("  type",k,mb(v));
for(const [k,v] of [...byName].sort((a,b)=>b[1][0]-a[1][0]).slice(0,8))console.log("  object",k,mb(v[0]),"MB",v[1]);
const chain=i=>{const out=[];let v=i,guard=0;while(v>0&&guard++<14){const e=via[v];const et=etypes[edges[e*EF]],en=edges[e*EF+1];const label=(et==="element"||et==="hidden")?`[${en}]`:strings[en];out.push(`${type(parent[v])}:${name(parent[v]).slice(0,28).replace(/\n/g," ")}.${et}:${String(label).slice(0,24)}`);v=parent[v];}return out.reverse().join(" > ");};
const order=Array.from({length:count},(_,i)=>i).filter(i=>seen[i]&&type(i)!=="string").sort((a,b)=>self(b)-self(a)).slice(0,top);
console.log("biggest non-string nodes");
for(const i of order)console.log("  ",mb(self(i)),"MB",type(i),name(i).slice(0,30),"\n      ",chain(i));
const groups=new Map();
for(let i=0;i<count;i++){if(!seen[i]||type(i)!=="string")continue;const s=name(i);const k=s.length<200?"(short) "+(s.startsWith("/")?"path":/^[a-z]+\0/u.test(s)?"rowkey:"+s.slice(0,s.indexOf("\0")):/^[0-9a-f]{40,}$/u.test(s)?"hex":"other"):s.slice(0,48);const g=groups.get(k)??{bytes:0,n:0,sample:i};g.bytes+=self(i);g.n++;groups.set(k,g);}
console.log("biggest string groups (first 48 chars)");
for(const [k,g] of [...groups].sort((a,b)=>b[1].bytes-a[1].bytes).slice(0,top))console.log("  ",mb(g.bytes),"MB x",g.n,JSON.stringify(k),"\n      ",chain(g.sample));
