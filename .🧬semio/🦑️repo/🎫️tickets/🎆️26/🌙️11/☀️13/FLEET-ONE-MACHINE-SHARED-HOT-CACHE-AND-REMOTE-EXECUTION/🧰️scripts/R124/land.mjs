import{readFileSync,writeFileSync,mkdirSync,copyFileSync,renameSync,rmSync,existsSync,mkdtempSync}from"node:fs";import{createHash}from"node:crypto";import{dirname,join}from"node:path";import{spawnSync}from"node:child_process";import{tmpdir}from"node:os";
const R="/Users/ueli/Documents/semio/",SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R124/",BK=SC+"backup/",here=import.meta.dir+"/";
const L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/",O=L+"🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/",P=L+"🏗️build/🌐️remote-execution/🛰️sidecar/🏊️pool/";
const G=L+"🏗️build/🪪️graph-state/📦️project-records/";
const files=[
 [O+"🟦️.ts","stage/owner-pool.ts","base/owner-pool.ts"],[O+"🏃️runner/🟨️.mjs","stage/runner.mjs","base/runner.mjs"],[O+"🧬️schema/🔣️.json","stage/schema.json","base/schema.json"],[O+"🧮️lifecycle/🟦️.ts","stage/lifecycle.ts","base/lifecycle.ts"],[P+"🟦️.ts","stage/sidecar-pool.ts","base/sidecar-pool.ts"],
 [O+"🧪️tests/🟦️.ts","stage/tests-main.ts","base/tests-main.ts"],[O+"🧪️tests/🧮️lifecycle/🟦️.ts","stage/lifecycle.test.ts","base/lifecycle.test.ts"],[O+"🧪️tests/🧫️fixtures/🔣️.json","stage/fixture.json","base/fixture.json"],[O+"🧪️tests/🧫️fixtures/🐍️oracle.py","stage/oracle.py","base/oracle.py"],
 [O+"🧪️tests/🎛️one-budget/🟦️.ts",here+"one-budget.test.ts","base/one-budget.ts"],
 [G+"🟦️.ts","f/stage/records.ts","f/base/records.ts"],[G+"🌱️fresh/🟦️.ts","f/stage/fresh.ts","f/base/fresh.ts"],[G+"🌱️fresh/🧪️tests/🟦️.ts","f/stage/fresh.test.ts","f/base/fresh.test.ts"],[G+"🌱️fresh/🧪️tests/🐍️.py","f/stage/fresh.py","f/base/fresh.py"],[G+"🌱️fresh/🧪️tests/🔣️.json","f/stage/fresh.json","f/base/fresh.json"],[G+"🧪️tests/🤝️pool-reader/🟦️.ts","f/stage/pool-reader.test.ts","f/base/pool-reader.test.ts"],
];
const sha=p=>createHash("sha256").update(readFileSync(p)).digest("hex").slice(0,12);
const out=new Map(),merged=[];
for(const[live,staged,base]of files){
 const stagedPath=staged.startsWith("/")?staged:SC+staged;
 if(existsSync(R+live)&&sha(R+live)===sha(stagedPath))continue;
 if(base===null){if(existsSync(R+live)&&sha(R+live)!==sha(stagedPath)){console.log("new file exists",live);process.exit(2);}out.set(live,readFileSync(stagedPath));continue;}
 if(sha(R+live)===sha(SC+base)){out.set(live,readFileSync(stagedPath));continue;}
 const tmp=mkdtempSync(join(tmpdir(),"r124-merge-")),cur=tmp+"/cur";copyFileSync(R+live,cur);
 const r=spawnSync("git",["merge-file","-p",cur,SC+base,stagedPath],{encoding:"buffer",maxBuffer:2**28});
 rmSync(tmp,{recursive:true,force:true});
 if(r.status!==0){console.log("CONFLICT or peer change not mergeable:",live,r.status);process.exit(3);}
 out.set(live,r.stdout);merged.push(live);
}
if(process.argv.includes("--dry")){console.log("dry ok; merged over peer edits:",merged);process.exit(0);}
rmSync(BK,{recursive:true,force:true});const before={};
for(const[live]of out){before[live]=existsSync(R+live)?sha(R+live):"new";if(existsSync(R+live)){mkdirSync(dirname(BK+live),{recursive:true});copyFileSync(R+live,BK+live);}}
for(const[live,data]of out){mkdirSync(dirname(R+live),{recursive:true});writeFileSync(R+live+".r124tmp",data);renameSync(R+live+".r124tmp",R+live);}
writeFileSync(SC+"landed.json",JSON.stringify([...out.keys()]));
for(const[live]of out)console.log(live.slice(-44),before[live],"->",sha(R+live));
