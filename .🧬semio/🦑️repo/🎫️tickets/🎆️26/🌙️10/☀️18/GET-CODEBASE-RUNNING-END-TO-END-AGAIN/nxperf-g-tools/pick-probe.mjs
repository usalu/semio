import {pathToFileURL} from "node:url";
import {readFileSync} from "node:fs";
const lib=process.cwd()+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const s=await import(pathToFileURL(lib+"🏗️build/📥️source/🔏️closure/🗓️scheduling/🟦️.ts").href);
const files=readFileSync(process.argv[2],"utf8").split("\n").filter(Boolean);
for(const d of ["leases","capacity","process-supervision","retry-observation","progress-reporting"]){const f=files.find(x=>s.buildSchedulingDomainOfV1(x)===d&&x.endsWith(".ts"));console.log(d,f);}
