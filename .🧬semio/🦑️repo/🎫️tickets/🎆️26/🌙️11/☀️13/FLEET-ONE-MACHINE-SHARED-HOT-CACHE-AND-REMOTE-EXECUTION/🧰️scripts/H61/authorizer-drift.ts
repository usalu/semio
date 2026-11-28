import {writeFileSync,readFileSync,existsSync} from "node:fs";
import {captureNxAuthorizerSourcesV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
const root="/Users/ueli/Documents/semio",recipe="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools",out=process.argv[2]!,previous=process.argv[3];
const started=performance.now(),captured=await captureNxAuthorizerSourcesV1(root,recipe+"/🔏️authorizers/📦️runtime/🟦️.ts",new AbortController().signal);
const rows=Object.fromEntries(captured.manifest.sources.map(row=>[row.path,row.sha256]));
writeFileSync(out,JSON.stringify({at:new Date().toISOString(),ms:Math.round(performance.now()-started),files:captured.manifest.sources.length,digest:captured.digest,rows}));
console.log(JSON.stringify({ms:Math.round(performance.now()-started),files:captured.manifest.sources.length,digest:captured.digest}));
if(previous&&existsSync(previous)){const before=JSON.parse(readFileSync(previous,"utf8")).rows as Record<string,string>;const changed=Object.keys({...before,...rows}).filter(path=>before[path]!==rows[path]);console.log("changed",changed.length);for(const path of changed)console.log(" ",path);}
