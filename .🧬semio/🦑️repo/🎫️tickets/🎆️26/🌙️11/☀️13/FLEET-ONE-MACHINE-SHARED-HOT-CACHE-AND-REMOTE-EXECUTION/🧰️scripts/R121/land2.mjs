import{readFileSync,writeFileSync,mkdirSync,copyFileSync,renameSync,rmSync}from"node:fs";import{createHash}from"node:crypto";import{dirname}from"node:path";
const R="/Users/ueli/Documents/semio/",SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R121b/",BK=SC+"backup/",here=import.meta.dir+"/";
const{all,apply}=await import(here+"patches2.mjs");
const sha=p=>createHash("sha256").update(readFileSync(p)).digest("hex").slice(0,12);
const keys=[...new Set(all.map(a=>a.file))],out=new Map();for(const k of keys)out.set(k,apply(k,readFileSync(R+k,"utf8"),all));
if(process.argv.includes("--dry")){console.log("dry ok",[...out].map(([k,v])=>k.slice(-30)+" "+v.length).join(" | "));process.exit(0);}
rmSync(BK,{recursive:true,force:true});const before={};for(const k of keys){before[k]=sha(R+k);mkdirSync(dirname(BK+k),{recursive:true});copyFileSync(R+k,BK+k);}
for(const[k,text]of out){writeFileSync(R+k+".r121btmp",text);renameSync(R+k+".r121btmp",R+k);}
for(const k of keys)console.log(k.slice(-40),before[k],"->",sha(R+k));
