import{test,expect}from"bun:test";
import{mkdirSync,mkdtempSync,rmSync,utimesSync,readdirSync}from"node:fs";
import{join}from"node:path";
const root="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/";
test("burst of 3000 idle queued entries sweeps in bounded slices",async()=>{
 const{withBuildTestInvocationV1}=await import(root+"🧪️testing/📥️invocation/🟦️.ts"),{createIssuedResourceControlV1}=await import(root+"📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),leases=await import(root+"🔒️leases/🟦️.ts"),{sweepResourceLeaseStoreV1}=await import(root+"🔒️leases/♻️retirement/🧹️sweep/🟦️.ts");
 await withBuildTestInvocationV1(async original=>{
  const scratch=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"bu-")),directory=join(scratch,"d"),control=createIssuedResourceControlV1(original,"lease:burst",600000);mkdirSync(directory);
  try{
   const started=Date.now();
   for(let i=0;i<3000;i++){const lease=await leases.acquireQueuedResourceLease({directory,resource:"burst:"+i,mode:"exclusive",owner:"b",signal:control.signal,control,timeoutMs:10000});lease.release();}
   console.log("created 3000 queued leases in",Date.now()-started,"ms; files",readdirSync(directory).length,"claims",readdirSync(directory+"/claims").length);
   const old=new Date(Date.now()-7200000);for(const name of readdirSync(directory)){try{utimesSync(join(directory,name),old,old);}catch{}}
   for(let round=1;round<=6;round++){const t=Date.now(),sweep=await sweepResourceLeaseStoreV1(directory,control,{freeRatio:0.1});console.log("sweep",round,JSON.stringify({ms:Date.now()-t,databases:sweep.databases,queues:sweep.queues,claims:sweep.claims,slices:sweep.slices,complete:sweep.complete,busy:sweep.busy}),"left",readdirSync(directory).length,"claims",readdirSync(directory+"/claims").length);if(readdirSync(directory).length<4&&readdirSync(directory+"/claims").length===0)break;}
  }finally{rmSync(scratch,{recursive:true,force:true});}
 });
},600000);
