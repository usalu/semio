import{readFileSync,writeFileSync,mkdirSync,copyFileSync,renameSync,rmSync,existsSync}from"node:fs";import{createHash}from"node:crypto";import{dirname}from"node:path";import{spawnSync}from"node:child_process";
const R="/Users/ueli/Documents/semio/",SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R114/",BK=SC+"backup/",here=import.meta.dir+"/",FINAL=SC+"final/";
const B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",N=B+"🕸️graph/📥️inference/📦️project-nodes/",F=N+"🏊️project-farm/";
const files=new Map([[B+"👁️observation/🟨️.mjs","obs.mjs"],[N+"🟨️.mjs","project-nodes.mjs"],[F+"🟨️.mjs","farm.mjs"],[F+"🟨️.d.mts","farm.d.mts"],[F+"👷️service/🟨️.mjs","service.mjs"],[B+"👁️observation/🟨️.d.mts","obs.d.mts"]]);
const sha=p=>createHash("sha256").update(readFileSync(p)).digest("hex").slice(0,12);
rmSync(FINAL,{recursive:true,force:true});mkdirSync(FINAL,{recursive:true});
for(const[script,args]of[["patch-observation.py",[FINAL+"obs.mjs"]],["patch-nodes.py",[FINAL]],["patch-types.py",[FINAL+"obs.d.mts"]]]){const run=spawnSync("python3",[here+script,...args],{encoding:"utf8"});if(run.status!==0){console.log(run.stderr);process.exit(1);}}
for(const name of files.values())if(name.endsWith(".mjs")&&!name.endsWith(".d.mts")){const check=spawnSync("node",["--check",FINAL+name,"--input-type=module"],{encoding:"utf8"});const copy=FINAL+"c-"+name;copyFileSync(FINAL+name,copy);const second=spawnSync("node",["--check",copy],{encoding:"utf8"});rmSync(copy);if(second.status!==0&&!/Cannot use import|ERR_UNKNOWN/.test(second.stderr)){console.log("syntax",name,second.stderr.slice(0,300));process.exit(1);}}
if(process.argv.includes("--dry")){console.log("dry ok");process.exit(0);}
rmSync(BK,{recursive:true,force:true});const before={};for(const k of files.keys()){before[k]=sha(R+k);mkdirSync(dirname(BK+k),{recursive:true});copyFileSync(R+k,BK+k);}
for(const[k,name]of files){writeFileSync(R+k+".r114tmp",readFileSync(FINAL+name));renameSync(R+k+".r114tmp",R+k);}
const T=B+"👁️observation/🧪️tests/",tests=[["adopted-verdict.test.ts",T+"🤝️adopted-verdict/🟦️.ts"],["children.test.ts",T+"🧒️children/🟦️.ts"],["child.ts",T+"🧒️children/👷️child/🟦️.ts"]];
for(const[name,k]of tests){mkdirSync(dirname(R+k),{recursive:true});copyFileSync(SC+"tests/"+name,R+k);}
for(const k of files.keys())console.log(k.slice(-40),before[k],"->",sha(R+k));
