import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {createHash} from "node:crypto";
import {fleetCacheLocalPathV1,remoteCacheFetchV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts";
import {publishFleetCacheFactsV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/📣️publication/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheRequestEventV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {httpRemoteExecutionContentV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🗄️content/🟦️.ts";
import {encodeProtobufV1,REAPI_V1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import {generationExchangeKeyV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts";
import {ensureFleetIdentityV1,fleetRegistrationPathV1,loadFleetSignerV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {createReadyStatusV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts";
import {createOwnerPoolStatusV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/📊️status/🟦️.ts";
import {publishWholeGenerationAdoptionReceiptV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🪪️adoption/🟦️.ts";
import {verifyGraphAdoptionV1} from "./verify-graph-adoption.ts";
import fixtures from "./verify-graph-adoption.cases.json";

const payload=Buffer.from(JSON.stringify({generation:12}));
const eligible={origin:"producer",scope:"all",generation:12,manifest:createHash("sha256").update(payload).digest("hex"),identity:"c".repeat(64),implementation:"d".repeat(64),platform:"linux-x64"};
const now=1_000_000;

test("receipt verification is pending without a lookup, then requires pinned evidence, readiness and exact independent head",async()=>{
 const generated=join(import.meta.dir,"..","🗑️generated");mkdirSync(generated,{recursive:true});
 const folder=mkdtempSync(join(generated,"gav-"));
 try{
  const store=join(folder,"s"),root=join(folder,"r"),pinsFile=join(folder,"pins.json"),destination="receiver",producer="producer";
  const input={root,store,producer,destination,pinsFile,group:"g1",now};
  let lookups=0;
  expect(await verifyGraphAdoptionV1(input,async()=>{lookups++;return eligible;})).toEqual({status:"pending",reason:"receipt-missing"});
  expect(lookups).toBe(0);
  const pins:Record<string,{publicKey:string;pinnedAtMs:number}>={};
  const serverRoot=join(folder,"cache-root"),producerRoot=join(folder,"p");
  for(const origin of [destination,producer,"g1-server"]){
   const machineRoot=origin===destination?root:origin===producer?producerRoot:serverRoot,machine=join(machineRoot,".cache","f","machine.json");
   mkdirSync(dirname(machine),{recursive:true});writeFileSync(machine,JSON.stringify({originId:origin,groupId:"g1",storeRoot:store}));
   const {registration}=await ensureFleetIdentityV1({root:machineRoot,originId:origin,group:"g1"});
   const path=fleetRegistrationPathV1(store,origin);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,JSON.stringify(registration));
   pins[origin]={publicKey:registration.origin,pinnedAtMs:1};
  }
  const ready=createReadyStatusV1({store,origin:destination,platform:"linux",now:()=>now}),pool=createOwnerPoolStatusV1({store,origin:destination,role:"puller",now:()=>now});
  ready.update({state:"ready",ready:true,stage:null});
  await publishWholeGenerationAdoptionReceiptV1({root,store,origin:destination,localDirectory:join(folder,"local"),candidate:eligible,outcome:{status:"restored",head:eligible,generation:13,localManifest:"e".repeat(64)},now:()=>now});
  using sqlite=new Database(":memory:");
  const oracle=sqlite.query("SELECT CASE WHEN NOT ?1 OR ?2<>16384 OR ?4>600000 THEN 'refused' WHEN ?3='missing' THEN 'pending' WHEN ?3='different' THEN 'refused' ELSE 'verified' END AS status");
  for(const row of fixtures){
   const held=row.pinned?pins:{...pins,[destination]:{...pins[destination]!,publicKey:pins[producer]!.publicKey}};
   writeFileSync(pinsFile,JSON.stringify({version:1,pins:held}));
   pool.update({totalMegabytes:row.memory});pool.flush();
   lookups=0;
   const age="age" in row?row.age:0;
   const result=await verifyGraphAdoptionV1({...input,now:now+age},async()=>{lookups++;return row.head==="missing"?undefined:row.head==="different"?{...eligible,generation:14}:eligible;});
   expect(oracle.get(Number(row.pinned),row.memory,row.head,age),row.id).toEqual({status:row.status});
   expect(result.status,row.id).toBe(row.status);
   expect(lookups,row.id).toBe(row.lookups);
   expect(JSON.stringify(result)).not.toContain(destination);
  }
  const events:RemoteCacheRequestEventV1[]=[],cache=join(folder,"cache"),instance="semio",expectedFile=join(folder,"expected.json");
  writeFileSync(expectedFile,JSON.stringify(eligible));
  const server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**25,maximumEntryBytes:2**22,maximumAgeDays:30,defaultInstance:instance,tokens:{},validation:"complete",fleet:{group:"g1",root:serverRoot,storeRoot:store,inGroup:"sealed"},onEvent:event=>{if(event.stage==="request")events.push(event);}});
  try{
   const endpoint=server.url,admin=loadFleetSignerV1(serverRoot,"admin")!;
   await publishFleetCacheFactsV1({admin,storeRoot:store,directory:cache,group:"g1",instance,port:server.port,endpoints:[endpoint],transport:"sealed"});
   const facts=await Bun.file(join(store,"v1","fleet","cache.json")).json();
   for(const reader of [root,producerRoot])writeFileSync(fleetCacheLocalPathV1(reader),JSON.stringify({version:1,fetchedAtMs:Date.now(),checkedAtMs:Date.now(),selectedEndpoint:endpoint,facts}));
   const content=httpRemoteExecutionContentV1({endpoint,fetch:remoteCacheFetchV1({}, {root:producerRoot})}),key=generationExchangeKeyV1(eligible.identity,eligible.platform,producer);
   await content.write(instance,{hash:eligible.manifest,size:payload.length},payload);
   const note={version:1,generation:eligible.generation,owners:1,identity:eligible.identity,manifest:eligible.manifest,signature:"f".repeat(64),platform:eligible.platform,publishedMs:now,origin:producer,stateMs:now,implementation:eligible.implementation,stale:0};
   await content.writeActionResult(instance,key,encodeProtobufV1(REAPI_V1.actionResult,{output_files:[{path:`m/${eligible.manifest}.json`,digest:{hash:eligible.manifest,size_bytes:payload.length}}],stdout_raw:Buffer.from(JSON.stringify(note))}));
   events.length=0;
   expect(await verifyGraphAdoptionV1({...input,expectedFile})).toEqual({status:"verified",reason:"exact-head-and-reported-16gib"});
   expect(events.map(({method,path,status})=>({method,path,status}))).toEqual([{method:"GET",path:`/${instance}/ac/${key}`,status:200}]);
  }finally{await server.stop();}
 }finally{rmSync(folder,{recursive:true,force:true});}
});
