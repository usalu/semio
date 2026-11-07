import {readFileSync} from "node:fs";
import {canonicalTestAggregatePolicyV1 as p,canonicalTestRoutesV1,resolveCanonicalTestTargetsV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🗂️suites/🧮️aggregate/🟦️.ts";
const root="/Users/ueli/Documents/semio/",proj=JSON.parse(readFileSync(root+p.root+"/📋️project.json","utf8"));
const project={name:p.owner,root:p.root,targets:proj.targets};
let routes=[...canonicalTestRoutesV1()];console.log("routes",routes.length);
for(;;){try{resolveCanonicalTestTargetsV1(project,routes);console.log("OK remaining",routes.length);break}catch(e:any){const m=/reason":"([^"]+)"/.exec(JSON.stringify(e?.fields??e?.context??e?.message??""))||/reason[^a-z]*([a-z:-]+[0-9]*)/.exec(String(e?.message));const r=String(m?.[1]??e?.message);console.log("REFUSED",r);const id=/^route-targets:(.+):[0-9]+$/.exec(r)?.[1];if(!id)break;routes=routes.filter(x=>x.id!==id)}}