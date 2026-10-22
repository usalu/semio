import {captureNxAuthorizerSourcesV1,nxAuthorizerCapturedBytesV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
const root="/Users/ueli/Documents/semio",L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const entries=process.argv.slice(2);
for(const e of entries){const t=Date.now();try{const s=await captureNxAuthorizerSourcesV1(root,L+e,new AbortController().signal);console.log(e,"files",s.manifest.sources.length,"ms",Date.now()-t);}catch(err){console.log(e,"FAIL",String((err as Error).message).slice(0,300));}}
