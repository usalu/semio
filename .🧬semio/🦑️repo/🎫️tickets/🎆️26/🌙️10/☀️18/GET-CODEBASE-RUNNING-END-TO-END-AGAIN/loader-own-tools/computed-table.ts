// computed-table.ts: prints the ComputedImportSites rows derived from the TypeScript AST over all kernel closures (owner, count, roots kept from the live table).
import {readFileSync} from "node:fs";
import {captureNxAuthorizerSourcesV1,nxAuthorizerCapturedBytesV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import {astSitesV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🧪️tests/🔭️oracle/🟦️.ts";
const root="/Users/ueli/Documents/semio",lib="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const schemaPath=root+"/"+lib+"⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧬️schema/🔣️.json",schema=JSON.parse(readFileSync(schemaPath,"utf8"));
const kernels=Object.values(JSON.parse(readFileSync(root+"/"+lib+"🏗️build/⌨️command/🧭️selection/🧬️schema/🔣️.json","utf8")).properties.kernels.const as Record<string,string>).map(p=>lib+"🏗️build/⌨️command/"+p);
const files=new Map<string,Uint8Array>();
for(const k of kernels){const s=await captureNxAuthorizerSourcesV1(root,k,new AbortController().signal,{installed:true});for(const [p,b] of nxAuthorizerCapturedBytesV1(s))files.set(p,b);}
const counts=new Map<string,number>();
for(const [p,b] of files){if(!/\.[cm]?[jt]sx?$/.test(p)||/\.d\.[cm]?ts$/.test(p))continue;const n=astSitesV1(p,Buffer.from(b).toString("utf8")).filter(s=>s.form==="computed").length;if(n)counts.set(p,n);}
const old=new Map<string,{count:number;roots:string[]}>((schema.$defs.ComputedImportSites.const as any[]).map(r=>[r.owner,r]));
const rows=[...counts].sort(([a],[b])=>Buffer.compare(Buffer.from(a),Buffer.from(b))).map(([owner,count])=>({owner,count,roots:old.get(owner)?.roots??[]}));
for(const [o,r] of old)if(!counts.has(o))console.error("STALE",o,r.count);
for(const r of rows){const o=old.get(r.owner);if(!o||o.count!==r.count)console.error("CHANGE",r.owner.split("library/").pop(),o?.count??"-","->",r.count);}
console.log(JSON.stringify(rows));
