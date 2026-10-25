// usage: bun compose-selected.mjs <project> <target> <derived.json> <out.json>
// Emulates preciseInputsV1 on the LIVE fragment: drops the replaced groups, adds the derived patterns/groups/files, selects the files the Bazel adapter would declare.
import {readFileSync,writeFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
import {execFileSync} from "node:child_process";
const root="/Users/ueli/Documents/semio/";
const L=root+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const {buildTaskInputsV1}=await import(L+"🏗️build/🧮️task-model/🟦️.ts");
const {bazelTaskSourcePathsV1}=await import(L+"🏗️build/🔧️tools/🏗️bazel/📜️script.ts");
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
const policy=JSON.parse(readFileSync(L+"🏗️build/🔣️policy.json","utf8")).namedInputs;
const [project,target,derivedPath,out]=process.argv.slice(2);
const derived=JSON.parse(readFileSync(derivedPath,"utf8"));
const def=structuredClone(projects[project].targets[target]);
const kept=def.inputs.filter(i=>!(typeof i==="string"&&derived.drops.includes(i)&&true)&&i!=="testSourceExclusions");
const exact=derived.files.map(f=>`{workspaceRoot}/${f}`);
const node=projects[project];
node.namedInputs={...node.namedInputs,__derivedFiles:exact};
def.inputs=[...kept,...derived.patterns,...derived.groups,"__derivedFiles"];
const graph={version:1,projects,dependencies,namedInputs:policy};
const entries=buildTaskInputsV1(graph,{project,definition:def});
const patterns=entries.filter(e=>e.kind==="file").map(e=>(e.exclude?"!":"")+e.pattern);
const inventory=execFileSync("git",["ls-files","-z","--cached","--others","--exclude-standard"],{cwd:root,maxBuffer:1<<30}).toString("utf8").split("\0").filter(Boolean);
const files=bazelTaskSourcePathsV1(patterns,inventory);
writeFileSync(out,JSON.stringify({files,kept:kept.filter(i=>typeof i==="string")}));
console.log(project,target,"patterns",patterns.length,"files",files.length);
