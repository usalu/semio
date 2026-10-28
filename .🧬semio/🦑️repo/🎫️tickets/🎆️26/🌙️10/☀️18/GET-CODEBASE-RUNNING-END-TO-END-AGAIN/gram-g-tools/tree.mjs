// [gram-g] heap snapshot BFS-tree attribution: subtree self-size sums along the first-found retainer path. usage: bun tree.mjs <snapshot.json> [minMB=8] [maxDepth=12]
import{readFileSync}from"node:fs";
const file=process.argv[2],minBytes=Number(process.argv[3]??"8")*1048576,maxDepth=Number(process.argv[4]??"12");
const snap=JSON.parse(readFileSync(file,"utf8")),{node_fields:nf,node_types:nt,edge_fields:ef,edge_types:et}=snap.snapshot.meta,N=nf.length,E=ef.length,nodes=snap.nodes,edges=snap.edges,strings=snap.strings;
const iType=nf.indexOf("type"),iName=nf.indexOf("name"),iSize=nf.indexOf("self_size"),iEdges=nf.indexOf("edge_count"),eType=ef.indexOf("type"),eName=ef.indexOf("name_or_index"),eTo=ef.indexOf("to_node");
const count=nodes.length/N,types=nt[0],etypes=et[0];
const first=new Int32Array(count+1);{let e=0;for(let i=0;i<count;i++){first[i]=e;e+=nodes[i*N+iEdges];}first[count]=e;}
const parent=new Int32Array(count).fill(-1),pedge=new Int32Array(count).fill(-1),depth=new Int32Array(count),order=new Int32Array(count);let head=0,tail=0;
const seen=new Uint8Array(count);seen[0]=1;order[tail++]=0;
while(head<tail){const n=order[head++];for(let e=first[n];e<first[n+1];e++){const t=etypes[edges[e*E+eType]];if(t==="weak")continue;const to=edges[e*E+eTo]/N;if(seen[to])continue;seen[to]=1;parent[to]=n;pedge[to]=e;depth[to]=depth[n]+1;order[tail++]=to;}}
const sub=new Float64Array(count);for(let i=0;i<count;i++)sub[i]=nodes[i*N+iSize];
for(let k=tail-1;k>0;k--){const n=order[k];sub[parent[n]]+=sub[n];}
const label=n=>{const e=pedge[n];const t=etypes[edges[e*E+eType]],nm=(t==="element"||t==="hidden")?"["+edges[e*E+eName]+"]":strings[edges[e*E+eName]];const ty=types[nodes[n*N+iType]],name=strings[nodes[n*N+iName]];return nm+" -> "+ty+":"+String(name).slice(0,50);};
const kids=new Map();for(let k=1;k<tail;k++){const n=order[k];if(sub[n]>=minBytes){const p=parent[n];(kids.get(p)??kids.set(p,[]).get(p)).push(n);}}
const mb=x=>(x/1048576).toFixed(1).padStart(8);
const print=(n,d)=>{for(const c of (kids.get(n)??[]).sort((a,b)=>sub[b]-sub[a])){console.log(mb(sub[c])+" "+" ".repeat(d)+label(c));if(d<maxDepth)print(c,d+1);}};
console.log("total",mb(sub[0]),"MB reached",tail,"of",count);
{let un=0,unSize=0;const byType=new Map();for(let i=0;i<count;i++)if(!seen[i]){un++;const sz=nodes[i*N+iSize];unSize+=sz;const k=types[nodes[i*N+iType]]+":"+(types[nodes[i*N+iType]]==="string"?"":String(strings[nodes[i*N+iName]]).slice(0,40));const e=byType.get(k)??{n:0,s:0};e.n++;e.s+=sz;byType.set(k,e);}
console.log("unreached",un,"MB",mb(unSize));for(const [k,v] of [...byType].sort((a,b)=>b[1].s-a[1].s).slice(0,8))console.log(mb(v.s),v.n,k);}
print(0,0);
