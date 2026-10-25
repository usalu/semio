import {schemaContractDiagnostics,repoRootFromHere} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
const root=repoRootFromHere();
const t=performance.now();
const all=schemaContractDiagnostics(root);
const by:Record<string,number>={};for(const d of all)by[d.code]=(by[d.code]??0)+1;
console.log("[prof]",Math.round(performance.now()-t),"ms",JSON.stringify(by));
for(const d of all.filter(d=>d.code==="schema-owner-ineligible").slice(0,12))console.log("[prof]",d.path);
