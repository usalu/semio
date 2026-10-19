import {resolve,join} from "node:path";
const root=resolve(import.meta.dir,"../../../../../../../..");
const L=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const {createBuildBootstrapCapabilitiesV1}=await import(join(L,"🏗️build/📥️caller/📤️child/🟦️.ts"));
const {readDefaultBuildCliPolicyV1}=await import(join(L,"🏗️build/📜️script.ts"));
const routing=await import(join(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
const nx=await import(join(L,"⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts"));
const capabilities=await createBuildBootstrapCapabilitiesV1(root,["repo","show","projects"],process.env);
const envelope=routing.createScriptProcessEnvelope(readDefaultBuildCliPolicyV1(),capabilities,Date.now());
const t0=Date.now(),stamp=()=>`+${((Date.now()-t0)/1000).toFixed(1)}s`;
await routing.withScriptProcessEnvelope(envelope,async (original:any)=>{
  const control=routing.createIssuedResourceControlV1(original,"nxpart:probe",0);
  try{
    await nx.prepareNxGraphCacheStorageV1(root,original,control,0);console.log("PROBE PREPARED "+stamp());
    await nx.withNxGraphCacheStoreV1(root,original,control,async(store:any)=>{
      console.log("PROBE STORE OPEN "+stamp()+" root="+store.root);
      const lease=await nx.acquireNxGraphNamespaceV1(store,{version:1,toolingIdentity:"a".repeat(64),workspaceRoot:root,graphIdentity:null,implementationIdentity:"c".repeat(64)});
      console.log("PROBE NAMESPACE ACQUIRED "+stamp()+" native="+lease.nativeDirectory);
      await lease.recheck();console.log("PROBE NAMESPACE RECHECK OK "+stamp());
      await store.recheck();console.log("PROBE STORE RECHECK OK "+stamp());
      await lease.release();console.log("PROBE NAMESPACE RELEASED "+stamp());const ok=await nx.retireNxGraphNamespaceV1(store,{version:1,toolingIdentity:"a".repeat(64),workspaceRoot:root,graphIdentity:null,implementationIdentity:"c".repeat(64)});console.log("PROBE RETIRE "+ok);
    });
    console.log("PROBE DONE "+stamp());
  }catch(error){console.log("PROBE FAILED "+stamp()+" "+String((error as Error).stack).slice(0,1500));process.exitCode=1;}
});
