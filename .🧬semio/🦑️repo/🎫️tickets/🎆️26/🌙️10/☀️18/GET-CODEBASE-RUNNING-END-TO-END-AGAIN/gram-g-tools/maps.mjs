// [gram-g] heap snapshot: the Map/Set objects reachable from the GC roots, largest storage first, with the retainer path of each. usage: bun maps.mjs <snapshot.json> [top=25]
import{readFileSync}from"node:fs";
const file=process.argv[2],top=Number(process.argv[3]??"25");
const snap=JSON.parse(readFileSync(file,"utf8")),{node_fields:nf,node_types:nt,edge_fields:ef,edge_types:et}=snap.snapshot.meta,N=nf.length,E=ef.length,nodes=snap.nodes,edges=snap.edges,strings=snap.strings;
const iType=nf.indexOf("type"),iName=nf.indexOf("name"),iSize=nf.indexOf("self_size"),iEdges=nf.indexOf("edge_count"),eType=ef.indexOf("type"),eName=ef.indexOf("name_or_index"),eTo=ef.indexOf("to_node");
const count=nodes.length/N,types=nt[0],etypes=et[0];
const first=new Int32Array(count+1);{let e=0;for(let i=0;i<count;i++){first[i]=e;e+=nodes[i*N+iEdges];}first[count]=e;}
const parent=new Int32Array(count).fill(-1),pedge=new Int32Array(count).fill(-1),seen=new Uint8Array(count),order=new Int32Array(count);let head=0,tail=0;seen[0]=1;order[tail++]=0;
while(head<tail){const n=order[head++];for(let e=first[n];e<first[n+1];e++){if(etypes[edges[e*E+eType]]==="weak")continue;const to=edges[e*E+eTo]/N;if(seen[to])continue;seen[to]=1;parent[to]=n;pedge[to]=e;order[tail++]=to;}}
const label=n=>{const e=pedge[n];if(e<0)return "(root)";const t=etypes[edges[e*E+eType]],nm=(t==="element"||t==="hidden")?"["+edges[e*E+eName]+"]":strings[edges[e*E+eName]];return nm;};
const path=n=>{const out=[];for(let c=n,d=0;c>0&&d<14;c=parent[c],d++)out.push(label(c)+":"+String(strings[nodes[c*N+iName]]).slice(0,22));return out.reverse().join(" > ");};
const rows=[];
for(let n=0;n<count;n++){if(!seen[n])continue;const nm=String(strings[nodes[n*N+iName]]);if(nm!=="Map"&&nm!=="Set")continue;let store=0;for(let e=first[n];e<first[n+1];e++){const to=edges[e*E+eTo]/N;if(types[nodes[to*N+iType]]==="hidden")store+=nodes[to*N+iSize];}rows.push([store,n,nm]);}
rows.sort((a,b)=>b[0]-a[0]);console.log("maps/sets reached",rows.length);
for(const [store,n,nm] of rows.slice(0,top))console.log((store/1048576).toFixed(1).padStart(7)+" MB "+nm+" "+path(n).slice(0,400));
