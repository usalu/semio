import {spawnSync} from "node:child_process";
import {mkdtempSync} from "node:fs";
const B="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",W="/Users/Shared/semio";
const obs=await import(B+"👁️observation/🟨️.mjs");
if(process.argv[2]==="child"){
  const input=JSON.parse(await Bun.stdin.text());
  const svc=await import(B+"🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/👷️service/🟨️.mjs");
  const out=await svc.open(input.session,{slot:0,generation:0});
  console.log(JSON.stringify({records:out.records,conflicts:out.conflicts}));
}else{
  const inference=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
  const nodes=await import(B+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs");
  const contracts=nodes.projectGeneratorContractsV1(W);
  const session={version:3,workspaceRoot:W,closureStore:undefined,registry:null,contracts,commandInputs:null,lateClosures:false,projectsByRoot:[],componentRoots:[],stagingBase:mkdtempSync("/tmp/idc-"),authority:obs.inferenceAuthorityRowsV1()};
  const id=obs.inferenceRecordIdentityV1();
  const r=spawnSync(process.execPath,[import.meta.path,"child"],{input:JSON.stringify({session}),maxBuffer:1<<28});
  console.log("[DEBUG] parent",id,session.authority.length);console.log("[DEBUG] child",r.stdout.toString().slice(0,300),r.stderr.toString().slice(0,500));
}
