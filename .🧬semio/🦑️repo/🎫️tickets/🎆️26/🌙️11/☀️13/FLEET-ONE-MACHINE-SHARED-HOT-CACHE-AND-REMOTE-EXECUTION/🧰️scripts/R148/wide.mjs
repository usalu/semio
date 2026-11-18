import {readFileSync,readdirSync,statSync} from "node:fs";import {createRequire} from "node:module";import {join} from "node:path";
const lib="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const ts=createRequire(lib+"🟨️.mjs")("typescript");
const {scanModuleSpecifiersV1}=await import(lib+"🏗️build/🧮️task-model/🔎️module-references/🟦️.ts");
const files=[];const walk=(d)=>{for(const e of readdirSync(d,{withFileTypes:true})){if(e.name==="node_modules"||e.name===".git"||e.name===".cache"||e.name==="dist"||e.name==="target")continue;const p=join(d,e.name);if(e.isDirectory())walk(p);else if(/\.(ts|tsx|mjs|js|cjs|mts)$/.test(e.name)&&statSync(p).size<400000)files.push(p);}};
walk(process.argv[2]);
let bad=0;for(const f of files){const s=readFileSync(f,"utf8");const a=JSON.stringify(ts.preProcessFile(s,true,true).importedFiles.map(x=>x.fileName));const b=JSON.stringify(scanModuleSpecifiersV1(s));if(a!==b){bad++;if(bad<5)console.log("DIFF",f,a.slice(0,200),b.slice(0,200));}}
console.log(process.argv[2],"files",files.length,"diffs",bad);
