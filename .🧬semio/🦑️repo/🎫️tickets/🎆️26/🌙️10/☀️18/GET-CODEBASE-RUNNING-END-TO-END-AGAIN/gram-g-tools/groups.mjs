// [gram-g] heap snapshot: group the subgraphs hidden inside Map/Set storage (unreached from the GC roots) by the shape of their top objects. usage: bun groups.mjs <snapshot.json> [top=40]
import{readFileSync}from"node:fs";
const file=process.argv[2],top=Number(process.argv[3]??"40");
const snap=JSON.parse(readFileSync(file,"utf8")),{node_fields:nf,node_types:nt,edge_fields:ef,edge_types:et}=snap.snapshot.meta,N=nf.length,E=ef.length,nodes=snap.nodes,edges=snap.edges,strings=snap.strings;
const iType=nf.indexOf("type"),iName=nf.indexOf("name"),iSize=nf.indexOf("self_size"),iEdges=nf.indexOf("edge_count"),eType=ef.indexOf("type"),eName=ef.indexOf("name_or_index"),eTo=ef.indexOf("to_node");
const count=nodes.length/N,types=nt[0],etypes=et[0];
const first=new Int32Array(count+1);{let e=0;for(let i=0;i<count;i++){first[i]=e;e+=nodes[i*N+iEdges];}first[count]=e;}
const seen=new Uint8Array(count),order=new Int32Array(count),parent=new Int32Array(count).fill(-1);let tail=0;
const bfs=(starts)=>{let head=tail;for(const s of starts){if(!seen[s]){seen[s]=1;order[tail++]=s;}}while(head<tail){const n=order[head++];for(let e=first[n];e<first[n+1];e++){if(etypes[edges[e*E+eType]]==="weak")continue;const to=edges[e*E+eTo]/N;if(seen[to])continue;seen[to]=1;parent[to]=n;order[tail++]=to;}}};
bfs([0]);const reachedCount=tail;
const indeg=new Uint32Array(count);for(let n=0;n<count;n++){if(seen[n])continue;for(let e=first[n];e<first[n+1];e++){if(etypes[edges[e*E+eType]]==="weak")continue;indeg[edges[e*E+eTo]/N]++;}}
const roots=[];for(let n=0;n<count;n++)if(!seen[n]&&indeg[n]===0)roots.push(n);
// self-contained cycles: handled by a final sweep
const rootStart=tail;for(const r of roots){bfs([r]);}
const leftovers=[];for(let n=0;n<count;n++)if(!seen[n])leftovers.push(n);for(const n of leftovers)bfs([n]);
const sub=new Float64Array(count);for(let i=0;i<count;i++)sub[i]=nodes[i*N+iSize];
for(let k=tail-1;k>0;k--){const n=order[k];if(parent[n]>=0)sub[parent[n]]+=sub[n];}
const sig=n=>{const ty=types[nodes[n*N+iType]],nm=String(strings[nodes[n*N+iName]]);if(ty==="string"||ty==="concatenated string")return "string";let props=[];for(let e=first[n];e<first[n+1]&&props.length<7;e++){const t=etypes[edges[e*E+eType]];if(t==="property")props.push(strings[edges[e*E+eName]]);}
 let len=0;if(nm==="Array"){for(let e=first[n];e<first[n+1];e++)if(etypes[edges[e*E+eType]]==="element")len++;}
 return ty+":"+nm.slice(0,40)+(props.length?"{"+props.join(",")+"}":"")+(nm==="Array"?"["+(len>1000?"1000+":len>100?"100+":len>10?"10+":len)+"]":"");};
const groups=new Map();for(const r of roots.concat(leftovers.filter(n=>parent[n]===-1))){const k=sig(r),g=groups.get(k)??{n:0,size:0};g.n++;g.size+=sub[r];groups.set(k,g);}
const mb=x=>(x/1048576).toFixed(1).padStart(8);let all=0;for(const [,g] of groups)all+=g.size;
console.log("nodes",count,"reached from GC roots",reachedCount,"subgraph roots in Map/Set storage",roots.length,"MB in groups",mb(all));
for(const [k,g] of [...groups].sort((a,b)=>b[1].size-a[1].size).slice(0,top))console.log(mb(g.size),String(g.n).padStart(8),k.slice(0,160));
if(process.argv[3+1]==="memo"||process.env.MEMO==="1"){
 const sub2=new Map();
 const targetOf=(n,propName)=>{for(let e=first[n];e<first[n+1];e++){if(etypes[edges[e*E+eType]]==="property"&&strings[edges[e*E+eName]]===propName)return edges[e*E+eTo]/N;}return -1;};
 const elems=n=>{const out=[];for(let e=first[n];e<first[n+1];e++)if(etypes[edges[e*E+eType]]==="element")out.push(edges[e*E+eTo]/N);return out;};
 for(const r of roots){if(sig(r).indexOf("{observations,value}")<0)continue;const v=targetOf(r,"value");if(v<0)continue;const ty=types[nodes[v*N+iType]],nm=String(strings[nodes[v*N+iName]]);let key=ty+":"+nm.slice(0,20);
  if(nm==="Array"){const el=elems(v);const f=el[0];let sample="";if(f!==undefined){const ft=types[nodes[f*N+iType]];sample=ft.includes("string")?String(strings[nodes[f*N+iName]]).slice(0,40):ft+":"+String(strings[nodes[f*N+iName]]).slice(0,20);}key+="["+(el.length>1000?"1000+":el.length>300?"300+":el.length>50?"50+":el.length>5?"5+":el.length)+"] first="+sample;}
  else if(nm==="Object"){const ps=[];for(let e=first[v];e<first[v+1]&&ps.length<5;e++)if(etypes[edges[e*E+eType]]==="property")ps.push(strings[edges[e*E+eName]]);key+="{"+ps.join(",")+"}";}
  const g=sub2.get(key)??{n:0,size:0};g.n++;g.size+=sub[r];sub2.set(key,g);}
 console.log("--- memo entries by value shape");for(const [k,g] of [...sub2].sort((a,b)=>b[1].size-a[1].size).slice(0,25))console.log(mb(g.size),String(g.n).padStart(7),k.slice(0,170));
}
if(process.env.PROMISES==="1"){
 const ps=roots.filter(r=>types[nodes[r*N+iType]]==="object"&&String(strings[nodes[r*N+iName]])==="Promise").sort((a,b)=>sub[b]-sub[a]);
 console.log("--- promises",ps.length,"top sizes MB",ps.slice(0,5).map(r=>mb(sub[r]).trim()).join(","));
 const desc=(n,d)=>{let out=[];for(let e=first[n];e<first[n+1];e++){const t=etypes[edges[e*E+eType]],to=edges[e*E+eTo]/N;const nm=(t==="element"||t==="hidden")?"["+edges[e*E+eName]+"]":strings[edges[e*E+eName]];out.push(nm+"->"+types[nodes[to*N+iType]]+":"+String(strings[nodes[to*N+iName]]).slice(0,30)+"("+(nodes[to*N+iSize])+")");}return out.slice(0,12).join(" | ");};
 for(const r of ps.slice(0,6)){let o=-1;for(let e=first[r];e<first[r+1];e++){const to=edges[e*E+eTo]/N;if(types[nodes[to*N+iType]]==="object"&&String(strings[nodes[to*N+iName]])==="Object")o=to;}console.log(mb(sub[r]),o<0?"-":desc(o));}
 for(const r of ps.slice(3000,3002)){console.log(mb(sub[r]),desc(r));}
}
