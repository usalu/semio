/** 🔬️ Runs the compiler capture's built-in read rewrite over recently changed library sources and names each one it refuses. */
import {readFileSync,statSync} from "node:fs";
import {redirectCapturedBuiltinReadsV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🍞️bun/🟦️.ts";
const targets=new Map([["node:fs","./a"],["fs","./a"],["node:fs/promises","./b"],["fs/promises","./b"],["node:module","./c"],["module","./c"]]);
const js=new Bun.Transpiler({loader:"js",target:"bun"}),hours=Number(process.argv[2]??6),since=Date.now()-hours*3600e3;
const files=new Bun.Glob("🧰️framework/🛍️products/🦑️repo/**/*.{ts,mjs,js}").scanSync({dot:false});
let checked=0;
for(const file of files){
  if(file.includes("node_modules"))continue;
  try{if(statSync(file).mtimeMs<since)continue;}catch{continue;}
  checked++;
  const raw=readFileSync(file,"utf8"),source=file.endsWith(".ts")?new Bun.Transpiler({loader:"ts",target:"bun"}).transformSync(raw):raw;
  try{redirectCapturedBuiltinReadsV1(source,false,js,targets,(code)=>Object.assign(new Error(code),{code}));}catch(error){console.log("REFUSED",(error as Error).message,file);}
}
console.log("checked",checked);
