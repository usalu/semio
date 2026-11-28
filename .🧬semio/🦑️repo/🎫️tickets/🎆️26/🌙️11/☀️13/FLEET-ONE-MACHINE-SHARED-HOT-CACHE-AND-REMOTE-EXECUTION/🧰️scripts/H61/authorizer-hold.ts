import {captureNxAuthorizerSourcesV1,nxAuthorizerSourcesHoldV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
const root="/Users/ueli/Documents/semio",recipe="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools";
for(const entry of [recipe+"/🔏️authorizers/📦️runtime/🟦️.ts",recipe+"/🔏️authorizers/📜️script.ts"]){
 const signal=new AbortController().signal,started=performance.now(),first=await captureNxAuthorizerSourcesV1(root,entry,signal),mid=performance.now();
 let second:any,error:any;try{second=await captureNxAuthorizerSourcesV1(root,entry,signal,{installed:false,roots:undefined});}catch(caught){error=caught;}
 const held=await nxAuthorizerSourcesHoldV1(root,first,signal);
 console.log(JSON.stringify({entry:entry.slice(-30),files:first.manifest.sources.length,captureMs:Math.round(mid-started),holdMs:Math.round(performance.now()-mid),held,sameDigest:second?.digest===first.digest,error:error?.message}));
}
