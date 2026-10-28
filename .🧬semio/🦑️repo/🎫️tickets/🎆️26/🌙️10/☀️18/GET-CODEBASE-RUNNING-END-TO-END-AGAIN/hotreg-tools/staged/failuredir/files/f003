/** 🫥️ Contract and oracle: a Bazel plan record survives the folder of the failed-action output tails. Every build empties that folder before it starts (`runBazelBatchWithChurnV1`) and a failed action refills it, so the plan recorded beside a leftover folder must stay reusable once the next build removed it (before: `the generated workspace changed` and a second planner run of 25-50 s on the next hot no-op), a failure after the record must not matter either, and a file the planner itself wrote that is replaced or removed still breaks the record and is named in both languages. Oracles: the build's own removal of the folder and an independent directory scan filtered by the authored policy lists.
 * @see ../../🟦️.ts `bazelPlanOwnsV1`
 * @see ../../../../🪵️failure-tail/🟦️.ts `runBazelBatchWithChurnV1` */
import {expect,test}from"bun:test";
import {mkdirSync,mkdtempSync,readdirSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import {tmpdir}from"node:os";
import {join}from"node:path";
import failureTailSchema from"../../../🪵️failure-tail/🧬️schema/🔣️.json";
import reuseSchema from"../../🧬️schema/🔣️.json";
import {bazelFailureTailDirectoryV1,createBazelFailureTailV1,runBazelBatchWithChurnV1,writeBazelFailureTailV1}from"../../../🪵️failure-tail/🟦️.ts";
import {bazelPlanOutputsV1,bazelPlanOwnsV1,checkBazelPlanReuseV1,recordBazelPlanV1}from"../../🟦️.ts";
import {renderWorldWitnessTextV1}from"../../../../../🔭️witness/🟨️.mjs";

const signal=new AbortController().signal,policy=reuseSchema.definitions.Policy.const,directory=failureTailSchema.definitions.Policy.const.directory;
const implementation=(workspace:string)=>({version:1 as const,snapshot:join(workspace,"implementation.json"),runner:join(workspace,"📜️script.ts"),sha256:"a".repeat(64),loaderSha256:"b".repeat(64)});
const key="c".repeat(64),witness=JSON.stringify({version:1,observations:[],racy:null,unstable:[],tainted:[]});
function workspace(){
 const base=realpathSync(mkdtempSync(join(tmpdir(),"owned-"))),space=join(base,"ws");mkdirSync(space);
 writeFileSync(join(space,"BUILD.bazel"),"build\n");writeFileSync(join(space,"tasks.bzl"),"tasks\n");writeFileSync(join(space,"t_a.json"),"{}\n");
 return{base,space};
}
async function record(space:string){
 writeFileSync(join(space,policy.witness),witness);
 return recordBazelPlanV1({workspace:space,key,implementation:implementation(space),adapter:implementation(space),scopes:{},ignore:new Set(),signal,nowMs:Date.now()});
}
const check=(space:string)=>checkBazelPlanReuseV1({workspace:space,key,ignore:new Set(),signal});
const leave=async(space:string,task:string)=>{const tail=createBazelFailureTailV1();tail.push("failed "+task);await writeBazelFailureTailV1(bazelFailureTailDirectoryV1(space),task,tail);};
const scan=(space:string)=>readdirSync(space).filter(name=>name!==directory&&!policy.excluded.includes(name)&&!policy.excludedPrefixes.some(prefix=>name.startsWith(prefix))&&!policy.excludedSuffixes.some(suffix=>name.endsWith(suffix))).sort();

test("a plan recorded beside the failure folder of an earlier failed run is reused after the next build emptied the folder",async()=>{
 const{base,space}=workspace();
 try{
  await leave(space,"@s/a:test");
  expect(await record(space)).toMatchObject({recorded:true});
  expect(await check(space),"nothing moved yet").toMatchObject({reuse:true});
  expect(await runBazelBatchWithChurnV1(bazelFailureTailDirectoryV1(space),async()=>"built")).toBe("built");
  expect(readdirSync(space),"oracle: the build removed the folder").not.toContain(directory);
  expect(await check(space),"the removal of the executor's own folder is no change of the plan").toMatchObject({reuse:true});
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("a failure after the record leaves the plan reusable and the recorded outputs are exactly the files the planner wrote",async()=>{
 const{base,space}=workspace();
 try{
  expect(await record(space)).toMatchObject({recorded:true});
  await leave(space,"@s/b:test");
  expect(await check(space)).toMatchObject({reuse:true});
  const rows=await bazelPlanOutputsV1(space,new Set(),true,signal);
  expect(rows.map(row=>row.name),"oracle: an independent scan of the directory").toEqual(scan(space));
  expect(rows.map(row=>row.name)).not.toContain(directory);
  expect(bazelPlanOwnsV1(directory,new Set())).toBe(false);
  expect(bazelPlanOwnsV1("BUILD.bazel",new Set())).toBe(true);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("a file the planner wrote that is replaced or removed breaks the record and the reason names the file in both languages",async()=>{
 const{base,space}=workspace();
 try{
  expect(await record(space)).toMatchObject({recorded:true});
  rmSync(join(space,"tasks.bzl"));
  const verdict=await check(space);
  expect(verdict).toMatchObject({reuse:false,reason:"outputs",subject:"tasks.bzl"});
  if(verdict.reuse)throw Error("unreachable");
  for(const language of["en","de"]as const)expect(renderWorldWitnessTextV1("Reasons",language,verdict.reason,{kind:verdict.kind??"",subject:verdict.subject??""})).toContain("tasks.bzl");
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the folder name is the one authored by the failure tail policy, not a copy",()=>{
 expect(directory).toBe("failures");
 expect(policy.excluded,"no second authored copy").not.toContain(directory);
});
