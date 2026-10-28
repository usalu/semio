import{canonicalJsonV1,compareCodePointsV1}from"../🧮️fact-codec/🟨️.mjs";
import{decodeVerifiedScopeV1,mergeVerifiedStampV1}from"../🔭️coverage/🟨️.mjs";
import{isDerivedOwnerV1,orphanedDerivedOwnersV1,unsupportedDerivedV1}from"../✂️cutoff/🟨️.mjs";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 🔀️ The declared constants of the generation merge; the schema file is the single source and a law keeps both equal. */
export const MERGE_POLICY_V1=Object.freeze(structuredClone(schema.definitions.Policy.const));
const P=MERGE_POLICY_V1,canon=value=>value===undefined?"":canonicalJsonV1(value),sorted=names=>[...names].sort(compareCodePointsV1);
const stubOf=row=>row===undefined?"":canonicalJsonV1(P.stubFields.filter(field=>row[field]!==undefined).map(field=>[field,row[field]]));

/** 👻️ Whether an owner is a pseudo owner (the environment or a derived fact) whose rows two writers may both carry. */
export const isPseudoOwnerV1=name=>name===P.envOwner||isDerivedOwnerV1(name);

/** 🧾️ The rows of the named owners of one fact index, in the order of its classes; an owner the index does not name has none. */
export function ownerRowsOfV1(index,names){
 const ids=new Map(),out=new Map([...names].map(name=>[name,[]]));
 index.owners.forEach((name,id)=>{if(out.has(name))ids.set(id,name);});
 if(ids.size)for(const cls of index.classes)for(const id of cls.o){const name=ids.get(id);if(name!==undefined)for(const row of cls.r)out.get(name).push(index.rows[row]);}
 return out;
}

/**
 * 🧮️ Validates (symmetrically for stubs: a peer's stub change and, once a peer changed any owner, a stub change of this writer both refuse, since every computation reads all stubs) the delta of one writer (`mine`, the manifest it built on `base`, and `patch`, the fact index patch it applied to the base index) against the `head` a peer published meanwhile, and answers either `{conflict, owner?}` or the merged owners, global fragment, membership, frontier, index patch for the head index and verified stamp. `key` names a row (`rowKeyV1`).
 */
export function mergeGenerationV1({base,head,mine,baseIndex,headIndex,patch,verified,headVerified,key}){
 if(head.identity!==mine.identity||head.implementation!==mine.implementation)return{conflict:"identity"};
 if(head.env!==base.env)return{conflict:"environment"};
 const theirs=new Set(),ours=new Set();
 for(const name of new Set([...Object.keys(base.owners),...Object.keys(head.owners),...Object.keys(mine.owners)])){
  const before=canon(base.owners[name]);
  if(canon(head.owners[name])!==before){if(stubOf(head.owners[name])!==stubOf(base.owners[name]))return{conflict:"structure",owner:name};theirs.add(name);}
  if(canon(mine.owners[name])!==before)ours.add(name);
 }
 for(const name of[...patch.drop,...patch.upserts.keys(),...patch.append.keys()])if(!isPseudoOwnerV1(name))ours.add(name);
 if(theirs.size)for(const name of sorted(ours))if(stubOf(mine.owners[name])!==stubOf(base.owners[name]))return{conflict:"structure",owner:name};
 const keys=rows=>sorted(new Set(rows.map(key))).join("\n"),before=ownerRowsOfV1(baseIndex,ours),after=ownerRowsOfV1(headIndex,ours);
 for(const name of sorted(ours))if(theirs.has(name)||keys(before.get(name))!==keys(after.get(name)))return{conflict:"owner",owner:name};
 const global=head.global===base.global?mine.global:mine.global===base.global||mine.global===head.global?head.global:undefined;
 if(global===undefined)return{conflict:"global"};
 const side=mine.membership===base.membership?head:head.membership===base.membership||head.membership===mine.membership?mine:undefined;
 if(!side)return{conflict:"membership"};
 const owners={...head.owners};
 for(const name of ours){if(mine.owners[name]===undefined)delete owners[name];else owners[name]=mine.owners[name];}
 for(const[name,row]of Object.entries(owners))if(row.edgesEnv!==mine.env)return{conflict:"environment",owner:name};
 const upserts=new Map(patch.upserts),shared=ownerRowsOfV1(headIndex,new Set([...upserts.keys()].filter(isPseudoOwnerV1)));
 for(const[name,extra]of shared){const rows=[...upserts.get(name)],seen=new Set(rows.map(key));for(const row of extra)if(!seen.has(key(row))){seen.add(key(row));rows.push(row);}upserts.set(name,rows);}
 const names=sorted(Object.keys(owners)),carried=headVerified&&headVerified.count===Object.keys(head.owners).length?{atMs:headVerified.atMs,names:decodeVerifiedScopeV1(sorted(Object.keys(head.owners)),headVerified)}:undefined;
 const stamp=verified?mergeVerifiedStampV1({owners:names,scope:new Set([...decodeVerifiedScopeV1(sorted(Object.keys(mine.owners)),verified)].filter(name=>!theirs.has(name))),observedAtMs:verified.atMs,carried}):undefined;
 return{owners,global,membership:side.membership,frontier:side.frontier,drop:patch.drop,upserts,append:patch.append,theirs:sorted(theirs),ours:sorted(ours),verified:stamp};
}

/**
 * 🔁️ After a lost compare-and-swap, merges the delta of `mine` (built on `base`) onto the newest head until a swap succeeds, a conflict appears or the rounds are spent. `port` carries the store operations (`read`, `readIndex`, `writeManifest`, `publish`, `patch`, `pack`, `keep`, `key`, `stats`); answers `{outcome, next, index, packed, kept, theirs, rounds}` when it published and `{conflict}` otherwise.
 */
export async function mergeIntoHeadV1(port,{base,mine,patch,verified,signal}){
 const count=found=>{if(port.stats)port.stats[found.conflict?"mergeConflicts":"merged"]++;return found;};
 const baseIndex=await port.readIndex(base.manifest.factIndex);if(!baseIndex)return count({conflict:"index"});
 for(let round=1;round<=P.rounds;round++){
  signal?.throwIfAborted();
  const head=await port.read(signal);if(!head?.manifest)return count({conflict:"head"});
  const headIndex=await port.readIndex(head.manifest.factIndex);if(!headIndex)return count({conflict:"index"});
  const found=mergeGenerationV1({base:base.manifest,head:head.manifest,mine,baseIndex,headIndex,patch,verified,headVerified:head.pointer.verified,key:port.key});
  if(found.conflict)return count(found);
  let index=port.patch(headIndex,{drop:found.drop,upserts:found.upserts,append:found.append});
  const orphans=orphanedDerivedOwnersV1(index);if(orphans.length)index=port.patch(index,{drop:orphans});
  if(unsupportedDerivedV1(index).length)return count({conflict:"derived"});
  const packed=port.pack(index),kept=await port.keep(packed.bytes);
  const next={...mine,generation:head.pointer.generation+1,parent:head.pointer.manifest,owners:found.owners,global:found.global,membership:found.membership,frontier:found.frontier,factIndex:kept.digest,built:{kind:"delta",base:head.manifest.generation,dirty:mine.built.dirty}};
  const outcome=await port.publish({manifestDigest:await port.writeManifest(next),expectedGeneration:head.pointer.generation,verified:found.verified,signal});
  if(outcome.published)return count({outcome,next,index,packed,kept,theirs:found.theirs,rounds:round});
 }
 return count({conflict:"rounds"});
}
