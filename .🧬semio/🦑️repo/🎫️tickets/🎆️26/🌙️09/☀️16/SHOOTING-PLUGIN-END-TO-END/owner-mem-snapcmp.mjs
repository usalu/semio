import {readFileSync} from "node:fs";
const MiB=1048576;
const load=file=>{const snap=JSON.parse(readFileSync(file,"utf8")),{node_fields:nf,edge_fields:ef,node_types:nt,edge_types:et}=snap.snapshot.meta,N=nf.length,E=ef.length,nodes=snap.nodes,edges=snap.edges,strings=snap.strings;
const iType=nf.indexOf("type"),iName=nf.indexOf("name"),iSize=nf.indexOf("self_size"),iEC=nf.indexOf("edge_count"),eType=ef.indexOf("type"),eName=ef.indexOf("name_or_index"),eTo=ef.indexOf("to_node");
const types=nt[0],etypes=et[0];const parent=new Map();let off=0;
for(let n=0;n<nodes.length;n+=N){const ec=nodes[n+iEC];for(let e=0;e<ec;e++){const o=off+e*E,t=etypes[edges[o+eType]];if(t==="weak")continue;const to=edges[o+eTo];if(!parent.has(to))parent.set(to,[n,t==="property"||t==="context"||t==="shortcut"?strings[edges[o+eName]]:t==="element"?"[i]":t]);}off+=ec*E;}
const label=n=>types[nodes[n+iType]]+":"+strings[nodes[n+iName]].slice(0,22);
const agg=new Map();
for(let n=0;n<nodes.length;n+=N){const size=nodes[n+iSize];let cur=n,chain=[],guard=0;while(guard++<80){const p=parent.get(cur);if(!p)break;if(p[1]!=="[i]")chain.push(p[1]);cur=p[0];if(chain.length>=5)break;}
 const root=cur===n?"?":label(cur);const key=chain.slice(-4).reverse().join(".")+" @ "+root;const r=agg.get(key)??[0,0];r[0]++;r[1]+=size;agg.set(key,r);}
return agg;};
const a=load(process.argv[2]),b=load(process.argv[3]);
const rows=[];for(const [k,[c,s]] of b){const [c0=0,s0=0]=a.get(k)??[];rows.push([k,c-c0,s-s0]);}
console.log(rows.sort((x,y)=>y[2]-x[2]).slice(0,30).map(([k,c,s])=>`${(s/MiB).toFixed(1).padStart(7)}MB ${String(c).padStart(8)} ${k}`).join("\n"));
