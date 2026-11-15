import {createHash} from "node:crypto";
import {existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,readlinkSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {startRemoteCacheServerV1} from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {fleetFetchContextV1,pinnedFetchV1} from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🟦️.ts";
import {fleetInstallAcquireV1,fleetInstallKeyV1,type FleetInstallShapeV1} from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/📦️install/🟦️.ts";
import {cargoLockRegistryCratesV1,cargoRegistryCacheDirectoryV1,fleetCratesPublishV1,fleetCratesRestoreV1} from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🦀️crates/🟦️.ts";

export const scratch=process.env.PROOF_SCRATCH??mkdtempSync(join(tmpdir(),"fleet-proof-"));
mkdirSync(scratch,{recursive:true});
export const mb=(bytes:number)=>Math.round(bytes/1048576*10)/10,seconds=(ms:number)=>Math.round(ms/100)/10;
export const size=(path:string):number=>{try{const facts=lstatSync(path);if(!facts.isDirectory())return facts.size;return readdirSync(path).reduce((sum,name)=>sum+size(join(path,name)),0);}catch{return 0;}};
export const run=async(argv:string[],cwd:string,env:Record<string,string|undefined>={})=>{const started=performance.now(),child=Bun.spawn(argv,{cwd,stdout:"pipe",stderr:"pipe",env:{...process.env,...env}}),[out,err,code]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);return {out,err,code,ms:performance.now()-started};};
export const store=join(scratch,"store");mkdirSync(store,{recursive:true});
export const machine=(name:string)=>{
 const root=join(scratch,name,"worktree"),home=join(scratch,name,"home"),ledger=join(scratch,name,"ledger.jsonl");
 mkdirSync(join(root,".cache","f"),{recursive:true});mkdirSync(home,{recursive:true});writeFileSync(join(root,".cache","f","machine.json"),JSON.stringify({version:1,storeRoot:store}));
 return {name,root,home,ledger,context:(remote?:{endpoint:string;instance:string;token:string|undefined},withStore=true)=>fleetFetchContextV1({SEMIO_FLEET_FETCH_LEDGER:ledger},root,{storeRoot:withStore?store:undefined,remote})};
};
export const upstreamBytes=(ledger:string)=>existsSync(ledger)?readFileSync(ledger,"utf8").trim().split("\n").filter(Boolean).map(line=>JSON.parse(line) as {source:string;bytes:number;direction?:string}).filter(row=>row.source==="upstream"&&row.direction==="get").reduce((sum,row)=>sum+row.bytes,0):0;
export const drain=async(response:Response)=>{const hash=createHash("sha256");let bytes=0;for await(const chunk of response.body as unknown as AsyncIterable<Uint8Array>){hash.update(chunk);bytes+=chunk.byteLength;}return {bytes,sha256:hash.digest("hex")};};
export const serve=(mirror=false)=>startRemoteCacheServerV1({directory:join(scratch,"cache"),host:"127.0.0.1",port:0,maximumBytes:2**33,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"proof-write-token"},...(mirror?{mirror:{directory:join(scratch,"mirror")}}:{})});
export const BUN="/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun";
export {fleetInstallAcquireV1,fleetInstallKeyV1,cargoLockRegistryCratesV1,cargoRegistryCacheDirectoryV1,fleetCratesPublishV1,fleetCratesRestoreV1,pinnedFetchV1,readFileSync,writeFileSync,mkdirSync,join,existsSync,lstatSync,readdirSync,readlinkSync,rmSync,createHash};
export type {FleetInstallShapeV1};
