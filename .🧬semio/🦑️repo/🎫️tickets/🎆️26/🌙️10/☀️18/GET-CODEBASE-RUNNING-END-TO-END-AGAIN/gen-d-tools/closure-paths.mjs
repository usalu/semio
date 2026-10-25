import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const g=".cache/b/g/m/generations/";
const cur=JSON.parse(readFileSync(g+"current.json","utf8"));
const man=JSON.parse(readFileSync(g+"m/"+cur.manifest+".json","utf8"));
const projects={},edges={};
for(const [name,row] of Object.entries(man.owners)){
  if(row.kind!=="project")continue;
  let frag;try{frag=JSON.parse(brotliDecompressSync(readFileSync(g+"o/"+row.fragment+".json.br")).toString("utf8"));}catch{continue}
  projects[name]=frag;
  edges[name]=(frag.edges??[]).map(e=>e.target);
}
const [start,goal]=process.argv.slice(2);
const prev={[start]:null},q=[start];
while(q.length){const n=q.shift();if(n===goal)break;for(const t of edges[n]??[])if(!(t in prev)){prev[t]=n;q.push(t);}}
if(!(goal in prev)){console.log("no path");process.exit(0)}
const path=[];for(let n=goal;n;n=prev[n])path.push(n);
console.log(path.reverse().join("\n  -> "));
const sample=(projects[start].edges??[]).slice(0,3);console.log(JSON.stringify(sample));
