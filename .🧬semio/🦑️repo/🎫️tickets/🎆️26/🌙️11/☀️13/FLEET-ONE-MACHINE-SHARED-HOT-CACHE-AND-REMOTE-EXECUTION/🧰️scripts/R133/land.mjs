import {readFileSync,writeFileSync,mkdirSync,copyFileSync,existsSync} from "node:fs";
import {createHash} from "node:crypto";
import {dirname,join} from "node:path";
import {files,patchOnlyFiles,transform,FRAMEWORK} from "./seam.mjs";
const SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R133",BACKUP=join(SC,"backup"),LIB=join(FRAMEWORK,"🛍️products/🦑️repo/🔨️modules/📚️library");
const NEW=[["spelling",join(FRAMEWORK,"🔨️modules/🏃️process/🏗️build/🏠️machine-state/🧪️tests/🔒️spelling/🟦️.ts")],["plan-keys",join(LIB,"🏗️build/🏡️resident/🧪️tests/🔤️plan-keys/🟦️.ts")],["slice-read",join(LIB,"🏗️build/🕸️graph/🧪️tests/🔪️slice-read/🟦️.ts")],["stage-plan",join(LIB,"🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🧪️tests/🪜️stage-plan/🟦️.ts")]];
const sha=t=>createHash("sha256").update(t).digest("hex").slice(0,12);
mkdirSync(BACKUP,{recursive:true});
const manifest=[];
for(const f of [...files(),...patchOnlyFiles()]){const t=readFileSync(f,"utf8"),o=transform(f,t);if(o===t)continue;const copy=join(BACKUP,sha(f));writeFileSync(copy,t);manifest.push({file:f,backup:copy,before:sha(t),after:sha(o)});writeFileSync(f,o);}
for(const[name,target]of NEW){mkdirSync(dirname(target),{recursive:true});const source=join(SC,"new",name,"🟦️.ts");manifest.push({file:target,backup:null,before:null,after:sha(readFileSync(source,"utf8"))});copyFileSync(source,target);}
writeFileSync(join(BACKUP,"manifest.json"),JSON.stringify(manifest,null,1));
console.log("landed",manifest.length,"files");
