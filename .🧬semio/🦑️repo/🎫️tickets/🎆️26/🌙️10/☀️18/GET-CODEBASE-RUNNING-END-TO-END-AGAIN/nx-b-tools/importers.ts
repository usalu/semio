const root="/Users/ueli/Documents/semio",{captureNxAuthorizerSourcesV1}=await import(root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts");
const result=await captureNxAuthorizerSourcesV1(root,process.argv[2]!,new AbortController().signal);
const {readFileSync}=await import("node:fs");
for(const row of result.manifest.sources){if(!/\.(ts|mjs|js)$/.test(row.path))continue;const text=readFileSync(root+"/"+row.path,"utf8");if(/runtime\/🍞️bun\/🗃️sqlite|🍞️bun\/🗃️sqlite/.test(text)||/🗃️sqlite\/🟦️/.test(text)){const m=text.match(/.{0,80}🗃️sqlite.{0,40}/);console.log(row.path.slice(-100),"|",m?.[0]);}}
