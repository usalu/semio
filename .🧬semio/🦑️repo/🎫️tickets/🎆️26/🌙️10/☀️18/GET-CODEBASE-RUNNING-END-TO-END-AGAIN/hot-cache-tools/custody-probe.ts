// ticket tool: how much of the per-command fixed cost is store custody, measured on the real store code in a scratch workspace
import {pathToFileURL} from "node:url";
const live="/Users/ueli/Documents/semio/",L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/",F="🧰️framework/🔨️modules/🏃️process/";
const load=(path:string)=>import(pathToFileURL(live+path).href);
const routing=await load(F+"📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const opening=await load(L+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts");
const session=await load(L+"⚡️caching/💾️storage/🔒️enforcement/🧵️session/🟦️.ts");
const [,, root, mode, countText, partitionsText="leases"]=process.argv,count=Number(countText??"10"),partitions=partitionsText.split(",");
const policy=routing.readScriptPolicy(Object.freeze({version:1,owner:"repository:build",maximumElapsedMilliseconds:900000}));
const envelope=routing.createScriptProcessEnvelope(policy,{probe:true},Date.now());
const rows:number[]=[];
const openAll=async(invocation:any,control:any,at=0):Promise<void>=>{
  if(at>=partitions.length)return;
  await opening.withRepositoryCacheStoreV1(root,partitions[at],invocation,control,async(_store:any,actual:any)=>openAll(invocation,actual,at+1));
};
if(mode==="cold"){
  for(let i=0;i<count;i++){
    const t=performance.now();
    await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope(policy,{probe:true},Date.now()),async(invocation:any)=>{
      const control=routing.createIssuedResourceControlV1(invocation,"probe",0);
      await openAll(invocation,control);
    });
    rows.push(performance.now()-t);
  }
}else{
  await routing.withScriptProcessEnvelope(envelope,async(invocation:any)=>{
    const control=routing.createIssuedResourceControlV1(invocation,"probe",0);
    const t0=performance.now();
    const held=await session.openRepositoryCustodySessionV1({workspace:root,invocation,control,partitions});
    rows.push(performance.now()-t0);
    for(let i=0;i<count;i++){
      const t=performance.now();
      await held.run(async(inner:any)=>{await openAll(invocation,inner);});
      rows.push(performance.now()-t);
    }
    await held.close();
  });
}
console.log(JSON.stringify({mode,partitions,rowsMs:rows.map(v=>Math.round(v*10)/10),processAgeMs:Math.round(performance.now())}));
