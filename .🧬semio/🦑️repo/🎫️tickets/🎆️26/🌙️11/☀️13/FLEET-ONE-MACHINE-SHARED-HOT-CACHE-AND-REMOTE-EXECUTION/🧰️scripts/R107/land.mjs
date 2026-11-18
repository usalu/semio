import{readFileSync,writeFileSync,mkdirSync,copyFileSync,renameSync,existsSync}from"node:fs";import{createHash}from"node:crypto";import{dirname}from"node:path";import{apply,all}from"./patches.mjs";
const R="/Users/ueli/Documents/semio/",BK="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R107/backup/",names=(process.argv[2]??"0,1,2").split(","),dry=process.argv.includes("--dry"),here=import.meta.dir+"/";
const B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/";
const tests={[B+"🧪️tests/🪦️stale-take/🟦️.ts"]:"stale-take.test.ts",[B+"📦️project-nodes/🧪️tests/🧮️known-unit/🟦️.ts"]:"known-unit.test.ts",[B+"🧭️script-inputs/💾️entries/🧪️tests/🧰️members-set/🟦️.ts"]:"members-set.test.ts"};
const sha=p=>createHash("sha256").update(readFileSync(p)).digest("hex").slice(0,12);
const chosen=all.filter((_,i)=>names.includes(String(i))),plan=new Map();
for(const p of chosen)plan.set(p.file,apply(p.file,readFileSync(R+p.file,"utf8"),[p]));
if(dry){for(const[k,v]of plan)console.log("ok",k.slice(-50),v.length);process.exit(0);}
import{rmSync}from"node:fs";rmSync(BK,{recursive:true,force:true});const before={};for(const k of plan.keys()){before[k]=sha(R+k);mkdirSync(dirname(BK+k),{recursive:true});copyFileSync(R+k,BK+k);}
for(const[k,v]of plan){writeFileSync(R+k+".r107tmp",v);renameSync(R+k+".r107tmp",R+k);}
const wanted={0:B+"🧪️tests/🪦️stale-take/🟦️.ts",1:B+"📦️project-nodes/🧪️tests/🧮️known-unit/🟦️.ts",2:B+"🧭️script-inputs/💾️entries/🧪️tests/🧰️members-set/🟦️.ts"};
for(const i of names){const dest=R+wanted[i];mkdirSync(dirname(dest),{recursive:true});copyFileSync(here+tests[wanted[i]],dest);}
for(const k of plan.keys())console.log(k.slice(-60),before[k],"->",sha(R+k));
