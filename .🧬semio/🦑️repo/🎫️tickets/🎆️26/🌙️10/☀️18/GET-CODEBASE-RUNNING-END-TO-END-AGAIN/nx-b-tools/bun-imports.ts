const root="/Users/ueli/Documents/semio",{captureNxAuthorizerSourcesV1}=await import(root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts");
const result=await captureNxAuthorizerSourcesV1(root,process.argv[2]!,new AbortController().signal);
const {readFileSync}=await import("node:fs");
for(const row of result.manifest.sources){if(!/\.(ts|mjs|js)$/.test(row.path))continue;const text=readFileSync(root+"/"+row.path,"utf8");for(const m of text.matchAll(/(?:from\s*|import\s*\(\s*|import\s+)["'](bun:[^"']+)["']/g))console.log(row.path.slice(-110),m[1]);}
