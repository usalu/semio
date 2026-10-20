import{mkdirSync,mkdtempSync,rmSync,readdirSync,existsSync}from"node:fs";
import{join}from"node:path";
import{receiveScriptProcessInvocation}from"../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{scriptCompilerAdmissionControlV1}from"@semio-tech/process/compiler-capacity";
import{openNativeImageStorageV1,retireNativeImageStorageV1}from"@semio-tech/process/artifacts/native-build/storage/image";
import{acquireNativeStorageV1,nativeStorageRetirementPortV1,recoverNativeStorageReservationV1}from"@semio-tech/process/artifacts/native-build/storage";
import{openRepositoryCachePartitionV1,repositoryLeaseDirectoryV1}from"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import{readPhysicalCargoSourceV1}from"../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/📖️read/🟦️.ts";
import{captureOwnedCommandV1}from"@semio-tech/process/execution";
const N=Number(process.env.BENCH_N||15),log=(...a:unknown[])=>console.log("[bench]",...a);
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
  const mirror=process.env.BENCH_MIRROR!,adm="🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts",oldApi=await import(join(workspace,adm)),newApi=await import(join(mirror,adm));
  const nb=join(workspace,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build"),pool=readdirSync(nb).map(n=>join(nb,n,"🟦️.ts")).filter(f=>existsSync(f));
  const files=pool.slice(0,Number(process.env.BENCH_FILES||12)),request={version:1 as const,root:workspace,expectedBytes:null,maximumBytes:1<<24,remainingFiles:1000},unit:any={compilerArguments:[],compilerFlags:[],features:[],platform:null,kind:"lib"};
  log("files",files.length);
  const oldPort=oldApi.actualCargoSourceAdmissionPortV1(process.env,workspace,storage),newPort=newApi.actualCargoSourceAdmissionPortV1(process.env,workspace,storage);
  const s0=performance.now();await oldPort.cfg(unit,control);for(const f of files)await oldPort.read(f,control,request);const oldMs=performance.now()-s0;log("OLD unit (1 cfg + "+files.length+" reads)",oldMs.toFixed(0)+"ms");
  const s1=performance.now();await newPort.recheck(control,"unit");await newPort.cfg(unit,control);for(const f of files)await newPort.read(f,control,request);await newPort.recheck(control,"complete");const newMs=performance.now()-s1;log("NEW unit (2 rechecks + 1 cfg + "+files.length+" reads)",newMs.toFixed(0)+"ms");
  const s2=performance.now();await oldPort.cfg(unit,control);log("OLD second cfg same key",(performance.now()-s2).toFixed(0)+"ms");
  const s3=performance.now();await newPort.cfg(unit,control);log("NEW second cfg same key (memo)",(performance.now()-s3).toFixed(0)+"ms");
 }finally{
  try{if(storage&&policy)await(await recoverNativeStorageReservationV1({policy,control,port:nativeStorageRetirementPortV1(image!.port,async()=>{}),id:storage.allocation.id})).retire();}catch(e){log("retire storage failed",String(e).slice(0,200));}
  try{await image?.close();await retireNativeImageStorageV1({directory,leaseDirectory,control,filesystemBytes:33554432,maximumPhysicalBytes:34603008});}catch(e){log("retire image failed",String(e).slice(0,200));}
  rmSync(parent,{recursive:true,force:true});await diagnostics.close();
 }
});
