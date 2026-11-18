import{readFileSync,writeFileSync,mkdirSync,copyFileSync,renameSync,rmSync,readdirSync,statSync}from"node:fs";import{createHash}from"node:crypto";import{dirname,join}from"node:path";
const LIST=process.env.R120_LIST??"all";const R="/Users/ueli/Documents/semio/",SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R120/",BK=SC+"backup-"+LIST+"/",STAGE=SC+"stage/",here=import.meta.dir+"/";
const P=await import(here+"patches.mjs"),all=P[LIST],apply=P.apply;
const sha=p=>createHash("sha256").update(readFileSync(p)).digest("hex").slice(0,12);
const keys=[...new Set(all.map(a=>a.file))];
const out=new Map();for(const k of keys)out.set(k,apply(k,readFileSync(R+k,"utf8"),all));
const staged=[];const walk=d=>{for(const n of readdirSync(d)){const p=join(d,n);if(statSync(p).isDirectory())walk(p);else staged.push(p.slice(STAGE.length));}};if(LIST==="all")walk(STAGE);
if(process.argv.includes("--dry")){console.log("dry ok",[...out].map(([k,v])=>k.slice(-30)+" "+v.length).join(" | "),"staged",staged.length);process.exit(0);}
rmSync(BK,{recursive:true,force:true});const before={};for(const k of keys){before[k]=sha(R+k);mkdirSync(dirname(BK+k),{recursive:true});copyFileSync(R+k,BK+k);}
for(const[k,text]of out){writeFileSync(R+k+".r120tmp",text);renameSync(R+k+".r120tmp",R+k);}
for(const k of staged){mkdirSync(dirname(R+k),{recursive:true});copyFileSync(STAGE+k,R+k);}
writeFileSync(SC+"staged-"+LIST+".list",staged.join("\n"));
for(const k of keys)console.log(k.slice(-40),before[k],"->",sha(R+k));
