import{readFileSync,writeFileSync,mkdirSync,copyFileSync,renameSync,rmSync,existsSync}from"node:fs";import{createHash}from"node:crypto";import{dirname}from"node:path";import{spawnSync}from"node:child_process";
const R="/Users/ueli/Documents/semio/",SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R117/",BK=SC+"backup/",ST=SC+"stage/",BS=SC+"base/",here=import.meta.dir+"/",step=process.argv[2]??"step1";
const manifest=JSON.parse(readFileSync(here+"files.json","utf8")),files=manifest[step],tests=manifest["tests"+step.slice(4)]??[];
const sha=p=>createHash("sha256").update(readFileSync(p)).digest("hex").slice(0,12);
const merged=new Map();
for(const[k,name]of files){
 const live=R+k,base=BS+name,stage=ST+name;
 if(sha(live)===sha(base)){merged.set(k,readFileSync(stage));continue;}
 const tmp=SC+"merge-"+name;copyFileSync(live,tmp);
 const diff=spawnSync("diff",["-u",base,stage],{encoding:"utf8"});
 const run=spawnSync("patch",["-s",tmp],{input:diff.stdout,encoding:"utf8"});
 if(run.status!==0){console.log("MERGE FAILED",name,run.stdout,run.stderr);process.exit(1);}
 merged.set(k,readFileSync(tmp));console.log("merged over a live edit",name);
}
for(const[k,data]of merged)if(/\.mjs$/.test(k)){const copy=SC+"c.mjs";writeFileSync(copy,data);const c=spawnSync("node",["--check",copy],{encoding:"utf8"});if(c.status!==0){console.log("syntax",k.slice(-30),c.stderr.slice(0,300));process.exit(1);}}
if(process.argv.includes("--dry")){console.log("dry ok");process.exit(0);}
rmSync(BK,{recursive:true,force:true});const before={};
for(const k of merged.keys()){before[k]=sha(R+k);mkdirSync(dirname(BK+k),{recursive:true});copyFileSync(R+k,BK+k);}
for(const[k,data]of merged){writeFileSync(R+k+".r117tmp",data);renameSync(R+k+".r117tmp",R+k);}
for(const[k,name]of tests){mkdirSync(dirname(R+k),{recursive:true});copyFileSync(SC+name,R+k);}
for(const k of merged.keys())console.log(k.slice(-40),before[k],"->",sha(R+k));
