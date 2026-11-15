import {readFileSync} from "node:fs";
const snap=JSON.parse(readFileSync(process.argv[2],"utf8")),{node_fields:nf,edge_fields:ef,node_types:nt,edge_types:et}=snap.snapshot.meta,N=nf.length,E=ef.length,nodes=snap.nodes,edges=snap.edges,strings=snap.strings;
const iType=nf.indexOf("type"),iName=nf.indexOf("name"),iSize=nf.indexOf("self_size"),iEC=nf.indexOf("edge_count"),eType=ef.indexOf("type"),eName=ef.indexOf("name_or_index"),eTo=ef.indexOf("to_node");
const types=nt[0],etypes=et[0];
const parent=new Map(); // node -> [srcNode, edgeLabel] first referrer that is not weak/internal
let off=0;const edgeStart=new Map();
for(let n=0;n<nodes.length;n+=N){const ec=nodes[n+iEC];edgeStart.set(n,off);for(let e=0;e<ec;e++){const o=off+e*E,t=etypes[edges[o+eType]];if(t==="weak")continue;const to=edges[o+eTo];if(!parent.has(to))parent.set(to,[n,t==="property"||t==="context"||t==="shortcut"?strings[edges[o+eName]]:t==="element"?"[i]":t]);}off+=ec*E;}
const label=n=>types[nodes[n+iType]]+":"+strings[nodes[n+iName]].slice(0,24);
const agg=new Map();
for(let n=0;n<nodes.length;n+=N){if(types[nodes[n+iType]]!=="string")continue;const s=strings[nodes[n+iName]];if(!s.startsWith("/Users/ueli/Documents/semio/"))continue;
 let cur=n,chain=[],guard=0;while(guard++<40){const p=parent.get(cur);if(!p)break;if(p[1]!=="[i]")chain.push(label(p[0])+"."+p[1]);cur=p[0];if(chain.length>=3)break;}
 const key=chain.join(" <- ")||"?";
 const r=agg.get(key)??[0,0];r[0]++;r[1]+=nodes[n+iSize];agg.set(key,r);}
console.log([...agg].sort((a,b)=>b[1][1]-a[1][1]).slice(0,14).map(([k,[c,s]])=>`${(s/1048576).toFixed(0).padStart(5)}MB ${String(c).padStart(8)} ${k}`).join("\n"));
