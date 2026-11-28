const T="/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H94/farm/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧰️tool-snapshot/";
const core=await import(T+"🟦️.ts"),gate=await import(T+"🚧️gate/🟦️.ts"),watch=await import(T+"👁️watch/🟦️.ts");
const root="/Users/ueli/Documents/semio",store=core.toolStoreAtV1(process.argv[2],false);
let t=Date.now();
const cap=await core.captureToolTreeV1(root,{putBlob:(s:string,b:Uint8Array)=>core.toolPutBlobV1(store,s,b),sleep:(ms:number)=>new Promise(r=>setTimeout(r,ms))});
if(!("sourceDigest" in cap)){console.log("capture",JSON.stringify(cap));process.exit(1);}
const snap=core.toolSnapshotOfV1(cap);
const code=snap.files.filter((f:any)=>f.kind==="code"),tests=snap.files.filter((f:any)=>f.kind==="test");
console.log("capture ms",Date.now()-t,"attempts",cap.attempts,"code",code.length,"bytes",code.reduce((a:number,f:any)=>a+f.bytes,0),"tests",tests.length,"skipped",cap.skipped,"id",snap.id.slice(0,12));
const ctx={root,store,snapshot:snap,overlay:async()=>root,language:"en",signal:new AbortController().signal,environment:{},policy:core.toolPolicyV1};
t=Date.now();const sy=await gate.toolSyntaxGateV1(ctx);console.log("syntax ms",Date.now()-t,sy.passed,JSON.stringify(sy.rows.slice(0,5)));
t=Date.now();const im=await gate.toolImportsGateV1(ctx);console.log("imports ms",Date.now()-t,im.passed,im.rows.length,JSON.stringify(im.rows.slice(0,6)));
// role closure parity against the live scanner
const sc=await import(root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🟦️.ts");
const B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/";
const roles:Record<string,string>={agent:"👷️agent",scheduler:"🗓️scheduler",cache:"🗄️cache",graph:"🕸️graph",pool:"🏊️pool",ready:"🚦️ready",spool:"📮️spool"};
const scan=(text:string)=>new Bun.Transpiler({loader:"ts"}).scan(text).imports.filter(r=>r.path.startsWith(".")).map(r=>({specifier:r.path,dynamic:r.kind==="dynamic-import"}));
for(const [role,folder] of Object.entries(roles)){
 const mine=watch.toolRoleClosureV1(store,snap,[B+"🟦️.ts",B+folder+"/🟦️.ts"],[B+"🟦️.ts"],scan);
 const live=sc.remoteExecutionSidecarWatchV1(role).closure();
 const liveRel=new Map([...live.files].map(([p,h]:any)=>[p.replace(root+"/",""),h]));
 const onlyMine=[...mine.keys()].filter(p=>!liveRel.has(p)),onlyLive=[...liveRel.keys()].filter(p=>!mine.has(p)&&!/📖️texts\//u.test(p)),differ=[...mine].filter(([p,h])=>liveRel.has(p)&&liveRel.get(p)!==h).length;
 console.log(role,"mine",mine.size,"live",liveRel.size,"onlyMine",onlyMine.length,"onlyLive",onlyLive.length,onlyLive.slice(0,2),"differ",differ);
}
