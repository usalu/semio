import {cpSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir,homedir} from "node:os";
import {join} from "node:path";
const L="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const {fleetFetchContextV1}=await import(L+"⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🟦️.ts");
const {syncFleetIdentityV1}=await import(L+"🏗️build/🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts");
const {fleetUvPublishV1,fleetUvRestoreV1,fleetUvWheelsV1}=await import(L+"⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🐍️uv/🟦️.ts");
const size=(path:string):number=>{let total=0;try{for(const e of readdirSync(path,{withFileTypes:true}))total+=e.isDirectory()?size(join(path,e.name)):statSync(join(path,e.name)).size;}catch{}return total;};
const root=mkdtempSync(join(tmpdir(),"uv-measure-")),store=join(root,"store"),worktree=join(root,"w");
try{
 mkdirSync(join(worktree,".cache","f"),{recursive:true});writeFileSync(join(worktree,".cache","f","machine.json"),JSON.stringify({version:1,originId:"g1-m01",groupId:"g1",storeRoot:store,key:"k"}));
 await syncFleetIdentityV1({root:worktree});mkdirSync(join(store,"v1","origins","g1-m01"),{recursive:true});cpSync(join(worktree,".cache","f","identity","registration.json"),join(store,"v1","origins","g1-m01","fleet-auth.json"));
 const lock=readFileSync("/Users/Shared/semio/uv.lock","utf8"),donor=join(homedir(),".cache","uv"),context=fleetFetchContextV1({},worktree,{storeRoot:store});
 const wheels=fleetUvWheelsV1(lock);console.log(JSON.stringify({lockedWheels:wheels.length}));
 let t=performance.now();const events:string[]=[];
 const published=await fleetUvPublishV1(context,{cacheDirectory:donor,lock,event:e=>events.push(`${e.stage}:${e.detail}`)});
 console.log(JSON.stringify({published,seconds:(performance.now()-t)/1000,storeMB:Math.round(size(store)/1e6),events}));
 const fresh=join(root,"fresh");t=performance.now();
 const placed=await fleetUvRestoreV1(context,{cacheDirectory:fresh,lock});
 console.log(JSON.stringify({placed,seconds:(performance.now()-t)/1000,freshMB:Math.round(size(fresh)/1e6)}));
}finally{rmSync(root,{recursive:true,force:true});}
