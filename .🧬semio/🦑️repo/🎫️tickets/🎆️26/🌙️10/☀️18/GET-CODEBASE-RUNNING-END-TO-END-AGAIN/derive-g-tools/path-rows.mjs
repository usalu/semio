// [derive-g] rows and holders of paths in a sealed generation index (read only). usage: bun path-rows.mjs <gen-dir> <path-substring>...
import{readFileSync}from"node:fs";import{join}from"node:path";
const D=process.argv[2],WANT=process.argv.slice(3);
const g=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs");
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
const index=g.unpackFactIndexV1(readFileSync(join(D,"x",manifest.factIndex+".facts.br")),g.GENERATION_POLICY_V1.maximumIndexBytes);
for(const c of index.classes)for(const id of c.r){const r=index.rows[id];if(WANT.some(w=>r.path===w))console.log(r.kind,r.path.slice(0,80),c.o.length,c.o.slice(0,3).map(i=>index.owners[i]).join(","))}
