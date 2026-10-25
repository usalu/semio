import{mkdirSync}from"node:fs";
import{join}from"node:path";
const P="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/";
const[directory,leaseDirectory,filesystemBytes,maximumPhysicalBytes]=process.argv.slice(2);
const{receiveScriptProcessInvocation,createIssuedResourceControlV1}=await import(P+"📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),{openNativePlatformStorageV1}=await import(P+"📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts"),leases=await import(P+"🔒️leases/🟦️.ts");
await receiveScriptProcessInvocation(process.env,async original=>{
 const control=createIssuedResourceControlV1(original,"swap:holder",1800000),image=await openNativePlatformStorageV1({directory:directory!,leaseDirectory:leaseDirectory!,filesystemBytes:Number(filesystemBytes),maximumPhysicalBytes:Number(maximumPhysicalBytes),control,maximumLeaseDatabaseBytes:16384});
 const data=join(image.root,"data");mkdirSync(data,{recursive:true});
 const held=await leases.acquireQueuedResourceLease({directory:data,resource:"swap:held",mode:"exclusive",owner:"holder",signal:control.signal,control});
 console.log("READY "+data);
 await new Promise<void>(resolve=>{process.stdin.on("end",resolve);process.stdin.on("close",resolve);process.stdin.resume();});
 held.release();await image.close();console.log("CLOSED");
});
