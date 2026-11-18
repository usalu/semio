import{readFileSync,writeFileSync,mkdirSync,copyFileSync,renameSync,rmSync,existsSync,mkdtempSync}from"node:fs";import{createHash}from"node:crypto";import{dirname,join}from"node:path";import{spawnSync}from"node:child_process";import{tmpdir}from"node:os";
const R="/Users/ueli/Documents/semio/",SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R135/",BK=SC+"backup/";
const B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const step=process.env.R135_STEP??"1";
const sets={
"1":[
 [B+"🔣️policy.json","stage/policy.json","stage/policy.base.json"],
 [B+"🧠️memory/🪜️plan-stage/🧬️schema/🔣️.json","stage/planschema.json","stage/planschema.base.json"],
 [B+"🧠️memory/🪜️plan-stage/🟨️.mjs","stage/planstage.mjs","stage/planstage.base.mjs"],
 [B+"🪪️graph-state/🙋️wanted/💰️cost/🧬️schema/🔣️.json","stage/costschema.json","stage/costschema.base.json"],
 [B+"🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts","stage/cost.ts","stage/cost.base.ts"],
 [B+"🪪️graph-state/🙋️wanted/💰️cost/🧪️tests/🟦️.ts","stage/costtest.ts","stage/costtest.base.ts"],
 [B+"🌡️host-capacity/🧪️tests/🟦️.ts","stage/hctest.ts","stage/hctest.base.ts"],
 [B+"🌡️host-capacity/🧪️tests/📉️shrink/🟦️.ts","stage/shrinktest.ts","stage/shrinktest.base.ts"],
 [B+"🧠️memory/🪜️plan-stage/🧪️tests/🟦️.ts","stage/plantest.ts","stage/plantest.base.ts"],
 [B+"🧠️memory/🪜️plan-stage/🧪️tests/🧮️tree-plan/🟦️.ts","stage/treeplan.ts","stage/treeplan.base.ts"],
 [B+"🧠️memory/🪜️plan-stage/🧪️tests/📐️farm-width/🟦️.ts","stage/fwtest.ts","stage/fwtest.base.ts"],
 [B+"🧠️memory/🪜️plan-stage/🧪️tests/📐️farm-width/🔣️.json","stage/fwfix.json","stage/fwfix.base.json"],
 [B+"🪪️graph-state/🙋️wanted/🧪️tests/🍏️idle-grant/🟦️.ts","stage/idle.ts","stage/idle.base.ts"],
 [B+"🪪️graph-state/🙋️wanted/🧪️tests/🐧️linux-host/🟦️.ts","stage/linuxhost.ts","stage/linuxhost.base.ts"],
 [B+"🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🧯️cap/🟦️.ts","stage/cap.ts","stage/cap.base.ts"],
 [B+"🌡️host-capacity/🧪️tests/🖥️host-matrix/🟦️.ts","stage/matrix.test.ts",null],
],
"2":[
 [B+"🐢️speed/🟨️.mjs","stage/speed/speed.mjs",null],[B+"🐢️speed/🟨️.d.mts","stage/speed/speed.d.mts",null],[B+"🐢️speed/🧬️schema/🔣️.json","stage/speed/schema.json",null],
 [B+"🐢️speed/🧪️tests/🟦️.ts","stage/speed/speedtest.ts",null],[B+"🐢️speed/🧪️tests/🔣️.json","stage/speed/fixture.json",null],[B+"🐢️speed/🧪️tests/🐍️.py","stage/speed/oracle.py",null],
 [B+"🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🟨️.mjs","stage/pool.mjs","stage/pool.base.mjs"],
 [B+"🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🟨️.d.mts","stage/pool.d.mts","stage/pool.base.d.mts"],
 [B+"🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/⏳️progress-timer/🟦️.ts","stage/progresstest/test.ts",null],
 [B+"🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/⏳️progress-timer/🛰️service/🟨️.mjs","stage/progresstest/service.mjs",null],
 [B+"🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🏭️walk-farm/🟨️.mjs","stage/walk.mjs","stage/walk.base.mjs"],
 [B+"🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🏭️walk-farm/🧪️tests/🟦️.ts","stage/walktest.ts","stage/walktest.base.ts"],
 [B+"🕸️graph/📥️inference/📦️project-nodes/📥️take/🟨️.mjs","stage/take.mjs","stage/take.base.mjs"],
 [B+"🕸️graph/📥️inference/📦️project-nodes/📥️take/🧪️tests/🚦️no-progress/🟦️.ts","stage/takenp.ts","stage/takenp.base.ts"],
 [B+"🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts","stage/ready.ts","stage/ready.base.ts"],
 [B+"🌐️remote-execution/🛰️sidecar/🚦️ready/🧬️schema/🔣️.json","stage/readyschema.json","stage/readyschema.base.json"],
 [B+"🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🟦️.ts","stage/tree.ts","stage/tree.base.ts"],
 [B+"🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🧬️schema/🔣️.json","stage/treeschema.json","stage/treeschema.base.json"],
 [B+"🌐️remote-execution/🛰️sidecar/🚦️ready/🧪️tests/🟦️.ts","stage/readytest.ts","stage/readytest.base.ts"],
 [B+"🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🧪️tests/🟦️.ts","stage/treetest.ts","stage/treetest.base.ts"],
],
};
const files=sets[step];
const sha=p=>createHash("sha256").update(readFileSync(p)).digest("hex").slice(0,12);
const out=new Map(),merged=[];
for(const[live,staged,base]of files){
 const stagedPath=SC+staged;
 if(existsSync(R+live)&&sha(R+live)===sha(stagedPath))continue;
 if(base===null){if(existsSync(R+live)){console.log("new file exists",live);process.exit(2);}out.set(live,readFileSync(stagedPath));continue;}
 if(sha(R+live)===sha(SC+base)){out.set(live,readFileSync(stagedPath));continue;}
 const tmp=mkdtempSync(join(tmpdir(),"r135-merge-")),cur=tmp+"/cur";copyFileSync(R+live,cur);
 const r=spawnSync("git",["merge-file","-p",cur,SC+base,stagedPath],{encoding:"buffer",maxBuffer:2**28});
 rmSync(tmp,{recursive:true,force:true});
 if(r.status!==0){console.log("CONFLICT or peer change not mergeable:",live,r.status);process.exit(3);}
 out.set(live,r.stdout);merged.push(live);
}
if(process.argv.includes("--dry")){console.log("dry ok; merged over peer edits:",merged,"files:",out.size);process.exit(0);}
rmSync(BK,{recursive:true,force:true});const before={};
for(const[live]of out){before[live]=existsSync(R+live)?sha(R+live):"new";if(existsSync(R+live)){mkdirSync(dirname(BK+live),{recursive:true});copyFileSync(R+live,BK+live);}}
writeFileSync(SC+"landed.json",JSON.stringify([...out.keys()]));
for(const[live,data]of out){mkdirSync(dirname(R+live),{recursive:true});writeFileSync(R+live+".r135tmp",data);renameSync(R+live+".r135tmp",R+live);}
for(const[live]of out)console.log(live.slice(-44),before[live],"->",sha(R+live));
