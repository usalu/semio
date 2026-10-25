import{test}from"bun:test";
import{mkdtempSync}from"node:fs";import{join}from"node:path";
import{acquireNativeStorageV1,ordinaryNativeStoragePortV1,requireNativeStorageReservationV1}from"/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import{createScriptProcessEnvelope,createIssuedResourceControlV1,withScriptProcessEnvelope,withIssuedResourceControlV1}from"/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
test("probe",async()=>{
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"gp-")),policy={version:1 as const,root,leaseDirectory:join(root,"leases"),capacityBytes:100,maximumWorkspaceBytes:100,maximumArtifactBytes:100};
 const port:any={...ordinaryNativeStoragePortV1(),withOperation:(a:any,run:any)=>run(a),withJournal:(a:any,run:any)=>run(a),observe:async()=>({root,identity:"gp",maximumPhysicalBytes:100}),recheck:async()=>{},syncMetadata:async()=>{},retire:async()=>{}};
 const out:string[]=[];
 await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"fixture:gp",maximumElapsedMilliseconds:0},{fixture:"gp"},Date.now()),async original=>{
  const issuer=createIssuedResourceControlV1(original,"gp:issuer",0),reservation=await acquireNativeStorageV1({policy,control:issuer,port,owner:"gp",kind:"compiler-workspace",bytes:10});
  const t=(n:string,c:any)=>{try{requireNativeStorageReservationV1(reservation,c);out.push(n+"=accepted");}catch(e){out.push(n+"=refused:"+String(e).slice(0,70));}};
  t("forged-same-signal",Object.freeze({signal:issuer.signal,remainingMilliseconds:()=>1000,publish:async()=>{},yieldContinuation:async()=>{}}));
  t("sibling-top-same-signal",createIssuedResourceControlV1(original,"gp:sibling",0));
  let held:any;await withIssuedResourceControlV1(original,"gp:loan",0,async c=>{held=c;});
  t("retired-loan-same-signal",held);
 });
 console.log("EVT "+out.join(" | "));
});
