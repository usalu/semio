import{captureNxAuthorizerSourcesV1,nxAuthorizerCapturedBytesV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
const root="/Users/ueli/Documents/semio",entry="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/⚙️runtime/🟦️.ts";
try{await captureNxAuthorizerSourcesV1(root,entry,new AbortController().signal,{installed:true});console.log("ok")}catch(e:any){console.log(String(e.message).slice(0,200));console.log(JSON.stringify(e.diagnostic??e.cause??Object.keys(e),null,1))}
