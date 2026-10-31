import{test,expect}from"bun:test";
import{constants}from"node:fs";
import{mkdtemp,open,lstat,writeFile,rm}from"node:fs/promises";
import{join}from"node:path";
import Ajv from"ajv";
import corpus from"./lsof-sentinel-cases.json" with{type:"json"};
import{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import{createIssuedResourceControlV1}from"../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{captureOwnedCommandResultV1}from"../../../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🟦️.ts";
import{withArtifactDescriptorV1}from"../../../../../../../🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
import{assertNativeSelectedDescriptorCustodyV1}from"../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/📂️descriptors/🟦️.ts";
declareBuildTestEnvelopeV1({owner:"test:lsof-sentinel-selection",maximumElapsedMilliseconds:45000,capabilities:{artifacts:true}});
test("owned positive sentinel does not satisfy unmatched recursive search terms",async()=>withBuildTestInvocationV1(async original=>{
 const control=createIssuedResourceControlV1(original,"test:lsof-sentinel-selection",30000),base=await mkdtemp(join(import.meta.dir,"🗑️generated/sentinel-")),scope=join(base,"scope"),sentinel=join(base,"sentinel"),rows:any[]=[];
 const fs=await import("node:fs/promises");await fs.mkdir(scope);await writeFile(join(scope,"closed-child"),"closed ordinary search term");await writeFile(sentinel,"held outside scope");
 const valid=new Ajv({strict:true}).compile({type:"array",minItems:4,maxItems:4,items:{type:"object",required:["name","status","accepted"],additionalProperties:false,properties:{name:{type:"string"},status:{enum:[0,1]},accepted:{type:"boolean"}}}});
 expect(valid(corpus)).toBe(true);
 try{await withArtifactDescriptorV1(()=>open(scope,constants.O_RDONLY|constants.O_NOFOLLOW|constants.O_NONBLOCK),async root=>withArtifactDescriptorV1(()=>open(sentinel,constants.O_RDONLY|constants.O_NOFOLLOW|constants.O_NONBLOCK),async witness=>{
 const first=await root.stat({bigint:true}),sentinelBefore=await witness.stat({bigint:true});
 const cases=[["recursive",["+D",scope]],["recursive-or-owned-pid",["-p",String(process.pid),"+D",scope]],["recursive-or-held-sentinel",["+D",scope,"--",sentinel]],["owned-pid-only",["-p",String(process.pid)]]] as const;
 for(const[name,selection]of cases){const result=await captureOwnedCommandResultV1("lsof",["-nP","-F0pcfatn",...selection],base,{control,output:{maximumBytes:131072,maximumLines:2048}});const current=await root.stat({bigint:true}),named=await lstat(scope,{bigint:true}),still=await witness.stat({bigint:true}),sentinelNamed=await lstat(sentinel,{bigint:true});expect([first.dev,first.ino]).toEqual([current.dev,current.ino]);expect([first.dev,first.ino]).toEqual([named.dev,named.ino]);expect([sentinelBefore.dev,sentinelBefore.ino]).toEqual([still.dev,still.ino]);expect([still.dev,still.ino]).toEqual([sentinelNamed.dev,sentinelNamed.ino]);expect(named.isSymbolicLink()||sentinelNamed.isSymbolicLink()).toBe(false);expect(result.signal).toBe(null);expect(result.stderr).toBe("");
 let accepted=true,refusal="";try{assertNativeSelectedDescriptorCustodyV1({...result,reason:"exit"},{selection:"filesystem",pid:process.pid,files:[{fd:root.fd,path:scope,type:"DIR"}]});}catch(e){accepted=false;refusal=String((e as Error).message);}
 const row={name,status:result.status,accepted,signal:result.signal,stderrBytes:Buffer.byteLength(result.stderr),stdoutBytes:Buffer.byteLength(result.stdout),scopePresent:result.stdout.includes(scope),sentinelPresent:result.stdout.includes(sentinel),refusal};rows.push(row);console.log("[DEBUG] "+JSON.stringify(row));}
 expect(rows.map(({name,status,accepted})=>({name,status,accepted}))).toEqual(corpus);expect(rows.slice(0,3).every(r=>r.scopePresent)).toBe(true);expect(rows[2].sentinelPresent).toBe(true);expect(rows[3].sentinelPresent).toBe(true);
 }));await writeFile(join(import.meta.dir,"🗑️generated/lsof-sentinel-observation.json"),JSON.stringify({platform:process.platform,pid:process.pid,rows},null,2)+"\n");
 }finally{await rm(base,{recursive:true,force:true});}
}),40000);
