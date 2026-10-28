/** 🔎️ Parse-checks files the way the runtime will: Bun transpiles (keeping imports), then `node --check` parses the ESM output, which rejects duplicate bindings that `bun build --no-bundle` and `Transpiler.scan` let through: bun scan.ts <list file of repo-relative paths> <scratch dir> */
import {mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
const [listFile,scratch]=process.argv.slice(2) as [string,string];mkdirSync(scratch,{recursive:true});
const list=readFileSync(listFile,"utf8").split("\n").filter(Boolean);
let bad=0,n=0;
for(const file of list){
 if(/\.d\.m?ts$/u.test(file))continue;
 const loader=file.endsWith(".tsx")?"tsx":/\.m?ts$/u.test(file)?"ts":"js";
 let out:string;try{out=new Bun.Transpiler({loader,trimUnusedImports:false}).transformSync(readFileSync(file,"utf8"));}catch(error){bad++;console.log("[DEBUG] TRANSPILE",file,String(error).slice(0,160));continue;}
 const target=join(scratch,`f${n++}.mjs`);writeFileSync(target,out);
 const check=spawnSync("node",["--check",target],{encoding:"utf8"});
 if(check.status!==0){const line=(check.stderr.split("\n").find(l=>l.includes("Error"))??"").slice(0,160);if(!/await is only valid|Cannot use import statement/u.test(line)){bad++;console.log("[DEBUG] PARSE",file,line);}}
}
console.log("[DEBUG] scanned",list.length,"bad",bad);
