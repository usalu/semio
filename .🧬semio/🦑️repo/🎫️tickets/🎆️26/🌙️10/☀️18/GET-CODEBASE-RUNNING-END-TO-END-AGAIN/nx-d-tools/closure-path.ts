const root="/Users/ueli/Documents/semio",L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/";
const a=await import(L+"🟦️.ts"),snap=await import(L+"📦️snapshot/🟦️.ts");
const result=await a.captureNxAuthorizerSourcesV1(root,process.argv[2]!,new AbortController().signal);
const snapshot=snap.snapshotNxAuthorizerSourcesV1(result);
const files=new Map(snapshot.files.map((r:any)=>[r.path,Buffer.from(r.base64,"base64")]));
const maps=a.nxAuthorizerImportProjectionV1(root,files,snapshot.manifest.resolutions);
const target=process.argv[3]!,entry=snapshot.manifest.entry,prev=new Map<string,string>([[entry,""]]),queue=[entry];
while(queue.length){const c=queue.shift()!;for(const n of Object.values(maps.get(c)??{}) as string[])if(!prev.has(n)){prev.set(n,c);queue.push(n);}}
const hit=[...prev.keys()].find(k=>k.endsWith(target));
if(!hit)console.log("not reachable");else{const chain=[];for(let c=hit;c;c=prev.get(c)!)chain.push(c.slice(-100));console.log(chain.reverse().join("\n -> "));}
