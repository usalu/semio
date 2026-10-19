import{mkdirSync,mkdtempSync,rmSync,readdirSync}from"node:fs";
import{join}from"node:path";
import{receiveScriptProcessInvocation}from"../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{scriptCompilerAdmissionControlV1}from"@semio-tech/process/compiler-capacity";
import{openNativeImageStorageV1,retireNativeImageStorageV1}from"@semio-tech/process/artifacts/native-build/storage/image";
import{acquireNativeStorageV1,nativeStorageRetirementPortV1,recoverNativeStorageReservationV1}from"@semio-tech/process/artifacts/native-build/storage";
import{openRepositoryCachePartitionV1,repositoryLeaseDirectoryV1}from"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import{readPhysicalCargoSourceV1}from"../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/📖️read/🟦️.ts";
import{captureOwnedCommandV1}from"@semio-tech/process/execution";
const N=Number(process.env.BENCH_N||15),log=(...a:unknown[])=>console.log("[DEBUG] bench",...a);
const t=async<T>(label:string,n:number,f:(i:number)=>Promise<T>)=>{const s=performance.now(),times:number[]=[];for(let i=0;i<n;i++){const a=performance.now();await f(i);times.push(performance.now()-a);}times.sort((x,y)=>x-y);log(label,"n="+n,"total="+(performance.now()-s).toFixed(0)+"ms","median="+times[Math.floor(n/2)].toFixed(1)+"ms","max="+times[n-1].toFixed(1)+"ms");};
await receiveScriptProcessInvocation(process.env,async original=>{
 const workspace=process.cwd(),control=scriptCompilerAdmissionControlV1(original,"rust:cargo-pressure-bench",480000),diagnostics=await openRepositoryCachePartitionV1(workspace,"diagnostics",control),leaseDirectory=repositoryLeaseDirectoryV1(workspace),area=join(diagnostics.root,"cp");mkdirSync(area,{recursive:true});
 const parent=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"b-")),directory=join(parent,"i");
 let image:Awaited<ReturnType<typeof openNativeImageStorageV1>>|undefined,storage:Awaited<ReturnType<typeof acquireNativeStorageV1>>|undefined,policy:any;
 try{
  const t0=performance.now();image=await openNativeImageStorageV1({directory,leaseDirectory,filesystemBytes:33554432,maximumPhysicalBytes:34603008,control});log("open image",(performance.now()-t0).toFixed(0)+"ms");
  policy={version:1 as const,root:image.root,leaseDirectory,capacityBytes:image.maximumPhysicalBytes,maximumWorkspaceBytes:25165824,maximumArtifactBytes:1048576};
  const t1=performance.now();storage=await acquireNativeStorageV1({policy,control,port:image.port,owner:"repo:cargo-pressure-bench",kind:"compiler-workspace",bytes:25165824});log("acquire",(performance.now()-t1).toFixed(0)+"ms");
  await t("storage.recheck",N,()=>storage!.recheck());
  const dir="🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission",root=workspace,files=[join(root,dir,"🟦️.ts"),join(root,dir,"📖️read/🟦️.ts"),join(root,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts")];
  await t("readPhysicalCargoSourceV1 (67KB storage file)",N,i=>readPhysicalCargoSourceV1(files[2],{version:1,root,expectedBytes:null,maximumBytes:1<<22,remainingFiles:10},control));
  await t("readPhysicalCargoSourceV1 (8KB file)",N,i=>readPhysicalCargoSourceV1(files[0],{version:1,root,expectedBytes:null,maximumBytes:1<<22,remainingFiles:10},control));
  await t("control.publish+yieldContinuation",N,async()=>{await control.publish({resource:"cargo:source-admission:bench",mode:"shared",elapsedMs:0});await control.yieldContinuation();});
  await t("rustc --print cfg",3,()=>captureOwnedCommandV1("rustc",["--print","cfg"],workspace,{control,env:{...process.env,PATH:process.env.PATH},output:{maximumBytes:1048576,maximumLines:65536}}).catch(e=>log("rustc failed",String(e).slice(0,200))));
 }finally{
  try{if(storage&&policy)await(await recoverNativeStorageReservationV1({policy,control,port:nativeStorageRetirementPortV1(image!.port,async()=>{}),id:storage.allocation.id})).retire();}catch(e){log("retire storage failed",String(e).slice(0,200));}
  try{await image?.close();await retireNativeImageStorageV1({directory,leaseDirectory,control,filesystemBytes:33554432,maximumPhysicalBytes:34603008});}catch(e){log("retire image failed",String(e).slice(0,200));}
  rmSync(parent,{recursive:true,force:true});await diagnostics.close();
 }
});
