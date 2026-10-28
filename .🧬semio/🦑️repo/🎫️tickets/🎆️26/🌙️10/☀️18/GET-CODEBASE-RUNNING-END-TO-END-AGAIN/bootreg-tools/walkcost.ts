/** 🧪 Splits the cost of the Cargo discovery walk on a root: readdir only, readdir + one lstat per directory, readdir + the per-operation ancestor lstat chain. `bun walkcost.ts <root>` */
import {readdirSync,lstatSync} from "node:fs";import {join} from "node:path";
const root=process.argv[2]!,opaque=new Set(["node_modules","target","dist","build","🤖️generated","🗑️generated","coverage","temp","compose"]);
function walk(mode:number){let dirs=0,lst=0;const stack=[""];while(stack.length){const d=stack.pop()!;dirs++;
 if(mode===1){lstatSync(join(root,d));lst++;}else if(mode===2){let cur=root;lstatSync(cur);lst++;for(const p of d.split("/").filter(Boolean)){cur=join(cur,p);lstatSync(cur);lst++;}}
 for(const e of readdirSync(join(root,d),{withFileTypes:true})){if(e.name.startsWith(".")||opaque.has(e.name)||e.isSymbolicLink())continue;if(e.isDirectory())stack.push(d?d+"/"+e.name:e.name);}}return{dirs,lst};}
for(const mode of [0,1,2,0]){const a=performance.now(),r=walk(mode);console.log(`[DEBUG] mode ${mode} dirs ${r.dirs} lstat ${r.lst} ${(performance.now()-a).toFixed(0)} ms`);}
