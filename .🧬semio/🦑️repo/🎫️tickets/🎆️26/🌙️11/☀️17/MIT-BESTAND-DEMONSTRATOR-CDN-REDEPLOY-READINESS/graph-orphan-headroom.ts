import {readdirSync} from "node:fs";
import {join,resolve} from "node:path";
import {receiveScriptProcessInvocation} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import {acquireResourceLease,checkResourceLeaseControl} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {nativeImageSpaceV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {repositoryLeaseDirectoryV1,withRepositoryPartitionOperationV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {openRepositoryCachePartitionV1,reclaimImageIfQuiescentV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts";
import {retireOrphanSelectedAdmissionsV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🟦️.ts";
import {buildGraphPublicationWriterResourceV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🔒️writer/🟨️.mjs";

await receiveScriptProcessInvocation(process.env,async invocation=>{
 const root=resolve(process.cwd()),leaseDirectory=repositoryLeaseDirectoryV1(root);
 const control=scriptCompilerAdmissionControlV1(invocation,"graph:orphan-headroom",180_000);
 const collector=await acquireResourceLease({directory:leaseDirectory,resource:"repository-graph-optional-collector:"+root,mode:"exclusive",timeoutMs:0,signal:control.signal,control});
 try{
  const image=await openRepositoryCachePartitionV1(root,"graph",control);
  try{
   await withRepositoryPartitionOperationV1(image,control,async(owned,actual)=>{
    const available=()=>{const volume=nativeImageSpaceV1(owned.root,owned.filesystemBytes);return volume.bavail*volume.bsize;};
    const before=available(),directory=join(owned.root,"selected");
    const result=await retireOrphanSelectedAdmissionsV1(owned,directory,readdirSync(directory),actual,async(scope,body)=>{
     const resource=buildGraphPublicationWriterResourceV1(owned.root,scope);
     try{
      const lease=await acquireResourceLease({directory:leaseDirectory,resource,mode:"exclusive",timeoutMs:0,signal:actual.signal,control:actual});
      try{return await body();}finally{lease.release();}
     }catch(error){
      checkResourceLeaseControl(actual);
      if(error instanceof Error&&error.message==="Resource lease timed out: "+resource)return;
      throw error;
     }
    },()=>available()>=3.75*1024**3);
    console.log(JSON.stringify({phase:"orphan-retirement",before,after:available(),result}));
   });
  }finally{await image.close();}
 }finally{collector.release();}
 const compact=await reclaimImageIfQuiescentV1(invocation,control,"graph",(done,total)=>console.log(JSON.stringify({phase:"compact-progress",done,total})));
 console.log(JSON.stringify({phase:"compaction",result:compact}));
});
