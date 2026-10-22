import {join} from "node:path";
import {pathToFileURL} from "node:url";
const root=process.cwd(),entry=process.argv[2]!,authorizers=await import(pathToFileURL(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts")).href);
const original=Bun.resolve.bind(Bun);(Bun as unknown as {resolve:unknown}).resolve=async(specifier:string,from:string)=>{const located=await original(specifier,from);console.error("[DEBUG] resolve",specifier,"from",from.slice(-110),"->",located.slice(-60));return located;};
try{const result=await authorizers.captureNxAuthorizerSourcesV1(root,entry,new AbortController().signal);console.log("captured",result.manifest.sources.length);}catch(error){console.error("[DEBUG] failed",(error as Error).message);}
