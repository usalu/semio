import{mkdtempSync}from"node:fs";import{tmpdir}from"node:os";import{join}from"node:path";import{createHash}from"node:crypto";
const T=process.env.TREE!;const O=T+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";const OP=O+"/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool";
const {startRemoteCacheServerV1}=await import(O+"/🌐️remote-cache/🖥️server/🟦️.ts");
const {graphFleetClientV1}=await import(O+"/🪪️graph-state/🛰️exchange/🟦️.ts");const {graphFleetSettingsV1}=await import(O+"/🪪️graph-state/🧶️nx/🟦️.ts");const {remoteCacheFetchV1}=await import(O+"/🌐️remote-cache/🛰️fleet/🟦️.ts");
const pool=await import(OP+"/🟦️.ts");const {runOwnerPoolV1}=await import(OP+"/🏃️runner/🟨️.mjs");
const children=Number(process.argv[2]),count=Number(process.argv[3]),burn=Number(process.argv[4]);
const dir=mkdtempSync(join(tmpdir(),"pb-")),server=await startRemoteCacheServerV1({directory:dir,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
const env={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
const root=mkdtempSync(join(tmpdir(),"pbr-")),client=graphFleetClientV1(graphFleetSettingsV1(root,env)!,{fetch:remoteCacheFetchV1(env,{root})});
const units=Array.from({length:count},(_a,i)=>({name:`owner-${String(i).padStart(4,"0")}`,kind:"k",expectedMilliseconds:400,json:{name:"o"+i}}));
const partial={version:1 as const,identity:"i",globals:{projectAuthority:"a",registryDigest:"r",commandRevision:"c",late:false},publisher:"p"};
const head={...partial,id:pool.ownerPoolIdV1(partial,units),createdMs:Date.now()};
await pool.publishOwnerPoolV1({client,owner:"o",manifest:{head,session:Buffer.from(JSON.stringify({burnMilliseconds:burn})),units}});
const t0=Date.now();const log:string[]=[];let firstDone=0,lastDone=0,ready=0;
await runOwnerPoolV1({root,environment:{...process.env,...env},signal:new AbortController().signal,capacity:async()=>({children,reason:null}),once:true,driver:T+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧪️tests/🧫️fixtures/🧮️driver.mjs",priority:0,targetPollMilliseconds:100,pollMilliseconds:500,
 observe:(id:number,line:any,at:number)=>{const t=at-t0;if(line.t==="ready"){ready++;log.push(`${t} child${id} ready open=${line.openMilliseconds}`);}if(line.t==="done"){if(!firstDone)firstDone=t;lastDone=t;}if(line.t==="end")log.push(`${t} child${id} end ${line.reason} attempts=${line.stats?.attempts}`);}});
console.log(JSON.stringify({children,count,burn,total:Date.now()-t0,firstDone,lastDone,perMinute:Math.round(count/((lastDone-firstDone)/60000))}));console.log(log.join("\n"));
await server.stop();process.exit(0);
