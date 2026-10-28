import{captureNxAuthorizerSourcesV1,nxAuthorizerCapturedBytesV1,readNxAuthorizerSourceV1}from"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import{closureLiteralSitesV1}from"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts";
import{resolve}from"node:path";
const root=resolve(import.meta.dirname,"../../../../../../../..");const entry=process.argv[2]!;
const c=await captureNxAuthorizerSourcesV1(root,entry,new AbortController().signal,{installed:true});
const bytes=nxAuthorizerCapturedBytesV1(c),paths=[...bytes.keys()];
const sig=new AbortController().signal;
let t=performance.now(),cpu=process.cpuUsage();
for(const p of paths)await readNxAuthorizerSourceV1(root,p,sig);
console.log("read all",paths.length,"files",Math.round(performance.now()-t),"ms",JSON.stringify(process.cpuUsage(cpu)));
t=performance.now();cpu=process.cpuUsage();let rows=0;
for(const [p,b] of bytes){if(p.endsWith(".json"))continue;rows+=closureLiteralSitesV1(p,Buffer.from(b).toString("utf8").replace(/^#![^\r\n]*/u,"")).length;}
console.log("scan all",rows,"rows",Math.round(performance.now()-t),"ms",JSON.stringify(process.cpuUsage(cpu)));
