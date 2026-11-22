import {readFileSync} from "node:fs";
import {readCacheLedgerV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
const policy=JSON.parse(readFileSync("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🔣️policy.json","utf8"));
for(const bp of [1000,800,600,500,400,300,200,100]){const copy=structuredClone(policy);copy.areas.find((a:any)=>a.id==="fleet-cache-index").budget.basisPoints=bp;try{readCacheLedgerV1(copy);console.log(bp,"ok");}catch(e){console.log(bp,String((e as Error).message).slice(0,120));}}
