/** ⏱️ Times `patchFactIndexV1` of a real head index with the rebased delta of `replay.mjs`, with and without the pseudo-owner union, to find what the merge's re-patch costs. usage: bun profile-patch.mjs --code <mirror> --store <generations dir> --base <B> */
import {readFileSync,readdirSync} from "node:fs";
import {join} from "node:path";
const argument=name=>{const at=process.argv.indexOf("--"+name);return at>0?process.argv[at+1]:undefined;};
const G=join(argument("code"),"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations"),store=argument("store"),B=Number(argument("base"));
const generations=await import(join(G,"🟨️.mjs")),merge=await import(join(G,"🔀️merge/🟨️.mjs")),chunks=await import(join(G,"🧱️chunk-classes/🟨️.mjs"));
const ms=new Map();for(const f of readdirSync(join(store,"m"))){const m=generations.readGenerationManifestV1(readFileSync(join(store,"m",f)));ms.set(m.generation,m);}
const index=d=>generations.unpackFactIndexV1(readFileSync(join(store,"x",d+".facts.br")),generations.GENERATION_POLICY_V1.maximumIndexBytes);
const head=ms.get(B+1),next=ms.get(B+2),canon=generations.canonicalJsonV1;
const delta=[...new Set([...Object.keys(head.owners),...Object.keys(next.owners)])].filter(n=>canon(head.owners[n]??null)!==canon(next.owners[n]??null));
const headIndex=index(head.factIndex),nextIndex=index(next.factIndex);
const pseudo=nextIndex.owners.filter(merge.isPseudoOwnerV1),np=merge.ownerRowsOfV1(nextIndex,new Set([...delta,...pseudo])),hp=merge.ownerRowsOfV1(headIndex,new Set(pseudo));
const key=generations.rowKeyV1,keys=r=>[...new Set(r.map(key))].sort().join("\n");
const plain=new Map(delta.map(n=>[n,np.get(n)]));for(const n of pseudo)if(keys(hp.get(n))!==keys(np.get(n)))plain.set(n,np.get(n));
const union=new Map(plain);for(const n of plain.keys())if(merge.isPseudoOwnerV1(n)){const seen=new Set(plain.get(n).map(key)),rows=[...plain.get(n)];for(const r of hp.get(n))if(!seen.has(key(r))){seen.add(key(r));rows.push(r);}union.set(n,rows);}
const pairs=m=>[...m.values()].reduce((s,r)=>s+r.length,0);
console.log(JSON.stringify({bulkUpsertRows:chunks.INDEX_CHUNK_POLICY_V1.bulkUpsertRows,owners:plain.size,pairsPlain:pairs(plain),pairsUnion:pairs(union),envRows:np.get("@env")?.length,envRowsHead:hp.get("@env")?.length,indexRows:headIndex.rows.length,classes:headIndex.classes.length}));
for(const[label,upserts]of[["plain",plain],["union",union]]){const t=performance.now();const out=generations.patchFactIndexV1(headIndex,{drop:[],upserts,append:new Map()});console.log(JSON.stringify({label,patchMs:Math.round(performance.now()-t),rows:out.rows.length,classes:out.classes.length,rss:Math.round(process.memoryUsage().rss/1048576)}));}
