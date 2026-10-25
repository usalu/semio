const root="/Users/ueli/Documents/semio",L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/";
const a=await import(L+"🟦️.ts"),snap=await import(L+"📦️snapshot/🟦️.ts");
const result=await a.captureNxAuthorizerSourcesV1(root,process.argv[2]!,new AbortController().signal);
const snapshot=snap.snapshotNxAuthorizerSourcesV1(result);
const files=new Map(snapshot.files.map((r:any)=>[r.path,Buffer.from(r.base64,"base64")]));
const maps=a.nxAuthorizerImportProjectionV1(root,files,snapshot.manifest.resolutions);
const rows=snap.capturedSourceImagesV1(snapshot.manifest,files,maps);
const withText=rows.filter((r:any)=>r.text!==undefined);
console.log(JSON.stringify({sources:rows.length,withText:withText.length,textBytes:withText.reduce((s:number,r:any)=>s+Buffer.byteLength(r.text),0),bazel:rows.filter((r:any)=>r.path.includes("generations/🟨️.mjs")).map((r:any)=>r.text!==undefined)}));
