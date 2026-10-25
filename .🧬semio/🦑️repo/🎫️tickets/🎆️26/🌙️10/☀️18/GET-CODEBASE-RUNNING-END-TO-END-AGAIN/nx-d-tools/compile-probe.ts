const root="/Users/ueli/Documents/semio",L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/",R=root+"/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const a=await import(L+"🟦️.ts"),snap=await import(L+"📦️snapshot/🟦️.ts"),api=await import(R);
const entry=process.argv[2]!;
await api.withScriptProcessEnvelope(api.createScriptProcessEnvelope({version:1,owner:"fixture:compile-probe",maximumElapsedMilliseconds:0},{fixture:"probe"},Date.now()),async original=>{
 const control=api.createIssuedResourceControlV1(original,"fixture:compile-probe",0);
 const result=await a.captureNxAuthorizerSourcesV1(root,entry,new AbortController().signal),snapshot=snap.snapshotNxAuthorizerSourcesV1(result);
 const out=await snap.compileNxAuthorizerSnapshotV1(root,snapshot,control,process.argv[4]??"node",process.argv[5]??"module");
 const path=process.argv[3]!;await Bun.write(path,out.bytes);console.log(JSON.stringify({bytes:out.bytes.length,inputs:out.inputs.length}));
});
