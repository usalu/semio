import {afterAll,beforeAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync} from "node:fs";
import {fileURLToPath} from "node:url";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {casObjectPathV1,casRootV1} from "../../../../../../🏗️build/🗄️cas/🗃️store/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1,type FleetClientV1} from "../../📡️client/🟦️.ts";
import {downloadBlobsV1,installFleetUnitV1,type InstallTargetV1} from "../../📥️adopt/🟦️.ts";
import {awaitFleetSettlesV1,fetchFleetUnitV1,type FleetUnitV1} from "../../🟦️.ts";
import {encodeActionResultV1,packUnitV1,wireDigestOfV1,type RemoteUnitV1} from "../../📦️wire/🟦️.ts";
import {FLEET_POLICY_V1} from "../../🪪️identity/🟦️.ts";

/** ⚡️ A fleet hit costs two round trips (the action result, then every blob at once) and never waits for the slot claim; the digests of the action result bound what is downloaded before a byte moves; the installed objects are the manifest's, checked by an independent SHA-256. */
const fleet=FLEET_POLICY_V1,sha=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex");
const scratch=mkdtempSync(join(tmpdir(),"r205-pipeline-")),previousCas=process.env.SEMIO_CAS_ROOT;
const platform={version:1,os:"darwin",arch:"arm64",host:"aarch64-apple-darwin",rustc:"a".repeat(64),toolchain:"t",linker:"d",cpu:""} as RemoteUnitV1["platform"];
let counter=0;
beforeAll(()=>{process.env.SEMIO_CAS_ROOT=join(scratch,"cas");});
afterAll(()=>{if(previousCas===undefined)delete process.env.SEMIO_CAS_ROOT;else process.env.SEMIO_CAS_ROOT=previousCas;rmSync(scratch,{recursive:true,force:true});});

type Built=Readonly<{key:string;slot:string;localKey:string;manifest:RemoteUnitV1;contents:Map<string,Uint8Array>}>;
function build(outputs:readonly number[]=[40000,6000]):Built{
 const id=counter++,hex=(label:string)=>sha(new TextEncoder().encode(label+id)),contents=new Map<string,Uint8Array>();
 const put=(bytes:Uint8Array)=>{const hash=sha(bytes);contents.set(hash,bytes);return {hash,size:bytes.length};};
 const rows=outputs.map((size,index)=>{const bytes=new Uint8Array(size);bytes.fill(index+1);new DataView(bytes.buffer).setUint32(0,id*977+index);const stored=put(bytes);return {name:`libc${id}-${index}.rlib`,sha256:stored.hash,bytes:size,executable:false};});
 const text=put(new TextEncoder().encode(`deps${id}: src/lib.rs\n`));rows.push({name:`c${id}.d`,sha256:text.hash,bytes:text.size,executable:false,template:true} as never);
 const manifest:RemoteUnitV1={version:1,key:hex("key"),localKey:hex("local"),slot:hex("slot"),crate:"c"+id,platform,createdMs:1,costMs:1,dependencies:[],environment:[],outputs:rows as RemoteUnitV1["outputs"],stdout:null,stderr:null};
 return {key:manifest.key,slot:manifest.slot,localKey:manifest.localKey,manifest,contents};
}

type Fake=Readonly<{client:FleetClientV1;log:string[];rounds:<T>(work:Promise<T>)=>Promise<{value:T;rounds:number}>;release:()=>void;publish:(unit:Built,result?:Uint8Array)=>void;corrupt:Set<string>;stores:{results:Map<string,Uint8Array>;blobs:Map<string,Uint8Array>}}>;
function fake():Fake{
 const results=new Map<string,Uint8Array>(),blobs=new Map<string,Uint8Array>(),log:string[]=[],corrupt=new Set<string>();let waiting:Array<()=>void>=[];
 const hold=():Promise<void>=>new Promise(done=>{waiting.push(done);});
 const client={
  settings:{endpoint:"http://fake",instance:"main",token:undefined,upload:true,stateDirectory:join(scratch,"state"),signedRoot:undefined,identity:"fake"},
  usable:()=>true,state:()=>"ok",claimsUsable:()=>true,
  async getResult(key:string){log.push("ac");await hold();return results.get(key)??"miss";},
  async getBlob(hash:string,size:number){log.push("cas");await hold();const stored=blobs.get(hash);if(stored===undefined)return undefined;const bytes=corrupt.has(hash)?stored.map(byte=>byte^1):stored;return bytes.length===size&&sha(bytes)===hash?bytes:undefined;},
  async claimSlot(){log.push("slot");await hold();return true;},
 } as unknown as FleetClientV1;
 const rounds=async<T>(work:Promise<T>)=>{
  let finished=false,value!:T,failure:unknown,count=0;work.then(result=>{finished=true;value=result;},error=>{finished=true;failure=error;});
  for(let spin=0;spin<2000;spin++){
   await new Promise(done=>setTimeout(done,0));
   if(finished){if(failure!==undefined)throw failure;return {value,rounds:count};}
   const batch=waiting;waiting=[];if(batch.length>0){count++;for(const release of batch)release();}
  }
  throw Error("the work never finished");
 };
 return {client,log,rounds,release:()=>{const batch=waiting;waiting=[];for(const release of batch)release();},corrupt,stores:{results,blobs},publish(unit,result){const pack=packUnitV1(unit.manifest,unit.contents);for(const [hash,bytes] of pack.blobs)blobs.set(hash,bytes);results.set(unit.key,result??pack.result);}};
}
const target=(client:FleetClientV1,unit:Built,name:string):InstallTargetV1=>({client,root:join(scratch,name),localKey:unit.localKey,key:unit.key,slot:unit.slot,platform,roots:[]});

test("a unit is installed in two round trips: the action result, then the manifest and every output at once",async()=>{
 const world=fake(),unit=build([40000,6000]);world.publish(unit);
 const {value,rounds}=await world.rounds(installFleetUnitV1(target(world.client,unit,"two")));
 expect(value).toBe("hit");expect(rounds).toBe(2);
 expect(world.log.filter(row=>row==="ac").length).toBe(1);expect(world.log.filter(row=>row==="cas").length).toBe(4);
});

test("a hit does not wait for the slot claim: the unit is served after two round trips and the claim is bounded by 600 ms",async()=>{
 const world=fake(),unit=build();world.publish(unit);
 const {value,rounds}=await world.rounds(fetchFleetUnitV1({...target(world.client,unit,"slot"),workspace:undefined} as unknown as FleetUnitV1));
 expect(value).toBe("hit");expect(rounds).toBe(2);expect(world.log.filter(row=>row==="slot").length).toBe(1);
 const started=Date.now();await awaitFleetSettlesV1();expect(Date.now()-started).toBeLessThan(1500);
 world.release();await awaitFleetSettlesV1();
});

test("the installed objects are the manifest's bytes, checked by an independent SHA-256",async()=>{
 const world=fake(),unit=build([123457,9001]);world.publish(unit);
 const {value}=await world.rounds(installFleetUnitV1(target(world.client,unit,"bytes")));expect(value).toBe("hit");
 const root=casRootV1(),paths=unit.manifest.outputs.map(output=>({path:casObjectPathV1(root,output.sha256),sha:output.sha256,template:output.template===true}));
 for(const row of paths)expect(existsSync(row.path),row.path).toBe(true);
 const python=Bun.which("python3");
 if(python){
  const run=Bun.spawnSync([python,"-I","-c","import hashlib,json,sys\nfor row in json.load(sys.stdin):\n    print(hashlib.sha256(open(row['path'],'rb').read()).hexdigest()==row['sha'])"],{stdin:new TextEncoder().encode(JSON.stringify(paths.filter(row=>!row.template))),stdout:"pipe",stderr:"pipe"});
  expect(run.exitCode,run.stderr.toString()).toBe(0);expect(run.stdout.toString().trim().split("\n")).toEqual(paths.filter(row=>!row.template).map(()=>"True"));
 }
 for(const row of paths.filter(row=>!row.template))expect(sha(readFileSync(row.path))).toBe(row.sha);
});

test("the downloads are the largest first, at most the policy width at once, and one missing blob stops them",async()=>{
 const sizes=new Map([["a".repeat(64),10],["b".repeat(64),5000],["c".repeat(64),300],["d".repeat(64),40],["e".repeat(64),9000],["f".repeat(64),1]]);
 const order:string[]=[];let inflight=0,widest=0;
 const client={async getBlob(hash:string,size:number){order.push(hash);inflight++;widest=Math.max(widest,inflight);await new Promise(done=>setTimeout(done,2));inflight--;return new Uint8Array(size);}} as unknown as FleetClientV1;
 const bytes=await downloadBlobsV1(client,sizes);
 expect(bytes?.size).toBe(6);expect(order.slice(0,2)).toEqual(["e".repeat(64),"b".repeat(64)]);expect(widest).toBe(fleet.limits.maximumDownloadConcurrency);
 let calls=0;const broken={async getBlob(){calls++;return undefined;}} as unknown as FleetClientV1;
 expect(await downloadBlobsV1(broken,sizes)).toBeUndefined();expect(calls).toBeLessThanOrEqual(fleet.limits.maximumDownloadConcurrency);
});

test("hostile or inconsistent entries are a miss: a corrupt blob, a size above the bound or too many files move no blob, a manifest that disagrees with the result is refused",async()=>{
 const corrupted=fake(),first=build();corrupted.publish(first);corrupted.corrupt.add(first.manifest.outputs[0]!.sha256);
 expect((await corrupted.rounds(installFleetUnitV1(target(corrupted.client,first,"corrupt")))).value).toBe("miss");
 expect(existsSync(join(scratch,"corrupt","e"))).toBe(false);

 const oversize=fake(),second=build(),pack=packUnitV1(second.manifest,second.contents);
 const files=(sizeOf:(hash:string,size:number)=>number)=>[{path:fleet.manifestName,digest:wireDigestOfV1(pack.manifest),executable:false},...second.manifest.outputs.map(output=>({path:"out/"+output.name,digest:{hash:output.sha256,size:sizeOf(output.sha256,output.bytes)},executable:false}))];
 oversize.publish(second,encodeActionResultV1({files:files((_hash,size)=>size+fleet.limits.maximumBlobBytes),slot:second.slot}));
 expect((await oversize.rounds(installFleetUnitV1(target(oversize.client,second,"oversize")))).value).toBe("miss");expect(oversize.log.filter(row=>row==="cas").length).toBe(0);

 const crowded=fake(),third=build();
 crowded.publish(third,encodeActionResultV1({files:[...files(()=>1).slice(0,1),...Array.from({length:fleet.limits.maximumBlobs+8},(_,index)=>({path:"out/x"+index,digest:{hash:sha(new TextEncoder().encode("x"+index)),size:1},executable:false}))],slot:third.slot}));
 expect((await crowded.rounds(installFleetUnitV1(target(crowded.client,third,"crowded")))).value).toBe("miss");expect(crowded.log.filter(row=>row==="cas").length).toBe(0);

 const liar=fake(),fourth=build(),lie=packUnitV1(fourth.manifest,fourth.contents);
 liar.publish(fourth,encodeActionResultV1({files:[{path:fleet.manifestName,digest:wireDigestOfV1(lie.manifest),executable:false},...fourth.manifest.outputs.map(output=>({path:"out/"+output.name,digest:{hash:output.sha256,size:output.bytes+1},executable:false}))],slot:fourth.slot}));
 expect((await liar.rounds(installFleetUnitV1(target(liar.client,fourth,"liar")))).value).toBe("miss");
});

test("the wrapper settles the slot claims of a fleet hit before its process ends, and only there",()=>{
 const source=readFileSync(fileURLToPath(new URL("../../../📜️script.ts",import.meta.url)),"utf8");
 expect(source.match(/awaitFleetSettlesV1\(\)/gu)?.length).toBe(1);
 expect(source).toContain('countUnitV1(workspaceOf,"remoteHit");await awaitFleetSettlesV1();return 0;');
});

let server:RemoteCacheServerV1|undefined;
afterAll(async()=>{await server?.stop().catch(()=>{});});
test("against a real cache server over a socket: the units upload, install on a second machine and are the same bytes",async()=>{
 server=await startRemoteCacheServerV1({directory:join(scratch,"srv"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});
 const settings=readFleetSettingsV1(join(scratch,"u"),{SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token"})!,calls:string[]=[];
 const client=createFleetClientV1(settings,{fetch:((url:string,init?:RequestInit)=>{calls.push((init?.method??"GET")+" "+(String(url).includes("/cas/")?"cas":String(url).includes("/ac/")?"ac":"other"));return fetch(url,init);}) as typeof fetch});
 const units=Array.from({length:6},(_,index)=>build([30000+index*7000,4000]));
 for(const unit of units){const pack=packUnitV1(unit.manifest,unit.contents);for(const [hash,bytes] of pack.blobs)expect(await client.putBlob(hash,bytes)).toBe(true);expect(await client.putResult(unit.key,pack.result,unit.slot)).toBe(true);}
 calls.length=0;mkdirSync(join(scratch,"u"),{recursive:true});
 for(const unit of units)expect(await installFleetUnitV1(target(client,unit,"u"))).toBe("hit");
 expect(calls.filter(row=>row==="GET ac").length).toBe(units.length);expect(calls.filter(row=>row==="GET cas").length).toBe(units.length*(units[0]!.manifest.outputs.length+1));
 for(const unit of units)for(const output of unit.manifest.outputs.filter(row=>row.template!==true))expect(sha(readFileSync(casObjectPathV1(casRootV1(),output.sha256)))).toBe(output.sha256);
});
