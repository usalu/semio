import {writeFileSync} from "node:fs";
import {remoteCacheFetchV1,resolveAuthorisedRemoteCacheV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts";
const root="/Users/ueli/Documents/semio",out=process.argv[2]!;
const found=await resolveAuthorisedRemoteCacheV1({...process.env,SEMIO_WORKSPACE_ROOT:root} as never,{root}),r=found.resolution!;
const call=remoteCacheFetchV1(r.environment,{root}),ep=r.contract.endpoint;
const t=await (await call(ep+"/evidence")).text();writeFileSync(out,t);console.log("bytes",t.length);
