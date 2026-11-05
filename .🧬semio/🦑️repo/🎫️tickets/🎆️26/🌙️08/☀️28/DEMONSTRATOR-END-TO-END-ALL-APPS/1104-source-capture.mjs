import{writeFile}from"node:fs/promises";
import{resolve,relative,join}from"node:path";
import{pathToFileURL}from"node:url";
const root=process.cwd(),lib="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",uri=p=>pathToFileURL(resolve(root,p)).href;
const{repositoryCommandKernelV1}=await import(uri(lib+"/🏗️build/⌨️command/🧭️selection/🟦️.ts"));
const{captureNxAuthorizerSourcesV1,nxAuthorizerCapturedBytesV1}=await import(uri(lib+"/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts"));
const entry=relative(root,repositoryCommandKernelV1(["cargo","--store=shared","--configuration=prototype","--","check","-p","semio-framework-os-flow-host-session","--lib"]).entry).replaceAll("\\","/");
let result;try{const capture=await captureNxAuthorizerSourcesV1(root,entry,AbortSignal.timeout(60000),{installed:true}),bytes=nxAuthorizerCapturedBytesV1(capture);result={version:1,successful:true,entry,files:capture.manifest.sources.length,bytes:[...bytes.values()].reduce((sum,value)=>sum+value.byteLength,0),digest:capture.digest,inventory:[...bytes.keys()].filter(path=>path.includes("/📇️inventory/"))};}catch(error){if(error.sourceCensus)await writeFile(join(import.meta.dirname,"🗑️generated/1104-source-census-bun-exact.json"),JSON.stringify(error.sourceCensus,null,2)+"\n");result={version:1,successful:false,entry,message:error.message,diagnostic:error.diagnostic??null};}
await writeFile(join(import.meta.dirname,"🗑️generated/1104-source-capture-final.json"),JSON.stringify(result,null,2)+"\n");console.log(JSON.stringify(result));if(!result.successful)process.exitCode=1;
