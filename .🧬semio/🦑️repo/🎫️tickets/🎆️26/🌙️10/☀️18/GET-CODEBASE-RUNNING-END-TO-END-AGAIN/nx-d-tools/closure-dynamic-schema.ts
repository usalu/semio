const root="/Users/ueli/Documents/semio",L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/";
const a=await import(L+"🟦️.ts"),snap=await import(L+"📦️snapshot/🟦️.ts");
const result=await a.captureNxAuthorizerSourcesV1(root,process.argv[2]!,new AbortController().signal);
const snapshot=snap.snapshotNxAuthorizerSourcesV1(result);
const hits:string[]=[];
for(const row of snapshot.files){const text=Buffer.from(row.base64,"base64").toString("utf8");for(const line of text.split("\n")){if(line.includes("schema/")&&/\.json/u.test(line)&&!/^\s*(import|export)\b[^;]*\bwith\s*\{/u.test(line)&&/(readFile|new URL|fs\.|capturePhysical)/u.test(line))hits.push(row.path.slice(-90)+" :: "+line.trim().slice(0,200));}}
console.log(JSON.stringify({sources:snapshot.manifest.sources.length,hits},null,1));
