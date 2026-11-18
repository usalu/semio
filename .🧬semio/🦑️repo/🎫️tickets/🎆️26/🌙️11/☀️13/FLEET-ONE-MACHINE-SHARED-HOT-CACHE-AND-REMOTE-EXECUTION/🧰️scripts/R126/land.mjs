import{readFileSync,writeFileSync,mkdirSync,copyFileSync,renameSync,rmSync,existsSync,mkdtempSync}from"node:fs";import{createHash}from"node:crypto";import{dirname,join}from"node:path";import{spawnSync}from"node:child_process";import{tmpdir}from"node:os";
const R=process.env.R126_ROOT??"/Users/ueli/Documents/semio/",SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R126/",BK=SC+"backup/",here=import.meta.dir+"/";
const files=JSON.parse(readFileSync(here+"files.json","utf8")),baseOf={"inputs.mjs":"base/inputs.mjs","entries.mjs":"base/entries.mjs","entries.d.mts":"base/entries.d.mts","reuse.mjs":"base/reuse.mjs","reuse.d.mts":"base/reuse.d.mts","reuse-schema.json":"base/reuse-schema.json","limits.json":"base/limits.json","nodes.mjs":"base/nodes.mjs","t-set-aside.ts":"base/t-set-aside.ts"};
const list=[...files.mods.map(([live,name])=>[live,"stage/"+name,baseOf[name]]),...files.newTests.map(([live,name])=>[live,"stage/"+name,null])];
const sha=p=>createHash("sha256").update(readFileSync(p)).digest("hex").slice(0,12);
const out=new Map(),merged=[];
for(const[live,staged,base]of list){
 const stagedPath=SC+staged;
 if(existsSync(R+live)&&sha(R+live)===sha(stagedPath))continue;
 if(base===null){if(existsSync(R+live)){console.log("new file exists",live);process.exit(2);}out.set(live,readFileSync(stagedPath));continue;}
 if(sha(R+live)===sha(SC+base)){out.set(live,readFileSync(stagedPath));continue;}
 const tmp=mkdtempSync(join(tmpdir(),"r126-merge-")),cur=tmp+"/cur";copyFileSync(R+live,cur);
 const r=spawnSync("git",["merge-file","-p",cur,SC+base,stagedPath],{encoding:"buffer",maxBuffer:2**28});
 rmSync(tmp,{recursive:true,force:true});
 if(r.status!==0){console.log("CONFLICT or peer change not mergeable:",live,r.status);process.exit(3);}
 out.set(live,r.stdout);merged.push(live);
}
if(process.argv.includes("--dry")){console.log("dry ok; merged over peer edits:",merged,"files:",out.size);process.exit(0);}
rmSync(BK,{recursive:true,force:true});const before={};
for(const[live]of out){before[live]=existsSync(R+live)?sha(R+live):"new";if(existsSync(R+live)){mkdirSync(dirname(BK+live),{recursive:true});copyFileSync(R+live,BK+live);}}
writeFileSync(SC+"landed.json",JSON.stringify([...out.keys()]));
for(const[live,data]of out){mkdirSync(dirname(R+live),{recursive:true});writeFileSync(R+live+".r126tmp",data);renameSync(R+live+".r126tmp",R+live);}
for(const[live]of out)console.log(live.slice(-44),before[live],"->",sha(R+live));
