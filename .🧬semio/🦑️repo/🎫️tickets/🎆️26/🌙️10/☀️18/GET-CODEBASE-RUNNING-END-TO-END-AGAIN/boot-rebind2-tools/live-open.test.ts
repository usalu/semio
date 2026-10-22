import{test,expect}from"bun:test";
import{withBuildTestInvocationV1}from"/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import{createIssuedResourceControlV1}from"/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{openRepositoryCachePartitionV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
const workspace=process.env.OPEN_WORKSPACE??process.env.LIVE_WORKSPACE??process.cwd();
test("[DEBUG] live open of every cache partition through the generalized preflight",async()=>{
 await withBuildTestInvocationV1(async invocation=>{
  const control=createIssuedResourceControlV1(invocation,"cache:boot-rebind2-live-open",300000);
  for(const id of(process.env.LIVE_PARTITIONS??"leases,graph").split(",")){const started=performance.now();const image=await openRepositoryCachePartitionV1(workspace,id as never,control);console.log("[DEBUG] opened",id,Math.round(performance.now()-started),"ms");await image.close();}
 });
},320000);
