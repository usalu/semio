import {writeFileSync,mkdirSync} from "node:fs";
import {remoteCacheFetchV1,resolveAuthorisedRemoteCacheV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts";
const root="/Users/ueli/Documents/semio",out=process.argv[2]!,keys=process.argv.slice(3);
const found=await resolveAuthorisedRemoteCacheV1({...process.env,SEMIO_WORKSPACE_ROOT:root} as never,{root}),r=found.resolution!;
const call=remoteCacheFetchV1(r.environment,{root}),ep=r.contract.endpoint;
mkdirSync(out,{recursive:true});
for(const key of keys){
 const res=await call(`${ep.replace(/\/+$/,"")}/${r.contract.instance}/ac/${key}`);const b=new Uint8Array(await res.arrayBuffer());
 writeFileSync(`${out}/${key.slice(0,8)}.ac`,b);console.log(key.slice(0,8),res.status,b.length);
}
