// 🧩 Merges the per-rule-group "after" graphs of apply-derivation into one: every target whose inputs differ from the base graph takes the derived inputs.
import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
const G=join(import.meta.dir,"..","🗑️generated","input-precision"),base=JSON.parse(readFileSync(join(G,"graph.json"),"utf8")),pristine=JSON.parse(readFileSync(join(G,"graph.json"),"utf8"));
const out=process.argv[2]!,parts=process.argv.slice(3);
const changed:Record<string,number>={};
for(const part of parts){
 const after=JSON.parse(readFileSync(join(G,part),"utf8"));
 for(const [key,project] of Object.entries<any>(after.projects))for(const [name,target] of Object.entries<any>(project.targets)){
  const original=base.projects[key].targets[name];
  if(JSON.stringify(target.inputs)!==JSON.stringify(pristine.projects[key].targets[name].inputs)){original.inputs=target.inputs;changed[`${project.name}:${name}`]=1;}
 }
}
writeFileSync(join(G,out),JSON.stringify(base));
console.log("merged",Object.keys(changed).length,"targets from",parts.length,"graphs");
