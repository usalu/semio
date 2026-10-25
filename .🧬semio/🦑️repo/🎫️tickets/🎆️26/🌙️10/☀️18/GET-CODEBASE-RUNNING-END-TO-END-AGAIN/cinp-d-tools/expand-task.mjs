import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const {buildTaskInputsV1}=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧮️task-model/🟦️.ts");
const g=".cache/b/g/m/generations/";
const cur=JSON.parse(readFileSync(g+"current.json","utf8"));
const man=JSON.parse(readFileSync(g+"m/"+cur.manifest+".json","utf8"));
const projects={},dependencies=[];
for(const [name,row] of Object.entries(man.owners)){
  if(row.kind!=="project")continue;
  let frag;try{frag=JSON.parse(brotliDecompressSync(readFileSync(g+"o/"+row.fragment+".json.br")).toString("utf8"));}catch{continue}
  projects[name]={name,root:frag.root,namedInputs:frag.node?.namedInputs,targets:frag.node?.targets??{}};
  for(const e of frag.edges??[])dependencies.push({source:e.source,target:e.target});
}
const policy=JSON.parse(readFileSync("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json","utf8")).namedInputs;
const graph={version:1,projects,dependencies,namedInputs:policy};
const [project,target]=process.argv.slice(2);
const def=projects[project].targets[target];
const entries=buildTaskInputsV1(graph,{project,definition:def});
const files=entries.filter(e=>e.kind==="file");
console.log(JSON.stringify({total:entries.length,positives:files.filter(f=>!f.exclude).map(f=>f.pattern),negatives:files.filter(f=>f.exclude).map(f=>f.pattern)}));
