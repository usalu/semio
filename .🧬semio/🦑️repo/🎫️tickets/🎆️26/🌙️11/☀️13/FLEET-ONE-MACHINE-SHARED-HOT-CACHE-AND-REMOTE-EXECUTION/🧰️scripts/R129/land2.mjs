import{readFileSync,writeFileSync,mkdirSync,copyFileSync,renameSync,rmSync,existsSync,mkdtempSync}from"node:fs";import{createHash}from"node:crypto";import{dirname,join}from"node:path";import{spawnSync}from"node:child_process";import{tmpdir}from"node:os";
const R="/Users/ueli/Documents/semio/",SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R129/",BK=SC+"backup/";
const H="🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/",B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",F=B+"🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/";
const files=[
 [F+"🧪️tests/🧬️schema/🔣️.json","stage/tests/t-schema.json","stage/tests/t-schema.json.base"],
];
const sha=p=>createHash("sha256").update(readFileSync(p)).digest("hex").slice(0,12);
const out=new Map(),merged=[];
for(const[live,staged,base]of files){
 const stagedPath=SC+staged;
 if(existsSync(R+live)&&sha(R+live)===sha(stagedPath))continue;
 if(base===null){if(existsSync(R+live)){console.log("new file exists",live);process.exit(2);}out.set(live,readFileSync(stagedPath));continue;}
 if(sha(R+live)===sha(SC+base)){out.set(live,readFileSync(stagedPath));continue;}
 const tmp=mkdtempSync(join(tmpdir(),"r129-merge-")),cur=tmp+"/cur";copyFileSync(R+live,cur);
 const r=spawnSync("git",["merge-file","-p",cur,SC+base,stagedPath],{encoding:"buffer",maxBuffer:2**28});
 rmSync(tmp,{recursive:true,force:true});
 if(r.status!==0){console.log("CONFLICT or peer change not mergeable:",live,r.status);process.exit(3);}
 out.set(live,r.stdout);merged.push(live);
}
if(process.argv.includes("--dry")){console.log("dry ok; merged over peer edits:",merged,"files:",out.size);process.exit(0);}
rmSync(BK,{recursive:true,force:true});const before={};
for(const[live]of out){before[live]=existsSync(R+live)?sha(R+live):"new";if(existsSync(R+live)){mkdirSync(dirname(BK+live),{recursive:true});copyFileSync(R+live,BK+live);}}
writeFileSync(SC+"landed.json",JSON.stringify([...out.keys()]));
for(const[live,data]of out){mkdirSync(dirname(R+live),{recursive:true});writeFileSync(R+live+".r129tmp",data);renameSync(R+live+".r129tmp",R+live);}
for(const[live]of out)console.log(live.slice(-44),before[live],"->",sha(R+live));
