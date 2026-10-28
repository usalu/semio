import {pathToFileURL} from "node:url";
import {readFileSync,existsSync,lstatSync} from "node:fs";
import {resolve,dirname,relative} from "node:path";
const root=process.cwd(),lib=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const m=await import(pathToFileURL(lib+"🟨️.mjs").href);
const short=p=>p.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/","LIB/").replace("🧰️framework/🔨️modules/","FM/");
for(const f of m.nxBuildEngineClosureV1().files){if(f.endsWith(".json"))continue;const abs=resolve(root,f);let refs=[];try{refs=m.nxBuildEngineReferencesV1(readFileSync(abs,"utf8"));}catch{continue}
 for(const r of refs)if(r.kind==="file-url"){const t=resolve(dirname(abs),r.specifier);if(!existsSync(t)||!lstatSync(t).isDirectory())console.log(short(f),"->",r.specifier);}}
