/** lane-g: READ-ONLY look at the real lane pool of this machine: the machine-budget quota, the lanes with recorded sizes and what the pool would trim. Removes nothing. */
import {join} from "node:path";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",R="/Users/ueli/Documents/semio";
const {repositoryStrictStorageV1}=await import(`${L}/⚡️caching/💾️storage/🔒️enforcement/🟦️.ts`);
const {compilerLaneQuotaBytesV1}=await import(`${L}/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🛤️lane/🟦️.ts`);
const {readPoolV1,planPoolTrimV1,poolBoundsV1}=await import(`${L}/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🟦️.ts`);
const {listLanesV1}=await import(`${L}/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🟦️.ts`);
const declaration=repositoryStrictStorageV1(R),capacity=declaration.capacityBytes-declaration.maximumWorkspaceBytes,quota=await compilerLaneQuotaBytesV1(R,process.env,capacity);
const root=join(R,".🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u"),now=Date.now(),measured=process.argv[2]==="measure"?listLanesV1(root,now):[];
const lanes=readPoolV1(root,now),bounds=poolBoundsV1(quota);
const GiB=1024**3;
console.log(JSON.stringify({imageCapacityGiB:declaration.capacityBytes/GiB,workspaceGiB:declaration.maximumWorkspaceBytes/GiB,unitCacheCapacityGiB:capacity/GiB,laneQuotaGiB:quota/GiB,bounds,lanes:lanes.map(l=>({id:l.id,claimed:l.claimed,ageMin:Math.round((now-l.lastUsedMs)/60000),key:l.crateKey,MiB:l.bytes===null?null:Math.round(l.bytes/1048576)})),measuredMiB:measured.map(m=>({id:m.id,MiB:Math.round(m.bytes/1048576)})),wouldTrim:planPoolTrimV1(lanes,bounds)},null,1));
