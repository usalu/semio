import{createIssuedResourceControlV1,createScriptProcessEnvelope,withScriptProcessEnvelope}from"/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{captureNxAuthorizerSourcesV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import{compileNxAuthorizerSnapshotV1,snapshotNxAuthorizerSourcesV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️snapshot/🟦️.ts";
const mb=()=>{const m=process.memoryUsage();return Math.round(m.rss/1048576)+" MB (heap "+Math.round(m.heapUsed/1048576)+" ext "+Math.round(m.external/1048576)+" ab "+Math.round(m.arrayBuffers/1048576)+")";};
const t0=Date.now(),log=(s:string)=>console.log(((Date.now()-t0)/1000).toFixed(1)+"s",s,mb());
await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"fixture:compiler-init",maximumElapsedMilliseconds:0},{fixture:"compiler-init"},Date.now()),async original=>{
 const control=createIssuedResourceControlV1(original,"fixture:compiler-init",0);log("start");
 const captured=await captureNxAuthorizerSourcesV1(process.cwd(),"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📜️script.ts",control.signal);log("captured "+captured.manifest.sources.length+" sources");
 const snap=snapshotNxAuthorizerSourcesV1(captured);log("snapshot");
 const sealed=await compileNxAuthorizerSnapshotV1(process.cwd(),snap,control,"node","module");log("compiled "+sealed.bytes.length+" bytes");await Bun.write(process.env.DUMP!,sealed.bytes);Bun.gc(true);log("gc");
});
