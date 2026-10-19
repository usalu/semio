import {resolve,join} from "node:path";
const root=resolve(import.meta.dir,"../../../../../../../..");
const L=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const {createBuildBootstrapCapabilitiesV1}=await import(join(L,"🏗️build/📥️caller/📤️child/🟦️.ts"));
const {readDefaultBuildCliPolicyV1}=await import(join(L,"🏗️build/📜️script.ts"));
const routing=await import(join(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
const E=join(L,"⚡️caching/💾️storage/🔒️enforcement");
const store=await import(join(E,"📦️budget/📥️opening/🏪️store/🟦️.ts"));
const budget=await import(join(E,"📦️budget/🟦️.ts"));
const core=await import(join(root,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts"));
const capabilities=await createBuildBootstrapCapabilitiesV1(root,["repo","show","projects"],process.env);
const envelope=routing.createScriptProcessEnvelope(readDefaultBuildCliPolicyV1(),capabilities,Date.now());
const t0=Date.now(),stamp=()=>`+${((Date.now()-t0)/1000).toFixed(1)}s`;
await routing.withScriptProcessEnvelope(envelope,async (original:any)=>{
  const control=routing.createIssuedResourceControlV1(original,"nxpart:retire",0);
  try{
    await store.withRepositoryCacheStoreV1(root,"nx",original,control,async(base:any,phase:any)=>{
      const nx=budget.repositoryCacheBudgetV1(root).partitions.nx,policy={version:1,root:base.root,leaseDirectory:join(root,".cache/b/l/m/data"),capacityBytes:nx.capacityBytes,maximumWorkspaceBytes:nx.filesystemBytes,maximumArtifactBytes:nx.maximumArtifactBytes};
      const state=await core.observeNativeStorageStateV1({policy,control:phase,port:base.image.port});
      console.log("PROBE ALLOCATIONS "+JSON.stringify(state.allocations.map((a:any)=>[a.owner,a.phase,a.bytes])));
      const row=state.allocations.find((a:any)=>a.child);if(!row){console.log("PROBE NO CHILD");return;}
      const reservation=await core.recoverNativeStorageReservationV1({policy,control:phase,port:base.image.port,id:row.id});
      await store.retireRepositoryCacheStoreFundedChildStorageV1(base,reservation,phase,phase);
      const after=await core.observeNativeStorageStateV1({policy,control:phase,port:base.image.port});
      console.log("PROBE RETIRED "+stamp()+" allocations="+JSON.stringify(after.allocations.map((a:any)=>[a.owner,a.phase,a.bytes])));
    });
    console.log("PROBE DONE "+stamp());
  }catch(error){console.log("PROBE FAILED "+stamp()+" "+String((error as Error).stack).slice(0,1500));process.exitCode=1;}
});
