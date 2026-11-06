import {captureNxAuthorizerSourcesV1,nxAuthorizerCapturedBytesV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
const base="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
for(const entry of ["⌨️command/⚙️runtime/🟦️.ts","⌨️command/🧰️utility/🟦️.ts"]){
 try{const result=await captureNxAuthorizerSourcesV1(process.cwd(),base+entry,new AbortController().signal,{installed:true});console.log(JSON.stringify({entry,files:result.manifest.sources.length,bytes:[...nxAuthorizerCapturedBytesV1(result).values()].reduce((n,row)=>n+row.byteLength,0)}));}
 catch(error){console.log(JSON.stringify({entry,error:(error as Error).message}));}
}
