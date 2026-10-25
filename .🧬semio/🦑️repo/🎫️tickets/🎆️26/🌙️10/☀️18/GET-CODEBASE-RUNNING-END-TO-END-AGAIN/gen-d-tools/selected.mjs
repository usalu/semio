// usage: bun selected.mjs <project> <target> <out.json> [generationDir]  -> {inputs,total,files:[...],env:[...]}
import {readFileSync,writeFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
import {execFileSync} from "node:child_process";
const root="/Users/ueli/Documents/semio/";
const {buildTaskInputsV1}=await import(root+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧮️task-model/🟦️.ts");
const {bazelTaskSourcePathsV1}=await import(root+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts");
const g=root+".cache/b/g/m/generations/";
const cur=JSON.parse(readFileSync(g+"current.json","utf8"));
const man=JSON.parse(readFileSync(g+"m/"+cur.manifest+".json","utf8"));
const projects={},dependencies=[];
for(const [name,row] of Object.entries(man.owners)){
  if(row.kind!=="project")continue;
  let frag;try{frag=JSON.parse(brotliDecompressSync(readFileSync(g+"o/"+row.fragment+".json.br")).toString("utf8"));}catch{continue}
  projects[name]={name,root:frag.root,namedInputs:frag.node?.namedInputs,targets:frag.node?.targets??{}};
  for(const e of frag.edges??[])dependencies.push({source:e.source,target:e.target});
}
const policy=JSON.parse(readFileSync(root+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json","utf8")).namedInputs;
const graph={version:1,projects,dependencies,namedInputs:policy};
const [project,target,out]=process.argv.slice(2);
const def=projects[project].targets[target];
const entries=buildTaskInputsV1(graph,{project,definition:def});
const patterns=entries.filter(e=>e.kind==="file").map(e=>(e.exclude?"!":"")+e.pattern);
const inventory=execFileSync("git",["ls-files","-z","--cached","--others","--exclude-standard"],{cwd:root,maxBuffer:1<<30}).toString("utf8").split("\0").filter(Boolean);
const files=bazelTaskSourcePathsV1(patterns,inventory);
writeFileSync(out,JSON.stringify({generation:cur.generation,total:entries.length,patterns:patterns.length,files,metadata:def.metadata?.semio,env:entries.filter(e=>e.kind!=="file").map(e=>JSON.stringify(e)).slice(0,200)}));
console.log(project,target,"entries",entries.length,"patterns",patterns.length,"files",files.length,"precision",JSON.stringify(def.metadata?.semio?.inputPrecision));
