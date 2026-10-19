import {resolve,join} from "node:path";
const root=resolve(import.meta.dir,"../../../../../../../..");
const L=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const {createBuildBootstrapCapabilitiesV1}=await import(join(L,"🏗️build/📥️caller/📤️child/🟦️.ts"));
const {readDefaultBuildCliPolicyV1}=await import(join(L,"🏗️build/📜️script.ts"));
const routing=await import(join(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
const E=join(L,"⚡️caching/💾️storage/🔒️enforcement");
const store=await import(join(E,"📦️budget/📥️opening/🏪️store/🟦️.ts"));
const nx=await import(join(E,"🔌️nx/📦️graph/🟦️.ts"));
const mode=process.argv[2]!,seconds=Number(process.argv[3]??"20"),cancelAfter=Number(process.argv[4]??"0");
const capabilities=await createBuildBootstrapCapabilitiesV1(root,["repo","show","projects"],process.env);
const envelope=routing.createScriptProcessEnvelope(readDefaultBuildCliPolicyV1(),capabilities,Date.now());
const t0=Date.now(),stamp=()=>`+${((Date.now()-t0)/1000).toFixed(1)}s`;
await routing.withScriptProcessEnvelope(envelope,async (original:any)=>{
  if(mode==="hold"){
    await store.withRepositoryCacheStoreV1(root,"graph",original,routing.createIssuedResourceControlV1(original,"lq:hold",0),async()=>{console.log("PROBE HOLDING graph shared "+stamp());await new Promise(done=>setTimeout(done,seconds*1000));});
    console.log("PROBE RELEASED "+stamp());return;
  }
  const abort=new AbortController();if(cancelAfter>0)setTimeout(()=>abort.abort(Object.freeze(Error("lq cancellation"))),cancelAfter*1000);
  console.log("PROBE PREPARING "+stamp());
  try{await nx.prepareNxGraphCacheStorageV1(root,original,routing.createIssuedResourceControlV1(original,"lq:prepare",0,abort.signal),0);console.log("PROBE PREPARED "+stamp());}
  catch(error){console.log("PROBE FAILED "+stamp()+" "+String(error).slice(0,300));process.exitCode=1;}
});
