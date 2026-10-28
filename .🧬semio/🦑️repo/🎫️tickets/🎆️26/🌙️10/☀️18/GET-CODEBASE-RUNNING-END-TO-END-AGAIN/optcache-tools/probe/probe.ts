import{test,expect}from"bun:test";
import{mkdtemp,rm,realpath,readdir,lstat}from"node:fs/promises";
import{existsSync}from"node:fs";
import{join,resolve}from"node:path";
import{createNativeBuildCallerCapabilitiesV1}from"../../../📥️caller/🟦️.ts";
import{createIssuedResourceControlV1,createScriptProcessEnvelope,withScriptProcessEnvelope}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{repositoryCacheBudgetV1}from"../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts";
import{repositoryLeaseDirectoryV1}from"../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import{openNativePlatformStorageV1,retireNativePlatformStorageV1,requireNativeStoragePathV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts";
import{nativeImageServiceEntryV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import{recoverNativeStorageReservationV1,observeNativeStorageStateV1,nativeStorageRetirementPortV1}from"@semio-tech/process/artifacts/native-build/storage";
import{withRepositoryBuildGraphFactStoreV1}from"../../../📥️inputs/💾️storage/🟨️.mjs";
async function retire(options:any){
 const image=await openNativePlatformStorageV1(options);let primary:any;
 try{const held=await image.withExclusiveMaintenance({version:1,wait:false},async(port:any)=>{
  const policy={version:1 as const,root:image.root,leaseDirectory:options.leaseDirectory,capacityBytes:options.maximumPhysicalBytes,maximumWorkspaceBytes:options.filesystemBytes,maximumArtifactBytes:options.filesystemBytes},state=await observeNativeStorageStateV1({policy,control:options.control,port});console.log("[DEBUG] allocations",JSON.stringify(state.allocations.map((r:any)=>r.owner)));let cleared=false;
  const retiring=nativeStorageRetirementPortV1(port,async()=>{if(cleared)return;for(const name of await readdir(image.root)){if(name==="storage-events.jsonl"||nativeImageServiceEntryV1(name))continue;const path=requireNativeStoragePathV1(image.root,join(image.root,name));await rm(path,{recursive:true,force:true});}cleared=true;});
  for(const row of state.allocations){const receipt=await recoverNativeStorageReservationV1({policy,control:options.control,port:retiring,id:row.id});await receipt.retire();}
 });if(!held.acquired)throw Error("image still has readers");}catch(e){primary=e;throw e;}finally{try{await image.close();}catch(e){if(!primary)throw e;}}
 if(!await retireNativePlatformStorageV1(options,{version:1,wait:false}))throw Error("retire refused");
}
test("probe",async()=>{
 const base=await realpath(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!)),ws=await mkdtemp(join(base,"gw-"));console.log("[DEBUG] ws",ws.length);
 const t0=Date.now();
 try{
  await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"graph-fixture-publisher",maximumElapsedMilliseconds:90000},createNativeBuildCallerCapabilitiesV1(ws,["show","projects"],"nx",process.env),Date.now()),async original=>{
   await withRepositoryBuildGraphFactStoreV1(ws,original,createIssuedResourceControlV1(original,"graph:fixture-writer",0),async()=>{console.log("[DEBUG] writer entered",Date.now()-t0);});
   console.log("[DEBUG] writer done",Date.now()-t0);
   const budget=repositoryCacheBudgetV1(ws),control=createIssuedResourceControlV1(original,"graph:private-retire",0);
   const g=budget.partitions.graph,l=budget.partitions.leases;
   await retire({...g,maximumPhysicalBytes:g.capacityBytes,leaseDirectory:repositoryLeaseDirectoryV1(ws),control});console.log("[DEBUG] graph retired",Date.now()-t0);
   await retire({...l,maximumPhysicalBytes:l.capacityBytes,maximumLeaseDatabaseBytes:16384,leaseDirectory:budget.controlDirectory,control});console.log("[DEBUG] leases retired",Date.now()-t0);
  });
 }catch(e:any){console.log("[DEBUG] ERR",String(e?.stack??e).slice(0,1500));throw e;}finally{try{await rm(ws,{recursive:true,force:true});}catch(e){console.log("[DEBUG] rm failed",String(e).slice(0,200));}console.log("[DEBUG] exists",existsSync(ws),Date.now()-t0);}
},170000);
