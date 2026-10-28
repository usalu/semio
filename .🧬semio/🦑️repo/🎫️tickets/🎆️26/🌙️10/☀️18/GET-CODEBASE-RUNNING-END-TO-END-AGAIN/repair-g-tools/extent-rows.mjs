// [repair-g] which owners hold rows on a path fragment (read only). usage: bun extent-rows.mjs <gen-dir> <pathFragment>
import{readFileSync}from"node:fs";import{join}from"node:path";
const D=process.argv[2],FILTER=process.argv[3];
const E="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs";
const g=await import(E);
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
const index=g.unpackFactIndexV1(readFileSync(join(D,"x",manifest.factIndex+".facts.br")),g.GENERATION_POLICY_V1.maximumIndexBytes);
const hits=new Map();
for(const c of index.classes){const rs=c.r.filter(id=>index.rows[id].path.includes(FILTER));if(!rs.length)continue;for(const o of c.o){const n=index.owners[o];const set=hits.get(n)??new Map();for(const id of rs){const r=index.rows[id];set.set(r.kind+" "+r.path.slice(-60),JSON.stringify(r.fact).slice(0,100));}hits.set(n,set);}}
const unav=new Set(Object.entries(manifest.owners).filter(([,r])=>r.kind==="unavailable").map(([n])=>n));
console.log("owners with rows:",hits.size,"unavailable among them:",[...hits.keys()].filter(n=>unav.has(n)).length,"unavailable total",unav.size);
let k=0;for(const[n,s]of hits){if(k++>6)break;console.log(n,unav.has(n)?"UNAV":"",[...s].slice(0,4));}
const by={};for(const[n,r]of Object.entries(manifest.owners)){if(r.kind!=="unavailable")continue;const k=(r.tag??"primary")+"/"+(hits.has(n)?"rows":"norows");(by[k]??=[]).push(n);}
for(const[k,l]of Object.entries(by))console.log(k,l.length,l.slice(0,5).join(" "));
