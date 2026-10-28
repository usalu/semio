import{captureNxAuthorizerSourcesV1}from"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import{resolve}from"node:path";import{writeFileSync}from"node:fs";
const root=resolve(import.meta.dirname,"../../../../../../../..");
const c=await captureNxAuthorizerSourcesV1(root,process.argv[2]!,new AbortController().signal,{installed:true});
writeFileSync(process.argv[3]!,JSON.stringify(c.manifest.sources.map(s=>s.path)));
let max=0;for(const s of c.manifest.sources)max=Math.max(max,[...s.path].length+root.length);console.log(c.manifest.sources.length,"maxAbsLen(root len",root.length,")",max);
