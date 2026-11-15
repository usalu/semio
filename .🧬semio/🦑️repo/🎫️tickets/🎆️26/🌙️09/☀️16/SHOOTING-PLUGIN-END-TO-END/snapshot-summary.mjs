import {readFileSync} from "node:fs";
const snap=JSON.parse(readFileSync(process.argv[2],"utf8")),{node_fields:nf,edge_fields:ef,node_types:nt,edge_types:et}=snap.snapshot.meta,N=nf.length,E=ef.length,nodes=snap.nodes,edges=snap.edges,strings=snap.strings;
const iType=nf.indexOf("type"),iName=nf.indexOf("name"),iSize=nf.indexOf("self_size"),iEC=nf.indexOf("edge_count"),eType=ef.indexOf("type"),eName=ef.indexOf("name_or_index");
const types=nt[0],etypes=et[0],byType=new Map(),byShape=new Map();let off=0;
const add=(m,k,s)=>{const r=m.get(k)??[0,0];r[0]++;r[1]+=s;m.set(k,r);};
for(let n=0;n<nodes.length;n+=N){
 const t=types[nodes[n+iType]],name=strings[nodes[n+iName]],size=nodes[n+iSize],ec=nodes[n+iEC];
 add(byType,t+":"+(t==="string"||t==="concatenated string"?"":name.slice(0,60)),size);
 if(t==="object"&&name==="Object"){const props=[];for(let e=0;e<ec&&props.length<6;e++){const o=off+e*E;if(etypes[edges[o+eType]]==="property")props.push(strings[edges[o+eName]]);}add(byShape,props.join(","),size);}
 off+=ec*E;
}
const show=(m,k)=>[...m].sort((a,b)=>b[1][1]-a[1][1]).slice(0,k).map(([key,[c,s]])=>`${(s/1048576).toFixed(0).padStart(6)}MB ${String(c).padStart(9)}  ${key}`).join("\n");
console.log("== by type/name\n"+show(byType,25)+"\n== Object shapes\n"+show(byShape,25));
