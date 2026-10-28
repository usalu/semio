import {pathToFileURL} from "node:url";
import {readFileSync} from "node:fs";
const lib=process.cwd()+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const s=await import(pathToFileURL(lib+"🏗️build/📥️source/🔏️closure/🗓️scheduling/🟦️.ts").href),e=await import(pathToFileURL(lib+"⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🟦️.ts").href);
const files=readFileSync(process.argv[2],"utf8").split("\n").filter(Boolean),v=s.buildSchedulingV1(),p=e.nxEnginePolicyV1(),counts={};
for(const f of files){const d=p.domains.find(x=>x.prefixes.some(q=>f.startsWith(q)))?.id??s.buildSchedulingDomainOfV1(f,v)??"keyed";counts[d]=(counts[d]??0)+1;}
console.log(files.length,JSON.stringify(counts));
