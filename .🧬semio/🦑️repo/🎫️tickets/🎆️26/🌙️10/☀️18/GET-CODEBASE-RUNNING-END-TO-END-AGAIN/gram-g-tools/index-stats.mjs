// [gram-g] shape of the real fact index: rows, owners, classes, row/owner pairs, class size distribution. usage: bun index-stats.mjs <gen-dir>
import{readFileSync}from"node:fs";import{join}from"node:path";
const D=process.argv[2],B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/",g=await import(B+"🟨️.mjs");
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
const index=g.unpackFactIndexV1(readFileSync(join(D,"x",manifest.factIndex+".facts.br")),g.GENERATION_POLICY_V1.maximumIndexBytes);
let pairs=0,refs=0;const size=[];for(const c of index.classes){pairs+=c.r.length*c.o.length;refs+=c.r.length+c.o.length;size.push(c.o.length);}
size.sort((a,b)=>b-a);
const kinds={};for(const r of index.rows)kinds[r.kind]=(kinds[r.kind]??0)+1;
console.log(JSON.stringify({owners:index.owners.length,rows:index.rows.length,classes:index.classes.length,refs,pairs,topClassOwners:size.slice(0,8),kinds}));
