// [derive-g] kinds + sample paths of the @env rows of a sealed generation (read only). usage: bun env-rows.mjs <gen-dir> <kind>
import{readFileSync}from"node:fs";import{join}from"node:path";
const D=process.argv[2],KIND=process.argv[3]??"digest";
const g=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs");
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
const index=g.unpackFactIndexV1(readFileSync(join(D,"x",manifest.factIndex+".facts.br")),g.GENERATION_POLICY_V1.maximumIndexBytes);
const at=index.owners.indexOf("@env");
for(const c of index.classes){if(!c.o.includes(at))continue;for(const id of c.r){const r=index.rows[id];if(r.kind===KIND)console.log(c.o.length,r.path)}}
