import {readFileSync} from "node:fs";
import {remoteCacheFetchV1,resolveAuthorisedRemoteCacheV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📡️client/🟦️.ts";
import {decodeActionResultV1,parseRemoteUnitV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📦️wire/🟦️.ts";
const root="/Users/ueli/Documents/semio",crate=process.argv[2]!,lookups="/Users/ueli/Documents/semio/.cache/b/r/m/u/y/lookups.jsonl";
const slots=[...new Set(readFileSync(lookups,"utf8").split("\n").filter(Boolean).map(line=>{try{return JSON.parse(line) as {crate:string;slot?:string};}catch{return {crate:""};}}).filter(row=>row.crate===crate&&row.slot).map(row=>row.slot!))];
const found=await resolveAuthorisedRemoteCacheV1({...process.env,SEMIO_WORKSPACE_ROOT:root} as never,{root}),r=found.resolution!;
const settings=readFleetSettingsV1(root,{SEMIO_REMOTE_CACHE_ENDPOINT:r.contract.endpoint,SEMIO_REMOTE_CACHE_INSTANCE:r.contract.instance,SEMIO_REMOTE_CACHE_ACCESS:"read-only"})!;
const client=createFleetClientV1(settings,{fetch:remoteCacheFetchV1(r.environment,{root})});
for(const slot of slots){
 const rows=(await client.slotKeys!(slot))??[];console.log("slot",slot.slice(0,8),rows.length,"keys");
 const records:{key:string;origin:string;components:Record<string,string>;values?:Record<string,string>}[]=[];
 for(const row of rows){const ar=await client.getResult(row.key);if(typeof ar==="string")continue;const m=decodeActionResultV1(ar).files.find(f=>f.path==="unit.json");if(!m)continue;const b=await client.getBlob(m.digest.hash,m.digest.size);if(!b)continue;const k=parseRemoteUnitV1(b,{key:row.key}).keyComponents;if(k)records.push({key:row.key,origin:k.origin,components:{...k.components},...k.values?{values:{...k.values}}:{}});}
 for(let i=0;i<records.length;i++)for(let j=i+1;j<records.length;j++){const a=records[i]!,b=records[j]!,names=[...new Set([...Object.keys(a.components),...Object.keys(b.components)])].filter(n=>a.components[n]!==b.components[n]).sort();console.log(a.key.slice(0,8),a.origin,"vs",b.key.slice(0,8),b.origin,"differ:",names.join(",")||"-",names.filter(n=>a.values?.[n]!==undefined||b.values?.[n]!==undefined).map(n=>`\n   ${n}: ${a.values?.[n]} | ${b.values?.[n]}`).join(""));}
}
