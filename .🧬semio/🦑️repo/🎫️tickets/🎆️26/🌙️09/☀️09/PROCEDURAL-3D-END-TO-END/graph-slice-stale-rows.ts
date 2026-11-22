import {readFileSync} from "node:fs";
const obs=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs");
const rows=JSON.parse(readFileSync(process.argv[2],"utf8")).observations as {kind:string;path:string}[];
const stale:Record<string,number>={};const byDir:Record<string,number>={};
for(const row of rows){if(!["readdir","file","analysis","lstat"].includes(row.kind))continue;
 if(!await obs.observationsHold([row],1,new Set(),new Set())){stale[row.kind]=(stale[row.kind]??0)+1;const key=row.path.split("/").slice(0,8).join("/");byDir[key]=(byDir[key]??0)+1;}}
console.log(stale);console.log(Object.entries(byDir).sort((a,b)=>b[1]-a[1]).slice(0,15));
