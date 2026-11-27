import{captureNxAuthorizerSourcesV1,nxAuthorizerCapturedBytesV1,nxAuthorizerCapturedImportsV1}from"../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import{buildSourceClosureTextV1,buildSourceClosureStreamLoaderV1}from"../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts";
import{writeFileSync}from"node:fs";
const[,,entry,out]=process.argv,root=process.cwd(),captured=await captureNxAuthorizerSourcesV1(root,entry!,new AbortController().signal,{installed:true}),bytes=nxAuthorizerCapturedBytesV1(captured),imports=nxAuthorizerCapturedImportsV1(root,captured);
const files=captured.manifest.sources.map(row=>({...row,base64:Buffer.from(bytes.get(row.path)!).toString("base64"),imports:{...imports.get(row.path)!}})),{text,sha256}=buildSourceClosureTextV1({version:1,root,entry:entry!,files}as never);
writeFileSync(out+".json",text);writeFileSync(out+".loader.js",buildSourceClosureStreamLoaderV1);writeFileSync(out+".sha",sha256);console.log(JSON.stringify({files:files.length,bytes:text.length,sha256}));
