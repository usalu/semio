/** 🔁️ Read-only replay of the multiwriter merge on REAL generations: for consecutive generations B -> H -> N whose deltas are disjoint, the delta H -> N is rebased as if it had been built on B concurrently with H ("mine"), and the merge of batch multiwriter (from `--code`, a mirror with the batch) decides it against the head H on the real fact indexes. Expectation when accepted: the merged owners equal N's owners exactly and every project owner's merged fact rows equal N's (pseudo owners: a superset). Never writes the store (keep/manifest/publish are in-memory stubs).
 * usage: bun replay.mjs --code <mirror root with the batch> --store <generations dir> --base <B> [--limit-mb 3000] */
import {readFileSync,readdirSync} from "node:fs";
import {join} from "node:path";

const argument=name=>{const at=process.argv.indexOf("--"+name);return at>0?process.argv[at+1]:undefined;};
const G=join(argument("code"),"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations"),store=argument("store"),B=Number(argument("base")),limit=Number(argument("limit-mb")??"3000");
const generations=await import(join(G,"🟨️.mjs")),merge=await import(join(G,"🔀️merge/🟨️.mjs"));
const rss=()=>Math.round(process.memoryUsage().rss/1048576),guard=where=>{const mb=rss();console.log("[DEBUG] rss",where,mb,"MB");if(mb>limit)throw Error("[DEBUG] RSS above "+limit+" MB at "+where);};
const manifests=new Map();for(const file of readdirSync(join(store,"m"))){const m=generations.readGenerationManifestV1(readFileSync(join(store,"m",file)));manifests.set(m.generation,{digest:file.slice(0,64),manifest:m});}
const pointer=JSON.parse(readFileSync(join(store,"current.json"),"utf8"));
const index=digest=>generations.unpackFactIndexV1(readFileSync(join(store,"x",digest+".facts.br")),generations.GENERATION_POLICY_V1.maximumIndexBytes);
const canon=generations.canonicalJsonV1,key=generations.rowKeyV1;
const base=manifests.get(B).manifest,head=manifests.get(B+1).manifest,next=manifests.get(B+2).manifest;
const changed=(a,b)=>[...new Set([...Object.keys(a.owners),...Object.keys(b.owners)])].filter(name=>canon(a.owners[name]??null)!==canon(b.owners[name]??null));
const theirs=changed(base,head),delta=changed(head,next);
console.log(JSON.stringify({base:B,head:B+1,next:B+2,headDirty:theirs.length,deltaDirty:delta.length,overlap:theirs.filter(name=>delta.includes(name)).length,global:[base.global,head.global,next.global].map(d=>d.slice(0,10)),env:[base.env,head.env,next.env].map(d=>d.slice(0,10)),membership:[base.membership,head.membership,next.membership].map(d=>d.slice(0,10))}));
const headIndex=index(head.factIndex);guard("head index");const nextIndex=index(next.factIndex);guard("next index");
const rowsOf=(ix,names)=>merge.ownerRowsOfV1(ix,new Set(names));
const nextOwners=new Set(nextIndex.owners),headOwners=new Set(headIndex.owners);
const pseudoChanged=[...nextOwners].filter(name=>merge.isPseudoOwnerV1(name)),headPseudo=rowsOf(headIndex,pseudoChanged),nextPseudo=rowsOf(nextIndex,pseudoChanged);
const keys=rows=>[...new Set(rows.map(key))].sort().join("\n");
const upserts=new Map(),drop=[];
for(const name of delta){if(next.owners[name]===undefined)drop.push(name);else upserts.set(name,rowsOf(nextIndex,[name]).get(name));}
for(const name of pseudoChanged)if(!headOwners.has(name)||keys(headPseudo.get(name))!==keys(nextPseudo.get(name)))upserts.set(name,nextPseudo.get(name));
const mineOwners={...base.owners};for(const name of delta){if(next.owners[name]===undefined)delete mineOwners[name];else mineOwners[name]=next.owners[name];}
const mine={...next,generation:B+1,parent:manifests.get(B).digest,owners:mineOwners};
console.log(JSON.stringify({upserts:upserts.size,pseudoUpserts:[...upserts.keys()].filter(merge.isPseudoOwnerV1).length,drop:drop.length}));
const baseIndex=index(base.factIndex);guard("base index");
const started=performance.now(),found=merge.mergeGenerationV1({base,head,mine,baseIndex,headIndex,patch:{drop,upserts,append:new Map()},key});
console.log(JSON.stringify({decisionMs:Math.round(performance.now()-started),conflict:found.conflict??null,owner:found.owner??null,theirs:found.theirs?.length,ours:found.ours?.length}));
if(!found.conflict&&!process.argv.includes("--decide-only")){
 const ownersEqual=canon(found.owners)===canon(next.owners);
 const t0=performance.now();let merged=generations.patchFactIndexV1(headIndex,{drop:found.drop,upserts:found.upserts,append:found.append});const patchMs=Math.round(performance.now()-t0);guard("merged index");
 const checked=[...new Set([...delta,...theirs])].filter(name=>next.owners[name]),a=rowsOf(merged,checked),b=rowsOf(nextIndex,checked);let rowsDiffer=0;for(const name of checked)if(keys(a.get(name))!==keys(b.get(name)))rowsDiffer++;
 const pseudoNames=[...upserts.keys()].filter(merge.isPseudoOwnerV1),pa=rowsOf(merged,pseudoNames),pn=rowsOf(nextIndex,pseudoNames);let notSuperset=0;for(const name of pseudoNames){const have=new Set(pa.get(name).map(key));if(!pn.get(name).every(row=>have.has(key(row))))notSuperset++;}
 const ownersSame=canon(merged.owners.filter(name=>!merge.isPseudoOwnerV1(name)).sort())===canon(Object.keys(next.owners).sort());
 const port={read:async()=>({pointer:{generation:B+1,manifest:manifests.get(B+1).digest,verified:undefined},manifest:head}),readIndex:async digest=>digest===base.factIndex?baseIndex:digest===head.factIndex?headIndex:undefined,writeManifest:async()=> "0".repeat(64),publish:async()=>({published:true,pointer:null}),patch:generations.patchFactIndexV1,pack:value=>generations.packFactIndexV1(value,generations.GENERATION_POLICY_V1.maximumIndexBytes),keep:async bytes=>({digest:"f".repeat(64),bytes:bytes.length}),key,stats:{merged:0,mergeConflicts:0}};
 const t1=performance.now(),whole=await merge.mergeIntoHeadV1(port,{base:{pointer:{generation:B},manifest:base},mine,patch:{drop,upserts,append:new Map()},signal:undefined});guard("whole merge");
 console.log(JSON.stringify({ownersEqualNext:ownersEqual,projectRowsDifferFromNext:rowsDiffer,checkedProjects:checked.length,pseudoNotSupersetOfNext:notSuperset,pseudoChecked:pseudoNames.length,indexOwnersEqualNext:ownersSame,patchMs,wholeMergeMs:Math.round(performance.now()-t1),wholeConflict:whole.conflict??null,packedBytes:whole.kept?.bytes,nextPackedBytes:readFileSync(join(store,"x",next.factIndex+".facts.br")).length,pointer:pointer.generation}));
}
