import assert from "node:assert/strict";
import {mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {admitPlannedBuildCapacityV1,reserveGraphPlanCapacityV1,withBuildCapacityV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts";
import {readHostCapacityPolicyV1,readHostCapacityStatusV1,type HostSampleV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🟦️.ts";
import buildPolicy from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json";

const policy=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0}});
const sample=async()=>({host:{version:1,atMs:Date.now(),platform:process.platform,cores:12,totalMegabytes:32768,availableMegabytes:24000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8,openFilesMaximum:100,attachedImages:0} as HostSampleV1,swappedOutMegabytes:0});
const control=()=>({signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const sleep=(ms:number)=>new Promise(resolve=>setTimeout(resolve,ms));
async function resolveWhen<T>(probe:()=>Promise<T|undefined>):Promise<T>{
  const until=Date.now()+8000;
  while(Date.now()<until){const value=await probe();if(value!==undefined)return value;await sleep(25);}
  throw Error("Capacity fixture did not reach its checkpoint");
}
const {role,dir}= {role:process.argv[2],dir:process.argv[3]};
if (role==="host"||role==="client") {
  const environment={LANG:"en_US.UTF-8",SEMIO_HOST_CAPACITY_DIRECTORY:dir!},ports={directory:dir!,policy,environment,sample,write:()=>undefined};
  const status=()=>readHostCapacityStatusV1({...ports,control:control()});
  const path=(file:string)=>join(dir!,file);
  if(role==="host"){
    const grant=await reserveGraphPlanCapacityV1({...ports,megabytes:2048,observe:async()=>850});
    const worker=Bun.spawn([process.execPath,"-e","setInterval(()=>{},200)"],{stdin:"ignore",stdout:"ignore",stderr:"inherit"});
    grant.attach(worker.pid);
    try {
      writeFileSync(path("graph.json"),JSON.stringify({host:process.pid,worker:worker.pid}));
      await resolveWhen(async()=>Bun.file(path("complete")).exists().then(exists=>exists?true:undefined));
    }finally{worker.kill();await worker.exited;grant.release();}
  }else{
    let clientSample=45;
    await withBuildCapacityV1(["run-many","--projects=repo-mcp","--targets=build","--parallel=4"],environment,async ({handle})=>{
      const id=handle!.id;
      const first=(await status()).grants.find(grant=>grant.id===id)!;
      assert.equal(first.quiet,true);
      const graph=await resolveWhen(async()=>Bun.file(path("graph.json")).exists().then(exists=>exists?JSON.parse(readFileSync(path("graph.json"),"utf8")):undefined));
      assert.notEqual(graph.host,process.pid);
      assert.notEqual(graph.worker,process.pid);
      const during=(await status()).grants.find(grant=>grant.operation==="graph-plan")!;
      assert.equal(during.pid,graph.host);
      assert.equal(during.reservedMegabytes,2048);
      await resolveWhen(async()=>((await status()).grants.find(grant=>grant.pid===graph.host)?.observedMegabytes===850)?true:undefined);
      clientSample=first.reservedMegabytes+256;
      await resolveWhen(async()=>((await status()).grants.find(grant=>grant.id===id)?.observedMegabytes===clientSample)?true:undefined);
      clientSample=45;
      await sleep(650);
      const waiting=(await status()).grants.find(grant=>grant.id===id)!;
      assert.deepEqual({quiet:waiting.quiet,reserved:waiting.reservedMegabytes},{quiet:true,reserved:first.reservedMegabytes},"Planning client grew execution reservation while external graph child was still running");
      process.kill(graph.worker,0);
      writeFileSync(path("complete"),"done");
      await admitPlannedBuildCapacityV1(undefined,environment);
      const running=(await status()).grants.find(grant=>grant.id===id)!;
      assert.ok(running.reservedMegabytes>first.reservedMegabytes && running.quiet!==true);
    },{...ports,plannedByService:true,strip:args=>args,observe:async()=>clientSample});
  }
}else{
 const directory=mkdtempSync(join(import.meta.dirname,"🗑️generated","pg-"));
 const host=Bun.spawn([process.execPath,import.meta.path,"host",directory],{stdin:"ignore",stdout:"inherit",stderr:"inherit"});
 const client=Bun.spawn([process.execPath,import.meta.path,"client",directory],{stdin:"ignore",stdout:"inherit",stderr:"inherit"});
 try {
  const clientExit=await client.exited;
  writeFileSync(join(directory,"complete"),"done");
  const hostExit=await host.exited;
  assert.deepEqual([clientExit,hostExit],[0,0]);
 }finally{rmSync(directory,{recursive:true,force:true});}
}
