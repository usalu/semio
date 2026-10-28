// [repair-g] which owners of the sealed generation are unfounded withheld (read only). usage: bun unfounded-live.mjs <gen-dir>
import{readFileSync}from"node:fs";import{join}from"node:path";
const D=process.argv[2];
const g=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs");
const c=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs");
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
const raw=readFileSync(join(D,"x",manifest.factIndex+".facts.br"));
let t=performance.now();
const index=g.unpackFactIndexV1(raw,g.GENERATION_POLICY_V1.maximumIndexBytes);
let r=g.unfoundedWithheldOwnersV1(index,manifest.owners);console.log("eager",r.length,Math.round(performance.now()-t),"ms",r.slice(0,3));
const view=g.openPackedFactIndexV1(raw,g.GENERATION_POLICY_V1.maximumIndexBytes);t=performance.now();
r=g.unfoundedWithheldOwnersV1(view,manifest.owners);console.log("lazy",r.length,Math.round(performance.now()-t),"ms");
