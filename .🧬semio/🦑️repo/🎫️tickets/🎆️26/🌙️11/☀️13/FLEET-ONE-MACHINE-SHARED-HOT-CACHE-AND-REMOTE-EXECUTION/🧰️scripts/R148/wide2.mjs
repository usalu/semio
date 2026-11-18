import {readFileSync,readdirSync,statSync} from "node:fs";import {join} from "node:path";
const lib="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const {classifyModuleReferencesV1}=await import(lib+"🏗️build/🧮️task-model/🔎️module-references/🟦️.ts");
const old=await import(lib+"🟨️.mjs");
const files=[];const walk=(d)=>{for(const e of readdirSync(d,{withFileTypes:true})){if(["node_modules",".git",".cache","dist","target"].includes(e.name))continue;const p=join(d,e.name);if(e.isDirectory())walk(p);else if(/\.(ts|tsx|mjs|js|cjs|mts)$/.test(e.name)&&statSync(p).size<400000)files.push(p);}};
walk(process.argv[2]);let bad=0,withUrl=0,withRows=0;
for(const f of files){const s=readFileSync(f,"utf8");if(!s.includes("URL"))continue;withUrl++;const a=JSON.stringify(old.nxBuildEngineReferencesV1(s)),b=JSON.stringify(classifyModuleReferencesV1(s));if(/file-url|directory-url/.test(a))withRows++;if(a!==b){bad++;if(bad<6)console.log("DIFF",f,a.slice(0,300),b.slice(0,300));}}
console.log(process.argv[2],"withURL",withUrl,"withUrlRows",withRows,"diffs",bad);
