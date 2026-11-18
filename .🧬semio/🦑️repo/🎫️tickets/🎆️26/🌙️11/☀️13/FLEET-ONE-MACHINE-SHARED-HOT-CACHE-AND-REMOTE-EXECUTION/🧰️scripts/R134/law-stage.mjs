import {readFileSync,writeFileSync,mkdirSync} from "node:fs";
import {SC,L,HERE,PATCHES} from "./patches.mjs";
const MIRROR=SC+(process.env.R134_BASE==="1"?"mirror-base/":"mirror/");
const growth=PATCHES.find(p=>p.file.endsWith("📈️growth/🟦️.ts")).edits.find(e=>e.append!==undefined).append;
const head=`import{expect,test}from"bun:test";\nimport{readFileSync,readdirSync,existsSync}from"node:fs";\nimport{fileURLToPath}from"node:url";\n`+(process.env.R134_BASE==="1"?`const scaleDeclaration=JSON.parse(readFileSync(${JSON.stringify(HERE+"new/workspace-scale.json")},"utf8"));\n`:`const scaleDeclaration=JSON.parse(readFileSync(${JSON.stringify(MIRROR+"🏗️build/📏️workspace-scale/🧬️schema/🔣️.json")},"utf8"));\n`);
let body=growth.replace('const scaleLibrary=fileURLToPath(new URL("../../../../../../",import.meta.url));',`const scaleLibrary=${JSON.stringify(L)},scaleOverlay=${JSON.stringify(MIRROR)};`).replace('const scaleRead=(file:string)=>JSON.parse(readFileSync(scaleLibrary+file,"utf8"));','const scaleRead=(file:string)=>JSON.parse(readFileSync((existsSync(scaleOverlay+file)?scaleOverlay:scaleLibrary)+file,"utf8"));');
mkdirSync(SC+"law",{recursive:true});writeFileSync(SC+"law/law.test.ts",head+body);console.log("law staged");
