import {spawnSync} from "node:child_process";
const B="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const inference=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const obs=await import(B+"👁️observation/🟨️.mjs");
if(process.argv[2]==="child"){
  const rows=JSON.parse(await Bun.stdin.text());
  const conflicts=obs.adoptInferenceAuthorityV1(rows);
  console.log(JSON.stringify({id:obs.inferenceRecordIdentityV1(),conflicts,n:rows.length}));
}else{
  const rows=obs.inferenceAuthorityRowsV1(),id=obs.inferenceRecordIdentityV1();
  const r=spawnSync(process.execPath,[import.meta.path,"child"],{input:JSON.stringify(rows)});
  console.log("[DEBUG] parent",id,rows.length);console.log("[DEBUG] child",r.stdout.toString().slice(0,400),r.stderr.toString().slice(0,300));
}
