import {readdirSync,readFileSync,statSync} from "node:fs";
import {join} from "node:path";
import {stripTypeScriptTypes} from "node:module";
const skip=new Set(["node_modules","target","dist",".git",".cache","📦️packages"]);
const out=[];let n=0;
function walk(d){for(const e of readdirSync(d,{withFileTypes:true})){if(e.isDirectory()){if(skip.has(e.name)&&e.name!=="📦️packages")continue;if(e.name==="node_modules"||e.name==="target"||e.name==="dist")continue;walk(join(d,e.name));}else if(/\.(ts|mts)$/.test(e.name)&&!e.name.endsWith(".d.ts")){n++;const p=join(d,e.name);try{stripTypeScriptTypes(readFileSync(p,"utf8"),{mode:"strip"});}catch(err){out.push(p+" :: "+String(err.message).split("\n")[0]);}}}}
for(const r of process.argv.slice(2))walk(r);
console.log("scanned",n);for(const o of out)console.log(o);
