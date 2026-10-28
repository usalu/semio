import{resolve}from"node:path";
const root=resolve(import.meta.dirname,"../../../../../../../..");const entry=process.argv[2]!;
const dir="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/";
for(const name of["🟦️.old-noop2.ts","🟦️.ts"]){
 const m=await import(resolve(root,dir+name));const t=performance.now(),cpu=process.cpuUsage();
 const c=await m.captureNxAuthorizerSourcesV1(root,entry,new AbortController().signal,{installed:true});
 console.log(name,"files",c.manifest.sources.length,"digest",c.digest.slice(0,16),"ms",Math.round(performance.now()-t),JSON.stringify(process.cpuUsage(cpu)));
}
