import {readFileSync} from "node:fs";
import {files,transform,FRAMEWORK} from "./seam.mjs";
const SPELLING=/(?<![\w$.])(?:\w+\.)?(?:join|resolve)\(\s*([^,()]*(?:\([^()]*\))?[^,()]*),\s*["']\.(?:cache|nx)(?:["'/])/gu,EX=new Set(["home","homedir()","fixture","match[1]!","at"]);
let before=0,after=0;const rest=[];
for(const f of files()){const t=readFileSync(f,"utf8"),o=transform(f,t);for(const m of t.matchAll(SPELLING))if(!EX.has(m[1].trim()))before++;for(const m of o.matchAll(SPELLING))if(!EX.has(m[1].trim())){after++;rest.push(f.replace(FRAMEWORK,"")+" :: "+m[0].slice(0,80));}}
console.log({before,after});console.log(rest.join("\n"));
