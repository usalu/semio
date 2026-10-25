const root="/Users/ueli/Documents/semio",L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/";
const a=await import(L+"🟦️.ts"),snap=await import(L+"📦️snapshot/🟦️.ts");
const result=await a.captureNxAuthorizerSourcesV1(root,process.argv[2]!,new AbortController().signal);
const snapshot=snap.snapshotNxAuthorizerSourcesV1(result);
const files=new Map(snapshot.files.map((r:any)=>[r.path,Buffer.from(r.base64,"base64")]));
const maps=a.nxAuthorizerImportProjectionV1(root,files,snapshot.manifest.resolutions);
const imported=new Set<string>();let shown=0;for(const [k,v] of maps){if(shown++<1)console.log(k.slice(-40),JSON.stringify(v).slice(0,300));for(const i of Object.values(v as any))imported.add(String(i));}
const data=snapshot.manifest.sources.filter((r:any)=>!imported.has(r.path)&&r.path!==snapshot.manifest.entry);
console.log(JSON.stringify({sources:snapshot.manifest.sources.length,data:data.length,bytes:data.reduce((s:number,r:any)=>s+files.get(r.path)!.length,0),sample:data.slice(0,12).map((r:any)=>r.path.slice(-70))},null,1));
