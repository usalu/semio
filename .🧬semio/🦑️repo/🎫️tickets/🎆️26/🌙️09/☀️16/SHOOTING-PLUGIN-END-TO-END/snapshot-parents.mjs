import {readFileSync} from "node:fs";
const snap=JSON.parse(readFileSync(process.argv[2],"utf8")),{node_fields:nf,edge_fields:ef,node_types:nt,edge_types:et}=snap.snapshot.meta,N=nf.length,E=ef.length,nodes=snap.nodes,edges=snap.edges,strings=snap.strings;
const iType=nf.indexOf("type"),iName=nf.indexOf("name"),iSize=nf.indexOf("self_size"),iEC=nf.indexOf("edge_count"),eType=ef.indexOf("type"),eName=ef.indexOf("name_or_index"),eTo=ef.indexOf("to_node");
const types=nt[0],etypes=et[0],count=nodes.length/N;
const classify=s=>s.startsWith("/Users/ueli/Documents/semio/")?"abs-path":s.startsWith("build:")?"fact-key":/^[0-9a-f]{64}$/.test(s)?"sha256":s.length<40?"short":"other";
const kind=new Map();for(let n=0;n<nodes.length;n+=N)if(types[nodes[n+iType]]==="string")kind.set(n,classify(strings[nodes[n+iName]]));
// shape of each object node (first props)
const shapeOf=new Map(),agg=new Map();let off=0;
for(let n=0;n<nodes.length;n+=N){const ec=nodes[n+iEC],t=types[nodes[n+iType]];const props=[];
 for(let e=0;e<ec;e++){const o=off+e*E,to=edges[o+eTo];if(kind.has(to)){const k=kind.get(to),pname=etypes[edges[o+eType]]==="property"?strings[edges[o+eName]]:etypes[edges[o+eType]]==="element"?"[i]":etypes[edges[o+eType]];const key=k+" <- "+t+":"+strings[nodes[n+iName]].slice(0,20)+"."+pname;const r=agg.get(key)??[0,0];r[0]++;r[1]+=nodes[to+iSize];agg.set(key,r);}}
 off+=ec*E;}
console.log([...agg].sort((a,b)=>b[1][1]-a[1][1]).slice(0,30).map(([k,[c,s]])=>`${(s/1048576).toFixed(0).padStart(5)}MB ${String(c).padStart(8)} ${k}`).join("\n"));
