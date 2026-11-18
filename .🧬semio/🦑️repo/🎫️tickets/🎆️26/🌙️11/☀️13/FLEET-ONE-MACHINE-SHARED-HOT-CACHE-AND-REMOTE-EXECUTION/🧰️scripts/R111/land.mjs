import{readFileSync,writeFileSync,mkdirSync,copyFileSync,renameSync,rmSync}from"node:fs";import{createHash}from"node:crypto";import{dirname}from"node:path";import{apply,all}from"./patches.mjs";
const R="/Users/ueli/Documents/semio/",BK="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R111/backup/",here=import.meta.dir+"/";
const TEST="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧯️old-record-settle/🟦️.ts";
const sha=p=>createHash("sha256").update(readFileSync(p)).digest("hex").slice(0,12);
const plan=new Map();for(const p of all)plan.set(p.file,apply(p.file,readFileSync(R+p.file,"utf8"),[p]));
if(process.argv.includes("--dry")){for(const[k,v]of plan)console.log("ok",k.slice(-40),v.length);process.exit(0);}
rmSync(BK,{recursive:true,force:true});const before={};for(const k of plan.keys()){before[k]=sha(R+k);mkdirSync(dirname(BK+k),{recursive:true});copyFileSync(R+k,BK+k);}
for(const[k,v]of plan){writeFileSync(R+k+".r111tmp",v);renameSync(R+k+".r111tmp",R+k);}
mkdirSync(dirname(R+TEST),{recursive:true});copyFileSync(here+"old-record-settle.test.ts",R+TEST);
for(const k of plan.keys())console.log(k.slice(-40),before[k],"->",sha(R+k));
