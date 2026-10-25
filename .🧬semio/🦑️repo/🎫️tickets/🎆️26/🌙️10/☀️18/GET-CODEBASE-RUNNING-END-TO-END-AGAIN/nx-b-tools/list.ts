const root="/Users/ueli/Documents/semio",{captureNxAuthorizerSourcesV1}=await import(root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts");
const result=await captureNxAuthorizerSourcesV1(root,process.argv[2]!,new AbortController().signal);
console.log(result.manifest.sources.map((r:{path:string})=>r.path).join("\n"));
