import {resolve,join} from "node:path";
const root=resolve(import.meta.dir,"../../../../../../../..");
const L=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const {createBuildBootstrapCapabilitiesV1}=await import(join(L,"🏗️build/📥️caller/📤️child/🟦️.ts"));
const {readDefaultBuildCliPolicyV1}=await import(join(L,"🏗️build/📜️script.ts"));
const routing=await import(join(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
const nx=await import(join(L,"⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts"));
const capabilities=await createBuildBootstrapCapabilitiesV1(root,["repo","show","projects"],process.env);
const envelope=routing.createScriptProcessEnvelope(readDefaultBuildCliPolicyV1(),capabilities,Date.now());
const value={version:1,toolingIdentity:"a".repeat(64),workspaceRoot:root,graphIdentity:null,implementationIdentity:"c".repeat(64)};
await routing.withScriptProcessEnvelope(envelope,async (original:any)=>{
  const control=routing.createIssuedResourceControlV1(original,"nxpart:probe",0);
  try{
    await nx.withNxGraphCacheStoreV1(root,original,control,async(store:any)=>{const lease=await nx.acquireNxGraphNamespaceV1(store,value);console.log("ACQ1");await lease.release();console.log("REL1");});
    await nx.withNxGraphCacheStoreV1(root,original,control,async(store:any)=>{const lease=await nx.acquireNxGraphNamespaceV1(store,value);console.log("ACQ2");await lease.recheck();await lease.release();console.log("REL2");
      for(let i=0;i<3;i++){const ok=await nx.retireNxGraphNamespaceV1(store,value);console.log("RETIRE "+i+" "+ok);if(ok)break;await new Promise(r=>setTimeout(r,1000));}
    });
    console.log("DONE");
  }catch(error){console.log("FAILED "+String((error as Error).stack).slice(0,800));process.exitCode=1;}
});
