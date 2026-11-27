import {readFileSync} from "node:fs";
const text=readFileSync(process.argv[2],"utf8"),data=JSON.parse(text),{node_fields:nf,node_types:nt}=data.snapshot.meta,stride=nf.length,types=nt[0],nodes=data.nodes,strings=data.strings;
const by=new Map();let total=0;
for(let i=0;i<nodes.length;i+=stride){const type=types[nodes[i]],name=strings[nodes[i+1]],size=nodes[i+3];total+=size;const key=type+":"+(type==="string"||type==="concatenated string"||type==="sliced string"?"(strings)":name.slice(0,60));const row=by.get(key)??[0,0];row[0]++;row[1]+=size;by.set(key,row);}
console.log("total MB",Math.round(total/1048576),"nodes",nodes.length/stride);
for(const[k,[c,s]]of[...by].sort((a,b)=>b[1][1]-a[1][1]).slice(0,Number(process.argv[3]??25)))console.log(String(Math.round(s/1048576*10)/10).padStart(7),"MB",String(c).padStart(9),k);
