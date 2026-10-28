// [repair-g] rows of one owner in the sealed generation index (read only). usage: bun owner-rows.mjs <gen-dir> <owner> [pathFilter]
import{readFileSync}from"node:fs";import{join}from"node:path";
const D=process.argv[2],NAME=process.argv[3],FILTER=process.argv[4];
const E="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs";
const g=await import(E);
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
const raw=readFileSync(join(D,"x",manifest.factIndex+".facts.br"));
const index=g.unpackFactIndexV1(raw,g.GENERATION_POLICY_V1.maximumIndexBytes);
const at=index.owners.indexOf(NAME);console.log("generation",pointer.generation,"owner",NAME,"index",at,"rss MB",Math.round(process.memoryUsage().rss/1048576));
const kinds={};let shown=0;
for(const c of index.classes){if(!c.o.includes(at))continue;for(const id of c.r){const r=index.rows[id];kinds[r.kind]=(kinds[r.kind]??0)+1;if(FILTER?r.path.includes(FILTER):shown<40){shown++;console.log(JSON.stringify(r).slice(0,300));}}}
console.log("kinds",JSON.stringify(kinds));
