import {cpSync,mkdirSync,readdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {fleetFetchContextV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🟦️.ts";
import {fleetInstallRestoreV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/📦️install/🟦️.ts";

/** 🧪️ Second-machine restore on this Mac: scratch HOME, scratch worktree path, an empty generation directory; the pack comes from the real fleet store. Prints one JSON line. Arguments: store, manifest file, donor generation (only its non-node_modules files are used), scratch directory. */
const [store,manifestFile,donor,scratch]=process.argv.slice(2) as [string,string,string,string];
const manifest=JSON.parse(readFileSync(manifestFile,"utf8"));
const worktree=join(scratch,"checkout"),generation=join(scratch,"gen"),home=join(scratch,"home");
mkdirSync(join(worktree,".cache","f"),{recursive:true});mkdirSync(home,{recursive:true});
writeFileSync(join(worktree,".cache","f","machine.json"),JSON.stringify({version:1,originId:"g1-m09",groupId:"g1",storeRoot:store,key:"k"}));
mkdirSync(generation,{recursive:true});
for(const name of readdirSync(donor)){if(name==="node_modules")continue;cpSync(join(donor,name),join(generation,name),{recursive:true,verbatimSymlinks:true,filter:source=>!source.includes("/node_modules")});}
const events:string[]=[];
const context=fleetFetchContextV1({},worktree,{storeRoot:store});
const started=performance.now();
const restored=await fleetInstallRestoreV1({context,projectionSha256:manifest.projectionSha256,runtime:manifest.runtime,directory:generation,storeRoot:worktree,roots:manifest.roots,check:()=>{},scan:async()=>manifest.shape,remove:async path=>{rmSync(path,{recursive:true,force:true});},installed:()=>manifest.installed,event:event=>events.push(`${event.stage}:${event.detail}`)});
console.log(JSON.stringify({restored,seconds:(performance.now()-started)/1000,events,generation,home}));
