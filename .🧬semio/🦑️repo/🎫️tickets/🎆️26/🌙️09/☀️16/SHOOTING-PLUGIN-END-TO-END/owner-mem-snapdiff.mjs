import {readFileSync} from "node:fs";
const snap=JSON.parse(readFileSync(process.argv[2],"utf8")),{node_fields:nf,edge_fields:ef,node_types:nt,edge_types:et}=snap.snapshot.meta,N=nf.length,E=ef.length,nodes=snap.nodes,edges=snap.edges,strings=snap.strings;
const iType=nf.indexOf("type"),iName=nf.indexOf("name"),iSize=nf.indexOf("self_size"),iEC=nf.indexOf("edge_count"),eType=ef.indexOf("type"),eName=ef.indexOf("name_or_index"),eTo=ef.indexOf("to_node");
const types=nt[0],etypes=et[0],MiB=1048576;
const owner=new Map();let off=0;const starts=[];
for(let n=0;n<nodes.length;n+=N){starts.push(off);off+=nodes[n+iEC]*E;}
const parent=new Map(),nodeIndex=n=>n/N;
off=0;
for(let n=0;n<nodes.length;n+=N){const ec=nodes[n+iEC];for(let e=0;e<ec;e++){const o=off+e*E,t=etypes[edges[o+eType]];if(t==="weak")continue;const to=edges[o+eTo];if(!parent.has(to)||t==="property"&&!String(parent.get(to)[1]).startsWith("p:"))parent.set(to,[n,(t==="property"||t==="context"||t==="shortcut"?"p:"+strings[edges[o+eName]]:t==="element"?"[i]":t)]);}off+=ec*E;}
const label=n=>types[nodes[n+iType]]+":"+strings[nodes[n+iName]].slice(0,28);
const agg=new Map();
for(let n=0;n<nodes.length;n+=N){const t=types[nodes[n+iType]];if(t!=="string"&&t!=="concatenated string"&&t!=="object"&&t!=="array")continue;const size=nodes[n+iSize];
 let cur=n,chain=[],guard=0;while(guard++<60){const p=parent.get(cur);if(!p)break;if(p[1]!=="[i]"){chain.push(label(p[0])+"."+p[1].replace("p:",""));}cur=p[0];if(chain.length>=4)break;}
 const key=(t==="string"?"S":t[0])+" <- "+(chain.join(" <- ")||"?");const r=agg.get(key)??[0,0];r[0]++;r[1]+=size;agg.set(key,r);}
console.log([...agg].sort((a,b)=>b[1][1]-a[1][1]).slice(0,28).map(([k,[c,s]])=>`${(s/MiB).toFixed(1).padStart(7)}MB ${String(c).padStart(8)} ${k}`).join("\n"));
