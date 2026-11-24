import {reachedFilesV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🗺️paths/🟦️.ts";
import {dirname,resolve} from "node:path";
const [manifest,source]=process.argv.slice(2);
for(const max of [512,5000]){
 const r=reachedFilesV1(resolve(manifest,source),undefined,max,manifest);
 console.log(max,r===undefined?"undefined":r.length);
 if(r){let c=resolve(manifest,source);for(const p of r){while(!p.startsWith(c.endsWith("/")?c:c+"/")&&c!=="/")c=dirname(c);}console.log("common",c);}
}
