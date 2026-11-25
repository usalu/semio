/** 🧪️ W12: per-owner cost of the batched take prefetch on the coordinator: reads N stored records of the workspace selection (read-only), extracts their inventory names with the real capture, counts distinct names and the asks a pass needs. Usage: bun probe-prefetch.mjs <np-dir> [N] */
import{readdirSync}from"node:fs";
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const{takeStoredRecordResultV1,prefetchTakePresenceV1}=await import(B+"🕸️graph/📥️inference/📦️project-nodes/📥️take/🧺️presence-prefetch/🟨️.mjs");
const{captureInferenceInventoryReferencesV1}=await import(B+"👁️observation/📥️reuse/🟨️.mjs");
const dir=process.argv[2],n=Number(process.argv[3]??60),files=readdirSync(dir).filter(f=>/^[0-9a-f]{24}\.json$/.test(f)).sort().slice(0,n).map(f=>dir+"/"+f);
let names=0;const t0=performance.now();for(const f of files){const v=takeStoredRecordResultV1(f);if(v)names+=captureInferenceInventoryReferencesV1(v).length;}const serial=performance.now()-t0;
let asks=0;const counts=await prefetchTakePresenceV1({records:files,capture:captureInferenceInventoryReferencesV1,verify:async list=>{asks++;return{present:list.length};}});
console.log(JSON.stringify({records:files.length,readMsPerOwner:+(serial/files.length).toFixed(2),namesPerOwner:+(names/files.length).toFixed(2),distinct:counts.names,asks,prefetchMs:counts.milliseconds,skipped:counts.skipped}));
