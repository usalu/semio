import {resolve} from "node:path";
import authored from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🔣️policy.json";
import {classifyLeaseResourceV1,readLeaseOrderPolicyV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts";
import {censusOf,listSources} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🧪️tests/🧭️sites/🔎️census.mjs";
const framework="/Users/ueli/Documents/semio/🧰️framework",roots=[framework+"/🔨️modules/🏃️process",framework+"/🛍️products/🦑️repo/🔨️modules/📚️library"];
const live=readLeaseOrderPolicyV1(authored),sites=censusOf(roots.flatMap(r=>listSources(r)));
const nameOf=(s:any)=>s.literal===undefined?undefined:s.literal.startsWith("json:")?'["'+s.literal.slice(5)+'"':s.literal;
const dyn:Record<string,number>={};for(const s of sites)if(s.literal===undefined){const k=s.file.slice(framework.length+1);dyn[k]=(dyn[k]??0)+1;}
console.log(JSON.stringify({sites:sites.length,dyn:Object.fromEntries(Object.entries(dyn).sort()),unclassified:[...new Set(sites.map(nameOf).filter((n:any)=>n!==undefined&&classifyLeaseResourceV1(live,n).id==="unclassified"))].sort()}));
