import {pathToFileURL} from "node:url";
import {createReadStream} from "node:fs";
import {createInterface} from "node:readline";
const lib=process.cwd()+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const s=await import(pathToFileURL(lib+"🏗️build/📥️source/🔏️closure/🗓️scheduling/🟦️.ts").href);
const v=s.buildSchedulingV1();const counts={};let total=0;
for await(const line of createInterface({input:createReadStream(process.argv[2])})){total++;const d=s.buildSchedulingDomainOfV1(line,v)??"keyed";counts[d]=(counts[d]??0)+1;}
console.log(total,counts);
